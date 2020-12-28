use std::collections::VecDeque;
use std::fs;

type Deck = VecDeque<usize>;
type Decks = (Deck, Deck);
fn parse<'a>(mut players: impl Iterator<Item = &'a str>) -> Decks {
    let get_hand = |list: &str| {
        list.split('\n')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Deck>()
    };
    let player1 = get_hand(players.next().unwrap().split(":\n").skip(1).next().unwrap());
    let player2 = get_hand(players.next().unwrap().split(":\n").skip(1).next().unwrap());
    (player1, player2)
}

fn get_total(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .zip((1..=deck.len()).rev())
        .map(|(i, j)| i * j)
        .sum::<usize>()
}

fn play_game(mut decks: Decks, trick_the_crab: bool) -> Decks {
    let mut record = Vec::new();
    let collect_cards = |winner: &mut Deck, a: usize, b: usize| {
        winner.push_back(a);
        winner.push_back(b);
    };
    while decks.0.len() > 0 && decks.1.len() > 0 {
        if trick_the_crab {
            if record.iter().any(|state: &Decks| state == &decks) {
                decks.1.clear();
                return decks;
            }
            record.push(decks.clone());
        }
        let (a, b) = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
        let (p, q) = (decks.0.len(), decks.1.len());
        if trick_the_crab && p >= a && q >= b {
            let sub1 = decks.0.iter().cloned().take(a).collect::<Deck>();
            let sub2 = decks.1.iter().cloned().take(b).collect::<Deck>();
            let end = play_game((sub1, sub2), trick_the_crab);
            if end.1.is_empty() {
                collect_cards(&mut decks.0, a, b);
            } else {
                collect_cards(&mut decks.1, b, a);
            }
        } else {
            if a > b {
                collect_cards(&mut decks.0, a, b);
            } else {
                collect_cards(&mut decks.1, b, a);
            }
        }
    }
    decks
}

fn solve(decks: &Decks, trick_the_crab: bool) {
    let end = play_game(decks.clone(), trick_the_crab);
    let res = if end.0.is_empty() {
        get_total(&end.1)
    } else {
        get_total(&end.0)
    };
    println!("{:?}", res);
}

fn main() {
    let input = fs::read_to_string("./input_day22.txt").unwrap();
    let decks = parse(input.split("\n\n"));
    solve(&decks, false);
    solve(&decks, true);
}
