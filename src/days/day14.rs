use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::iproduct;
use std::collections::HashMap;


#[derive(Clone, Eq, PartialEq, Debug)]
struct Platform {
    map: Vec<Vec<char>>,
}

impl FromStr for Platform {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            map: s.lines().map(|l| l.chars().collect()).collect(),
        })
    }
}

impl Platform {

    fn rotate(&mut self) {
        let mut map = vec![vec!['_'; self.map.len()]; self.map[0].len()];
        let h = self.map.len() - 1;
        iproduct!(0..self.map.len(), 0..self.map[0].len())
            .for_each(|(y, x)| map[x][h - y] = self.map[y][x]);
        self.map = map;
    }

    fn tilt(&mut self) {
        for x in 0..self.map[0].len() {
            let mut block = 0;
            for y in 0..self.map.len() {
                if self.map[y][x] == 'O' && block < y {
                    self.map[block][x] = 'O';
                    self.map[y][x] = '.';
                    block += 1;
                }
                match self.map[y][x] {
                    'O' | '#' => block = y + 1,
                    _ => (), 
                }
            }
        }
    }

    fn score(&self) -> usize {
        let mut total = 0;
        for (i, y) in (0..self.map.len()).rev().enumerate() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == 'O' {
                    total += i + 1;
                }
            }
        }
        total
    }
   
}

pub fn day14(args: &[String]) {
    println!("Day 14");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    // Part 1
    let mut platform: Platform = contents.parse().unwrap();
    platform.tilt();
    println!("Part 1: {}", platform.score());
    
    // Part 2
    platform = contents.parse().unwrap();
    let mut loop_start = 0;
    let mut loop_size = 0;
    let mut count_pos = HashMap::new();
    let mut scores = vec![];
    let mut count_nochange = 0;
    for r in 0.. {
        for _ in 0..4 {
            platform.tilt();
            platform.rotate();
        }
        let s = platform.score();
        // detect the repeating pattern
        scores.push(s);
        if count_pos.contains_key(&s) {
            let d = r - count_pos.remove(&s).unwrap();
            if d > loop_size {
                loop_size = d;
                count_nochange = 0;
            } else {
                count_nochange += 1;
            }
            // println!("{} | {} {} = {}", loop_size, r, s, d);
        }
        if count_nochange > 100 {
            let mut loop_detected = true;
            for i in 0..loop_size {
                if scores[r - i] != scores[r - i - loop_size] {
                    loop_detected = false;
                    break;
                }
            }
            if loop_detected {
                loop_start = r - loop_size;
                println!("Loop start: {} size : {}", loop_start, loop_size);
                break;
            }
        }
        count_pos.insert(s, r);
    }
    let offset = (1_000_000_000 - 1 - loop_start) % loop_size;
    let pattern = &scores[loop_start..loop_start+loop_size];

    println!("Part 2: {}", pattern[offset]); 
}
