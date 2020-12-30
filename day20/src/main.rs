#![allow(dead_code)]
use itertools::iproduct;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::mem::discriminant;
use text_io::scan;

#[derive(Debug, Default, Hash, Clone)]
struct Tile {
    number: u64,
    pixels: Vec<Vec<char>>,
    borders: [String; 4],
    orientation: u8,
}

#[derive(Debug, Default, Copy, Clone)]
struct Aligns {
    edge: usize,
    other: usize,
    tile: u64,
    is_flipped: bool,
}

fn eval_borders(tile: &mut Tile) {
    tile.borders[0] = tile.pixels[0].iter().collect::<String>();
    tile.borders[1] = tile
        .pixels
        .iter()
        .map(|row| row[tile.pixels.len() - 1])
        .collect::<String>();
    tile.borders[2] = tile.pixels[tile.pixels.len() - 1]
        .iter()
        .rev()
        .collect::<String>();
    tile.borders[3] = tile
        .pixels
        .iter()
        .rev()
        .map(|row| row[0])
        .collect::<String>();
}

impl Tile {
    pub fn matches(&self, other: &Tile, match_map: &mut HashMap<u64, Vec<Aligns>>) {
        for i in 0..=3 {
            other
                .borders
                .iter()
                .enumerate()
                .for_each(|(index, border)| {
                    if border.eq(&self.borders[i]) {
                        let adj = Aligns {
                            edge: i,
                            other: index,
                            tile: other.number,
                            is_flipped: false,
                        };
                        match_map
                            .entry(self.number)
                            .and_modify(|matched: &mut Vec<Aligns>| {
                                matched.push(adj);
                            })
                            .or_insert(vec![adj]);
                    }
                    let reversed = border.chars().rev().collect::<String>();
                    if reversed.eq(&self.borders[i]) {
                        let adj = Aligns {
                            edge: i,
                            other: index,
                            tile: other.number,
                            is_flipped: true,
                        };
                        match_map
                            .entry(self.number)
                            .and_modify(|matched: &mut Vec<Aligns>| {
                                matched.push(adj);
                            })
                            .or_insert(vec![adj]);
                    }
                });
        }
    }

    pub fn rotate_by_cw(&mut self, degrees: u32) -> &mut Self {
        let mut roatated = Tile::default();
        roatated.number = self.number;
        let side = self.pixels.len() - 1;
        roatated.pixels = self.pixels.clone();
        match degrees {
            90 => {
                for (i, j) in iproduct!(0..=side, 0..=side) {
                    roatated.pixels[i][j] = self.pixels[side - j][i];
                }
                roatated.orientation = self.orientation + 1;
            }
            180 => {
                for (i, j) in iproduct!(0..=side, 0..=side) {
                    roatated.pixels[i][j] = self.pixels[side - i][side - j];
                }
                roatated.orientation = self.orientation + 2;
            }
            270 => {
                for (i, j) in iproduct!(0..=side, 0..=side) {
                    roatated.pixels[i][j] = self.pixels[j][side - i];
                }
                roatated.orientation = self.orientation + 3;
            }
            0 => {
                roatated.orientation = self.orientation;
            }
            _ => unimplemented!(),
        }
        eval_borders(&mut roatated);
        *self = roatated;
        self
    }

    pub fn rotate_by_ccw(&mut self, degrees: u32) -> &mut Self {
        let mut roatated = Tile::default();
        roatated.number = self.number;
        let side = self.pixels.len() - 1;
        roatated.pixels = self.pixels.clone();
        match degrees {
            270 => {
                for (i, j) in iproduct!(0..=side, 0..=side) {
                    roatated.pixels[i][j] = self.pixels[side - j][i];
                }
                roatated.orientation = self.orientation + 3;
            }
            180 => {
                for (i, j) in iproduct!(0..=side, 0..=side) {
                    roatated.pixels[i][j] = self.pixels[side - i][side - j];
                }
                roatated.orientation = self.orientation + 2;
            }
            90 => {
                for (i, j) in iproduct!(0..=side, 0..=side) {
                    roatated.pixels[i][j] = self.pixels[j][side - i];
                }
                roatated.orientation = self.orientation + 1;
            }
            0 => {
                roatated.orientation = self.orientation;
            }
            _ => unimplemented!(),
        }
        eval_borders(&mut roatated);
        *self = roatated;
        self
    }

    pub fn flip(&mut self) -> &mut Self {
        let mut flipped = Tile::default();
        flipped.number = self.number;
        let side = self.pixels.len() - 1;
        flipped.pixels = self.pixels.clone();
        for (i, j) in iproduct!(0..=side, 0..=side) {
            flipped.pixels[i][j] = self.pixels[side - i][j];
        }
        // only rotations need to be switched to ccw.
        flipped.orientation = self.orientation + 4;
        eval_borders(&mut flipped);
        *self = flipped;
        self
    }
}

fn parse_tile(tile: &str) -> Tile {
    let (info, image) = tile.split(":\n").collect_tuple().unwrap();
    let mut tile = Tile::default();
    scan!(info.bytes()=>"Tile {}",tile.number);
    tile.pixels = image
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    eval_borders(&mut tile);
    tile
}

fn solve_1(tiles: &Vec<Tile>) -> HashMap<u64, Vec<Aligns>> {
    let mut match_map = HashMap::new();
    tiles.iter().for_each(|tile| {
        tiles
            .iter()
            .filter(|other| other.number != tile.number)
            .for_each(|ot| tile.matches(ot, &mut match_map));
    });

    let res = match_map
        .iter()
        .filter(|entry| entry.1.len() == 2)
        .map(|tile| tile.0)
        .product::<u64>();
    println!("{:?}", res);
    match_map
}

fn pixel_print(pixels: &Vec<Vec<char>>) {
    pixels
        .iter()
        .for_each(|line| println!("{:?}", line.iter().collect::<String>()));
}

fn insert_into_image(image: &mut Vec<Vec<char>>, coord: (usize, usize), tile: &Tile) {
    let pixels = &tile.pixels;
    for i in 0..10 {
        for j in 0..10 {
            image[coord.0 * 10 + j][coord.1 * 10 + i] = pixels[i][j];
        }
    }
}

fn get_orientation(i: u8, tile: &mut Tile) -> &mut Tile {
    let angle = ((i as u32) % 4) * 90;
    if i > 3 {
        return tile.flip().rotate_by_ccw(angle);
    }
    return tile.rotate_by_cw(angle);
}

fn get_tile(number: u64, tiles: &Vec<Tile>) -> &Tile {
    tiles.iter().find(|tile| tile.number == number).unwrap()
}

#[derive(PartialEq, Eq)]
enum RelativePosition {
    Above(isize, isize),
    Below(isize, isize),
    Left(isize, isize),
    Right(isize, isize),
}

// This should ideally be a look-up table.
fn get_aligning_orentation(
    fixed: &Tile,
    tiles: &Vec<Tile>,
    adjacent: u64,
    pos: &RelativePosition,
) -> Tile {
    let reversed = |b: &str| b.chars().rev().collect::<String>();
    for i in 0..=7 {
        let mut tile = get_tile(adjacent, tiles).clone();
        let option = get_orientation(i, &mut tile);

        if (discriminant(&pos) == discriminant(&&RelativePosition::Right(0, 0))
            && fixed.borders[1].eq(&reversed(&option.borders[3])))
            || (discriminant(&pos) == discriminant(&&RelativePosition::Below(0, 0))
                && fixed.borders[2].eq(&reversed(&option.borders[0])))
            || (discriminant(&pos) == discriminant(&&RelativePosition::Left(0, 0))
                && fixed.borders[3].eq(&reversed(&option.borders[1])))
            || (discriminant(&pos) == discriminant(&&RelativePosition::Above(0, 0))
                && fixed.borders[0].eq(&reversed(&option.borders[2])))
        {
            return option.clone();
        }
    }
    unreachable!()
}

fn process_tile_adjacents<'a>(
    processing_queue: &mut VecDeque<Tile>,
    tiles: &Vec<Tile>,
    adjacency: &'a HashMap<u64, Vec<Aligns>>,
    mut image: &mut Vec<Vec<char>>,
    inserted: &mut std::collections::HashMap<u64, ((usize, usize), Tile)>,
) {
    let oriented = processing_queue.pop_front().unwrap();
    for i in 0..adjacency[&oriented.number].len() {
        let adjacent = adjacency[&oriented.number][i];
        // could have done with filter, but fighting the borrow checker is left for another day
        if inserted.get(&adjacent.tile).is_some() {
            continue;
        }
        let inserted_orientation = inserted[&oriented.number].1.orientation;
        let rel_pos = match (inserted_orientation, adjacent.edge) {
            (0, 1) | (1, 0) | (2, 3) | (3, 2) | (4, 1) | (5, 0) | (6, 3) | (7, 2) => {
                RelativePosition::Right(1, 0)
            }
            (0, 2) | (1, 1) | (2, 0) | (3, 3) | (4, 0) | (5, 3) | (6, 2) | (7, 1) => {
                RelativePosition::Below(0, 1)
            }
            (0, 3) | (1, 2) | (2, 1) | (3, 0) | (4, 3) | (5, 2) | (6, 1) | (7, 0) => {
                RelativePosition::Left(-1, 0)
            }
            (0, 0) | (1, 3) | (2, 2) | (3, 1) | (4, 2) | (5, 1) | (6, 0) | (7, 3) => {
                RelativePosition::Above(0, -1)
            }
            _ => unimplemented!(),
        };
        let aligning_orentation =
            get_aligning_orentation(&oriented, &tiles, adjacent.tile, &rel_pos).clone();
        // println!(
        //     "orientation {:?} adjacent {:?} aligns to {:?}",
        //     inserted[&oriented.number].1.orientation, adjacent, aligning_orentation.orientation
        // );
        let loc = inserted[&oriented.number].0;
        // println!("relative to {:?}", loc);
        let destination_loc;
        match rel_pos {
            RelativePosition::Above(i, j)
            | RelativePosition::Below(i, j)
            | RelativePosition::Left(i, j)
            | RelativePosition::Right(i, j) => {
                destination_loc = ((loc.0 as isize + i) as usize, (loc.1 as isize + j) as usize)
            }
        }
        // println!("inserting at {:?}", destination_loc);
        insert_into_image(&mut image, destination_loc, &aligning_orentation);
        let _ = inserted.insert(
            adjacent.tile,
            (destination_loc, aligning_orentation.clone()),
        );
        processing_queue.push_back(aligning_orentation);
    }
}

fn search_for_monster(extracted: &mut Vec<Vec<char>>, monster: &Vec<Vec<char>>) -> bool {
    let mut res = false;
    let m_offsets = monster
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c.eq(&'#'))
                .map(move |(x, _)| (i, x))
        })
        .collect::<Vec<_>>();

    for i in 0..(extracted.len() - monster.len()) {
        for j in 0..(extracted[0].len() - monster[0].len()) {
            if m_offsets
                .iter()
                .all(|(a, b)| extracted[i + a][j + b].eq(&'#'))
            {
                m_offsets
                    .iter()
                    .for_each(|(a, b)| extracted[i + a][j + b] = 'O');
                res = true;
            }
        }
    }
    return res;
}

fn solve_2(tiles: &Vec<Tile>, adjacency: &HashMap<u64, Vec<Aligns>>) {
    // final image
    let side = ((tiles.len() as f64).sqrt() as usize) * 10;
    let mut image = vec![vec!['0'; side]; side];

    // fix the top-left
    let corners = adjacency
        .iter()
        .filter(|entry| entry.1.len() == 2)
        .collect::<Vec<(&u64, &Vec<Aligns>)>>();
    // println!("{:?}", corners);
    // println!("{:?}", adjacency);

    // re-orient so that top left can be aligned with others
    let mut oriented = get_tile(*corners[0].0, &tiles).clone();
    let adjacents = (corners[0].1[0].edge, corners[0].1[1].edge);
    match adjacents {
        (0, 1) | (1, 0) => oriented.rotate_by_cw(90),
        (0, 3) | (3, 1) => oriented.rotate_by_cw(180),
        (2, 3) | (3, 2) => oriented.rotate_by_cw(270),
        _ => oriented.rotate_by_cw(0),
    };
    // pixel_print(&oriented.pixels);

    let mut inserted: HashMap<u64, ((usize, usize), Tile)> = HashMap::new();

    let coord = (0, 0);
    // println!("inserting {:?} at {:?}", oriented.number, coord);
    insert_into_image(&mut image, coord, &oriented);
    let _ignore = inserted.insert(oriented.number, (coord, oriented.clone()));
    let mut processing_queue = VecDeque::new();
    processing_queue.push_back(oriented.clone());
    while processing_queue.len() > 0 {
        process_tile_adjacents(
            &mut processing_queue,
            &tiles,
            adjacency,
            &mut image,
            &mut inserted,
        );
    }
    // pixel_print(&image);

    let extracted = image
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 10 != 9 && i % 10 != 0)
        .map(|(_, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .filter(|(j, _)| j % 10 != 9 && j % 10 != 0)
                .map(|(_, c)| c)
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    // pixel_print(&extracted);

    let m_in = fs::read_to_string("./monster.txt").unwrap();
    let monster = m_in
        .split('\n')
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for i in 0..=7 {
        let mut reoriented = Tile::default();
        reoriented.pixels = extracted.clone();
        let bla = get_orientation(i, &mut reoriented);
        if search_for_monster(&mut bla.pixels, &monster) {
            println!(
                "roughness {:?}",
                bla.pixels
                    .iter()
                    .flat_map(|row| row.iter())
                    .filter(|&c| c.eq(&'#'))
                    .count()
            );
            break;
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input_d20.txt").unwrap();
    let tiles = input.split("\n\n").collect::<Vec<&str>>();
    let parsed = tiles
        .iter()
        .map(|tile| parse_tile(tile))
        .collect::<Vec<Tile>>();
    let adjacency = solve_1(&parsed);
    solve_2(&parsed, &adjacency);
}
