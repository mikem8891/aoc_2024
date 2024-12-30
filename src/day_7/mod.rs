const DAY_NUM: &str = "7";

use std::fmt::Display;


#[derive(PartialEq, Clone, Copy)]
enum Operator {
    Add, Multiply
}

impl Operator {
    fn eval(&self, n1: u64, n2: u64) -> u64 {
        match self {
            Operator::Add      => n1 + n2,
            Operator::Multiply => n1 * n2
        }
    }
}

struct OperatorPurmutations {
    ops: Box<[Operator]>,
    started: bool
}

impl OperatorPurmutations {
    fn new(cal_vals: &[u64]) -> OperatorPurmutations {
        let len = cal_vals.len() - 1;
        let ops: Box<[Operator]> = Box::from(vec![Operator::Add; len]);
        OperatorPurmutations{ops, started: false}
    }
}

impl Iterator for OperatorPurmutations {
    type Item = Box<[Operator]>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.ops.clone())
        }
        let mut i = 0;
        while (i < self.ops.len()) && (self.ops[i] == Operator::Multiply) {
            self.ops[i] = Operator::Add;
            i += 1;
        }
        if i < self.ops.len() {
            self.ops[i] = Operator::Multiply;
            Some(self.ops.clone())
        } else {
            None
        }
    }
}

trait CalibrationEquation{
    fn can_result_in(&self, cal_result: u64) -> bool;
    fn evaluate_with(&self, cal_ops: &[Operator]) -> u64;
}

impl CalibrationEquation for Vec<u64> {
    fn can_result_in(&self, cal_result: u64) -> bool {
        let mut op_perms = OperatorPurmutations::new(self);
        op_perms.any(|o| self.evaluate_with(&o) == cal_result)
    }
    
    fn evaluate_with(&self, cal_ops: &[Operator]) -> u64 {
        let mut cal_result = self[0];
        for (op, val) in cal_ops.iter().zip(&self[1..]) {
            cal_result = op.eval(cal_result, *val);
        }
        cal_result
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
