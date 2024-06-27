mod raw_line;
use raw_line::RawLine;

mod lexer;
use lexer::{lex_line, tokens::Token};

mod parser;
use parser::{parse_line, tree::Root};

pub mod error;
use error::CompilationError;

pub fn compile_to_binary(program: String) -> Result<String, Vec<CompilationError>> {
    let mut errors: Vec<CompilationError> = Vec::new();

    let lines: Vec<RawLine> = program
        .lines()
        .map(|s| s.to_string())
        .enumerate()
        .map(RawLine::from_tuple)
        .collect();

    let lexical_lines: Vec<Vec<Token>> = lines.iter().map(lex_line).collect();

    let parse_trees: Vec<Option<Root>> = lexical_lines
        .iter()
        .zip(lines.iter())
        .map(|(lex_line, raw_line)| {
            parse_line(lex_line, raw_line).unwrap_or_else(|e| {
                errors.push(e);
                None
            })
        })
        .collect();

    // As you may have realized, I have no idea how a compiler works. (but I did read a few wikipedia pages on it)
    // I am just making stuff up as I go ¯\_(ツ)_/¯

    errors.push(CompilationError::new(
        error::CompilationErrorKind::ConstantOverflow,
        lines[0].clone(),
        "test123".to_string(),
    ));

    dbg!(&lexical_lines);
    dbg!(&errors);

    let binary = "".to_string();

    errors.is_empty().then_some(binary).ok_or(errors)
}
