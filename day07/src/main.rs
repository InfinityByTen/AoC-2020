use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn parse<'a>(input: impl Iterator<Item = &'a str>) -> HashMap<String, Vec<String>> {
    let mut rule_book = HashMap::new();
    input.for_each(|entry| {
        let (parent, children) = entry.split("bags contain").collect_tuple().unwrap();
        let bar = &parent.replace(" ", "");
        let _foo = if !children.contains("no other") {
            children.split(',').for_each(|x| {
                let mut bla = x.split(" bag").next().unwrap().to_string();
                bla.retain(|c| !(c.is_numeric() || c.is_whitespace()));
                rule_book
                    .entry(bla)
                    .and_modify(|parents: &mut Vec<String>| parents.push(bar.to_string()))
                    .or_insert(vec![bar.to_string()]);
            })
        } else {
            ()
        };
    });
    rule_book
}

fn solve_part_1(input: &HashMap<String, Vec<String>>) {
    // println!("{:?}", input);
    let mut keys: Vec<String> = input.get("shinygold").unwrap().to_vec();
    let mut set = HashSet::<String>::new();
    while let Some(upper) = keys.pop() {
        // println!("{:?}", upper);
        set.insert(upper.clone());
        match input.get(&upper) {
            Some(even_upper) => even_upper.iter().for_each(|n| {
                if set.get(n).is_none() {
                    keys.push(n.to_string())
                }
            }),
            None => (),
        }
        // println!("{:?}", keys);
    }
    println!("{:?}", set.len());
}

fn main() {
    let input = fs::read_to_string("./input_d7.txt").unwrap();
    let rule_book = parse(input.split('\n'));
    solve_part_1(&rule_book);
}
