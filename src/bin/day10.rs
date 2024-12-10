use std::collections::{HashSet, VecDeque};

use advent_of_code_2024::{
    direction::{AsVector, Direction},
    grid::Grid,
    load_input,
    vector::Vector2d,
};

fn find_trails(grid: &Grid<u8>, position: Vector2d<usize>, height: u8) -> (usize, usize) {
    use Direction::*;

    let mut to_check: VecDeque<(Vector2d<usize>, u8)> = VecDeque::new();
    let mut tops: HashSet<Vector2d<usize>> = HashSet::new();

    to_check.push_back((position, height));

    let mut rating = 0;

    while let Some((position, height)) = to_check.pop_front() {
        if height == 9 {
            rating += 1;
            tops.insert(position);
            continue;
        }

        let position: Vector2d<isize> = position
            .try_components_into()
            .expect("Position convertible to isize");

        for direction in [North, East, South, West] {
            let dir_vec: Vector2d<isize> = direction.as_vector();
            let new_pos = position + dir_vec;

            let new_pos: Vector2d<usize> = match new_pos.try_components_into() {
                Ok(pos) => pos,
                Err(_) => continue,
            };

            let Some(next_slope) = grid.get(new_pos.x, new_pos.y) else {
                continue;
            };

            if *next_slope > height && next_slope.abs_diff(height) == 1 {
                to_check.push_back((new_pos, *next_slope));
            }
        }
    }

    (tops.len(), rating)
}

fn count_trailheads(grid: &Grid<u8>) -> (usize, usize) {
    let (scores, ratings) = grid
        .item_indices()
        .filter(|(_, height)| **height == 0)
        .map(|(position, height)| find_trails(grid, position, *height))
        .reduce(|(score_acc, rating_acc), (score, rating)| (score_acc + score, rating_acc + rating))
        .expect("Trails found");

    (scores, ratings)
}

fn main() {
    let input = load_input();

    let grid: Grid<u8> = Grid::from_text(&input, |ch| ch.to_digit(10).expect("Valid digit") as u8);

    let (scores, ratings) = count_trailheads(&grid);
    println!("Part 1: {}", scores);
    println!("Part 2: {}", ratings);
}
