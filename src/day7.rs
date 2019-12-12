use crate::input::read_input;
use crate::intcode::{Intcode, Program, Pipe};

pub fn execute() {
    let intcode: Intcode = read_input("day7")
        .split(",")
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect();
    let highest_signal = find_best_phase_setting(&intcode);
    println!("7:1 Hightest signal: {}", highest_signal.unwrap());
}

fn find_best_phase_setting(intcode: &Intcode) -> Option<i32> {
    PermutationsGenerator::new(5)
        .map(|s| try_phase_settings(intcode, &s))
        .max()
}

fn try_phase_settings(intcode: &Intcode, settings: &Vec<i32>) -> i32 {
    let output_pipe = Pipe::new();
    let mut amplifier_a = Program::new(intcode.clone());
    let mut amplifier_b = Program::new(intcode.clone());
    let mut amplifier_c = Program::new(intcode.clone());
    let mut amplifier_d = Program::new(intcode.clone());
    let mut amplifier_e = Program::new(intcode.clone());
    amplifier_a.connect(&amplifier_b);
    amplifier_b.connect(&amplifier_c);
    amplifier_c.connect(&amplifier_d);
    amplifier_d.connect(&amplifier_e);
    amplifier_e.set_output(&output_pipe);
    amplifier_a.write(settings[0]);
    amplifier_a.write(0);
    amplifier_b.write(settings[1]);
    amplifier_c.write(settings[2]);
    amplifier_d.write(settings[3]);
    amplifier_e.write(settings[4]);
    let mut results = [
        amplifier_a.run(),
        amplifier_b.run(),
        amplifier_c.run(),
        amplifier_d.run(),
        amplifier_e.run(),
    ];
    while results.iter().any(|r| !r) {
        results = [
            amplifier_a.run(),
            amplifier_b.run(),
            amplifier_c.run(),
            amplifier_d.run(),
            amplifier_e.run(),
        ];
    }
    output_pipe.read().unwrap()
}

struct PermutationsGenerator {
    vector: Vec<i32>,
    c: Vec<usize>,
    i: usize,
    first: bool,
}

impl PermutationsGenerator {
    fn new(size: usize) -> Self {
        PermutationsGenerator {
            vector: (0..size as i32).collect(),
            c: vec![0; size],
            i: 0,
            first: true,
        }
    }
}

impl Iterator for PermutationsGenerator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.vector.clone());
        }
        while self.i < self.vector.len() {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.vector.swap(0, self.i);
                } else {
                    self.vector.swap(self.c[self.i], self.i);
                }
                self.c[self.i] += 1;
                self.i = 0;
                return Some(self.vector.clone());
            }
            self.c[self.i] = 0;
            self.i += 1;
        }
        None
    }
}

#[cfg(test)]
mod permutations_generator_should {
    use super::*;

    #[test]
    fn return_all_permutations() {
        let generator = PermutationsGenerator::new(3);

        let result: Vec<Vec<i32>> = generator.collect();

        assert_eq!(
            result,
            vec![
                vec![0, 1, 2],
                vec![1, 0, 2],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![1, 2, 0],
                vec![2, 1, 0],
            ]
        );
    }
}
