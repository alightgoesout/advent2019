use crate::input::read_input;
use im_rc::Vector;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::Index;

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
    let (a, b, c, d, e) = (
        settings[0],
        settings[1],
        settings[2],
        settings[3],
        settings[4],
    );
    let amplifier_a = ProgramState::new(intcode, Input::new(Vector::from(vec![a, 0])));
    let amplifier_a_result = run(amplifier_a);
    let amplifier_b = ProgramState::new(
        intcode,
        Input::new(Vector::from(vec![
            b,
            amplifier_a_result.output.value.unwrap(),
        ])),
    );
    let amplifier_b_result = run(amplifier_b);
    let amplifier_c = ProgramState::new(
        intcode,
        Input::new(Vector::from(vec![
            c,
            amplifier_b_result.output.value.unwrap(),
        ])),
    );
    let amplifier_c_result = run(amplifier_c);
    let amplifier_d = ProgramState::new(
        intcode,
        Input::new(Vector::from(vec![
            d,
            amplifier_c_result.output.value.unwrap(),
        ])),
    );
    let amplifier_d_result = run(amplifier_d);
    let amplifier_e = ProgramState::new(
        intcode,
        Input::new(Vector::from(vec![
            e,
            amplifier_d_result.output.value.unwrap(),
        ])),
    );
    let amplifier_e_result = run(amplifier_e);
    amplifier_e_result.output.value.unwrap()
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
        input: state.input.clone(),
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
        input: state.input.clone(),
        output: state.output,
    }
};

static INPUT_EXECUTOR: OpcodeExecutor = |state, _parameters| {
    let mut input = state.input.clone();
    let result = input.read();
    let output_position = state.read_parameter(0, ParameterMode::Immediate);
    let intcode = state.intcode.write(output_position as usize, result);
    ProgramState {
        intcode,
        current_instruction: state.current_instruction + 2,
        input,
        output: state.output,
    }
};

static OUTPUT_EXECUTOR: OpcodeExecutor = |state, parameters| {
    let value = state.read_parameter(0, parameters[0]);
    let output = state.output.write(value);
    ProgramState {
        intcode: state.intcode.clone(),
        current_instruction: state.current_instruction + 2,
        input: state.input.clone(),
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
        input: state.input.clone(),
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
        input: state.input.clone(),
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
        input: state.input.clone(),
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
        input: state.input.clone(),
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

#[derive(Clone)]
struct Input {
    values: Vector<i32>,
    current_position: usize,
}

impl Input {
    fn new(values: Vector<i32>) -> Self {
        Input {
            values,
            current_position: 0,
        }
    }

    fn read(&mut self) -> i32 {
        let value = self.values[self.current_position];
        self.current_position += 1;
        value
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
