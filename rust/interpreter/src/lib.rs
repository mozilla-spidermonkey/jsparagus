mod evaluate;
mod object;
mod value;

#[cfg(test)]
mod tests;

pub use evaluate::{evaluate, EvalError};
pub use value::JSValue;
