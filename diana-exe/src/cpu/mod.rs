use crate::{
    error::Error,
    instructions::{Instruction, Operation, Register},
    utils::tuple_as_usize,
    Args,
};
use arbitrary_int::u6;

mod memory;
use memory::Memory;

mod program_counter;
use program_counter::ProgramCounter;

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

    fn get_register(&mut self, reg: Register) -> Result<u6, Error> {
        match reg {
            Register::A => Ok(self.a),
            Register::B => Ok(self.b),
            Register::C => Ok(self.c),
            Register::Immediate => {
                self.memory.pc.increment();
                self.memory.get(self.memory.pc.as_tuple())
            }
        }
    }

    fn set_register(&mut self, reg: Register, value: u6) -> Result<(), Error> {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::Immediate => Err(Error::AttemptToModifyImmediateValue)?,
        }

        Ok(())
    }

    pub fn cycle(&mut self) -> Result<(), Error> {
        let instruction =
            Instruction::new_with_raw_value(self.memory.get(self.memory.pc.as_tuple())?);
        let one = self.get_register(instruction.one())?;
        let two = self.get_register(instruction.two())?;

        match instruction.operation() {
            Operation::NOR => self.set_register(instruction.one(), !one & !two)?,
            Operation::PC => self.memory.pc.set((one, two)),
            Operation::LOAD => self.c = self.memory.get((one, two))?,
            Operation::STORE => self.memory.set((one, two), self.c)?,
        }
        self.memory.pc.increment();
        Ok(())
    }

    pub fn execute(&mut self, args: Args) -> Result<(), Error> {
        let mut limit = args.limit;
        loop {
            if let Some(mut value) = limit {
                if value == 0 {
                    break;
                }
                value -= 1;
                limit = Some(value);
            }

            self.cycle()?;

            if tuple_as_usize(self.memory.pc.as_tuple()) >= args.break_point.unwrap() {
                break;
            }
        }

        self.debug();

        Ok(())
    }

    pub fn debug(&self) {
        println!("{:#?}", self);
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
