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
    // Specification: https://tc39.es/ecma262/#sec-strict-equality-comparison
    match (x, y) {
        (JSValue::Undefined, JSValue::Undefined) => true,
        (JSValue::Null, JSValue::Null) => true,
        (JSValue::Boolean(a), JSValue::Boolean(b)) => a == b,
        (JSValue::Number(a), JSValue::Number(b)) => a == b,
        (JSValue::String(ref a), JSValue::String(ref b)) => a == b,
        (JSValue::Object(ref a), JSValue::Object(ref b)) => a.as_ptr() == b.as_ptr(),
        (JSValue::NativeFunction(a), JSValue::NativeFunction(b)) => std::ptr::eq(a, b),
        _ => false,
    }
}

pub fn equality(x: &JSValue, y: &JSValue) -> bool {
    // Specification: https://tc39.es/ecma262/#sec-abstract-equality-comparison
    match (x, y) {
        // 1. If Type(x) is the same as Type(y), then
        //     Return the result of performing Strict Equality Comparison x === y.
        (JSValue::Undefined, JSValue::Undefined)
        | (JSValue::Null, JSValue::Null)
        | (JSValue::Boolean(_), JSValue::Boolean(_))
        | (JSValue::Number(_), JSValue::Number(_))
        | (JSValue::String(_), JSValue::String(_))
        | (JSValue::Object(_), JSValue::Object(_))
        | (JSValue::NativeFunction(_), JSValue::NativeFunction(_)) => strict_equality(x, y),

        // 2. If x is null and y is undefined, return true.
        (JSValue::Undefined, JSValue::Null) => true,
        // 3. If x is undefined and y is null, return true.
        (JSValue::Null, JSValue::Undefined) => true,
        // 4. If Type(x) is Number and Type(y) is String, return the result of the comparison x == ! ToNumber(y).
        (JSValue::Number(a), JSValue::String(_)) => a == &to_number(y),
        // 5. If Type(x) is String and Type(y) is Number, return the result of the comparison ! ToNumber(x) == y.
        (JSValue::String(_), JSValue::Number(b)) => &to_number(x) == b,
        // TODO: 6. If Type(x) is BigInt and Type(y) is String, then
        //           Let n be ! StringToBigInt(y).
        //           If n is NaN, return false.
        //           Return the result of the comparison x == n.
        // TODO: 7. If Type(x) is String and Type(y) is BigInt, return the result of the comparison
        //       y == x.
        // 8. If Type(x) is Boolean, return the result of the comparison ! ToNumber(x) == y.
        (JSValue::Boolean(_), _) => equality(&JSValue::Number(to_number(x)), y),
        // 9. If Type(y) is Boolean, return the result of the comparison x == ! ToNumber(y).
        (_, JSValue::Boolean(_)) => equality(x, &JSValue::Number(to_number(y))),
        // TODO: 10. If Type(x) is either String, Number, BigInt, or Symbol and Type(y) is Object, return the
        //       result of the comparison x == ToPrimitive(y).
        // TODO: 11. If Type(x) is Object and Type(y) is either String, Number, BigInt, or Symbol,
        //       return the result of the comparison ToPrimitive(x) == y.
        // TODO: 12. If Type(x) is BigInt and Type(y) is Number, or if Type(x) is Number and
        //       Type(y) is BigInt, then
        //            If x or y are any of NaN, +∞, or -∞, return false.
        //            If the mathematical value of x is equal to the mathematical value of y,
        //                 return true; otherwise return false.
        // 13. Return false.
        (_, _) => false,
    }
}
