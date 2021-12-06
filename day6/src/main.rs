use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut school = [0 as u64; 9];
    for line in reader.lines() {
        let l = line?;
        let list : Vec<u64> = l.split(",").map(|v| u64::from_str_radix(&v, 10).unwrap()).collect();
        for v in list.iter() {
            school[*v as usize] += 1;
        }
    }
    for day in 1..=256 {
        let mut next = [0 as u64; 9];
        for i in 0..=8 {
            if i == 0 {
                next[6] += school[0];
                next[8] += school[0];
            } else {
                next[i-1] += school[i];
            }
        }
        println!("after day {}: {:?}", day, next);
        school = next;
    }
    let mut total = 0;
    for v in school {
        total += v;
    }
    println!("total fish: {}", total);

    Ok(())
}
