mod lexer;
use lexer::{lex_line, Token};

mod error;
pub use error::Error;

pub fn compile_to_binary(program: String) -> Result<String, Error> {
    dbg!(program
        .to_uppercase()
        .lines()
        .map(|s| lex_line(s))
        .collect::<Result<Vec<_>, Error>>());

    Ok("".to_string())
}
