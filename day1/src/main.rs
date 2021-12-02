use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut v = vec![];
    let mut count = 0;
    for line in reader.lines() {
        let i = line?.parse::<i32>().unwrap();
        v.push(i);
        if v.len() > 3 {
            if i > v.remove(0) {
                count += 1;
            }
        }
    }
    println!("increases: {}", count);

    Ok(())
}
