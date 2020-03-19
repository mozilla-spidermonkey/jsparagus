use std::collections::HashMap;

use crate::value::JSValue;

#[derive(Debug)]
pub struct Object {
    properties: HashMap<String, JSValue>,
}

impl Object {
    pub fn new() -> Self {
        Object {
            properties: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: JSValue) {
        self.properties.insert(name, value);
    }

    pub fn has(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    pub fn get(&self, name: String) -> JSValue {
        self.properties
            .get(&name)
            .unwrap_or(&JSValue::Undefined)
            .clone()
    }
}
