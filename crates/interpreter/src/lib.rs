mod evaluate;
mod globals;
mod object;
mod value;

#[cfg(test)]
mod tests;

extern crate jsparagus_emitter as emitter;

#[cfg(test)]
extern crate jsparagus_ast as ast;
#[cfg(test)]
extern crate jsparagus_parser as parser;

pub use evaluate::{evaluate, EvalError};
pub use value::JSValue;
