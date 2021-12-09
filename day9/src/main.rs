use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::collections::HashSet;
//use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let mut row = Vec::new();
        for c in l.as_bytes() {
            row.push(c-b'0');
        }
        grid.push(row);
    }

    let mut part1:u64 = 0;
    for (j, row) in grid.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if i > 0 && *v >= row[i-1] { continue; }
            if i < row.len()-1 && *v >= row[i+1] { continue; }
            if j > 0 && *v >= grid[j-1][i] { continue; }
            if j < grid.len()-1 && *v >= grid[j+1][i] { continue; }
            //println!("grid[{}][{}] = {} is low", j, i, *v);
            part1 += (*v+1) as u64;
        }
    }

    println!("part1 total: {}", part1);

    Ok(())
}
