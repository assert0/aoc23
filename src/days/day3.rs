use std::fs;
use std::iter;
use std::iter::zip;
use std::collections::HashMap;
use itertools::iproduct;

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0),
        (-1, -1), (-1, 1), (1, -1), (1, 1)
    ];
}

pub fn day3(args: &[String]) {
    println!("Day 3");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let symbols: Vec<char> = "$-*/=@#%+&".chars().collect();

    let schematic: Vec<Vec<_>> = contents.lines()
        .map(|l| l.chars().collect())
        .collect();

    let special_pos: Vec<_> = iproduct!(0..schematic.len(), 0..schematic[0].len())
        .filter(|(y, x)| symbols.contains(&schematic[*y][*x]))
        .map(|(y, x)| (y as isize, x as isize))
        .collect();
    
    let gear_pos: Vec<_> = iproduct!(0..schematic.len(), 0..schematic[0].len())
        .filter(|(y, x)| schematic[*y][*x] == '*')
        .map(|(y, x)| (y as isize, x as isize))
        .collect();

    let mut gears: HashMap<(isize, isize), Vec<u32>> = HashMap::from_iter(zip(gear_pos.clone(), iter::repeat(vec![])));
    let mut values = vec![];
    for y in 0..schematic.len() {
        let mut x: usize = 0;
        while x < schematic[y].len() {
            let mut value = None;
            let mut adj_special = false;
            let mut done = false;
            let mut adj_gear: Option<(isize, isize)> = None;
            while !done {
                match schematic[y][x].to_digit(10) {
                    Some(v) => {
                        value = match value {
                            None => Some(v),
                            Some(value) => Some(value * 10 + v)
                        };
                        adj_special |= ADJ.iter()
                                          .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
                                          .any(|p| special_pos.contains(&p));
                        let adj_gears: Vec<_> = ADJ.iter()
                                                   .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
                                                   .filter(|p| gear_pos.contains(p))
                                                   .collect();
                        if adj_gears.len() == 1 {
                            adj_gear = Some(adj_gears[0]);
                        }
                    },
                    None => done = true
                }
                x += 1;
                done |= x == schematic[y].len();  // reached the end of the line
            }
            if done && value.is_some() && adj_special {
                values.push(value.unwrap());
                // add value to the gear position
                if adj_gear.is_some() {
                    if let Some(gear) = gears.get_mut(&adj_gear.unwrap()) {
                        gear.push(value.unwrap());
                    }
                }
            }
        }
    }
    println!("Part 1: {:?}", values.iter().sum::<u32>());
    
    let part2: u32 = gears.iter()
                          .filter(|(_, values)| values.len() == 2)
                          .map(|(_, values)| values.iter().product::<u32>())
                          .sum();
    println!("Part 2: {:?}", part2);
}
