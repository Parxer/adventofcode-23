use std::env;
use std::fs::File;
use std::io::Read;
use rayon::prelude::*;
use crate::common::{Part};

fn get_parsed_numbers (line: &str, part: &Part) -> Vec<u64> {
    let str = line.split_once(':').unwrap().1;
    match part {
        Part::First => { str.split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap()).collect() }
        Part::Second => {
            let num_str: String = str.chars().filter(|c| !c.is_whitespace()).collect();
            vec![num_str.parse::<u64>().unwrap()]
        }
    }
}

pub fn run(input: &String, part: Part) -> String {
    let mut result = 1;

    let mut line_iter = input.lines().into_iter();
    let times = get_parsed_numbers(line_iter.next().unwrap(), &part);
    let distances = get_parsed_numbers(line_iter.next().unwrap(), &part);
    let mut races_iter = times.iter().zip(distances.iter());

    loop {
        match races_iter.next() {
            None => { break; }
            Some((&time, &distance)) => {
                let wins_iter = (0..=time).into_par_iter().filter(|&speed| {
                    let time_moving = time - speed;
                    time_moving * speed > distance
                });

                result *= u32::try_from(wins_iter.count()).unwrap();
            }
        }
    }

    format!("{result}")
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day06/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::First);
    assert_eq!(result, "288");
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day06/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::Second);
    assert_eq!(result, "71503");
}
