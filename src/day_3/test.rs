const EXAMPLE_1: &str = "161";
const EXAMPLE_2: &str = "_";
const PART_1: &str = include_str!("part_1.txt");
const PART_2: &str = include_str!("part_2.txt");

use super::*;

#[test]
fn example_1() {
    let input = include_str!("example_1.txt");
    let (solution_1, _) = solve(&input);
    assert_eq!(solution_1.to_string(), EXAMPLE_1);
}

#[test]
fn part_1() {
    let input = include_str!("input.txt");
    let (solution_1, _) = solve(&input);
    assert_eq!(solution_1.to_string(), PART_1);
}

#[test]
fn example_2() {
    let input = include_str!("example_2.txt");
    let (_, solution_2) = solve(&input);
    assert_eq!(solution_2.to_string(), EXAMPLE_2);
}

#[test]
fn part_2() {
    let input = include_str!("input.txt");
    let (_, solution_2) = solve(&input);
    assert_eq!(solution_2.to_string(), PART_2);
}
