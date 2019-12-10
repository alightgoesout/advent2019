use im_rc::Vector;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::Index;

const END_CODE: i32 = 99;

pub struct Program<I: Input, O: Output> {
    state: ProgramState,
    input: I,
    pub output: O,
    excutors: HashMap<OpCode, Box<dyn InstructionExecutor<I, O>>>,
}

impl<I: Input, O: Output> Program<I, O> {
    pub fn new(intcode: Intcode, input: I, output: O) -> Self {
        Program {
            state: ProgramState {
                intcode,
                current_position: 0,
            },
            input,
            output,
            excutors: create_executors(),
        }
    }

    pub fn run(&mut self) -> Option<ProgramState> {
        self.last()
    }

    fn execute(&mut self) -> Option<ProgramState> {
        match self.state.is_over() {
            true => None,
            false => {
                let instruction = self.state.current_instruction();
                Some(self.excutors[&instruction.opcode].execute(
                    &self.state,
                    &instruction,
                    &mut self.input,
                    &mut self.output,
                ))
            }
        }
    }
}

impl<I: Input, O: Output> Iterator for Program<I, O> {
    type Item = ProgramState;

    fn next(&mut self) -> Option<Self::Item> {
        match self.execute() {
            Some(s) => {
                self.state = s;
                Some(self.state.clone())
            }
            None => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProgramState {
    intcode: Intcode,
    current_position: usize,
}

impl ProgramState {
    fn is_over(&self) -> bool {
        self.intcode[self.current_position] == END_CODE
    }

    fn current_instruction(&self) -> Instruction {
        Instruction::from(self.intcode[self.current_position])
    }

    fn read_parameter(&self, index: usize, mode: &ParameterMode) -> i32 {
        self.intcode.read(self.current_position + 1 + index, mode)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Intcode {
    code: Vector<i32>,
}

impl Intcode {
    fn read(&self, position: usize, mode: &ParameterMode) -> i32 {
        let position_value = self.code[position];
        match mode {
            ParameterMode::Position => self.code[position_value as usize],
            ParameterMode::Immediate => position_value,
        }
    }

    fn write(&self, position: usize, value: i32) -> Intcode {
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

struct Instruction {
    opcode: OpCode,
    parameters_mode: Vector<ParameterMode>,
}

impl Instruction {
    fn from(instruction_code: i32) -> Self {
        let opcode = OpCode::from(instruction_code % 100);
        let mut parameters = Vector::new();
        let mut rest = instruction_code / 100;
        while rest > 0 {
            parameters.push_back(match rest % 10 {
                0 => ParameterMode::Position,
                _ => ParameterMode::Immediate,
            });
            rest /= 10;
        }
        Instruction {
            opcode,
            parameters_mode: parameters,
        }
    }

    fn get_parameter_mode(&self, parameter: usize) -> &ParameterMode {
        self.parameters_mode
            .get(parameter)
            .unwrap_or(&ParameterMode::Position)
    }
}

#[derive(PartialEq, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl OpCode {
    fn from(code: i32) -> OpCode {
        match code {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            _ => unreachable!("Unknown opcode {}", code),
        }
    }
}

pub trait Input: Clone {
    fn read(&mut self) -> Option<i32>;
}

pub trait Output: Clone {
    fn write(&mut self, output: i32);
}

#[derive(Clone)]
pub struct VectorInput {
    vector: Vec<i32>,
    position: usize,
}

impl VectorInput {
    pub fn new(vector: Vec<i32>) -> Self {
        VectorInput {
            vector,
            position: 0,
        }
    }
}

impl Input for VectorInput {
    fn read(&mut self) -> Option<i32> {
        match self.vector.get(self.position) {
            Some(i) => {
                self.position += 1;
                Some(*i)
            }
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct LastOutput {
    pub value: Option<i32>,
}

impl LastOutput {
    pub fn new() -> Self {
        LastOutput { value: None }
    }
}

impl Output for LastOutput {
    fn write(&mut self, output: i32) {
        self.value = Some(output);
    }
}

fn create_executors<I: Input, O: Output>() -> HashMap<OpCode, Box<dyn InstructionExecutor<I, O>>> {
    let mut map: HashMap<OpCode, Box<dyn InstructionExecutor<I, O>>> = HashMap::new();
    map.insert(OpCode::Add, Box::new(AddExecutor {}));
    map.insert(OpCode::Multiply, Box::new(MultiplyExecutor {}));
    map.insert(OpCode::Input, Box::new(InputExecutor {}));
    map.insert(OpCode::Output, Box::new(OutputExecutor {}));
    map.insert(OpCode::JumpIfFalse, Box::new(JumpIfFalseExecutor {}));
    map.insert(OpCode::JumpIfTrue, Box::new(JumpIfTrueExecutor {}));
    map.insert(OpCode::LessThan, Box::new(LessThanExecutor {}));
    map.insert(OpCode::Equals, Box::new(EqualsExecutor {}));
    map
}

trait InstructionExecutor<I: Input, O: Output> {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        input: &mut I,
        output: &mut O,
    ) -> ProgramState;
}

struct AddExecutor {}

impl<I: Input, O: Output> InstructionExecutor<I, O> for AddExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: &mut I,
        _output: &mut O,
    ) -> ProgramState {
        let parameter1_value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let parameter2_value = state.read_parameter(1, instruction.get_parameter_mode(1));
        let result = parameter1_value + parameter2_value;
        let output_position = state.read_parameter(2, &ParameterMode::Immediate);
        let intcode = state.intcode.write(output_position as usize, result);
        ProgramState {
            intcode,
            current_position: state.current_position + 4,
        }
    }
}

struct MultiplyExecutor {}

impl<I: Input, O: Output> InstructionExecutor<I, O> for MultiplyExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: &mut I,
        _output: &mut O,
    ) -> ProgramState {
        let parameter1_value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let parameter2_value = state.read_parameter(1, instruction.get_parameter_mode(1));
        let result = parameter1_value * parameter2_value;
        let output_position = state.read_parameter(2, &ParameterMode::Immediate);
        let intcode = state.intcode.write(output_position as usize, result);
        ProgramState {
            intcode,
            current_position: state.current_position + 4,
        }
    }
}

struct InputExecutor {}

impl<I: Input, O: Output> InstructionExecutor<I, O> for InputExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        _instruction: &Instruction,
        input: &mut I,
        _output: &mut O,
    ) -> ProgramState {
        let result = input.read().unwrap();
        let output_position = state.read_parameter(0, &ParameterMode::Immediate);
        let intcode = state.intcode.write(output_position as usize, result);
        ProgramState {
            intcode,
            current_position: state.current_position + 2,
        }
    }
}

struct OutputExecutor {}

impl<I: Input, O: Output> InstructionExecutor<I, O> for OutputExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: &mut I,
        output: &mut O,
    ) -> ProgramState {
        let value = state.read_parameter(0, instruction.get_parameter_mode(0));
        output.write(value);
        ProgramState {
            intcode: state.intcode.clone(),
            current_position: state.current_position + 2,
        }
    }
}

struct JumpIfTrueExecutor {}

impl<I: Input, O: Output> InstructionExecutor<I, O> for JumpIfTrueExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: &mut I,
        _output: &mut O,
    ) -> ProgramState {
        let value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let new_position = match value {
            0 => state.current_position + 3,
            _ => state.read_parameter(1, instruction.get_parameter_mode(1)) as usize,
        };
        ProgramState {
            intcode: state.intcode.clone(),
            current_position: new_position,
        }
    }
}

struct JumpIfFalseExecutor {}

impl<I: Input, O: Output> InstructionExecutor<I, O> for JumpIfFalseExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: &mut I,
        _output: &mut O,
    ) -> ProgramState {
        let value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let new_position = match value {
            0 => state.read_parameter(1, instruction.get_parameter_mode(1)) as usize,
            _ => state.current_position + 3,
        };
        ProgramState {
            intcode: state.intcode.clone(),
            current_position: new_position,
        }
    }
}

struct LessThanExecutor {}

impl<I: Input, O: Output> InstructionExecutor<I, O> for LessThanExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: &mut I,
        _output: &mut O,
    ) -> ProgramState {
        let parameter1_value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let parameter2_value = state.read_parameter(1, instruction.get_parameter_mode(1));
        let output_position = state.read_parameter(2, &ParameterMode::Immediate);
        let output = match parameter1_value.cmp(&parameter2_value) {
            Ordering::Less => 1,
            _ => 0,
        };
        let intcode = state.intcode.write(output_position as usize, output);
        ProgramState {
            intcode,
            current_position: state.current_position + 4,
        }
    }
}

struct EqualsExecutor {}

impl<I: Input, O: Output> InstructionExecutor<I, O> for EqualsExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: &mut I,
        _output: &mut O,
    ) -> ProgramState {
        let parameter1_value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let parameter2_value = state.read_parameter(1, instruction.get_parameter_mode(1));
        let output_position = state.read_parameter(2, &ParameterMode::Immediate);
        let output = match parameter1_value.cmp(&parameter2_value) {
            Ordering::Equal => 1,
            _ => 0,
        };
        let intcode = state.intcode.write(output_position as usize, output);
        ProgramState {
            intcode,
            current_position: state.current_position + 4,
        }
    }
}

#[cfg(test)]
mod intcode_tests {
    use super::*;

    #[test]
    fn execute_sample_code_with_add_and_multiply() {
        let mut program = Program::new(
            Intcode::from(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            VectorInput::new(Vec::new()),
            LastOutput::new(),
        );

        let result = program.run();

        assert_eq!(
            result.unwrap().intcode,
            Intcode::from(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])
        );
    }

    #[test]
    fn sample1_should_return_1_when_input_is_8() {
        let mut program = Program::new(
            Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]),
            VectorInput::new(vec![8]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(1));
    }

    #[test]
    fn sample1_should_return_0_when_input_is_not_8() {
        let mut program = Program::new(
            Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]),
            VectorInput::new(vec![7]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(0));
    }

    #[test]
    fn sample2_should_return_1_when_input_is_less_than_8() {
        let mut program = Program::new(
            Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]),
            VectorInput::new(vec![5]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(1));
    }

    #[test]
    fn sample2_should_return_0_when_input_is_8() {
        let mut program = Program::new(
            Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]),
            VectorInput::new(vec![8]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(0));
    }

    #[test]
    fn sample3_should_return_1_when_input_is_8() {
        let mut program = Program::new(
            Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]),
            VectorInput::new(vec![8]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(1));
    }

    #[test]
    fn sample3_should_return_0_when_input_is_not_8() {
        let mut program = Program::new(
            Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]),
            VectorInput::new(vec![9]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(0));
    }

    #[test]
    fn sample4_should_return_1_when_input_is_less_than_8() {
        let mut program = Program::new(
            Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]),
            VectorInput::new(vec![5]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(1));
    }

    #[test]
    fn sample4_should_return_0_when_input_is_8() {
        let mut program = Program::new(
            Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]),
            VectorInput::new(vec![8]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(0));
    }

    #[test]
    fn sample5_should_return_0_when_input_is_0() {
        let mut program = Program::new(
            Intcode::from(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]),
            VectorInput::new(vec![0]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(0));
    }

    #[test]
    fn sample5_should_return_1_when_input_is_not_0() {
        let mut program = Program::new(
            Intcode::from(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]),
            VectorInput::new(vec![2]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(1));
    }

    #[test]
    fn sample6_should_return_0_when_input_is_0() {
        let mut program = Program::new(
            Intcode::from(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]),
            VectorInput::new(vec![0]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(0));
    }

    #[test]
    fn sample6_should_return_1_when_input_is_not_0() {
        let mut program = Program::new(
            Intcode::from(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]),
            VectorInput::new(vec![2]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(1));
    }

    #[test]
    fn sample7_should_return_999_when_input_is_less_than_8() {
        let mut program = Program::new(
            Intcode::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            VectorInput::new(vec![7]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(999));
    }

    #[test]
    fn sample7_should_return_1000_when_input_is_8() {
        let mut program = Program::new(
            Intcode::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            VectorInput::new(vec![8]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(1000));
    }

    #[test]
    fn sample7_should_return_1001_when_input_is_more_than_8() {
        let mut program = Program::new(
            Intcode::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            VectorInput::new(vec![9]),
            LastOutput::new(),
        );

        program.run();

        assert_eq!(program.output.value, Some(1001));
    }
}
