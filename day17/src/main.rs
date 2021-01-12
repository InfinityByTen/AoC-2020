#![allow(dead_code)]
// use core::hash::Hash;
use itertools::iproduct;
use std::collections::HashSet;
use std::fs;

type Point3 = (i32, i32, i32);
type Point4 = (i32, i32, i32, i32);

fn is_in_plane3(point: &Point3, neighbour: &Point3) -> bool {
    point.2 == neighbour.2
}

fn active_neighbours3(
    set: &HashSet<Point3>,
    neighbours: impl Iterator<Item = Point3>,
    point: &Point3,
) -> usize {
    let is_active = |point: &Point3, set: &HashSet<Point3>| set.contains(&point);
    neighbours
        .map(|x| {
            if is_active(&x, set) {
                if point.2 == 0 && !is_in_plane3(point, &x) {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        })
        .sum()
}

fn active_neighbours4(
    set: &HashSet<Point4>,
    neighbours: impl Iterator<Item = Point4>,
    point: &Point4,
) -> usize {
    let is_active = |point: &Point4, set: &HashSet<Point4>| set.contains(&point);
    neighbours
        .map(|x| {
            let mut res = 0;
            if is_active(&x, set) {
                if point.2 == 0 && x.2 != 0 {
                    res += 2;
                }
                if point.3 == 0 && x.3 != 0 {
                    res += 2;
                }
                if res == 0 {
                    res = 1;
                }
            }
            res
        })
        .sum()
}

fn neighbours3(point: Point3) -> impl Iterator<Item = (i32, i32, i32)> {
    let (i, j, k) = (point.0, point.1, point.2);
    iproduct!(i - 1..=i + 1, j - 1..=j + 1, k - 1..=k + 1)
        .filter(move |option| option != &(i, j, k) && option.2 >= 0)
}

fn neighbours4(point: Point4) -> impl Iterator<Item = (i32, i32, i32, i32)> {
    let (i, j, k, l) = (point.0, point.1, point.2, point.3);
    iproduct!(i - 1..=i + 1, j - 1..=j + 1, k - 1..=k + 1, l - 1..=l + 1)
        .filter(move |option| option != &(i, j, k, l) && option.2 >= 0 && option.3 >= 0)
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
        let zlim: (i32, i32) = (0, init.iter().max_by_key(|(_, _, c)| c).unwrap().2 + 1);

        for point in iproduct!(
            (xlim.0)..=(xlim.1),
            (ylim.0)..=(ylim.1),
            (zlim.0)..=(zlim.1)
        ) {
            let is_active = init.contains(&point);
            let active_count = active_neighbours3(&init, neighbours3(point), &point);
            if is_active && !(2..=3).contains(&active_count) {
                copy.remove(&point);
            } else if !is_active && active_count == 3 {
                copy.insert(point);
            }
        }
        init = copy;
    });
    let out_of_plane = init.iter().filter(|x| x.2 > 0).count();
    println!("{:?}", init.len() + out_of_plane);
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
        let zlim: (i32, i32) = (0, init_4.iter().max_by_key(|(_, _, c, _)| c).unwrap().2 + 1);
        let wlim: (i32, i32) = (0, init_4.iter().max_by_key(|(_, _, _, d)| d).unwrap().3 + 1);

        for point in iproduct!(
            (xlim.0)..=(xlim.1),
            (ylim.0)..=(ylim.1),
            (zlim.0)..=(zlim.1),
            (wlim.0)..=(wlim.1)
        ) {
            let is_active = init_4.contains(&point);
            let active_count = active_neighbours4(&init_4, neighbours4(point), &point);
            if is_active && !(2..=3).contains(&active_count) {
                copy.remove(&point);
            } else if !is_active && active_count == 3_usize {
                copy.insert(point);
            }
        }
        init_4 = copy;
    });
    let baa = init_4.iter().filter(|x| x.2 == 0 && x.3 == 0).count();
    let boo = init_4.iter().filter(|x| x.2 == 0 || x.3 == 0).count();
    let oof_11 = init_4.iter().filter(|x| x.2 > 0 && x.3 > 0).count();
    println!("{:?}", (init_4.len() - oof_11) + (oof_11 * 4) + (boo - baa));
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
