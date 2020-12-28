use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;

fn eval_bracket(operations: &VecDeque<char>, operands: &VecDeque<u64>) -> u64 {
    let mut res = operands[0];
    let mut it = operands.iter().skip(1);
    operations.iter().for_each(|op| match op {
        '+' => res += it.next().unwrap(),
        '*' => res *= it.next().unwrap(),
        _ => unreachable!(),
    });
    res
}

fn eval_bracket_2(operations: &VecDeque<char>, operands: &VecDeque<u64>) -> u64 {
    let mut res: u64 = 1;
    let mut stack = vec![operands[0]];
    let mut num = operands.iter().skip(1);
    operations.iter().for_each(|op| match op {
        '+' => stack.push(*num.next().unwrap()),
        '*' => {
            res *= stack.iter().sum::<u64>();
            stack.clear();
            stack.push(*num.next().unwrap());
        }
        _ => unreachable!(),
    });
    res *= stack.iter().sum::<u64>();
    res
}

fn solve(problems: &Vec<String>, evaluator: fn(&VecDeque<char>, &VecDeque<u64>) -> u64) {
    let res = problems
        .iter()
        .map(|entry| {
            let op_stack = RefCell::new(Vec::new());
            let num_stack = RefCell::new(Vec::new());
            let (mut operations, mut operands) = (op_stack.borrow_mut(), num_stack.borrow_mut());

            entry.chars().for_each(|c| {
                if c.is_numeric() {
                    operands.push(c.to_digit(10).unwrap() as u64);
                } else if c.eq(&')') {
                    let (mut bracket_ops, mut bracket_num) = (VecDeque::new(), VecDeque::new());
                    while operations.last().copied().unwrap() != '(' {
                        bracket_num.push_front(operands.pop().unwrap());
                        bracket_ops.push_front(operations.pop().unwrap());
                    }
                    bracket_num.push_front(operands.pop().unwrap());
                    operands.push(evaluator(&bracket_ops, &bracket_num));
                    operations.pop();
                } else {
                    operations.push(c)
                }
            });
            // Can make this gobble up the vec and convert into VecDeque...
            let ops = operations.iter().cloned().collect::<VecDeque<char>>();
            let nums = operands.iter().cloned().collect::<VecDeque<u64>>();
            evaluator(&ops, &nums)
        })
        .sum::<u64>();
    println!("{:?}", res);
}

fn main() {
    let input = fs::read_to_string("./input_d18.txt").unwrap();
    let problems = input
        .split('\n')
        .map(|l| {
            let mut line = l.to_string();
            line.retain(|c| !c.is_whitespace());
            line
        })
        .collect::<Vec<String>>();
    solve(&problems, eval_bracket);
    solve(&problems, eval_bracket_2);
}
