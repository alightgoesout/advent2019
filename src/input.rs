use im_rc::Vector;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Result};

pub fn read_input(day: &str) -> String {
    read_to_string(format!("src/input/{}", day))
        .expect(format!("Could not read INPUT file for {}", day).as_ref())
}

pub fn read_lines(day: &str) -> Result<Vector<String>> {
    let file = File::open(format!("src/input/{}", day))?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}
