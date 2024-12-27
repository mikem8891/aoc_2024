const DAY_NUM: &str = "4";

use std::fmt::Display;

const XMAS_FORWARD: &str = 
"
XMAS
";

struct Word {
    letters: Vec<(u8, (usize, usize))>,
    size: (usize, usize)
}

impl Word {
    fn new(word: &str) -> Self {
        let mut letters = vec![];
        let (mut row_size, mut col_size) = (0, 0);
        let word: Vec<_> = word.lines()
            .map(str::as_bytes).collect();
        for (row, line) in word.iter().enumerate() {
            for (col, &letter) in line.iter().enumerate() {
                if letter != b'.' {
                    letters.push((letter, (row, col)));
                    row_size = row_size.max(row);
                    col_size = col_size.max(col);
                }
            }
        }
        let size = (row_size, col_size);
        Word {letters, size}
    }

    
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let word_search: Vec<_> = input.lines().map(str::as_bytes).collect();
    
    
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
