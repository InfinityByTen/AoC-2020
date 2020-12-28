use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::RangeInclusive;
use text_io::scan;

type Rule<'a> = (&'a str, (RangeInclusive<u32>, RangeInclusive<u32>));
fn parse_ticket<'a>(input: impl Iterator<Item = &'a str>) -> Vec<u32> {
    input
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn is_valid(val: &u32, rule: &Rule) -> bool {
    (rule.1).0.contains(val) || (rule.1).1.contains(val)
}

fn solve_1(others: &Vec<Vec<u32>>, rules: &Vec<Rule>) {
    let res = others
        .iter()
        .flat_map(|entry| entry.iter())
        .filter(|val| !rules.iter().any(|rule| is_valid(val, rule)))
        .sum::<u32>();
    println!("{:?}", res);
}

fn process_ticket(start: &mut Vec<HashSet<usize>>, rules: &Vec<Rule>, ticket: &Vec<u32>) {
    (0..ticket.len()).for_each(|i| {
        start[i] = start[i]
            .intersection(
                &rules
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| is_valid(&ticket[i], &r))
                    .map(|(index, _)| index)
                    .collect::<HashSet<usize>>(),
            )
            .cloned()
            .collect();
    });
}

fn reduce_mapping(start: &mut Vec<HashSet<usize>>) {
    let mut fixed = HashSet::new();
    let mut copy = start.clone();
    while let Some(minimal) = copy
        .iter()
        .enumerate()
        .filter(|entry| !fixed.contains(&entry.0))
        .min_by_key(|(_, set)| set.len())
    {
        (0..start.len())
            .filter(|id| *id != minimal.0)
            .for_each(|f_id| start[f_id] = start[f_id].difference(minimal.1).cloned().collect());
        fixed.insert(minimal.0);
        copy = start.clone();
    }
}

fn solve_2(others: &Vec<Vec<u32>>, rules: &Vec<Rule>, ticket: &Vec<u32>) {
    // let departure_indices: Vec<usize>;
    let valid = others
        .iter()
        .filter(|entry| {
            entry
                .iter()
                .all(|val| rules.iter().any(|rule| is_valid(val, rule)))
        })
        .collect::<Vec<&Vec<u32>>>();

    // mapping from ticket position to the rules.
    let mut start = vec![(0..rules.len()).collect::<HashSet<usize>>(); ticket.len()];
    process_ticket(&mut start, &rules, ticket);
    valid
        .iter()
        .for_each(|t| process_ticket(&mut start, &rules, t));
    // reduce to one rule per position
    reduce_mapping(&mut start);

    let rule_to_index = start
        .iter()
        .enumerate()
        .map(|(index, rule)| (*rule.iter().next().unwrap(), index))
        .collect::<HashMap<usize, usize>>();

    println!("{:?}", rule_to_index);
    println!(
        "{:?}",
        (0..6)
            .map(|id| ticket[rule_to_index[&id]] as u64)
            .product::<u64>()
    );
}

fn main() {
    let input = fs::read_to_string("./input_d16.txt").unwrap();
    let mut sections = input.split("\n\n");

    let rules: Vec<(&str, (RangeInclusive<u32>, RangeInclusive<u32>))> = sections
        .next()
        .unwrap()
        .split('\n')
        .map(|rule| {
            let (name, ranges) = rule.split(": ").collect_tuple().unwrap();
            let (a, b, c, d);
            scan!(ranges.bytes()=> "{}-{} or {}-{}",a,b,c,d);
            (name, (a..=b, c..=d))
        })
        .collect();

    let mut ticket = sections.next().unwrap().split('\n').skip(1);
    let my_precious = parse_ticket(ticket.next().unwrap().split(','));

    let nearby = sections.next().unwrap().split('\n').skip(1);
    let others = nearby
        .map(|other| parse_ticket(other.split(',')))
        .collect::<Vec<_>>();

    solve_1(&others, &rules);
    solve_2(&others, &rules, &my_precious);
}
