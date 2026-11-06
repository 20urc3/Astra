//! This file provide mutations in an AFL/LibFuzzer style

use rand::Rng;

// This should happen in corpus collection
// When the corpus is load in memory from disk and triage
/// Reasons a mutation couldn't be applied.
#[derive(Debug)]
pub enum MutationError {
    /// The input was shorter than the mutation needs.
    TooShort { len: usize, min: usize },
}

/// Checks that the input is long enough for a mutation to run
#[inline]
fn require_min_len(len: usize, min: usize) -> Result<(), MutationError> {
    if len < min {
        return Err(MutationError::TooShort { len, min });
    }
    Ok(())
}

/// Type alias for the function pointer signature
pub type MutatorFunction = fn(&mut Vec<u8>) -> Result<(), MutationError>;

/// Flips a random bit in a random byte
pub fn bit_flip(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let byte_index = rng.random_range(0..length) as usize;
            let bit_index = rng.random_range(0..8) as usize;
            input[byte_index] ^= 1 << bit_index;
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

/// Flips a random byte with another random byte
pub fn bytes_swap(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let previous_byte = rng.random_range(0..length) as usize;
            let next_byte = rng.random_range(0..length) as usize;
            input.swap(previous_byte, next_byte);
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

/// Inserts a random byte at random index
pub fn bytes_insert(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let byte_index = rng.random_range(0..length) as usize;
            let random_byte: u8 = rng.random_range(0..=255);
            input.insert(byte_index, random_byte);
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

/// Deletes a random byte at a random index
/// To avoid producing too small test cases, the function check if the length is
/// superior to 23 bytes (arbitrary value)
pub fn bytes_delete(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let byte_index = rng.random_range(0..length) as usize;
            input.remove(byte_index);
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

/// Increase a random byte at random index
pub fn bytes_inc(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let byte_index = rng.random_range(0..length) as usize;
            input[byte_index] = input[byte_index].wrapping_add(1); // This allows to overflow if 255
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

/// Decrement a random byte at random index
pub fn bytes_dec(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let byte_index = rng.random_range(0..length) as usize;
            input[byte_index] = input[byte_index].wrapping_sub(1); // This allows to underflow if 0
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

/// Negates a random byte at a random index
pub fn bytes_neg(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let byte_index = rng.random_range(0..length) as usize;
            input[byte_index] = input[byte_index].wrapping_neg();
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

/// Randomized the value of a random byte at random index
pub fn bytes_rand(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let byte_index = rng.random_range(0..length) as usize;
            input[byte_index] = rng.random_range(0..=255) as u8;
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

/// Copy a random byte at random index and insert it right after
pub fn bytes_copy(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let byte_index = rng.random_range(0..length) as usize;
            let byte_copy = input[byte_index];
            input.insert(byte_index + 1, byte_copy);
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Expand the input by a random byte
pub fn bytes_expand(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            if length < 10_000_000 {
                let mut rng = rand::rng();
                let random_byte = rng.random_range(0..=255) as u8;
                input.push(random_byte);
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Shrinks the input by a byte
pub fn byte_shrink(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            if length > 23 {
                input.pop();
            }
        }

        Err(e) => return Err(e),
    }

    Ok(())
}

// LibFuzzer inspired mutators
// from `FuzzerMutate.cpp` - https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/fuzzer/FuzzerMutate.cpp

/// Random char
/// - Provides a random ASCII character
pub fn random_char() -> u8 {
    let char = rand::rng().random_range(0..=127);
    char
}

/// Shuffle 1 to 8 bytes
pub fn shuffle_bytes(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let numbyte = rand::rng().random_range(0..=8);

            for _ in 0..numbyte {
                let a = rand::rng().random_range(0..length);
                let b = rand::rng().random_range(0..length);

                input.swap(a, b);
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Delete 1 byte: remove a randomly chosen byte, shrinking the Vec by one.
pub fn erase_byte(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let index = rand::rng().random_range(0..length);
            input.remove(index);
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

// - InsertRepeatedByte
/// Bitwise operation on a byte
pub fn change_bit(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let index = rng.random_range(0..length);
            let byte = input[index];
            let operand: u8 = rng.random();

            input[index] = match rng.random_range(0..6) {
                0 => byte ^ operand,        // XOR
                1 => byte & operand,        // AND
                2 => byte | operand,        // OR
                3 => !byte,                 // NOT (operand unused)
                4 => byte << (operand % 8), // shift left (0..=7, avoids overflow)
                _ => byte >> (operand % 8), // shift right (0..=7)
            };
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Inspired by LLVM `CopyPartOf`
/// Overwrites part of the input from [idx, size] with [rand_idx, sample_size]
/// The sample size is hardcoded to 16 for the moment
pub fn sample_replace(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let sample_size = rng.random_range(1..16);
            let mut idx1 = rng.random_range(0..=length - sample_size);
            let mut idx2 = rng.random_range(0..=length - sample_size);

            for _ in 0..sample_size {
                input[idx1] = input[idx2];
                idx1 += 1;
                idx2 += 1;
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Inspired by LLVM `CopyPartOf`
/// Swap part of the input from [idx, size] with sample [idx2, size]
/// Sample size is hardcoded to 16
pub fn sample_swap(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let sample_size = rng.random_range(1..16);
            let mut idx1 = rng.random_range(0..=length - sample_size);
            let mut idx2 = rng.random_range(0..=length - sample_size);

            for _ in 0..sample_size {
                input.swap(idx1, idx2);
                idx1 += 1;
                idx2 += 1;
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Inspired by LLVM `InsertPartOf`
/// Inserts part of the input at random index
pub fn insert_sample(input: &mut Vec<u8>) -> Result<(), MutationError> {
    let length = input.len();
    match require_min_len(length, 32) {
        Ok(()) => {
            let mut rng = rand::rng();
            let sample_size = rng.random_range(1..16);
            let src = rng.random_range(0..=length - sample_size);
            let dst = rng.random_range(0..=length);

            // Copy the contiguous chunk out first, so later inserts don't disturb the source.
            let sample = input[src..src + sample_size].to_vec();
            input.splice(dst..dst, sample);
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
