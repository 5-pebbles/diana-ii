use arbitrary_int::u6;

use crate::{cpu::ProgramCounter, Error};

const RAM_SIZE: usize = 3902;

fn tuple_to_usize(tuple: (u6, u6)) -> usize {
    ((u16::from(tuple.0) << 6) | u16::from(tuple.1)) as usize
}

pub struct Memory {
    pub pc: ProgramCounter,
    ram: [u6; RAM_SIZE],
}

impl Memory {
    pub fn new(program: Vec<u6>) -> Self {
        let mut ram = [u6::default(); RAM_SIZE];
        ram[..program.len()].copy_from_slice(&program);

        Self {
            pc: ProgramCounter::default(),
            ram,
        }
    }

    pub fn get(&self, index: (u6, u6)) -> Result<u6, Error> {
        let address: usize = tuple_to_usize(index);

        Ok(match address {
            0x000..=0xF3D => self.ram[address],
            0xF3E => self.pc.0,
            0xF3F => self.pc.1,
            0xF80..=0xFBF => index.1.wrapping_shl(1),
            0xFC0..=0xFFF => index.1.wrapping_shr(1),
            _ => Err(Error::InvalidMemoryAddress)?,
        })
    }

    pub fn set(&mut self, index: (u6, u6), value: u6) -> Result<(), Error> {
        let address: usize = tuple_to_usize(index);

        match address {
            0x000..=0xF3D => self.ram[address] = value,
            0xF3E..=0xFFF => Err(Error::AttemptToModifyROM)?,
            _ => Err(Error::InvalidMemoryAddress)?,
        }

        Ok(())
    }
}
