use arbitrary_int::u6;

use crate::emul::{
    error::RuntimeError,
    instructions::{Instruction, Operation, Register},
    memory::Memory,
};

pub struct Cpu {
    a: u6,
    b: u6,
    c: u6,
    memory: Memory,
}

impl Cpu {
    pub fn new(program: Vec<u6>) -> Self {
        Self {
            a: u6::default(),
            b: u6::default(),
            c: u6::default(),
            memory: Memory::new(program),
        }
    }

    fn get_register(&mut self, reg: Register) -> Result<u6, RuntimeError> {
        match reg {
            Register::A => Ok(self.a),
            Register::B => Ok(self.b),
            Register::C => Ok(self.c),
            Register::Immediate => self.memory.get(self.memory.pc.as_tuple()).and_then(|v| {
                self.memory.pc.increment();
                Ok(v)
            }),
        }
    }

    fn set_register(&mut self, reg: Register, value: u6) -> Result<(), RuntimeError> {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::Immediate => Err(RuntimeError::AttemptToModifyImmediateValue)?,
        }

        Ok(())
    }

    pub fn cycle(&mut self) -> Result<(), RuntimeError> {
        let instruction =
            Instruction::new_with_raw_value(self.memory.get(self.memory.pc.as_tuple())?);
        self.memory.pc.increment();
        let one = self.get_register(instruction.one())?;
        let two = self.get_register(instruction.two())?;

        match instruction.operation() {
            Operation::NOR => self.set_register(instruction.one(), !one & !two)?,
            Operation::PC => self.memory.pc.set((one, two)),
            Operation::LOAD => self.c = self.memory.get((one, two))?,
            Operation::STORE => self.memory.set((one, two), self.c)?,
        }
        Ok(())
    }

    pub fn execute(&mut self) -> Result<String, RuntimeError> {
        for _ in 0..200 {
            self.cycle()?;
        }

        Ok(self.debug())
    }

    pub fn debug(&self) -> String {
        format!("{:#?}", self)
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPU {{ 
    a: {:0>6b},
    b: {:0>6b},
    c: {:0>6b},
    pc: {:?}
}}",
            self.a, self.b, self.c, self.memory.pc,
        )
    }
}
