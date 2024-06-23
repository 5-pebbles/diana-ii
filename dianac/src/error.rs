use crate::comp;

#[derive(Debug)]
pub enum Error {
    InvalidMemoryAddress,
    AttemptToModifyImmediateValue,
    AttemptToModifyROM,
    IoError(String),
    Compilation(comp::Error),
}

impl From<comp::Error> for Error {
    fn from(value: comp::Error) -> Self {
        Self::Compilation(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(format!("IoError: {}", value))
    }
}
