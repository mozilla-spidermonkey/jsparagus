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
    strings: Vec<String>,
}

impl Emitter {
    fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            strings: Vec::new(),
        }
    }

    fn emit1(&mut self, opcode: &Opcode) {
        self.bytecode.push(opcode.value);
    }

    fn emit_i8(&mut self, opcode: &Opcode, value: i8) {
        self.bytecode.push(opcode.value);
        self.bytecode.extend_from_slice(&value.to_ne_bytes());
    }

    fn emit_u16(&mut self, opcode: &Opcode, value: u16) {
        self.bytecode.push(opcode.value);
        self.bytecode.extend_from_slice(&value.to_ne_bytes());
    }

    fn emit_i32(&mut self, opcode: &Opcode, value: i32) {
        self.bytecode.push(opcode.value);
        self.bytecode.extend_from_slice(&value.to_ne_bytes());
    }

    fn emit_f64(&mut self, opcode: &Opcode, value: f64) {
        self.bytecode.push(opcode.value);
        self.bytecode
            .extend_from_slice(&value.to_bits().to_ne_bytes());
    }

    fn emit_str(&mut self, opcode: &Opcode, value: &str) {
        self.bytecode.push(opcode.value);
        let mut index = None;
        // Eventually we should be fancy and make this O(1)
        for (i, string) in self.strings.iter().enumerate() {
            if string == value {
                index = Some(i as u32);
            }
        }
        let index: u32 = match index {
            Some(index) => index,
            None => {
                let index = self.strings.len();
                self.strings.push(value.to_string());
                index as u32
            }
        };
        self.bytecode.extend_from_slice(&index.to_ne_bytes());
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
        self.emit1(&RETRVAL);
    }

    fn emit_statement(&mut self, ast: &Statement) {
        match ast {
            Statement::IterationStatement(_) => unimplemented!(),
            Statement::ClassDeclaration(_) => unimplemented!(),
            Statement::BlockStatement(_) => unimplemented!(),
            Statement::BreakStatement(_) => unimplemented!(),
            Statement::ContinueStatement(_) => unimplemented!(),
            Statement::DebuggerStatement => unimplemented!(),
            Statement::EmptyStatement => (),
            Statement::ExpressionStatement(ast) => {
                self.emit_expression(ast);
                self.emit1(&SETRVAL)
            }
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
            Expression::LiteralInfinityExpression => unimplemented!(),
            Expression::LiteralNullExpression => unimplemented!(),
            Expression::LiteralNumericExpression(ast) => self.emit_numeric_expression(ast),
            Expression::LiteralRegExpExpression(_) => unimplemented!(),
            Expression::LiteralStringExpression(_) => unimplemented!(),
            Expression::ArrayExpression(_) => unimplemented!(),
            Expression::ArrowExpression(_) => unimplemented!(),
            Expression::AssignmentExpression(_) => unimplemented!(),
            Expression::BinaryExpression(ast) => self.emit_binary_expression(ast),
            Expression::CallExpression(ast) => self.emit_call_expression(ast),
            Expression::CompoundAssignmentExpression(_) => unimplemented!(),
            Expression::ConditionalExpression(_) => unimplemented!(),
            Expression::FunctionExpression(_) => unimplemented!(),
            Expression::IdentifierExpression(ast) => self.emit_identifier_expression(ast),
            Expression::NewExpression(_) => unimplemented!(),
            Expression::NewTargetExpression => unimplemented!(),
            Expression::ObjectExpression(_) => unimplemented!(),
            Expression::UnaryExpression(_) => unimplemented!(),
            Expression::TemplateExpression(_) => unimplemented!(),
            Expression::ThisExpression => unimplemented!(),
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

    // We only want to emit integer values if the f64 value is *exactly* equivalent to a integer,
    // hence, direct equality of f64 values is okay.
    #[allow(clippy::float_cmp)]
    fn emit_numeric_expression(&mut self, ast: &LiteralNumericExpression) {
        let value = ast.value;
        let value_i8 = value as i8;
        let value_i32 = value as i32;
        if f64::from(value_i8) == value {
            self.emit_i8(&INT8, value_i8);
        } else if f64::from(value_i32) == value {
            self.emit_i32(&INT32, value_i32);
        } else {
            self.emit_f64(&DOUBLE, value);
        }
    }

    fn emit_identifier_expression(&mut self, ast: &IdentifierExpression) {
        self.emit_variable_reference(&ast.var);
    }

    fn emit_variable_reference(&mut self, ast: &VariableReference) {
        match ast {
            VariableReference::BindingIdentifier(ast) => self.emit_binding_identifier(ast),
            VariableReference::AssignmentTargetIdentifier(_) => unimplemented!(),
        }
    }

    fn emit_binding_identifier(&mut self, ast: &BindingIdentifier) {
        let name = &ast.name.value;
        self.emit_str(&GETGNAME, name);
    }

    fn emit_call_expression(&mut self, ast: &CallExpression) {
        // Don't do super handling in an emit_expresion_or_super because the bytecode heavily
        // depends on how you're using the super
        match &ast.callee {
            ExpressionOrSuper::Expression(ast) => self.emit_expression(ast),
            ExpressionOrSuper::Super => unimplemented!(),
        }

        self.emit_str(&GIMPLICITTHIS, "asdf");

        self.emit_arguments(&ast.arguments);
        self.emit_u16(&CALL, ast.arguments.args.len() as u16);
    }

    fn emit_arguments(&mut self, ast: &Arguments) {
        for argument in &ast.args {
            self.emit_argument(argument);
        }
    }

    fn emit_argument(&mut self, ast: &Argument) {
        match ast {
            Argument::Expression(ast) => self.emit_expression(ast),
            Argument::SpreadElement(_) => unimplemented!(),
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
            vec![
                INT8.value,
                0,
                INT8.value,
                0,
                ADD.value,
                SETRVAL.value,
                RETRVAL.value
            ]
        )
    }

    #[test]
    fn dis_call() {
        assert_eq!(
            run("dis()"),
            vec![
                GETGNAME.value,
                0,
                0,
                0,
                0,
                GIMPLICITTHIS.value,
                1,
                0,
                0,
                0,
                CALL.value,
                0,
                0,
                SETRVAL.value,
                RETRVAL.value
            ]
        )
    }
}
