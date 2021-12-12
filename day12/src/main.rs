use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::option::Option;
//use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
//use std::collections::BinaryHeap;
use std::collections::HashMap;
//use itertools::Itertools;

struct Cave {
    neighbors: Vec<String>
}

struct Path {
    doubled: bool,
    steps: Vec<String>
}

impl Path {
    fn step(&self, n: &String) -> Option<Path> {
        if n == "start" { return None; }
        let mut doubled = self.doubled;
        if &n[0..1]>="a" && self.steps.contains(n) {
            if doubled {
                return None;
            }
            doubled = true;
        }
        let mut p = Path{
            doubled: doubled,
            steps: self.steps.clone(),
        };
        p.steps.push(n.clone());
        return Some(p);
    }
}

fn explore(caves: &HashMap<String,Cave>, path: Path) -> Vec<Vec<String>> {
    let mut out:Vec<Vec<String>> = Vec::new();
    let tail = caves.get(path.steps.last().unwrap()).unwrap();
    for n in &tail.neighbors {
        let p = path.step(&n);
        if p.is_none() { continue; }
        let p = p.unwrap();
        if n == "end" {
            out.push(p.steps);
        } else {
            let got = explore(caves, p);
            out.extend(got);
        }
    }
    return out
}

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).expect("missing input filename");
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    let mut caves = HashMap::new();
    for line in reader.lines() {
        let l = String::from(line?).clone();
        let (a, b) = l.split_once('-').unwrap();
        let a = a.to_string();
        let b = b.to_string();
        {
            if !caves.contains_key(&a) {
                caves.insert(a.clone(), Cave{neighbors: Vec::new()});
            }
            let first = caves.get_mut(&a).unwrap();
            first.neighbors.push(b.clone());
        }

        {
            if !caves.contains_key(&b) {
                caves.insert(b.clone(), Cave{neighbors: Vec::new()});
            }
            let second = caves.get_mut(&b).unwrap();
            second.neighbors.push(a.clone());
        }
    }

    let paths = explore(&caves, Path{doubled: false, steps: vec!["start".to_string()]});
    let part1 = paths.len();
    for p in &paths {
        println!("{:?}", p);
    }
    println!("part1: {}", part1);

    Ok(())
}
