use std::env;
use std::fs::File;
use std::io::Read;
use rayon::prelude::*;
use crate::common::{Part};


pub fn run(input: &String, part: Part) -> String{
    let expansion_rate = match part {
        Part::First => { 2 }
        Part::Second => { 1000000 }
    };

    run_with_expansion_rate(input, expansion_rate)
}
fn run_with_expansion_rate(input: &String, expansion_rate: usize) -> String {

    let mut result: u64 = 0;

    let mut grid: Vec<Vec<char>> = vec![];
    let mut galaxies: Vec<(usize, usize)> = vec![];

    let mut expanding_rows: Vec<usize> = vec![];
    for (i, line) in input.lines().enumerate() {
        grid.push(line.chars().collect());
        if line.chars().all(|c| c == '.') {
            expanding_rows.push(i);
        }
    }


    let mut expanding_columns: Vec<usize> = vec![];
    for c in 0..grid[0].len() {
        let mut is_empty = true;
        for r in 0..grid.len() {
            if grid[r][c] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty { expanding_columns.push(c); }
    }

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '#' { galaxies.push((r, c)); }
        }
    }

    result = galaxies.par_iter().enumerate().fold(|| 0usize, |mut sum, (i_s, &start)| {
        sum += galaxies.par_iter().skip(i_s + 1).fold(|| 0usize, |mut sum, &finish| {
            let min: usize = start.0.abs_diff(finish.0) + start.1.abs_diff(finish.1);

            let num_expanding_rows_crossed = expanding_rows.iter().filter(|&&i_r| {
                i_r > start.0.min(finish.0) && i_r < start.0.max(finish.0)
            }).count();
            let num_expanding_cols_crossed = expanding_columns.iter().filter(|&&i_c| {
                i_c > start.1.min(finish.1) && i_c < start.1.max(finish.1)
            }).count();

            sum += min + (expansion_rate - 1) * (num_expanding_rows_crossed + num_expanding_cols_crossed);
            sum
        }).sum::<usize>();
        sum
    }).sum::<usize>().try_into().unwrap();

    format!("{result}")
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day11/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_with_expansion_rate(&sample_input, 2);
    assert_eq!(result, "374");
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day11/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run_with_expansion_rate(&sample_input, 10);
    assert_eq!(result, "1030");
}
