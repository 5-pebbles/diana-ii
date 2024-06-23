use std::{iter::Peekable, str::FromStr};

use arbitrary_int::u6;
use strum::EnumString;

use crate::comp::Error;

#[derive(Debug)]
pub enum Token {
    Comment(String),
    Constant(u6),
    Identifier(Identifier),
    Keyword(Keyword),
    Lable(String),
    Operator(Operator),
    Separator(Separator),
}

#[derive(EnumString, Debug)]
pub enum Keyword {
    // Logic
    NOT,
    AND,
    NAND,
    OR,
    NOR,
    XOR,
    NXOR,
    // Arithmetic
    LWRAP,
    RWRAP,
    LSHIFT,
    RSHIFT,
    ADD,
    SATADD,
    SUB,
    SATSUB,
    MUL,
    SATMUL,
    DIV,
    SATDIV,
    // Memory
    SET,
    COPY,
    LOAD,
    STORE,
    // Jumps
    LABLE,
    PC,
    LIH,
}

#[derive(EnumString, Debug)]
pub enum Identifier {
    A,
    B,
    C,
}

#[derive(Debug)]
pub enum Operator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
}

#[derive(Debug)]
pub enum Separator {
    OpenParenthesis,
    CloseParenthesis,
    Colon,
    Unexpected(char),
}

fn lex_word(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
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

    if let Ok(ident) = Identifier::from_str(&word) {
        return Token::Identifier(ident);
    }

    if let Ok(keyword) = Keyword::from_str(&word) {
        return Token::Keyword(keyword);
    }

    Token::Lable(word)
}

fn lex_constant(characters: &mut Peekable<impl Iterator<Item = char>>, radix: u32) -> Token {
    let mut as_u32 = 0;

    while let Some(digit) = characters
        .peek()
        .map(|c| c.to_digit(radix))
        .unwrap_or_default()
    {
        as_u32 = as_u32 * radix + digit;
        characters.next();
    }

    Token::Constant(u6::new(as_u32 as u8))
}

fn lex_operator(characters: &mut Peekable<impl Iterator<Item = char>>) -> Token {
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

fn lex_comment(characters: &mut impl Iterator<Item = char>) -> Token {
    Token::Comment(characters.collect())
}

fn lex_separator(characters: &mut impl Iterator<Item = char>) -> Token {
    Token::Separator(match characters.next().unwrap() {
        '(' => Separator::OpenParenthesis,
        ')' => Separator::CloseParenthesis,
        ':' => Separator::Colon,
        unexpected => Separator::Unexpected(unexpected),
    })
}

pub fn lex_line(line: &str) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut characters = line.chars().peekable();

    loop {
        tokens.push(match characters.peek() {
            Some(w) if w.is_whitespace() => {
                characters.next();
                continue;
            }
            Some('A'..='Z') => lex_word(&mut characters),
            Some('%') => {
                characters.next();
                lex_constant(&mut characters, 2)
            }
            // why not 0x you may ask? who the fuck knows... (it was too much work)
            Some('$') => {
                characters.next();
                lex_constant(&mut characters, 16)
            }
            Some('0'..='9') => lex_constant(&mut characters, 10),
            Some('<' | '>' | '!' | '=') => lex_operator(&mut characters),
            Some('#') => lex_comment(&mut characters),
            Some(_) => lex_separator(&mut characters),
            None => break,
        });
    }

    Ok(tokens)
}
