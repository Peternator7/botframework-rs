
pub enum Lit {
    Int(isize),
    Bool(bool),
    Float(f32),
    Str(String),
}

pub enum Token {

    Plus,
    Minus,
    Times,
    Divide,

    Equals,
    PlusEquals,
    MinusEquals,
    TimesEquals,
    DivideEquals,

    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    NotEquals,
    EqualsEquals,
    AndAnd,
    OrOr,
    Not,

    OpenParens,
    CloseParens,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,

    Colon,
    SemiColon,
    Period,

    Function,
    Var,
    Let,
    Where,
    Import,
    If,
    For,
    While,
    Do,
    Ident(String),
    Literal(Lit),

    Invalid,
}