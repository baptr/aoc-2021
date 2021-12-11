use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use itertools::Itertools;

struct Octo {
    energy: Cell<u8>,
    pos: (u8, u8),
    spill: Cell<u8>,
    neighbors: Vec<Weak<RefCell<Octo>>>
}

impl Ord for Octo {
    fn cmp(&self, other: &Self) -> Ordering {
        let e = self.energy.get().cmp(&other.energy.get());
        if e != Ordering::Equal {
            return e;
        }
        return self.pos.cmp(&other.pos);
    }
}

impl PartialOrd for Octo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Octo { }

impl PartialEq for Octo {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let mut row = Vec::new();
        for c in l.as_bytes() {
            row.push(Rc::new(RefCell::new(Octo {
                energy: Cell::new(c-b'0'),
                pos: (grid.len() as u8, row.len() as u8),
                spill: Cell::new(0),
                neighbors: Vec::new(),
            })));
        }
        grid.push(row);
    }

    let mut heap = BinaryHeap::new();
    let height = (grid.len() as isize)-1;
    for (j, row) in grid.iter().enumerate() {
        let width = (row.len() as isize)-1;
        for (i, o) in row.iter().enumerate() {
            let m = j as isize;
            let n = i as isize;
            for y in m-1..=m+1 {
                if y < 0 || y > height { continue; }
                for x in n-1..=n+1 {
                    if x < 0 || x > width { continue; }
                    if m==y && n==x { continue; }
                    o.borrow_mut().neighbors.push(Rc::downgrade(&grid[y as usize][x as usize]));
                }
            }
            heap.push(o);
        }
    }

    let mut part1:u64 = 0;
    for _day in 1..=100 {
        let mut flashed = Vec::new();
        let mut daily = 1;
        loop {
            let mut next = Vec::new();
            while heap.len() > 0 {
                let o_rc = heap.pop().unwrap();
                let o = o_rc.borrow();
                o.energy.set(o.energy.get()+o.spill.get()+daily);
                o.spill.set(0);
                if o.energy.get() > 9 {
                    for n_weak in &o.neighbors {
                        let n_rc = n_weak.upgrade().unwrap();
                        let n = n_rc.borrow();
                        n.spill.set(n.spill.get()+1);
                    }
                    o.energy.set(0);
                    flashed.push(o_rc);
                } else {
                    next.push(o_rc);
                }
                // println!("{:?} now {}", o.pos, o.energy.get());
            }
            daily = 0;
            let mut redo = false;
            for o_rc in &next {
                let o = o_rc.borrow();
                o.energy.set(o.energy.get()+o.spill.get());
                o.spill.set(0);
                if o.energy.get() > 9 {
                    redo = true;
                }
                heap.push(o_rc);
            }
            if !redo {
                for o_rc in &flashed {
                    let o = o_rc.borrow();
                    o.spill.set(0);
                    heap.push(o_rc);
                }
                break;
            }
        }
        println!("day {} ended with {} flashes", _day, flashed.len());
        part1 += flashed.len() as u64;
    }
    println!("part1: {}", part1);

    Ok(())
}
