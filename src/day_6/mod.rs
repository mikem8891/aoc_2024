const DAY_NUM: &str = "6";

use std::{fmt::Display, ops::{Deref, DerefMut}};

enum Direction {
    Up, Right, Down, Left
}

struct Guard {
    row: isize,
    col: isize,
    dir: Direction
}

impl Guard {
    fn new<T: Deref<Target = [u8]>>(map: &[T]) -> Self{
        let (row, col) = find_start(&*map);
        let dir = Direction::Up;
        Guard {row, col, dir}
    }

    fn step_thru<T: DerefMut<Target = [u8]>>(&mut self, map: &mut [T]) {
        use Direction as Dir;
        
        const OBS: Option<u8> = Some(b'#');
        
        let (row, col) = (self.row, self.col);
        map[row as usize][col as usize] = b'X';
        
        let map_get = |row, col| map.get(row as usize).and_then(|r: &T| r.get(col as usize).copied());
        
        match self.dir {
            Dir::Up => {
                if map_get(row - 1, col) == OBS {
                    self.dir = Dir::Right;
                } else {
                    self.row -= 1;
                }
            },
            Dir::Right => {
                if map_get(row, col + 1) == OBS {
                    self.dir = Dir::Down;
                } else {
                    self.col += 1;
                }
            },
            Dir::Down => {
                if map_get(row + 1, col) == OBS {
                    self.dir = Dir::Left;
                } else {
                    self.row += 1;
                }
            },
            Dir::Left => {
                if map_get(row, col - 1) == OBS {
                    self.dir = Dir::Up;
                } else {
                    self.col -= 1;
                }
            }
        }
    }
}

fn find_start<T: Deref<Target = [u8]>>(map: &[T]) -> (isize, isize) {
    for (row, ref line) in map.iter().enumerate() {
        if let Some(col) = line.iter().position(|c| *c == b'^') {
            return (row as isize, col as isize);
        }
    }
    panic!("couldn't find the start");
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let mut map: Vec<_> = input.lines()
        .map(|s| Box::<[u8]>::from(s.as_bytes()))
        .collect();
    let mut guard = Guard::new(&map);
    
    let row_range = 0..map.len() as isize;
    let col_range = 0..map[0].len() as isize;
    
    while row_range.contains(&guard.row) && col_range.contains(&guard.col) {
        guard.step_thru(&mut map);
    }
    
    let dist_pos = map.iter().flatten().filter(|c| **c == b'X').count();

    (dist_pos, "todo")
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let input = include_str!("input.txt");
    let ((part_1, part_2), elaped) = stopwatch!(solve(input));
    println!("[{:.6}s] ({part_1}, {part_2})", elaped.as_secs_f64());
}

#[cfg(test)]
mod test;
