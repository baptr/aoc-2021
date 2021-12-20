use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::boxed::Box;
//use std::cmp;
//use std::option::Option;
//use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::collections::BinaryHeap;
//use std::collections::BTreeMap;
//use std::collections::BTreeSet;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use itertools::Itertools;

//use bitstream_io::{BitReader, BigEndian, BitRead};

type Image = Vec<Vec<char>>;
type Algo = Vec<char>;

fn pix_val(img: &Image, border: usize, x: isize, y: isize) -> usize {
    if x < 0 || y < 0 || x >= img.len() as isize || y >= img.len() as isize {
        return border;
    }
    return if img[y as usize][x as usize] == '#' { 1 } else { 0 };
}

fn apply(img: &Image, border: char, algo: &Algo) -> (Image, char) {
    let mut out = Vec::new();
    let b_val = if border == '#' { 1 } else { 0 };
    for y in -1 as isize..=img.len() as isize  {
        let mut row = Vec::new();
        for x in -1 as isize..=img.len() as isize {
            let mut det = 0;
            for j in y-1..=y+1 {
                for i in x-1..=x+1 {
                    det = det<<1 | pix_val(img, b_val, i, j);
                }
            }
            row.push(algo[det]);
        }
        out.push(row);
    }
    let out_border = if border == '#' { algo[511] } else { algo[0] };
    return (out, out_border);
}

fn print_img(img: &Image, border: char) {
    println!("** image border={} **", border);
    for row in img {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let algo:Algo = lines.next().unwrap().unwrap().chars().collect();
    lines.next(); // blank
    let mut border = '.';
    let mut img:Image = Vec::new();
    for line in lines {
        let l = line?;
        img.push(l.chars().collect());
    }
    println!("algo={:?}", algo);
    //print_img(&img, border);

    for _idx in 0..50 {
       let (i, b) = apply(&img, border, &algo);
       img = i;
       border = b;
       //print_img(&img, border);
    }
    let mut part1 = 0;
    for row in &img {
        for c in row {
            part1 += if *c == '#' { 1 } else { 0 };
        }
    }
    print_img(&img, border);
    println!("part1={}", part1);


    Ok(())
}
