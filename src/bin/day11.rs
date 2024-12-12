use std::collections::HashMap;

use advent_of_code_2024::load_input;

fn has_even_digits(number: u64) -> bool {
    (number.ilog10() + 1) % 2 == 0
}

fn split_number(number: u64) -> (u64, u64) {
    let n_digits = (number.ilog10() + 1) / 2;
    let denominator = 10u64.pow(n_digits);

    (number / denominator, number % denominator)
}

fn increment_stones(stone_count: &mut HashMap<u64, usize>, number: u64, count: usize) {
    *stone_count.entry(number).or_default() += count
}

fn simulate_stones(stones: &[u64], blinks: usize) -> usize {
    let mut stone_count: HashMap<u64, usize> =
        stones
            .iter()
            .copied()
            .fold(HashMap::new(), |mut acc, stone| {
                *acc.entry(stone).or_default() += 1;
                acc
            });

    for _ in 0..blinks {
        let mut new_stones: HashMap<u64, usize> = HashMap::new();

        for (stone, count) in stone_count {
            match stone {
                0 => increment_stones(&mut new_stones, 1, count),
                n if has_even_digits(n) => {
                    let (l, r) = split_number(n);
                    increment_stones(&mut new_stones, l, count);
                    increment_stones(&mut new_stones, r, count);
                }
                n => increment_stones(&mut new_stones, n * 2024, count),
            }
        }

        stone_count = new_stones;
    }

    stone_count.values().sum()
}

fn main() {
    let input = load_input();

    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Valid number"))
        .collect();

    let final_stones = simulate_stones(&stones, 25);
    println!("Part 1: {}", final_stones);

    let final_stones = simulate_stones(&stones, 75);
    println!("Part 2: {}", final_stones);
}
