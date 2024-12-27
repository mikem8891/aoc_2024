const DAY_NUM: &str = "4";

use std::fmt::Display;

const XMAS_W: &str = 
"
XMAS
";
const XMAS_NW: &str = 
"
X
.M
..A
...S
";
const XMAS_N: &str = 
"
X
M
A
S
";
const XMAS_NE: &str = 
"
...X
..M
.A
S
";
const XMAS_E: &str = 
"
SAMX
";
const XMAS_SE: &str = 
"
S
.A
..M
...X
";
const XMAS_S: &str = 
"
S
A
M
X
";
const XMAS_SW: &str = 
"
...S
..A
.M
X
";
const XMAS_WORDS: [&str, 8] = [
    XMAS_W, XMAS_NW, XMAS_N, XMAS_NE,
    XMAS_E, XMAS_SE, XMAS_S, XMAS_SW
]

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
                    row_size = row_size.max(row + 1);
                    col_size = col_size.max(col + 1);
                }
            }
        }
        let size = (row_size, col_size);
        Word {letters, size}
    }

    fn is_in_at(&self, word_search: &[&[u8]], (row_loc, col_loc): (usize, usize)) -> bool {
        self.letters.iter().all(|(l, (r, c))| *l == word_search[row_loc + r][col_loc + c])
    }

    fn count_in(&self, word_search: &[&[u8]]) -> u32{
        let mut count = 0;
        let (size_row, size_col) = self.size;
        let (search_rows, search_cols) = 
            (word_search.len() - size_row, word_search[0].len() - size_col)
        for r in 0..=search_rows {
            for c in 0..=search_cols {
                if self.is_in_at(word_search, (r, c)) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let word_search: Vec<_> = input.lines().map(str::as_bytes).collect();
    let sum = XMAS_WORDS.iter()
        .map(|w| Word::new(w).count_in(word_search)).sum();
    
    
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
