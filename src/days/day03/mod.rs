use std::env;
use crate::common::Part;

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

fn neighbors_with_symbol(number: &Number, matrix: &Vec<Vec<bool>>) -> bool {
    let mat_w = matrix[0].len();
    let mat_h = matrix.len();

    let i = number.start[0];
    for j in number.start[1]..=number.end[1] {
        if
            (i > 0 && j > 0 && matrix[i-1][j-1]) ||
            (i > 0 && matrix[i-1][j]) ||
            (i > 0 && j + 1 < mat_w && matrix[i-1][j+1]) ||
            (j > 0 && matrix[i][j-1]) ||
            (j + 1 < mat_w && matrix[i][j+1]) ||
            (i + 1 < mat_h && j > 0 && matrix[i+1][j-1]) ||
            (i + 1 < mat_h && matrix[i+1][j]) ||
            (i + 1 < mat_h && j + 1 < mat_w && matrix[i+1][j+1]) {
            return true;
        }
    }
    false
}

pub fn run(input: &String, part: Part) -> String {
    if part == Part::Second { return format!("N/A") }
    let mut result: u32 = 0;

    let w = input.lines().peekable().peek().unwrap().len();
    let h = input.lines().count();
    let mut matrix = vec![vec![false; w]; h];
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

            matrix[i][j] = !is_digit && c != '.';
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
        let has_symbol = neighbors_with_symbol(&number, &matrix);
        if has_symbol { result += number.value; }
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