use itertools::Itertools;
use std::fs;

fn bezout(a: i64, b: i64) -> (i64, i64) {
    let (mut r, mut r_p) = (a, b);
    let (mut s, mut s_p) = (1, 0);
    let (mut t, mut t_p) = (0, 1);

    while r_p > 0 {
        let q = r / r_p;
        let (i, j, k) = (r_p, s_p, t_p);
        r_p = r % r_p;
        s_p = s - (q * s_p);
        t_p = t - (q * t_p);
        r = i;
        s = j;
        t = k;
    }
    (s, t)
}

fn crt(problem: &Vec<(u32, u32)>) -> i64 {
    let prod = problem.iter().map(|(_, n)| *n as i64).product::<i64>();
    problem
        .iter()
        .map(|(r, n)| {
            let c = prod / (*n as i64);
            (*r as i64) * bezout(*n as i64, c).1 * (c)
        })
        .sum::<i64>()
}

fn solve_2(schedule: &str) {
    let busses = schedule
        .split(',')
        .enumerate()
        .filter(|entry| !entry.1.eq("x"))
        .map(|x| (x.0 as u32, x.1.parse::<u32>().unwrap()));

    let problem = busses
        .map(|(r, n)| if r > 0 { (n - (r % n), n) } else { (r, n) })
        .collect::<Vec<(u32, u32)>>();
    println!("{:?}", problem);

    let prod = problem.iter().map(|(_, n)| *n as i64).product::<i64>();
    let mut x: i64 = crt(&problem);
    x += ((x.abs() / prod) + 1) * prod;
    println!("{:?}", x);
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
        .enumerate()
        .map(|(id, bus)| (bus - (time % bus), id));
    let res = remainders.min().unwrap();
    println!("{:?}", res.0 * busses[res.1]);
}

fn main() {
    let input = fs::read_to_string("./input_d13.txt").unwrap();
    solve_1(input.split('\n').collect_tuple().unwrap());
    solve_2(input.split('\n').collect_tuple::<(_, &str)>().unwrap().1);
}
