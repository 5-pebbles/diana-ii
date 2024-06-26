use arbitrary_int::u6;

mod instructions;
mod memory;
mod program_counter;

mod cpu;
use cpu::Cpu;

mod error;
use error::RuntimeError;

pub fn emulate_binary(program: String) -> Result<String, RuntimeError> {
    let mut bytes = program.into_bytes();
    bytes.retain(|b| b == &b'1' || b == &b'0');

    // parse input
    let instructions = bytes
        .chunks(6)
        .map(|chunk| {
            u6::new(
                u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 2)
                    .expect("Expected Binary"),
            )
        })
        .collect();

    // execute
    let mut cpu = Cpu::new(instructions);
    cpu.execute()
}
