const DAY_NUM: &str = "1";

use std::fmt::Display;

fn solve(input: &str) -> (impl Display, impl Display) {
    
    ("todo", "todo")
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let (part_1, part_2) = solve(&input);
    println!(" part 1 is {part_1}");
    println!(" part 2 is {part_2}");
}

#[cfg(test)]
mod test;