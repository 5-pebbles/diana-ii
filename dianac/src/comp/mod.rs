mod lexer;
use lexer::{lex_line, LexicalLine};

mod error;
use error::CompilationError;

pub fn compile_to_binary(program: String) -> Result<String, Vec<CompilationError>> {
    let mut errors: Vec<CompilationError> = Vec::new();

    let lexical_lines: Vec<LexicalLine> = program
        .to_uppercase()
        .lines()
        .enumerate()
        .map(|(nr, line)| lex_line(line, nr))
        .flat_map(|r| {
            if r.is_err() {
                errors.push(r.unwrap_err());
                None
            } else {
                Some(r.unwrap())
            }
        })
        .collect();

    if errors.is_empty() {
        Ok("".to_string())
    } else {
        Err(errors)
    }
}
