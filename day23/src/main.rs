#![allow(dead_code)]
use std::collections::VecDeque;
use std::iter::successors;

fn indexed_list(mut input: Vec<usize>, mut current: usize, max: usize) {
    let size = input.len() - 1;
    for _ in 0..max {
        let picked = successors(Some(input[current]), |&p| Some(input[p]))
            .take(3)
            .collect::<Vec<_>>();
        let mut destination = if current > 1 { current - 1 } else { size };
        while picked.iter().find(|x| x == &&destination).is_some() {
            destination -= 1;
            if destination < 1 {
                destination = size;
            }
        }
        input[current] = input[picked[2]];
        input[picked[2]] = input[destination];
        input[destination] = picked[0];
        current = input[current];
    }
    if max < 101 {
        let res = successors(Some(input[1]), |&p| Some(input[p]))
            .take(size - 1)
            .collect::<Vec<_>>();
        println!("{:?}", res);
        return;
    }
    println!(
        "Enough! Give me my stars now! {:?}",
        input[1] * input[input[1]]
    );
}

fn main() {
    let input = vec![9, 4, 2, 3, 8, 7, 6, 1, 5];
    let mut indices = vec![0; 10];
    input.windows(2).for_each(|pair| indices[pair[0]] = pair[1]);
    indices[*input.last().unwrap()] = input[0];
    indexed_list(indices.clone(), input[0], 100);

    let (big_size, max_iter) = (1_000_000, 10_000_000);
    indices[*input.last().unwrap()] = 10;
    indices.extend(successors(Some(11_usize), |n| Some(n + 1)).take(big_size - 9));
    indices[big_size] = input[0];
    indexed_list(indices.clone(), input[0], max_iter);
}

//legacy
// don't even think about part2
fn solve_naive_vec_deque(mut input: VecDeque<u64>, max: u64) {
    let mut current = (input[0], 0);
    let size = input.len();
    for _i in 0..max {
        let pick = (1..=3_usize)
            .map(|_| {
                let to_pick = if current.1 >= (input.len() - 1) {
                    0
                } else {
                    current.1 + 1
                };
                input.remove(to_pick).unwrap()
            })
            .collect::<Vec<u64>>();
        let mut destination = if current.0 > 1 {
            current.0 - 1_u64
        } else {
            size as u64
        };
        while pick.iter().find(|&x| x == &destination).is_some() {
            destination -= 1;
            if destination < 1 {
                destination = size as u64;
            }
        }
        let loc = input
            .iter()
            .cloned()
            .enumerate()
            .find(|x| x.1 == destination)
            .unwrap();
        pick.iter()
            .enumerate()
            .for_each(|(idx, val)| input.insert(loc.0 + idx + 1, *val));
        let next_c = input.iter().find(|&x| x == &current.0).unwrap();
        current = (
            input[(next_c + 1) as usize % size],
            (next_c + 1) as usize % size,
        );
    }
    println!("end: {:?}", input);
}
