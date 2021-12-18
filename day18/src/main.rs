use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::boxed::Box;
//use std::cmp;
use std::option::Option;
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

#[derive(Debug)]
enum Element {
    Number(u8),
    Pair(Box<Pair>),
}

impl Element {
    fn would_explode(&self, depth: u8) -> (u8, bool, u8) {
        if depth >= 3 {
            if let Element::Pair(p) = self {
                if let Element::Number(l) = p.left {
                    if let Element::Number(r) = p.right {
                        return (l, true, r);
                    }
                }
                panic!("exploded non-number pair {:?}", self);
            }
        }
        return match self {
            Element::Number(_) => (0, false, 0),
            Element::Pair(p) => {
                let (ll, lb, _) = p.left.would_explode(depth+1);
                if lb {
                    return (ll, true, 0);
                }
                let (_, rb, rr) = p.right.would_explode(depth+1);
                if rb {
                    return (0, true, rr);
                }
                return (0, false, 0);
            }
        }
    }

    fn clone(&self) -> Element {
        return match self {
            Element::Number(n) => Element::Number(*n),
            Element::Pair(p) => Element::Pair(Box::new(Pair{left: p.left.clone(), right:p.right.clone()})),
        };
    }

    fn carry_left(&self, carry: u8) -> Element {
        return match self {
            Element::Number(n) => Element::Number(*n + carry),
            Element::Pair(p) => Element::Pair(Box::new(Pair{
                left: p.left.carry_left(carry),
                right: p.right.clone(),
            })),
        }
    }

    fn carry_right(&self, carry: u8) -> Element {
        return match self {
            Element::Number(n) => Element::Number(*n + carry),
            Element::Pair(p) => Element::Pair(Box::new(Pair{
                left: p.left.clone(),
                right: p.right.carry_right(carry),
            })),
        }
    }

    fn split(&self) -> Element {
        if let Element::Number(n) = self {
            let l = ((*n as f32)/2.0).floor();
            let r = ((*n as f32)/2.0).ceil();
            return Element::Pair(Box::new(Pair{
                left: Element::Number(l as u8),
                right: Element::Number(r as u8),
            }));
        } else {
            panic!("split pair {:?}", self);
        }
    }

    fn to_string(&self) -> String {
        return match self {
            Element::Number(n) => format!("{}", *n),
            Element::Pair(p) => p.to_string(),
        };
    }
}

#[derive(Debug)]
struct Pair {
    left: Element,
    right: Element,
}

impl Pair {
    fn new(ch: &mut std::str::Chars) -> Pair {
        let l = ch.next().unwrap(); 
        let left: Element;
        if l == '[' {
            left = Element::Pair(Box::new(Pair::new(ch)));
        } else {
            left = Element::Number(l as u8 - b'0');
        }
        let c = ch.next().unwrap();
        if c != ',' { println!("expected ',' got '{}' rem: {:?}", c, ch) }

        let r = ch.next().unwrap();
        let right: Element;
        if r == '[' {
            right = Element::Pair(Box::new(Pair::new(ch)));
        } else {
            right = Element::Number(r as u8 - b'0');
        }

        let e = ch.next().unwrap();
        if e != ']' { println!("expected ']' got '{}' rem: {:?}", e, ch) }

        return Pair{left, right};
    }

    fn add(self, other: Self) -> Pair {
        let mut p = Pair{
            left: Element::Pair(Box::new(self)),
            right: Element::Pair(Box::new(other)),
        };

        loop {
            let (v, b) = p.reduce();
            if b {
                //println!("** red={}", v.to_string());
                p = v;
            } else {
                break;
            }
        }
        return p;
    }

    fn magnitude(&self) -> u64 {
        let mut out = 0u64;
        match &self.left {
            Element::Number(n) => out += 3*(*n as u64),
            Element::Pair(p) => out += 3*p.magnitude(),
        }
        match &self.right {
            Element::Number(n) => out += 2*(*n as u64),
            Element::Pair(p) => out += 2*p.magnitude(),
        }
        return out;
    }

    fn explode(&self, depth: u8) -> (Pair, bool) {
        let (_, lb, lr) = self.left.would_explode(depth);
        if lb {
            if depth >= 3 {
                return (Pair{left: Element::Number(0), right: self.right.carry_left(lr)}, true);
            }
            if let Element::Pair(lp) = &self.left {
                let (p, b) = lp.explode(depth+1);
                assert!(b);
                return (Pair{left: Element::Pair(Box::new(p)), right: self.right.carry_left(lr)}, b);
            } else {
                panic!("exploded non-pair");
            }
        }

        let (rl, rb, _) = self.right.would_explode(depth);
        if rb {
            if depth >= 3 {
                return (Pair{left: self.left.carry_right(rl), right: Element::Number(0)}, true);
            }
            if let Element::Pair(rp) = &self.right {
                let (p, b) = rp.explode(depth+1);
                assert!(b);
                return (Pair{left: self.left.carry_right(rl), right: Element::Pair(Box::new(p))}, b);
            } else {
                panic!("exploded non-pair");
            }
        }

        return (Pair{left: self.left.clone(), right: self.right.clone()}, false);
    }

    fn split(&self) -> (Pair, bool) {
        let mut split = false;
        let l:Element = match &self.left {
            Element::Number(n) => {
                if *n >= 10 {
                    split = true;
                    self.left.split()
                } else {
                    Element::Number(*n)
                }
            },
            Element::Pair(p) => {
                let (s, b) = p.split();
                split = b;
                Element::Pair(Box::new(s))
            }
        };
        if split {
            return (Pair{left: l, right: self.right.clone()}, true);
        }
        let r:Element = match &self.right {
            Element::Number(n) => {
                if *n >= 10 {
                    split = true;
                    self.right.split()
                } else {
                    Element::Number(*n)
                }
            },
            Element::Pair(p) => {
                let (s, b) = p.split();
                split = b;
                Element::Pair(Box::new(s))
            }
        };
        return (Pair{left: l, right: r}, split);
    }

    fn reduce(&self) -> (Pair, bool) {
        let (p, b) = self.explode(0);
        if b {
            return (p, b);
        }

        return self.split();
    }

    fn to_string(&self) -> String {
        format!("[{},{}]", self.left.to_string(), self.right.to_string())
    }

    fn clone(&self) -> Pair {
        return Pair{left: self.left.clone(), right: self.right.clone()};
    }
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut vals = Vec::new();
    for line in lines {
        let l = line?;
        let mut c_iter = l.chars();
        c_iter.next(); // leading [
        let new = Pair::new(&mut c_iter);
        vals.push(new);
    }

    let mut max = 0;
    for (i, a) in vals.iter().enumerate() {
        for (j, b) in vals.iter().enumerate() {
            if i == j { continue }
            let v = a.clone().add(b.clone());
            let m = v.magnitude();
            if m > max {
                max = m;
            }
        }
    }

    println!("part2={}", max);

    Ok(())
}
