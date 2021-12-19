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
use std::collections::HashMap;
use std::collections::HashSet;
//use itertools::Itertools;

//use bitstream_io::{BitReader, BigEndian, BitRead};

type Pos = (i16, i16, i16);

trait Permutable {
    fn perm(&self, axis: u8, neg: u8) -> Self;
    fn offset(&self, other: &Self) -> Self;
    fn len2(&self) -> i32;
}

impl Permutable for Pos {
    // axis 0-5 neg 0-3
    fn perm(&self, axis: u8, neg: u8) -> Pos {
        let mut a = self.0;
        let mut b = self.1;
        let mut c = self.2;
        let tmp = match axis {
            0 => (a, b, c),
            1 => (-a, -b, c),
            2 => (b, -a, c),
            3 => (-b, a, c),
            4 => (c, b, -a),
            5 => (-c, b, a),
            _ => panic!("invalid axis"),
        };
        
        a = tmp.0;
        b = tmp.1;
        c = tmp.2;
        return match neg {
            0 => (a, b, c),
            1 => (a, c, -b),
            2 => (a, -c, b),
            3 => (a, -b, -c),
            _ => panic!("invalid neg"),
        };
    }

    fn offset(&self, o: &Pos) -> Pos {
        return (self.0-o.0, self.1-o.1, self.2-o.2);
    }

    fn len2(&self) -> i32 {
        let a = self.0 as i32;
        let b = self.1 as i32;
        let c = self.2 as i32;
        return a*a + b*b + c*c;
    }
}

fn dist_set(input: &HashSet<Pos>) -> HashSet<Pos> {
    let mut out = HashSet::new();
    for (i, a) in input.iter().enumerate() {
        let mut min_dist = i32::MAX;
        let mut min:Pos = (0,0,0);
        for (j, b) in input.iter().enumerate() {
            if i == j { continue }
            let t = a.offset(b);
            let l = t.len2();
            if l < min_dist {
                min_dist = l;
                min = t;
            }
        }
        out.insert(min);
    }
    return out;
}

fn orient(a: &HashSet<Pos>, b: &HashSet<Pos>) -> (usize, HashSet<Pos>) {
    let base = dist_set(a);
    for axis in 0..6 {
        for neg in 0 .. 4{
            let mut s = HashSet::new();
            for p in b {
                s.insert(p.perm(axis, neg));
            }
            let test = dist_set(&s);
            let i: HashSet<_> = base.intersection(&test).collect();
            if i.len() >= 12 {
                println!("axis {} neg {} has {} intersections", axis, neg, i.len());
                return (i.len(), s);
            }
        }
    }
    return (0, HashSet::new());
}

fn find_offset(a: &HashSet<Pos>, b: &HashSet<Pos>) -> Pos {
    let mut counts = HashMap::new();
    for f in a {
        for g in b {
            let o = g.offset(f);
            match counts.get_mut(&o) {
                Some(v) => *v += 1,
                None => {counts.insert(o, 1); ()},
            }
        }
    }
    let mut best = 0;
    let mut best_pos = (0, 0, 0);
    for (k, v) in counts.iter() {
        if *v > best {
            best = *v;
            best_pos = *k;
        }
        if *v >= 12 {
            return *k;
        }
    }
    panic!("no offset, best={:?} with {} matches", best_pos, best);
}

fn merge(a: &HashSet<Pos>, b: &HashSet<Pos>) -> HashSet<Pos> {
    let o = find_offset(a, b);
    let mut out = a.clone();
    for p in b {
        out.insert((p.0-o.0, p.1-o.1, p.2-o.2));
    }
    return out
}

fn shrink(input: &Vec<HashSet<Pos>>) -> Vec<HashSet<Pos>> {
    let mut redo = Vec::new();
    let mut merged = HashSet::new();
    for (i, a) in input.iter().enumerate() {
        let mut out = a.clone();
        let mut parts = HashSet::new();
        parts.insert(i);
        for (j, b) in input.iter().enumerate() {
            if i >= j { continue }
            let (overlap, o) = orient(a, b);
            if overlap >= 12 {
                println!("#{} matches #{} by {}", i, j, overlap);
                out = merge(&out, &o);
                parts.insert(j);
            }
        }
        if !merged.is_superset(&parts) {
            println!("keeping {:?}", parts);
            redo.push(out);
            for p in parts {
                merged.insert(p);
            }
        }
    }
    println!("pass len: {}", redo.len());
    return redo;
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut scanners = Vec::new();
    let mut len = 0;
    for line in lines {
        let l = line?;
        if l.is_empty() { continue }
        if l.starts_with("--- scanner ") {
            scanners.push(HashSet::new());
            len += 1;
            continue;
        }
        let i:Vec<i16> = l.split(",").map(|v| v.parse::<i16>().unwrap()).collect();
        scanners[len-1].insert((i[0], i[1], i[2]));
    }

    println!("{:?}", scanners[0]);

    let mut redo = shrink(&scanners);
    while redo.len() > 1 {
        redo = shrink(&redo);
    }
    let mut part1 = Vec::new();
    for p in &redo[0] {
        part1.push(p);
    }
    part1.sort();
    println!("part1={}", part1.len());
    //for p in part1 { println!("{:?}", p); }

    Ok(())
}

/*
 *  x  y  z
 * -2 -3  1 | l d f
 *  2 -1  3 | r d f
 * -1 -3 -2 | l d b
 *  1  3 -2 | r u b
 *  3  1  2 | r u f
 */
