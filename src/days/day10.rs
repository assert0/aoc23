use std::fs;
use itertools::iproduct;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/*
 Solution command: 
    cargo run -- day10 data/day10.txt \|
*/

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

pub fn day10(args: &[String]) {
    println!("Day 10");
    if args.len() != 2 {
        println!("Missing input file or 'S' char replacement.");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let s_replacement = &args[1];
    assert!(s_replacement.len() == 1);

    let mut tiles: Vec<Vec<_>> = contents.lines()
        .map(|l| l.chars().collect())
        .collect();

    // note replacing the start char is only needed for part2
    let start = iproduct!(0..tiles.len(), 0..tiles[0].len()).find(|(y, x)| tiles[*y][*x] == 'S').unwrap();
    tiles[start.0 as usize][start.1 as usize] = s_replacement.chars().next().unwrap();

    let (part1, loweststeps) = shortest_path(&tiles, start);
    println!("Part 1: {}", part1);

    let loopmap = loop_map(&tiles, &loweststeps);
    //print_map(&loopmap);
    println!("Part 2: {}", get_inside_count(&loopmap));   
}

fn get_inside_count(loop_map: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for y in 0..loop_map.len() {
        let mut intersections = 0;
        let mut prev = None;
        loop_map[y].iter().for_each(|c| {
            match c {
                '.' => count += intersections % 2,
                '-' => (),
                '|' | 'L' | 'F' => intersections += 1,
                '7' => if prev != Some('L') { intersections += 1 },
                'J' => if prev != Some('F') { intersections += 1 },
                _ => unreachable!()
            };
            // Note that pairs "L7" & "FJ" (with any "-" removed from in between) count
            // as a single intersection.
            prev = match c {
                'L' => Some('L'),
                'F' => Some('F'),
                '-' => prev,
                _ => None
            };
        });
    }
    count
}

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0)
    ];
}

pub fn adjacent_pipes(tiles: &Vec<Vec<char>>, pos: (isize, isize)) -> Vec<(isize, isize)> {
    ADJ.iter().filter_map(|direction| valid_pipe(&tiles, pos, *direction)).collect()
}

pub fn valid_pipe(tiles: &Vec<Vec<char>>, pos: (isize, isize), direction: (isize, isize)) -> Option<(isize, isize)> {
    let pipe = get_value(&tiles, pos).unwrap();
    let next_pipe = get_value(&tiles, (pos.0 + direction.0, pos.1 + direction.1));
    if next_pipe.is_none() { return None }
    if match direction {
        (0, 1) => "-LF".contains(pipe) && "-J7".contains(next_pipe.unwrap()),
        (0, -1) => "-J7".contains(pipe) && "-LF".contains(next_pipe.unwrap()),
        (1, 0) => "|F7".contains(pipe) && "|LJ".contains(next_pipe.unwrap()),
        (-1, 0) => "|LJ".contains(pipe) && "|F7".contains(next_pipe.unwrap()),
        _ => false
    } {
        return Some(((pos.0 + direction.0) as isize, (pos.1 + direction.1) as isize));
    }
    None
}

pub fn get_value(tiles: &Vec<Vec<char>>, pos: (isize, isize)) -> Option<char> {
    let (my, mx) = (tiles.len(), tiles[0].len());
    if pos.0 >= 0 && pos.1 >= 0 && pos.0 < my as isize && pos.1 < mx as isize {
        return Some(tiles[pos.0 as usize][pos.1 as usize]);
    }
    None
}

// derived from https://doc.rust-lang.org/std/collections/binary_heap/index.html
pub fn shortest_path(tiles: &Vec<Vec<char>>, start: (usize, usize)) -> (u32, Vec<Vec<u32>>) {
    let (ysize, xsize) = (tiles.len(), tiles[0].len());
    let mut loweststeps = vec![vec![u32::MAX; xsize]; ysize];

    let mut heap = BinaryHeap::new();

    // `start` with a zero steps
    loweststeps[start.0][start.1] = 0;
    heap.push(State { steps: 0, position: (start.0, start.1) });

    let mut maxvalue = 0;
    // Examine lower steps positions first (min-heap)
    while let Some(State { steps, position }) = heap.pop() {
        //println!("pos: {:?}", position);
        if steps > maxvalue {
            maxvalue = steps;
        }
        // Important as we may have already found a better way
        if steps > loweststeps[position.0][position.1] { 
            continue; 
        }
        // For each node we can reach, see if we can find a way with
        // a lower steps going through this node
        for adj in adjacent_pipes(&tiles, (position.0 as isize, position.1 as isize)) {
            
            let next = State { steps: steps + 1, position: (adj.0 as usize, adj.1 as usize) };
            // If so, add it to the frontier and continue
            if next.steps < loweststeps[next.position.0][next.position.1] {
                heap.push(next);
                // Relaxation, we have now found a better way
                loweststeps[next.position.0][next.position.1] = next.steps;
            }
        }
    }
    (maxvalue, loweststeps)
}

fn loop_map(tiles: &Vec<Vec<char>>, loweststeps: &Vec<Vec<u32>>) -> Vec<Vec<char>>
{
    let mut output = vec![vec!['?'; loweststeps[0].len()]; loweststeps.len()];
    for (y, x) in iproduct!(0..loweststeps.len(), 0..loweststeps[0].len()) {
        output[y][x] = match loweststeps[y][x] {
            u32::MAX => '.',
            _ => tiles[y][x]
        } 
    }
    output
}

// fn print_map(tiles: &Vec<Vec<char>>)
// {
//     let mut output = Vec::new();
//     for y in 0..tiles.len() {
//         for x in 0..tiles[0].len() {
//             output.push(tiles[y][x].to_string());
//         }
//         output.push("\n".to_string());
//     }
//     println!("{}", output.join(""));
// }
