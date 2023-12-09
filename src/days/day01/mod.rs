use aho_corasick::{AhoCorasick};
use std::{env, usize};
use crate::common::Part;

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;


enum Position {
    First,
    Last
}

pub fn run(input: &String, part: Part) -> String {
    let mut numeric_only: bool = false;
    if part == Part::First { numeric_only = true; }

    let mut result = 0;
    for line in input.lines() {
        let (i_num_first, num_first) = find_numeric_item(line, Position::First);
        let (i_num_last, num_last) = find_numeric_item(line, Position::Last);

        if numeric_only {
            result += num_first * 10;
            result += num_last;

            continue
        }

        let (i_spelled_first, spelled_first) = find_spelled_item(line.to_string(), Position::First);
        let (i_spelled_last, spelled_last) = find_spelled_item(line.to_string(), Position::Last);

        let mut line_result = 0;
        line_result += if i_num_first < i_spelled_first { num_first } else { spelled_first } * 10;
        line_result += if i_num_last >= i_spelled_last { num_last } else { spelled_last };
        result += line_result;

        if !(env::var("AOC_DEBUG").is_ok()) { continue }
        println!("-------------\n\n");
        println!("Line is: {}", line);
        println!("First num is {} at index {}, last num is {} at index {}", num_first, i_num_first, num_last, i_num_last);
        println!("First spelled is {} at index {}, last spelled is {} at index {}", spelled_first, i_spelled_first, spelled_last, i_spelled_last);
        println!("Result is: {}\n\n\n\n", line_result);
    }

    format!("{result}")
}

fn find_numeric_item(line: &str, position: Position) -> (usize, u32) {
    match position {
        Position::First => {
            match line.find(char::is_numeric) {
                None => (usize::MAX, 0),
                Some(i) => (i, get_numeric_value_from_byte(line.as_bytes()[i]))
            }
        }
        Position::Last => {
            match line.rfind(char::is_numeric) {
                None => (usize::MAX, 0),
                Some(i) => (i, get_numeric_value_from_byte(line.as_bytes()[i]))
            }
        }
    }
}

fn get_numeric_value_from_byte(byte: u8) -> u32 {
    char::from(byte).to_digit(10).unwrap()
}

const SPELLED_NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_numeric_value_from_spelled(spelled: &str) -> u32 {
    SPELLED_NUMBERS.iter().position(|&item| item == spelled).unwrap() as u32 + 1
}

fn find_spelled_item(line: String, position: Position) -> (usize, u32) {
    let ac = AhoCorasick::new(SPELLED_NUMBERS).unwrap();
    let matches: Vec<(usize, &str)> = ac.find_overlapping_iter(line.as_str()).map(|m | (m.start(), SPELLED_NUMBERS[m.pattern()])).collect();
    match position {
        Position::First => {
            match matches.first() {
                None => (usize::MAX, 0),
                Some(&first_match) => (first_match.0, get_numeric_value_from_spelled(first_match.1))
            }
        }
        Position::Last => {
            match matches.last() {
                None => (0, 0),
                Some(&last_match) => (last_match.0, get_numeric_value_from_spelled(last_match.1))
            }
        }
    }
}

#[test]
fn test_sample_input_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input_1 = String::new();
    File::open("src/days/day01/test_input_1").expect("Failed to open sample input").read_to_string(&mut sample_input_1).ok();

    let result = run(&sample_input_1, Part::First);
    assert_eq!(result, "142");
}

#[test]
fn test_sample_input_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input_2 = String::new();
    File::open("src/days/day01/test_input_2").expect("Failed to open sample input").read_to_string(&mut sample_input_2).ok();

    let result = run(&sample_input_2, Part::Second);
    assert_eq!(result, "434");
}
