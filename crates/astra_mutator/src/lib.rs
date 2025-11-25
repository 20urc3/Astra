mod havoc_mutators;
use havoc_mutators::*;

use rand::Rng;

pub fn random_havoc(input: &mut Vec<u8>) {
    let input_length = input.len() as u32;

    let functions: Vec<MutatorFunction> = vec![
        bit_flip, bytes_swap, bytes_insert, bytes_delete, bytes_inc,
        bytes_dec, bytes_neg, bytes_rand, bytes_copy, bytes_expand,
        byte_shrink,
    ];

    let num_functions = functions.len();
    let mut rng = rand::rng();
    let function_index = rng.random_range(0..num_functions);

    let mut rng = rand::rng();
    let num_mut = rng.random_range(0..=255);
    for _ in 0..num_mut {
        functions[function_index](input, input_length);
    }
}