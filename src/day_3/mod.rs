const DAY_NUM: &str = "3";

use std::fmt::Display;

struct Args<'a> {
    input: &'a str,
    i: usize
}

impl<'a> Iterator for Args<'a> {
    
    type Item = (u32,u32);
    
    fn next(&mut self) -> Option<(u32,u32)> {
        loop {
            self.i = match &self.input[self.i..].find("mul(") {
                Some(i) => i + 4,
                _ => return None
            };
            let args_str: Option<(&str, &str)> = match &self.input[self.i..][..8].find(')') {
                Some(i) => &self.input[self.i..][..i].split_once(','),
                _ => continue
            };
            let args_num = match args_str {
                Some((s1, s2)) => {
                    let n1 = s1.parse::<u32>();
                    let n2 = s2.parse::<u32>();
                    (n1, n2)
                },
                _ => continue
            };
            match args_num {
                (Ok(n1), Ok(n2)) => return Some((n1, n2)),
                _ => continue
            }
        }   
    }
}

fn solve(input: &str) -> (impl Display, impl Display) {

    let sum: u32= Args{input, i: 0}.map(|(n1, n2)| n1 * n2).sum();

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
