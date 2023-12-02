use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::Read;
use crate::common::Part;

const R_MAX: u32 = 12;
const G_MAX: u32 = 13;
const B_MAX: u32 = 14;

fn check_validity(color: &str, value: u32) -> bool {
    match color.chars().next().unwrap() {
        'r' => value <= R_MAX,
        'g' => value <= G_MAX,
        'b' => value <= B_MAX,
        _ => true
    }
}

fn compare_assign_max(color: &str, value: u32, rgb_max: &mut [u32; 3]) {
    match color.chars().next().unwrap() {
        'r' => rgb_max[0] = max(value, rgb_max[0]),
        'g' => rgb_max[1] = max(value, rgb_max[1]),
        'b' => rgb_max[2] = max(value, rgb_max[2]),
        _ => { }
    }
}

pub fn run_day_02(input: String, part: Part) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let (game_str, draws_str) = line.split_once(':').unwrap();
        let game_num = game_str.split_ascii_whitespace().last().unwrap();

        let draws: Vec<&str> = draws_str.split(';').collect();

        let mut rgb_max: [u32; 3] = [0, 0, 0];
        let mut is_game_valid = true;

        for draw in draws {
            let colors: Vec<&str> = draw.split(',').collect();

            for color_str in colors {
                let mut color_iter = color_str.split_ascii_whitespace();
                let num = u32::from_str_radix(color_iter.next().unwrap(), 10).unwrap();
                let color = color_iter.next().unwrap();

                is_game_valid = check_validity(color, num);
                compare_assign_max(color, num, &mut rgb_max);

                if part == Part::First && !is_game_valid { break; }
            }
            if part == Part::First && !is_game_valid { break; }
        }

        if env::var("AOC_DEBUG").is_ok() {
            match part {
                Part::First => { println!("Game {game_num} is {}VALID", if !is_game_valid { "IN" } else { "" }); }
                Part::Second => { println!("Game maximums are: {rgb_max:?}"); }
            }
        }

        match part {
            Part::First => {
                if is_game_valid {
                    result += u32::from_str_radix(game_num, 10).expect("Failed to parse game num");
                }
            }
            Part::Second => {
                result += rgb_max.iter().fold(1, |pow, item  | item * pow);
            }
        }
    }
    return result;
}

#[test]
fn test_part_1() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input_1 = String::new();
    File::open("src/day02/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input_1).ok();

    let result = run_day_02(sample_input_1, Part::First);
    assert_eq!(result, 8);
}

#[test]
fn test_part_2() {
    env::set_var("AOC_DEBUG", "1");

    let mut sample_input_2 = String::new();
    File::open("src/day02/test_input").expect("Failed to open sample input").read_to_string(&mut sample_input_2).ok();

    let result = run_day_02(sample_input_2, Part::Second);
    assert_eq!(result, 2286);
}
