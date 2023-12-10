use std::env;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;
use crate::common::{get_grid_neigh_indexes, Part, usize_safe_sub};
use colored::Colorize;

fn get_pos_diff(left: &(usize, usize), right: &(usize, usize)) -> (i8, i8) {
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

fn find_steps(prev_pos: (usize, usize), cur: char, cur_pos: (usize, usize), grid: &Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
    if cur == 'S' { return Some(vec![cur_pos]); }
    match match_shape(cur).iter().find(|&con| *con != get_pos_diff(&prev_pos, &cur_pos) ) {
        None => { None }
        Some(&next_diff) => {
            let next_pos = (cur_pos.0.checked_add_signed(next_diff.0.into()).unwrap(), cur_pos.1.checked_add_signed(next_diff.1.into()).unwrap());
            match find_steps(cur_pos, grid[next_pos.0][next_pos.1], next_pos, grid) {
                None => { return None; }
                Some(mut ret) => { ret.append(&mut vec![cur_pos]);
                    return Some(ret)}
            }
        }
    }
}

fn calc_path(s: (usize, usize), grid: &Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
    for n in get_grid_neigh_indexes(s, (grid.len(), grid[0].len())) {
        let char = grid[n.0][n.1];
        if char == '.' { continue; }
        if match_shape(char).contains(&get_pos_diff(&s, &n)) {
            return Some(find_steps(s, char, n, grid).unwrap());
        }
    }
    None
}

pub fn run(input: &String, part: Part) -> String {
    let mut result: u32 = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start: Option<(usize, usize)> = None;

    // parse input into grid; find start
    for (i_x, line) in input.lines().enumerate() {
        if start == None {
            if let Some(i_y) = line.chars().position(|c| c == 'S') { start = Some((i_x, i_y)); }
        }
        grid.push(line.chars().collect());
    }
    let start = start.unwrap();

    // find path
    let path = calc_path(start, &grid).expect("No full path found!");
    if path.len() == 0 { return format!("No route found!") }

    // replace start with correct pipe shape
    let start_diffs = [get_pos_diff(path.last().unwrap(), &path[0]), get_pos_diff(&path[0], &path[1])];
    for c in ['-', '|', 'L', 'J', 'F', '7'] {
        let diffs = match_shape(c);
        if diffs.eq(&start_diffs) {
            grid[start.0][start.1] = c;
            break;
        }
    }

    match part {
        Part::First => {
            let mut path_length = path.len();
            if path_length % 2 == 0 { path_length -= 1; }
            result = (u32::try_from(path_length).unwrap() +1 ) / 2;
        }
        Part::Second => {
            let mut inside: bool = false;
            for x in 0..grid.len() {
                for y in 0..grid[x].len() {
                    let char = grid[x][y];
                    if path.contains(&(x, y)) {
                        if env::var("AOC_DEBUG").is_ok() { print!("{}", char.to_string().yellow()); }
                        match char {
                            '|' => { inside = !inside; },
                            'L' => {
                                for yy in y+1..grid[x].len() {
                                    match grid[x][yy] {
                                        '-' => {},
                                        '7' => { inside = !inside; break; },
                                        _ => { break; }
                                    }
                                }
                            },
                            'F' => {
                                for yy in y+1..grid[x].len() {
                                    match grid[x][yy] {
                                        '-' => {},
                                        'J' => { inside = !inside; break; },
                                        _ => { break; }
                                    }
                                }
                            },
                            _ => {}
                        }
                    } else {
                        if inside {
                            if env::var("AOC_DEBUG").is_ok() { print!("{}", char.to_string().red()); }
                            result += 1;
                        } else if env::var("AOC_DEBUG").is_ok() { print!("{char}"); }
                    }
                }
                if env::var("AOC_DEBUG").is_ok() { println!(); }
            }
        }
    }

    format!("{}", result)
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
    File::open("src/days/day10/test_input_2").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::Second);
    assert_eq!(result, "10");
}
