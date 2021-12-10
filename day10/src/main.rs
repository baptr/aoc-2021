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

fn parse(l: &str, open: &str) -> (usize, i64) {
    println!("{} : {}: start", open, l);
    if l.len() == 0 {
        return (0, 0);
    }
    if is_terminal(open) { return (1, -1); }
    let close = match open {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        "<" => ">",
        "EOL" => "EOL",
        _ => return (0, 0),
    };
    let mut pos = 0;
    loop {
        // XXX handle eol
        if pos+1 >= l.len() {
            println!("{} : {}: - exhausted @ {}", open, l, pos);
            return (pos+1, 0);
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
                return (pos+1, score);
            }
            // matched pop
            println!("{} : {}: found {}, popping {}", open, l, next, pos+1);
            return (pos+1, -1);
        }

        let (skip, sub_score) = parse(&l[pos+1..], next);
        println!("{} : {}: recurse at +{}={} returned: skip={} sub_score={}", open, l, pos, next, skip, sub_score);
        if sub_score != -1 {
            return (pos+skip, sub_score);
        }
        pos += skip + 1;
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    for line in reader.lines() {
        let l = line?;
        let (skip, score) = parse(&l, "EOL");
        println!("{} {}", skip, score);
        part1 += score;
    }

    println!("part1 total: {}", part1);
    Ok(())
}
