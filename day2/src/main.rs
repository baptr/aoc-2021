use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut h = 0;
    let mut d = 0;
    for line in reader.lines() {
        let l = line?;
        let (dir, dist_str) = l.split_whitespace().next_tuple().unwrap();
        let dist = dist_str.parse::<i32>().unwrap();
        match dir {
            "forward" => h += dist,
            "up" => d -= dist,
            "down" => d += dist,
            _ => println!("Unhandled direction {}", dir),
        }
    }
    println!("forward: {} depth: {} mult: {}", h, d, h*d);

    Ok(())
}
