use crate::input::read_input;
use im_rc::{HashMap, Vector};
use std::iter::FromIterator;
use std::ops::Index;

pub fn execute() {
    let intcode: Intcode = read_input("day5")
        .split(",")
        .map(|i| i.parse::<i32>().unwrap_or(0))
        .collect();
    let mut program = Program {
        state: ProgramState {
            intcode,
            current_instruction: 0,
            input: Input { value: 1 },
            output: Output { value: None },
        },
        executors: HashMap::from(vec![
            (ADD_OPCODE, ADD_EXECUTOR),
            (MULTIPLY_OPCODE, MULTIPLY_EXECUTOR),
            (INPUT_OPCODE, INPUT_EXECUTOR),
            (OUTPUT_OPCODE, OUTPUT_EXECUTOR),
        ]),
    };
    program.run();

    println!("5:1 — Diagnostic code: {}", program.state.output.value.unwrap());
}

const ADD_OPCODE: i32 = 1;
const MULTIPLY_OPCODE: i32 = 2;
const INPUT_OPCODE: i32 = 3;
const OUTPUT_OPCODE: i32 = 4;
const END_OPCODE: i32 = 99;

static INSTRUCTION_NB_PARAMETERS: [usize; 5] = [0, 3, 3, 1, 1];

static ADD_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let parameter1_value = state.read_parameter(0, parameters[0]);
    let parameter2_value = state.read_parameter(1, parameters[1]);
    let result = parameter1_value + parameter2_value;
    let output_position = state.read_parameter(2, ParameterMode::Immediate);
    ProgramState {
        intcode: state.intcode.write(output_position as usize, result),
        current_instruction: state.current_instruction + 4,
        input: state.input,
        output: state.output,
    }
};

static MULTIPLY_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let parameter1_value = state.read_parameter(0, parameters[0]);
    let parameter2_value = state.read_parameter(1, parameters[1]);
    let result = parameter1_value * parameter2_value;
    let output_position = state.read_parameter(2, ParameterMode::Immediate);
    ProgramState {
        intcode: state.intcode.write(output_position as usize, result),
        current_instruction: state.current_instruction + 4,
        input: state.input,
        output: state.output,
    }
};

static INPUT_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let result = state.input.read();
    let output_position = state.read_parameter(0, ParameterMode::Immediate);
    ProgramState {
        intcode: state.intcode.write(output_position as usize, result),
        current_instruction: state.current_instruction + 2,
        input: state.input,
        output: state.output,
    }
};

static OUTPUT_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let value = state.read_parameter(0, parameters[0]);
    let output = state.output.write(value);
    ProgramState {
        intcode: state.intcode.clone(),
        current_instruction: state.current_instruction + 2,
        input: state.input,
        output,
    }
};

#[derive(Copy, Clone)]
struct Input {
    value: i32,
}

impl Input {
    fn read(&self) -> i32 {
        self.value
    }
}

#[derive(Copy, Clone)]
struct Output {
    pub value: Option<i32>,
}

impl Output {
    fn write(&self, out: i32) -> Output {
        Output { value: Some(out) }
    }
}

struct Program {
    pub state: ProgramState,
    executors: HashMap<i32, OpcodeExecutor>,
}

struct ProgramState {
    intcode: Intcode,
    current_instruction: usize,
    input: Input,
    output: Output,
}

#[derive(Clone)]
struct Intcode {
    code: Vector<i32>,
}

type OpcodeExecutor = fn(&ProgramState, parameters: &Vector<ParameterMode>) -> ProgramState;

#[derive(PartialEq, Clone, Debug, Copy)]
enum ParameterMode {
    Position,
    Immediate,
}

impl Program {
    pub fn run(&mut self) {
        while !self.state.is_over() {
            let instruction = self.state.get_next_instruction();
            let executor = self.executors[&instruction.0];
            self.state = executor(&self.state, &instruction.1);
        }
    }
}

impl ProgramState {
    pub fn is_over(&self) -> bool {
        self.instruction() == END_OPCODE
    }

    pub fn get_next_instruction(&self) -> (i32, Vector<ParameterMode>) {
        parse_instruction(self.instruction())
    }

    pub fn read_parameter(&self, index: usize, mode: ParameterMode) -> i32 {
        self.intcode
            .read(self.current_instruction + 1 + index, mode)
    }

    fn instruction(&self) -> i32 {
        self.intcode[self.current_instruction]
    }
}

impl Intcode {
    pub fn read(&self, position: usize, mode: ParameterMode) -> i32 {
        let position_value = self.code[position];
        match mode {
            ParameterMode::Position => self.code[position_value as usize],
            ParameterMode::Immediate => position_value,
        }
    }

    pub fn write(&self, position: usize, value: i32) -> Intcode {
        Intcode {
            code: self.code.update(position, value),
        }
    }
}

impl FromIterator<i32> for Intcode {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        Intcode {
            code: Vector::from_iter(iter),
        }
    }
}

impl Index<usize> for Intcode {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.code[index]
    }
}

fn parse_instruction(instruction: i32) -> (i32, Vector<ParameterMode>) {
    let opcode = instruction % 100;
    let mut parameters = parse_parameters(instruction / 100);
    fill_parameters(opcode, &mut parameters);
    (opcode, parameters)
}

fn parse_parameters(instruction_parameters: i32) -> Vector<ParameterMode> {
    let mut parameters = Vector::new();
    let mut rest = instruction_parameters;

    while rest > 0 {
        parameters.push_back(match rest % 10 {
            0 => ParameterMode::Position,
            _ => ParameterMode::Immediate,
        });
        rest /= 10;
    }

    parameters
}

fn fill_parameters(opcode: i32, parameters: &mut Vector<ParameterMode>) {
    let nb_parameters = *INSTRUCTION_NB_PARAMETERS.get(opcode as usize).unwrap_or(&0);
    while parameters.len() < nb_parameters {
        parameters.push_back(ParameterMode::Position)
    }
}

#[cfg(test)]
mod parse_instruction_should {
    use super::*;

    #[test]
    fn return_opcode_for_instruction_with_no_parameters() {
        let result = parse_instruction(END_OPCODE);

        assert_eq!(result, (END_OPCODE, Vector::new()));
    }

    #[test]
    fn return_opcode_and_position_mode_for_instruction_with_one_parameter_in_position_mode() {
        let result = parse_instruction(INPUT_OPCODE);

        assert_eq!(
            result,
            (INPUT_OPCODE, Vector::from(vec![ParameterMode::Position]))
        );
    }

    #[test]
    fn return_opcode_and_immediate_mode_for_instruction_with_one_parameter_in_immediate_mode() {
        let result = parse_instruction(103);

        assert_eq!(
            result,
            (INPUT_OPCODE, Vector::from(vec![ParameterMode::Immediate]))
        );
    }

    #[test]
    fn return_opcode_and_parameter_modes_for_example() {
        let result = parse_instruction(1002);

        assert_eq!(
            result,
            (
                MULTIPLY_OPCODE,
                Vector::from(vec![
                    ParameterMode::Position,
                    ParameterMode::Immediate,
                    ParameterMode::Position
                ])
            )
        );
    }
}
