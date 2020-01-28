use std::f64;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum JSValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Undefined,
    Null,
}

pub fn to_number(v: &JSValue) -> f64 {
    match v {
        JSValue::Boolean(true) => 1.0,
        JSValue::Boolean(false) => 0.0,
        JSValue::Number(n) => *n,
        JSValue::String(ref s) => f64::from_str(s).unwrap_or(f64::NAN),
        JSValue::Undefined => f64::NAN,
        JSValue::Null => 0.0,
    }
}

pub fn to_boolean(v: &JSValue) -> bool {
    match v {
        JSValue::Null | JSValue::Undefined => false,
        JSValue::Boolean(b) => *b,
        JSValue::Number(n) => {
            if *n == 0.0 || n.is_nan() {
                false
            } else {
                true
            }
        }
        JSValue::String(ref s) => !s.is_empty()
    }
}
