use std::fs;
use std::str::FromStr;
use regex::Regex;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Card\s+(\d+):([\d\s]+)\|([\d\s]+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            id: caps[1].parse::<u32>().unwrap(),
            winners: caps[2].split_whitespace().map(|n| n.trim().parse().unwrap()).collect(),
            numbers: caps[3].split_whitespace().map(|n| n.trim().parse().unwrap()).collect(),
        })
    }
}

impl Card {

    fn winning_numbers(&self) -> Vec<u32> {
        self.numbers.clone().into_iter()
            .filter(|n| self.winners.contains(n))
            .collect()
    }

    fn points(&self) -> u32 {
        let count = self.winning_numbers().len() as u32;
        2u32.pow(count) / 2
    }

}

fn part2_instances(counts: &Vec<usize>, index: usize, depth: u32) -> u32 {
    let mut count = 1;
    for i in index+1..index+1+counts[index] {
        count += part2_instances(&counts, i, depth+1);
    }
    count
}

pub fn day4(args: &[String]) {
    println!("Day 4");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let cards: Vec<Card> = contents.lines()
        .map(|l| l.parse::<Card>().unwrap())
        .collect();

    let part1: u32 = cards.iter()
        .map(|c| c.points())
        .sum();
    println!("Part 1: {}", part1);

    let counts: Vec<_> = cards.iter()
        .map(|c| c.winning_numbers().len())
        .collect();

    // recursive
    let part2: u32 = (0..counts.len())
        .map(|i| part2_instances(&counts, i, 0)).sum();
    println!("Part 2: {}", part2);

    // non-recursive
    let mut copies = vec![1; counts.len()];
    for (i, count) in counts.iter().enumerate() {
        for j in i+1..i+1+count {
            copies[j] += copies[i]
        }
    }
    println!("Part 2: {}", copies.iter().sum::<u32>());
}
