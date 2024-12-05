use std::env;
use std::fs;

pub mod direction;
pub mod grid;
pub mod vector;

pub fn load_input() -> String {
    let input_filename = env::args().nth(1).expect("Missing input file name");

    let input_str = fs::read_to_string(input_filename).expect("Could not load input file");

    input_str
}
