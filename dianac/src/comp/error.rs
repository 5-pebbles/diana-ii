use colored::Colorize;
use std::fmt;
use strum::Display as EnumDisplay;

#[derive(Debug)]
pub struct CompilationError {
    pub kind: CompilationErrorKind,
    pub line_number: usize,
    pub raw_text: String,
    pub help: String,
}

impl CompilationError {
    pub fn new(
        kind: CompilationErrorKind,
        line_number: usize,
        raw_text: String,
        help: String,
    ) -> Self {
        Self {
            kind,
            line_number,
            raw_text,
            help,
        }
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = format!("{}: {}", "Error".red(), self.kind).bold();

        let prefix = format!(" {} |", " ".repeat(self.line_number.to_string().len()))
            .blue()
            .bold();

        let line_details = format!(
            "{}{}",
            format!(" {} | ", self.line_number).blue().bold(),
            self.raw_text
        );

        let help = format!("{} {}", format!("{}:", "help".cyan()).bold(), self.help);

        write!(f, "{header}\n{prefix}\n{line_details}\n{prefix}\n{help}")
    }
}

#[derive(Debug, EnumDisplay)]
#[strum(serialize_all = "snake_case")]
pub enum CompilationErrorKind {
    ConstantOverflow,
    InvalidOperator,
    UnexpectedCharacter,
}
