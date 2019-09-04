use generated_parser::Token;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    IllegalCharacter(char),
    InvalidEscapeSequence,
    UnterminatedString,
    UnterminatedRegExp,
    SyntaxError(Token<'static>),
    UnexpectedEnd,
    LexerError,
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
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

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

pub type Result<T> = std::result::Result<T, ParseError>;
