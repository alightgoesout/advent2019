use crate::input::read_input;
use im_rc::vector::Vector;

const ADD_OPCODE: usize = 1;
const MULTIPLY_OPCODE: usize = 2;
const END_OPCODE: usize = 99;

pub fn execute() {
    let int_code: Vector<usize> = read_input("day2")
        .split(",")
        .map(|l| l.parse::<usize>().unwrap_or(0))
        .collect();
    println!(
        "2:1 — Int code program result: {}",
        execute_instructions(int_code.update(1, 12).update(2, 2), 0)[0]
    );
}

fn execute_instructions(int_code: Vector<usize>, position: usize) -> Vector<usize> {
    let opcode = int_code[position];
    match opcode {
        END_OPCODE => int_code,
        _ => execute_instructions(execute_instruction(int_code, position), position + 4),
    }
}

fn execute_instruction(int_code: Vector<usize>, position: usize) -> Vector<usize> {
    let opcode = int_code[position];
    let value = match opcode {
        ADD_OPCODE => read_value(&int_code, position + 1) + read_value(&int_code, position + 2),
        MULTIPLY_OPCODE => {
            read_value(&int_code, position + 1) * read_value(&int_code, position + 2)
        }
        _ => panic!("Unknown opcode {}", opcode),
    };
    with_value(int_code, position + 3, value)
}

fn read_value(int_code: &Vector<usize>, position: usize) -> usize {
    int_code[int_code[position]]
}

fn with_value(int_code: Vector<usize>, position: usize, value: usize) -> Vector<usize> {
    int_code.update(int_code[position], value)
}

#[cfg(test)]
mod execute_instruction_should {
    use super::execute_instruction;

    #[test]
    fn sum_two_numbers_with_opcode_1() {
        let int_code = im_rc::vector![1, 0, 0, 0, 99];

        let result = execute_instruction(int_code, 0);

        assert_eq!(im_rc::vector![2, 0, 0, 0, 99], result);
    }

    #[test]
    fn multiply_two_numbers_with_opcode_2() {
        let int_code = im_rc::vector![2, 3, 0, 3, 99];

        let result = execute_instruction(int_code, 0);

        assert_eq!(im_rc::vector![2, 3, 0, 6, 99], result);
    }

    #[test]
    fn modify_positions_outside_of_the_instruction() {
        let int_code = im_rc::vector![2, 4, 4, 5, 99, 0];

        let result = execute_instruction(int_code, 0);

        assert_eq!(im_rc::vector![2, 4, 4, 5, 99, 9801], result);
    }
}

#[cfg(test)]
mod execute_instructions_should {
    use super::execute_instructions;

    #[test]
    fn execute_multiple_instructions() {
        let int_code = im_rc::vector![1, 1, 1, 4, 99, 5, 6, 0, 99];

        let result = execute_instructions(int_code, 0);

        assert_eq!(im_rc::vector!(30, 1, 1, 4, 2, 5, 6, 0, 99), result);
    }

    #[test]
    fn execute_the_sample() {
        let int_code = im_rc::vector![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let result = execute_instructions(int_code, 0);

        assert_eq!(
            im_rc::vector!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50),
            result
        );
    }
}
