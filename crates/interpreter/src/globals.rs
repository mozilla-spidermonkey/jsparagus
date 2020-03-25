use std::cell::RefCell;
use std::rc::Rc;

use crate::object::Object;
use crate::value::{strict_equality, to_number, JSValue};

fn print(_this_value: JSValue, args: &[JSValue]) -> JSValue {
    println!("{:?}", args);
    JSValue::Undefined
}

fn sqrt(_this_value: JSValue, args: &[JSValue]) -> JSValue {
    let n = to_number(args.first().unwrap_or(&JSValue::Undefined));
    JSValue::Number(n.sqrt())
}

fn assert_eq(_this_value: JSValue, args: &[JSValue]) -> JSValue {
    if args.len() != 2 {
        panic!("assertEq expects exactly two arguments");
    }

    if !strict_equality(&args[0], &args[1]) {
        panic!("{:?} is not equal to {:?}", args[0], args[1]);
    }

    JSValue::Undefined
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
        .borrow_mut()
        .set("assertEq".to_owned(), JSValue::NativeFunction(assert_eq));

    global
}
