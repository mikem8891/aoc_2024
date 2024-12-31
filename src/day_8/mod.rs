const DAY_NUM: &str = "8";

use std::fmt::Display;

fn enumerate2D(input: &str) -> impl Iterater {
    let enumerate_line = |r: usize, l: &str| {
        l.as_bytes().enumerate().map(|(c, b)| ((r, c), b))
    };
    input.line().enumerate()
        .map(|(r,l)| enumerate_line(r, l)).flatten()
}


fn solve(input: &str) -> (impl Display, impl Display) {
    let ant_iter = enumerate2D(input).filter(|p, b| b != b'.');
    let ant_map = HashMap::new();
    ant_iter.map(|(p, b)| ant_map.entry(b))
    
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
