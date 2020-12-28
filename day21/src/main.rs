use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<(HashSet<String>, HashSet<String>)> {
    let parse_food = |foods: &'a str| foods.split(' ').map(|ig| ig.to_string());
    let remove_end = |stream: &'a str| stream.split(")").next().unwrap();
    let remove_whitespaces = |stream: &'a str| {
        stream
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    };
    let parse_allergens = |allergens: &'a str| {
        remove_end(allergens)
            .split(',')
            .map(|entry| remove_whitespaces(entry))
    };
    lines
        .filter_map(|line| {
            line.split(" (contains").collect_tuple().map(|(a, b)| {
                (
                    parse_food(a).collect::<HashSet<String>>(),
                    parse_allergens(b).collect::<HashSet<String>>(),
                )
            })
        })
        .collect::<Vec<_>>()
}

fn solve(mut data: Vec<(HashSet<String>, HashSet<String>)>) {
    let mut dangerous = Vec::new();
    let mut updated = true;
    while updated {
        let (foods, allergens) = data
            .iter()
            .fold((HashSet::new(), HashSet::new()), |acc, x| {
                (
                    acc.0.union(&x.0).cloned().collect::<HashSet<String>>(),
                    acc.1.union(&x.1).cloned().collect::<HashSet<String>>(),
                )
            });
        let matched = allergens
            .iter()
            .map(|allergen| {
                (
                    data.iter()
                        .filter(|recipe| recipe.1.contains(allergen))
                        .map(|(food, _)| food.clone())
                        .fold(foods.clone(), |acc, x| {
                            acc.intersection(&x).cloned().collect::<HashSet<String>>()
                        }),
                    allergen,
                )
            })
            .filter(|(ig_set, _)| ig_set.len() == 1)
            .map(|(ig_set, alg)| (ig_set.iter().next().unwrap().to_string(), alg.to_string()))
            .collect::<Vec<(String, String)>>();
        updated = matched.len() > 0;
        matched
            .iter()
            .for_each(|(food, allergen)| dangerous.push((allergen.to_string(), food.to_string())));
        data.iter_mut().for_each(|entry| {
            matched.iter().for_each(|matched| {
                let _ignored = (entry.0.remove(&matched.0), entry.1.remove(&matched.1));
            });
        });
    }
    let res = data.iter().map(|entry| entry.0.len()).sum::<usize>();
    println!("{:?}", res);
    dangerous.sort();
    println!("{:?}", dangerous);
}

fn main() {
    let input = fs::read_to_string("./input_d21.txt").unwrap();
    let lines = input.split('\n');
    let data = parse(lines);
    solve(data);
}
