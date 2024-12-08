use std::iter;

use advent_of_code_2024::load_input;

#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

fn parse_equation(equation: &str) -> Equation {
    let (result, numbers) = equation.split_once(": ").expect("Line separated by ': '");

    let result: u64 = result.parse().expect("Valid result number");
    let numbers: Vec<u64> = numbers
        .split_whitespace()
        .map(|num| num.parse().expect("Valid number"))
        .collect();

    Equation { result, numbers }
}

fn parse_equations(input: &str) -> Vec<Equation> {
    input.lines().map(parse_equation).collect()
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    let r_digits = rhs.ilog10() + 1;
    lhs * 10u64.pow(r_digits) + rhs
}

fn possible_results<'val>(
    values: &'val [u64],
    with_concat: bool,
) -> Box<dyn Iterator<Item = u64> + 'val> {
    match values {
        [l, r] => {
            let sum = iter::once(l + r);
            let mul = iter::once(l * r);

            if with_concat {
                let cat = iter::once(concat(*l, *r));
                return Box::new(sum.chain(mul).chain(cat));
            } else {
                return Box::new(sum.chain(mul));
            }
        }

        [prev @ .., r] => {
            let sum_res = possible_results(prev, with_concat).map(move |res| res + r);
            let mul_res = possible_results(prev, with_concat).map(move |res| res * r);

            if with_concat {
                let concat_res =
                    possible_results(prev, with_concat).map(move |res| concat(res, *r));

                return Box::new(sum_res.chain(mul_res).chain(concat_res));
            } else {
                return Box::new(sum_res.chain(mul_res));
            }
        }
        _ => panic!("Value list contains less than 2 elements"),
    }
}

fn validate_equation(equation: &Equation, with_concat: bool) -> bool {
    possible_results(&equation.numbers, with_concat)
        .filter(|n| *n == equation.result)
        .next()
        .is_some()
}

fn sum_valid(equations: &[Equation], with_concat: bool) -> u64 {
    equations
        .iter()
        .filter_map(|eq| {
            if validate_equation(eq, with_concat) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = load_input();
    let equations = parse_equations(&input);

    let total: u64 = sum_valid(&equations, false);
    let total_concat: u64 = sum_valid(&equations, true);

    println!("Part 1: {}", total);
    println!("Part 2: {}", total_concat);
}
