use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::boxed::Box;
//use std::cmp;
//use std::option::Option;
//use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::collections::BinaryHeap;
//use std::collections::BTreeMap;
//use std::collections::BTreeSet;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use itertools::Itertools;

//use bitstream_io::{BitReader, BigEndian, BitRead};

#[derive(Copy,Clone)]
enum Cumber {
    Empty,
    East,
    South
}

impl Cumber {
    fn is_empty(&self) -> bool {
        return match self {
            Cumber::Empty => true,
            _ => false,
        }
    }
    fn is_east(&self) -> bool {
        return match self {
            Cumber::East => true,
            _ => false,
        }
    }
    fn is_south(&self) -> bool {
        return match self {
            Cumber::South => true,
            _ => false,
        }
    }
}

type Grid = Vec<Vec<Cumber>>;

fn step(grid: &Grid) -> (Grid, bool) {
    let mut east = Vec::new();
    let len = grid.len();
    let mut changed = false;
    for (j, row) in grid.iter().enumerate() {
        let mut out_row = Vec::new();
        for c in row {
            out_row.push(*c);
        }
        let eol = row.len()-1;
        for (i, c) in row.iter().enumerate() {
            match c {
                Cumber::Empty => {
                    if i > 0 && row[i-1].is_east() {
                        out_row[i-1] = Cumber::Empty;
                        out_row[i] = Cumber::East;
                        changed = true;
                    } else if i == 0 && row[eol].is_east() {
                        out_row[0] = Cumber::East;
                        out_row[eol] = Cumber::Empty;
                        changed = true;
                    }
                },
                _ => {},
            }
        }
        east.push(out_row);
    }

    let mut out = Vec::new();
    for r in &east {
        let mut row = Vec::new();
        for c in r {
            row.push(*c);
        }
        out.push(row);
    }
    let eoc = east.len()-1;
    for (j, row) in east.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            match c {
                Cumber::Empty => {
                    if j == 0 && east[eoc][i].is_south() {
                        out[0][i] = Cumber::South;
                        out[eoc][i] = Cumber::Empty;
                        changed = true;
                    } else if j > 0 && east[j-1][i].is_south() {
                        out[j-1][i] = Cumber::Empty;
                        out[j][i] = Cumber::South;
                        changed = true;
                    }
                },
                _ => {},
            }
        }
    }
    return (out, changed);
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut grid = Vec::new();
    let mut width = 0;
    for line in lines {
        let l = line?;
        let mut row = Vec::new();
        for c in l.chars() {
            row.push(match c {
                '>' => Cumber::East,
                'v' => Cumber::South,
                '.' => Cumber::Empty,
                _ => panic!("invalid input character"),
            });
        }
        if width == 0 {
            width = row.len();
        } else if width != row.len() {
            panic!("mismatched row size");
        }
        grid.push(row);
    }
    let mut height = grid.len();

    let mut steps = 0;
    loop {
        let (g, change) = step(&grid);
        if !change { break }
        steps += 1;
        grid = g;
    }
    println!("part1: {}", steps+1);

    Ok(())
}
