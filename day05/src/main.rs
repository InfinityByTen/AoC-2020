use std::fs;

fn main() {
    let input = fs::read_to_string("./input_d5.txt").unwrap();

    let boarding_passes: Vec<_> = input.split("\n").collect();

    let ids: Vec<u32> = boarding_passes
        .iter()
        .map(|pass| {
            let bin = pass
                .chars()
                .map(|x| match x {
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    _ => 'X',
                })
                .collect::<String>();
            usize::from_str_radix(&bin, 2).unwrap() as u32
        })
        .collect();

    let max_id = *ids.iter().max().unwrap();
    let min_id = *ids.iter().min().unwrap();
    println!("Max ID: {:?}, Min ID: {:?}", max_id, min_id);
    let sum_id = max_id * (max_id + 1) / 2 - (min_id * (min_id - 1) / 2);
    println!("My seat: {:?}", sum_id - ids.iter().sum::<u32>());
}
