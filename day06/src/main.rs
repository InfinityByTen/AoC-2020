use itertools::Itertools;
use std::fs;

fn solve_part_1<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    input
        .map(|group| group.replace("\n", "").chars().unique().count())
        .sum::<usize>()
}

fn solve_part_2<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    input
        .map(|response| {
            let group = response.split('\n').collect::<Vec<&str>>();
            group[0]
                .chars()
                .filter(|&x| group.iter().all(|y| y.contains(x)))
                .count()
        })
        .sum::<usize>()
}

fn main() {
    let input = fs::read_to_string("./input_d6.txt").unwrap();
    println!("{:?}", solve_part_1(input.split("\n\n")));
    println!("{:?}", solve_part_2(input.split("\n\n")));
}
