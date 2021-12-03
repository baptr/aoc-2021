use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use itertools::Itertools;

fn high(vec: &Vec<u32>, pos: i32) -> Vec<u32> {
    let half: usize = vec.len()/2;
    let det = vec[half] >> pos;
    let mut out: Vec<u32> = Vec::new();
    for v in vec {
        if (v>>pos) == det {
            out.push(*v);
        }
    }
    return out;
}

fn low(vec: &Vec<u32>, pos: i32) -> Vec<u32> {
    let half: usize = vec.len()/2;
    let det = (vec[half] >> pos) ^ 1;
    let mut out: Vec<u32> = Vec::new();
    for v in vec {
        if (v>>pos) == det {
            out.push(*v);
        }
    }
    return out;
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut vec: Vec<u32> = Vec::new();
    let mut width: i32 = 0;
    for line in reader.lines() {
        let l = line?;
        if width == 0 {
            width = l.len() as i32;
        }
        vec.push(u32::from_str_radix(&l, 2).unwrap());
    }
    vec.sort();

    let mut pos = width-1;
    let mut h = high(&vec, pos);
    while h.len() > 1 {
        pos-=1;
        h = high(&h, pos);
    }
    pos = width-1;
    let mut l = low(&vec, pos);
    while l.len() > 1 {
        pos-=1;
        l = low(&l, pos);
    }
    println!("high: {} low: {} h*l: {}", h[0], l[0], h[0]*l[0]);

    Ok(())
}
