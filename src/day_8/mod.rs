const DAY_NUM: &str = "8";

use std::{collections::{HashMap, HashSet}, fmt::Display};

fn enumerate2D<'a>(input: &str) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    input.lines().enumerate()
        .map(|(r,l)| 
            l.as_bytes().iter().enumerate()
                .map(move |(c, &b)| ((r, c), b))).flatten()
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let ant_iter = enumerate2D(input)
        .filter(|(p, b)| *b != b'.');
    let mut ant_map = HashMap::new();
    ant_iter.for_each(|(p, b)| {
            ant_map.entry(b).or_insert(HashSet::new()).insert(p);
    });
    
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
