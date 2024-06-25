use std::{iter::Peekable, str::FromStr};

use crate::comp::lexer::tokens::{Constant, Token};

use super::tokens::{Identifier, Keyword, Operator, Separator};

// TODO: we can use a type aliase after https://github.com/rust-lang/rust/issues/63063

pub fn lex_comment(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
    Token::Comment(characters.collect())
}

fn lex_constant(characters: &mut Peekable<impl Iterator<Item = char>>, radix: u32) -> String {
    let mut number = String::new();

    while characters
        .peek()
        .map(|c| c.to_digit(radix).is_some())
        .unwrap_or_default()
    {
        number.push(characters.next().unwrap());
    }

    number
}

pub fn lex_binary(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
    characters.next();
    Token::Constant(Constant::Binary(lex_constant(characters, 2)))
}

pub fn lex_hex(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
    characters.next();
    Token::Constant(Constant::Hex(lex_constant(characters, 16)))
}

pub fn lex_decimal(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
    Token::Constant(Constant::Decimal(lex_constant(characters, 10)))
}

pub fn lex_word(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
    let mut word = String::new();

    loop {
        if !characters
            .peek()
            .map(|c| c.is_ascii_uppercase() || *c == '_')
            .unwrap_or_default()
        {
            break;
        }
        word.push(characters.next().unwrap());
    }

    if let Ok(identifier) = Identifier::from_str(&word) {
        return Token::Identifier(identifier);
    }

    if let Ok(keyword) = Keyword::from_str(&word) {
        return Token::Keyword(keyword);
    }

    Token::Lable(word)
}

pub fn lex_operator(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
    let first = characters.next().unwrap();

    if characters.peek() == Some(&'=') {
        characters.next();
        Token::Operator(match first {
            '>' => Operator::GreaterOrEqual,
            '<' => Operator::LessOrEqual,
            '!' => Operator::NotEqual,
            '=' => Operator::Equal,
            _ => unreachable!(),
        })
    } else {
        match first {
            '>' => Token::Operator(Operator::Greater),
            '<' => Token::Operator(Operator::Less),
            _ => Token::Separator(Separator::Unexpected(first)),
        }
    }
}

pub fn lex_separator(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
    Token::Separator(match characters.next().unwrap() {
        '(' => Separator::OpenParenthesis,
        ')' => Separator::CloseParenthesis,
        ':' => Separator::Colon,
        unexpected => Separator::Unexpected(unexpected),
    })
}
