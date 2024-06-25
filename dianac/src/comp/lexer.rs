use std::{iter::Peekable, str::FromStr};

use arbitrary_int::u6;
use strum::EnumString;

use crate::comp::error::{CompilationError, CompilationErrorKind};

#[derive(Debug)]
pub struct LexicalLine {
    pub tokens: Vec<Token>,
    pub raw_text: String,
    pub line_number: usize,
}

impl LexicalLine {
    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

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
}

fn lex_word(lexical_line: &mut LexicalLine, characters: &mut Peekable<impl Iterator<Item = char>>) {
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

    lexical_line.push(if let Ok(ident) = Identifier::from_str(&word) {
        Token::Identifier(ident)
    } else if let Ok(keyword) = Keyword::from_str(&word) {
        Token::Keyword(keyword)
    } else {
        Token::Lable(word)
    })
}

fn lex_constant(
    lexical_line: &mut LexicalLine,
    characters: &mut Peekable<impl Iterator<Item = char>>,
    radix: u32,
) -> Result<(), CompilationError> {
    // TODO show the number found in errors
    let mut as_u32: u32 = 0;
    let make_error = || {
        CompilationError::new(
            CompilationErrorKind::ConstantOverflow,
            lexical_line.line_number,
            lexical_line.raw_text.clone(),
            "use a value in the range 0..64".to_string(),
        )
    };

    while let Some(digit) = characters
        .peek()
        .map(|c| c.to_digit(radix))
        .unwrap_or_default()
    {
        characters.next();

        as_u32 = as_u32.checked_mul(radix).ok_or_else(make_error)?;
        as_u32 = as_u32.checked_add(digit).ok_or_else(make_error)?;
    }

    lexical_line.push(Token::Constant(
        u6::try_new(as_u32.try_into().map_err(|_| make_error())?).map_err(|_| make_error())?,
    ));

    Ok(())
}

fn lex_operator(
    lexical_line: &mut LexicalLine,
    characters: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<(), CompilationError> {
    let first = characters.next().unwrap();

    lexical_line.push(if characters.peek() == Some(&'=') {
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
            _ => {
                return Err(CompilationError::new(
                    CompilationErrorKind::InvalidOperator,
                    lexical_line.line_number,
                    lexical_line.raw_text.clone(),
                    format!("did you mean to use `{first}=`"),
                ))
            },
        }
    });

    Ok(())
}

fn lex_comment(lexical_line: &mut LexicalLine, characters: &mut impl Iterator<Item = char>) {
    lexical_line.push(Token::Comment(characters.collect()));
}

fn lex_separator(
    lexical_line: &mut LexicalLine,
    characters: &mut impl Iterator<Item = char>,
) -> Result<(), CompilationError> {
    lexical_line.push(Token::Separator(match characters.next().unwrap() {
        '(' => Separator::OpenParenthesis,
        ')' => Separator::CloseParenthesis,
        ':' => Separator::Colon,
        unexpected => {
            return Err(CompilationError::new(
                CompilationErrorKind::UnexpectedCharacter,
                lexical_line.line_number,
                lexical_line.raw_text.clone(),
                format!("found unexpeted character `{unexpected}`"),
            ))
        },
    }));

    Ok(())
}

pub fn lex_line(line: &str, linenr: usize) -> Result<LexicalLine, CompilationError> {
    let mut lexical_line = LexicalLine {
        tokens: Vec::new(),
        raw_text: line.to_string(),
        line_number: linenr,
    };
    let mut characters = line.chars().peekable();

    loop {
        match characters.peek() {
            Some(w) if w.is_whitespace() => {
                characters.next();
                continue;
            },
            Some('A'..='Z') => lex_word(&mut lexical_line, &mut characters),
            Some('%') => {
                characters.next();
                lex_constant(&mut lexical_line, &mut characters, 2)?
            },
            // why not 0x you may ask? who the fuck knows... (it was too much work)
            Some('$') => {
                characters.next();
                lex_constant(&mut lexical_line, &mut characters, 16)?
            },
            Some('0'..='9') => lex_constant(&mut lexical_line, &mut characters, 10)?,
            Some('<' | '>' | '!' | '=') => lex_operator(&mut lexical_line, &mut characters)?,
            Some('#') => lex_comment(&mut lexical_line, &mut characters),
            Some(_) => lex_separator(&mut lexical_line, &mut characters)?,
            None => break,
        };
    }

    Ok(lexical_line)
}
