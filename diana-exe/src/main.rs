use arbitrary_int::u6;
use clap::Parser;
use std::str;

mod error;
use error::Error;

mod cpu;
use cpu::Cpu;

mod instructions;

/// An emulator for the Diana CPU
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(index = 1, required = true)]
    program: String,

    #[arg(short, long)]
    limit: Option<usize>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let instructions: Vec<u6> = args
        .program
        .as_bytes()
        .chunks(6)
        .map(|chunk| {
            u6::new(u8::from_str_radix(str::from_utf8(chunk).unwrap(), 2).expect("Expected Binary"))
        })
        .collect();
    let mut cpu = Cpu::new(instructions);
    cpu.execute(args.limit)?;
    println!("{:#?}", cpu);
    Ok(())
}
