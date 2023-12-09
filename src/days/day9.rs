use std::fs;


pub fn day9(args: &[String]) {
    println!("Day 9");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let histories: Vec<Vec<i32>> = contents.lines()
        .map(|l| l.split_whitespace().map(|d| d.parse().unwrap()).collect())
        .collect();

    let part1: i32 = histories.iter().map(|h| part1(&h)).sum();
    println!("{:?}", part1);

    let part2: i32 = histories.iter().map(|h| part2(&h)).sum();
    println!("{:?}", part2);
}

fn extrapolate(history: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut histories = vec![history.clone()];
    loop {
        histories.push(histories.last().unwrap().windows(2).map(|v| v[1] - v[0]).collect());
        if histories.last().unwrap().iter().all(|v| *v == 0) { break; }
    }
    histories
}

fn part1(history: &Vec<i32>) -> i32 {
    let histories = extrapolate(history);
    histories.iter().map(|h|h.last().unwrap()).sum()
}

fn part2(history: &Vec<i32>) -> i32 {
    let histories = extrapolate(history);
    let firsts: Vec<_> = histories.iter().rev().map(|h| h.first().unwrap()).collect();
    firsts.iter().fold(0, |acc, &f| f - acc)
}