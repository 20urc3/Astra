pub fn copy_map(dest_map: &mut Vec<u8>, src_map: &[u8]) {
    dest_map
        .iter_mut()
        .zip(src_map.iter())
        .for_each(|(dest, &src)| {
            *dest = (*dest).max(src);
        });
}

// AFL-style bucket lookup table
pub fn bucketize(hit_count: u8) -> u8 {
    const BUCKETS: [u8; 256] = [
        0, 1, 2, 3, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
        16, 16, 16, 16, 16, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 128,
    ];
    BUCKETS[hit_count as usize]
}

pub struct CoverageFlags {
    pub new_edge: bool,
    pub new_hitcount: bool,
}

/// Compare two maps and returns flag
/// if the next map is different than the original one
pub fn compare_maps(original_map: &Vec<u8>, new_map: &mut [u8]) -> CoverageFlags {
    let mut flags = CoverageFlags {
        new_edge: false,
        new_hitcount: false,
    };

    for (idx, &edge_count) in original_map.iter().enumerate() {
        let new_edge_count = new_map[idx];

        if new_edge_count > edge_count {
            flags.new_edge = true;
        }

        let original_bucket = bucketize(edge_count);
        let new_bucket = bucketize(new_edge_count);

        if new_bucket > original_bucket {
            flags.new_hitcount = true;
        }
    }

    flags
}

pub fn count_raw_edges(map: &[u8]) -> usize {
    map.iter().filter(|&&x| x > 0).count()
}

pub fn total_raw_hits(map: &[u8]) -> usize {
    map.iter().map(|&x| x as usize).sum()
}

pub fn union_maps(map1: &mut Vec<u8>, map2: &[u8]) {
    if map1.len() < map2.len() {
        map1.resize(map2.len(), 0);
    }
    for (a, &b) in map1.iter_mut().zip(map2.iter()) {
        *a = (*a).max(b);
    }
}
