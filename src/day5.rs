use crate::input::read_input;
use im_rc::Vector;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::Index;

pub fn execute() {
    let intcode: Intcode = read_input("day5")
        .split(",")
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect();
    let program1 = run(ProgramState::new(&intcode, Input { value: 1 }));
    let program2 = run(ProgramState::new(&intcode, Input { value: 5 }));
    println!("5:1 — Diagnostic code: {}", program1.output.value.unwrap());
    println!("5:2 — Diagnostic code: {}", program2.output.value.unwrap());
}

const END_OPCODE: i32 = 99;

static INSTRUCTION_NB_PARAMETERS: [usize; 8] = [3, 3, 1, 1, 2, 2, 3, 3];
static EXECUTORS: [OpcodeExecutor; 8] = [
    ADD_EXECUTOR,
    MULTIPLY_EXECUTOR,
    INPUT_EXECUTOR,
    OUTPUT_EXECUTOR,
    JUMP_IF_TRUE_EXECUTOR,
    JUMP_IF_FALSE_EXECUTOR,
    LESS_THAN_EXECUTOR,
    EQUALS_EXECUTOR,
];

static ADD_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let parameter1_value = state.read_parameter(0, parameters[0]);
    let parameter2_value = state.read_parameter(1, parameters[1]);
    let result = parameter1_value + parameter2_value;
    let output_position = state.read_parameter(2, ParameterMode::Immediate);
    let intcode = state.intcode.write(output_position as usize, result);
    ProgramState {
        intcode,
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
    let intcode = state.intcode.write(output_position as usize, result);
    ProgramState {
        intcode,
        current_instruction: state.current_instruction + 4,
        input: state.input,
        output: state.output,
    }
};

static INPUT_EXECUTOR: OpcodeExecutor = |state, _parameters| {
    let result = state.input.read();
    let output_position = state.read_parameter(0, ParameterMode::Immediate);
    let intcode = state.intcode.write(output_position as usize, result);
    ProgramState {
        intcode,
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

static JUMP_IF_TRUE_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let value = state.read_parameter(0, parameters[0]);
    let new_instruction = match value {
        0 => state.current_instruction + 3,
        _ => state.read_parameter(1, parameters[1]) as usize,
    };
    ProgramState {
        intcode: state.intcode.clone(),
        current_instruction: new_instruction,
        input: state.input,
        output: state.output,
    }
};

static JUMP_IF_FALSE_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let value = state.read_parameter(0, parameters[0]);
    let new_instruction = match value {
        0 => state.read_parameter(1, parameters[1]) as usize,
        _ => state.current_instruction + 3,
    };
    ProgramState {
        intcode: state.intcode.clone(),
        current_instruction: new_instruction,
        input: state.input,
        output: state.output,
    }
};

static LESS_THAN_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let parameter1_value = state.read_parameter(0, parameters[0]);
    let parameter2_value = state.read_parameter(1, parameters[1]);
    let output_position = state.read_parameter(2, ParameterMode::Immediate);
    let output = match parameter1_value.cmp(&parameter2_value) {
        Ordering::Less => 1,
        _ => 0,
    };
    let intcode = state.intcode.write(output_position as usize, output);
    ProgramState {
        intcode,
        current_instruction: state.current_instruction + 4,
        input: state.input,
        output: state.output,
    }
};

static EQUALS_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let parameter1_value = state.read_parameter(0, parameters[0]);
    let parameter2_value = state.read_parameter(1, parameters[1]);
    let output_position = state.read_parameter(2, ParameterMode::Immediate);
    let output = match parameter1_value.cmp(&parameter2_value) {
        Ordering::Equal => 1,
        _ => 0,
    };
    let intcode = state.intcode.write(output_position as usize, output);
    ProgramState {
        intcode,
        current_instruction: state.current_instruction + 4,
        input: state.input,
        output: state.output,
    }
};

fn run(state: ProgramState) -> ProgramState {
    let mut current_state = state;
    while !current_state.is_over() {
        let instruction = current_state.get_next_instruction();
        let executor = EXECUTORS[(&instruction.0 - 1) as usize];
        current_state = executor(&current_state, &instruction.1);
    }
    current_state
}

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
    fn new() -> Self {
        Output { value: None }
    }

    fn write(&self, out: i32) -> Output {
        Output { value: Some(out) }
    }
}

struct ProgramState {
    intcode: Intcode,
    current_instruction: usize,
    input: Input,
    output: Output,
}

#[derive(Clone, Debug)]
struct Intcode {
    code: Vector<i32>,
}

type OpcodeExecutor = fn(&ProgramState, parameters: &Vector<ParameterMode>) -> ProgramState;

#[derive(PartialEq, Clone, Debug, Copy)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ProgramState {
    pub fn new(intcode: &Intcode, input: Input) -> Self {
        ProgramState {
            intcode: intcode.clone(),
            current_instruction: 0,
            input,
            output: Output::new(),
        }
    }

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

impl From<Vec<i32>> for Intcode {
    fn from(vec: Vec<i32>) -> Self {
        Intcode {
            code: Vector::from(vec),
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
    let nb_parameters = *INSTRUCTION_NB_PARAMETERS
        .get(opcode as usize - 1)
        .unwrap_or(&0);
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
        let result = parse_instruction(3);

        assert_eq!(result, (3, Vector::from(vec![ParameterMode::Position])));
    }

    #[test]
    fn return_opcode_and_immediate_mode_for_instruction_with_one_parameter_in_immediate_mode() {
        let result = parse_instruction(103);

        assert_eq!(result, (3, Vector::from(vec![ParameterMode::Immediate])));
    }

    #[test]
    fn return_opcode_and_parameter_modes_for_example() {
        let result = parse_instruction(1002);

        assert_eq!(
            result,
            (
                2,
                Vector::from(vec![
                    ParameterMode::Position,
                    ParameterMode::Immediate,
                    ParameterMode::Position
                ])
            )
        );
    }
}

#[cfg(test)]
mod run_tests {
    use super::*;

    #[test]
    fn sample1_should_return_1_when_input_is_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]),
            Input { value: 8 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(1));
    }

    #[test]
    fn sample1_should_return_0_when_input_is_not_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]),
            Input { value: 7 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(0));
    }

    #[test]
    fn sample2_should_return_1_when_input_is_less_than_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]),
            Input { value: 5 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(1));
    }

    #[test]
    fn sample2_should_return_0_when_input_is_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]),
            Input { value: 8 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(0));
    }

    #[test]
    fn sample3_should_return_1_when_input_is_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]),
            Input { value: 8 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(1));
    }

    #[test]
    fn sample3_should_return_0_when_input_is_not_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]),
            Input { value: 9 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(0));
    }

    #[test]
    fn sample4_should_return_1_when_input_is_less_than_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]),
            Input { value: 5 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(1));
    }

    #[test]
    fn sample4_should_return_0_when_input_is_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]),
            Input { value: 8 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(0));
    }

    #[test]
    fn sample5_should_return_0_when_input_is_0() {
        let program = ProgramState::new(
            &Intcode::from(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]),
            Input { value: 0 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(0));
    }

    #[test]
    fn sample5_should_return_1_when_input_is_not_0() {
        let program = ProgramState::new(
            &Intcode::from(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]),
            Input { value: 2 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(1));
    }

    #[test]
    fn sample6_should_return_0_when_input_is_0() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]),
            Input { value: 0 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(0));
    }

    #[test]
    fn sample6_should_return_1_when_input_is_not_0() {
        let program = ProgramState::new(
            &Intcode::from(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]),
            Input { value: 2 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(1));
    }

    #[test]
    fn sample7_should_return_999_when_input_is_less_than_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            Input { value: 7 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(999));
    }

    #[test]
    fn sample7_should_return_1000_when_input_is_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            Input { value: 8 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(1000));
    }

    #[test]
    fn sample7_should_return_1001_when_input_is_more_than_8() {
        let program = ProgramState::new(
            &Intcode::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            Input { value: 9 },
        );

        let result = run(program);

        assert_eq!(result.output.value, Some(1001));
    }
}
