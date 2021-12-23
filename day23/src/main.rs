//use std::env;
//use std::fs::File;
//use std::io::BufReader;
//use std::io::prelude::*;
//use std::boxed::Box;
//use std::cmp;
use std::option::Option;
use std::cmp::Ordering;
//use std::cell::Cell;
//use std::cell::RefCell;
//use std::rc::Rc;
//use std::rc::Weak;
use std::collections::BinaryHeap;
use std::collections::BTreeMap;
//use std::collections::BTreeSet;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use itertools::Itertools;

//use bitstream_io::{BitReader, BigEndian, BitRead};

#[derive(Debug,PartialEq,Eq,Clone,Copy,PartialOrd,Ord)]
enum Cell {
    A, B, C, D,
    Empty,
}

impl Cell {
    fn cost(&self, dist: i32) -> u32 {
        return dist as u32 * match self {
            Cell::A => 1,
            Cell::B => 10,
            Cell::C => 100,
            Cell::D => 1000,
            _ => panic!("calculated cost for empty cell"),
        };
    }
}

#[derive(Debug,PartialEq,Eq,Clone)]
struct State {
    cost: u32,
    hall: [Cell; 7], // ##_#_#_#_##
    rooms: [[Cell; 4]; 4], // [door..back]
}

impl Ord for State {
    fn cmp(&self, o: &Self) -> Ordering {
        (o.cost, o.hall, o.rooms).cmp(&(self.cost, self.hall, self.rooms))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

const HALL_POS:[i32;7] = [0, 1, 3, 5, 7, 9, 10];
const ROOM_POS:[i32;4] = [2, 4, 6, 8];

impl State {
    fn blocked(&self, room_idx: usize, hall_idx: usize) -> bool {
        if hall_idx < room_idx+2 { // going left
            for i in hall_idx+1..room_idx+2 {
                if self.hall[i] != Cell::Empty { return true; }
            }
        } else { // going right
            for i in room_idx+2..hall_idx {
                if self.hall[i] != Cell::Empty { return true; }
            }
        }
        return false;
    }

    fn done(&self) -> bool {
        for (i, r) in self.rooms.iter().enumerate() {
            for v in r {
                if *v as usize != i { return false }
            }
        }
        return true;
    }

    fn next(&self) -> Vec<Self> {
        let mut out = Vec::new();
        
        let mut room_open = [false, false, false, false];
        for (i, r) in self.rooms.iter().enumerate() {
            let mut ok = true;
            for v in r {
                if *v != Cell::Empty && *v as usize != i {
                    ok = false;
                    break;
                }
            }
            room_open[i] = ok;
        }

        // If any hall cells contain pods that can move home, do that.
        for (i, c) in self.hall.iter().enumerate() {
            let c_idx = *c as usize;
            if *c != Cell::Empty && room_open[c_idx] && !self.blocked(c_idx, i) {
                let mut hall = self.hall.clone();
                hall[i] = Cell::Empty;

                let mut dist = (ROOM_POS[c_idx] - HALL_POS[i]).abs();
                let mut rooms = self.rooms.clone();
                for r_slot in (0..=3).rev() {
                    if rooms[c_idx][r_slot] == Cell::Empty {
                        rooms[c_idx][r_slot] = *c;
                        dist += 1+r_slot as i32;
                        break;
                    }
                }
                //println!("moving {:?} from hall[{}] = {} to room[{}] = {} is {} steps for {} cost. now: {:?}",
                         //*c, i, HALL_POS[i], c_idx, ROOM_POS[c_idx], dist, c.cost(dist), rooms);
                out.push(State{
                    cost: self.cost + c.cost(dist),
                    hall,
                    rooms,
                });
            }
        }

        // Pick each outer most pod not already home and try each valid position: home if possible,
        // or any open hall cell.
        // TODO: Could save some iterations by checking if a pod in the wrong room and move
        // directly home
        for (i, r) in self.rooms.iter().enumerate() {
            let mut room_done = true;
            let mut room_vacant = true;
            let mut src = 0;
            for v in r {
                if *v != Cell::Empty {
                    room_vacant = false;
                }
                if *v as usize != i {
                    room_done = false;
                }
                if room_vacant { 
                    src+=1;
                }
            }
            if room_done { continue }
            if room_vacant { continue }

            let c = r[src];
            for (h, s) in self.hall.iter().enumerate() {
                if *s != Cell::Empty { continue }
                if self.blocked(i, h) { continue }

                let mut hall = self.hall.clone();
                hall[h] = r[src];

                let mut rooms = self.rooms.clone();
                rooms[i][src] = Cell::Empty;

                let dist = (ROOM_POS[i] - HALL_POS[h]).abs() + 1 + src as i32;
                //println!("moving {:?} from room[{}] = {} to hall[{}] = {} is {} steps for {} cost",
                         //c, i, ROOM_POS[i], h, HALL_POS[h], dist, c.cost(dist));
                out.push(State{
                    cost: self.cost + c.cost(dist),
                    hall,
                    rooms,
                });
            }
        }

        return out;
    }
}


fn main() -> std::io::Result<()> {
    /*
    let a_start = env::args().nth(1).expect("missing position 1").parse::<u32>().unwrap()-1;
    let b_start = env::args().nth(2).expect("missing position 2").parse::<u32>().unwrap()-1;
    */

    let init = State{
        cost: 0,
        hall: [Cell::Empty; 7],
        rooms: [
            [Cell::A, Cell::D, Cell::D, Cell::D],
            [Cell::C, Cell::C, Cell::B, Cell::D],
            [Cell::B, Cell::B, Cell::A, Cell::B],
            [Cell::A, Cell::A, Cell::C, Cell::C],
            /*
            [Cell::B, Cell::D, Cell::D, Cell::A],
            [Cell::C, Cell::C, Cell::B, Cell::D],
            [Cell::B, Cell::B, Cell::A, Cell::C],
            [Cell::D, Cell::A, Cell::C, Cell::A],
            */
        ],
    };

    //let mut end = init.clone(); // hax
    let mut pending = BinaryHeap::new();
    let mut cost = BTreeMap::new();
    //let mut seen = BTreeMap::new();
    //seen.insert(init.clone(), init.clone());
    cost.insert(init.clone(), 0);
    pending.push(init);

    while !pending.is_empty() {
        let p = pending.pop().unwrap();
        //println!("step: {:?}", p);
        if p.done() {
            println!("part1: {}", p.cost);
            //end = p;
            break;
        }
        for r in p.next() {
            let mut check = r.clone();
            check.cost = 0;
            if cost.contains_key(&check) && *cost.get(&check).unwrap() <= r.cost { continue }
            cost.insert(check, r.cost);
            //seen.insert(check, p.clone());
            pending.push(r);
        }
    }

    /*
    println!("Final: {:?}", end);
    while end.cost != 0 {
        end.cost = 0;
        end = seen.get_mut(&end).unwrap().clone();
        println!("Prev: {:?}", end);
    }
    */

    Ok(())
}
