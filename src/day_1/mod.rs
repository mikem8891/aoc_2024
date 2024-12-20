const DAY_NUM: &str = "1";

use std::{collections::BTreeMap, fmt::Display, iter::zip};

fn solve(input: &str) -> (impl Display, impl Display) {
  let mut id_list_1 = vec!();
  let mut id_list_2 = vec!();
  for line in input.lines() {
    let mut ids = line.split("   ").filter_map(|s| s.parse::<u64>().ok());
    id_list_1.push(ids.next().expect("1st ID"));
    id_list_2.push(ids.next().expect("2nd ID"));
  }
  fn sorted_copy(list: &Vec<u64>) -> Vec<u64>{
    let mut sorted = list.clone();
    sorted.sort_unstable();
    sorted
  }
  let id_list_1_sorted = sorted_copy(&id_list_1);
  let id_list_2_sorted = sorted_copy(&id_list_2);
  let sum_1 = zip(id_list_1_sorted, id_list_2_sorted)
    .map(|(id_1, id_2)| id_1.abs_diff(id_2))
    .sum::<u64>();

  let mut count = BTreeMap::new();
  for id in id_list_2 {
    count.entry(id).and_modify(|c| *c += 1).or_insert(1_u64);
  }
  let sum_2 = id_list_1.iter()
    .filter_map(|id| count.get_key_value(id))
    .map(|(id, c)| id * c)
    .sum::<u64>();

  (sum_1, sum_2)
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = stopwatch!(solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;