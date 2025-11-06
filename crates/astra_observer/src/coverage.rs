pub fn populate_global_map(edge_map: &[u8]) -> Vec<u8> {
    edge_map.iter().map(|&hit| bucketize(hit)).collect()
}

pub fn print_edge_found(edge_map: &[u8]) {
    for (idx, &edge_count) in edge_map.iter().enumerate() {
        if edge_count != 0 {
            println!("edge_map[{}] = {}", idx, edge_count);
        }
    }
}

// AFL-style bucket lookup table
pub fn bucketize(hit_count: u8) -> u8 {
        const BUCKETS: [u8; 256] = [
        0, 1, 2, 3, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 8, 16, 16, 16, 16,
        16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 32, 32, 32, 32, 
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 128
    ];
    BUCKETS[hit_count as usize]
}

pub struct CoverageFlags {
    pub new_edge: bool,
    pub new_hit: bool,
}

/// Compare a new edge map to the global map, updating global_map with any new coverage.
pub fn compare_global_to_edge(edge_map: &[u8], global_map: &mut Vec<u8>) -> CoverageFlags {
    let mut flags = CoverageFlags { new_edge: false, new_hit: false };

    for (idx, &edge_count) in edge_map.iter().enumerate() {
        if edge_count == 0 {
            continue;
        }

        let bucketed = bucketize(edge_count);

        if global_map[idx] == 0 {
            println!("New edge found: {}", idx);
            global_map[idx] = bucketed;
            flags.new_edge = true;
        } else if bucketed > global_map[idx] {
            println!("New hit-count found: {} ({} -> {})", idx, global_map[idx], bucketed);
            global_map[idx] = bucketed;
            flags.new_hit = true;
        }
    }

    flags
}
