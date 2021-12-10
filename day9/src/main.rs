use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
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
    let mut low_pos = Vec::new();
    for (j, row) in grid.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if i > 0 && *v >= row[i-1] { continue; }
            if i < row.len()-1 && *v >= row[i+1] { continue; }
            if j > 0 && *v >= grid[j-1][i] { continue; }
            if j < grid.len()-1 && *v >= grid[j+1][i] { continue; }
            low_pos.push((j, i));
            part1 += (*v+1) as u64;
        }
    }

    println!("part1 total: {}", part1);

    let mut basin_size = Vec::new();
    for start in low_pos {
        let mut seen:HashSet<(usize,usize)> = HashSet::new();
        let mut pending = vec![start];
        let mut size = 0;
        while !pending.is_empty() {
            size += 1;
            let (j, i) = pending.pop().expect("failed pop");
            let mut add = |p: (usize, usize)| { if !seen.contains(&p) { pending.push(p); seen.insert(p); } };
            if i > 0 && grid[j][i-1] < 9 { add((j, i-1)); }
            if i < grid[j].len()-1 && grid[j][i+1] < 9 { add((j, i+1)); }
            if j > 0 && grid[j-1][i] < 9 { add((j-1, i)); }
            if j < grid.len()-1 && grid[j+1][i] < 9 { add((j+1, i)); }
        }
        println!("basin from grid[{}][{}] is size={}", start.0, start.1, size);
        basin_size.push(size-1);
    }
    basin_size.sort();
    let l = basin_size.len();
    let part2 = basin_size[l-1] * basin_size[l-2] * basin_size[l-3];
    println!("part2 total: {}", part2);

    Ok(())
}
