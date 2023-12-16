use std::fs;
use itertools::Itertools;
use std::iter::zip;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Spring {
    condition: String,
    counts: Vec<usize>,
}

impl FromStr for Spring {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([#\.\?]+) ([\d\,]+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            condition: caps[1].to_string(),
            counts: caps[2].split(',').map(|c| c.parse().unwrap()).collect()
        })
    }

}

impl Spring {

    fn new(condition: String, counts: Vec<usize>) -> Self {
        Self { condition, counts }
    }

    fn is_valid(&self, cond: String) -> bool {
        if cond.contains("?") { return false }
        cond.split(".").map(|c| c.len())
            .filter(|c| *c > 0)
            .collect::<Vec<usize>>() == self.counts
    }

    fn is_invalid(&self) -> bool {
        self.condition.split(".").map(|c| c.len()).sum::<usize>() < self.counts.iter().sum()
    }

    fn populate(&self, fill: String) -> String {
        // let result = self.condition.clone();
        let mut it = fill.chars();
        self.condition.chars().map(|c| if c == '?' { it.next().unwrap() } else { c }).collect()
    }

}

fn num_to_string(num: u32, count: u32) -> String {
    (0..count).rev().map(|i| if num>>i & 1 == 1 { '#' } else { '.' }).collect()
}

pub fn day12(args: &[String]) {
    println!("Day 12");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Part 1
    let springs: Vec<Spring> = contents.lines()
        .map(|l| l.parse().unwrap())
        .collect();
    // println!("{:?}", springs);

    let mut part1 = 0;
    for spring in springs {
        let mut valid = 0;
        let unk_len = spring.condition.chars().filter(|c| *c == '?').count() as u32;
        let unk_count = 2u32.pow(unk_len);
        for j in 0..unk_count {
            let n = num_to_string(j, unk_len);
            let p = spring.populate(n.clone());
            if spring.is_valid(p.clone()) {
                // println!("{} {} {}", spring.condition, n, p);
                valid += 1;
            }
        }
        part1 += valid;
        // println!("{} = {}", spring.condition, valid);
    }
    println!("Part 1: {}", part1);

    // Part 2
    let mut valids: Vec<_> = vec![];
    for repeat in 1..=2 {
        println!("repeat: {}", repeat);
        let springs2: Vec<Spring> = contents.lines()
            .map(|l| {
                let (l, r) = l.split_whitespace().next_tuple().unwrap();
                let s = vec![vec![l].repeat(repeat).join("?"), vec![r].repeat(repeat).join(",")].join(" ");
                s.parse().unwrap()
            }).collect();

        //valids.push(springs2.into_iter().map(|s| solve(s)).collect::<Vec<_>>());
        // for spring in springs2 {
        //     let s = solve(spring.clone());
        //     println!("{} {}", spring.condition, s);
        //     part2 += s;
        //     // println!("{} = {}", spring.condition, valid);
        // }
        let mut valid = vec![];
        for (i, spring) in springs2.iter().enumerate() {
            let s = solve(spring.clone());
            println!("{:4}: {} = {}", i, spring.condition, s);
            valid.push(s);
            // println!("{} = {}", spring.condition, valid);
        }
        valids.push(valid);
    }
    println!("{:?}", valids);
    let part2: usize = zip(&valids[0], &valids[1]).map(|(a, b)| (a, b / a))
            .map(|(a, x) | a * x.pow(4))
            .sum();
    println!("Part 2: {}", part2);
}

fn solve(spring: Spring) -> usize {
    if spring.is_invalid() { return 0 }
    match spring.condition.chars().position(|c| c == '?') {
        Some(pos) => {
            let mut a = spring.condition.clone();
            a.replace_range(pos..pos+1, ".");
            let mut b = spring.condition.clone();
            b.replace_range(pos..pos+1, "#");
            return solve(Spring::new(a, spring.counts.clone())) + solve(Spring::new(b, spring.counts.clone()))
        }
        //None => { assert!(spring.clone().is_valid(spring.condition)); return 1 }
        None => return if spring.clone().is_valid(spring.condition) { 1 } else { 0 }
    }
}

