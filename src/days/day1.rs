use std::fs;


pub fn day1(args: &[String]) {
    println!("Day 1");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let part1: u32 = contents.lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .filter(|n| n.len() > 0)
        .map(|n| n.first().unwrap() * 10 + n.last().unwrap())
        .sum();
    println!("Part 1: {}", part1); 

    let mapping = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    
    let part2: u32 = contents.split("\n")
        .map(|l| first(&mapping, l) * 10 + last(&mapping, l))
        .sum();
    println!("Part 2: {}", part2);
}


fn first(mapping: &Vec<(&str, u32)>, input: &str) -> u32 {
    mapping.iter()
        .map(|(s, v)| (input.find(s), v))
        .filter(|(i, _)| i.is_some())
        .min_by_key(|(i, _)| i.unwrap()).unwrap().1.clone()
}

fn last(mapping: &Vec<(&str, u32)>, input: &str) -> u32 {
    mapping.iter()
        .map(|(s, v)| (input.rfind(s), v))
        .filter(|(i, _)| i.is_some())
        .max_by_key(|(i, _)| i.unwrap()).unwrap().1.clone()
}