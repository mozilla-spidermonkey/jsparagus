mod opcode;

use ast::*;
use opcode::*;

pub fn emit(ast: &Program) -> Vec<u8> {
    let mut emitter = Emitter::new();
    emitter.emit_program(ast);
    emitter.bytecode
}

struct Emitter {
    bytecode: Vec<u8>,
}

impl Emitter {
    fn new() -> Self {
        Self {
            bytecode: Vec::new(),
        }
    }

    fn emit1(&mut self, opcode: &Opcode) {
        self.bytecode.push(opcode.value);
    }

    fn emit_int8(&mut self, opcode: &Opcode, value: i8) {
        self.bytecode.push(opcode.value);
        self.bytecode.extend_from_slice(&value.to_ne_bytes());
    }

    fn emit_int32(&mut self, opcode: &Opcode, value: i32) {
        self.bytecode.push(opcode.value);
        self.bytecode.extend_from_slice(&value.to_ne_bytes());
    }

    fn emit_double(&mut self, opcode: &Opcode, value: f64) {
        self.bytecode.push(opcode.value);
        self.bytecode
            .extend_from_slice(&value.to_bits().to_ne_bytes());
    }

    fn emit_program(&mut self, ast: &Program) {
        match ast {
            Program::Script(script) => self.emit_script(script),
            _ => unimplemented!(),
        }
    }

    fn emit_script(&mut self, ast: &Script) {
        for statement in &ast.statements {
            self.emit_statement(statement);
        }
        self.emit1(&RETURN);
    }

    fn emit_statement(&mut self, ast: &Statement) {
        match ast {
            Statement::IterationStatement(_) => unimplemented!(),
            Statement::ClassDeclaration(_) => unimplemented!(),
            Statement::BlockStatement(_) => unimplemented!(),
            Statement::BreakStatement(_) => unimplemented!(),
            Statement::ContinueStatement(_) => unimplemented!(),
            Statement::DebuggerStatement(_) => unimplemented!(),
            Statement::EmptyStatement => (),
            Statement::ExpressionStatement(ast) => self.emit_expression(ast),
            Statement::IfStatement(_) => unimplemented!(),
            Statement::LabeledStatement(_) => unimplemented!(),
            Statement::ReturnStatement(_) => unimplemented!(),
            Statement::SwitchStatement(_) => unimplemented!(),
            Statement::SwitchStatementWithDefault(_) => unimplemented!(),
            Statement::ThrowStatement(_) => unimplemented!(),
            Statement::TryCatchStatement(_) => unimplemented!(),
            Statement::TryFinallyStatement(_) => unimplemented!(),
            Statement::VariableDeclarationStatement(_) => unimplemented!(),
            Statement::WithStatement(_) => unimplemented!(),
            Statement::FunctionDeclaration(_) => unimplemented!(),
        }
    }

    fn emit_expression(&mut self, ast: &Expression) {
        match ast {
            Expression::MemberExpression(_) => unimplemented!(),
            Expression::ClassExpression(_) => unimplemented!(),
            Expression::LiteralBooleanExpression(_) => unimplemented!(),
            Expression::LiteralInfinityExpression(_) => unimplemented!(),
            Expression::LiteralNullExpression(_) => unimplemented!(),
            Expression::LiteralNumericExpression(ast) => self.emit_numeric_expression(ast),
            Expression::LiteralRegExpExpression(_) => unimplemented!(),
            Expression::LiteralStringExpression(_) => unimplemented!(),
            Expression::ArrayExpression(_) => unimplemented!(),
            Expression::ArrowExpression(_) => unimplemented!(),
            Expression::AssignmentExpression(_) => unimplemented!(),
            Expression::BinaryExpression(ast) => self.emit_binary_expression(ast),
            Expression::CallExpression(_) => unimplemented!(),
            Expression::CompoundAssignmentExpression(_) => unimplemented!(),
            Expression::ConditionalExpression(_) => unimplemented!(),
            Expression::FunctionExpression(_) => unimplemented!(),
            Expression::IdentifierExpression(_) => unimplemented!(),
            Expression::NewExpression(_) => unimplemented!(),
            Expression::NewTargetExpression(_) => unimplemented!(),
            Expression::ObjectExpression(_) => unimplemented!(),
            Expression::UnaryExpression(_) => unimplemented!(),
            Expression::TemplateExpression(_) => unimplemented!(),
            Expression::ThisExpression(_) => unimplemented!(),
            Expression::UpdateExpression(_) => unimplemented!(),
            Expression::YieldExpression(_) => unimplemented!(),
            Expression::YieldGeneratorExpression(_) => unimplemented!(),
            Expression::AwaitExpression(_) => unimplemented!(),
        }
    }

    fn emit_binary_expression(&mut self, ast: &BinaryExpression) {
        self.emit_expression(&*ast.left);
        self.emit_expression(&*ast.right);
        match ast.operator {
            BinaryOperator::Equals => unimplemented!(),
            BinaryOperator::NotEquals => unimplemented!(),
            BinaryOperator::StrictEquals => unimplemented!(),
            BinaryOperator::StrictNotEquals => unimplemented!(),
            BinaryOperator::LessThan => unimplemented!(),
            BinaryOperator::LessThanOrEqual => unimplemented!(),
            BinaryOperator::GreaterThan => unimplemented!(),
            BinaryOperator::GreaterThanOrEqual => unimplemented!(),
            BinaryOperator::In => unimplemented!(),
            BinaryOperator::Instanceof => unimplemented!(),
            BinaryOperator::LeftShift => unimplemented!(),
            BinaryOperator::RightShift => unimplemented!(),
            BinaryOperator::RightShiftExt => unimplemented!(),
            BinaryOperator::Add => self.emit1(&ADD),
            BinaryOperator::Sub => unimplemented!(),
            BinaryOperator::Mul => unimplemented!(),
            BinaryOperator::Div => unimplemented!(),
            BinaryOperator::Mod => unimplemented!(),
            BinaryOperator::Pow => unimplemented!(),
            BinaryOperator::Comma => unimplemented!(),
            BinaryOperator::LogicalOr => unimplemented!(),
            BinaryOperator::LogicalAnd => unimplemented!(),
            BinaryOperator::BitwiseOr => unimplemented!(),
            BinaryOperator::BitwiseXor => unimplemented!(),
            BinaryOperator::BitwiseAnd => unimplemented!(),
        }
    }

    fn emit_numeric_expression(&mut self, ast: &LiteralNumericExpression) {
        let value = ast.value;
        let value_i8 = value as i8;
        let value_i32 = value as i32;
        if value_i8 as f64 == value {
            self.emit_int8(&INT8, value_i8);
        } else if value_i32 as f64 == value {
            self.emit_int32(&INT32, value_i32);
        } else {
            self.emit_double(&DOUBLE, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::emit;
    use crate::opcode::*;
    use parser::parse_script;

    fn run(source: &str) -> Vec<u8> {
        let parse_result = parse_script(source).expect("Failed to parse");
        println!("{:?}", parse_result);
        emit(&ast::Program::Script(*parse_result))
    }

    #[test]
    fn it_works() {
        assert_eq!(
            run("2 + 2"),
            vec![INT8.value, 0, INT8.value, 0, ADD.value, RETURN.value,]
        )
    }
}
