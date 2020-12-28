use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn right_skip_down_1(i: u32, line: &str, skip: u32) -> u64 {
    (line
        .chars()
        .nth(((i * skip) % line.len() as u32).try_into().unwrap())
        .unwrap()
        == '#') as u64
}

fn right_1_down_2(i: u32, line: &str) -> u64 {
    if i % 2 == 0 {
        (line
            .chars()
            .nth(((i / 2) % line.len() as u32).try_into().unwrap())
            .unwrap()
            == '#') as u64
    } else {
        0
    }
}

fn main() {
    let filename = "./input_d3.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let pattern: Vec<_> = reader.lines().enumerate().collect();

    let skips = vec![1, 3, 5, 7];

    let mut trees: Vec<u64> = skips
        .iter()
        .map(|skip| {
            let count: u64 = pattern
                .iter()
                .map(|entry| right_skip_down_1(entry.0 as u32, &entry.1.as_ref().unwrap(), *skip))
                .sum();
            println!("skip {}: {:?}", skip, count);
            count
        })
        .collect();

    trees.push(
        pattern
            .iter()
            .map(|entry| right_1_down_2(entry.0 as u32, &entry.1.as_ref().unwrap()))
            .sum(),
    );

    println!("{:?}", trees);
    println!("product {:?}", trees.iter().product::<u64>());
}
