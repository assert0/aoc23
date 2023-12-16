use std::fs;
use itertools::iproduct;
use std::collections::HashSet;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Moving {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Beam {
    position: (isize, isize),
    moving: Moving
}

impl Beam {
    fn new(position: (isize, isize), moving: Moving) -> Self {
        Self { position, moving }
    }

    fn step(self, grid: char) -> Vec<Self> {
        let (y, x) = self.position;
        match grid {
            '.' => match self.moving {
                Moving::Right => vec![Beam::new((y, x+1), self.moving)],
                Moving::Left => vec![Beam::new((y, x-1), self.moving)],
                Moving::Up => vec![Beam::new((y-1, x), self.moving)],
                Moving::Down => vec![Beam::new((y+1, x), self.moving)],
            },
            '/' => match self.moving {
                Moving::Right => vec![Beam::new((y-1, x), Moving::Up)],
                Moving::Left => vec![Beam::new((y+1, x), Moving::Down)],
                Moving::Up => vec![Beam::new((y, x+1), Moving::Right)],
                Moving::Down => vec![Beam::new((y, x-1), Moving::Left)],
            },
            '\\' => match self.moving {
                Moving::Right => vec![Beam::new((y+1, x), Moving::Down)],
                Moving::Left => vec![Beam::new((y-1, x), Moving::Up)],
                Moving::Up => vec![Beam::new((y, x-1), Moving::Left)],
                Moving::Down => vec![Beam::new((y, x+1), Moving::Right)],
            },
            '-' => match self.moving {
                Moving::Right => vec![Beam::new((y, x+1), Moving::Right)],
                Moving::Left => vec![Beam::new((y, x-1), Moving::Left)],
                Moving::Up | Moving::Down => vec![Beam::new((y, x-1), Moving::Left), Beam::new((y, x+1), Moving::Right)],
            },
            '|' => match self.moving {
                Moving::Right | Moving::Left => vec![Beam::new((y-1, x), Moving::Up), Beam::new((y+1, x), Moving::Down)],
                Moving::Up => vec![Beam::new((y-1, x), Moving::Up)],
                Moving::Down => vec![Beam::new((y+1, x), Moving::Down)],
            }
            _ => unreachable!()
        }
    }
}


fn move_beam(start: Beam, contraption: &Vec<Vec<char>>) -> usize {
    let mut beams = vec![start];
    let mut energized: Vec<Vec<HashSet<Moving>>> = vec![vec![HashSet::new(); contraption[0].len()]; contraption.len()];

    while let Some(beam) = beams.pop() {
        // println!("{:?} = {:?}", beam, beams.len());
        // filter out beams that move off the contraption
        if beam.position.0 < 0 || beam.position.0 >= contraption.len() as isize || 
                beam.position.1 < 0 || beam.position.1 >= contraption[0].len() as isize {
            continue;
        }
        // maintain set of seen beams
        if energized[beam.position.0 as usize][beam.position.1 as usize].insert(beam.moving) {
            // only step new beams 
            beams.append(&mut beam.step(contraption[beam.position.0 as usize][beam.position.1 as usize]));
        }
    }
    iproduct!(0..energized.len(), 0..energized[0].len())
            .filter(|(y, x)| energized[*y][*x].len() > 0).count()
}


pub fn day16(args: &[String]) {
    println!("Day 16");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let contraption: Vec<Vec<_>> = contents.lines()
        .map(|l| l.chars().collect())
        .collect();

    // Part 1
    let part1 = move_beam(Beam::new((0,0), Moving::Right), &contraption);
    println!("Part 1: {}", part1);

    // Part 2
    let mut beams: Vec<Beam> = vec![];
    beams.append(&mut (0..contraption.len()).map(|y| Beam::new((y as isize, 0), Moving::Right)).collect());
    beams.append(&mut (0..contraption.len()).map(|y| Beam::new((y as isize, contraption[y].len() as isize - 1), Moving::Left)).collect());
    beams.append(&mut (0..contraption[0].len()).map(|x| Beam::new((0, x as isize), Moving::Down)).collect());
    beams.append(&mut (0..contraption[0].len()).map(|x| Beam::new((contraption.len() as isize - 1, x as isize), Moving::Up)).collect());
    println!("Part 2: {}", beams.iter().map(|b| move_beam(*b, &contraption)).max().unwrap());
}
