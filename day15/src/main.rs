use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs;

fn solve(init: &Vec<u32>, mut account: HashMap<u32, (i32, i32)>, terminus: usize) {
    let (mut last, mut next) = (0, *init.last().unwrap());
    for i in (init.len() + 1)..=terminus {
        if let Some(previous) = account.get(&last) {
            next = u32::try_from(previous.0).unwrap()
                - u32::try_from(previous.1).unwrap_or(previous.0.try_into().unwrap());
            account
                .entry(next)
                .and_modify(|val| {
                    val.1 = val.0;
                    val.0 = i.try_into().unwrap();
                })
                .or_insert((i.try_into().unwrap(), -1));
            last = next;
        }
    }
    println!("{:?}", next);
}

fn main() {
    let input = fs::read_to_string("./input_d15.txt").unwrap();
    let init = input
        .split(',')
        .map(|a| a.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let account: HashMap<u32, (i32, i32)> = init
        .iter()
        .enumerate()
        .map(|(index, num)| (num.clone(), ((index + 1) as i32, -1)))
        .collect();
    solve(&init, account.clone(), 2020);
    solve(&init, account.clone(), 30000000);
}
