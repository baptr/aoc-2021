use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use itertools::Itertools;

struct Board {
    grid: Vec<Vec<u16>>,
}

impl Board {
    fn new() -> Board {
        return Board{
            grid: Vec::<Vec<u16>>::new(),
        };
    }

    fn mark_diag(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        if y2 >= self.grid.len() {
            self.grid.resize_with(y2+1, || { Vec::<u16>::new() });
        }
        let mut x:isize = x1 as isize;
        for y in y1..=y2 {
            if x as usize >= self.grid[y].len() {
                self.grid[y].resize(x as usize +1, 0);
            }
            self.grid[y][x as usize] += 1;
            if x1 < x2 {
                x += 1;
            } else {
                x -= 1;
            }
        }
    }

    fn mark(&mut self, v: Vec::<Vec<usize>>) {
        let mut x1 = v[0][0];
        let mut y1 = v[0][1];
        let mut x2 = v[1][0];
        let mut y2 = v[1][1];
        if x1 != x2 && y1 != y2 {
            // XXX Point struct
            if y2 < y1 {
                self.mark_diag(x2, y2, x1, y1);
            } else { 
                self.mark_diag(x1, y1, x2, y2);
            }
            return;
        }
        if x2 < x1 || y2 < y1 {
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut y1, &mut y2);
        }
        if y2 >= self.grid.len() {
            self.grid.resize_with(y2+1, || { Vec::<u16>::new() });
        }
        for y in y1..=y2 {
            if x2 >= self.grid[y].len() {
                self.grid[y].resize(x2+1, 0);
            }
            for x in x1..=x2 {
                self.grid[y][x] += 1;
            }
        }
    }

    fn depth(self, min: u16) -> u32 {
        let mut count : u32 = 0;
        for row in self.grid {
            for v in row {
                if v >= min {
                    count += 1;
                }
            }
        }
        return count;
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut b = Board::new();
    for line in reader.lines() {
        let l = line?;
        let pair = l.split(" -> ").map(|coord| coord.split(",").map(|v| usize::from_str_radix(&v, 10).unwrap()).collect()).collect();
        b.mark(pair);
        //boards.last_mut().unwrap().add_row(row);
    }
    println!("answer: {}", b.depth(2));

    Ok(())
}
