use std::fs;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;
use geo::{Area, Polygon, EuclideanLength};
use geo::geometry::LineString;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dig {
    dir: char,
    distance: isize,
    // color: (u32, u32, u32)
}

impl FromStr for Dig {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([LRUD])\s(\d+)\s\(#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})\)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            dir: caps[1].chars().next().unwrap(),
            distance: caps[2].parse().unwrap(),
            // color: (
            //     u32::from_str_radix(&caps[3], 16).unwrap(),
            //     u32::from_str_radix(&caps[4], 16).unwrap(),
            //     u32::from_str_radix(&caps[5], 16).unwrap()
            // )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dig2 {
    dir: char,
    distance: isize,
}

impl FromStr for Dig2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([LRUD])\s(\d+)\s\(#([0-9a-f]{5})([0-9a-f]{1})\)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            dir: match caps[4].chars().next().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!()
            },
            distance: isize::from_str_radix(&caps[3], 16).unwrap(),
        })
    }
}

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0)
    ];
}

pub fn adjacent(pos: (isize, isize)) -> Vec<(isize, isize)> {
    ADJ.iter().map(|(dy, dx)| (pos.0 + dy, pos.1 + dx)).collect()
}

pub fn polygon_area(vertices: Vec<(f64, f64)>) -> usize {
    let ls = LineString::from(vertices);
    let edgelen = ls.euclidean_length();  // outer edge count
    let polygon = Polygon::new(ls, vec![]);
    edgelen as usize / 2 + 1 + polygon.unsigned_area() as usize
}

pub fn to_vertices(moves: &Vec<(char, isize)>) -> Vec<(f64, f64)> {
    let mut pos: (isize, isize) = (0, 0);
    let mut vertices = vec![pos]; 
    vertices.append(&mut moves.iter().map(|(dir, d)| {
        pos = match dir {
            'R' => (pos.0, pos.1 + d),
            'L' => (pos.0, pos.1 - d),
            'D' => (pos.0 + d, pos.1),
            'U' => (pos.0 - d, pos.1),
            _ => unreachable!()
        };
        pos
    }).collect());
    // println!("{:?} {}", vertices, vertices.len());
    assert!(vertices.first().unwrap() == vertices.last().unwrap());
    vertices.into_iter().map(|(y, x)| (y as f64, x as f64)).collect()
}

pub fn day18(args: &[String]) {
    println!("Day 18");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Part 1
    let plan: Vec<Dig> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let vertices: Vec<_> = to_vertices(&plan.iter().map(| p | (p.dir, p.distance)).collect());
    println!("Part 1: {}", polygon_area(vertices));

    // Part 2
    let plan2: Vec<Dig2> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let vertices2: Vec<_> = to_vertices(&plan2.iter().map(| p | (p.dir, p.distance)).collect());
    println!("Part 2: {}", polygon_area(vertices2)); 
}
