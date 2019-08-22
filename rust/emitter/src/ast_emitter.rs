//! High-level bytecode emitter.
//!
//! Converts AST nodes to bytecode.

use super::emitter::{EmitResult, Emitter};
use super::opcode::Opcode;
use ast::*;

/// Emit a program, converting the AST directly to bytecode.
pub fn emit_program(ast: &Program) -> EmitResult {
    let mut emitter = AstEmitter {
        emit: Emitter::new(),
    };

    match ast {
        Program::Script(script) => emitter.emit_script(script),
        _ => unimplemented!(),
    }

    emitter.emit.into_emit_result()
}

struct AstEmitter {
    emit: Emitter,
}

impl AstEmitter {
    fn emit_script(&mut self, ast: &Script) {
        for statement in &ast.statements {
            self.emit_statement(statement);
        }
        self.emit.ret_rval();
    }

    fn emit_statement(&mut self, ast: &Statement) {
        match ast {
            Statement::ClassDeclaration(_) => unimplemented!(),
            Statement::BlockStatement(ast) => self.emit_block_statement(ast),
            Statement::BreakStatement(_) => unimplemented!(),
            Statement::ContinueStatement(_) => unimplemented!(),
            Statement::DebuggerStatement => unimplemented!(),
            Statement::DoWhileStatement(_) => unimplemented!(),
            Statement::EmptyStatement => (),
            Statement::ExpressionStatement(ast) => {
                self.emit_expression(ast);
                self.emit.set_rval();
            }
            Statement::ForInStatement(_) => unimplemented!(),
            Statement::ForOfStatement(_) => unimplemented!(),
            Statement::ForStatement(_) => unimplemented!(),
            Statement::IfStatement(_) => unimplemented!(),
            Statement::LabeledStatement(_) => unimplemented!(),
            Statement::ReturnStatement(ast) => self.emit_return_statement(ast),
            Statement::SwitchStatement(_) => unimplemented!(),
            Statement::SwitchStatementWithDefault(_) => unimplemented!(),
            Statement::ThrowStatement(_) => unimplemented!(),
            Statement::TryCatchStatement(_) => unimplemented!(),
            Statement::TryFinallyStatement(_) => unimplemented!(),
            Statement::VariableDeclarationStatement(ast) => self.emit_variable_declaration(ast),
            Statement::WhileStatement(_) => unimplemented!(),
            Statement::WithStatement(_) => unimplemented!(),
            Statement::FunctionDeclaration(_) => unimplemented!(),
        }
    }

    fn emit_block_statement(&mut self, _ast: &BlockStatement) {
        println!("TODO: emit_block");
    }

    fn emit_variable_declaration(&mut self, ast: &VariableDeclaration) {
        match ast.kind {
            VariableDeclarationKind::Var => (),
            VariableDeclarationKind::Let => (),
            VariableDeclarationKind::Const => (),
        }
        for declarator in &ast.declarators {
            let _ = match &declarator.binding {
                Binding::BindingPattern(_) => unimplemented!(),
                Binding::BindingIdentifier(ident) => &ident.name.value,
            };
            // TODO
            self.emit.uninitialized();
            self.emit.init_lexical(0);
            self.emit.pop();

            if let Some(init) = &declarator.init {
                self.emit_expression(&*init);
            }

            self.emit.init_lexical(0);
            self.emit.pop();
        }
    }

    fn emit_return_statement(&mut self, ast: &ReturnStatement) {
        match &ast.expression {
            Some(ast) => self.emit_expression(ast),
            None => self.emit.undefined(),
        }
        self.emit.set_rval();
        self.emit.ret_rval();
    }

    fn emit_this(&mut self) {
        unimplemented!();
    }

    fn emit_expression(&mut self, ast: &Expression) {
        match ast {
            Expression::MemberExpression(MemberExpression::ComputedMemberExpression(
                ComputedMemberExpression {
                    object: ExpressionOrSuper::Expression(object),
                    expression,
                },
            )) => {
                self.emit_expression(object);
                self.emit_expression(expression);
                self.emit.get_elem();
            }

            Expression::MemberExpression(MemberExpression::ComputedMemberExpression(
                ComputedMemberExpression {
                    object: ExpressionOrSuper::Super,
                    expression,
                },
            )) => {
                self.emit_this();
                self.emit_expression(expression);
                self.emit.callee();
                self.emit.super_base();
                self.emit.get_elem_super();
            }

            Expression::MemberExpression(MemberExpression::StaticMemberExpression(
                StaticMemberExpression {
                    object: ExpressionOrSuper::Expression(object),
                    property,
                },
            )) => {
                self.emit_expression(object);
                self.emit.get_prop(&property.value);
            }

            Expression::MemberExpression(MemberExpression::StaticMemberExpression(
                StaticMemberExpression {
                    object: ExpressionOrSuper::Super,
                    property,
                },
            )) => {
                self.emit_this();
                self.emit.callee();
                self.emit.super_base();
                self.emit.get_prop_super(&property.value);
            }

            Expression::ClassExpression(_) => unimplemented!(),

            Expression::LiteralBooleanExpression(literal) => {
                self.emit.emit_boolean(literal.value);
            }

            Expression::LiteralInfinityExpression => {
                self.emit.double(std::f64::INFINITY);
            }

            Expression::LiteralNullExpression => {
                self.emit.null();
            }

            Expression::LiteralNumericExpression(ast) => {
                self.emit_numeric_expression(ast);
            }

            Expression::LiteralRegExpExpression(_) => unimplemented!(),

            Expression::LiteralStringExpression(ast) => {
                self.emit.string(&ast.value);
            }

            Expression::ArrayExpression(_) => unimplemented!(),
            Expression::ArrowExpression(_) => unimplemented!(),
            Expression::AssignmentExpression(_) => unimplemented!(),
            Expression::BinaryExpression(ast) => {
                self.emit_binary_expression(ast);
            }

            Expression::CallExpression(ast) => {
                self.emit_call_expression(ast);
            }

            Expression::CompoundAssignmentExpression(_) => unimplemented!(),
            Expression::ConditionalExpression(_) => unimplemented!(),
            Expression::FunctionExpression(_) => unimplemented!(),
            Expression::IdentifierExpression(ast) => {
                self.emit_identifier_expression(ast);
            }

            Expression::NewExpression(_) => unimplemented!(),
            Expression::NewTargetExpression => unimplemented!(),
            Expression::ObjectExpression(_) => unimplemented!(),

            Expression::UnaryExpression(UnaryExpression { operator, operand }) => {
                let opcode = match operator {
                    UnaryOperator::Plus => Opcode::Pos,
                    UnaryOperator::Minus => Opcode::Neg,
                    UnaryOperator::LogicalNot => Opcode::Not,
                    UnaryOperator::BitwiseNot => Opcode::BitNot,
                    UnaryOperator::Void => Opcode::Void,
                    UnaryOperator::Typeof => unimplemented!(),
                    UnaryOperator::Delete => unimplemented!(),
                };
                self.emit_expression(operand);
                self.emit.emit_unary_op(opcode);
            }

            Expression::TemplateExpression(_) => unimplemented!(),

            Expression::ThisExpression => {
                self.emit_this();
            }

            Expression::UpdateExpression(_) => unimplemented!(),
            Expression::YieldExpression(_) => unimplemented!(),
            Expression::YieldGeneratorExpression(_) => unimplemented!(),
            Expression::AwaitExpression(_) => unimplemented!(),
        }
    }

    fn emit_binary_expression(&mut self, ast: &BinaryExpression) {
        self.emit_expression(&*ast.left);
        let opcode = match ast.operator {
            BinaryOperator::Equals => Opcode::Eq,
            BinaryOperator::NotEquals => Opcode::Ne,
            BinaryOperator::StrictEquals => Opcode::StrictEq,
            BinaryOperator::StrictNotEquals => Opcode::StrictNe,
            BinaryOperator::LessThan => Opcode::Lt,
            BinaryOperator::LessThanOrEqual => Opcode::Le,
            BinaryOperator::GreaterThan => Opcode::Gt,
            BinaryOperator::GreaterThanOrEqual => Opcode::Ge,
            BinaryOperator::In => Opcode::In,
            BinaryOperator::Instanceof => Opcode::Instanceof,
            BinaryOperator::LeftShift => Opcode::Lsh,
            BinaryOperator::RightShift => Opcode::Rsh,
            BinaryOperator::RightShiftExt => Opcode::Ursh,
            BinaryOperator::Add => Opcode::Add,
            BinaryOperator::Sub => Opcode::Sub,
            BinaryOperator::Mul => Opcode::Mul,
            BinaryOperator::Div => Opcode::Div,
            BinaryOperator::Mod => Opcode::Mod,
            BinaryOperator::Pow => Opcode::Pow,
            BinaryOperator::BitwiseOr => Opcode::BitOr,
            BinaryOperator::BitwiseXor => Opcode::BitXor,
            BinaryOperator::BitwiseAnd => Opcode::BitAnd,
            BinaryOperator::LogicalOr => unimplemented!(),
            BinaryOperator::LogicalAnd => unimplemented!(),
            BinaryOperator::Comma => {
                self.emit.pop();
                self.emit_expression(&*ast.right);
                return;
            }
        };
        self.emit_expression(&*ast.right);
        self.emit.emit_binary_op(opcode);
    }

    // We only want to emit integer values if the f64 value is *exactly* equivalent to a integer,
    // hence, direct equality of f64 values is okay.
    #[allow(clippy::float_cmp)]
    fn emit_numeric_expression(&mut self, ast: &LiteralNumericExpression) {
        let value = ast.value;
        let value_i8 = value as i8;
        let value_i32 = value as i32;
        if f64::from(value_i8) == value {
            self.emit.int8(value_i8);
        } else if f64::from(value_i32) == value {
            self.emit.int32(value_i32);
        } else {
            self.emit.double(value);
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
        self.emit.get_gname(name);
    }

    fn emit_call_expression(&mut self, ast: &CallExpression) {
        // Don't do super handling in an emit_expresion_or_super because the bytecode heavily
        // depends on how you're using the super
        match &ast.callee {
            ExpressionOrSuper::Expression(ast) => self.emit_expression(ast),
            ExpressionOrSuper::Super => unimplemented!(),
        }

        self.emit.g_implicit_this("asdf");

        self.emit_arguments(&ast.arguments);
        self.emit.call(ast.arguments.args.len() as u16);
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
