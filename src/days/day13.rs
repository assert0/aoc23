use std::fs;
use itertools::iproduct;
use std::iter::zip;

type Map = Vec<Vec<char>>;

trait Score {

    fn transform(&self, map: &Map) -> Map {
        let mut map2 = vec![vec!['_'; map.len()]; map[0].len()];
        iproduct!(0..map.len(), 0..map[0].len())
            .for_each(|(y, x)| map2[x][y] = map[y][x]);
        map2
    }

    fn valid_fold_row(&self, rmap: &Map, row: usize) -> bool;

    fn mirror_row(&self, map: &Map) -> Option<usize> {
        (0..map.len()).filter(|i| self.valid_fold_row(map, *i)).next()
    }

    fn score(&self, map: &Map) -> usize {
        match self.mirror_row(map) {
            Some(v) => (v + 1) * 100,
            None => {
                match self.mirror_row(&self.transform(map)) {
                    Some(v) => v + 1,
                    None => unreachable!()
                }
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Part1 {
    map: Map
}

impl Score for Part1 {

    fn valid_fold_row(&self, map: &Map, row: usize) -> bool {
        let mut dist: usize = 0;
        loop {
            if (row as isize - dist as isize) < 0 || row + dist + 1 >= map.len() {
                break;
            }
            if map[row - dist] != map[row + dist + 1] {
                return false;
            }
            dist += 1; 
        }
        dist > 0
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Part2 {
    map: Map
}

impl Score for Part2 {

    fn valid_fold_row(&self, map: &Map, row: usize) -> bool {
        let mut dist: usize = 0;
        let mut diffcount = 0;
        loop {
            if (row as isize - dist as isize) < 0 || row + dist + 1 >= map.len() {
                break;
            }
            diffcount += zip(map[row - dist].iter(), map[row + dist + 1].iter())
                            .filter(|(a, b)| a != b).count();
            if diffcount > 1 {
                return false;
            }
            dist += 1; 
        }
        dist > 0 && diffcount == 1
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
    
    let patterns: Vec<Vec<Vec<char>>> = contents.split("\n\n")
                    .map(|p| p.lines()
                        .map(|l| l.chars().collect()).collect()
                    ).collect();

    // Part 1
    let part1: usize = patterns.iter().map(|p| {
            let pattern = Part1 { map: p.to_vec() };
            pattern.score(&pattern.map)
        }).sum();
    println!("Part 1: {}", part1);

    // Part 2
    let part2: usize = patterns.iter().map(|p| {
        let pattern = Part2 { map: p.to_vec() };
        pattern.score(&pattern.map)
    }).sum();
    println!("Part 2: {}", part2);

}
