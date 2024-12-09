use std::env;
use std::fs;

use vector::Vector2d;

pub mod direction;
pub mod grid;
pub mod vector;

pub fn load_input() -> String {
    let input_filename = env::args().nth(1).expect("Missing input file name");

    let input_str = fs::read_to_string(input_filename).expect("Could not load input file");

    input_str
}

pub fn grid_size(grid_str: &str) -> (usize, usize) {
    let width = grid_str.lines().next().expect("Non empty grid").len();
    let height = grid_str.lines().count();

    (width, height)
}

pub fn grid_indices<'grid>(
    grid_str: &'grid str,
) -> Box<dyn Iterator<Item = (Vector2d<usize>, char)> + 'grid> {
    let width = grid_str.lines().next().expect("Non empty grid").len();
    Box::new(
        grid_str
            .chars()
            .filter(|ch| *ch != '\n')
            .enumerate()
            .map(move |(idx, ch)| (Vector2d::new(idx % width, idx / width), ch)),
    )
}
