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
    fn new<'a, T: Deref<Target = &'a[u8]>>(map: &[T]) -> Gaurd{
        let (row, col) = find_start(&*map);
        let dir = Direction::Up;
        Gaurd {row, col, dir}
    }

    fn step_thru<'a, T: DerefMut<Target = &'a mut [u8]>>(&mut self, map: &mut [T]) {
        use Direction as Dir;
        
        const OBS: Option<u8> = Some(b'#');
        
        let (row, col) = (self.row, self.col);
        map[row][col] = b'X';
        
        let map_get = |row, col| map.get(row).and_then(|r| r.get(col));
        
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

fn find_start<'a, T: Deref<Target = &'a[u8]>>(map: &[T]) -> (usize, usize) {
    for (row, ref line) in map.iter().enumerate() {
        if let Some(col) = line.iter().position(|c| *c == b'^') {
            return (row, col);
        }
    }
    panic!("couldn't find the start");
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let mut map: Vec<_> = input.lines()
        .map(|s| Box::from(s.as_bytes()))
        .collect();
    let gaurd = Gaurd::new(&*map);
    
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
