pub mod day01;

use std::env;
use std::error::Error;
use dotenv::dotenv;
use aocf::Aoc;
use day01::run_day_01;

fn main() {
    dotenv().ok();

    let year = 2023;
    let session_cookie: Option<String> = env::var("AOC_SESSION_COOKIE").ok();

    if let Ok(input) = get_day(year, 1, &session_cookie) {
        print_day(1, run_day_01(input, false));
    }
}

fn get_day(year: i32, day: u32, session_cookie: &Option<String>) -> Result<String, Box<dyn Error>> {
    let mut aoc = Aoc::new().year(Some(year)).day(Some(day));
    if let Some(session_str) = session_cookie {
        aoc = aoc.cookie(session_str);
    }

    aoc = aoc.init().unwrap();
    let input = aoc.get_input(false)?;
    Ok(input)
}

fn print_day(day: u32, result: u32) {
    println!("Day {day} result:");
    println!("{result}");
    println!("- - - - - - - -");
    println!("\n\n");
}
