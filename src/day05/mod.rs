use std::env;
use crate::common::{Part};
use crate::debug;
use rayon::prelude::*;

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;

const STEPS: &'static [&'static str] = &["soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];

pub fn run_day_05(input: String, part: Part) -> u32 {
    let mut result = 0;

    let mut input_iter = input.lines();
    let seeds_str = input_iter.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<u32> = seeds_str.1.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
    let mut values= vec![];

    match part {
        Part::First => { values = seeds.clone(); }
        Part::Second => {
            let mut seeds_iter = seeds.iter();
            loop {
                match seeds_iter.next() {
                    None => { break; }
                    Some(&start) => {
                        let &range = seeds_iter.next().unwrap();
                        for value in start..=start + (range-1) {
                            values.push(value);
                        }
                    }
                }
            }
        }
    }

    debug!(values);

    input_iter.next();
    for &step in STEPS {
        debug!(step);
        let mut new_values = values.clone();
        loop {
            match input_iter.next() {
                None => { break; }
                Some("") => { break; }
                Some(line) => {
                    if line.ends_with (':') {
                        if line.contains(step) { continue; }
                        else { panic!("Unexpected step description: {}", line); }
                    } else {
                        let maps: Vec<u32> = line.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
                        let dest = maps[0];
                        let src = maps[1];
                        let range = maps[2];

                        let indices_in_range: Vec<usize> = values.par_iter().enumerate().filter(|(i, &val)| val >= src && val <= src + (range - 1)).map(|(i, &val)| i).collect();
                        for index in indices_in_range {
                            if dest > src { new_values[index] += dest-src; } else { new_values[index] -= src-dest; }
                        };

                    }
                }
            }
        }
        debug!(new_values);
        values = new_values;
    }

    result += values.iter().min().ok_or_else(|| 0).unwrap();

    result
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/day05/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_day_05(sample_input, Part::First);
    assert_eq!(result, 35);
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/day05/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_day_05(sample_input, Part::Second);
    assert_eq!(result, 46);
}
