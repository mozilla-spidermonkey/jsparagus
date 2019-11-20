//! High-level bytecode emitter.
//!
//! Converts AST nodes to bytecode.

use super::emitter::{EmitResult, EmitError, Emitter};
use super::opcode::Opcode;
use ast::{arena, types::*};

/// Emit a program, converting the AST directly to bytecode.
pub fn emit_program(ast: &Program) -> Result<EmitResult, EmitError> {
    let mut emitter = AstEmitter {
        emit: Emitter::new(),
    };

    match ast {
        Program::Script(script) => emitter.emit_script(script)?,
        _ => {
            return Err(EmitError::Unimplemented(
                "TODO: modules".to_string()));
        }
    }

    Ok(emitter.emit.into_emit_result())
}

struct AstEmitter {
    emit: Emitter,
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
                return Err(EmitError::Unimplemented(
                    "TODO: ClassDeclaration".to_string()));
            },
            Statement::BlockStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: BlockStatement".to_string()));
            },
            Statement::BreakStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: BreakStatement".to_string()));
            },
            Statement::ContinueStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: ContinueStatement".to_string()));
            },
            Statement::DebuggerStatement => {
                return Err(EmitError::Unimplemented(
                    "TODO: DebuggerStatement".to_string()));
            },
            Statement::DoWhileStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: DoWhileStatement".to_string()));
            },
            Statement::EmptyStatement => (),
            Statement::ExpressionStatement(ast) => {
                self.emit_expression(ast)?;
                self.emit.set_rval();
            }
            Statement::ForInStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: ForInStatement".to_string()));
            },
            Statement::ForOfStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: ForOfStatement".to_string()));
            },
            Statement::ForStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: ForStatement".to_string()));
            },
            Statement::IfStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: IfStatement".to_string()));
            },
            Statement::LabeledStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: LabeledStatement".to_string()));
            },
            Statement::ReturnStatement { expression } => {
                self.emit_return_statement(expression)?;
            },
            Statement::SwitchStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: SwitchStatement".to_string()));
            },
            Statement::SwitchStatementWithDefault { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: SwitchStatementWithDefault".to_string()));
            },
            Statement::ThrowStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: ThrowStatement".to_string()));
            },
            Statement::TryCatchStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: TryCatchStatement".to_string()));
            },
            Statement::TryFinallyStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: TryFinallyStatement".to_string()));
            },
            Statement::VariableDeclarationStatement(ast) => {
                self.emit_variable_declaration(ast)?
            },
            Statement::WhileStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: WhileStatement".to_string()));
            },
            Statement::WithStatement { .. } => {
                return Err(EmitError::Unimplemented(
                    "TODO: WithStatement".to_string()));
            },
            Statement::FunctionDeclaration(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: FunctionDeclaration".to_string()));
            },
        };

        Ok(())
    }

    fn emit_variable_declaration(&mut self, ast: &VariableDeclaration)
                                 -> Result<(), EmitError> {
        match ast.kind {
            VariableDeclarationKind::Var => (),
            VariableDeclarationKind::Let => (),
            VariableDeclarationKind::Const => (),
        }
        for declarator in &ast.declarators {
            let _ = match &declarator.binding {
                Binding::BindingPattern(_) => {
                    return Err(EmitError::Unimplemented(
                        "TODO: BindingPattern".to_string()));
                },
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

    fn emit_return_statement(&mut self, expression: &Option<arena::Box<Expression>>)
                             -> Result<(), EmitError> {
        match expression {
            Some(ast) => self.emit_expression(ast)?,
            None => self.emit.undefined(),
        }
        self.emit.set_rval();
        self.emit.ret_rval();

        Ok(())
    }

    fn emit_this(&mut self) -> Result<(), EmitError> {
        Err(EmitError::Unimplemented(
            "TODO: this".to_string()))
    }

    fn emit_expression(&mut self, ast: &Expression) -> Result<(), EmitError> {
        match ast {
            Expression::MemberExpression(MemberExpression::ComputedMemberExpression(
                ComputedMemberExpression {
                    object: ExpressionOrSuper::Expression(object),
                    expression,
                },
            )) => {
                self.emit_expression(object)?;
                self.emit_expression(expression)?;
                self.emit.get_elem();
            }

            Expression::MemberExpression(MemberExpression::ComputedMemberExpression(
                ComputedMemberExpression {
                    object: ExpressionOrSuper::Super,
                    expression,
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
                },
            )) => {
                self.emit_expression(object)?;
                self.emit.get_prop(&property.value);
            }

            Expression::MemberExpression(MemberExpression::StaticMemberExpression(
                StaticMemberExpression {
                    object: ExpressionOrSuper::Super,
                    property,
                },
            )) => {
                self.emit_this()?;
                self.emit.callee();
                self.emit.super_base();
                self.emit.get_prop_super(&property.value);
            }

            Expression::ClassExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: ClassExpressionr".to_string()));
            },

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

            Expression::LiteralRegExpExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: LiteralRegExpExpression".to_string()));
            },

            Expression::LiteralStringExpression(ast) => {
                self.emit.string(&ast.value);
            }

            Expression::ArrayExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: ArrayExpression".to_string()));
            },
            Expression::ArrowExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: ArrowExpression".to_string()));
            },
            Expression::AssignmentExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: AssignmentExpression".to_string()));
            },
            Expression::BinaryExpression(ast) => {
                self.emit_binary_expression(ast)?;
            }

            Expression::CallExpression(ast) => {
                self.emit_call_expression(ast)?;
            }

            Expression::CompoundAssignmentExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: CompoundAssignmentExpression".to_string()));
            },
            Expression::ConditionalExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: ConditionalExpression".to_string()));
            },
            Expression::FunctionExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: FunctionExpression".to_string()));
            },
            Expression::IdentifierExpression(ast) => {
                self.emit_identifier_expression(ast);
            }

            Expression::NewExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: NewExpression".to_string()));
            },
            Expression::NewTargetExpression => {
                return Err(EmitError::Unimplemented(
                    "TODO: NewTargetExpression".to_string()));
            },
            Expression::ObjectExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: ObjectExpression".to_string()));
            },

            Expression::UnaryExpression(UnaryExpression { operator, operand }) => {
                let opcode = match operator {
                    UnaryOperator::Plus => Opcode::Pos,
                    UnaryOperator::Minus => Opcode::Neg,
                    UnaryOperator::LogicalNot => Opcode::Not,
                    UnaryOperator::BitwiseNot => Opcode::BitNot,
                    UnaryOperator::Void => Opcode::Void,
                    UnaryOperator::Typeof => {
                        return Err(EmitError::Unimplemented(
                            "TODO: Typeof".to_string()));
                    },
                    UnaryOperator::Delete => {
                        return Err(EmitError::Unimplemented(
                            "TODO: Delete".to_string()));
                    },
                };
                self.emit_expression(operand)?;
                self.emit.emit_unary_op(opcode);
            }

            Expression::TemplateExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: TemplateExpression".to_string()));
            },

            Expression::ThisExpression => {
                self.emit_this()?;
            }

            Expression::UpdateExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: UpdateExpression".to_string()));
            },
            Expression::YieldExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: YieldExpression".to_string()));
            },
            Expression::YieldGeneratorExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: YieldGeneratorExpression".to_string()));
            },
            Expression::AwaitExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: AwaitExpression".to_string()));
            },
            Expression::ImportCallExpression(_) => {
                return Err(EmitError::Unimplemented(
                    "TODO: ImportCallExpression".to_string()));
            },
        }

        Ok(())
    }

    fn emit_binary_expression(&mut self, ast: &BinaryExpression)
                              -> Result<(), EmitError> {
        self.emit_expression(&*ast.left)?;
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
            BinaryOperator::Coalesce => {
                return Err(EmitError::Unimplemented(
                    "TODO: Coalescer".to_string()));
            }
            BinaryOperator::LogicalOr => {
                return Err(EmitError::Unimplemented(
                    "TODO: LogicalOr".to_string()));
            },
            BinaryOperator::LogicalAnd => {
                return Err(EmitError::Unimplemented(
                    "TODO: LogicalAndr".to_string()));
            },
            BinaryOperator::Comma => {
                self.emit.pop();
                self.emit_expression(&*ast.right)?;
                return Ok(());
            }
        };
        self.emit_expression(&*ast.right)?;
        self.emit.emit_binary_op(opcode);

        Ok(())
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
        let name = &ast.name.value;
        self.emit.get_gname(name);
    }

    fn emit_call_expression(&mut self, ast: &CallExpression)
                            -> Result<(), EmitError> {
        // Don't do super handling in an emit_expresion_or_super because the bytecode heavily
        // depends on how you're using the super
        match &ast.callee {
            ExpressionOrSuper::Expression(ast) => self.emit_expression(ast)?,
            ExpressionOrSuper::Super => {
                return Err(EmitError::Unimplemented(
                    "TODO: Super".to_string()));
            }
        }

        self.emit.g_implicit_this("asdf");

        self.emit_arguments(&ast.arguments)?;
        self.emit.call(ast.arguments.args.len() as u16);

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
                return Err(EmitError::Unimplemented(
                    "TODO: SpreadElement".to_string()));
            }
        }

        Ok(())
    }
}
