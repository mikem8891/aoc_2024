const DAY_NUM: &str = "1";

use std::{fmt::Display, iter::zip};

fn solve(input: &str) -> (impl Display, impl Display) {
  let mut id_list_1 = vec!();
  let mut id_list_2 = vec!();
  for line in input.lines() {
    let mut ids = line.split(' ').filter_map(|s| s.parse::<u64>().ok());
    id_list_1.push(ids.next().expect("1st ID"));
    id_list_2.push(ids.next().expect("2nd ID"));
  }
  id_list_1.sort_unstable();
  id_list_2.sort_unstable();
  let sum = zip(id_list_1, id_list_2)
    .map(|(id_1, id_2)| id_1.abs_diff(id_2))
    .sum::<u64>();
  (sum, "todo")
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = crate::util::stopwatch(|| solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;