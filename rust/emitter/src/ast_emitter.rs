//! High-level bytecode emitter.
//!
//! Converts AST nodes to bytecode.

use super::emitter::{EmitError, EmitResult, InstructionWriter};
use super::opcode::Opcode;
use ast::{arena, types::*};

/// Emit a program, converting the AST directly to bytecode.
pub fn emit_program(ast: &Program) -> Result<EmitResult, EmitError> {
    let mut emitter = AstEmitter {
        emit: InstructionWriter::new(),
    };

    match ast {
        Program::Script(script) => emitter.emit_script(script)?,
        _ => {
            return Err(EmitError::NotImplemented("TODO: modules"));
        }
    }

    Ok(emitter.emit.into_emit_result())
}

struct AstEmitter {
    emit: InstructionWriter,
}

impl AstEmitter {
    fn emit_script(&mut self, ast: &Script) -> Result<(), EmitError> {
        for statement in &ast.statements {
            self.emit_statement(statement)?;
        }
        self.emit.ret_rval();

        Ok(())
    }

    fn emit_statement(&mut self, ast: &Statement) -> Result<(), EmitError> {
        match ast {
            Statement::ClassDeclaration(_) => {
                return Err(EmitError::NotImplemented("TODO: ClassDeclaration"));
            }
            Statement::BlockStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: BlockStatement"));
            }
            Statement::BreakStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: BreakStatement"));
            }
            Statement::ContinueStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: ContinueStatement"));
            }
            Statement::DebuggerStatement(_) => {
                return Err(EmitError::NotImplemented("TODO: DebuggerStatement"));
            }
            Statement::DoWhileStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: DoWhileStatement"));
            }
            Statement::EmptyStatement(_) => (),
            Statement::ExpressionStatement(ast) => {
                self.emit_expression(ast)?;
                self.emit.set_rval();
            }
            Statement::ForInStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: ForInStatement"));
            }
            Statement::ForOfStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: ForOfStatement"));
            }
            Statement::ForStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: ForStatement"));
            }
            Statement::IfStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: IfStatement"));
            }
            Statement::LabeledStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: LabeledStatement"));
            }
            Statement::ReturnStatement { expression, .. } => {
                self.emit_return_statement(expression)?;
            }
            Statement::SwitchStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: SwitchStatement"));
            }
            Statement::SwitchStatementWithDefault { .. } => {
                return Err(EmitError::NotImplemented(
                    "TODO: SwitchStatementWithDefault",
                ));
            }
            Statement::ThrowStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: ThrowStatement"));
            }
            Statement::TryCatchStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: TryCatchStatement"));
            }
            Statement::TryFinallyStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: TryFinallyStatement"));
            }
            Statement::VariableDeclarationStatement(ast) => self.emit_variable_declaration(ast)?,
            Statement::WhileStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: WhileStatement"));
            }
            Statement::WithStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: WithStatement"));
            }
            Statement::FunctionDeclaration(_) => {
                return Err(EmitError::NotImplemented("TODO: FunctionDeclaration"));
            }
        };

        Ok(())
    }

    fn emit_variable_declaration(&mut self, ast: &VariableDeclaration) -> Result<(), EmitError> {
        match ast.kind {
            VariableDeclarationKind::Var(_) => (),
            VariableDeclarationKind::Let(_) => (),
            VariableDeclarationKind::Const(_) => (),
        }
        for declarator in &ast.declarators {
            let _ = match &declarator.binding {
                Binding::BindingPattern(_) => {
                    return Err(EmitError::NotImplemented("TODO: BindingPattern"));
                }
                Binding::BindingIdentifier(ident) => &ident.name.value,
            };
            // TODO
            self.emit.uninitialized();
            self.emit.init_lexical(0);
            self.emit.pop();

            if let Some(init) = &declarator.init {
                self.emit_expression(&*init)?;
            }

            self.emit.init_lexical(0);
            self.emit.pop();
        }

        Ok(())
    }

    fn emit_return_statement(
        &mut self,
        expression: &Option<arena::Box<Expression>>,
    ) -> Result<(), EmitError> {
        match expression {
            Some(ast) => self.emit_expression(ast)?,
            None => self.emit.undefined(),
        }
        self.emit.set_rval();
        self.emit.ret_rval();

        Ok(())
    }

    fn emit_this(&mut self) -> Result<(), EmitError> {
        Err(EmitError::NotImplemented("TODO: this"))
    }

    fn emit_expression(&mut self, ast: &Expression) -> Result<(), EmitError> {
        match ast {
            Expression::MemberExpression(MemberExpression::ComputedMemberExpression(
                ComputedMemberExpression {
                    object: ExpressionOrSuper::Expression(object),
                    expression,
                    ..
                },
            )) => {
                self.emit_expression(object)?;
                self.emit_expression(expression)?;
                self.emit.get_elem();
            }

            Expression::MemberExpression(MemberExpression::ComputedMemberExpression(
                ComputedMemberExpression {
                    object: ExpressionOrSuper::Super(_),
                    expression,
                    ..
                },
            )) => {
                self.emit_this()?;
                self.emit_expression(expression)?;
                self.emit.callee();
                self.emit.super_base();
                self.emit.get_elem_super();
            }

            Expression::MemberExpression(MemberExpression::StaticMemberExpression(
                StaticMemberExpression {
                    object: ExpressionOrSuper::Expression(object),
                    property,
                    ..
                },
            )) => {
                self.emit_expression(object)?;
                self.emit.get_prop(&property.value);
            }

            Expression::MemberExpression(MemberExpression::StaticMemberExpression(
                StaticMemberExpression {
                    object: ExpressionOrSuper::Super(_),
                    property,
                    ..
                },
            )) => {
                self.emit_this()?;
                self.emit.callee();
                self.emit.super_base();
                self.emit.get_prop_super(&property.value);
            }

            Expression::ClassExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: ClassExpression"));
            }

            Expression::LiteralBooleanExpression { value, .. } => {
                self.emit.emit_boolean(*value);
            }

            Expression::LiteralInfinityExpression(_) => {
                self.emit.double(std::f64::INFINITY);
            }

            Expression::LiteralNullExpression(_) => {
                self.emit.null();
            }

            Expression::LiteralNumericExpression { value, .. } => {
                self.emit_numeric_expression(*value);
            }

            Expression::LiteralRegExpExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: LiteralRegExpExpression"));
            }

            Expression::LiteralStringExpression { value, .. } => {
                self.emit.string(value);
            }

            Expression::ArrayExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: ArrayExpression"));
            }

            Expression::ArrowExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: ArrowExpression"));
            }

            Expression::AssignmentExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: AssignmentExpression"));
            }

            Expression::BinaryExpression {
                operator,
                left,
                right,
                ..
            } => {
                self.emit_binary_expression(operator, left, right)?;
            }

            Expression::CallExpression {
                callee, arguments, ..
            } => {
                self.emit_call_expression(callee, arguments)?;
            }

            Expression::CompoundAssignmentExpression { .. } => {
                return Err(EmitError::NotImplemented(
                    "TODO: CompoundAssignmentExpression",
                ));
            }

            Expression::ConditionalExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: ConditionalExpression"));
            }

            Expression::FunctionExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: FunctionExpression"));
            }

            Expression::IdentifierExpression(ast) => {
                self.emit_identifier_expression(ast);
            }

            Expression::NewExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: NewExpression"));
            }

            Expression::NewTargetExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: NewTargetExpression"));
            }

            Expression::ObjectExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: ObjectExpression"));
            }

            Expression::UnaryExpression {
                operator, operand, ..
            } => {
                let opcode = match operator {
                    UnaryOperator::Plus(_) => Opcode::Pos,
                    UnaryOperator::Minus(_) => Opcode::Neg,
                    UnaryOperator::LogicalNot(_) => Opcode::Not,
                    UnaryOperator::BitwiseNot(_) => Opcode::BitNot,
                    UnaryOperator::Void(_) => Opcode::Void,
                    UnaryOperator::Typeof(_) => {
                        return Err(EmitError::NotImplemented("TODO: Typeof"));
                    }
                    UnaryOperator::Delete(_) => {
                        return Err(EmitError::NotImplemented("TODO: Delete"));
                    }
                };
                self.emit_expression(operand)?;
                self.emit.emit_unary_op(opcode);
            }

            Expression::TemplateExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: TemplateExpression"));
            }

            Expression::ThisExpression(_) => {
                self.emit_this()?;
            }

            Expression::UpdateExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: UpdateExpression"));
            }

            Expression::YieldExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: YieldExpression"));
            }

            Expression::YieldGeneratorExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: YieldGeneratorExpression"));
            }

            Expression::AwaitExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: AwaitExpression"));
            }

            Expression::ImportCallExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: ImportCallExpression"));
            }
        }

        Ok(())
    }

    fn emit_binary_expression(
        &mut self,
        operator: &BinaryOperator,
        left: &Expression,
        right: &Expression,
    ) -> Result<(), EmitError> {
        let opcode = match operator {
            BinaryOperator::Equals(_) => Opcode::Eq,
            BinaryOperator::NotEquals(_) => Opcode::Ne,
            BinaryOperator::StrictEquals(_) => Opcode::StrictEq,
            BinaryOperator::StrictNotEquals(_) => Opcode::StrictNe,
            BinaryOperator::LessThan(_) => Opcode::Lt,
            BinaryOperator::LessThanOrEqual(_) => Opcode::Le,
            BinaryOperator::GreaterThan(_) => Opcode::Gt,
            BinaryOperator::GreaterThanOrEqual(_) => Opcode::Ge,
            BinaryOperator::In(_) => Opcode::In,
            BinaryOperator::Instanceof(_) => Opcode::Instanceof,
            BinaryOperator::LeftShift(_) => Opcode::Lsh,
            BinaryOperator::RightShift(_) => Opcode::Rsh,
            BinaryOperator::RightShiftExt(_) => Opcode::Ursh,
            BinaryOperator::Add(_) => Opcode::Add,
            BinaryOperator::Sub(_) => Opcode::Sub,
            BinaryOperator::Mul(_) => Opcode::Mul,
            BinaryOperator::Div(_) => Opcode::Div,
            BinaryOperator::Mod(_) => Opcode::Mod,
            BinaryOperator::Pow(_) => Opcode::Pow,
            BinaryOperator::BitwiseOr(_) => Opcode::BitOr,
            BinaryOperator::BitwiseXor(_) => Opcode::BitXor,
            BinaryOperator::BitwiseAnd(_) => Opcode::BitAnd,
            BinaryOperator::Coalesce(_) => {
                return Err(EmitError::NotImplemented("TODO: Coalescer"));
            }
            BinaryOperator::LogicalOr(_) => {
                return Err(EmitError::NotImplemented("TODO: LogicalOr"));
            }
            BinaryOperator::LogicalAnd(_) => {
                return Err(EmitError::NotImplemented("TODO: LogicalAndr"));
            }
            BinaryOperator::Comma(_) => {
                self.emit_expression(left)?;
                self.emit.pop();
                self.emit_expression(right)?;
                return Ok(());
            }
        };

        self.emit_expression(left)?;
        self.emit_expression(right)?;
        self.emit.emit_binary_op(opcode);
        Ok(())
    }

    fn emit_numeric_expression(&mut self, value: f64) {
        if value.is_finite() && value.fract() == 0.0 {
            if i8::min_value() as f64 <= value && value <= i8::max_value() as f64 {
                self.emit.int8(value as i8);
                return;
            }
            if i32::min_value() as f64 <= value && value <= i32::max_value() as f64 {
                self.emit.int32(value as i32);
                return;
            }
        }
        self.emit.double(value);
    }

    fn emit_identifier_expression(&mut self, ast: &IdentifierExpression) {
        let name = &ast.name.value;
        self.emit.get_gname(name);
    }

    fn emit_call_expression(
        &mut self,
        callee: &ExpressionOrSuper,
        arguments: &Arguments,
    ) -> Result<(), EmitError> {
        // Don't do super handling in an emit_expresion_or_super because the bytecode heavily
        // depends on how you're using the super
        match callee {
            ExpressionOrSuper::Expression(ast) => self.emit_expression(ast)?,
            ExpressionOrSuper::Super(_) => {
                return Err(EmitError::NotImplemented("TODO: Super"));
            }
        }

        self.emit.g_implicit_this("asdf");

        self.emit_arguments(arguments)?;
        self.emit.call(arguments.args.len() as u16);

        Ok(())
    }

    fn emit_arguments(&mut self, ast: &Arguments) -> Result<(), EmitError> {
        for argument in &ast.args {
            self.emit_argument(argument)?;
        }

        Ok(())
    }

    fn emit_argument(&mut self, ast: &Argument) -> Result<(), EmitError> {
        match ast {
            Argument::Expression(ast) => self.emit_expression(ast)?,
            Argument::SpreadElement(_) => {
                return Err(EmitError::NotImplemented("TODO: SpreadElement"));
            }
        }

        Ok(())
    }
}
