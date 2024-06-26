use arbitrary_int::u6;

#[derive(Debug)]
pub enum RuntimeError {
    AttemptToModifyImmediateValue,
    AttemptToModifyROM(u6, u6),
    IoError(String),
}

impl From<std::io::Error> for RuntimeError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(format!("IoError: {}", value))
    }
}
