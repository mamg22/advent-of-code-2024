use std::collections::HashSet;

use advent_of_code_2024::direction::{AsVector, Direction8};
use advent_of_code_2024::grid::Grid;
use advent_of_code_2024::load_input;
use advent_of_code_2024::vector::Vector2d;

use Direction8::*;

const XMAS_DIRECTIONS: [Direction8; 8] = [
    North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
];

const MAS_DIRECTIONS: [Direction8; 4] = [NorthEast, SouthEast, SouthWest, NorthWest];

fn find_in_grid(
    target: &str,
    grid: &Grid<char>,
    start: Vector2d<usize>,
    direction: Direction8,
) -> bool {
    for (idx, ch) in target.chars().enumerate() {
        let delta: Vector2d<isize> = direction
            .as_vector::<isize>()
            .scalar_mul(idx.try_into().unwrap())
            .try_components_into()
            .unwrap();

        let position: Vector2d<isize> = start.try_components_into().unwrap() + delta;
        let position: Vector2d<usize> = match position.try_components_into() {
            Ok(v) => v,
            Err(_) => return false,
        };

        match grid.get(position.x, position.y) {
            Some(grid_ch) if *grid_ch == ch => (),
            _ => return false,
        }
    }

    true
}

fn find_xmas(
    grid: &Grid<char>,
    start: Vector2d<usize>,
    direction: Direction8,
) -> Option<Vector2d<usize>> {
    if find_in_grid("XMAS", grid, start, direction) {
        Some(start)
    } else {
        None
    }
}

fn find_mas(grid: &Grid<char>, position: Vector2d<usize>) -> bool {
    if !matches!(grid.get(position.x, position.y), Some('A')) {
        false
    } else {
        let mut neighbors: String = String::new();

        for dir in MAS_DIRECTIONS {
            let delta: Vector2d<isize> = dir.as_vector();

            let position = position.try_components_into().unwrap() + delta;
            let position: Vector2d<usize> = match position.try_components_into() {
                Ok(v) => v,
                Err(_) => continue,
            };

            match grid.get(position.x, position.y) {
                Some(ch @ 'M') | Some(ch @ 'S') => neighbors.push(*ch),
                _ => continue,
            }
        }

        if neighbors.len() != 4 {
            return false;
        }

        // Order ABCD
        // Checked as D A
        //             X
        //            C B

        matches!(neighbors.as_str(), "MSSM" | "MMSS" | "SSMM" | "SMMS")
    }
}

fn main() {
    let input = load_input();

    let grid: Grid<char> = Grid::from_text(&input, |ch| ch);

    let mut xmas: HashSet<(Vector2d<usize>, Direction8)> = HashSet::new();
    let mut mas = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Vector2d::new(x, y);

            for dir in XMAS_DIRECTIONS {
                if let Some(pos) = find_xmas(&grid, pos, dir) {
                    xmas.insert((pos, dir));
                }
            }

            if find_mas(&grid, pos) {
                mas += 1;
            }
        }
    }

    println!("Part 1: {}", xmas.len());
    println!("Part 2: {}", mas);
}
