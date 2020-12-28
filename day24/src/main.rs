#![allow(dead_code)]
use std::collections::HashMap;
use std::fs;

fn solve_1<'a>(input: impl Iterator<Item = &'a str>) -> HashMap<(i32, i32), u32> {
    let mut ident = HashMap::new();
    let offesets = [(1_i32, 1_i32), (1, 0), (0, 1), (-1, 0), (-1, -1), (0, -1)];
    let traverse = |pos: &(i32, i32), off: &(i32, i32)| (pos.0 + off.0, pos.1 + off.1);
    input.for_each(|tile| {
        let it = &mut tile.chars().peekable();
        let mut pos = (0_i32, 0_i32);
        while it.peek().is_some() {
            let n = it.peek().unwrap();
            if n == &'s' || n == &'n' {
                match it.take(2).collect::<String>().as_str() {
                    "se" => pos = traverse(&pos, &offesets[0]),
                    "sw" => pos = traverse(&pos, &offesets[2]),
                    "ne" => pos = traverse(&pos, &offesets[5]),
                    "nw" => pos = traverse(&pos, &offesets[4]),
                    _ => unreachable!(),
                }
            } else {
                match it.next().unwrap() {
                    'w' => pos = traverse(&pos, &offesets[3]),
                    'e' => pos = traverse(&pos, &offesets[1]),
                    _ => unreachable!(),
                }
            }
        }
        ident.entry(pos).and_modify(|val| *val += 1).or_insert(1);
    });
    let res = ident.iter().filter(|(_, &count)| count % 2 == 1).count();
    println!("{:?}", res);
    ident
}

fn solve_2(pattern: &HashMap<(i32, i32), u32>) {
    let offsets = [(1_i32, 1_i32), (1, 0), (0, 1), (-1, 0), (-1, -1), (0, -1)];
    let adj = |&pos: &(_, _)| offsets.iter().map(move |of| (pos.0 + of.0, pos.1 + of.1));
    let is_black = |val: &u32| val % 2 == 1;
    let adj_black_count = |pos: &(i32, i32), layout: &HashMap<(i32, i32), u32>| {
        adj(&pos)
            .filter_map(|tile| layout.get(&tile))
            .filter(|x| is_black(x))
            .count()
    };

    let mut baseline = pattern.clone();
    let mut black_count = 0;
    for _ in 0..100 {
        baseline
            .clone()
            .iter()
            .flat_map(|(&pos, _)| adj(&pos))
            .for_each(|tile| {
                baseline.entry(tile).or_insert(0);
            });
        let mut next_day = baseline.clone();
        baseline.iter().for_each(|(position, count)| {
            let adj_count = adj_black_count(position, &baseline);
            if is_black(count) && (adj_count == 0 || adj_count > 2) {
                next_day.entry(*position).and_modify(|val| *val += 1);
            } else if !is_black(count) && adj_count == 2 {
                next_day.entry(*position).and_modify(|val| *val += 1);
            }
        });
        baseline = next_day.clone();
        black_count = next_day.iter().filter(|(_, count)| is_black(count)).count();
    }
    println!("final black count {:?}", black_count);
}

fn main() {
    let input = fs::read_to_string("./input_d24.txt").unwrap();
    let lines = input.split('\n');
    let map = solve_1(lines);
    solve_2(&map);
}
