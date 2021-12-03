use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut vec: Vec<i32> = Vec::new();
    let mut count = 0;
    for line in reader.lines() {
        let l = line?;
        let cv: Vec<char> = l.chars().collect();
        for (i, c) in cv.iter().enumerate() {
            if i >= vec.len() {
                vec.push(0);
            }
            if *c == '1' {
                vec[i] += 1;
            }
        }
        count+=1;
    }
    println!("vec: {:#?} count: {}", vec, count);
    
    let mut a = 0;
    let mut b = 0;
    for v in vec {
        a <<= 1;
        b <<= 1;
        if v < count/2 {
            a |= 1;
        } else {
            b |= 1;
        }
    }
    println!("a={} b={} a*b={}", a, b, a*b);

    Ok(())
}
