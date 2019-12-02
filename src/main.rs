use std::io::{stdin, stdout, Result, Write};

mod day1;
mod day2;
mod input;

fn main() -> Result<()> {
    let puzzle = read_console_input()?;
    execute_puzzle(puzzle);
    Ok(())
}

fn read_console_input() -> Result<String> {
    print!("Puzzle to run: ");
    stdout().flush()?;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    Ok(line)
}

fn execute_puzzle(puzzle: String) {
    match puzzle.trim().as_ref() {
        "1" => day1::execute(),
        "2" => day2::execute(),
        s => println!("Unknown puzzle: '{}'", s),
    }
}
