mod havoc_mutators;
use havoc_mutators::*;

use rand::Rng;

pub fn random_havoc(input: &mut Vec<u8>) {
    let input_length = input.len() as u32;
    let mutations_count: u8 = 16;

    let functions: Vec<MutatorFunction> = vec![
        bit_flip, bytes_swap, bytes_insert, bytes_delete, bytes_inc,
        bytes_dec, bytes_neg, bytes_rand, bytes_copy, bytes_expand,
        byte_shrink,
    ];

    let num_functions = functions.len();
    let mut rng = rand::rng();
    let function_index = rng.random_range(0..num_functions);

    functions[function_index](input, input_length, mutations_count);
}
