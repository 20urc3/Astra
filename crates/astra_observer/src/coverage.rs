pub fn populate_from_map(previous_map: &[u8]) -> Vec<u8> {
    previous_map.iter().map(|&hit| bucketize(hit)).collect()
}

pub fn print_map(map: &[u8]) {
    for (idx, &edge_count) in map.iter().enumerate() {
        if edge_count != 0 {
            println!("map[{}] = {}", idx, edge_count);
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

/// Compare two maps and returns flag i
/// if the next map is different than the previous one
pub fn compare_maps(previous_map: &[u8], new_map: &[u8]) -> CoverageFlags {
    let mut flags = CoverageFlags {
        new_edge: false,
        new_hit: false,
    };

    for (idx, &prev_hit) in previous_map.iter().enumerate() {
        let new_hit = new_map[idx];

        // Case 1: New edge discovered (was 0 before, now non-zero)
        if prev_hit == 0 && new_hit > 0 {
            flags.new_edge = true;
        }

        // Case 2: Edge seen before, but hitcount (bucketized) increased
        let prev_bucket = bucketize(prev_hit);
        let next_bucket = bucketize(new_hit);

        if prev_bucket < next_bucket {
            flags.new_hit = true;
        }

        // Optional early exit if both found
        if flags.new_edge && flags.new_hit {
            break;
        }
    }

    flags
}

/// Copies from a map to a map
/// AFL style (bucketized)
pub fn copy_map(from: &[u8], to: &mut Vec<u8>) {
    if to.len() < from.len() {
        to.resize(from.len(), 0);
    }

    for (i, &val) in from.iter().enumerate() {
        if val > 0 {
            to[i] = std::cmp::max(to[i], val);
        }
    }
}
