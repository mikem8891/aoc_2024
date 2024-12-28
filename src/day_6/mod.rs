const DAY_NUM: &str = "6";

use std::{fmt::Display, ops::{Deref, DerefMut}};

enum Direction {
    Up, Right, Down, Left
}

struct Gaurd {
    row: usize,
    col: usize,
    dir: Direction
}

impl Gaurd {
    fn new(){}

    fn walk<'a, T: DerefMut<Target = &'a mut [u8]>>(&mut self, map: &mut [T]) {
        match self.dir {
            Direction::Up => {
                if map[self.row - 1][self.col] == b'#' {
                    self.dir = Direction::Right;
                } else {
                    self.row -= 1;
                    map[self.row][self.col] = b'X';
                }
            },
            Direction::Right => {
                if map[self.row][self.col + 1] == b'#' {
                    self.dir = Direction::Right;
                } else {
                    self.row -= 1;
                    map[self.row][self.col] = b'X';
                }
            },
            Direction::Down => todo!(),
            Direction::Left => todo!(),
        }
    }
}

fn find_start<'a, T: Deref<Target = &'a[u8]>>(map: &[T]) -> (usize, usize) {
    for (row, ref line) in map.iter().enumerate() {
        if let Some(col) = line.iter().position(|c| *c == b'^') {
            return (row, col);
        }
    }
    panic!("couldn't find the start");
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let mut map: Vec<_> = input.lines().map(|s| Box::from(s.as_bytes())).collect();
    let (row, col) = find_start(&*map);
    use Direction as Dir;
    let dir = Dir::Up;
    let gaurd = Gaurd {row, col, dir};
    
    'walking: loop {

    }


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
