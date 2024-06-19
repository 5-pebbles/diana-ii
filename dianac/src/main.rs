use arbitrary_int::u6;
use clap::{Parser, Subcommand};
use std::{
    io::{self, Read},
    str,
};

type Result<T> = std::result::Result<T, Error>;

mod error;
use error::Error;

mod cpu;
use cpu::Cpu;

mod instructions;
mod utils;

/// An emulator and compiler for the Diana CPU
///
/// The compile and run a program use `<input> | dianac | dianac emulate`
#[derive(Parser)]
#[command(version, about, propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Sub>,
}

#[derive(Subcommand)]
pub enum Sub {
    /// Emulate running a binary program
    Emulate,
}

fn parse_program(program: String) -> Vec<u6> {
    let mut bytes = program.into_bytes();
    bytes.retain(|c| c == &b'1' || c == &b'0');

    bytes
        .chunks(6)
        .map(|chunk| {
            u6::new(u8::from_str_radix(str::from_utf8(chunk).unwrap(), 2).expect("Expected Binary"))
        })
        .collect()
}

fn emulate(args: Args) -> Result<()> {
    // read input
    let mut program = String::new();
    io::stdin().read_to_string(&mut program)?;

    // parse input
    let instructions: Vec<u6> = parse_program(program);

    // execute
    let mut cpu = Cpu::new(instructions);
    cpu.execute(args)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(Sub::Emulate) = args.command {
        emulate(args)?;
    } else {
        unimplemented!();
    }

    Ok(())
}
