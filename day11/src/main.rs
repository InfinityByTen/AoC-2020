use itertools::Itertools;
use std::cmp::min;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;

fn scan_1(plan: &Vec<Vec<char>>) -> HashMap<(i32, i32), Vec<(usize, usize)>> {
    let row: i32 = plan.len().try_into().unwrap();
    let col: i32 = plan[0].len().try_into().unwrap();
    let valid_tuple = |(i, j)| i >= 0 && j >= 0 && i < row && j < col;
    let valid_seat = |(i, j): (usize, usize)| r#"#|L"#.contains(plan[i][j]);
    let mut scanned = HashMap::new();
    for (i, j) in (0..row).cartesian_product(0..col) {
        let it = (i - 1..=i + 1).cartesian_product(j - 1..=j + 1);
        it.filter(|&index| {
            index != (i, j)
                && valid_tuple(index)
                && valid_seat((index.0 as usize, index.1 as usize))
        })
        .for_each(|x| {
            let val = (x.0 as usize, x.1 as usize);
            scanned
                .entry((i, j))
                .and_modify(|nb: &mut Vec<(usize, usize)>| nb.push(val))
                .or_insert(vec![val]);
        });
    }
    scanned
}

fn process_1(
    plan: &Vec<Vec<char>>,
    layout: &HashMap<(i32, i32), Vec<(usize, usize)>>,
) -> Option<Vec<Vec<char>>> {
    let row: i32 = plan.len().try_into().unwrap();
    let col: i32 = plan[0].len().try_into().unwrap();
    let valid_seat = |(i, j): (usize, usize)| r#"#|L"#.contains(plan[i][j]);
    let occupied = |(i, j)| plan[i as usize][j as usize].eq(&'#');
    let eval_box = |(i, j)| layout[&(i, j)].iter().filter(|&x| occupied(*x)).count();

    let mut copy = plan.clone();
    for location in (0usize..(row as usize)).cartesian_product(0usize..(col as usize)) {
        if valid_seat(location) {
            let status = eval_box((
                location.0.try_into().unwrap(),
                location.1.try_into().unwrap(),
            ));
            if plan[location.0][location.1].eq(&'L') && status == 0 {
                copy[location.0][location.1] = '#';
            }
            if plan[location.0][location.1].eq(&'#') && status >= 4 {
                copy[location.0][location.1] = 'L';
            }
        }
    }
    if copy.eq(plan) {
        None
    } else {
        Some(copy)
    }
}

fn scan_2(plan: &Vec<Vec<char>>) -> HashMap<(i32, i32), Vec<(usize, usize)>> {
    let row: i32 = plan.len().try_into().unwrap();
    let col: i32 = plan[0].len().try_into().unwrap();
    let valid_tuple = |(i, j)| i >= 0 && j >= 0 && i < row && j < col;
    let valid_seat = |(i, j): (i32, i32)| r#"#|L"#.contains(plan[i as usize][j as usize]);
    let mut scanned = HashMap::new();

    for (i, j) in (0..row).cartesian_product(0..col) {
        if valid_tuple((i, j)) {
            let mut visible = Vec::new();
            if let Some(p) = (0..i).rev().find(|x| valid_seat((*x, j))) {
                visible.push((p, j))
            }
            if let Some(q) = (i + 1..row).find(|x| valid_seat((*x, j))) {
                visible.push((q, j))
            }
            if let Some(r) = (0..j).rev().find(|x| valid_seat((i, *x))) {
                visible.push((i, r))
            }
            if let Some(s) = (j + 1..col).find(|x| valid_seat((i, *x))) {
                visible.push((i, s))
            }
            if let Some(a) = (1..min(i + 1, j + 1)).find(|x| valid_seat((i - x, j - x))) {
                visible.push((i - a, j - a));
            }
            if let Some(b) = (1..min(row - i, col - j)).find(|x| valid_seat((i + x, j + x))) {
                visible.push((i + b, j + b));
            }
            if let Some(c) = (1..min(i + 1, col - j)).find(|x| valid_seat((i - x, j + x))) {
                visible.push((i - c, j + c));
            }
            if let Some(d) = (1..min(row - i, j + 1)).find(|x| valid_seat((i + x, j - x))) {
                visible.push((i + d, j - d));
            }
            scanned.insert(
                (i, j),
                visible
                    .iter()
                    .map(|(i, j)| (*i as usize, *j as usize))
                    .collect(),
            );
        }
    }
    scanned
}

fn process_2(
    plan: &Vec<Vec<char>>,
    layout: &HashMap<(i32, i32), Vec<(usize, usize)>>,
) -> Option<Vec<Vec<char>>> {
    let row: i32 = plan.len().try_into().unwrap();
    let col: i32 = plan[0].len().try_into().unwrap();
    let valid_seat = |(i, j): (usize, usize)| r#"#|L"#.contains(plan[i][j]);
    let occupied = |(i, j)| plan[i as usize][j as usize].eq(&'#');
    let eval_box = |(i, j)| layout[&(i, j)].iter().filter(|&x| occupied(*x)).count();

    let mut copy = plan.clone();
    for location in (0usize..(row as usize)).cartesian_product(0usize..(col as usize)) {
        if valid_seat(location) {
            let status = eval_box((location.0 as i32, location.1 as i32));
            if plan[location.0][location.1].eq(&'L') && status == 0 {
                copy[location.0][location.1] = '#';
            }
            if plan[location.0][location.1].eq(&'#') && status >= 5 {
                copy[location.0][location.1] = 'L';
            }
        }
    }
    if copy.eq(plan) {
        None
    } else {
        Some(copy)
    }
}

fn solve(
    plan: &Vec<Vec<char>>,
    process: fn(
        &Vec<Vec<char>>,
        &HashMap<(i32, i32), Vec<(usize, usize)>>,
    ) -> Option<Vec<Vec<char>>>,
    scan: fn(plan: &Vec<Vec<char>>) -> HashMap<(i32, i32), Vec<(usize, usize)>>,
) -> usize {
    let mut start = plan.clone();
    let neighbourhood = scan(plan);
    loop {
        if let Some(result) = process(&start, &neighbourhood) {
            start = result;
        } else {
            break;
        }
    }
    start.iter().flatten().filter(|&y| y.eq(&'#')).count()
}

fn main() {
    let data = fs::read_to_string("./input_d11.txt").unwrap();
    let plan = data
        .split('\n')
        .map(|x| x.parse::<String>().unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("{:?}", solve(&plan, process_1, scan_1));
    println!("{:?}", solve(&plan, process_2, scan_2));
}
