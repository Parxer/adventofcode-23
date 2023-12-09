use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use num_integer::lcm;
use rayon::prelude::*;
use crate::common::Part;
use crate::debug;

pub fn run(input: &String, part: Part) -> String {
    let mut lines_iter = input.lines();
    let instructions = lines_iter.next().unwrap();
    lines_iter.next();

    let mut nodes: Vec<&str> = vec![];

    let mut parsed_input  =
        lines_iter.fold(HashMap::<&str, [&str; 2]>::new(),|mut acc, line| {
            let src = &line[0..3];
            let dest = [&line[7..10], &line[12..15]];
            nodes.push(src);
            acc.insert(src, dest);
            acc
        });

    let mappings: Vec<[usize; 2]> = nodes.iter().map(|&node| {
        let arr = parsed_input.remove(node).unwrap();
        [nodes.iter().position(|&x| x == arr[0]).unwrap(), nodes.iter().position(|&x| x == arr[1]).unwrap()]
    }).collect();

    debug!(mappings);
    let starting_indices: Vec<usize> = match part {
        Part::First => { vec![nodes.iter().position(|&node| node == "AAA").unwrap()]}
        Part::Second => { nodes.iter().enumerate().filter_map(|(i, &node)| if node.ends_with("A") { Some(i) } else { None }).collect() }
    };

    let mut results: Vec<u64> = vec![0u64; starting_indices.len()];

    results = starting_indices.par_iter().map(|&index| {
        let mut cur_i = index;
        let mut result = 0;
        for c in instructions.chars().cycle() {
            match part {
                Part::First => { if nodes[cur_i] == "ZZZ" { break } }
                Part::Second => { if nodes[cur_i].ends_with("Z") { break } }
            }
            result += 1;

            let i = if c == 'L' { 0 } else { 1 };
            cur_i = mappings[cur_i][i];
        }
        result
    }).collect();

    format!("{}", results.iter().fold(1, |acc, &next| lcm(acc, next)))
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day08/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::First);
    assert_eq!(result, "2");
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day08/test_input_2").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::Second);
    assert_eq!(result, "6");
}
