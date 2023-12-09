use std::env;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;
use rayon::iter::*;
// use rayon::prelude::*;
use crate::common::Part;
use crate::debug;

fn get_next_step(steps: Vec<i32>, part: Part) -> i32 {
    let new_steps: Vec<i32> = (0..steps.len() - 1).map(|i| {
        steps[i+1] - steps[i]
    }).collect();
    return if !new_steps.iter().any(|&s| s != 0) { steps[0] } else { match part {
        Part::First => { steps.last().unwrap() + get_next_step(new_steps, part) }
        Part::Second => { steps[0] - get_next_step(new_steps, part) }
    } }
}

pub fn run(input: &String, part: Part) -> String {

    let mut histories: Vec<Vec<i32>> = vec![];

    for line in input.lines() {
        histories.push(line.split_ascii_whitespace().map(|num| num.parse::<i32>().unwrap()).collect());
    }

    let results: Vec<i32> = histories.par_iter().fold(|| 0i32, |mut sum, hist| {
        sum += get_next_step(hist.clone(), part);
        sum
    }).collect();

    debug!(results);
    format!("{}", results.iter().sum::<i32>())
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day09/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::First);
    assert_eq!(result, "114");
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day09/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::Second);
    assert_eq!(result, "2");
}
