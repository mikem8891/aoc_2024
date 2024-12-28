const DAY_NUM: &str = "5";

use std::fmt::Display;

fn parse_input(input: &str) -> (Vec<(u8,u8)>, Vec<Vec<u8>>) {
    let mut rules = vec![];
    let mut updates = vec![];
    for line in input.lines() {
        if let Some(rule) = line.split_once('|') {
            rules.push(rule);
            continue;
        }
        let update: Vec<_> = line.split(',')
            .filter_map(|s| s.parse::<u8>().ok())
            .collect();
        if !update.is_empty() {
            updates.push(update);
        }
    }
    (rules, updates)
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let (rules, updates) = parse_input(input);
    
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
