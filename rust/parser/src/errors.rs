use generated_parser::Token;
use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum ParseError<'alloc> {
    IOError(io::Error),
    IllegalCharacter(char),
    InvalidEscapeSequence,
    UnterminatedString,
    UnterminatedRegExp,
    SyntaxError(Token<'alloc>),
    UnexpectedEnd,
    LexerError,
}

impl<'alloc> ParseError<'alloc> {
    pub fn message(&self) -> String {
        match self {
            ParseError::IOError(io_error) => format!("{}", io_error),
            ParseError::IllegalCharacter(c) => format!("illegal character: {:?}", c),
            ParseError::InvalidEscapeSequence => format!("invalid escape sequence"),
            ParseError::UnterminatedString => format!("unterminated string literal"),
            ParseError::UnterminatedRegExp => format!("unterminated regexp literal"),
            ParseError::SyntaxError(token) => format!("syntax error on: {:?}", token),
            ParseError::UnexpectedEnd => format!("unexpected end of input"),
            ParseError::LexerError => format!("lexical error"),
        }
    }
}

impl<'alloc> PartialEq for ParseError<'alloc> {
    fn eq(&self, other: &ParseError<'alloc>) -> bool {
        match (self, other) {
            (ParseError::IOError(e1), ParseError::IOError(e2)) => {
                format!("{:?}", e1) == format!("{:?}", e2)
            }
            (ParseError::IllegalCharacter(c1), ParseError::IllegalCharacter(c2)) => c1 == c2,
            (ParseError::InvalidEscapeSequence, ParseError::InvalidEscapeSequence) => true,
            (ParseError::UnterminatedString, ParseError::UnterminatedString) => true,
            (ParseError::UnterminatedRegExp, ParseError::UnterminatedRegExp) => true,
            (ParseError::SyntaxError(t1), ParseError::SyntaxError(t2)) => t1 == t2,
            (ParseError::UnexpectedEnd, ParseError::UnexpectedEnd) => true,
            (ParseError::LexerError, ParseError::LexerError) => true,
            _ => false,
        }
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

impl<'alloc> Error for ParseError<'alloc> {}

pub type Result<'alloc, T> = std::result::Result<T, ParseError<'alloc>>;
