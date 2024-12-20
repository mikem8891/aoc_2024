const DAY_NUM: &str = "2";

use std::fmt::Display;

fn solve(_input: &str) -> (impl Display, impl Display) {
    
    ("todo", "todo")
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = crate::util::stopwatch(|| solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;