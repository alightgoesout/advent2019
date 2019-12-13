use im_rc::Vector;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use std::ops::Index;
use std::rc::Rc;

const END_CODE: i32 = 99;

pub struct Program {
    state: ProgramState,
    input: Pipe,
    output: Option<Pipe>,
    excutors: HashMap<OpCode, Box<dyn InstructionExecutor>>,
}

impl Program {
    pub fn new(intcode: Intcode) -> Self {
        Program {
            state: ProgramState {
                status: ProgramStatus::Running,
                intcode,
                current_position: 0,
            },
            input: Pipe::new(),
            output: None,
            excutors: create_executors(),
        }
    }

    pub fn connect(&mut self, program: &Program) {
        self.output = Some(program.input.clone());
    }

    pub fn set_output(&mut self, output: &Pipe) {
        self.output = Some(output.clone());
    }

    pub fn write(&self, value: i32) {
        self.input.write(value);
    }

    pub fn read(&self) -> Option<i32> {
        self.output.clone().and_then(|p| p.peek())
    }

    pub fn run(&mut self) -> bool {
        if self.state.status != ProgramStatus::Over {
            self.execute();
        }
        while self.state.status == ProgramStatus::Running {
            self.execute();
        }
        self.state.status == ProgramStatus::Over
    }

    fn execute(&mut self) {
        self.state = match self.state.is_over() {
            true => ProgramState {
                status: ProgramStatus::Over,
                intcode: self.state.intcode.clone(),
                current_position: self.state.current_position,
            },
            false => {
                let instruction = self.state.current_instruction();
                self.excutors[&instruction.opcode].execute(
                    &self.state,
                    &instruction,
                    self.input.clone(),
                    self.output.clone(),
                )
            }
        }
    }
}

#[derive(Clone)]
pub struct Pipe {
    queue: Rc<RefCell<VecDeque<i32>>>,
}

impl Pipe {
    pub fn new() -> Self {
        Pipe {
            queue: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn read(&self) -> Option<i32> {
        let value = self.queue.borrow_mut().pop_front();
        value
    }

    pub fn write(&self, value: i32) {
        self.queue.borrow_mut().push_back(value);
    }

    pub fn peek(&self) -> Option<i32> {
        self.queue.borrow().front().cloned()
    }
}

#[derive(Clone)]
pub struct ProgramState {
    pub status: ProgramStatus,
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
pub enum ProgramStatus {
    Running,
    Waiting,
    Over,
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

fn create_executors() -> HashMap<OpCode, Box<dyn InstructionExecutor>> {
    let mut map: HashMap<OpCode, Box<dyn InstructionExecutor>> = HashMap::new();
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

trait InstructionExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        input: Pipe,
        output: Option<Pipe>,
    ) -> ProgramState;
}

struct AddExecutor {}

impl InstructionExecutor for AddExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: Pipe,
        _output: Option<Pipe>,
    ) -> ProgramState {
        let parameter1_value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let parameter2_value = state.read_parameter(1, instruction.get_parameter_mode(1));
        let result = parameter1_value + parameter2_value;
        let output_position = state.read_parameter(2, &ParameterMode::Immediate);
        let intcode = state.intcode.write(output_position as usize, result);
        ProgramState {
            status: ProgramStatus::Running,
            intcode,
            current_position: state.current_position + 4,
        }
    }
}

struct MultiplyExecutor {}

impl InstructionExecutor for MultiplyExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: Pipe,
        _output: Option<Pipe>,
    ) -> ProgramState {
        let parameter1_value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let parameter2_value = state.read_parameter(1, instruction.get_parameter_mode(1));
        let result = parameter1_value * parameter2_value;
        let output_position = state.read_parameter(2, &ParameterMode::Immediate);
        let intcode = state.intcode.write(output_position as usize, result);
        ProgramState {
            status: ProgramStatus::Running,
            intcode,
            current_position: state.current_position + 4,
        }
    }
}

struct InputExecutor {}

impl InstructionExecutor for InputExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        _instruction: &Instruction,
        input: Pipe,
        _output: Option<Pipe>,
    ) -> ProgramState {
        match input.read() {
            Some(i) => {
                let output_position = state.read_parameter(0, &ParameterMode::Immediate);
                let intcode = state.intcode.write(output_position as usize, i);
                ProgramState {
                    status: ProgramStatus::Running,
                    intcode,
                    current_position: state.current_position + 2,
                }
            }
            _ => ProgramState {
                status: ProgramStatus::Waiting,
                intcode: state.intcode.clone(),
                current_position: state.current_position,
            },
        }
    }
}

struct OutputExecutor {}

impl InstructionExecutor for OutputExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: Pipe,
        output: Option<Pipe>,
    ) -> ProgramState {
        let value = state.read_parameter(0, instruction.get_parameter_mode(0));
        output.map(|s| s.write(value));
        ProgramState {
            status: ProgramStatus::Running,
            intcode: state.intcode.clone(),
            current_position: state.current_position + 2,
        }
    }
}

struct JumpIfTrueExecutor {}

impl InstructionExecutor for JumpIfTrueExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: Pipe,
        _output: Option<Pipe>,
    ) -> ProgramState {
        let value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let new_position = match value {
            0 => state.current_position + 3,
            _ => state.read_parameter(1, instruction.get_parameter_mode(1)) as usize,
        };
        ProgramState {
            status: ProgramStatus::Running,
            intcode: state.intcode.clone(),
            current_position: new_position,
        }
    }
}

struct JumpIfFalseExecutor {}

impl InstructionExecutor for JumpIfFalseExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: Pipe,
        _output: Option<Pipe>,
    ) -> ProgramState {
        let value = state.read_parameter(0, instruction.get_parameter_mode(0));
        let new_position = match value {
            0 => state.read_parameter(1, instruction.get_parameter_mode(1)) as usize,
            _ => state.current_position + 3,
        };
        ProgramState {
            status: ProgramStatus::Running,
            intcode: state.intcode.clone(),
            current_position: new_position,
        }
    }
}

struct LessThanExecutor {}

impl InstructionExecutor for LessThanExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: Pipe,
        _output: Option<Pipe>,
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
            status: ProgramStatus::Running,
            intcode,
            current_position: state.current_position + 4,
        }
    }
}

struct EqualsExecutor {}

impl InstructionExecutor for EqualsExecutor {
    fn execute(
        &self,
        state: &ProgramState,
        instruction: &Instruction,
        _input: Pipe,
        _output: Option<Pipe>,
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
            status: ProgramStatus::Running,
            intcode,
            current_position: state.current_position + 4,
        }
    }
}

#[cfg(test)]
mod pipe_should {
    use super::*;

    #[test]
    fn read_a_value_written_to_one_of_its_clones() {
        let pipe = Pipe::new();
        let clone = pipe.clone();

        clone.write(33);
        let result = pipe.read();

        assert_eq!(result, Some(33));
    }
}

#[cfg(test)]
mod intcode_tests {
    use super::*;

    #[test]
    fn execute_sample_code_with_add_and_multiply() {
        let mut program = Program::new(Intcode::from(vec![
            1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50,
        ]));

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(
            program.state.intcode,
            Intcode::from(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])
        );
    }

    #[test]
    fn sample1_should_return_1_when_input_is_8() {
        let mut program = Program::new(Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]));
        program.write(8);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(1));
    }

    #[test]
    fn sample1_should_return_0_when_input_is_not_8() {
        let mut program = Program::new(Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]));
        program.write(7);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(0));
    }

    #[test]
    fn sample2_should_return_1_when_input_is_less_than_8() {
        let mut program = Program::new(Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]));
        program.write(5);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(1));
    }

    #[test]
    fn sample2_should_return_0_when_input_is_8() {
        let mut program = Program::new(Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]));
        program.write(8);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(0));
    }

    #[test]
    fn sample3_should_return_1_when_input_is_8() {
        let mut program = Program::new(Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]));
        program.write(8);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(1));
    }

    #[test]
    fn sample3_should_return_0_when_input_is_not_8() {
        let mut program = Program::new(Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]));
        program.write(9);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(0));
    }

    #[test]
    fn sample4_should_return_1_when_input_is_less_than_8() {
        let mut program = Program::new(Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]));
        program.write(5);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(1));
    }

    #[test]
    fn sample4_should_return_0_when_input_is_8() {
        let mut program = Program::new(Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]));
        program.write(8);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(0));
    }

    #[test]
    fn sample5_should_return_0_when_input_is_0() {
        let mut program = Program::new(Intcode::from(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]));
        program.write(0);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(0));
    }

    #[test]
    fn sample5_should_return_1_when_input_is_not_0() {
        let mut program = Program::new(Intcode::from(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]));
        program.write(2);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(1));
    }

    #[test]
    fn sample6_should_return_0_when_input_is_0() {
        let mut program = Program::new(Intcode::from(vec![
            3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1,
        ]));
        program.write(0);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(0));
    }

    #[test]
    fn sample6_should_return_1_when_input_is_not_0() {
        let mut program = Program::new(Intcode::from(vec![
            3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1,
        ]));
        program.write(2);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(1));
    }

    #[test]
    fn sample7_should_return_999_when_input_is_less_than_8() {
        let mut program = Program::new(Intcode::from(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]));
        program.write(7);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(999));
    }

    #[test]
    fn sample7_should_return_1000_when_input_is_8() {
        let mut program = Program::new(Intcode::from(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]));
        program.write(8);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(1000));
    }

    #[test]
    fn sample7_should_return_1001_when_input_is_more_than_8() {
        let mut program = Program::new(Intcode::from(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]));
        program.write(9);
        let output = Pipe::new();
        program.set_output(&output);

        program.run();

        assert_eq!(program.state.status, ProgramStatus::Over);
        assert_eq!(output.read(), Some(1001));
    }

    #[test]
    fn it_should_wait_for_input_then_resume() {
        let mut program = Program::new(Intcode::from(vec![3, 3, 99, 0]));

        let result = program.run();

        assert_eq!(result, false);

        program.write(33);
        let result = program.run();

        assert_eq!(result, true);
        assert_eq!(program.state.intcode, Intcode::from(vec![3, 3, 99, 33]));
    }
}
