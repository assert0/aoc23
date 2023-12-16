use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;
use ascii_converter::string_to_decimals;


#[derive(Clone, Eq, PartialEq, Debug)]
struct Step {
    name: String,
    length: Option<usize>,
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+)([=\-])(\d*)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            name: caps[1].to_string(),
            length: if caps[2].to_string() == "=" { Some(caps[3].parse().unwrap()) } else { None },
        })
    }
}

fn hash(step: &str) -> usize {
    string_to_decimals(step).unwrap().iter()
        .fold(0, |acc, x| ((acc + *x as usize) * 17) % 256)
}

pub fn day15(args: &[String]) {
    println!("Day 15");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Part 1
    let part1: usize = contents.split(",").map(|s| hash(s)).sum();
    println!("Part 1: {:?}", part1);

    // Part 2
    let steps: Vec<Step> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    let mut boxes: Vec<Vec<Step>> = vec![vec![]; 256];
    for step in steps {
        let h = hash(&step.name);
        match boxes[h].iter().position(|b| b.name == step.name) {
            Some(pos) => match step.length {
                Some(_) => boxes[h][pos] = step,
                None => { boxes[h].remove(pos); }
            },
            None => match step.length {
                Some(_) => boxes[h].push(step),
                None => ()
            }
        }
    }

    let part2: usize = boxes.iter().enumerate().map(|(bnum, b)|
            b.iter().enumerate().map(|(snum, s)| (bnum + 1) * (snum + 1) * s.length.unwrap()).sum::<usize>()
        ).sum();
    println!("Part 2: {:?}", part2);
}
