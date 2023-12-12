use std::fs;
use itertools::iproduct;
use rusttype::{Point, point};


pub fn day11(args: &[String]) {
    println!("Day 11");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let image: Vec<Vec<_>> = contents.lines()
        .map(|l| l.chars().collect())
        .collect();
    
    let empty_rows: Vec<_> = (0..image.len()).into_iter()
                                .filter(|r| (0..image[*r].len()).into_iter().all(|c| image[*r][c] == '.'))
                                .map(|i| i as isize).collect();
    let empty_cols: Vec<_> = (0..image[0].len()).into_iter()
                                .filter(|c| (0..image.len()).into_iter().all(|r| image[r][*c] == '.'))
                                .map(|i| i as isize).collect();
    // println!("{:?} {:?}", empty_rows, empty_cols);

    let galaxies: Vec<_> = iproduct!(0..image.len(), 0..image[0].len())
                    .filter(|(y, x)| image[*y][*x] == '#')
                    .map(|(y, x)| point(x as isize, y as isize))
                    .collect();
    // println!("{:?}", galaxies);

    let galaxy_pairs: Vec<_> = iproduct!(0..galaxies.len(), 0..galaxies.len()).filter(|(i, j)| i < j)
        .map(|(i, j)| ((galaxies[i], galaxies[j])))
        .collect();

    let part1: isize = galaxy_pairs.iter()
                            .map(|(a, b)| manhatten2(*a, *b, &empty_rows, &empty_cols, 1))
                            .sum();
    println!("Part 1: {}", part1);

    let part2: isize = galaxy_pairs.iter()
                            .map(|(a, b)| manhatten2(*a, *b, &empty_rows, &empty_cols, 1000000-1))
                            .sum();
    println!("Part 2: {}", part2);
}

pub fn manhatten(a: Point<isize>, b: Point<isize>) -> isize {
    let d = b - a;
    d.x.abs() + d.y.abs()
}

pub fn manhatten2(a: Point<isize>, b: Point<isize>, empty_rows: &Vec<isize>, empty_cols: &Vec<isize>, multiplier: isize) -> isize {
    let xrange = if a.x < b.x { a.x..b.x } else { b.x..a.x };
    let yrange = if a.y < b.y { a.y..b.y } else { b.y..a.y };
    
    let xexp = multiplier * empty_cols.iter().filter(|c| xrange.contains(c)).count() as isize;
    let yexp = multiplier * empty_rows.iter().filter(|r| yrange.contains(r)).count() as isize;

    let d = b - a;
    d.x.abs() + d.y.abs() + xexp + yexp
}
