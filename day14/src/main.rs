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

fn apply(pair: &String, steps: usize, rules: &HashMap<String,String>, memo: &mut HashMap<&String,Vec<HashMap<String,u64>>>) -> HashMap<String,u64> {
    {
        let seen = &memo.get(pair).unwrap()[steps];
        if seen.len() > 0 {
            return seen.clone();
        }
    }

    let v = rules.get(pair).unwrap();
    if steps == 0 {
        let out = &mut memo.get_mut(pair).unwrap()[steps];
        out.insert(v.to_string(), 1);
        return out.clone();
    }

    let a = pair[0..1].to_string() + v;
    let b = v.clone() + &pair[1..2].to_string();

    let mut out = HashMap::new();
    out.insert(v.to_string(), 1);

    let left = apply(&a, steps-1, rules, memo);
    for (k, v) in left {
        match out.get_mut(&k) {
            Some(o) => *o += v,
            None => {out.insert(k.to_string(), v); ()},
        }
    }

    let right = apply(&b, steps-1, rules, memo);
    for (k, v) in right {
        match out.get_mut(&k) {
            Some(o) => *o += v,
            None => {out.insert(k.to_string(), v); ()},
        }
    }
    memo.get_mut(pair).unwrap()[steps] = out.clone();
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

    let steps = 40;
    let mut memo = HashMap::new();
    for (k, _) in &rules {
        let mut m = Vec::new();
        m.resize(steps, HashMap::<String,u64>::new());
        memo.insert(k, m);
    }

    let mut counts = HashMap::new();
    for i in 0..base.len()-1 {
        let part = apply(&base[i..i+2].to_string(), steps-1, &rules, &mut memo);
        for (k, v) in part {
            match counts.get_mut(&k) {
                Some(o) => *o += v,
                None => {counts.insert(k.to_string(), v); ()},
            }
        }
    }
    for i in 0..base.len() {
        let k = &base[i..i+1];
        match counts.get_mut(k) {
            Some(o) => *o += 1,
            None => {counts.insert(k.to_string(), 1); ()},
        }
    }
    println!("{:?}", counts);

    let mut min = 99999999999999;
    let mut max = 0;
    for (_, v) in counts {
        if v < min { min = v }
        if v > max { max = v }
    }
    println!("max={} - min={} = {}", max, min, max-min);

    Ok(())
}
