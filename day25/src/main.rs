#![allow(dead_code)]
use std::iter::successors;

fn main() {
    let (card, door) = (8184785, 5293040);
    let (subject, def) = (7, 20201227);
    let compute = |sub: u64| successors(Some(sub), move |&n| Some((n * sub) % def));
    let mut seq = compute(subject).enumerate();
    let res = seq.find(|(_, x)| x == &card || x == &door).unwrap();
    if res.1 == card {
        println!("card {:?}", compute(door).take(res.0 + 1).last().unwrap());
    } else {
        println!("door {:?}", compute(card).take(res.0 + 1).last().unwrap());
    }
}
