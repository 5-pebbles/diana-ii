use arbitrary_int::u6;

pub enum Root {
    NOR(Mutable, Any),
    PC(Any, Any),
    LOAD(Any, Any),
    STORE(Any, Any),
}

pub enum Any {
    Mutable(Mutable),
    Constant(Constant),
}

pub enum Mutable {
    A,
    B,
    C,
}

pub enum Constant {
    Not(Box<Self>),
    Or(Box<Self>, Box<Self>),
    LableP0(String),
    LableP1(String),
    Number(u6),
}
