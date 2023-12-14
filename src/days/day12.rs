use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use itertools::iproduct;

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0)
    ];
}

pub fn adjacent(pos: (isize, isize)) -> Vec<(isize, isize)> {
    ADJ.iter().map(|(dy, dx)| (pos.0 + dy, pos.1 + dx)).collect()
}

pub fn get_value(map: &Vec<Vec<u32>>, pos: (isize, isize)) -> u32 {
    let (my, mx) = (map.len(), map[0].len());
    if pos.0 >= 0 && pos.1 >= 0 && pos.0 < my as isize && pos.1 < mx as isize {
        return map[pos.0 as usize][pos.1 as usize];
    }
    u32::MAX
}


const ITEMSET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    steps: u32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.steps.cmp(&self.steps)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// derived from https://doc.rust-lang.org/std/collections/binary_heap/index.html
pub fn shortest_path(map: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let (ysize, xsize) = (map.len(), map[0].len());
    let mut loweststeps = vec![vec![u32::MAX; xsize]; ysize];

    let mut heap = BinaryHeap::new();

    // `start` with a zero steps
    loweststeps[0][0] = 0;
    heap.push(State { steps: 0, position: start });

    // Examine lower steps positions first (min-heap)
    while let Some(State { steps, position }) = heap.pop() {
        if position == end { 
            return Some(steps); 
        }

        // Important as we may have already found a better way
        if steps > loweststeps[position.0][position.1] { 
            continue; 
        }
        let current = get_value(&map, (position.0 as isize, position.1 as isize));

        // For each node we can reach, see if we can find a way with
        // a lower steps going through this node
        for adj in adjacent((position.0 as isize, position.1 as isize)) {
            let adj_elev = get_value(&map, (adj.0, adj.1));
            if adj_elev <= current + 1 {
                let next = State { steps: steps + 1, position: (adj.0 as usize, adj.1 as usize) };

                // If so, add it to the frontier and continue
                if next.steps < loweststeps[next.position.0][next.position.1] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    loweststeps[next.position.0][next.position.1] = next.steps;
                }
            }
        }
    }
    // Goal not reachable
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    start: (usize, usize),
    end: (usize, usize),
    elevation: Vec<Vec<u32>>,
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<char>> = s.lines()
            .map(|l| l.chars().collect())
            .collect();
        let start = get_char_position(&map, 'S');
        let end = get_char_position(&map, 'E');
        let s = s.replace("S", "a").replace("E", "z");
        let elevation: Vec<Vec<u32>> = s.lines()
            .map(|l| l.chars().map(|c| ITEMSET.find(c).unwrap() as u32).collect())
            .collect();
        Ok(Self {
            start,
            end,
            elevation
        })
    }
}

pub fn get_char_position(map: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    let (ysize, xsize) = (map.len(), map[0].len());
    for (y, x) in iproduct!(0..ysize, 0..xsize) {
        if map[y][x] == c {
            return (y, x)
        }
    }
    panic!("No match");
}

pub fn starting_elevations(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let (ysize, xsize) = (map.len(), map[0].len());
    let mut all_a = vec![];
    for (y, x) in iproduct!(0..ysize, 0..xsize) {
        if map[y][x] == 0 {
            all_a.push((y, x));
        }
    }
    all_a
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

    let map: Map = contents.parse().unwrap();

    // Part 1
    println!("Part 1: {}", shortest_path(&map.elevation, map.start, map.end).unwrap());

    // Part 2
    let part2 = starting_elevations(&map.elevation).iter()
        .filter_map(|&p| shortest_path(&map.elevation, p, map.end))
        .min().unwrap();
    println!("Part 2: {}", part2);
}

