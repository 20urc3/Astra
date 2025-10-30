#[no_mangle]
pub extern "C" fn __sanitizer_cov_trace_pc_guard_init(start: *mut u32, stop: *mut u32) -> () {
    println!("START: {:?}", start);
    println!("STOP: {:?}", stop);
}

#[no_mangle]
pub extern "C" fn __sanitizer_cov_trace_pc_guard(guard: *const u32) -> () {
    println!("GUARD: {:?}", guard);
}