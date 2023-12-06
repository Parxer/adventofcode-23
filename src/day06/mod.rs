use std::env;
use std::fs::File;
use std::io::Read;
use std::iter::Map;
use std::str::SplitAsciiWhitespace;
use rayon::prelude::*;
use crate::common::{Part};

fn get_parsed_numbers_iter (line: &str) -> Map<SplitAsciiWhitespace, fn(&str) -> u32> {
    line.split_once(':').unwrap().1.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap())
}

pub fn run_day_06(input: String, part: Part) -> u32 {
    let mut result = 1;

    let mut line_iter = input.lines().into_iter();
    let times = get_parsed_numbers_iter(line_iter.next().unwrap());
    let distances = get_parsed_numbers_iter(line_iter.next().unwrap());
    let mut races_iter = times.zip(distances);

    loop {
        match races_iter.next() {
            None => { break; }
            Some((time, distance)) => {
                let wins_iter = (0u32..=time).into_par_iter().filter(|&speed| {
                    let time_moving = time - speed;
                    time_moving * speed > distance
                });

                result *= u32::try_from(wins_iter.count()).unwrap();
            }
        }
    }

    result
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/day06/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_day_06(sample_input, Part::First);
    assert_eq!(result, 288);
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/day06/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_day_06(sample_input, Part::Second);
    assert_eq!(result, 0);
}
