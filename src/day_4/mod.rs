const DAY_NUM: &str = "4";

use std::fmt::Display;

const XMAS_FORWARD: str = 
"
XMAS
";



fn solve(input: &str) -> (impl Display, impl Display) {
    let word_search: Vec<_> = input.lines().map(str::as_bytes).collect();
    
    
    ("todo", "todo")
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = stopwatch!(solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;
