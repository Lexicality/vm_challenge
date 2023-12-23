use crate::value::{Value, ValueState};

#[derive(Debug)]
pub enum Opcode {
    Halt,
    Out,
    Noop,
}

impl Opcode {
    fn num_args(&self) -> usize {
        match self {
            Self::Halt | Self::Noop => 0,
            Self::Out => 1,
        }
    }
}

impl TryFrom<Value> for Opcode {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let value = value.get_value();
        if let ValueState::Number(n) = value {
            match n {
                0 => Ok(Self::Halt),
                19 => Ok(Self::Out),
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

pub struct VM {
    memory: Vec<Value>,
    stack: Vec<Value>,
    registers: [Value; 8],
    pc: usize,
    input: Vec<Value>,
}

impl VM {
    pub fn new(memory: Vec<u16>) -> Self {
        Self {
            memory: memory.into_iter().map(Value::mew).collect(),
            stack: Vec::new(),
            registers: [Value::mew(0); 8],
            pc: 0,
            input: Vec::new(),
        }
    }

    fn get_instruction(&self) -> Result<Opcode, String> {
        self.memory[self.pc].try_into()
    }

    fn get_value(&self, offset: usize) -> Value {
        let v = self.memory[self.pc + offset];
        match v.get_value() {
            ValueState::Register(i) => self.registers[i as usize],
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
                    Opcode::Out => {
                        print!("{}", self.get_value(1).to_ascii());
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
