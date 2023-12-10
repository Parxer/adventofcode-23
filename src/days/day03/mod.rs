use std::env;
use crate::common::{get_grid_neigh_indexes, Part};

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;

#[derive(Debug)]
struct Number {
    value: u32,
    start: [usize; 2],
    end: [usize; 2]
}

pub fn run(input: &String, part: Part) -> String {
    if part == Part::Second { return format!("N/A") }
    let mut result: u32 = 0;

    let w = input.lines().peekable().peek().unwrap().len();
    let h = input.lines().count();
    let mut grid = vec![vec![' '; w]; h];
    let mut numbers: Vec<Number> = vec![];

    let mut i = 0;
    for line in input.lines() {
        let mut started_reading_number = line.chars().next().unwrap().is_digit(10);
        let mut start = [i, 0];

        for (j, c) in line.char_indices() {
            let is_digit = c.is_digit(10);

            if !started_reading_number && is_digit {
                started_reading_number = true;
                start[1] = j;
            } else if started_reading_number && !is_digit {
                started_reading_number = false;
                let end = [i, j-1];

                numbers.push(Number {
                    value: u32::from_str_radix(&line[start[1]..=end[1]], 10).unwrap(),
                    start,
                    end
                });
            }

            grid[i][j] = if is_digit || c == '.' { ' ' } else { c }
        }

        if started_reading_number {
            let end = [i, line.len() - 1];
            numbers.push(Number {
                value: u32::from_str_radix(&line[start[1]..=end[1]], 10).unwrap(),
                start,
                end
            });
        }
        i += 1;
    }

    for number in numbers {
        let mut all_neighbors: Vec<char> = vec![];
        (number.start[1]..=number.end[1]).for_each(|y| {
            let i_neighbors = get_grid_neigh_indexes((number.start[0], y), (grid.len(), grid[0].len()));
            for i_n in i_neighbors {
                if i_n.0 != number.start[0] &&
                    (number.start[1]..=number.end[1]).contains(&i_n.1) &&
                    grid[i_n.0][i_n.1] != ' ' {all_neighbors.push(grid[i_n.0][i_n.1]) }
            }
        });
        if all_neighbors.iter().filter(|&c| *c != ' ').count() > 0 {
            println!("Number {} has a neighboring symbol(s): {:?}", number.value, all_neighbors);
            result += number.value;
        }
    }

    format!("{result}")
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input = String::new();
    File::open("src/days/day03/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();

    let result = run(&sample_input, Part::First);
    assert_eq!(result, "4474");
}

// #[test]
// fn test_part_2() {
//     env::set_var("AOC_DEBUG", "1");
//
//     let mut sample_input = String::new();
//     File::open("src/days/day03/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input).ok();
//
//     let result = run(&sample_input, Part::Second);
//     assert_eq!(result, "467835");
// }