pub mod coverage;
pub mod shm;

/// Size of the coverage edge map, in bytes. Single source of truth for the
/// `astra` crate. NOTE: `astra_sancov` is a separate, dependency-free crate
/// and defines its own `MAP_SIZE` — the two must be kept equal.
pub const MAP_SIZE: usize = 262_144;
