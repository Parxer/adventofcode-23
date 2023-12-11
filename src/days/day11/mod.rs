use std::env;
use std::fs::File;
use std::io::Read;
use rayon::prelude::*;
use crate::common::{find_shortest_path, Part};


pub fn run(input: &String, part: Part) -> String {
    if part == Part::Second { return format!("N/A") } // TODO: delete later
    let mut result = 0;

    let mut grid: Vec<Vec<char>> = vec![];
    let mut galaxies: Vec<(usize, usize)> = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
        if line.chars().all(|c| c == '.') { grid.push(line.chars().collect()); }
    }


    let mut columns_to_expand: Vec<usize> = vec![];
    for c in 0..grid[0].len() {
        let mut is_empty = true;
        for r in 0..grid.len() {
            if grid[r][c] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty { columns_to_expand.push(c); }
    }
    columns_to_expand.reverse();
    for c in columns_to_expand {
        for r in 0..grid.len() {
            grid[r].insert(c, '.');
        }
    }

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '#' { galaxies.push((r, c)); }
        }
    }

    result = galaxies.par_iter().enumerate().fold(|| 0u32, |mut sum, (i_s, &start)| {
        sum += galaxies.par_iter().skip(i_s + 1).fold(|| 0u32, |mut sum, &finish| {
            let min = find_shortest_path(start, finish, &grid);
            if env::var("AOC_DEBUG").is_ok() { println!("Path from {},{} to {},{}: {}", start.0, start.1, finish.0, finish.1, min); }
            sum += min;
            sum
        }).sum::<u32>();
        sum
    }).sum();

    format!("{result}")
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day11/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::First);
    assert_eq!(result, "374");
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day11/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::Second);
    assert_eq!(result, "");
}
