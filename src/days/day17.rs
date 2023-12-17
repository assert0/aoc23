
use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Moving {
    Right,
    Left,
    Up,
    Down,
}

fn moving_index(moving: Moving) -> usize {
    match moving {
        Moving::Right => 0,
        Moving::Left => 1,
        Moving::Up => 2,
        Moving::Down => 3,
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    heatloss: u32,
    path: Vec<(isize, isize)>,
    moving: Moving,
    moving_count: u32
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.heatloss.cmp(&self.heatloss)
            .then_with(|| self.path.last().unwrap().cmp(&other.path.last().unwrap()))
            .then_with(|| moving_index(self.moving).cmp(&moving_index(other.moving)))
            .then_with(|| self.moving_count.cmp(&other.moving_count))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day17(args: &[String]) {
    println!("Day 17");
    if args.len() != 1 {
        println!("Missing input file.");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let map: Vec<Vec<_>> = contents.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let end = (map.len() as isize - 1, map[0].len() as isize - 1);
    let part1 = shortest_path(&map, end);
    println!("Part 1: {}", part1);

    let part2 = shortest_path2(&map, end);
    println!("Part 2: {}", part2);
}

lazy_static! {
    static ref ADJ: Vec<(isize, isize, Moving)> = vec![
        (0, 1, Moving::Right), (0, -1, Moving::Left), (1, 0, Moving::Down), (-1, 0, Moving::Up)
    ];
}

pub fn adjacent(pos: (isize, isize)) -> Vec<(isize, isize, Moving)> {
    ADJ.iter().map(|(dy, dx, moving)| (pos.0 + dy, pos.1 + dx, *moving)).collect()
}

pub fn get_value(map: &Vec<Vec<u32>>, pos: (isize, isize)) -> u32 {
    let (my, mx) = (map.len(), map[0].len());
    if pos.0 >= 0 && pos.1 >= 0 && pos.0 < my as isize && pos.1 < mx as isize {
        return map[pos.0 as usize][pos.1 as usize];
    }
    u32::MAX
}

fn shortest_path(map: &Vec<Vec<u32>>, end: (isize, isize)) -> u32 {
    let (ysize, xsize) = (map.len(), map[0].len());
    let mut lowest = vec![vec![vec![vec![u32::MAX; 3]; 4]; xsize]; ysize];

    let mut paths = BinaryHeap::new();
    paths.push(State {
        heatloss: 0,
        path: vec![(0, 0)],
        moving: Moving::Up, // don't use right or down
        moving_count: 0,
    });

    // let mut best_path = vec![];
    let mut best = u32::MAX;
    while let Some(current) = paths.pop() {
        let (heatloss, path, moving, moving_count) = (current.heatloss, current.path, current.moving, current.moving_count);
        // out of bounds
        if get_value(map, *path.last().unwrap()) == u32::MAX {
            continue;
        }
        assert!(heatloss == path.iter().skip(1).map(|p| get_value(map, *p)).sum());
        let (cur_y, cur_x) = *path.last().unwrap();
        // at end
        if (cur_y, cur_x) == end {
            best = heatloss;
            // best_path = path.clone();
            break;
        }
        if heatloss > best || heatloss > lowest[cur_y as usize][cur_x as usize][moving_index(moving)][moving_count as usize] { 
            continue;
        }
        // println!("{:?} {} {:?} {}", path.last().unwrap(), heatloss, moving, moving_count);
        
        for (y, x, m) in adjacent(*path.last().unwrap()) {
            // detect loops
            if path.contains(&(y, x)) {
                continue;
            }
            let adj_heatloss = get_value(&map, (y, x));
            if adj_heatloss == u32::MAX {
                continue;
            }
            let next_heatloss = heatloss + adj_heatloss;
            let next_moving_index = moving_index(m);
            let next_moving_count = if moving == m { moving_count + 1 } else { 0 };
            // three steps in the same direction
            if next_moving_count >= 3 {
                continue;
            }
            if next_heatloss < lowest[y as usize][x as usize][next_moving_index][next_moving_count as usize] {
                let mut next_path = path.clone();
                next_path.push((y, x));
                paths.push(State {
                    heatloss: next_heatloss,
                    path: next_path,
                    moving: m,
                    moving_count: next_moving_count
                });
                lowest[y as usize][x as usize][next_moving_index][next_moving_count as usize] = next_heatloss;
            }
        }
    }
    best
}

fn shortest_path2(map: &Vec<Vec<u32>>, end: (isize, isize)) -> u32 {
    let (ysize, xsize) = (map.len(), map[0].len());
    let mut lowest = vec![vec![vec![vec![u32::MAX; 10]; 4]; xsize]; ysize];

    let mut paths = BinaryHeap::new();
    paths.push(State {
        heatloss: 0,
        path: vec![(0, 0)],
        moving: Moving::Up, // don't use right or down
        moving_count: 4,
    });

    // let mut best_path = vec![];
    let mut best = u32::MAX;
    while let Some(current) = paths.pop() {
        let (heatloss, path, moving, moving_count) = (current.heatloss, current.path, current.moving, current.moving_count);
        // out of bounds
        if get_value(map, *path.last().unwrap()) == u32::MAX {
            continue;
        }
        assert!(heatloss == path.iter().skip(1).map(|p| get_value(map, *p)).sum());
        let (cur_y, cur_x) = *path.last().unwrap();
        // at end
        if (cur_y, cur_x) == end {
            best = heatloss;
            // best_path = path.clone();
            break;
        }
        if heatloss > best || heatloss > lowest[cur_y as usize][cur_x as usize][moving_index(moving)][moving_count as usize] { 
            continue;
        }
        // println!("{:?} {} {:?} {}", path.last().unwrap(), heatloss, moving, moving_count);
        
        for (y, x, m) in adjacent(*path.last().unwrap()) {
            // detect loops
            if path.contains(&(y, x)) {
                continue;
            }
            let adj_heatloss = get_value(&map, (y, x));
            if adj_heatloss == u32::MAX {
                continue;
            }
            let next_heatloss = heatloss + adj_heatloss;
            let next_moving_index = moving_index(m);
            let next_moving_count = if moving == m { moving_count + 1 } else { 0 };
            // 10 steps in the same direction
            if next_moving_count >= 10 {
                continue;
            }
            // can't change direction if haven't moved 4 blocks in the same direction
            if moving_count < 3 && moving != m {
                continue;  // can't switch directions yet
            }
            if next_heatloss < lowest[y as usize][x as usize][next_moving_index][next_moving_count as usize] {
                let mut next_path = path.clone();
                next_path.push((y, x));
                paths.push(State {
                    heatloss: next_heatloss,
                    path: next_path,
                    moving: m,
                    moving_count: next_moving_count
                });
                lowest[y as usize][x as usize][next_moving_index][next_moving_count as usize] = next_heatloss;
            }
        }
    }
    best
}
