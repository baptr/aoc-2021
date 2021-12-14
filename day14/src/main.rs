use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::cmp;
//use std::option::Option;
//use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::collections::BinaryHeap;
use std::collections::HashMap;
//use itertools::Itertools;

fn step(old: &String, rules: &HashMap<String, String>) -> String {
    let mut out = String::new();
    for i in 0..old.len()-1 {
        let k = &old[i..i+2];
        //let v = rules.get(k);
        out.extend(old[i..i+1].chars());
        let v = rules.get(k);
        if v.is_some() {
            out.extend(v.unwrap().chars());
        }
    }
    out.extend(old[old.len()-1..].chars());
    return out;
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let base = lines.next().unwrap().unwrap().to_string();
    lines.next();
    let mut rules = HashMap::new();
    for line in lines {
        let l = line?;
        let (a, b) = l.split_once(" -> ").unwrap();
        rules.insert(a.to_string(), b.to_string());
    }
    println!("base={} rules.len={}", base, rules.len());

    let mut vals = base.clone();
    for i in 1..=10 {
        vals = step(&vals, &rules);
        println!("After step {} len={}: {}", i, vals.len(), vals);
    }

    let mut counts = HashMap::new();
    for c in vals.chars() {
        match counts.get_mut(&c) {
            Some(v) => *v += 1,
            None => {counts.insert(c, 1); ()},
        };
    }
    println!("counts: {:?}", counts);

    let mut min = 2147483647;
    let mut max = 0;
    for (_, v) in counts {
        if v < min { min = v }
        if v > max { max = v }
    }
    println!("max={} - min={} = {}", max, min, max-min);

    Ok(())
}
