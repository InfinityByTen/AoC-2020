use core::hash::Hash;
use itertools::iproduct;
use std::collections::HashSet;
use std::fs;

type Point3 = (i32, i32, i32);
type Point4 = (i32, i32, i32, i32);

fn active_neighbours<T: Eq + Hash>(set: &HashSet<T>, neighbours: impl Iterator<Item = T>) -> usize {
    let is_active = |point: &T, set: &HashSet<T>| set.contains(&point);
    neighbours.filter(|x| is_active(x, set)).count()
}

fn neighbours3(point: Point3) -> impl Iterator<Item = (i32, i32, i32)> {
    let (i, j, k) = (point.0, point.1, point.2);
    iproduct!(i - 1..=i + 1, j - 1..=j + 1, k - 1..=k + 1)
        .filter(move |option| option != &(i, j, k))
}

fn neighbours4(point: Point4) -> impl Iterator<Item = (i32, i32, i32, i32)> {
    let (i, j, k, l) = (point.0, point.1, point.2, point.3);
    iproduct!(i - 1..=i + 1, j - 1..=j + 1, k - 1..=k + 1, l - 1..=l + 1)
        .filter(move |option| option != &(i, j, k, l))
}

fn solve_1(baseline: &HashSet<(i32, i32, i32)>) {
    let mut init = baseline.clone();
    (0..6).for_each(|_| {
        let mut copy = init.clone();
        let xlim: (i32, i32) = (
            init.iter().min_by_key(|(a, _, _)| a).unwrap().0 - 1,
            init.iter().max_by_key(|(a, _, _)| a).unwrap().0 + 1,
        );
        let ylim: (i32, i32) = (
            init.iter().min_by_key(|(_, b, _)| b).unwrap().1 - 1,
            init.iter().max_by_key(|(_, b, _)| b).unwrap().1 + 1,
        );
        let zlim: (i32, i32) = (
            init.iter().min_by_key(|(_, _, c)| c).unwrap().2 - 1,
            init.iter().max_by_key(|(_, _, c)| c).unwrap().2 + 1,
        );

        for point in iproduct!(
            (xlim.0)..=(xlim.1),
            (ylim.0)..=(ylim.1),
            (zlim.0)..=(zlim.1)
        ) {
            let is_active = init.contains(&point);
            let active_count = active_neighbours(&init, neighbours3(point));
            if is_active && !(2..=3).contains(&active_count) {
                copy.remove(&point);
            } else if !is_active && active_count == 3 {
                copy.insert(point);
            }
        }
        init = copy;
    });
    println!("{:?}", init.len());
}

fn solve_2(init: &HashSet<(i32, i32, i32)>) {
    let mut init_4 = init
        .iter()
        .map(|(i, j, k)| (*i, *j, *k, 0_i32))
        .collect::<HashSet<(i32, i32, i32, i32)>>();

    (0..6).for_each(|_| {
        let mut copy = init_4.clone();
        let xlim: (i32, i32) = (
            init_4.iter().min_by_key(|(a, _, _, _)| a).unwrap().0 - 1,
            init_4.iter().max_by_key(|(a, _, _, _)| a).unwrap().0 + 1,
        );
        let ylim: (i32, i32) = (
            init_4.iter().min_by_key(|(_, b, _, _)| b).unwrap().1 - 1,
            init_4.iter().max_by_key(|(_, b, _, _)| b).unwrap().1 + 1,
        );
        let zlim: (i32, i32) = (
            init_4.iter().min_by_key(|(_, _, c, _)| c).unwrap().2 - 1,
            init_4.iter().max_by_key(|(_, _, c, _)| c).unwrap().2 + 1,
        );
        let wlim: (i32, i32) = (
            init_4.iter().min_by_key(|(_, _, _, d)| d).unwrap().3 - 1,
            init_4.iter().max_by_key(|(_, _, _, d)| d).unwrap().3 + 1,
        );

        for point in iproduct!(
            (xlim.0)..=(xlim.1),
            (ylim.0)..=(ylim.1),
            (zlim.0)..=(zlim.1),
            (wlim.0)..=(wlim.1)
        ) {
            let is_active = init_4.contains(&point);
            let active_count = active_neighbours(&init_4, neighbours4(point));
            if is_active && !(2..=3).contains(&active_count) {
                copy.remove(&point);
            } else if !is_active && active_count == 3_usize {
                copy.insert(point);
            }
        }
        init_4 = copy;
    });
    println!("{:?}", init_4.len());
}

fn main() {
    let input = fs::read_to_string("./input_d17.txt").unwrap();
    let lines = input.split('\n');

    let mut init = HashSet::new();
    let mut count = 0;
    lines.for_each(|l| {
        l.chars()
            .enumerate()
            .filter(|(_, c)| c.eq(&'#'))
            .for_each(|(y, _)| {
                let _ = init.insert((count as i32, y as i32, 0_i32));
            });
        count += 1;
    });
    solve_1(&init);
    solve_2(&init);
}
