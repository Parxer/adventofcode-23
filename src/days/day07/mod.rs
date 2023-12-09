use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;
use crate::common::Part;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

fn card_to_value (card: char, part: Part) -> u32 {
    match card {
        'T' => { 10 }
        'J' => { match part {
            Part::First => 11,
            Part::Second => 1
        }}
        'Q' => { 12 }
        'K' => { 13 }
        'A' => { 14 }
        rest => {
            rest.to_digit(10).or(Some(0)).unwrap()
        }
    }
}

fn cmp_hands (left: &str, right: &str, part: Part) -> Ordering {
    let cmp = match part {
        Part::First => { get_hand_type(left).cmp(&get_hand_type(right))}
        Part::Second => {
            let best_left = if left.contains('J') { get_best_hand_type(left) } else { get_hand_type(left) };
            let best_right = if right.contains('J') { get_best_hand_type(right) } else { get_hand_type(right) };
            best_left.cmp(&best_right)
        }
    };

    match cmp {
        Ordering::Equal => {
            match left.chars().zip(right.chars()).find(|(l, r)| card_to_value(*l, part) != card_to_value(*r, part)) {
                None => { Ordering::Equal },
                Some((l, r)) => { card_to_value(l, part).cmp(&card_to_value(r, part)) }
            }
        },
        ordering => ordering
    }
}

fn get_best_hand_type(hand: &str) -> HandType {
    if hand.chars().find(|c| *c == 'J') == None { return get_hand_type(hand) }

    let mut other_cards = hand.chars().filter(|c| *c != 'J').collect::<HashSet<_>>();
    if other_cards.is_empty() { other_cards.extend(['A']) }

    let mut combinations: Vec<HandType> = vec![];
    for &card in &other_cards {
        combinations.push(get_hand_type(str::replace(hand, 'J', card.to_string().as_str()).as_str()));
    }

    let max = *combinations.iter().max().unwrap();
    max
}

fn get_hand_type (hand: &str) -> HandType {
    let mut hand: Vec<usize> = hand.chars().fold(HashMap::<char, usize>::new(), |mut h, x | {
        *h.entry(x).or_default() += 1;
        h
    }).into_iter().map(|(_, val)| val).collect();
    hand.sort();
    let max_two_iter = hand.iter().rev().take(2);
    let max_two = max_two_iter.collect::<Vec<_>>();
    match max_two[0] {
        &5 => { HandType::FiveOfAKind },
        &4 => { HandType::FourOfAKind },
        &3 => { if *max_two[1] == 2 { HandType::FullHouse } else { HandType::ThreeOfAKind } },
        &2 => { if *max_two[1] == 2 { HandType::TwoPair } else { HandType::OnePair } },
        _ => HandType::HighCard
    }
}

pub fn run(input: &String, part: Part) -> String {
    let mut result = 0;
    let mut hands: Vec<(&str, u32)> = vec![];

    for line in input.lines() {
        let line_data: Vec<&str> = line.split_ascii_whitespace().collect();
        hands.push((line_data[0], line_data[1].parse::<u32>().unwrap()));
    }

    hands.sort_unstable_by(|&a, &b| cmp_hands(a.0, b.0, part));

    hands.iter().enumerate().for_each(|(i, &(_,score))| {
        result += (u32::try_from(i+1).unwrap()) * score;
    });

    format!("{result}")
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day07/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::First);
    assert_eq!(result, "6440");
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day07/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::Second);
    assert_eq!(result, "5905");
}
