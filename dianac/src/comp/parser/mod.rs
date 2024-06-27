use super::{error::CompilationError, lexer::tokens::Token, raw_line::RawLine};

pub mod tree;
use tree::Root;

pub fn parse_line(
    lexical_line: &Vec<Token>,
    raw_line: &RawLine,
) -> Result<Option<Root>, CompilationError> {
    Ok(None)
}
