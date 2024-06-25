use strum::EnumString;

#[derive(Debug)]
pub enum Token {
    Comment(String),
    Constant(Constant),
    Identifier(Identifier),
    Keyword(Keyword),
    Lable(String),
    Operator(Operator),
    Separator(Separator),
}

#[derive(Debug)]
pub enum Constant {
    Binary(String),
    Decimal(String),
    Hex(String),
}

#[derive(EnumString, Debug)]
pub enum Identifier {
    A,
    B,
    C,
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

#[derive(Debug)]
pub enum Operator {
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    NotEqual,
}

#[derive(Debug)]
pub enum Separator {
    CloseParenthesis,
    Colon,
    OpenParenthesis,
    /// results in a compilation error
    Unexpected(char),
}
