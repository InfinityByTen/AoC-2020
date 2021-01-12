#![allow(dead_code)]
use itertools::iproduct;
use std::collections::HashSet;
use std::fs;

type Point = (i32, i32, i32, i32);

fn active_neighbours(
    set: &HashSet<Point>,
    neighbours: impl Iterator<Item = Point>,
    point: &Point,
) -> usize {
    neighbours
        .map(|x| {
            let mut res = 0;
            if set.contains(&x) {
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

fn neighbours(point: Point) -> impl Iterator<Item = Point> {
    let (i, j, k, l) = (point.0, point.1, point.2, point.3);
    iproduct!(i - 1..=i + 1, j - 1..=j + 1, k - 1..=k + 1, l - 1..=l + 1)
        .filter(move |option| option != &(i, j, k, l) && option.2 >= 0 && option.3 >= 0)
}

fn solve(mut active_set: HashSet<Point>, dim: &u8) {
    (0..6).for_each(|_| {
        let mut copy = active_set.clone();
        let xlim: (i32, i32) = (
            active_set.iter().min_by_key(|(a, _, _, _)| a).unwrap().0 - 1,
            active_set.iter().max_by_key(|(a, _, _, _)| a).unwrap().0 + 1,
        );
        let ylim: (i32, i32) = (
            active_set.iter().min_by_key(|(_, b, _, _)| b).unwrap().1 - 1,
            active_set.iter().max_by_key(|(_, b, _, _)| b).unwrap().1 + 1,
        );
        let zlim: (i32, i32) = (
            0,
            active_set.iter().max_by_key(|(_, _, c, _)| c).unwrap().2 + 1,
        );
        let wlim: (i32, i32) = match dim {
            3 => (0, 0),
            4 => (
                0,
                active_set.iter().max_by_key(|(_, _, _, d)| d).unwrap().3 + 1,
            ),
            _ => unreachable!(),
        };

        for point in iproduct!(
            (xlim.0)..=(xlim.1),
            (ylim.0)..=(ylim.1),
            (zlim.0)..=(zlim.1),
            (wlim.0)..=(wlim.1)
        ) {
            let is_active = active_set.contains(&point);
            let active_count = active_neighbours(&active_set, neighbours(point), &point);
            if is_active && !(2..=3).contains(&active_count) {
                copy.remove(&point);
            } else if !is_active && active_count == 3_usize {
                copy.insert(point);
            }
        }
        active_set = copy;
    });
    let in_plane = active_set.iter().filter(|x| x.2 == 0 && x.3 == 0).count();
    let orthogonal_planes = active_set.iter().filter(|x| x.2 == 0 || x.3 == 0).count() - in_plane;
    let in_free_zone = active_set.iter().filter(|x| x.2 > 0 && x.3 > 0).count();
    // final count = (active_set.len() - in_free_zone - orthogonal_planes)
    //         + (in_free_zone * 4)
    //         + orthogonal_planes * 2
    println!(
        "{:?}",
        active_set.len() + (in_free_zone * 3) + orthogonal_planes
    );
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
                let _ = init.insert((count as i32, y as i32, 0_i32, 0_i32));
            });
        count += 1;
    });
    solve(init.clone(), &3);
    solve(init.clone(), &4);
}
