use std::collections::HashMap;
use std::fs;

fn parse<'a>(input: impl Iterator<Item = &'a str>) -> Vec<(char, u32)> {
    input
        .map(|entry| {
            let mut ins = entry.chars();
            (
                ins.next().unwrap(),
                ins.collect::<String>().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(char, u32)>>()
}

fn solve_1(route: &Vec<(char, u32)>) {
    let cosines: HashMap<_, _> = vec![(0, 1), (90, 0), (180, -1), (270, 0)]
        .into_iter()
        .collect();
    let sines: HashMap<_, _> = vec![(0, 0), (90, 1), (180, 0), (270, -1)]
        .into_iter()
        .collect();
    let mut location = (0i32, 0i32);
    let mut compass = 0i32;
    route.iter().for_each(|ins| match ins.0 {
        'F' => {
            location.0 += cosines[&compass] * (ins.1 as i32);
            location.1 += sines[&compass] * (ins.1 as i32);
        }
        'N' => location.1 += ins.1 as i32,
        'S' => location.1 -= ins.1 as i32,
        'E' => location.0 += ins.1 as i32,
        'W' => location.0 -= ins.1 as i32,
        'R' => compass = (compass - (ins.1 as i32) + 360) % 360,
        'L' => compass = (compass + (ins.1 as i32) + 360) % 360,
        _ => unreachable!(),
    });
    println!("{:?}, {:?}", location.0.abs() + location.1.abs(), compass);
}

fn right(angle: u32, wp: &(i32, i32)) -> (i32, i32) {
    match angle {
        90 => (wp.1, wp.0 * -1),
        180 => (wp.0 * -1, wp.1 * -1),
        270 => (wp.1 * -1, wp.0),
        _ => (0, 0),
    }
}

fn left(angle: u32, wp: &(i32, i32)) -> (i32, i32) {
    match angle {
        90 => (wp.1 * -1, wp.0),
        180 => (wp.0 * -1, wp.1 * -1),
        270 => (wp.1, wp.0 * -1),
        _ => (0, 0),
    }
}

fn solve_2(route: &Vec<(char, u32)>) {
    let mut pos = (0i32, 0i32);
    let mut wp = (10i32, 1i32);
    route.iter().for_each(|ins| match ins.0 {
        'F' => {
            pos.0 += (ins.1 as i32) * wp.0;
            pos.1 += (ins.1 as i32) * wp.1;
        }
        'N' => wp.1 += ins.1 as i32,
        'S' => wp.1 -= ins.1 as i32,
        'E' => wp.0 += ins.1 as i32,
        'W' => wp.0 -= ins.1 as i32,
        'R' => wp = right((ins.1 + 360) % 360, &wp),
        'L' => wp = left((ins.1 + 360) % 360, &wp),
        _ => unreachable!(),
    });
    println!(
        "{:?}, {:?}",
        pos.0.abs() + pos.1.abs(),
        wp.0.abs() + wp.1.abs(),
    );
}

fn main() {
    let input = fs::read_to_string("./input_d12.txt").unwrap();
    let route = parse(input.split('\n'));
    solve_1(&route);
    solve_2(&route);
}
