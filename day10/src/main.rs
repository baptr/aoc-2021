use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::collections::HashSet;
//use itertools::Itertools;

fn is_terminal(l: &str) -> bool {
    match l {
        ")"|"]"|"}"|">" => true,
        _ => false,
    }
}

fn parse(l: &str, open: &str) -> (usize, u64, u64) {
    if is_terminal(open) { return (1, 0, 0); }
    let (close, complete_points) = match open {
        "(" => (")", 1),
        "[" => ("]", 2),
        "{" => ("}", 3),
        "<" => (">", 4),
        "EOL" => ("EOL", 0),
        _ => {
            println!("Invalid open char in {} : {}", open, l);
            return (0, 0, 0);
        },
    };
    println!("{} : {}: looking for {}", open, l, close);

    let mut pos = 0;
    let mut child_score = 0;
    loop {
        if pos+1 > l.len() { // handle EOL
            println!("{} : {}: - exhausted @ {}, adding {} for {} points", open, l, pos, close, complete_points);
            if complete_points == 0 {
                return  (0, 0, child_score);
            }
            return (pos+1, 0, complete_points+child_score*5);
        }
        let next = &l[pos..pos+1];
        if is_terminal(next) {
            if next != close {
                let score = match next {
                    ")" => 3,
                    "]" => 57,
                    "}" => 1197,
                    ">" => 25137,
                    _ => 0,
                };
                println!("{} : {}: expected {} but found {} instead : {})", open, l, close, next, score);
                return (pos+1, score, 0);
            }
            // matched pop
            println!("{} : {}: found {}, popping {}", open, l, next, pos+1);
            return (pos+1, 0, 0);
        }

        let (skip, sub_part1, sub_part2) = parse(&l[pos+1..], next);
        println!("{} : {}: recurse at +{}={} returned: skip={} sub_part1={} sub_part2={}", open, l, pos, next, skip, sub_part1, sub_part2);
        if sub_part1 > 0 {
            return (pos+skip, sub_part1, sub_part2);
        }
        child_score = 5*child_score + sub_part2;
        pos += skip + 1;
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let (skip, incomplete_score, complete_score) = parse(&l, "EOL");
        println!("{} done: skip={} fail={} finish={}", l, skip, incomplete_score, complete_score);
        part1 += incomplete_score;
        if complete_score > 0 {
            part2.push(complete_score);
        }
    }
    part2.sort();

    println!("part1 total: {}", part1);
    println!("part2 median: {}", part2[part2.len()/2]);
    Ok(())
}
