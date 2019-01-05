use nom;
use crate::tokens::{Token, Lit};

macro_rules! parse_operator {
    ($name:tt, $tok:expr, $output:expr) => {
        named!($name (&[u8]) -> Token, 
            preceded!(nom::multispace0, value!($output, tag!($tok))));
    };
    ($name:tt, $tok:expr, $except:expr, $output:expr) => {
        named!($name (&[u8]) -> Token,
            delimited!(nom::multispace0, 
                       value!($output, tag!($tok)), 
                       peek!(none_of!($except))
            )
        );
    }
}

macro_rules! parse_keyword {
    ($name:tt, $tok:expr, $output:expr) => {
        named!($name (&[u8]) -> Token,
            delimited!(
                nom::multispace0,
                value!($output, tag!($tok)),
                peek!(not!(nom::alphanumeric1))
            )
        );
    };
}

// Assignment Operators
parse_operator!(parse_assign, "=", "=", Token::Equals);
parse_operator!(parse_plus_equals, "+=", Token::PlusEquals);
parse_operator!(parse_minus_equals, "-=", Token::MinusEquals);
parse_operator!(parse_divide_equals, "/=", Token::DivideEquals);
parse_operator!(parse_times_equals, "*=", Token::TimesEquals);

// Arithmatic Operators
parse_operator!(parse_plus, "+", "=", Token::Plus);
parse_operator!(parse_minus, "-", "=", Token::Minus);
parse_operator!(parse_divide, "/", "=", Token::Divide);
parse_operator!(parse_times, "*", "=", Token::Times);

// Logical Operators
parse_operator!(parse_less_than, "<", "=", Token::LessThan);
parse_operator!(parse_greater_than, ">", "=", Token::GreaterThan);
parse_operator!(parse_less_equal, "<=", Token::LessEqual);
parse_operator!(parse_greater_equal, ">=", Token::GreaterEqual);
parse_operator!(parse_not_equals, "!=", Token::NotEquals);
parse_operator!(parse_equals_equals, "==", Token::EqualsEquals);
parse_operator!(parse_and_and, "&&", Token::AndAnd);
parse_operator!(parse_or_or, "||", Token::OrOr);
parse_operator!(parse_not, "!", "=", Token::Not);

// Braces
parse_operator!(parse_open_paren, "(", Token::OpenParens);   
parse_operator!(parse_close_paren, ")", Token::CloseParens);
parse_operator!(parse_open_curly, "{", Token::OpenCurly);
parse_operator!(parse_close_curly, "}", Token::CloseCurly);
parse_operator!(parse_open_square, "[", Token::OpenSquare);
parse_operator!(parse_close_square, "]", Token::CloseSquare);

parse_operator!(parse_colon, ":", Token::Colon);
parse_operator!(parse_semicolon, ";", Token::SemiColon);

parse_keyword!(parse_function, "function", Token::Function);
parse_keyword!(parse_where, "where", Token::Where);
parse_keyword!(parse_let, "let", Token::Let);
parse_keyword!(parse_var, "var", Token::Var);
parse_keyword!(parse_import, "import", Token::Import);
parse_keyword!(parse_do, "do", Token::Do);
parse_keyword!(parse_for, "for", Token::For);
parse_keyword!(parse_while, "while", Token::While);
parse_keyword!(parse_if, "if", Token::If);

named!(parse_int (&[u8]) -> Option<i32>, 
    do_parse!(
        digits: take_while!(nom::is_digit) >>
        peek!(not!(nom::alphanumeric1)) >> 
        ( std::str::from_utf8(digits).ok().and_then(|d| d.parse().ok()) )
    )
);

named!(parse_float (&[u8]) -> Option<f32>, 
    do_parse!(
        digits: alt!(
            recognize!(
                tuple!(
                    take_while!(nom::is_digit),
                    char!('.'),
                    take_while!(nom::is_digit),
                    peek!(not!(nom::alphanumeric1))
                )
            )
            | recognize!(
                tuple!(
                    take_while!(nom::is_digit),
                    peek!(not!(nom::alphanumeric1))
                )
            )
        ) >>
        ( std::str::from_utf8(digits).ok().and_then(|d| d.parse().ok()) )
    )
);

named!(parse_digits (&[u8]) -> String ,
    do_parse!(
        digits: take_while!(nom::is_digit) >>
        peek!(not!(nom::alphanumeric1)) >> 
        ( String::from_utf8_lossy(digits).into() )
    )
);

named!(parse_string (&[u8]) -> Token,
    do_parse!(
        toks: delimited!(char!('"'),  take_until!("\""), char!('"')) >>
        ( Token::Literal(Lit::Str(String::from_utf8_lossy(toks).into())))
    )
);

named!(parse_named (&[u8]) -> Result<Token, std::string::FromUtf8Error>, 
    do_parse!( txt: take_while!(nom::is_alphanumeric) 
            >> (String::from_utf8(txt.to_vec()).map(|s| Token::Ident(s))) ));

named!(parse_literal (&[u8]) -> Token, alt!(
    parse_string_lit
    | map_res()
    | map_res!(parse_digits, |digits: String| {
        digits.parse::<isize>().map(|i| Token::Literal(Lit::Int(i)))
    })
));

pub fn parse(input: &str) -> Token {
    match parse_function(input.as_bytes()) {
        Ok((_, tok)) => tok,
        Err(_) => Token::Invalid,
    }
}