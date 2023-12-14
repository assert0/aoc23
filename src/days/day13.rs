use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::iproduct;
use std::iter::zip;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Pattern {
    map: Vec<Vec<char>>,
}

impl FromStr for Pattern {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            map: s.lines().map(|l| l.chars().collect()).collect(),
        })
    }

}

impl Pattern {

    fn valid_fold_row(&self, row: usize) -> bool {
        let mut dist: usize = 0;
        loop {
            if (row as isize - dist as isize) < 0 || row + dist + 1 >= self.map.len() {
                break;
            }
            if self.map[row - dist] != self.map[row + dist + 1] {
                return false;
            }
            dist += 1; 
        }
        dist > 0
    }

    fn valid_fold_row2(&self, row: usize) -> bool {
        let mut dist: usize = 0;
        let mut diffcount = 0;
        loop {
            if (row as isize - dist as isize) < 0 || row + dist + 1 >= self.map.len() {
                break;
            }
            diffcount += zip(self.map[row - dist].iter(), self.map[row + dist + 1].iter())
                            .filter(|(a, b)| a != b).count();
            if diffcount > 1 {
                return false;
            }
            dist += 1; 
        }
        dist > 0 && diffcount == 1
    }

    fn transform(&self) -> Pattern {
        let mut map = vec![vec!['_'; self.map.len()]; self.map[0].len()];
        iproduct!(0..self.map.len(), 0..self.map[0].len())
            .for_each(|(y, x)| map[x][y] = self.map[y][x]);
        Pattern { map }
    }

    fn mirror_row(&self) -> Option<usize> {
        (0..self.map.len()).filter(|i| self.valid_fold_row(*i)).next()
    }

    fn mirror_row2(&self) -> Option<usize> {
        (0..self.map.len()).filter(|i| self.valid_fold_row2(*i)).next()
    }
}

pub fn day13(args: &[String]) {
    println!("Day 13");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    // Part 1
    let patterns: Vec<Pattern> = contents.split("\n\n")
                    .map(|p| p.parse().unwrap()).collect();

    let part1: usize = patterns.iter().map(|p| {
            match p.mirror_row() {
                Some(v) => (v + 1) * 100,
                None => {
                    match p.transform().mirror_row() {
                        Some(v) => v + 1,
                        None => unreachable!()
                    }
                }
            }
        }).sum();
    println!("Part 1: {}", part1);

    let part2: usize = patterns.iter().map(|p| {
            match p.mirror_row2() {
                Some(v) => (v + 1) * 100,
                None => {
                    match p.transform().mirror_row2() {
                        Some(v) => v + 1,
                        None => unreachable!()
                    }
                }
            }
        }).sum();
    println!("Part 2: {}", part2);
}
