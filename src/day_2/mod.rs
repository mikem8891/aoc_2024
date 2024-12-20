const DAY_NUM: &str = "2";

use std::{fmt::Display, iter::zip};

fn solve(input: &str) -> (impl Display, impl Display) {
    let reports = input.lines().map(parse_report);
    let safe_count_1 = reports.clone().filter(is_safe_1).count();
    let safe_count_2 = reports.clone().filter(is_safe_2).count();
    (safe_count_1, safe_count_2)
}

fn parse_report(report_str: &str) -> Vec<i64> {
    report_str.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect()
}

fn is_safe_1(levels: &Vec<i64>) -> bool {
    let diff: Vec<_> = zip(&levels[..], &levels[1..]).map(|(a, b)| b - a).collect();
    let sign = diff[0].signum();
    if sign == 1 {
        !diff.iter().any(|d| !(1..=3).contains(d))
    } else if sign == -1 {
        !diff.iter().any(|d| !(-3..=-1).contains(d))
    } else {false}
}

fn is_safe_2(levels: &Vec<i64>) -> bool {
    if is_safe_1(levels) {
        true
    } else {
        let len = levels.len();
        for i in 0..len {
            let mut dampened = Vec::from(&levels[0..i]);
            dampened.extend_from_slice(&levels[(i + 1)..len]);
            if is_safe_1(&dampened) {
                return true
            }
        }
        false
    }
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = crate::util::stopwatch(|| solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;
