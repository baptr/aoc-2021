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
use std::collections::BTreeMap;
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

fn xsteps() -> (BTreeMap<u32,Vec<u32>>, u32) {
    let tx_min = 150;
    let tx_max = 171;

    let mut step_count = BTreeMap::new();
    let mut max_step = 0;
    step_count.insert(0, Vec::new());
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
        if v0 == 0 {
            if end_steps > max_step {
                max_step = end_steps;
            }
            step_count.get_mut(&0).unwrap().push(vx);
        }
        let mut start_steps = end_steps;
        let mut start = end;
        while start - v0 - 1 >= tx_min {
            v0 += 1;
            start -= v0;
            start_steps -= 1;
        }

        println!("with initial vx={}, we hit pos=[{}, {}] between steps=[{}, {}]", vx, start, end, start_steps, end_steps);
        for s in start_steps..=end_steps {
            match step_count.get_mut(&s) {
                Some(v) => v.push(vx),
                None => {step_count.insert(s, vec![vx]); ()},
            }
        }
    }
    // XXX meeeh ugly hack XXX
    // with initial vx=17, we hit pos=[150, 153] between steps=[15, 17]
    // with initial vx=18, we hit pos=[150, 171] between steps=[12, 18]
    step_count.get_mut(&max_step).unwrap().push(17);
    return (step_count, max_step);
}

fn fall_steps(vi: i32, target: i32, up: bool) -> u32 {
    // pos(i, s) = (2i+1-s)*s/2
    //           = ((2i+1)s - s^2) / 2
    // s^2 - (2i+1)s + 2p = 0
    let _a = 1 as f32;
    let b = -(2*vi+1) as f32;
    let c = (2*target) as f32;

    let s = (-b + (b*b - 4f32*c).sqrt()) / 2f32;
    if up {
        return s.ceil() as u32;
    } else {
        return s.floor() as u32;
    }
}

fn fall_pos(vi: i32, steps: i32) -> i32 {
    return (2*vi - (steps-1)) * steps/2;
}

fn ysteps(xstep_counts: BTreeMap<u32, Vec<u32>>, xv0_step: u32) -> u32 {
    let ty_min:i32 = -129;
    let ty_max:i32 = -70;

    let mut total = 0;
    // upward velocities will return at v+1 downward, so we can do half here
    for vy in ty_min..-ty_min {
        // vy between min and max is trivially 1-step
        // anything less will speed up as it falls
        let min_s = fall_steps(vy, ty_max, true) as i32;
        let max_p = fall_pos(vy, min_s);
        if max_p < ty_min { continue }
        let mut max_s = fall_steps(vy, ty_min, false) as i32;
        let mut min_p = fall_pos(vy, max_s);
        let mut used_x = BTreeSet::new();
        for s in min_s..=max_s {
            if s as u32 > xv0_step {
                for f in &xstep_counts[&0] {
                    used_x.insert(*f);
                }
            } else {
                for f in &xstep_counts[&(s as u32)] {
                    used_x.insert(*f);
                }
            }
        }
        println!("initial vy={} fall_steps=[{}..{}] hits=[{}..{}] matches vx={:?}", vy, min_s, max_s, max_p, min_p, used_x);
        total += used_x.len() as u32;
    }
    return total;
}

fn main() -> std::io::Result<()> {
    // Result:
    // with initial vx=17, we hit pos=[150, 153] between steps=[15, 17] (at v=0)
    // higher velocities fill in all other step values between 1 and 15
    // so x is unbounded.
    let (xes, xv0) = xsteps();
    println!("xes: {:?}", xes);
    
    // For upward trajectories, the velocity will always be the negative initial velocity
    // when passing back through y=0, so the highest velocity we can use is the one that hits the
    // far end of the target y in one tick
    let _part_1_v = 128; // (1 less because the velocity increases after returning to 0)

    let part2 = ysteps(xes, xv0);
    println!("part2={}", part2);

    Ok(())
}
