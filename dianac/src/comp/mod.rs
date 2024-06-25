mod raw_line;
use raw_line::RawLine;

mod lexer;
use lexer::{lex_line, tokens::Token};

mod error;
use error::CompilationError;

pub fn compile_to_binary(program: String) -> Result<String, Vec<CompilationError>> {
    let mut errors: Vec<CompilationError> = Vec::new();

    let lines: Vec<RawLine> = program
        .lines()
        .map(|s| s.to_string())
        .enumerate()
        .map(RawLine::from_tuple)
        .collect();

    let lexical_lines: Vec<Vec<Token>> = lines.iter().map(|l| lex_line(l)).collect();

    dbg!(&lexical_lines);
    dbg!(&errors);

    let binary = "".to_string();

    errors.is_empty().then_some(binary).ok_or(errors)
}
