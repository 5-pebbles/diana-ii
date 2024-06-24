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

fn main() {
    let args = Args::parse();

    // read input
    let mut program = String::new();
    io::stdin()
        .read_to_string(&mut program)
        .expect("Failed to read input");

    if let Some(Sub::Emulate) = args.command {
        println!("{:#?}", emulate_binary(program));
    } else {
        println!("{:#?}", compile_to_binary(program));
    }
}
