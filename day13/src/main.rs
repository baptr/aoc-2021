use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp;
//use std::option::Option;
//use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::collections::BinaryHeap;
//use std::collections::HashMap;
//use itertools::Itertools;

const Y_DEBUG:bool = false;
const X_DEBUG:bool = false;

fn dump(grid: &Vec<Vec<bool>>) {
    let h = 3;
    for (i, row) in grid.iter().enumerate() {
        print!("{:width$}: ", i, width=h);
        for v in row {
            if *v {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn fold(grid: Vec<Vec<bool>>, crease: &(String, usize)) -> Vec<Vec<bool>> {
    println!("doing fold {}={}, grid height={}", crease.0, crease.1, grid.len());
    let mut out = Vec::new();
    if crease.0 == "y" {
        for i in 0..crease.1 {
            let mut row = Vec::new();
            let a = &grid[i];
            let b = &grid[grid.len()-1-i];
            let m = cmp::max(a.len(), b.len());
            for j in 0..m {
                let mut v = false;
                if j < a.len() {
                    v |= a[j];
                }
                if j < b.len() {
                    v |= b[j];
                }
                row.push(v);
            }
            if Y_DEBUG {
                println!("folding y={},{}:", i, grid.len()-1-i);
                for v in a {
                    print!("{}", if *v { "#" } else { " " });
                }
                println!();
                for v in b {
                    print!("{}", if *v { "#" } else { " " });
                }
                println!();
                for v in &row {
                    print!("{}", if *v { "#" } else { " " });
                }
                println!();
            }
            out.push(row);
        }
    } else {
        for old in grid {
            let mut row = Vec::new();
            let end = cmp::min(crease.1, old.len());
            row.extend(&old[..end]);
            for a in crease.1+1..old.len() {
                let b = old.len()-1-a;
                row[b] |= old[a];
            }
            if X_DEBUG {
                println!("folding:");
                for (i, v) in old.iter().enumerate() {
                    if i == crease.1 {
                        print!("|");
                    } else {
                        print!("{}", if *v { "#" } else { "." });
                    }
                }
                println!();
                for v in &row {
                    print!("{}", if *v { "#" } else { " " });
                }
                println!();
            }
            out.push(row);
        }
    }
    dump(&out);
    return out;
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut grid = Vec::new();
    let mut folds = Vec::new();
    let mut read_folds = false;
    let mut max_y = 0;
    let mut max_x = 0;
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            read_folds = true;
            continue;
        }
        if !read_folds {
            let (a, b) = l.split_once(',').unwrap();
            let x = a.parse::<usize>().unwrap();
            if x > max_x { max_x = x; }
            let y = b.parse::<usize>().unwrap();
            if y > max_y { max_y = y; }
            if y >= grid.len() {
                grid.resize(y+1,Vec::new());
            }
            if x >= grid[y].len() {
                grid[y].resize(x+1, false);
            }
            grid[y][x] = true;
        } else {
            let p = l.split_whitespace().last().unwrap();
            let (dir, pos) = p.split_once('=').unwrap();
            folds.push((dir.to_string(), pos.parse::<usize>().unwrap()));
        }
    }
    println!("max_y={} max_x={}", max_y, max_x);
    for row in &mut grid {
        row.resize(max_x+1, false);
    }

    println!("grid height: {} fold len: {}", grid.len(), folds.len());
    dump(&grid);

    grid = fold(grid, &folds[0]);
    let mut part1 = 0;
    for row in &grid {
        for v in row {
            if *v {
                part1 += 1;
            }
        }
    }
    println!("part1: {}", part1);

    for f in &folds[1..] {
        grid = fold(grid, &f);
    }

    println!("part2:");
    for row in &grid {
        for v in row {
            if *v {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    Ok(())
}
