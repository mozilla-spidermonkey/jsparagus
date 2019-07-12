use crate::parser_generated::TerminalId;
use crate::parser_generated::NtNode;

// Danger: The order of these variants is chosen to match TerminalId, so that
// the .get_id() method is trivial.
#[derive(Debug)]
pub enum Token {
    Nt,  // 'nt' keyword
    Goal,  // 'goal' keyword
    Token,  // 'token' keyword
    Var,  // 'var' keyword
    Identifier(String),
    End,
    OpenBrace, // {
    EqualSign, // =
    Arrow, // =>
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
            Token::Token => TerminalId::Token,
            Token::Var => TerminalId::Var,
            Token::Identifier(_) => TerminalId::Identifier,
            Token::End => TerminalId::End,
            Token::OpenBrace => TerminalId::OpenBrace,
            Token::Arrow => TerminalId::Arrow,
            Token::EqualSign => TerminalId::EqualSign,
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
