//! High-level bytecode emitter.
//!
//! Converts AST nodes to bytecode.

use super::emitter::{BytecodeOffset, EmitError, EmitResult, InstructionWriter};
use super::opcode::Opcode;
use ast::types::*;

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
            Statement::DebuggerStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: DebuggerStatement"));
            }
            Statement::DoWhileStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: DoWhileStatement"));
            }
            Statement::EmptyStatement { .. } => (),
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
            Statement::ReturnStatement { .. } => {
                return Err(EmitError::NotImplemented("TODO: ReturnStatement"));
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
            Statement::VariableDeclarationStatement(_ast) => {
                return Err(EmitError::NotImplemented(
                    "TODO: VariableDeclarationStatement",
                ));
            }
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
                    object: ExpressionOrSuper::Super { .. },
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
                    object: ExpressionOrSuper::Super { .. },
                    property,
                    ..
                },
            )) => {
                self.emit_this()?;
                self.emit.callee();
                self.emit.super_base();
                self.emit.get_prop_super(&property.value);
            }

            Expression::MemberExpression(MemberExpression::PrivateFieldExpression(
                PrivateFieldExpression { .. },
            )) => {
                return Err(EmitError::NotImplemented("PrivateFieldExpression"));
            }

            Expression::ClassExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: ClassExpression"));
            }

            Expression::LiteralBooleanExpression { value, .. } => {
                self.emit.emit_boolean(*value);
            }

            Expression::LiteralInfinityExpression { .. } => {
                self.emit.double(std::f64::INFINITY);
            }

            Expression::LiteralNullExpression { .. } => {
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

            Expression::NewTargetExpression { .. } => {
                return Err(EmitError::NotImplemented("TODO: NewTargetExpression"));
            }

            Expression::ObjectExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: ObjectExpression"));
            }

            Expression::UnaryExpression {
                operator, operand, ..
            } => {
                let opcode = match operator {
                    UnaryOperator::Plus { .. } => Opcode::Pos,
                    UnaryOperator::Minus { .. } => Opcode::Neg,
                    UnaryOperator::LogicalNot { .. } => Opcode::Not,
                    UnaryOperator::BitwiseNot { .. } => Opcode::BitNot,
                    UnaryOperator::Void { .. } => Opcode::Void,
                    UnaryOperator::Typeof { .. } => {
                        return Err(EmitError::NotImplemented("TODO: Typeof"));
                    }
                    UnaryOperator::Delete { .. } => {
                        return Err(EmitError::NotImplemented("TODO: Delete"));
                    }
                };
                self.emit_expression(operand)?;
                self.emit.emit_unary_op(opcode);
            }

            Expression::TemplateExpression(_) => {
                return Err(EmitError::NotImplemented("TODO: TemplateExpression"));
            }

            Expression::ThisExpression { .. } => {
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
            BinaryOperator::Equals { .. } => Opcode::Eq,
            BinaryOperator::NotEquals { .. } => Opcode::Ne,
            BinaryOperator::StrictEquals { .. } => Opcode::StrictEq,
            BinaryOperator::StrictNotEquals { .. } => Opcode::StrictNe,
            BinaryOperator::LessThan { .. } => Opcode::Lt,
            BinaryOperator::LessThanOrEqual { .. } => Opcode::Le,
            BinaryOperator::GreaterThan { .. } => Opcode::Gt,
            BinaryOperator::GreaterThanOrEqual { .. } => Opcode::Ge,
            BinaryOperator::In { .. } => Opcode::In,
            BinaryOperator::Instanceof { .. } => Opcode::Instanceof,
            BinaryOperator::LeftShift { .. } => Opcode::Lsh,
            BinaryOperator::RightShift { .. } => Opcode::Rsh,
            BinaryOperator::RightShiftExt { .. } => Opcode::Ursh,
            BinaryOperator::Add { .. } => Opcode::Add,
            BinaryOperator::Sub { .. } => Opcode::Sub,
            BinaryOperator::Mul { .. } => Opcode::Mul,
            BinaryOperator::Div { .. } => Opcode::Div,
            BinaryOperator::Mod { .. } => Opcode::Mod,
            BinaryOperator::Pow { .. } => Opcode::Pow,
            BinaryOperator::BitwiseOr { .. } => Opcode::BitOr,
            BinaryOperator::BitwiseXor { .. } => Opcode::BitXor,
            BinaryOperator::BitwiseAnd { .. } => Opcode::BitAnd,

            BinaryOperator::Coalesce { .. }
            | BinaryOperator::LogicalOr { .. }
            | BinaryOperator::LogicalAnd { .. } => {
                self.emit_short_circuit(operator, left, right)?;
                return Ok(());
            }

            BinaryOperator::Comma { .. } => {
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

    fn emit_short_circuit(
        &mut self,
        operator: &BinaryOperator,
        left: &Expression,
        right: &Expression,
    ) -> Result<(), EmitError> {
        self.emit_expression(left)?;
        let mut jumplist: Vec<BytecodeOffset> = Vec::with_capacity(1);
        self.emit_jump(operator, &mut jumplist)?;
        self.emit.pop();
        self.emit_expression(right)?;
        self.emit_jump_target(jumplist);
        return Ok(());
    }

    fn emit_jump(
        &mut self,
        operator: &BinaryOperator,
        jumplist: &mut Vec<BytecodeOffset>,
    ) -> Result<(), EmitError> {
        let offset: BytecodeOffset = self.emit.bytecode_offset();
        jumplist.push(offset);

        // in the c++ bytecode emitter, the jumplist is emitted
        // and four bytes are used in order to save memory. We are not using that
        // here, so instead we are using a placeholder offset set to 0, which will
        // be updated later in patch_jump_target.
        let placeholder_offset: i32 = 0;
        match operator {
            BinaryOperator::Coalesce { .. } => {
                self.emit.coalesce(placeholder_offset);
            }
            BinaryOperator::LogicalOr { .. } => {
                self.emit.or(placeholder_offset);
            }
            BinaryOperator::LogicalAnd { .. } => {
                self.emit.and(placeholder_offset);
            }
            _ => panic!("unrecognized operator used in jump"),
        }

        return Ok(());
    }

    fn emit_jump_target(&mut self, jumplist: Vec<BytecodeOffset>) {
        self.emit.patch_jump_target(jumplist);
        self.emit.jump_target();
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
        self.emit.get_g_name(name);
    }

    fn emit_call_expression(
        &mut self,
        callee: &ExpressionOrSuper,
        arguments: &Arguments,
    ) -> Result<(), EmitError> {
        // Don't do super handling in an emit_expresion_or_super because the bytecode heavily
        // depends on how you're using the super
        match callee {
            ExpressionOrSuper::Expression(expr) => match &**expr {
                Expression::IdentifierExpression(IdentifierExpression { name, .. }) => {
                    self.emit_expression(expr)?;
                    self.emit.g_implicit_this(name.value);
                }
                _ => {
                    return Err(EmitError::NotImplemented(
                        "TODO: Call (only global functions are supported)",
                    ));
                }
            },
            _ => {
                return Err(EmitError::NotImplemented("TODO: Super"));
            }
        }

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
