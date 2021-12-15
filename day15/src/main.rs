use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::cmp;
use std::option::Option;
use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
use std::collections::BinaryHeap;
use std::collections::BTreeMap;
//use std::collections::HashMap;
//use itertools::Itertools;

type Pos = (usize, usize);
type Cost = u16;

struct Node {
    cost: Cost,
    pos: Pos,
}

impl Node {
    fn neighbors(&self, bounds: Pos) -> Vec<Pos> {
        let mut out = Vec::new();
        if self.pos.0 > 0 {
            out.push((self.pos.0-1, self.pos.1));
        }
        if self.pos.1 > 0 {
            out.push((self.pos.0, self.pos.1-1));
        }
        if self.pos.0 < bounds.0 {
            out.push((self.pos.0+1, self.pos.1));
        }
        if self.pos.1 < bounds.1 {
            out.push((self.pos.0, self.pos.1+1));
        }
        return out;
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > 0 && other.cost > 0 {
            let c = self.cost.cmp(&other.cost);
            if c != Ordering::Equal {
                return c.reverse();
            }
        }
        return self.pos.cmp(&other.pos).reverse();
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}
impl Eq for Node {} // Not strictly true, but maybe it works out? :-}

type Grid = Vec<Vec<u8>>;

fn manhattan_dist(from: Pos, to: Pos) -> Cost {
    return ((from.0 as isize - to.0 as isize).abs() + (from.1 as isize - to.1 as isize).abs()) as Cost;
}

fn astar(grid: &Grid, est: fn(Pos, Pos) -> Cost) -> Cost {
    let start = (0, 0);
    let end = (grid[0].len()-1, grid.len()-1);

    let mut cost = BTreeMap::new();
    cost.insert(start, 0);

    let mut open = BinaryHeap::new();
    open.push(Node{cost: est(start, end), pos: start});

    while !open.is_empty() {
        let p = open.pop().unwrap();
        if p.pos == end {
            return p.cost;
        }
        let cur_cost = cost.get(&p.pos).unwrap().clone();

        for n in p.neighbors(end) {
            let c = cur_cost + grid[n.1][n.0] as Cost;
            let prev = cost.get(&n);
            if prev.is_none() || c < *prev.unwrap() {
                cost.insert(n, c);
                open.push(Node{cost: c+est(n, end), pos: n});
            }
        }
    }

    return 0;
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut grid = Vec::new();
    for line in lines {
        let l = line?;
        let mut row = Vec::new();
        for c in l.chars() {
            row.push(c.to_string().parse::<u8>().unwrap());
        }
        grid.push(row);
    }
    println!("grid={:?}", grid);

    println!("part1={}", astar(&grid, manhattan_dist));

    // Could do this dynamically, but that seems harder...
    let tile_dim = grid[0].len();
    let width = tile_dim * 5;
    let mut grid2 = Vec::new();
    grid2.resize_with(grid.len()*5, || { let mut v = Vec::new(); v.resize(width, 0u8); v });

    for (j, row) in grid.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            for yx in 0..5 {
                let y = yx*tile_dim + j;
                for xx in 0..5 {
                    let x = xx*tile_dim + i;
                    let c = (((*v as usize + yx + xx)-1) % 9) + 1;
                    grid2[y][x] = c as u8;
                }
            }
        }
    }
    /*
    println!("grid2:");
    for row in &grid2 {
        println!("{:?}", row);
    }
    */
    println!("part2={}", astar(&grid2, manhattan_dist));

    Ok(())
}
