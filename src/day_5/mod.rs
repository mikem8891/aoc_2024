const DAY_NUM: &str = "5";

use std::{collections::{HashMap, HashSet}, fmt::Display};

fn parse_input(input: &str) -> (Vec<(u8,u8)>, Vec<Vec<u8>>) {
    let mut rules = vec![];
    let mut updates = vec![];
    for line in input.lines() {
        if let Some(rule) = line.split_once('|') {
            let (left, right) = rule;
            let left = left.parse::<u8>().unwrap();
            let right = right.parse::<u8>().unwrap();
            rules.push((left, right));
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

fn get_mid_num(nums: &[u8]) -> u8 {
    nums[(nums.len() - 1) / 2]
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let (rules, updates) = parse_input(input);
    let mut rules_map = HashMap::new();
    for (left, right) in rules {
        let right_num_set = rules_map.entry(left).or_insert(HashSet::new());
        right_num_set.insert(right);
    }

    let mut sum: u32 = 0;
    'update: for update in updates {
        let len = update.len();
        for li in 0..len {
            for ri in (li + 1)..len {
                if let Some(right_num_set) = rules_map.get(&update[ri]) {
                    if right_num_set.contains(&update[li]) {
                        continue 'update;
                    }
                }
            }
        }
        sum += get_mid_num(&update) as u32;
    }
    
    (sum, "todo")
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = stopwatch!(solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;
