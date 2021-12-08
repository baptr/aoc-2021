use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
//use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;
    for line in reader.lines() {
        let l = line?;
        let sections : Vec<&str> = l.split(" | ").collect();
        let input : Vec<&str> = sections[0].split(" ").collect();
        let output : Vec<&str> = sections[1].split(" ").collect();

        let mut one = HashSet::new();
        let mut four = HashSet::new();
        let mut seven = HashSet::new();
        let mut eight = HashSet::new();
        let mut twothreefive = Vec::new();
        let mut zerosixnine = Vec::new();
        for v in input.iter() {
            let set: HashSet<_> = v.chars().into_iter().collect();
            match v.len() {
                2 => one = set,
                3 => seven = set,
                4 => four = set,
                5 => twothreefive.push(set),
                6 => zerosixnine.push(set),
                7 => eight = set,
                _ => continue,
            }
        }

        let top = &seven - &one; // the segment of 7 not in 1 is the top
        let mut three = HashSet::new();
        for (i, v) in twothreefive.iter().enumerate() {
            // 2 3 and 5 all have 5 segments, 3 is the one containing 7
            if v.is_superset(&seven) {
                three = v.clone();
                twothreefive.remove(i);
                break;
            }
        }
        let top_left = &four - &three; // the segment of 4 not in 3 is top left
        let middle = &(&four - &one) - &top_left; // the remaining segment of 4 not in 1/7 is middle
        let _bottom = &(&(&three - &top) - &middle) - &one; // the unknown segment of 3 is bottom

        // 0 6 and 9 are all 6 segments
        let zero = &eight - &middle; // 0 is all but middle
        let mut nine = HashSet::new();
        let mut six = HashSet::new();
        for v in zerosixnine.iter() {
            if v == &zero { continue; }
            // 9 has both of 1
            if v.is_superset(&one) { nine = v.clone(); continue; }
            // 6 is the remaining 6 segment
            six = v.clone();
        }
        // the missing segment of 6 is the top right
        let top_right = &eight - &six;
        // leaving the other from 1 as bottom right
        let _bottom_right = &one - &top_right;

        // top vs bottom right should determine 2 vs 5
        let mut two = HashSet::new();
        let mut five = HashSet::new();
        for v in twothreefive.iter() {
            if v.is_superset(&top_right) {
                two = v.clone();
            } else {
                five = v.clone();
            }
        }

        //println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", zero, one, two, three, four, five, six, seven, eight, nine);
        
        let mut out = 0;
        for v in output.iter() {
            out *= 10;
            let set: HashSet<_> = v.chars().into_iter().collect();
            if set == one { out += 1; }
            if set == two { out += 2; }
            if set == three { out += 3; }
            if set == four { out += 4; }
            if set == five { out += 5; }
            if set == six { out += 6; }
            if set == seven { out += 7; }
            if set == eight { out += 8; }
            if set == nine { out += 9; }
            match v.len() {
                2 => part1 += 1, // 1
                3 => part1 += 1, // 7
                4 => part1 += 1, // 4
                7 => part1 += 1, // 8
                _ => continue,
            }
        }
        part2 += out;
        println!("output: {}", out);
    }

    println!("part1 count: {}", part1);
    println!("part2 total: {}", part2);

    Ok(())
}
