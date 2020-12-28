use std::fs;

fn solve_part_1(list: &Vec<u32>) {
    let mut iter = list.iter().peekable();
    let (mut step_1, mut step_3) = (0, 1); // for device
    while let (Some(val), Some(next)) = (iter.next(), iter.peek()) {
        match *next - *val {
            1 => step_1 += 1,
            3 => step_3 += 1,
            _ => unreachable!(),
        }
    }
    if list[0] == 1 {
        step_1 += 1;
    } else {
        step_3 += 1;
    }
    println!("{:?}", step_1 * step_3);
}

fn solve_part_2(list: &Vec<u32>) {
    let mut iter = list.iter().peekable();
    let mut c_block = 0;
    if list[0] == 1 {
        c_block += 1
    };
    let mut prod: u64 = 1;
    let mut compute = |x| match x {
        4 => prod *= 7,
        3 => prod *= 4,
        2 => prod *= 2,
        _ => prod *= 1,
    };
    while let (Some(val), Some(next)) = (iter.next(), iter.peek()) {
        match *next - *val {
            1 => c_block += 1,
            3 => {
                compute(c_block);
                c_block = 0;
            }
            _ => unreachable!(),
        }
    }
    compute(c_block);
    println!("{:?}", prod);
}

fn main() {
    let stream = fs::read_to_string("./input_d10.txt").unwrap();
    let mut list = stream
        .split('\n')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    list.sort();
    solve_part_1(&list);
    solve_part_2(&list);
}
