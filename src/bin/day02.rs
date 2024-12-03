use std::cmp::Ordering;

use advent_of_code_2024::load_input;

fn is_report_safe(report: &[u64]) -> bool {
    let expected_order = report[0].cmp(&report[1]);
    if expected_order == Ordering::Equal {
        return false;
    }

    for pair in report.windows(2) {
        if let [l, r] = pair {
            if l.cmp(r) != expected_order {
                return false;
            }

            let diff = l.abs_diff(*r);
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }

    true
}

fn is_report_safe_dampened(report: &[u64]) -> bool {
    if is_report_safe(report) {
        return true;
    }

    let safe_found = (0..report.len())
        .filter(|i| {
            let (l, r) = report.split_at(*i);

            let l_iter = l.iter().chain(r.iter().skip(1));
            let r_iter = l.iter().chain(r.iter().skip(1)).skip(1);

            let mut expected_order: Option<Ordering> = None;

            for (l, r) in l_iter.zip(r_iter) {
                if expected_order.is_none() {
                    let order = l.cmp(r);

                    if order == Ordering::Equal {
                        return false;
                    }

                    expected_order = Some(order);
                }

                if l.cmp(r) != *expected_order.as_ref().unwrap() {
                    return false;
                }

                let diff = l.abs_diff(*r);

                if diff < 1 || diff > 3 {
                    return false;
                }
            }

            true
        })
        .next();

    safe_found.is_some()
}

fn main() {
    let input = load_input();

    let reports: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect();

    let safe = reports
        .iter()
        .filter(|report| is_report_safe(&report))
        .count();

    println!("Part 1: {safe}");

    let safe_dampened = reports
        .iter()
        .filter(|report| is_report_safe_dampened(&report))
        .count();

    println!("Part 2: {safe_dampened}");
}
