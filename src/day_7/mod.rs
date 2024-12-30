const DAY_NUM: &str = "7";

use std::fmt::Display;

enum Operator {
    Add, Multiply
}

struct OperatorList {
    ops: Box<[Operator]>
}

impl Iterator for OperatorList {
    type Item = OperatorList;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

trait CalibrationEquation{
    fn can_result_in(&self, cal_result: u64) -> bool;

    
}

impl CalibrationEquation for Vec<u64> {
    fn can_result_in(&self, cal_result: u64) -> bool {
        todo!()
    }
}

fn evaluate_line(line: &str) -> Option<u64> {
    let (cal_result, cal_equ) = line.split_once(':').unwrap();
    let cal_result: u64 = cal_result.parse().unwrap();
    let cal_equ: Vec<_> = cal_equ.split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok()).collect();
    match cal_equ.can_result_in(cal_result) {
        true  => Some(cal_result),
        false => None
    }
}

fn solve(input: &str) -> (impl Display, impl Display) {

    let cal_total: u64 = input.lines().filter_map(evaluate_line).sum();


    (cal_total, "todo")
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = stopwatch!(solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;
