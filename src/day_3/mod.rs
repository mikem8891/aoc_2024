const DAY_NUM: &str = "3";

use std::fmt::Display;

fn solve(input: &str) -> (impl Display, impl Display) {

    let len = input.len();
    let mut i = 0;

    loop {
        let sub_str = &input[i..];
        if let Some(is) = sub_str.find("mul(") {
            let sub_str = &input[is..];
            if let Some(ie) = &sub_str[0..12].find(')') {
                let mut args = &sub_str[4..*ie].split(',');
                let args = (*args.next(), args.next());
                match args {
                    (Some(s1), Some(s2)) => {},
                    _ => break
                }
            } else {break}
        } else {break}
    }

    ("todo", "todo")
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = crate::util::stopwatch(|| solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;
