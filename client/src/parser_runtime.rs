pub use crate::parser_generated::{Handler, NonterminalId, Token};
use crate::parser::{Parser, Result};

pub trait TokenStream {
    type Token;
    fn peek(&mut self) -> &Self::Token;
    fn take(&mut self) -> Self::Token;
    fn token_as_index(t: &Self::Token) -> usize;
}

#[derive(Clone, Copy)]
pub struct ParserTables<'a> {
    pub state_count: usize,
    pub action_table: &'a [i64],
    pub action_width: usize,
    pub goto_table: &'a [usize],
    pub goto_width: usize,
}

impl<'a> ParserTables<'a> {
    pub fn check(&self) {
        assert_eq!(
            self.action_table.len(),
            self.state_count * self.action_width
        );
        assert_eq!(
            self.goto_table.len(),
            self.state_count * self.goto_width
        );
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

    let mut t = tokens.peek();
    while *t != Token::End {
        parser.write_token(tokens.take())?;
        t = tokens.peek();
    }
    parser.close()
}
