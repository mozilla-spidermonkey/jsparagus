use crate::parser::{Node, ParseError, Parser, Result};
pub use crate::parser_generated::{ErrorCode, Handler, NonterminalId, TerminalId, Token};

pub trait TokenStream {
    type Token;
    fn next<'a, Out, Reduce>(&mut self, parser: &Parser<'a, Out, Reduce>) -> Option<Self::Token>
    where
        Out: Handler,
        Reduce: Fn(&Out, usize, &mut Vec<Node>) -> NonterminalId;
    fn token_as_index(t: &Self::Token) -> usize;
}

#[derive(Clone, Copy)]
pub struct ParserTables<'a> {
    pub state_count: usize,
    pub action_table: &'a [i64],
    pub action_width: usize,
    pub error_codes: &'a [Option<ErrorCode>],
    pub reduce_simulator: &'a [(usize, NonterminalId)],
    pub goto_table: &'a [u16],
    pub goto_width: usize,
}

impl<'a> ParserTables<'a> {
    pub fn check(&self) {
        assert_eq!(
            self.action_table.len(),
            self.state_count * self.action_width
        );
        assert_eq!(self.goto_table.len(), self.state_count * self.goto_width);
    }
}

pub fn parse<H: Handler, In, Out>(
    handler: &H,
    mut tokens: In,
    start_state: usize,
    tables: &ParserTables,
    reduce: Out,
) -> Result<*mut ()>
where
    In: TokenStream<Token = Token>,
    Out: Fn(&H, usize, &mut Vec<*mut ()>) -> NonterminalId,
{
    tables.check();

    let mut parser = Parser::new(tables, reduce, handler, start_state);

    loop {
        if let Some(t) = tokens.next(&parser) {
            if t.terminal_id == TerminalId::End {
                break;
            }
            parser.write_token(&t)?;
        } else {
            return Err(ParseError::LexerError);
        }
    }
    parser.close()
}
