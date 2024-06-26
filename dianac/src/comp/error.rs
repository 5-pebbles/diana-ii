use colored::Colorize;
use std::fmt;
use strum::Display as EnumDisplay;

use super::raw_line::RawLine;

pub fn errors_to_string(errors: impl IntoIterator<Item = CompilationError>) -> String {
    errors.into_iter().fold(String::new(), |mut s, e| {
        s.push_str(&format!("\n{e}"));
        s
    })
}

#[derive(Debug)]
pub struct CompilationError {
    pub kind: CompilationErrorKind,
    pub line: RawLine,
    pub help: String,
}

impl CompilationError {
    pub fn new(kind: CompilationErrorKind, line: RawLine, help: String) -> Self {
        Self { kind, line, help }
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_number = self.line.line_number.to_string();

        let header = format!("{}: {}", "Error".red(), self.kind).bold();
        let prefix = format!(" {} |", " ".repeat(line_number.len()))
            .blue()
            .bold();
        let line_details = format!(
            "{}{}",
            format!(" {} | ", line_number).blue().bold(),
            self.line.raw_text
        );
        let help = format!("{} {}", format!("{}:", "help".cyan()).bold(), self.help);

        write!(f, "{header}\n{prefix}\n{line_details}\n{prefix}\n{help}")
    }
}

#[derive(Debug, EnumDisplay)]
#[strum(serialize_all = "snake_case")]
pub enum CompilationErrorKind {
    ConstantOverflow,
    UnexpectedCharacter,
}
