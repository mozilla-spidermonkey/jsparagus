use serde_json::Result;

pub fn to_string(ast: &super::Program) -> Result<String> {
    serde_json::to_string(ast)
}

pub fn to_string_pretty(ast: &super::Program) -> Result<String> {
    serde_json::to_string_pretty(ast)
}

pub fn from_string(ast: &str) -> Result<super::Program> {
    serde_json::from_str(ast)
}
