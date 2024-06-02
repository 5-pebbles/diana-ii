use arbitrary_int::u6;
use clap::Parser;
use std::{fs, io, str};

mod error;
use error::Error;

mod cpu;
use cpu::Cpu;

mod instructions;
mod utils;

/// An emulator for the Diana CPU
#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// The path of a program to execute if not provided via stdin
    #[arg(short, long)]
    file: Option<String>,

    /// The maximum number of instructions to execute
    #[arg(short, long)]
    limit: Option<usize>,

    /// The address at which the emulator should exit; defaults to input.len()
    #[arg(short, long)]
    break_point: Option<usize>,
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

fn main() -> Result<(), Error> {
    let mut args = Args::parse();

    // read input
    let mut program = String::new();
    if let Some(path) = &args.file {
        program = fs::read_to_string(path)?;
    } else {
        io::stdin().read_line(&mut program)?;
    }

    // parse input
    let instructions: Vec<u6> = parse_program(program);

    // set default values
    args.break_point.get_or_insert(instructions.len());

    // execute
    let mut cpu = Cpu::new(instructions);
    cpu.execute(args)?;
    Ok(())
}
