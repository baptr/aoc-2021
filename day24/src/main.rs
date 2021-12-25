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

#[derive(Clone,Debug)]
enum Op {
    Inp(usize),
    Add(Box<Op>, Box<Op>),
    Mul(Box<Op>, Box<Op>),
    Div(Box<Op>, Box<Op>),
    Mod(Box<Op>, Box<Op>),
    Eql(Box<Op>, Box<Op>),
    Not(Box<Op>),
    Lit(IV),
    Ref(usize, usize),
}

type IV = i128;

impl Op {
    fn is_zero(&self) -> bool {
        match self {
            Op::Lit(v) => return *v == 0,
            _ => return false,
        }
    }

    fn eval(&self, input: &Vec<IV>, hist: &Vec<Vec<Op>>) -> IV {
        match self {
            Op::Lit(l) => *l,
            Op::Inp(i) => input[*i],
            Op::Add(l, r) => l.eval(input, hist) + r.eval(input, hist),
            Op::Mul(l, r) => l.eval(input, hist) * r.eval(input, hist),
            Op::Div(l, r) => l.eval(input, hist) / r.eval(input, hist),
            Op::Mod(l, r) => l.eval(input, hist) % r.eval(input, hist),
            Op::Eql(l, r) => if l.eval(input, hist) == r.eval(input, hist) { 1 } else { 0 },
            Op::Not(v) => if v.eval(input, hist) == 0 { 1 } else { 0 },
            Op::Ref(epoch, var) => hist[*epoch][*var].eval(input, hist),
        }
    }
}


fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut vars = vec![Op::Lit(0), Op::Lit(0), Op::Lit(0), Op::Lit(0)];
    let mut var_hist = Vec::new();
    let mut in_idx = 0;
    let lines = reader.lines();
    for line in lines {
        let l = line?;
        //println!("processing {}", &l);
        let parts: Vec<&str> = l.split(' ').collect();
        let var_idx = (parts[1].chars().next().unwrap() as u8 - b'w') as usize;
        let v = Box::new(vars[var_idx].clone());
        let o2 = if parts.len() > 2 {
            let o2v = parts[2].parse::<IV>();
            if o2v.is_err() {
                let idx = (parts[2].chars().next().unwrap() as u8 - b'w') as usize;
                Box::new(vars[idx].clone())
            } else {
                Box::new(Op::Lit(o2v.unwrap()))
            }
        } else { Box::new(Op::Lit(-128)) };
        match parts[0] {
            "inp" => {
                if in_idx > 0 {
                    var_hist.push(vars.clone());
                    vars[0] = Op::Ref(in_idx-1, 0);
                    vars[1] = Op::Ref(in_idx-1, 1);
                    vars[2] = Op::Ref(in_idx-1, 2);
                    vars[3] = Op::Ref(in_idx-1, 3);
                }
                vars[var_idx] = Op::Inp(in_idx);
                in_idx += 1
            },
            "mul" => {
                vars[var_idx] = if o2.is_zero() {
                    *o2
                } else if v.is_zero() {
                    *v
                } else { Op::Mul(v, o2) }
            },
            "add" => {
                vars[var_idx] = if v.is_zero() {
                    *o2
                } else if o2.is_zero() { 
                    *v
                } else { Op::Add(v, o2) }
            },
            "div" => {
                if parts[2] != "1" && !v.is_zero() {vars[var_idx] = Op::Div(v, o2)}
            },
            "mod" => { 
                vars[var_idx] = if v.is_zero() { *v } else { Op::Mod(v, o2) } },
            "eql" => {
                vars[var_idx] = if o2.is_zero() { Op::Not(v) } else { Op::Eql(v, o2) }
            },
            _ => panic!("unrecognized op code {}", parts[0]),
        }
    }
    //println!("z =\n{:?}", &vars[3]);
    var_hist.push(vars.clone());
    for i in 0..=13 {
        println!("z[{}] = {:?}", i, &var_hist[i][3]);
    }

    println!("\nsolving...");
    let mut from = HashMap::new();
    let mut prev_z = HashSet::new();
    for i in 5..=5 {
        let v = &var_hist[0][3].eval(&vec![i], &var_hist);
        prev_z.insert(*v);
        println!("z[0]@{} = {}", i, &v);
        let mut s = HashSet::new();
        s.insert((i, 0));
        from.insert((0, *v), s);
    }
    for epoc in 1..=13 {
        let mut in_v = Vec::new();
        in_v.resize(14, 0);
        let mut next_z = HashSet::new();
        for z in prev_z {
            let mut hist = var_hist.clone(); // XXX overkill
            hist[epoc-1][3] = Op::Lit(z);
            for i in (1..=9).rev() {
                if epoc == 1 && i != 1 { continue }
                if epoc == 2 && i != 1 { continue }
                if epoc == 3 && i != 2 { continue }
                if epoc == 4 && i != 1 { continue }
                if epoc == 5 && i != 1 { continue }
                /*
                if epoc == 6 && i != 9 { continue }
                if epoc == 7 && i != 9 { continue }
                if epoc == 8 && i < 9 { continue }
                */
                in_v[epoc] = i;
                let v = &var_hist[epoc][3].eval(&in_v, &hist);
                if epoc < 13 || epoc == 13 && *v == 0 {
                    // XXX might need either, but only for the last digit, I think
                    //println!("z[{}]@{},z_-1={} = {}", epoc, i, z, v);
                }
                next_z.insert(*v);
                match from.get_mut(&(epoc, *v)) {
                    None => {
                        let mut s = HashSet::new();
                        s.insert((i, z));
                        from.insert((epoc, *v), s); ()},
                    Some(c) => {c.insert((i, z)); ()},
                }
            }
        }
        prev_z = next_z;
    }

    let mut out = walk(&from, 13, 0);
    out.sort();
    println!("part2 = {}", out[0]);

    let mut epoch_szs = Vec::new();
    let mut prev = HashSet::new();
    for v in from.get(&(13, 0)).unwrap() {
        prev.insert(v.1);
    }
    epoch_szs.push(prev.clone());
    println!("epoch {} zs {:?}", 13, &prev);
    for e in (1..13).rev() {
        let mut next = HashSet::new();
        let mut digs = HashSet::new();
        for z in &prev {
            for v in from.get(&(e, *z)).unwrap() {
                digs.insert(v.0);
                next.insert(v.1);
            }
        }
        println!("epoch {} zs {:?} form digs {:?}", e, &next, &digs);
        prev = next;
        epoch_szs.push(prev.clone());
    }

    Ok(())
}

fn walk(from: &HashMap<(usize, IV), HashSet<(IV, IV)>>, epoch: usize, z: IV) -> Vec<u64> {
    let mut out = Vec::new();
    for prev in from.get(&(epoch, z)).unwrap() {
        let base = prev.0 as u64 * 10u64.pow(13-epoch as u32);
        println!("epoch={} z={} came from: {:?} base={}", epoch, z, prev, base);
        if epoch > 0 {
            out.extend(walk(from, epoch-1, prev.1).iter().map(|v| base+v));
        } else {
            out.push(base);
        }
    }
    return out;
}
