const DAY_NUM: &str = "3";

use std::fmt::Display;

struct Args {
    input: &str,
    i: usize
}

impl Iterator for Args {
    
    type Item = Option<(u32,u32)>;
    
    fn next(&mut self) -> Option<(u32,u32)> {
        
        None
    }
}

fn solve(input: &str) -> (impl Display, impl Display) {

    let len = input.len();
    let mut i = 0;
    let mut sum = 0;

    loop {
        let sub_str = &input[i..];
        if let Some(is) = sub_str.find("mul(") {
            let sub_str = &input[is..];
            if let Some(ie) = &sub_str[0..12].find(')') {
                match &sub_str[4..ie].split_once(',') {
                    Some(s1, s2) => {
                        let n1 = s1.parse::<u32>();
                        let n2 = s2.parse::<u32>();
                        match (n1, n2) {
                            (Ok(n1), Ok(n2)) => {
                                sum += n1 * n2;
                                i = is + 4;
                            },
                            _ => {
                                i = is + 4;
                            }
                        }
                    },
                    _ => {
                        i = is + 4;
                    }
                }
            } else {
                i = is + 4;
            }
        } else {break;}
    }

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
