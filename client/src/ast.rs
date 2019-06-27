use crate::parser_generated::TerminalId;
use crate::parser_generated::NtNode;

// Danger: The order of these variants is chosen to match TerminalId, so that
// the .get_id() method is trivial.
#[derive(Debug)]
pub enum Token {
    Nt,  // 'nt' keyword
    Goal,  // 'goal' keyword
    Identifier(String),
    End,
    OpenBrace, // {
    CloseBrace, // }
    String(String),
    Semicolon, // ;
    QuestionMark, // ?
}

impl Token {
    pub fn get_id(&self) -> TerminalId {
        // This switch should be optimized away.
        match self {
            Token::Nt => TerminalId::Nt,
            Token::Goal => TerminalId::Goal,
            Token::Identifier(_) => TerminalId::Identifier,
            Token::End => TerminalId::End,
            Token::OpenBrace => TerminalId::OpenBrace,
            Token::CloseBrace => TerminalId::CloseBrace,
            Token::String(_) => TerminalId::String,
            Token::Semicolon => TerminalId::Semicolon,
            Token::QuestionMark => TerminalId::QuestionMark,
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Terminal(Token),
    Nonterminal(Box<NtNode>),
}
