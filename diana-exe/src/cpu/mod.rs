use crate::{
    error::Error,
    instructions::{Instruction, Operation, Register},
};
use arbitrary_int::{u6, Number};

pub struct Cpu {
    a: u6,
    b: u6,
    c: u6,
    pc: (u6, u6),
    memory: [u6; 4096],
}

impl Cpu {
    pub fn new(program: Vec<u6>) -> Self {
        let mut memory = [u6::default(); 4096];
        memory[..program.len()].copy_from_slice(&program);

        Self {
            a: u6::default(),
            b: u6::default(),
            c: u6::default(),
            pc: (u6::default(), u6::default()),
            memory,
        }
    }

    fn increment(&mut self) {
        if self.pc.1 < u6::MAX {
            self.pc.1 += u6::new(1);
        } else {
            self.pc.0 += u6::new(1);
            self.pc.1 = u6::default();
        }
    }

    fn load_register(&mut self, reg: Register) -> Result<u6, Error> {
        match reg {
            Register::A => Ok(self.a),
            Register::B => Ok(self.b),
            Register::C => Ok(self.c),
            Register::Immediate => {
                self.increment();
                self.load(self.pc)
            }
        }
    }

    fn store_register(&mut self, reg: Register, value: u6) -> Result<(), Error> {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::Immediate => Err(Error::AttemptToModifyImmediateValue)?,
        }

        Ok(())
    }

    fn load(&self, address: (u6, u6)) -> Result<u6, Error> {
        let address_16: u16 = u16::from(address.0) * u16::from(address.1);
        self.memory
            .get(address_16 as usize)
            .copied()
            .ok_or(Error::InvalidMemoryAddress)
    }

    fn store(&mut self, address: (u6, u6)) -> Result<(), Error> {
        let address_16: u16 = u16::from(address.0) * u16::from(address.1);
        self.memory
            .get_mut(address_16 as usize)
            .map(|value| *value = self.c)
            .ok_or(Error::InvalidMemoryAddress)
    }

    pub fn cycle(&mut self) -> Result<(), Error> {
        let instruction = Instruction::new_with_raw_value(self.load(self.pc)?);
        let one = self.load_register(instruction.one())?;
        let two = self.load_register(instruction.two())?;

        match instruction.operation() {
            Operation::NOR => self.store_register(instruction.one(), !one & !two)?,
            Operation::PC => self.pc = (one, two),
            Operation::LOAD => self.c = self.load((one, two))?,
            Operation::STORE => self.store((one, two))?,
        }
        self.increment();
        Ok(())
    }

    pub fn execute(&mut self, mut limit: Option<usize>) -> Result<(), Error> {
        loop {
            if let Some(value) = limit {
                if value <= 0 {
                    break;
                }

                limit = limit.map(|v| v - 1);
            }
            self.cycle()?;
        }

        Ok(())
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
    pc: ({:0>6b}, {:0>6b})
}}",
            self.a, self.b, self.c, self.pc.0, self.pc.1
        )
    }
}
