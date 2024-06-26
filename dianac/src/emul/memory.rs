use arbitrary_int::u6;

use crate::{
    emul::{program_counter::ProgramCounter, RuntimeError},
    utils::tuple_as_usize,
};

const RAM_SIZE: usize = 3902;

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

    pub fn get(&self, index: (u6, u6)) -> Result<u6, RuntimeError> {
        let address: usize = tuple_as_usize(index);

        Ok(match address {
            0x000..=0xF3D => self.ram[address],
            0xF3E => self.pc.0,
            0xF3F => self.pc.1,
            0xF80..=0xFBF => index.1.wrapping_shl(1),
            0xFC0..=0xFFF => index.1.wrapping_shr(1),
            _ => unreachable!(),
        })
    }

    pub fn set(&mut self, index: (u6, u6), value: u6) -> Result<(), RuntimeError> {
        let address: usize = tuple_as_usize(index);

        match address {
            0x000..=0xF3D => self.ram[address] = value,
            0xF3E..=0xFFF => return Err(RuntimeError::AttemptToModifyROM(index.0, index.1)),
            _ => unreachable!(),
        }

        Ok(())
    }
}
