//! This file provided havoc mutation in an AFL style 
//! 
//! The function tkaes 3 arguments:
//! - An input which is a vector of bytes
//! - The length of the input to avoid redundant computation
//! - The number of mutations

use rand::Rng;


pub fn bytes_swap(input: &mut [u8], length: u32, mutations: u8) {
    let mut rng = rand::rng();

    for _ in 0..mutations {
        let previous_byte = rng.random_range(0..length) as usize;
        let next_byte = rng.random_range(0..length) as usize;

        input.swap(previous_byte, next_byte);
    }
}

/*
pub fn BytesFlip(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesInsert(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesDelete(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesInc(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesDec(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesNeg(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesRand(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesCopy(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesExpand(input: &mut [u8],  length: u32, mutations: u8) {}
pub fn BytesShrink(input: &mut [u8],  length: u32, mutations: u8) {}

*/