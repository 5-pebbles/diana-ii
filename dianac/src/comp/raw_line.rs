#[derive(Debug, Clone)]
pub struct RawLine {
    pub line_number: usize,
    pub raw_text: String,
}

impl RawLine {
    pub fn from_tuple(tuple: (usize, String)) -> Self {
        Self {
            line_number: tuple.0,
            raw_text: tuple.1,
        }
    }
}
