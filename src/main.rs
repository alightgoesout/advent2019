use im_rc::Vector;
use std::env;
use std::io::{stdin, stdout, Result, Write};
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod input;
mod intcode;

fn main() -> Result<()> {
    let args: Vector<String> = env::args().collect();
    if args.len() > 1 {
        execute_day(&args[1]);
    } else {
        let day = read_console_input()?;
        execute_day(&day);
    }
    Ok(())
}

fn read_console_input() -> Result<String> {
    print!("Day to run: ");
    stdout().flush()?;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    Ok(line)
}

fn execute_day(day: &String) {
    let start = Instant::now();
    match day.trim().as_ref() {
        "1" => day1::execute(),
        "2" => day2::execute(),
        "3" => day3::execute(),
        "4" => day4::execute(),
        "5" => day5::execute(),
        "6" => day6::execute(),
        "7" => day7::execute(),
        s => println!("Unknown day: '{}'", s),
    }

    let duration = start.elapsed();
    println!("Done in: {:?}", duration);
}
