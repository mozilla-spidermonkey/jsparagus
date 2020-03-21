use std::cell::RefCell;
use std::f64;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

use crate::object::Object;

#[derive(Clone)]
pub enum JSValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Object(Rc<RefCell<Object>>),
    NativeFunction(fn(JSValue, &[JSValue]) -> JSValue),
    Undefined,
    Null,
}

impl fmt::Debug for JSValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::String(ref s) => write!(f, "{:?}", s),
            Self::Object(ref o) => write!(f, "{:?}", o),
            Self::NativeFunction(ref n) => write!(f, "<native-function: {:p}>", n),
            Self::Undefined => write!(f, "undefined"),
            Self::Null => write!(f, "null"),
        }
    }
}

pub fn to_number(v: &JSValue) -> f64 {
    match v {
        JSValue::Boolean(true) => 1.0,
        JSValue::Boolean(false) => 0.0,
        JSValue::Number(n) => *n,
        JSValue::String(ref s) => f64::from_str(s).unwrap_or(f64::NAN),
        JSValue::Object(_) | JSValue::NativeFunction(_) => f64::NAN, // ToDo: valueOf
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
        JSValue::String(ref s) => !s.is_empty(),
        JSValue::Object(_) | JSValue::NativeFunction(_) => true,
    }
}

pub fn strict_equality(x: &JSValue, y: &JSValue) -> bool {
    match (x, y) {
        (JSValue::Undefined, JSValue::Undefined) => true,
        (JSValue::Null, JSValue::Null) => true,
        (JSValue::Boolean(a), JSValue::Boolean(b)) => a == b,
        (JSValue::Number(a), JSValue::Number(b)) => a == b,
        (JSValue::String(ref a), JSValue::String(ref b)) => a == b,
        (JSValue::Object(ref a), JSValue::Object(ref b)) => a.as_ptr() == b.as_ptr(),
        (JSValue::NativeFunction(a), JSValue::NativeFunction(b)) => std::ptr::eq(a, b),
        _ => false
    }
}
