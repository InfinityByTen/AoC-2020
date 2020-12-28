#[allow(dead_code)]
use itertools::Itertools;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs;
use text_io::scan;

#[derive(Debug, Clone)]
struct Rule {
    val: Option<char>,
    options: Option<Vec<VecDeque<u32>>>,
}

fn process_rule(rule: &str) -> Rule {
    if rule.contains('|') {
        let ops = rule.split(" | ").collect_tuple::<(&str, &str)>().unwrap();
        let options = vec![
            ops.0
                .split(' ')
                .map(|r| r.parse::<u32>().unwrap())
                .collect::<VecDeque<u32>>(),
            ops.1
                .split(' ')
                .map(|r| r.parse::<u32>().unwrap())
                .collect::<VecDeque<u32>>(),
        ];
        Rule {
            val: None,
            options: Some(options),
        }
    } else if rule.contains('"') {
        let ch: char;
        scan!(rule.bytes() => "\"{}\"", ch);
        Rule {
            val: Some(ch),
            options: None,
        }
    } else {
        let seq = vec![rule
            .split(' ')
            .map(|r| r.parse::<u32>().unwrap())
            .collect::<VecDeque<u32>>()];
        Rule {
            val: None,
            options: Some(seq),
        }
    }
}

fn decode(
    rules: &HashMap<u32, Rule>,
    mut template: String,
    mut queue: VecDeque<u32>,
    mut possibilities: &mut HashSet<String>,
) {
    let next = queue.pop_front().unwrap();
    if let Some(options) = &rules[&next].options {
        options.iter().for_each(|option| {
            let mut temp_queue = queue.clone();
            option.iter().rev().for_each(|&entry| {
                temp_queue.push_front(entry);
            });
            decode(rules, template.clone(), temp_queue, &mut possibilities);
        });
    } else {
        template = template.to_owned() + &rules[&next].val.unwrap().to_string();
        if queue.is_empty() {
            possibilities.insert(template.to_string());
        } else {
            decode(rules, template, queue, &mut possibilities);
        }
    }
}

fn solve_1(rules: &HashMap<u32, Rule>, messages: &Vec<&str>) {
    let mut possibilities = HashSet::new();
    let template = "".to_string();
    let queue = rules[&0].options.as_ref().unwrap()[0]
        .iter()
        .map(|&x| x)
        .collect::<VecDeque<u32>>();
    decode(rules, template, queue, &mut possibilities);
    let res = messages
        .iter()
        .filter(|message| possibilities.contains(&message.to_string()))
        .count();
    println!("{:?}", res);
}

fn main() {
    let input = fs::read_to_string("./input_d19.txt").unwrap();
    let mut sections = input.split("\n\n");

    let rules = sections
        .next()
        .unwrap()
        .split('\n')
        .map(|r| {
            let (id, exp) = r.split(": ").collect_tuple().unwrap();
            (id.parse::<u32>().unwrap(), process_rule(exp))
        })
        .collect::<HashMap<u32, Rule>>();

    let messages = sections.next().unwrap().split('\n').collect::<Vec<&str>>();
    solve_1(&rules, &messages);
}
