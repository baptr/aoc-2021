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

type Pos = (i16, i16, i16);

trait Permutable {
    fn perm(&self, axis: u8, neg: u8) -> Self;
}

impl Permutable for Pos {
    // axis 0-5 neg 0-3
    fn perm(&self, axis: u8, neg: u8) -> Pos {
        let a = self.0;
        let b = self.1;
        let c = self.2;
        let mut out = (a, b, c);
        match axis {
            0 => out = (a, b, c),
            1 => out = (-a, -b, c),
            2 => out = (b, -a, c),
            3 => out = (-b, a, c),
            4 => out = (c, b, -a),
            5 => out = (-c, b, a),
            _ => panic!("invalid axis"),
        }
        let x = out.0;
        let y = out.1;
        let z = out.2;
        match neg {
            0 => out = (x, y, z),
            1 => out = (x, z, -y),
            2 => out = (x, -z, y),
            3 => out = (x, -y, -z),
            _ => panic!("invalid neg"),
        }
        return out;
    }
}

fn overlap(_a: &Vec<Pos>, b: &Vec<Pos>) -> (u8, Vec<Pos>) {
    for axis in 0..6 {
        for neg in 0 .. 4{
            let p = b[0].perm(axis, neg);
            println!("axis {} neg {}: {:?}", axis, neg, p);
        }
    }
    return (0, Vec::new());
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
            scanners.push(Vec::new());
            len += 1;
            continue;
        }
        let i:Vec<i16> = l.split(",").map(|v| v.parse::<i16>().unwrap()).collect();
        scanners[len-1].push((i[0], i[1], i[2]));
    }

    println!("{:?}", scanners[0]);
    //overlap(&scanners[0], &scanners[1]);

    let test = vec![(1i16, -2i16, 3i16)];
    overlap(&test, &test);

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
