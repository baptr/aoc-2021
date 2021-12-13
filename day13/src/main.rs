use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::cmp;
//use std::option::Option;
//use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::collections::BinaryHeap;
//use std::collections::HashMap;
//use itertools::Itertools;

fn dump(grid: &Vec<Vec<bool>>) {
    let h = 3;
    for (i, row) in grid.iter().enumerate() {
        print!("{:width$}: ", i, width=h);
        for v in row {
            if *v {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut points = Vec::new();
    let mut read_folds = false;
    let mut last_x = 0;
    let mut last_y = 0;
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            read_folds = true;
            continue;
        }
        if !read_folds {
            let (a, b) = l.split_once(',').unwrap();
            let x = a.parse::<usize>().unwrap();
            let y = b.parse::<usize>().unwrap();
            points.push((x, y));
        } else {
            let p = l.split_whitespace().last().unwrap();
            let (dir, pos) = p.split_once('=').unwrap();
            let v = pos.parse::<usize>().unwrap();
            if dir == "x" {
                last_x = v;
            } else {
                last_y = v;
            }
        }
    }
    println!("last_x={} last_y={}", last_x, last_y);

    let mut grid = Vec::new();
    grid.resize(last_y, Vec::new());
    for row in &mut grid {
        row.resize(last_x, false);
    }

    for (x, y) in points {
        let x_seg = (x+1) / (last_x+1);
        let mut i = (x+1) % (last_x+1) - 1;
        if x_seg % 2 == 1 { i = last_x-i-1; }

        let y_seg = (y+1) / (last_y+1);
        let mut j = (y+1) % (last_y+1) - 1;
        if y_seg % 2 == 1 { j = last_y-j-1; }

        grid[j][i] = true;
    }

    dump(&grid);

    Ok(())
}
