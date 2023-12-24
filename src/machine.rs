use std::{
    collections::VecDeque,
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use text_io::read;

use crate::value::{Value, ValueState};

#[derive(Debug)]
pub enum Opcode {
    Halt,
    Set,
    Push,
    Pop,
    Eq,
    Gt,
    Jmp,
    Jt,
    Jf,
    Add,
    Mult,
    Mod,
    And,
    Or,
    Not,
    Rmem,
    Wmem,
    Call,
    Ret,
    Out,
    In,
    Noop,
}

impl Opcode {
    fn num_args(&self) -> usize {
        match self {
            Self::Halt | Self::Noop => 0,
            Self::Push | Self::Pop | Self::Jmp | Self::Call | Self::Ret | Self::Out | Self::In => 1,
            Self::Set | Self::Jt | Self::Jf | Self::Not | Self::Rmem | Self::Wmem => 2,
            Self::Eq | Self::Gt | Self::Add | Self::Mult | Self::Mod | Self::And | Self::Or => 3,
        }
    }
}

impl TryFrom<Value> for Opcode {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let value = value.get_value_state();
        if let ValueState::Number(n) = value {
            match n {
                0 => Ok(Self::Halt),
                1 => Ok(Self::Set),
                2 => Ok(Self::Push),
                3 => Ok(Self::Pop),
                4 => Ok(Self::Eq),
                5 => Ok(Self::Gt),
                6 => Ok(Self::Jmp),
                7 => Ok(Self::Jt),
                8 => Ok(Self::Jf),
                9 => Ok(Self::Add),
                10 => Ok(Self::Mult),
                11 => Ok(Self::Mod),
                12 => Ok(Self::And),
                13 => Ok(Self::Or),
                14 => Ok(Self::Not),
                15 => Ok(Self::Rmem),
                16 => Ok(Self::Wmem),
                17 => Ok(Self::Call),
                18 => Ok(Self::Ret),
                19 => Ok(Self::Out),
                20 => Ok(Self::In),
                21 => Ok(Self::Noop),
                _ => Err(format!("Unknown opcode {n}")),
            }
        } else {
            Err(format!("Unexpected value: {value:?}"))
        }
    }
}

pub enum ExecutionState {
    Running,
    Complete,
}

#[derive(Serialize, Deserialize)]
pub struct VM {
    memory: Vec<Value>,
    stack: Vec<Value>,
    registers: [Value; 8],
    pc: usize,
    input: VecDeque<Value>,
}

impl VM {
    pub fn new(memory: Vec<u16>) -> Self {
        Self {
            memory: memory.into_iter().map(Value::mew).collect(),
            stack: Vec::new(),
            registers: [Value::mew(0); 8],
            pc: 0,
            input: VecDeque::new(),
        }
    }

    fn get_instruction(&self) -> Result<Opcode, String> {
        self.memory[self.pc].try_into()
    }

    fn get_memory(&self, offset: usize) -> Value {
        self.memory[self.pc + offset]
    }

    fn set_memory(&mut self, target: Value, value: Value) {
        match target.get_value_state() {
            ValueState::Number(n) => self.memory[n as usize] = value,
            ValueState::Register(r) => self.registers[r] = value,
            ValueState::Invalid => panic!("Attempt to write to invalid memory address {target}"),
        }
    }

    fn get_value(&self, offset: usize) -> Value {
        let v = self.get_memory(offset);
        match v.get_value_state() {
            ValueState::Register(i) => self.registers[i],
            // Just gonna return invalid values because why not
            _ => v,
        }
    }

    pub fn step(&mut self) -> ExecutionState {
        let opcode = self.get_instruction();
        match opcode {
            Ok(opcode) => {
                match opcode {
                    Opcode::Halt => return ExecutionState::Complete,
                    Opcode::Set => {
                        let target = self.get_memory(1).to_register();
                        let value = self.get_value(2);
                        self.registers[target] = value;
                    }
                    Opcode::Push => {
                        let value = self.get_value(1);
                        self.stack.push(value);
                    }
                    Opcode::Pop => {
                        let value = self.stack.pop().expect("Cannot pop an empty stack");
                        let target = self.get_memory(1);
                        self.set_memory(target, value);
                    }
                    Opcode::Eq => {
                        let target = self.get_memory(1);
                        let a = self.get_value(2);
                        let b = self.get_value(3);
                        let value = if a == b { 1 } else { 0 };
                        self.set_memory(target, Value::mew(value));
                    }
                    Opcode::Gt => {
                        let target = self.get_memory(1);
                        let a = self.get_value(2);
                        let b = self.get_value(3);
                        let value = if a > b { 1 } else { 0 };
                        self.set_memory(target, Value::mew(value));
                    }
                    Opcode::Jmp => {
                        self.pc = self.get_value(1).to_number() as usize;
                        // Avoid updating the pc
                        return ExecutionState::Running;
                    }
                    Opcode::Jt => {
                        let value = self.get_value(1).to_number();
                        if value != 0 {
                            self.pc = self.get_value(2).to_number() as usize;
                            return ExecutionState::Running;
                        }
                    }
                    Opcode::Jf => {
                        let value = self.get_value(1).to_number();
                        if value == 0 {
                            self.pc = self.get_value(2).to_number() as usize;
                            return ExecutionState::Running;
                        }
                    }
                    Opcode::Add => {
                        let target = self.get_memory(1);
                        let a = self.get_value(2);
                        let b = self.get_value(3);
                        self.set_memory(target, a + b);
                    }
                    Opcode::Mult => {
                        let target = self.get_memory(1);
                        let a = self.get_value(2);
                        let b = self.get_value(3);
                        self.set_memory(target, a * b);
                    }
                    Opcode::Mod => {
                        let target = self.get_memory(1);
                        let a = self.get_value(2);
                        let b = self.get_value(3);
                        self.set_memory(target, a % b);
                    }
                    Opcode::And => {
                        let target = self.get_memory(1);
                        let a = self.get_value(2);
                        let b = self.get_value(3);
                        self.set_memory(target, a & b);
                    }
                    Opcode::Or => {
                        let target = self.get_memory(1);
                        let a = self.get_value(2);
                        let b = self.get_value(3);
                        self.set_memory(target, a | b);
                    }
                    Opcode::Not => {
                        let target = self.get_memory(1);
                        let a = self.get_value(2);
                        self.set_memory(target, !a);
                    }
                    Opcode::Rmem => {
                        let target = self.get_memory(1);
                        let location = self.get_value(2).to_number() as usize;
                        let value = self.memory[location];
                        self.set_memory(target, value);
                    }
                    Opcode::Wmem => {
                        let location = self.get_value(1).to_number() as usize;
                        let value = self.get_value(2);
                        self.memory[location] = value;
                    }
                    Opcode::Call => {
                        let a = self.get_value(1);
                        self.stack.push(Value::mew((self.pc + 2) as u16));
                        self.pc = a.to_number() as usize;
                        return ExecutionState::Running;
                    }
                    Opcode::Ret => {
                        if let Some(value) = self.stack.pop() {
                            self.pc = value.to_number() as usize;
                            return ExecutionState::Running;
                        } else {
                            return ExecutionState::Complete;
                        }
                    }
                    Opcode::Out => {
                        print!("{}", self.get_value(1).to_ascii());
                    }
                    Opcode::In => {
                        if self.input.is_empty() {
                            print!("> ");
                            let mut line: String = read!("{}\n");
                            match line.as_str() {
                                "save" => {
                                    let vm = ron::to_string(self).unwrap();
                                    File::options()
                                        .create(true)
                                        .truncate(true)
                                        .write(true)
                                        .open("vm.ron")
                                        .unwrap()
                                        .write_all(&vm.into_bytes())
                                        .unwrap();
                                    println!("=== State Saved ===");
                                    return ExecutionState::Running;
                                }
                                "load" => {
                                    let mut raw_data = String::new();
                                    File::open("vm.ron")
                                        .expect("Save file doesn't exist!")
                                        .read_to_string(&mut raw_data)
                                        .unwrap();
                                    *self = ron::from_str(&raw_data).unwrap();
                                    println!("=== State Loaded ===");
                                    line = "look".to_owned();
                                }
                                line if !line.is_ascii() => {
                                    println!("Cannot use non-ascii input!");
                                    return ExecutionState::Running;
                                }
                                _ => (),
                            }
                            self.input
                                .extend(line.bytes().map(|b| Value::mew(b as u16)));
                            const MEWLINE: Value = Value::mew(('\n' as u32) as u16);
                            self.input.push_back(MEWLINE);
                        }
                        let value = self.input.pop_front().unwrap();
                        let target = self.get_memory(1);
                        self.set_memory(target, value);
                    }
                    Opcode::Noop => (),
                }
                self.pc += opcode.num_args() + 1;
            }
            Err(msg) => {
                eprintln!("Error at {}: {}", self.pc, msg);
                self.pc += 1;
            }
        }
        ExecutionState::Running
    }

    pub fn run(&mut self) {
        while let ExecutionState::Running = self.step() {}
    }
}
