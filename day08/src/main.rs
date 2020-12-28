use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;

fn parse<'a>(input: impl Iterator<Item = &'a str>) -> Vec<(String, i32)> {
    input
        .map(|ins| {
            let (op, val) = ins.split(' ').collect_tuple().unwrap();
            (op.to_string(), val.parse::<i32>().unwrap())
        })
        .collect::<Vec<(String, i32)>>()
}

fn run_simulation(code: &Vec<(String, i32)>) -> (i32, bool) {
    let mut executed = HashSet::new();
    let mut pos: usize = 0;
    let mut acc = 0;
    while executed.get(&pos).is_none() {
        if pos >= code.len() {
            return (acc, false);
        } else {
            executed.insert(pos);
            let (op, val) = &code[pos];
            match (op.as_str(), val) {
                ("nop", _) => pos += 1,
                ("jmp", val) => pos = usize::try_from(pos as i32 + val).unwrap(),
                ("acc", val) => {
                    acc += val;
                    pos += 1;
                }
                _ => unreachable!(),
            }
        }
    }
    (acc, true)
}

fn solve_part_1(code: &Vec<(String, i32)>) {
    let (acc, is_infinite) = run_simulation(code);
    println!("acc: {:?}, {:?}", acc, is_infinite);
}

fn solve_part_2(code: &Vec<(String, i32)>) {
    let mut bug_opt = code
        .iter()
        .enumerate()
        .filter(|(_pos, ins)| ins.0.eq("jmp") || ins.0.eq("nop"));
    while let Some((pos, ins)) = bug_opt.next() {
        let fix = match ins.0.as_str() {
            "jmp" => "nop",
            "nop" => "jmp",
            _ => unreachable!(),
        };
        let mut modified = code.clone();
        modified[pos].0 = fix.to_string();
        let (acc, is_infinite) = run_simulation(&modified);
        if !is_infinite {
            println!("Correct Exit: {:?}", acc);
            break;
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input_d8.txt").unwrap();
    let code = parse(input.split('\n'));
    solve_part_1(&code);
    solve_part_2(&code);
}
