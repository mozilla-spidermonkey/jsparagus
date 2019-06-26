pub use crate::ast::Node;


const ACCEPT: i64 = -0x7fff_ffff_ffff_ffff;
const ERROR: i64 = ACCEPT - 1;

#[derive(Clone, Copy)]
struct Action(i64);

impl Action {
    fn is_shift(self) -> bool { 0 <= self.0 }

    fn shift_state(self) -> usize {
        assert!(self.is_shift());
        self.0 as usize
    }

    fn is_reduce(self) -> bool { ACCEPT < self.0 && self.0 < 0 }

    fn reduce_prod_index(self) -> usize {
        assert!(self.is_reduce());
        (-self.0 - 1) as usize
    }

    fn is_accept(self) -> bool { self.0 == ACCEPT }

    fn is_error(self) -> bool { self.0 == ERROR }
}


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

pub fn parse<In, Out>(
    mut tokens: In,
    start_state: usize,
    tables: &ParserTables,
    reduce: Out,
) -> Result<Node, &'static str>
where
    In: TokenStream<Token=crate::ast::Token>,
    Out: Fn(usize, &mut Vec<Node>) -> usize,
{
    assert_eq!(tables.action_table.len(), tables.state_count * tables.action_width);
    assert_eq!(tables.goto_table.len(), tables.state_count * tables.goto_width);

    let mut t = In::token_as_index(tokens.peek());
    let mut state_stack: Vec<usize> = vec![start_state];
    let mut node_stack: Vec<Node> = vec![];

    loop {
        let state = *state_stack.last().unwrap();
        let action = Action(tables.action_table[state * tables.action_width + t]);

        if action.is_shift() {
            node_stack.push(Node::Terminal(tokens.take()));
            state_stack.push(action.shift_state());
            t = In::token_as_index(tokens.peek());
        } else if action.is_reduce() {
            let prod_index = action.reduce_prod_index();
            let nt = reduce(prod_index, &mut node_stack);
            assert!(nt < tables.goto_width);
            state_stack.truncate(node_stack.len());
            let prev_state = *state_stack.last().unwrap();
            let state_after = tables.goto_table[prev_state * tables.goto_width + nt];
            assert!(state_after < tables.state_count);
            state_stack.push(state_after);
        } else if action.is_accept() {
            assert_eq!(state_stack.len(), 2);
            assert_eq!(node_stack.len(), 1);
            return Ok(node_stack.pop().unwrap());
        } else {
            assert!(action.is_error());
            return Err("syntax error lol");
        }
    }
}
