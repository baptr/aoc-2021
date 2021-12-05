use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use itertools::Itertools;

struct Board {
    grid: Vec<Vec<u8>>,
    done: bool,
    rows: [u8; 5],
    cols: [u8; 5],
}

impl Board {
    fn new() -> Board {
        return Board{
            grid: Vec::<Vec<u8>>::new(),
            done: false,
            rows: [0; 5],
            cols: [0; 5],
        };
    }

    fn add_row(&mut self, v: Vec<u8>) {
        self.grid.push(v);
    }

    fn call(&mut self, n: u8) -> bool {
        if self.done {
            return false;
        }
        for (j, r) in self.grid.iter().enumerate() {
            for (i, v) in r.iter().enumerate() {
                if *v == n {
                    self.rows[j] |= 1<<i;
                    self.cols[i] |= 1<<j;
                    self.done = self.rows[j] == 0b11111 || self.cols[i] == 0b11111;
                    return self.done;
                }
            }
        }
        return false;
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for (j, mask) in self.rows.iter().enumerate() {
            for (i, v) in self.grid[j].iter().enumerate() {
                if mask & (1<<i) == 0 {
                    sum += *v as u32;
                }
            }
        }
        return sum
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut draws: Vec<u8> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    for line in reader.lines() {
        let l = line?;
        if draws.len() == 0 {
            draws = l.split(",").map(|v| u8::from_str_radix(&v, 10).unwrap()).collect();
            continue;
        }
        if l.is_empty() {
            boards.push(Board::new());
            continue;
        }
        let row = l.split_whitespace().map(|v| u8::from_str_radix(&v, 10).unwrap()).collect();
        boards.last_mut().unwrap().add_row(row);
    }

    println!("draws[0]: {} boards.len(): {}", draws[0], boards.len());
    for d in draws {
        for b in boards.iter_mut() {
            if b.call(d) {
                println!("bingo! after {}, score: {}, answer: {}", d, b.score(), d as u32 * b.score());
            }
        }
    }

    Ok(())
}
