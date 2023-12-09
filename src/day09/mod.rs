use std::env;
use std::fs::File;
use std::io::Read;
use crate::common::Part;

pub fn run_day_09(input: String, part: Part) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        result += 1;
    }

    result
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/day09/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_day_09(sample_input, Part::First);
    assert_eq!(result, 1);
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/day09/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_day_09(sample_input, Part::Second);
    assert_eq!(result, 1);
}
