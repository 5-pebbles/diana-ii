#[derive(Debug)]
pub enum Error {
    InvalidMemoryAddress,
    AttemptToModifyImmediateValue,
    AttemptToModifyROM,
}
