use crate::comp::raw_line::RawLine;

pub mod handlers;

pub mod tokens;
use tokens::Token;

pub fn lex_line(line: &RawLine) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let uppercase = line.raw_text.to_uppercase();
    let mut characters = uppercase.chars().peekable();

    while let Some(character) = characters.peek() {
        let handler = match character {
            w if w.is_whitespace() => {
                characters.next();
                continue;
            },
            '#' => handlers::lex_comment,
            '%' => handlers::lex_binary,
            '$' => handlers::lex_hex,
            '0'..='9' => handlers::lex_decimal,
            'A'..='Z' => handlers::lex_word,
            '<' | '>' | '!' | '=' => handlers::lex_operator,
            _ => handlers::lex_separator,
        };

        tokens.push(handler(&mut characters));
    }

    tokens
}
