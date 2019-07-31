use generated_parser::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    IllegalCharacter(char),
    UnterminatedString,
    UnterminatedRegExp,
    SyntaxError(Token),
    UnexpectedEnd,
    LexerError,
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
            ParseError::IllegalCharacter(c) => format!("illegal character: {:?}", c),
            ParseError::UnterminatedString => format!("unterminated string literal"),
            ParseError::UnterminatedRegExp => format!("unterminated regexp literal"),
            ParseError::SyntaxError(token) => format!("syntax error on: {:?}", token),
            ParseError::UnexpectedEnd => format!("unexpected end of input"),
            ParseError::LexerError => format!("lexical error"),
        }
    }
}


pub type Result<T> = std::result::Result<T, ParseError>;
