use regex::Regex;
use std::fs;

fn check_validity_1(input: &Vec<String>) -> usize {
    let required_fields = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    input
        .iter()
        .filter(|entry| {
            required_fields
                .iter()
                .all(|required| entry.contains(required))
        })
        .count()
}

fn check_validity_2(input: &Vec<String>) -> usize {
    let required_fields = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    let valid_year_between = |year: &str, s: u16, e: u16| -> bool {
        let some_val = year[4..].parse::<u16>();
        year.len() == 8 && some_val.is_ok() && (s..=e).contains(&some_val.unwrap())
    };
    let proper_colour = Regex::new(r"^#[0-9a-f]").unwrap();
    let eye_colours: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let correct_height = |height: &str| -> bool {
        if height.len() == 9 || height.len() == 8 {
            match &height[(height.len() - 2)..] {
                "cm" => {
                    height[4..7].chars().all(char::is_numeric)
                        && (150..=193).contains(&height[4..7].parse::<u16>().unwrap())
                }
                "in" => {
                    height[4..6].chars().all(char::is_numeric)
                        && (59..=76).contains(&height[4..6].parse::<u16>().unwrap())
                }
                _ => false,
            }
        } else {
            false
        }
    };

    input
        .iter()
        .filter(|entry| {
            let split_up = entry.split(' ').collect::<Vec<&str>>();
            split_up.len() >= required_fields.len()
                && required_fields
                    .iter()
                    .all(|required| entry.contains(required))
                && split_up.iter().all(|field| match &field[0..3] {
                    "byr" => valid_year_between(&field, 1920, 2002),
                    "iyr" => valid_year_between(&field, 2010, 2020),
                    "eyr" => valid_year_between(&field, 2020, 2030),
                    "hgt" => correct_height(&field),
                    "hcl" => field.len() == 11 && proper_colour.is_match(&field[4..]),
                    "ecl" => field.len() == 7 && eye_colours.iter().any(|&val| val.eq(&field[4..])),
                    "pid" => field.len() == 13 && field[4..].chars().all(char::is_numeric),
                    "cid" => true,
                    &_ => false,
                })
        })
        .count()
}

fn main() {
    let input = fs::read_to_string("./input_d4.txt").unwrap();

    let passports: Vec<_> = input
        .split("\n\n")
        .map(|entry| entry.replace("\n", " "))
        .collect();

    let valid_count_1 = check_validity_1(&passports);
    println!("valid 1: {:?}", valid_count_1);

    let valid_count_2 = check_validity_2(&passports);
    println!("valid 2: {:?}", valid_count_2);
}
