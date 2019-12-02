use crate::input::read_input;
use im_rc::vector::Vector;

const ADD_OPCODE: usize = 1;
const MULTIPLY_OPCODE: usize = 2;
const END_OPCODE: usize = 99;

struct ProgramState {
    pub int_code: Vector<usize>,
    pub instruction_pointer: usize,
}

pub fn execute() {
    let int_code: Vector<usize> = read_input("day2")
        .split(",")
        .map(|i| i.parse::<usize>().unwrap_or(0))
        .collect();
    let program_state = ProgramState {
        int_code: int_code.update(1, 12).update(2, 2),
        instruction_pointer: 0,
    };
    println!(
        "2:1 — Int code program result: {}",
        program_state.run().int_code[0]
    );
}

impl ProgramState {
    pub fn opcode(&self) -> usize {
        self.int_code[self.instruction_pointer]
    }

    pub fn run(self) -> ProgramState {
        match self.opcode() {
            END_OPCODE => self,
            _ => self.execute_instruction().run(),
        }
    }

    fn update(&self, address: usize, value: usize, instruction_size: usize) -> ProgramState {
        ProgramState {
            int_code: self.int_code.update(address, value),
            instruction_pointer: self.instruction_pointer + instruction_size,
        }
    }

    fn read_parameter_value(&self, parameter: usize) -> usize {
        self.int_code[self.instruction_pointer + parameter]
    }

    fn read_pointer_parameter_value(&self, parameter: usize) -> usize {
        self.int_code[self.read_parameter_value(parameter)]
    }

    fn execute_instruction(self) -> ProgramState {
        match self.opcode() {
            ADD_OPCODE => self.add(),
            MULTIPLY_OPCODE => self.multiply(),
            o => panic!("Unknown opcode {}", o),
        }
    }

    fn add(&self) -> ProgramState {
        let parameter1_value = self.read_pointer_parameter_value(1);
        let parameter2_value = self.read_pointer_parameter_value(2);
        self.update(
            self.read_parameter_value(3),
            parameter1_value + parameter2_value,
            4,
        )
    }

    fn multiply(&self) -> ProgramState {
        let parameter1_value = self.read_pointer_parameter_value(1);
        let parameter2_value = self.read_pointer_parameter_value(2);
        self.update(
            self.read_parameter_value(3),
            parameter1_value * parameter2_value,
            4,
        )
    }
}

#[cfg(test)]
mod execute_instruction_should {
    use super::ProgramState;

    #[test]
    fn sum_two_numbers_with_opcode_1() {
        let program_state = ProgramState {
            int_code: im_rc::vector![1, 0, 0, 0, 99],
            instruction_pointer: 0,
        };

        let result = program_state.execute_instruction();

        assert_eq!(im_rc::vector![2, 0, 0, 0, 99], result.int_code);
    }

    #[test]
    fn multiply_two_numbers_with_opcode_2() {
        let program_state = ProgramState {
            int_code: im_rc::vector![2, 3, 0, 3, 99],
            instruction_pointer: 0,
        };

        let result = program_state.execute_instruction();

        assert_eq!(im_rc::vector![2, 3, 0, 6, 99], result.int_code);
    }

    #[test]
    fn modify_positions_outside_of_the_instruction() {
        let program_state = ProgramState {
            int_code: im_rc::vector![2, 4, 4, 5, 99, 0],
            instruction_pointer: 0,
        };

        let result = program_state.execute_instruction();

        assert_eq!(im_rc::vector![2, 4, 4, 5, 99, 9801], result.int_code);
    }
}

#[cfg(test)]
mod run_should {
    use super::ProgramState;

    #[test]
    fn execute_multiple_instructions() {
        let program_state = ProgramState {
            int_code: im_rc::vector![1, 1, 1, 4, 99, 5, 6, 0, 99],
            instruction_pointer: 0,
        };

        let result = program_state.run();

        assert_eq!(im_rc::vector!(30, 1, 1, 4, 2, 5, 6, 0, 99), result.int_code);
    }

    #[test]
    fn execute_the_sample() {
        let program_state = ProgramState {
            int_code: im_rc::vector![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            instruction_pointer: 0,
        };

        let result = program_state.run();

        assert_eq!(
            im_rc::vector!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50),
            result.int_code
        );
    }
}
