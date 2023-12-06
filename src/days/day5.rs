use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;
use regex::Regex;
use std::ops::Range;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedMap {
    source: String,
    destination: String,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl FromStr for SeedMap {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+)\-to\-(\w+) map:").unwrap();
        }
        let mut lines = s.lines();
        let caps = RE.captures(lines.next().unwrap()).unwrap();
        Ok(Self {
            source: caps[1].to_string(),
            destination: caps[2].to_string(),
            ranges: lines.map(|l| parse_range(l)).collect(),
        })
    }
}

impl SeedMap {

    fn mapped_to(&self, source_number: u64) -> u64 {
        match self.ranges.iter().filter(|(_, sr)| sr.contains(&source_number)).next() {
            Some((dr, sr)) => dr.start + source_number - sr.start,
            None => source_number
        }
    }

}

fn parse_range(s: &str) -> (Range<u64>, Range<u64>) {
    let (drs, srs, rl) = s.split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .next_tuple().unwrap();
    (Range { start: drs, end: drs + rl }, Range { start: srs, end: srs + rl })
}

pub fn day5(args: &[String]) {
    println!("Day 5");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut sections = contents.split("\n\n");

    let seeds: Vec<u64> = sections.next().unwrap()
            .split(":").last().unwrap()
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap()).collect();

    let maps: Vec<_> = sections.map(|s| s.parse::<SeedMap>().unwrap()).collect();

    let part1 = seeds.clone().into_iter().map(|s| maps.iter().fold(s, | acc, m | m.mapped_to(acc)))
            .min().unwrap();
    println!("Part 1: {}", part1);
  
    let seeds2: Vec<_> = seeds.into_iter().tuples().map(|(s, l)| Range { start: s, end: s + l}).collect();

    let mut lowest = u64::MAX;
    for s2 in seeds2.into_iter() {
        for s in s2 {
            let t = maps.iter().fold(s, | acc, m | m.mapped_to(acc));
            if t < lowest {
                lowest = t;
                println!("{}", lowest);
            }
        }
    }
    println!("Part 2: {}", lowest);
}
