use std::fs;
use std::cmp::Ordering;
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;
use counter::Counter;


const ORDER_PART1: &'static str = "AKQJT98765432";

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Hand {
    cards: String,
    bid: usize,
}

impl FromStr for Hand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = parse_hand_bid(s);
        Ok(Hand { cards, bid })
    }
}

impl Hand {

    fn strength(&self) -> u32 {
        let c: Vec<_> =  self.cards.chars().collect::<Counter<_>>().most_common_ordered()
                    .iter().map(|v| v.1).collect();
        hand_strength(c)
    }
    
    fn value(&self) -> u32 {
        self.cards.chars().fold(0, |acc, h| 
            acc * ORDER_PART1.len() + ORDER_PART1.chars().rev().position(|o| o == h).unwrap()
        ) as u32
    }

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
            .then_with(|| self.value().cmp(&other.value()))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const ORDER_PART2: &'static str = "AKQT98765432J";

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Hand2 {
    cards: String,
    bid: usize,
}

impl FromStr for Hand2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = parse_hand_bid(s);
        Ok(Hand2 { cards, bid })
    }
}

impl Hand2 {

    fn strength(&self) -> u32 {
        let no_j: Vec<_> = self.cards.chars().filter(|&c| c != 'J').collect();
        let mut c: Vec<_> = no_j.iter().collect::<Counter<_>>().most_common_ordered()
                                .iter().map(|v| v.1).collect();
        if c.len() == 0 {
            // all J cards
            c = vec![0];
        }
        c[0] += self.cards.len() - no_j.len();
        hand_strength(c)
    }
    
    fn value(&self) -> u32 {
        self.cards.chars().fold(0, |acc, h| 
            acc * ORDER_PART2.len() + ORDER_PART2.chars().rev().position(|o| o == h).unwrap()
        ) as u32
    }

}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
            .then_with(|| self.value().cmp(&other.value()))
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hand_bid(s: &str) -> (String, usize) {
    let (cards, bid) = s.split(" ").next_tuple::<(&str, &str)>().unwrap();
    (cards.to_string(), bid.parse().unwrap())
}

fn hand_strength(c: Vec<usize>) -> u32 {
    if *c == vec![5]                  { 6 }
    else if *c == vec![4, 1]          { 5 } 
    else if *c == vec![3, 2]          { 4 }
    else if *c == vec![3, 1, 1]       { 3 }
    else if *c == vec![2, 2, 1]       { 2 }
    else if *c == vec![2, 1, 1, 1]    { 1 }
    else if *c == vec![1, 1, 1, 1, 1] { 0 }
    else { unreachable!() }
}


pub fn day7(args: &[String]) {
    println!("Day 7");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let mut hands: Vec<Hand> = contents.lines()
        .map(|l| l.parse().unwrap())
        .collect();
    hands.sort();
    let part1: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    println!("Part 1: {}", part1);

    let mut hands2: Vec<Hand2> = contents.lines()
        .map(|l| l.parse().unwrap())
        .collect();
    hands2.sort();
    let part2: usize = hands2.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    println!("Part 2: {}", part2);
}
