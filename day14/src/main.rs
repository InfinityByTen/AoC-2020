use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use text_io::scan;

fn solve_1<'a>(input: impl Iterator<Item = &'a str>) {
    let lines = input
        .map(|l| l.split(" = ").collect_tuple().unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut mmap = HashMap::new();
    let mut it = lines.iter();
    let (mut o_mask, mut a_mask): (u64, u64) = (0, 0);
    while let Some((ins, val)) = it.next() {
        if ins.eq(&"mask") {
            let o_str = &val
                .chars()
                .map(|c| if c.eq(&'X') { '0' } else { c })
                .collect::<String>();
            let a_str = &val
                .chars()
                .map(|c| if c.eq(&'X') { '1' } else { c })
                .collect::<String>();
            o_mask = u64::from_str_radix(o_str, 2).unwrap();
            a_mask = u64::from_str_radix(a_str, 2).unwrap();
        } else if ins.contains("mem") {
            let given = val.parse::<u64>().unwrap();
            let mem: u64;
            scan!(ins.bytes()=>"mem[{}]",mem);
            mmap.entry(mem)
                .and_modify(|val: &mut u64| *val = (given | o_mask) & a_mask)
                .or_insert((given | o_mask) & a_mask);
        }
    }
    println!("{:?}", mmap.values().sum::<u64>());
}

fn generate_masks(pattern: String) -> Vec<u64> {
    let floating = pattern.chars().filter(|c| c.eq(&'X')).count();
    (0..2_usize.pow(floating.try_into().unwrap()))
        .map(|x| {
            let options = format!("{:b}", x);
            let mask = ("0").repeat(floating - options.len()) + &options;
            let mut biter = mask.chars();
            let mut modified = pattern.clone();
            while let Some(c) = biter.next() {
                modified = modified.replacen('X', &c.to_string(), 1);
            }
            u64::from_str_radix(&modified, 2).unwrap()
        })
        .collect::<Vec<u64>>()
}

fn solve_2<'a>(input: impl Iterator<Item = &'a str>) {
    let lines = input
        .map(|l| l.split(" = ").collect_tuple().unwrap())
        .collect::<Vec<(&str, &str)>>();
    let mut mmap = HashMap::new();

    let mut it = lines.iter();
    let mut orig = String::new();
    while let Some((ins, val)) = it.next() {
        if ins.eq(&"mask") {
            orig = val.to_string();
        } else if ins.contains("mem") {
            let mem: u64;
            scan!(ins.bytes()=>"mem[{}]",mem);
            let value = val.parse::<u64>().unwrap();
            let applied = u64::from_str_radix(
                &orig
                    .chars()
                    .zip(format!("{:036b}", mem).chars())
                    .map(|(m, v)| match m {
                        'X' => '0',
                        '1' => '1',
                        '0' => v,
                        _ => unreachable!(),
                    })
                    .collect::<String>(),
                2,
            )
            .unwrap();
            generate_masks(orig.to_string()).iter().for_each(|entry| {
                mmap.entry(entry | applied)
                    .and_modify(|val: &mut u64| *val = value)
                    .or_insert(value);
            });
        }
    }
    println!("{:?}", mmap.values().sum::<u64>());
}

fn main() {
    let input = fs::read_to_string("./input_d14.txt").unwrap();
    solve_1(input.split('\n'));
    solve_2(input.split('\n'));
}
