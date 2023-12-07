use std::fs;
use itertools::Itertools;
use std::iter::zip;


pub fn day6(args: &[String]) {
    println!("Day 6");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let (time, distance) = contents.lines()
        .map(|l| l.split(":").skip(1).next().unwrap().split_whitespace()
            .map(|n| n.parse().unwrap()).collect::<Vec<u32>>())
        .next_tuple().unwrap();

    let part1: usize = zip(time.clone(), distance.clone())
        .map(|(t, d)| (0..t).map(|p| p * (t - p)).filter(|r| r > &d).count())
        .product();
    println!("Part 1: {}", part1);

    let (time2, distance2) = contents.lines()
        .map(|l| l.split(":").skip(1).next().unwrap()
            .split_whitespace().collect::<String>().parse::<usize>().unwrap())
        .next_tuple().unwrap();
   
    let part2: usize = (0..time2).map(|p| p * (time2 - p)).filter(|r| r > &distance2).count();
    println!("Part 2: {}", part2);
}
