pub mod mutators;
use mutators::*;

use rand::Rng;

// Havoc mutator
pub fn random_havoc(input: &mut Vec<u8>) {
    let functions: Vec<MutatorFunction> = vec![
        bit_flip,
        bytes_swap,
        bytes_insert,
        bytes_delete,
        bytes_inc,
        bytes_dec,
        bytes_neg,
        bytes_rand,
        bytes_copy,
        bytes_expand,
        byte_shrink,
        // LibFuzzer inspired
        shuffle_bytes,
        erase_byte,
        change_bit,
        sample_replace,
        sample_swap,
        insert_sample,
    ];

    let num_functions = functions.len();
    let mut rng = rand::rng();
    let num_mut = rng.random_range(1..=255);
    for _ in 0..num_mut {
        let function_index = rng.random_range(0..num_functions);
        functions[function_index](input);
    }
}
