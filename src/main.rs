use std::io::{stdin, stdout, Error, Write};

mod input;
mod day1;

fn main() -> Result<(), Error> {
    let puzzle = read_console_input()?;
    execute_puzzle(puzzle);
    Ok(())
}

fn read_console_input() -> Result<String, Error> {
    print!("Puzzle to run: ");
    stdout().flush()?;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    Ok(line)
}

fn execute_puzzle(puzzle: String) {
    match puzzle.trim().as_ref() {
        "1" => day1::execute(),
        s => println!("Unknown puzzle: '{}'", s),
    }
}
