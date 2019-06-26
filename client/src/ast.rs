use crate::parser_generated::TokenType;

#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub data: String,
}

impl Token {
    pub fn new(ty: TokenType) -> Token {
        Token {
            ty,
            data: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct NtNode {
    pub nt: &'static str,
    pub prod: usize,
    pub fields: Vec<Option<Node>>,
}

#[derive(Debug)]
pub enum Node {
    Terminal(Token),
    Nonterminal(Box<NtNode>),
}

impl Node {
    pub fn new(
        nt: &'static str,
        prod: usize,
        fields: Vec<Option<Node>>
    ) -> Node {
        Node::Nonterminal(Box::new(NtNode { nt, prod, fields }))
    }
}
