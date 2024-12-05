use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use advent_of_code_2024::load_input;

type Rules = HashMap<u8, HashSet<u8>>;

fn parse_rules(rules: &str) -> Rules {
    let mut rule_map: HashMap<u8, HashSet<u8>> = HashMap::new();

    rules.lines().for_each(|line| {
        let (page, precedes) = line.split_once("|").expect("Line contains '|'");
        let page: u8 = page.parse().expect("Page number");
        let precedes: u8 = precedes.parse().expect("Precedes number");

        rule_map.entry(page).or_default().insert(precedes);
    });

    rule_map
}

fn parse_updates(updates: &str) -> Vec<Vec<u8>> {
    Vec::from_iter(updates.lines().map(|line| {
        Vec::from_iter(
            line.split(',')
                .map(|num| num.parse().expect("Update page number")),
        )
    }))
}

fn is_update_ordered(rules: &Rules, update: &[u8]) -> bool {
    for (idx, page) in update.iter().enumerate() {
        for next in update.iter().skip(1 + idx) {
            let precedes = match rules.get(next) {
                Some(prec) => prec,
                None => continue,
            };
            if precedes.contains(page) {
                return false;
            }
        }
    }

    true
}

fn fix_update(rules: &Rules, update: &[u8]) -> Vec<u8> {
    let mut new: Vec<u8> = Vec::from(update);

    new.sort_by(|a, b| {
        use Ordering::*;
        match rules.get(a) {
            Some(precedes) => {
                if precedes.contains(b) {
                    Less
                } else {
                    Greater
                }
            }
            None => Equal,
        }
    });

    new
}

fn main() {
    let input = load_input();
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("Input two sections separated by \"\\n\\n\"");

    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    let correct: u64 = updates
        .iter()
        .filter(|up| is_update_ordered(&rules, up))
        .map(|up| up[up.len() / 2] as u64)
        .sum();

    let corrected: u64 = updates
        .iter()
        .filter(|up| !is_update_ordered(&rules, up))
        .map(|up| fix_update(&rules, up))
        .map(|up| up[up.len() / 2] as u64)
        .sum();

    println!("Part 1: {correct}");
    println!("Part 2: {corrected}");
}
