const EXAMPLE_SOLUTION: [&str; 2] = ["_", "_"];

use super::*;

#[test]
fn example_1(){
    let input = include_str!("example_1.txt");
    let (solution_1, _) = solve(&input);
    assert_eq!(solution_1.to_string(), EXAMPLE_SOLUTION[0]); 
}

#[test]
fn example_2(){
    let input = include_str!("example_2.txt");
    let (_, solution_2) = solve(&input);
    assert_eq!(solution_2.to_string(), EXAMPLE_SOLUTION[1]); 
}