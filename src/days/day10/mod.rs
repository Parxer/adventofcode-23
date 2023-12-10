use std::env;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;
use crate::common::{get_grid_neigh_indexes, Part, usize_safe_sub};
use crate::debug;

fn get_pos_diff(left: (usize, usize), right: (usize, usize)) -> (i8, i8) {
    (usize_safe_sub(left.0, right.0), usize_safe_sub(left.1, right.1))
}

fn match_shape(c: char) -> [(i8, i8); 2] {
    match c {
        '-' => [(0, -1), (0, 1)],
        '|' => [(-1, 0), (1, 0)],
        'L' => [(-1, 0), (0, 1)],
        'J' => [(-1, 0), (0, -1)],
        'F' => [(1, 0), (0, 1)],
        '7' => [(1, 0), (0, -1)],
        'S' => [(0, 0), (0, 0)],
        _ => { panic!("Wrong pipe shape ({c})!") }
    }
}
fn count_steps(prev_pos: (usize, usize), cur: char, cur_pos: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
    if cur == 'S' { return 1; }
    match match_shape(cur).iter().find(|&con| *con != get_pos_diff(prev_pos, cur_pos) ) {
        None => { 0 }
        Some(&next_diff) => {
            let next_pos = (cur_pos.0.checked_add_signed(next_diff.0.into()).unwrap(), cur_pos.1.checked_add_signed(next_diff.1.into()).unwrap());
            1 + count_steps(cur_pos, grid[next_pos.0][next_pos.1], next_pos, grid)
        }
    }
}

fn calc_path(s: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
    debug!(get_grid_neigh_indexes(s, (grid.len(), grid[0].len())));
    for n in get_grid_neigh_indexes(s, (grid.len(), grid[0].len())) {
        let char = grid[n.0][n.1];
        if char == '.' { continue; }
        if match_shape(char).contains(&get_pos_diff(s, n)) {
            return count_steps(s, char, n, grid);
        }
    }
    0
}

pub fn run(input: &String, part: Part) -> String {
    if part == Part::Second { return format!("N/A") } // TODO: delete later
    let mut path_length: u32 = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start: Option<(usize, usize)> = None;

    for (i_x, line) in input.lines().enumerate() {
        if start == None {
            if let Some(i_y) = line.chars().position(|c| c == 'S') { start = Some((i_x, i_y)); }
        }
        grid.push(line.chars().collect());
    }

    path_length = calc_path(start.unwrap(), &grid);

    if path_length == 0 { return format!("No route found!") }
    if path_length % 2 == 0 { path_length -= 1; }

    format!("{}", (path_length +1 ) / 2)
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day10/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::First);
    assert_eq!(result, "8");
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day10/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::Second);
    assert_eq!(result, "");
}
