use clap::{Parser, Subcommand};
use std::{
    io::{self, Read},
    str,
};

mod utils;

mod emul;
use emul::emulate_binary;

mod comp;
use comp::compile_to_binary;

/// An emulator and compiler for the Diana CPU
///
/// The compile and run a program use `<input> | dianac`
#[derive(Parser)]
#[command(version, about, propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    sub_command: Option<Sub>,
}

#[derive(Subcommand)]
pub enum Sub {
    /// Emulate the execution of a binary
    Emulate,
    /// Compile a binary without running it
    Compile,
}

fn main() {
    let args = Args::parse();

    // read input
    let mut program = String::new();
    io::stdin()
        .read_to_string(&mut program)
        .expect("Failed to read input");

    match args.sub_command {
        Some(Sub::Emulate) => println!("{:#?}", emulate_binary(program)),
        Some(Sub::Compile) => println!(
            "{}",
            compile_to_binary(program).unwrap_or_else(|v| v
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n\n"))
        ),
        None => emulate_binary(compile_to_binary(program).unwrap()).unwrap(),
    }
}
