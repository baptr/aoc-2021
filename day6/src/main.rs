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
        println!("after day {}: {}, {}, {}, {}, {}, {}, {}, {}", day, next[0], next[1], next[2], next[3], next[4], next[5], next[6], next[7]);
        school = next;
    }
    println!("total fish: {}", school[0]+school[1]+school[2]+school[3]+school[4]+school[5]+school[6]+school[7]+school[8]);

    Ok(())
}
