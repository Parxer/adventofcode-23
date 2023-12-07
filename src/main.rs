pub mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use std::env;
use std::error::Error;
use dotenv::dotenv;
use aocf::Aoc;

use crate::common::Part;
// use crate::day01::run_day_01;
// use crate::day02::run_day_02;
// use crate::day03::run_day_03;
// use crate::day04::run_day_04;
// use crate::day05::run_day_05;
// use crate::day06::run_day_06;
use crate::day07::run_day_07;

fn main() {
    dotenv().ok();

    let year = 2023;
    let session_cookie: Option<String> = env::var("AOC_SESSION_COOKIE").ok();

    // if let Ok(input) = get_day(year, 1, &session_cookie) {
    //     print_day(1, run_day_01(input, Part::Second));
    // }
    //
    // if let Ok(input) = get_day(year, 2, &session_cookie) {
    //     print_day(2, run_day_02(input, Part::Second));
    // }
    //
    // if let Ok(input) = get_day(year, 3, &session_cookie) {
    //     print_day(3, run_day_03(input, Part::First));
    // }
    //
    // if let Ok(input) = get_day(year, 4, &session_cookie) {
    //     print_day(4, run_day_04(input, Part::Second));
    // }
    //
    // if let Ok(input) = get_day(year, 5, &session_cookie) {
    //     print_day(5, run_day_05(input, Part::Second));
    // }

    // if let Ok(input) = get_day(year, 6, &session_cookie) {
    //     print_day(6, run_day_06(input, Part::Second));
    // }

    if let Ok(input) = get_day(year, 7, &session_cookie) {
        print_day(7, run_day_07(input, Part::Second));
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

fn print_day(day: u32, result: u32) {
    println!("Day {day} result:");
    println!("{result}");
    println!("- - - - - - - -");
    println!("\n\n");
}
