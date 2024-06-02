#[derive(Debug)]
pub enum Error {
    InvalidMemoryAddress,
    AttemptToModifyImmediateValue,
    AttemptToModifyROM,
    IoError(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(format!("IoError: {}", value))
    }
}
