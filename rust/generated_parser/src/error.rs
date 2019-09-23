use crate::Token;
use std::{convert::Infallible, error::Error, fmt, io};

#[derive(Debug)]
pub enum ParseError<'alloc> {
    IOError(io::Error),

    // Lexical errors
    IllegalCharacter(char),
    InvalidEscapeSequence,
    UnterminatedString,
    UnterminatedRegExp,
    LexerError,

    // Generic syntax errors
    SyntaxError(Token<'alloc>),
    UnexpectedEnd,
    InvalidParameter,

    // Destructuring errors
    ArrayPatternWithNonFinalRest,
    ArrayPatternWithInvalidRestBinding,
    ObjectPatternWithMethod,
    ObjectPatternWithNonFinalRest,
    ObjectPatternWithInvalidRestBinding,
}

impl<'alloc> ParseError<'alloc> {
    pub fn message(&self) -> String {
        match self {
            ParseError::IOError(io_error) => format!("{}", io_error),
            ParseError::IllegalCharacter(c) => format!("illegal character: {:?}", c),
            ParseError::InvalidEscapeSequence => format!("invalid escape sequence"),
            ParseError::UnterminatedString => format!("unterminated string literal"),
            ParseError::UnterminatedRegExp => format!("unterminated regexp literal"),
            ParseError::LexerError => format!("lexical error"),
            ParseError::SyntaxError(token) => format!("syntax error on: {:?}", token),
            ParseError::UnexpectedEnd => format!("unexpected end of input"),
            ParseError::InvalidParameter => format!("invalid parameter"),
            ParseError::ArrayPatternWithNonFinalRest => {
                format!("array patterns can have a rest element (`...x`) only at the end")
            }
            ParseError::ArrayPatternWithInvalidRestBinding => format!(
                "the expression after `...` in this array pattern must be a single identifier"
            ),
            ParseError::ObjectPatternWithMethod => format!("object patterns can't have methods"),
            ParseError::ObjectPatternWithNonFinalRest => {
                format!("object patterns can have a rest element (`...x`) only at the end")
            }
            ParseError::ObjectPatternWithInvalidRestBinding => format!(
                "the expression after `...` in this object pattern must be a single identifier"
            ),
        }
    }
}

impl<'alloc> PartialEq for ParseError<'alloc> {
    fn eq(&self, other: &ParseError<'alloc>) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl<'alloc> fmt::Display for ParseError<'alloc> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<'alloc> From<io::Error> for ParseError<'alloc> {
    fn from(err: io::Error) -> ParseError<'alloc> {
        ParseError::IOError(err)
    }
}

impl<'alloc> From<Infallible> for ParseError<'alloc> {
    fn from(err: Infallible) -> ParseError<'alloc> {
        match err {}
    }
}

impl<'alloc> Error for ParseError<'alloc> {}

pub type Result<'alloc, T> = std::result::Result<T, ParseError<'alloc>>;
