use std::f64;

#[derive(Debug)]
pub enum JSValue {
    Boolean(bool),
    Number(f64),
    Undefined,
    Null,
}

pub fn to_number(v: JSValue) -> f64 {
    match v {
        JSValue::Boolean(true) => 1.0,
        JSValue::Boolean(false) => 0.0,
        JSValue::Number(n) => n,
        JSValue::Undefined => f64::NAN,
        JSValue::Null => 0.0,
    }
}
