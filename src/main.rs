pub mod common;
mod days;

use days::{ day01, day02, day03, day04, day05, day06, day07, day08, day09, day10 };
use std::env;
use std::error::Error;
use dotenv::dotenv;
use aocf::Aoc;
use chrono::{Datelike, Local};

use crate::common::Part;

fn main() {
    dotenv().ok();

    let today = Local::now().date_naive();
    let year = today.year();
    let session_cookie: &Option<String> = &env::var("AOC_SESSION_COOKIE").ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No day arguments found; running for today ({today:?})");
    }

    let mut days: Vec<u32> = args[1..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();

    if days.len() == 0 { days.push(today.day()); }
    for day in days {
        run_day(year, day, session_cookie);
    }
}

fn get_day(year: i32, day: u32, session_cookie: &Option<String>) -> Result<String, Box<dyn Error>> {
    let mut aoc = Aoc::new().year(Some(year)).day(Some(day));
    if let Some(session_str) = session_cookie {
        aoc = aoc.cookie(session_str);
    }

    aoc = aoc.init().expect("Failed to initialize aoc");
    let input = aoc.get_input(false)?;
    Ok(input)
}

fn run_day(year: i32, day: u32, session_cookie: &Option<String>) {
    let run = match day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        7 => day07::run,
        8 => day08::run,
        9 => day09::run,
        10 => day10::run,
        _ => unimplemented!()
    };

    if let Ok(input) = get_day(year, day, session_cookie) {
        print_day(day, run(&input, Part::First), run(&input, Part::Second));
    }
}

fn print_day(day: u32, first_result: String, second_result: String) {
    println!("Day {day} results:");
    println!("Part 1: {first_result}");
    println!("Part 2: {second_result}");
    println!("- - - - - - - -");
    println!("\n\n");
}
