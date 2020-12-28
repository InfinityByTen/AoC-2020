use itertools::Itertools;
use std::fs;

fn solve_2((t_str, schedule): (&str, &str)) {
    let time = t_str.parse::<u32>().unwrap();
    let busses = schedule
        .split(',')
        .enumerate()
        .filter(|entry| !entry.1.eq("x"))
        .map(|x| (x.0, x.1.parse::<u32>().unwrap()))
        .collect::<Vec<(usize, u32)>>();
    println!("{:?}, {:?}", time, busses);
}

fn solve_1((t_str, schedule): (&str, &str)) {
    let time = t_str.parse::<u32>().unwrap();
    let busses = schedule
        .split(',')
        .filter(|entry| !entry.eq(&"x"))
        .map(|y| y.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    println!("{:?}, {:?}", time, busses);
    let remainders = busses
        .iter()
        .map(|bus| bus - (time % bus))
        .enumerate()
        .map(|(x, y)| (y, x))
        .collect::<Vec<(u32, usize)>>();
    let res = remainders.iter().min().unwrap();
    println!("{:?}", res.0 * busses[res.1]);
}

fn main() {
    let input = fs::read_to_string("./input_d13.txt").unwrap();
    solve_1(input.split('\n').collect_tuple().unwrap());
    solve_2(input.split('\n').collect_tuple().unwrap());
}
