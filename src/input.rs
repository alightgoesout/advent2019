use std::fs;

pub fn read_input(day: &str) -> String {
    fs::read_to_string(format!("src/input/{}", day))
        .expect(format!("Could not read INPUT file for {}", day).as_ref())
}
