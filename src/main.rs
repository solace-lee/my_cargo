// use rand::{thread_rng, Rng};
use rand::Rng;
use std::{cmp::Ordering, io};
mod data_type;
mod guess_number;
mod test_char;
mod vector;

fn main() {
    data_type::main_type();
    // vector::vector_demo::vector_test();
    vector::vector_demo::enum_test();
    test_char::test_char::char_test();
    test_char::test_hashmap::hash_map();
    guess_number::guess_number::guess::simple_guess();
    return;
}
