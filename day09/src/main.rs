use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

// wanted to make this a closure, but then it created problems with mutable and immutable borrows.
fn scan_preamble(x: &u64, preamble: &HashSet<u64>) -> bool {
    preamble
        .iter()
        .any(|y| (*x as i64 - *y as i64) >= 0 && preamble.get(&(x - y)).is_some())
}

fn solve_part_1<'a>(stream: impl Iterator<Item = &'a str> + Clone, psize: usize) -> u64 {
    let mut numbers = stream.clone();
    let mut preamble = HashSet::new();
    let mut ordered_ref: VecDeque<u64> = VecDeque::with_capacity(psize);
    for _i in 0..psize {
        let num = numbers.next().unwrap().parse::<u64>().unwrap();
        let _ = preamble.insert(num);
        ordered_ref.push_back(num);
    }
    // TODO: make this terse
    let mut result = 0;
    numbers.any(|num| {
        let val = num.parse::<u64>().unwrap();
        if scan_preamble(&val, &preamble) {
            let _ = preamble.remove(&ordered_ref[0]);
            preamble.insert(val);
            ordered_ref.push_back(val);
            ordered_ref.pop_front();
            false
        } else {
            println!("{:?}", val);
            result = val;
            true
        }
    });
    result
}

fn solve_part_2<'a>(stream: impl Iterator<Item = &'a str>, val: u64) {
    let list = stream
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut prefix_sum = Vec::with_capacity(list.len() + 1);
    prefix_sum.push(0);
    for i in 0..list.len() {
        prefix_sum.push(prefix_sum[i] + list[i])
    }
    for window in 2..list.len() {
        for i in window..prefix_sum.len() {
            if prefix_sum[i] - prefix_sum[i - window] == val {
                let slice = &list[i - window..=(i - 1)];
                println!(
                    "Eureka: {:?}",
                    slice.iter().min().unwrap() + slice.iter().max().unwrap()
                );
                break;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input_d9.txt").unwrap();
    let result = solve_part_1(input.split('\n'), 25);
    let _solved = solve_part_2(input.split('\n'), result);
}
