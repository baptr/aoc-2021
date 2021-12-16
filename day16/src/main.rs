use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp;
//use std::option::Option;
//use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::collections::BinaryHeap;
//use std::collections::BTreeMap;
//use std::collections::HashMap;
//use itertools::Itertools;

use bitstream_io::{BitReader, BigEndian, BitRead};

#[derive(Debug)]
enum Payload {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
}

//type Reader = BitReader<&[u8], BigEndian>;

fn parse_literal(r: &mut BitReader<&[u8], BigEndian>) -> (Payload, usize) {
    let mut read = 0;
    let mut out = 0u64;
    loop {
        let cont = r.read::<u8>(1).unwrap();
        out <<= 4;
        out |= r.read::<u8>(4).unwrap() as u64;
        read += 5;
        if cont == 0 { break }
    }
    (Payload::Literal(out), read)
}

impl Packet {
    fn new(r: &mut BitReader<&[u8], BigEndian>) -> (Packet, usize) {
        let v = r.read::<u8>(3).unwrap();
        let t = r.read::<u8>(3).unwrap();
        let mut read = 6;

        if t == 4 { // literal
            let (p, r) = parse_literal(r);
            read += r;
            return (Packet{
                version: v,
                type_id: t,
                payload: p,
            }, read);
        }

        let mut sub = Vec::new();
        let st = r.read::<u8>(1).unwrap();
        read += 1;
        if st == 0 {
            let mut bit_len = r.read::<u16>(15).unwrap() as isize;
            read += 15;
            while bit_len > 0 {
                let (p, r) = Packet::new(r);
                bit_len -= r as isize;
                read += r;
                sub.push(p);
            }
        } else {
            let num_sub = r.read::<u16>(11).unwrap();
            read += 11;
            for _ in 0..num_sub {
                let (p, r) = Packet::new(r);
                read += r;
                sub.push(p);
            }
        }

        return (Packet{
            version: v,
            type_id: t,
            payload: Payload::SubPackets(sub),
        }, read);
    }
}

fn part1(p: &Packet) -> u64 {
    let mut v = p.version as u64;
    match &p.payload {
        Payload::Literal(_) => (),
        Payload::SubPackets(children) => {
            for sp in children {
                v += part1(sp);
            }
        },
    }
    return v;
}

fn part2(p :&Packet) -> u64 {
    return match &p.payload {
    Payload::Literal(l) => *l,
    Payload::SubPackets(children) => {
        let mut sub_vals = Vec::new();
        for c in children {
            sub_vals.push(part2(c));
        }
        return match p.type_id {
            0 => sub_vals.iter().fold(0, |acc, x| acc + x),
            1 => sub_vals.iter().fold(1, |acc, x| acc * x),
            2 => sub_vals.iter().fold(u64::MAX, |acc, x| cmp::min(acc, *x)),
            3 => sub_vals.iter().fold(0, |acc, x| cmp::max(acc, *x)),
            5 => if sub_vals[0] > sub_vals[1] { 1 } else { 0 },
            6 => if sub_vals[0] < sub_vals[1] { 1 } else { 0 },
            7 => if sub_vals[0] == sub_vals[1] { 1 } else { 0 },
            _ => u64::MAX,
        };
    }
    };
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut bytes = Vec::new();
    for line in lines {
        let l = line?;
        let mut c_iter = l.chars();
        loop {
            let c = c_iter.next();
            if c.is_none() {
                break;
            }
            let mut v = c.unwrap().to_string();

            let d = c_iter.next();
            if c.is_none() {
                v.push('0');
            } else {
                v.push(d.unwrap());
            }
            bytes.push(u8::from_str_radix(&v, 16).expect("invalid hex"));
        }
    }
    println!("bytes={:?}", bytes);

    let mut r = BitReader::endian(bytes.as_slice(), BigEndian);
    let (p, _used) = Packet::new(&mut r);
    println!("packet={:?}", p);

    println!("part1={}", part1(&p));
    println!("part2={}", part2(&p));

    Ok(())
}
