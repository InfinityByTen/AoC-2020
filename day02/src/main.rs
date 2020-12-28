use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use text_io::scan;

struct Puzzle {
    m: u32,
    n: u32,
    rule: char,
    password: String,
}

fn is_valid_1(entry: &Puzzle) -> u32 {
    // I hate you nom. You couldn't do this. At least, not straightforwardly.
    RangeInclusive::new(entry.m, entry.n)
        .contains(&(entry.password.matches(entry.rule).count() as u32)) as u32
}

fn is_valid_2(entry: &Puzzle) -> u32 {
    ((entry.password.chars().nth((entry.m - 1) as usize).unwrap() == entry.rule)
        ^ (entry.password.chars().nth((entry.n - 1) as usize).unwrap() == entry.rule)) as u32
}

fn main() {
    // Thanks Isaac for this refined reading.
    let filename = "./input_d2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let puzzle: Vec<_> = reader
        .lines()
        .map(|entry| {
            let (m, n, rule, password): (u32, u32, char, String);
            let line = entry.unwrap();
            // I wouldn't have figured this out without looking at C Vetter's solution.
            scan!( line.bytes() => "{}-{} {}: {}", m, n, rule, password);
            Puzzle {
                m,
                n,
                rule,
                password,
            }
        })
        .collect();

    let count1: u32 = puzzle.iter().map(|entry| is_valid_1(&entry)).sum();
    println!("count1: {}", count1);

    let count2: u32 = puzzle.iter().map(|entry| is_valid_2(&entry)).sum();
    println!("count2: {}", count2);
}
