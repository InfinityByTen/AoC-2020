use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_puzzle() -> Option<Vec<i32>> {
    if let Ok(lines) = read_lines("./input_1_d1.txt") {
        let mut numbers = Vec::new();
        for line in lines {
            if let Ok(num_str) = line {
                if let Ok(num) = num_str.parse::<i32>() {
                    numbers.push(num);
                } else {
                    println!("{}", "Error Reading the file!!");
                }
            }
        }
        Some(numbers)
    } else {
        None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_components(puzzle: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    let mut missing_pieces = Vec::new();
    for entry in puzzle {
        missing_pieces.push(sum - entry);
    }

    for i in 0..puzzle.len() {
        if let Ok(component) = puzzle.binary_search(&missing_pieces[i]) {
            return Some((puzzle[i], puzzle[component]));
        }
    }
    None
}

fn solve_part_1(puzzle: &Vec<i32>) {
    if let Some(result) = get_components(&puzzle, 2020) {
        println!("Components: {} {}", result.0, result.1);
        println!("Product: {}", result.0 * result.1);
    }
}

fn solve_part_2(puzzle: &Vec<i32>) {
    let mut missing_pieces = Vec::new();
    for entry in puzzle {
        missing_pieces.push(2020 - entry)
    }

    for i in 0..puzzle.len() {
        if let Some(components) = get_components(puzzle, missing_pieces[i]) {
            println!(
                "Numbers are: {} {} {}",
                puzzle[i], components.0, components.1
            );
            println!("Product: {}", puzzle[i] * components.0 * components.1);
            return;
        }
    }
}

fn main() {
    let mut puzzle = get_puzzle().unwrap();
    puzzle.sort();

    solve_part_1(&puzzle);

    solve_part_2(&puzzle);
}
