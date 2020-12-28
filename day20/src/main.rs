use itertools::iproduct;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs;
use text_io::scan;

#[derive(Debug, Default, Hash, Clone)]
struct Tile {
    number: u64,
    pixels: Vec<Vec<char>>,
    borders: [String; 4],
    orientation: u8,
}

// impl PartialEq for Tile {
//     fn eq(&self, other: &Self) -> bool {
//         self.number == other.number
//     }
// }

// impl Eq for Tile {}

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
                roatated.orientation = 0;
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
                roatated.orientation = self.orientation + 1;
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
            image[coord.0 * 10 + i][coord.1 * 10 + j] = pixels[i][j];
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

// This should ideally be a look-up table.
fn get_aligning_orentation(fixed: &Tile, tiles: &Vec<Tile>, adjacent: u64, right: bool) -> Tile {
    let reversed = |b: &str| b.chars().rev().collect::<String>();
    for i in 0..=7 {
        let mut tile = get_tile(adjacent, tiles).clone();
        let option = get_orientation(i, &mut tile);
        if right && fixed.borders[1].eq(&reversed(&option.borders[3])) {
            return option.clone();
        } else if !right && fixed.borders[2].eq(&reversed(&option.borders[0])) {
            return option.clone();
        }
    }
    unreachable!()
}

fn process_tile_adjacents<'a>(
    oriented: &Tile,
    tiles: &Vec<Tile>,
    adjacency: &'a HashMap<u64, Vec<Aligns>>,
    mut image: &mut Vec<Vec<char>>,
    inserted: &mut std::collections::HashMap<&'a u64, ((usize, usize), Tile)>,
) {
    adjacency[&oriented.number].iter().for_each(|adjacent| {
        // could have done with filter, but fighting the borrow checker is left for another day
        if inserted.get(&adjacent.tile).is_some() {
            return;
        }
        let inserted_orientation = inserted[&oriented.number].1.orientation;
        let is_to_right = match (inserted_orientation, adjacent.edge) {
            (0, 1) | (1, 0) | (2, 3) | (3, 2) | (4, 1) | (5, 0) | (6, 3) | (7, 2) => true,
            (0, 2) | (1, 1) | (2, 0) | (3, 3) | (4, 0) | (5, 3) | (6, 2) | (7, 1) => false,
            _ => unimplemented!(),
        };
        let aligning_orentation =
            get_aligning_orentation(&oriented, &tiles, adjacent.tile, is_to_right).clone();
        println!(
            "adjacent {:?} aligns to {:?}",
            adjacent, aligning_orentation.orientation
        );
        let mut loc = inserted[&oriented.number].0;
        if is_to_right {
            loc.1 += 1;
            insert_into_image(&mut image, loc, &aligning_orentation);
        } else {
            loc.0 += 1;
            insert_into_image(&mut image, loc, &aligning_orentation);
        }
        let _ = inserted.insert(&adjacent.tile, (loc, aligning_orentation.clone()));
    });
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

    let mut inserted: HashMap<&u64, ((usize, usize), Tile)> = HashMap::new();

    let coord = (0, 0);
    insert_into_image(&mut image, coord, &oriented);
    let _ignore = inserted.insert(&oriented.number, (coord, oriented.clone()));
    let mut processing_queue = VecDeque::new();
    processing_queue.push_back(&oriented);
    while processing_queue.len() > 0 {
        process_tile_adjacents(
            &processing_queue.pop_front().unwrap(),
            &tiles,
            adjacency,
            &mut image,
            &mut inserted,
        );
    }
    pixel_print(&image);
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
