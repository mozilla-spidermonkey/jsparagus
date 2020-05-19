mod evaluate;
mod globals;
mod object;
mod value;

#[cfg(test)]
mod tests;

extern crate jsparagus_emitter as emitter;
extern crate jsparagus_stencil as stencil;

#[cfg(test)]
extern crate jsparagus_ast as ast;
#[cfg(test)]
extern crate jsparagus_parser as parser;

pub use evaluate::{evaluate, EvalError};
pub use globals::create_global;
pub use object::Object;
pub use value::JSValue;
