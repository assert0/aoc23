use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;
use std::num::ParseIntError;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cubes {
    count: u32,
    color: String
}

impl FromStr for Cubes {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            count: caps[1].parse::<u32>().unwrap(),
            color: caps[2].to_string()
        })
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    id: u32,
    handfuls: Vec<Vec<Cubes>>,
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Game (\d+): (.*)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            id: caps[1].parse::<u32>().unwrap(),
            handfuls: caps[2].split("; ")
                .map(|h| h.split(", ")
                    .map(|c| c.parse().unwrap()).collect())
                .collect()
        })
    }
}

impl Game {

    fn max_count(&self, color: &str) -> u32 {
        self.handfuls.iter()
            .map(|h| h.iter().filter(|g| g.color == color)
                .map(|g| g.count ).max()
            ).max().unwrap().unwrap()
    }

    fn max_counts(&self) -> (u32, u32, u32) {
        ["red", "green", "blue"].iter()
            .map(|c| self.max_count(c))
            .collect_tuple()
            .unwrap()
    }

    fn is_valid(&self, red: u32, green: u32, blue: u32) -> bool {
        let (r, g, b) = self.max_counts();
        r <= red && g <= green && b <= blue
    }

    fn power(&self) -> u32 {
        let (r, g, b) = self.max_counts();
        r * g * b
    }

}

pub fn day2(args: &[String]) {
    println!("Day 2");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let games: Vec<Game> = contents.split("\n")
        .map(|l| l.parse::<Game>().unwrap())
        .collect();

    let part1: u32 = games.iter()
        .filter(|g| g.is_valid(12, 13, 14))
        .map(|g| g.id)
        .sum();
    println!("Part 1: {}", part1);

    let part2: u32 = games.iter()
        .map(|g: &Game| g.power())
        .sum();
    println!("Part 2: {}", part2);
    
}