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
use std::collections::HashMap;
//use std::collections::HashSet;
//use itertools::Itertools;

//use bitstream_io::{BitReader, BigEndian, BitRead};

struct State {
    idx: u8, // 0 based
    score: u16,
    turns: u8,
    univ: u64,
}

// possible steps each turn = 3,4,5,6,7,8,9 (with uneven probability)
// from score=0, pos=7, what's the distribution of *turns* to reach score>=21?
// p3 = p9 = 1/27
// p4 = p8 = 3/27
// p5 = p7 = 6/27
// p6      = 7/27
fn turn_prob(init_idx: u8) -> HashMap<u8, u64> {
    let rolls = vec![1,3,6,7,6,3,1];
    let mut state = vec![State{idx: init_idx, score: 0, turns: 0, univ: 1}];
    let mut done = Vec::new();
    while !state.is_empty() {
        let mut next = Vec::new();
        for s in &state {
            for (r, p) in rolls.iter().enumerate() {
                let idx = (s.idx + r as u8 + 3)%10;
                let score = s.score + idx as u16 + 1;
                let s = State{idx: idx, score: score, turns: s.turns+1, univ: s.univ*p};
                if score >= 21 {
                    done.push(s);
                } else {
                    next.push(s);
                }
            }
        }
        state = next;
    }
    println!("{} end states", done.len());

    let mut turn_dist = HashMap::new();
    for s in &done {
        match turn_dist.get_mut(&s.turns) {
            Some(v) => *v += s.univ,
            None => {turn_dist.insert(s.turns, s.univ); ()},
        }
    }
    println!("turn dist: {:?}", turn_dist);
    return turn_dist;
}

fn main() -> std::io::Result<()> {
    let a_start = env::args().nth(1).expect("missing position 1").parse::<u32>().unwrap()-1;
    let b_start = env::args().nth(2).expect("missing position 2").parse::<u32>().unwrap()-1;

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
            println!("part1: a win = {}", b_score * rolls);
            break;
        }
        b_pos = (b_pos+die_start*3+3) % 10;
        b_score += b_pos+1;
        die_start = (die_start+2)%100 + 1;
        rolls += 3;
        if b_score >= 1000 {
            println!("part1: b win = {}", a_score * rolls);
            break;
        }
    }

    println!("processing player 1 evolution...");
    let a_turns = turn_prob(a_start as u8);
    println!("processing player 2 evolution...");
    let b_turns = turn_prob(b_start as u8);

    let possible = 27u128;
    let mut a_wins = 0u128;
    let mut a_rem = 1u128;
    let mut b_wins = 0u128;
    let mut b_rem = 1u128;
    for t in 1..=10 {
        {
            let w = match a_turns.get(&t) {
                Some(v) => *v as u128,
                None => 0,
            };
            a_wins += w * b_rem;
            a_rem = a_rem*possible - w;
        }
        println!("after t={} a_wins={} a_rem={}", t, a_wins, a_rem);
        if a_rem == 0 { break }
        { 
            let w = match b_turns.get(&t) {
                Some(v) => *v as u128,
                None => 0,
            };
            b_wins += w * a_rem;
            b_rem = b_rem*possible - w;
        }
        println!("after t={} b_wins={} b_rem={}", t, b_wins, b_rem);
        if b_rem == 0 { break }
    }

    Ok(())
}
