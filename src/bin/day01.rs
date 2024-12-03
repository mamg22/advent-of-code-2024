use std::collections::HashMap;

use advent_of_code_2024::load_input;

fn main() {
    let input = load_input();

    let (mut left_nums, mut right_nums): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            let l_num: u64 = l.parse().unwrap();
            let r_num: u64 = r.parse().unwrap();
            return (l_num, r_num);
        })
        .unzip();

    left_nums.sort();
    right_nums.sort();

    let total: u64 = left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("Part 1: {total}");

    let mut right_counts: HashMap<u64, u64> = HashMap::new();

    for number in right_nums {
        right_counts
            .entry(number)
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    let total: u64 = left_nums
        .iter()
        .map(|num| num * right_counts.get(num).cloned().unwrap_or(0))
        .sum();

    println!("Part 2: {total}");
}
