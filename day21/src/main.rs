use std::env;
//use std::fs::File;
//use std::io::BufReader;
//use std::io::prelude::*;
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


fn main() -> std::io::Result<()> {
    let a_start = env::args().nth(1).expect("missing position 1").parse::<u32>().unwrap()-1;
    let b_start = env::args().nth(2).expect("missing position 1").parse::<u32>().unwrap()-1;

    let mut a_pos = a_start;
    let mut b_pos = b_start;
    let mut rolls = 0;
    let mut a_score = 0;
    let mut b_score = 0;

    let mut die_start = 1;

    loop {
        a_pos = (a_pos+die_start*3+3) % 10;
        a_score += a_pos+1;
        die_start = (die_start+2)%100 + 1;
        rolls += 3;
        if a_score >= 1000 {
            println!("a win = {}", b_score * rolls);
            break;
        }
        b_pos = (b_pos+die_start*3+3) % 10;
        b_score += b_pos+1;
        die_start = (die_start+2)%100 + 1;
        rolls += 3;
        if b_score >= 1000 {
            println!("b win = {}", a_score * rolls);
            break;
        }
    }

    Ok(())
}
