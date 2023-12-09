use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;
use itertools::iproduct;
use lcmx::lcmx;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    name: String,
    instruction: (String, String)
}

impl FromStr for Node {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            name: caps[1].to_string(),
            instruction: (caps[2].to_string(), caps[3].to_string())
        })
    }
}

pub fn day8(args: &[String]) {
    println!("Day 8");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut sections = contents.split("\n\n");
    let intructions = sections.next().unwrap();
    let nodes: Vec<Node> = sections.next().unwrap().lines()
                                .map(| l | l.parse().unwrap()).collect();

    println!("Part 1: {:?}", get_count(intructions, &nodes, "AAA", "ZZZ"));
    println!("Part 2: {:?}", part2(intructions, &nodes));
}

fn get_count(instructions: &str, nodes: &Vec<Node>, start: &str, end: &str) -> Option<u64> {
    let mut it = instructions.chars().cycle();
    let mut name = start;
    let mut count = 0;
    loop {
        let cur = nodes.iter().find(|&n| n.name == name);
        if cur.is_none() { break; }
        name = match it.next().unwrap() {
            'L' => &cur.unwrap().instruction.0,
            'R' => &cur.unwrap().instruction.1,
            _ => unreachable!()
        };
        count += 1;
        if name == end { return Some(count) }
        if count > 30_000 { break }  // max count limit
    }
    None
}

fn nodes_ending_with(nodes: &Vec<Node>, value: &str) -> Vec<String> {
    nodes.iter()
        .filter(|&n| n.name.ends_with(value))
        .map(|n| n.name.to_string()).collect()
}

fn part2(instructions: &str, nodes: &Vec<Node>) -> Option<u64> {
    let starts: Vec<_> = nodes_ending_with(nodes, "A");
    let ends: Vec<_> = nodes_ending_with(nodes, "Z");

    // assume there is only one solution for getting from a given start to an end
    let counts: Vec<_> = iproduct!(starts, ends)
            .filter_map(|(s, e)| get_count(instructions, nodes, &s, &e))
            .collect();
    //println!("{:?}", counts);
    lcmx(&counts)
}