// use std::env;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::prelude::*;
// use std::cmp;
//use std::option::Option;
//use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::collections::BinaryHeap;
//use std::collections::BTreeMap;
use std::collections::BTreeSet;
//use std::collections::HashMap;
//use itertools::Itertools;
// use bitstream_io::{BitReader, BigEndian, BitRead};

//use roots;

fn damp(v: u32) -> u32 {
    return (v+1)*v/2;
}

fn steps_back(delta: u32) -> u32 {
    // 0         |######|   d
    // (v^2 + v)/2 = d
    // v^2 + v - 2d = 0
    // roots = (-1 +/- sqrt(1^2 - 4*1*-2d)) / 2*1
    //       = (-1 +/- sqrt(1 + 8d))/2
    let root = ((1f32+8f32*delta as f32).sqrt() - 1f32)/2f32;
    return root.ceil() as u32;
}

fn xsteps() {
    let tx_min = 150;
    let tx_max = 171;

    for vx in 1..=tx_max {
        let mut v0 = 0;
        let mut end = damp(vx);
        if end < tx_min { continue; }
        let mut end_steps = vx;
        if end > tx_max {
            let delta = end - tx_max;
            let back = steps_back(delta);
            let p = end-damp(back);
            if p < tx_min { continue; }
            // println!("from initial vx={}, overshoot end={} end_steps={} back={}", vx, end, end_steps, back);
            end = p;
            end_steps -= back;
            v0 = back;
        }
        let mut start_steps = end_steps;
        let mut start = end;
        while start - v0 - 1 >= tx_min {
            v0 += 1;
            start -= v0;
            start_steps -= 1;
        }
        println!("with initial vx={}, we hit pos=[{}, {}] between steps=[{}, {}]", vx, start, end, start_steps, end_steps);

        if end_steps == 2 {
            // 1 step would not let us go up and come down at all.
            // 2's also unlikely, but harder to justify
            break;
        }
    }
}

fn main() -> std::io::Result<()> {
    // Result:
    // with initial vx=17, we hit pos=[150, 153] between steps=[15, 17] (at v=0)
    // higher velocities fill in all other step values between 1 and 15
    // so x is unbounded.
    xsteps();
    
    // For upward trajectories, the velocity will always be the negative initial velocity
    // when passing back through y=0, so the highest velocity we can use is the one that hits the
    // far end of the target y in one tick
    let _part_1_v = 128; // (1 less because the velocity increases after returning to 0)


    Ok(())
}
/*
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut bytes = Vec::new();
    for line in lines {
        let l = line?;
        let mut c_iter = l.chars();
        loop {
            let c = c_iter.next();
            if c.is_none() {
                break;
            }
            let mut v = c.unwrap().to_string();

            let d = c_iter.next();
            if c.is_none() {
                v.push('0');
            } else {
                v.push(d.unwrap());
            }
            bytes.push(u8::from_str_radix(&v, 16).expect("invalid hex"));
        }
    }
    println!("bytes={:?}", bytes);

    let mut r = BitReader::endian(bytes.as_slice(), BigEndian);
    let (p, _used) = Packet::new(&mut r);
    println!("packet={:?}", p);

    println!("part1={}", part1(&p));
    println!("part2={}", part2(&p));

    Ok(())
}
*/
