use libc::c_void;
use rustix::{
    fs::Mode,
    mm::{MapFlags, ProtFlags, mmap},
    shm,
};
use std::cell::Cell;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, Ordering};

// Shared memory pointer. Written once in init, read in the hot path.
static SHM_PTR: AtomicPtr<c_void> = AtomicPtr::new(null_mut());

// Coverage granularity: true = edge (default), false = block.
static EDGE_MODE: AtomicBool = AtomicBool::new(true);

// Guard numbering, unique across every instrumented DSO. Replaces the old
// `static mut N`; init may run once per module, so it must persist.
static GUARD_COUNTER: AtomicU32 = AtomicU32::new(1);

thread_local! {
    // AFL-style previous location, for edge (transition) coverage.
    // Thread-local so a multithreaded target doesn't clobber transitions
    // across threads (a shared global would produce garbage edges).
    static PREV_LOCATION: Cell<usize> = const { Cell::new(0) };
}

// Constant map size for the edge map.
// MUST match `astra::coverage::MAP_SIZE` (separate crate, so not shared).
const MAP_SIZE: usize = 262_144;

/// This initiliziation function is *supposedly* ran only once per DSO
/// It opens the shared memory file only if it exist
/// Then map it to a global variable SHM_PTR used to write the coverage
#[unsafe(no_mangle)]
pub extern "C" fn __sanitizer_cov_trace_pc_guard_init(mut start: *mut u32, stop: *mut u32) -> () {
    // If not running under the fuzzer (e.g. during ./configure) just NO-OP.
    let shm_name = match std::env::var("ASTRA_SHM_ID") {
        Ok(s) => s,
        Err(_) => return,
    };

    // Coverage mode, selected by the fuzzer via the CLI (--coverage).
    // Anything other than "block" (or unset) => edge coverage.
    let edge_mode = std::env::var("ASTRA_COV_MODE")
        .map(|m| m != "block")
        .unwrap_or(true);
    EDGE_MODE.store(edge_mode, Ordering::Relaxed);

    // Start every run with a clean transition history. Redundant in
    // fork-per-exec (fresh process), required for persistent mode.
    PREV_LOCATION.with(|p| p.set(0));

    // Open existing shared memory created by worker
    let fd = match shm::open(&shm_name, shm::OFlags::RDWR, Mode::RUSR | Mode::WUSR) {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("Error while opening shared memory: {e}");
            return;
        }
    };

    let ptr: *mut c_void = unsafe {
        match mmap(
            null_mut(),
            MAP_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::SHARED,
            &fd,
            0,
        ) {
            Ok(ptr) => ptr,
            Err(e) => {
                eprintln!("Error creating pointer: {e}");
                return;
            }
        }
    };

    // Publish the mapped pointer for the coverage callback.
    SHM_PTR.store(ptr, Ordering::Relaxed);

    // Assert that the edge map isn't bigger than the shared memory.
    let map_size: isize = match MAP_SIZE.try_into() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error converting MAP_SIZE: {e}");
            return;
        }
    };

    if (unsafe { stop.offset_from(start) }) > map_size {
        return;
    }

    unsafe {
        if start == stop || *start != 0 {
            return;
        }
    };
    while start < stop {
        unsafe {
            *start = GUARD_COUNTER.fetch_add(1, Ordering::Relaxed);
            start = start.add(1);
        }
    }
}

/// Reset per-run coverage state. The fork-server / persistent loop must
/// call this before each iteration so the first edge of a run does not
/// bleed in from the previous one. Cost is a single store.
#[unsafe(no_mangle)]
pub extern "C" fn __astra_cov_reset() -> () {
    PREV_LOCATION.with(|p| p.set(0));
}

/// This function is called every time a block is reached.
///
/// In **block** mode it records the current block id directly.
/// In **edge** mode (default) it records the transition
/// `prev -> cur` AFL-style, so A->B and C->B are distinguished.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __sanitizer_cov_trace_pc_guard(guard: *mut u32) -> () {
    // If no shared memory mapped (e.g. ./configure) just return.
    let base = SHM_PTR.load(Ordering::Relaxed) as *const AtomicU8;
    if base.is_null() {
        return;
    }

    // Atomic view of the shared map: concurrent increments from a
    // multithreaded target are well-defined (no data race) this way.
    let edge_map: &[AtomicU8] = unsafe { std::slice::from_raw_parts(base, MAP_SIZE) };

    let cur = unsafe { *guard as usize };

    let idx = if EDGE_MODE.load(Ordering::Relaxed) {
        // Edge: hash the (prev, cur) pair into the map.
        let prev = PREV_LOCATION.with(|p| {
            let v = p.get();
            // Shift so that A->B and B->A map to different entries.
            p.set(cur >> 1);
            v
        });
        (cur ^ prev) % MAP_SIZE
    } else {
        // Block: just the current block id.
        cur % MAP_SIZE
    };

    // Never-zero: wrapping 255 -> 0 would make a hot edge read as "never
    // covered". fetch_add returns the previous value, so a previous 255 means
    // we just wrapped to 0 -> bump once more to land on 1 (AFL's classify).
    let prev = edge_map[idx].fetch_add(1, Ordering::Relaxed);
    if prev == u8::MAX {
        edge_map[idx].fetch_add(1, Ordering::Relaxed);
    }
}
