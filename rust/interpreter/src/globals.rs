use std::cell::RefCell;
use std::rc::Rc;

use crate::object::Object;
use crate::value::{to_number, JSValue};

fn print(_this_value: JSValue, args: &[JSValue]) -> JSValue {
    println!("{:?}", args);
    JSValue::Undefined
}

fn sqrt(_this_value: JSValue, args: &[JSValue]) -> JSValue {
    let n = to_number(args.first().unwrap_or(&JSValue::Undefined));
    JSValue::Number(n.sqrt())
}

pub fn create_global() -> Rc<RefCell<Object>> {
    let global = Rc::new(RefCell::new(Object::new()));
    global
        .borrow_mut()
        .set("print".to_owned(), JSValue::NativeFunction(print));
    global
        .borrow_mut()
        .set("undefined".to_owned(), JSValue::Undefined);

    let math = Rc::new(RefCell::new(Object::new()));
    math.borrow_mut()
        .set("sqrt".to_owned(), JSValue::NativeFunction(sqrt));

    global
        .borrow_mut()
        .set("Math".to_owned(), JSValue::Object(math));

    global
}
