//! This file provided havoc mutation in an AFL style 
//! 
//! The function tkaes 3 arguments:
//! - An input which is a vector of bytes
//! - The length of the input to avoid redundant computation
//! - The number of mutations

use rand::Rng;

/// Type alias for the function pointer signature
pub type MutatorFunction = fn(&mut Vec<u8>);

/// Flips a random bit in a random byte
pub fn bit_flip(input: &mut Vec<u8>) {
let length = input.len() as u32;
    let mut rng = rand::rng();
    let byte_index = rng.random_range(0..length-1) as usize;
    let bit_index = rng.random_range(0..8) as usize;
    input[byte_index] ^= 1 << bit_index; 

}

/// Flips a random byte with another random byte
pub fn bytes_swap(input: &mut Vec<u8>) {
let length = input.len() as u32;
    let mut rng = rand::rng();
    let previous_byte = rng.random_range(0..length-1) as usize;
    let next_byte = rng.random_range(0..length-1) as usize;
    input.swap(previous_byte, next_byte);

}

/// Inserts a random byte at random index
pub fn bytes_insert(input: &mut Vec<u8>) {
let length = input.len() as u32;
    let mut rng = rand::rng();
    let byte_index = rng.random_range(0..length-1) as usize;
    let random_byte: u8 = rng.random_range(0..=255);
    input.insert(byte_index, random_byte);
}

/// Deletes a random byte at a random index
/// To avoid producing too small test cases, the function check if the length is 
/// superior to 23 bytes (arbitrary value)
pub fn bytes_delete(input: &mut Vec<u8>) {
let length = input.len() as u32;
    if length > 23 {
        let mut rng = rand::rng();
        let byte_index = rng.random_range(0..length-1) as usize;
        input.remove(byte_index);
    }
}

/// Increase a random byte at random index
pub fn bytes_inc(input: &mut Vec<u8>) {
let length = input.len() as u32;
    let mut rng = rand::rng();
    let byte_index = rng.random_range(0..length-1) as usize;
    input[byte_index] = input[byte_index].wrapping_add(1); // This allows to overflow if 255
}

/// Decrement a random byte at random index
pub fn bytes_dec(input: &mut Vec<u8>) {
let length = input.len() as u32;
    let mut rng = rand::rng();
    let byte_index = rng.random_range(0..length-1) as usize;
    input[byte_index] = input[byte_index].wrapping_sub(1); // This allows to underflow if 0
}

/// Negates a random byte at a random index
pub fn bytes_neg(input: &mut Vec<u8>) {
let length = input.len() as u32;
    let mut rng = rand::rng();
    let byte_index = rng.random_range(0..length-1) as usize;
    input[byte_index] = input[byte_index].wrapping_neg();
}

/// Randomized the value of a random byte at random index
pub fn bytes_rand(input: &mut Vec<u8>) {
let length = input.len() as u32;
    let mut rng = rand::rng();
    let byte_index = rng.random_range(0..length-1) as usize;
    input[byte_index] = rng.random_range(0..=255) as u8;
}

/// Copy a random byte at random index and insert it right after
pub fn bytes_copy(input: &mut Vec<u8>) {
let length = input.len() as u32;
    let mut rng = rand::rng();
    let byte_index = rng.random_range(0..length-1) as usize;
    let byte_copy = input[byte_index];
    input.insert(byte_index +1, byte_copy);
}

/// Expand the input by a random byte for a number of mutations passed
pub fn bytes_expand(input: &mut Vec<u8>) {
let length = input.len() as u32;
    if length < 10_000_000 {
        let mut rng = rand::rng();
        let random_byte = rng.random_range(0..=255) as u8;
        input.push(random_byte);
    }
}

/// Shrinks the input by a byte
pub fn byte_shrink(input: &mut Vec<u8>) {
let length = input.len() as u32;
    if length > 23 {
    input.pop();
    }
}