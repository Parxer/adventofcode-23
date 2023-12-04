use std::env;
use std::fs::File;
use std::io::Read;
use crate::common::Part;

pub fn run_day_04(input: String, part: Part) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let line_parts: Vec<&str> = line.split([':','|']).collect();
        let winning_nums: Vec<u32> = line_parts[1].split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        let numbers: Vec<u32> = line_parts[2].split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();

        if env::var("AOC_DEBUG").is_ok() {
            println!("{:?}", winning_nums);
            println!("{:?}", numbers);
        }

        let mut matching_count = 0;
        for num in numbers {
            if winning_nums.contains(&num) { matching_count += 1; }
        }

        if env::var("AOC_DEBUG").is_ok() {
            println!("Number of matches: {matching_count}");
        }

        if matching_count > 0 { result += 2u32.pow(matching_count - 1); }
    }

    result
}

#[test]
fn test_sample_input_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/day04/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_day_04(sample_input, Part::First);
    assert_eq!(result, 13);
}
