use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
//use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut crabs = HashMap::new();
    let mut min = 0;
    let mut max = 99999;
    for line in reader.lines() {
        let l = line?;
        let list : Vec<i64> = l.split(",").map(|v| i64::from_str_radix(&v, 10).unwrap()).collect();
        for v in list.iter() {
            if *v < min { min = *v; }
            if *v > max { max = *v; }
            let old = crabs.get(v);
            if old == None {
                crabs.insert(*v, 1);
            } else {
                let count = old.unwrap() + 1;
                crabs.insert(*v, count);
            }
        }
    }

    let mut min_cost = 99999999999;
    let mut min_pos = 0;
    for pos in min..=max {
        let mut cost = 0;
        for (start, count) in crabs.iter() {
            let c = (*start-pos).abs();
            cost += count*(c+1)*c/2;
        }
        if cost < min_cost {
            min_cost = cost;
            min_pos = pos;
        }
    }
    println!("moving to {} costs: {}", min_pos, min_cost);
    Ok(())
}
