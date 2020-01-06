// WARNING: This file is auto-generated.

#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::arena;
use crate::types::*;
use bumpalo;

pub trait Pass<'alloc> {
    fn visit_argument(&mut self, ast: &mut Argument<'alloc>) {
        match ast {
            Argument::SpreadElement(ast) => {
                self.visit_expression(ast);
            }
            Argument::Expression(ast) => {
                self.visit_expression(ast);
            }
        }
    }

    fn visit_arguments(&mut self, ast: &mut Arguments<'alloc>) {
        for item in &mut ast.args {
            self.visit_argument(item);
        }
    }

    fn visit_identifier(&mut self, ast: &mut Identifier<'alloc>) {}

    fn visit_identifier_name(&mut self, ast: &mut IdentifierName<'alloc>) {}

    fn visit_label(&mut self, ast: &mut Label<'alloc>) {}

    fn visit_variable_declaration_kind(&mut self, ast: &mut VariableDeclarationKind) {
        match ast {
            VariableDeclarationKind::Var(_) => (),
            VariableDeclarationKind::Let(_) => (),
            VariableDeclarationKind::Const(_) => (),
        }
    }

    fn visit_compound_assignment_operator(&mut self, ast: &mut CompoundAssignmentOperator) {
        match ast {
            CompoundAssignmentOperator::Add(_) => (),
            CompoundAssignmentOperator::Sub(_) => (),
            CompoundAssignmentOperator::Mul(_) => (),
            CompoundAssignmentOperator::Div(_) => (),
            CompoundAssignmentOperator::Mod(_) => (),
            CompoundAssignmentOperator::Pow(_) => (),
            CompoundAssignmentOperator::LeftShift(_) => (),
            CompoundAssignmentOperator::RightShift(_) => (),
            CompoundAssignmentOperator::RightShiftExt(_) => (),
            CompoundAssignmentOperator::Or(_) => (),
            CompoundAssignmentOperator::Xor(_) => (),
            CompoundAssignmentOperator::And(_) => (),
        }
    }

    fn visit_binary_operator(&mut self, ast: &mut BinaryOperator) {
        match ast {
            BinaryOperator::Equals(_) => (),
            BinaryOperator::NotEquals(_) => (),
            BinaryOperator::StrictEquals(_) => (),
            BinaryOperator::StrictNotEquals(_) => (),
            BinaryOperator::LessThan(_) => (),
            BinaryOperator::LessThanOrEqual(_) => (),
            BinaryOperator::GreaterThan(_) => (),
            BinaryOperator::GreaterThanOrEqual(_) => (),
            BinaryOperator::In(_) => (),
            BinaryOperator::Instanceof(_) => (),
            BinaryOperator::LeftShift(_) => (),
            BinaryOperator::RightShift(_) => (),
            BinaryOperator::RightShiftExt(_) => (),
            BinaryOperator::Add(_) => (),
            BinaryOperator::Sub(_) => (),
            BinaryOperator::Mul(_) => (),
            BinaryOperator::Div(_) => (),
            BinaryOperator::Mod(_) => (),
            BinaryOperator::Pow(_) => (),
            BinaryOperator::Comma(_) => (),
            BinaryOperator::Coalesce(_) => (),
            BinaryOperator::LogicalOr(_) => (),
            BinaryOperator::LogicalAnd(_) => (),
            BinaryOperator::BitwiseOr(_) => (),
            BinaryOperator::BitwiseXor(_) => (),
            BinaryOperator::BitwiseAnd(_) => (),
        }
    }

    fn visit_unary_operator(&mut self, ast: &mut UnaryOperator) {
        match ast {
            UnaryOperator::Plus(_) => (),
            UnaryOperator::Minus(_) => (),
            UnaryOperator::LogicalNot(_) => (),
            UnaryOperator::BitwiseNot(_) => (),
            UnaryOperator::Typeof(_) => (),
            UnaryOperator::Void(_) => (),
            UnaryOperator::Delete(_) => (),
        }
    }

    fn visit_update_operator(&mut self, ast: &mut UpdateOperator) {
        match ast {
            UpdateOperator::Increment(_) => (),
            UpdateOperator::Decrement(_) => (),
        }
    }

    fn visit_function(&mut self, ast: &mut Function<'alloc>) {
        if let Some(item) = &mut ast.name {
            self.visit_binding_identifier(item);
        }
        self.visit_formal_parameters(&mut ast.params);
        self.visit_function_body(&mut ast.body);
    }

    fn visit_program(&mut self, ast: &mut Program<'alloc>) {
        match ast {
            Program::Module(ast) => {
                self.visit_module(ast);
            }
            Program::Script(ast) => {
                self.visit_script(ast);
            }
        }
    }

    fn visit_statement(&mut self, ast: &mut Statement<'alloc>) {
        match ast {
            Statement::BlockStatement { block, .. } => {
                self.visit_block(block);
            }
            Statement::BreakStatement { label, .. } => {
                if let Some(item) = label {
                    self.visit_label(item);
                }
            }
            Statement::ContinueStatement { label, .. } => {
                if let Some(item) = label {
                    self.visit_label(item);
                }
            }
            Statement::DebuggerStatement(_) => (),
            Statement::DoWhileStatement { block, test, .. } => {
                self.visit_statement(block);
                self.visit_expression(test);
            }
            Statement::EmptyStatement(_) => (),
            Statement::ExpressionStatement(ast) => {
                self.visit_expression(ast);
            }
            Statement::ForInStatement {
                left, right, block, ..
            } => {
                self.visit_variable_declaration_or_assignment_target(left);
                self.visit_expression(right);
                self.visit_statement(block);
            }
            Statement::ForOfStatement {
                left, right, block, ..
            } => {
                self.visit_variable_declaration_or_assignment_target(left);
                self.visit_expression(right);
                self.visit_statement(block);
            }
            Statement::ForStatement {
                init,
                test,
                update,
                block,
                ..
            } => {
                if let Some(item) = init {
                    self.visit_variable_declaration_or_expression(item);
                }
                if let Some(item) = test {
                    self.visit_expression(item);
                }
                if let Some(item) = update {
                    self.visit_expression(item);
                }
                self.visit_statement(block);
            }
            Statement::IfStatement {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.visit_expression(test);
                self.visit_statement(consequent);
                if let Some(item) = alternate {
                    self.visit_statement(item);
                }
            }
            Statement::LabeledStatement { label, body, .. } => {
                self.visit_label(label);
                self.visit_statement(body);
            }
            Statement::ReturnStatement { expression, .. } => {
                if let Some(item) = expression {
                    self.visit_expression(item);
                }
            }
            Statement::SwitchStatement {
                discriminant,
                cases,
                ..
            } => {
                self.visit_expression(discriminant);
                for item in cases {
                    self.visit_switch_case(item);
                }
            }
            Statement::SwitchStatementWithDefault {
                discriminant,
                pre_default_cases,
                default_case,
                post_default_cases,
                ..
            } => {
                self.visit_expression(discriminant);
                for item in pre_default_cases {
                    self.visit_switch_case(item);
                }
                self.visit_switch_default(default_case);
                for item in post_default_cases {
                    self.visit_switch_case(item);
                }
            }
            Statement::ThrowStatement { expression, .. } => {
                self.visit_expression(expression);
            }
            Statement::TryCatchStatement {
                body, catch_clause, ..
            } => {
                self.visit_block(body);
                self.visit_catch_clause(catch_clause);
            }
            Statement::TryFinallyStatement {
                body,
                catch_clause,
                finalizer,
                ..
            } => {
                self.visit_block(body);
                if let Some(item) = catch_clause {
                    self.visit_catch_clause(item);
                }
                self.visit_block(finalizer);
            }
            Statement::WhileStatement { test, block, .. } => {
                self.visit_expression(test);
                self.visit_statement(block);
            }
            Statement::WithStatement { object, body, .. } => {
                self.visit_expression(object);
                self.visit_statement(body);
            }
            Statement::VariableDeclarationStatement(ast) => {
                self.visit_variable_declaration(ast);
            }
            Statement::FunctionDeclaration(ast) => {
                self.visit_function(ast);
            }
            Statement::ClassDeclaration(ast) => {
                self.visit_class_declaration(ast);
            }
        }
    }

    fn visit_expression(&mut self, ast: &mut Expression<'alloc>) {
        match ast {
            Expression::MemberExpression(ast) => {
                self.visit_member_expression(ast);
            }
            Expression::ClassExpression(ast) => {
                self.visit_class_expression(ast);
            }
            Expression::LiteralBooleanExpression { value, .. } => {}
            Expression::LiteralInfinityExpression(_) => (),
            Expression::LiteralNullExpression(_) => (),
            Expression::LiteralNumericExpression { value, .. } => {}
            Expression::LiteralRegExpExpression {
                pattern,
                global,
                ignore_case,
                multi_line,
                sticky,
                unicode,
                ..
            } => {}
            Expression::LiteralStringExpression { value, .. } => {}
            Expression::ArrayExpression(ast) => {
                self.visit_array_expression(ast);
            }
            Expression::ArrowExpression {
                is_async,
                params,
                body,
                ..
            } => {
                self.visit_formal_parameters(params);
                self.visit_arrow_expression_body(body);
            }
            Expression::AssignmentExpression {
                binding,
                expression,
                ..
            } => {
                self.visit_assignment_target(binding);
                self.visit_expression(expression);
            }
            Expression::BinaryExpression {
                operator,
                left,
                right,
                ..
            } => {
                self.visit_binary_operator(operator);
                self.visit_expression(left);
                self.visit_expression(right);
            }
            Expression::CallExpression {
                callee, arguments, ..
            } => {
                self.visit_expression_or_super(callee);
                self.visit_arguments(arguments);
            }
            Expression::CompoundAssignmentExpression {
                operator,
                binding,
                expression,
                ..
            } => {
                self.visit_compound_assignment_operator(operator);
                self.visit_simple_assignment_target(binding);
                self.visit_expression(expression);
            }
            Expression::ConditionalExpression {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.visit_expression(test);
                self.visit_expression(consequent);
                self.visit_expression(alternate);
            }
            Expression::FunctionExpression(ast) => {
                self.visit_function(ast);
            }
            Expression::IdentifierExpression(ast) => {
                self.visit_identifier_expression(ast);
            }
            Expression::NewExpression {
                callee, arguments, ..
            } => {
                self.visit_expression(callee);
                self.visit_arguments(arguments);
            }
            Expression::NewTargetExpression(_) => (),
            Expression::ObjectExpression(ast) => {
                self.visit_object_expression(ast);
            }
            Expression::UnaryExpression {
                operator, operand, ..
            } => {
                self.visit_unary_operator(operator);
                self.visit_expression(operand);
            }
            Expression::TemplateExpression(ast) => {
                self.visit_template_expression(ast);
            }
            Expression::ThisExpression(_) => (),
            Expression::UpdateExpression {
                is_prefix,
                operator,
                operand,
                ..
            } => {
                self.visit_update_operator(operator);
                self.visit_simple_assignment_target(operand);
            }
            Expression::YieldExpression { expression, .. } => {
                if let Some(item) = expression {
                    self.visit_expression(item);
                }
            }
            Expression::YieldGeneratorExpression { expression, .. } => {
                self.visit_expression(expression);
            }
            Expression::AwaitExpression { expression, .. } => {
                self.visit_expression(expression);
            }
            Expression::ImportCallExpression { argument, .. } => {
                self.visit_expression(argument);
            }
        }
    }

    fn visit_member_expression(&mut self, ast: &mut MemberExpression<'alloc>) {
        match ast {
            MemberExpression::ComputedMemberExpression(ast) => {
                self.visit_computed_member_expression(ast);
            }
            MemberExpression::StaticMemberExpression(ast) => {
                self.visit_static_member_expression(ast);
            }
        }
    }

    fn visit_property_name(&mut self, ast: &mut PropertyName<'alloc>) {
        match ast {
            PropertyName::ComputedPropertyName(ast) => {
                self.visit_computed_property_name(ast);
            }
            PropertyName::StaticPropertyName(ast) => {
                self.visit_static_property_name(ast);
            }
        }
    }

    fn visit_object_property(&mut self, ast: &mut ObjectProperty<'alloc>) {
        match ast {
            ObjectProperty::NamedObjectProperty(ast) => {
                self.visit_named_object_property(ast);
            }
            ObjectProperty::ShorthandProperty(ast) => {
                self.visit_shorthand_property(ast);
            }
            ObjectProperty::SpreadProperty(ast) => {
                self.visit_expression(ast);
            }
        }
    }

    fn visit_named_object_property(&mut self, ast: &mut NamedObjectProperty<'alloc>) {
        match ast {
            NamedObjectProperty::MethodDefinition(ast) => {
                self.visit_method_definition(ast);
            }
            NamedObjectProperty::DataProperty(ast) => {
                self.visit_data_property(ast);
            }
        }
    }

    fn visit_method_definition(&mut self, ast: &mut MethodDefinition<'alloc>) {
        match ast {
            MethodDefinition::Method(ast) => {
                self.visit_method(ast);
            }
            MethodDefinition::Getter(ast) => {
                self.visit_getter(ast);
            }
            MethodDefinition::Setter(ast) => {
                self.visit_setter(ast);
            }
        }
    }

    fn visit_import_declaration(&mut self, ast: &mut ImportDeclaration<'alloc>) {
        match ast {
            ImportDeclaration::Import(ast) => {
                self.visit_import(ast);
            }
            ImportDeclaration::ImportNamespace(ast) => {
                self.visit_import_namespace(ast);
            }
        }
    }

    fn visit_export_declaration(&mut self, ast: &mut ExportDeclaration<'alloc>) {
        match ast {
            ExportDeclaration::ExportAllFrom(ast) => {
                self.visit_export_all_from(ast);
            }
            ExportDeclaration::ExportFrom(ast) => {
                self.visit_export_from(ast);
            }
            ExportDeclaration::ExportLocals(ast) => {
                self.visit_export_locals(ast);
            }
            ExportDeclaration::Export(ast) => {
                self.visit_export(ast);
            }
            ExportDeclaration::ExportDefault(ast) => {
                self.visit_export_default(ast);
            }
        }
    }

    fn visit_variable_reference(&mut self, ast: &mut VariableReference<'alloc>) {
        match ast {
            VariableReference::BindingIdentifier(ast) => {
                self.visit_binding_identifier(ast);
            }
            VariableReference::AssignmentTargetIdentifier(ast) => {
                self.visit_assignment_target_identifier(ast);
            }
        }
    }

    fn visit_binding_pattern(&mut self, ast: &mut BindingPattern<'alloc>) {
        match ast {
            BindingPattern::ObjectBinding(ast) => {
                self.visit_object_binding(ast);
            }
            BindingPattern::ArrayBinding(ast) => {
                self.visit_array_binding(ast);
            }
        }
    }

    fn visit_binding(&mut self, ast: &mut Binding<'alloc>) {
        match ast {
            Binding::BindingPattern(ast) => {
                self.visit_binding_pattern(ast);
            }
            Binding::BindingIdentifier(ast) => {
                self.visit_binding_identifier(ast);
            }
        }
    }

    fn visit_simple_assignment_target(&mut self, ast: &mut SimpleAssignmentTarget<'alloc>) {
        match ast {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(ast) => {
                self.visit_assignment_target_identifier(ast);
            }
            SimpleAssignmentTarget::MemberAssignmentTarget(ast) => {
                self.visit_member_assignment_target(ast);
            }
        }
    }

    fn visit_assignment_target_pattern(&mut self, ast: &mut AssignmentTargetPattern<'alloc>) {
        match ast {
            AssignmentTargetPattern::ArrayAssignmentTarget(ast) => {
                self.visit_array_assignment_target(ast);
            }
            AssignmentTargetPattern::ObjectAssignmentTarget(ast) => {
                self.visit_object_assignment_target(ast);
            }
        }
    }

    fn visit_assignment_target(&mut self, ast: &mut AssignmentTarget<'alloc>) {
        match ast {
            AssignmentTarget::AssignmentTargetPattern(ast) => {
                self.visit_assignment_target_pattern(ast);
            }
            AssignmentTarget::SimpleAssignmentTarget(ast) => {
                self.visit_simple_assignment_target(ast);
            }
        }
    }

    fn visit_parameter(&mut self, ast: &mut Parameter<'alloc>) {
        match ast {
            Parameter::Binding(ast) => {
                self.visit_binding(ast);
            }
            Parameter::BindingWithDefault(ast) => {
                self.visit_binding_with_default(ast);
            }
        }
    }

    fn visit_binding_with_default(&mut self, ast: &mut BindingWithDefault<'alloc>) {
        self.visit_binding(&mut ast.binding);
        self.visit_expression(&mut ast.init);
    }

    fn visit_binding_identifier(&mut self, ast: &mut BindingIdentifier<'alloc>) {
        self.visit_identifier(&mut ast.name);
    }

    fn visit_assignment_target_identifier(&mut self, ast: &mut AssignmentTargetIdentifier<'alloc>) {
        self.visit_identifier(&mut ast.name);
    }

    fn visit_expression_or_super(&mut self, ast: &mut ExpressionOrSuper<'alloc>) {
        match ast {
            ExpressionOrSuper::Expression(ast) => {
                self.visit_expression(ast);
            }
            ExpressionOrSuper::Super(_) => (),
        }
    }

    fn visit_member_assignment_target(&mut self, ast: &mut MemberAssignmentTarget<'alloc>) {
        match ast {
            MemberAssignmentTarget::ComputedMemberAssignmentTarget(ast) => {
                self.visit_computed_member_assignment_target(ast);
            }
            MemberAssignmentTarget::StaticMemberAssignmentTarget(ast) => {
                self.visit_static_member_assignment_target(ast);
            }
        }
    }

    fn visit_computed_member_assignment_target(
        &mut self,
        ast: &mut ComputedMemberAssignmentTarget<'alloc>,
    ) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_static_member_assignment_target(
        &mut self,
        ast: &mut StaticMemberAssignmentTarget<'alloc>,
    ) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_identifier_name(&mut ast.property);
    }

    fn visit_array_binding(&mut self, ast: &mut ArrayBinding<'alloc>) {
        for item in &mut ast.elements {
            if let Some(item) = item {
                self.visit_parameter(item);
            }
        }
        if let Some(item) = &mut ast.rest {
            self.visit_binding(item);
        }
    }

    fn visit_object_binding(&mut self, ast: &mut ObjectBinding<'alloc>) {
        for item in &mut ast.properties {
            self.visit_binding_property(item);
        }
        if let Some(item) = &mut ast.rest {
            self.visit_binding_identifier(item);
        }
    }

    fn visit_binding_property(&mut self, ast: &mut BindingProperty<'alloc>) {
        match ast {
            BindingProperty::BindingPropertyIdentifier(ast) => {
                self.visit_binding_property_identifier(ast);
            }
            BindingProperty::BindingPropertyProperty(ast) => {
                self.visit_binding_property_property(ast);
            }
        }
    }

    fn visit_binding_property_identifier(&mut self, ast: &mut BindingPropertyIdentifier<'alloc>) {
        self.visit_binding_identifier(&mut ast.binding);
        if let Some(item) = &mut ast.init {
            self.visit_expression(item);
        }
    }

    fn visit_binding_property_property(&mut self, ast: &mut BindingPropertyProperty<'alloc>) {
        self.visit_property_name(&mut ast.name);
        self.visit_parameter(&mut ast.binding);
    }

    fn visit_assignment_target_with_default(
        &mut self,
        ast: &mut AssignmentTargetWithDefault<'alloc>,
    ) {
        self.visit_assignment_target(&mut ast.binding);
        self.visit_expression(&mut ast.init);
    }

    fn visit_assignment_target_maybe_default(
        &mut self,
        ast: &mut AssignmentTargetMaybeDefault<'alloc>,
    ) {
        match ast {
            AssignmentTargetMaybeDefault::AssignmentTarget(ast) => {
                self.visit_assignment_target(ast);
            }
            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(ast) => {
                self.visit_assignment_target_with_default(ast);
            }
        }
    }

    fn visit_array_assignment_target(&mut self, ast: &mut ArrayAssignmentTarget<'alloc>) {
        for item in &mut ast.elements {
            if let Some(item) = item {
                self.visit_assignment_target_maybe_default(item);
            }
        }
        if let Some(item) = &mut ast.rest {
            self.visit_assignment_target(item);
        }
    }

    fn visit_object_assignment_target(&mut self, ast: &mut ObjectAssignmentTarget<'alloc>) {
        for item in &mut ast.properties {
            self.visit_assignment_target_property(item);
        }
        if let Some(item) = &mut ast.rest {
            self.visit_assignment_target(item);
        }
    }

    fn visit_assignment_target_property(&mut self, ast: &mut AssignmentTargetProperty<'alloc>) {
        match ast {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(ast) => {
                self.visit_assignment_target_property_identifier(ast);
            }
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(ast) => {
                self.visit_assignment_target_property_property(ast);
            }
        }
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        ast: &mut AssignmentTargetPropertyIdentifier<'alloc>,
    ) {
        self.visit_assignment_target_identifier(&mut ast.binding);
        if let Some(item) = &mut ast.init {
            self.visit_expression(item);
        }
    }

    fn visit_assignment_target_property_property(
        &mut self,
        ast: &mut AssignmentTargetPropertyProperty<'alloc>,
    ) {
        self.visit_property_name(&mut ast.name);
        self.visit_assignment_target_maybe_default(&mut ast.binding);
    }

    fn visit_class_expression(&mut self, ast: &mut ClassExpression<'alloc>) {
        if let Some(item) = &mut ast.name {
            self.visit_binding_identifier(item);
        }
        if let Some(item) = &mut ast.super_ {
            self.visit_expression(item);
        }
        for item in &mut ast.elements {
            self.visit_class_element(item);
        }
    }

    fn visit_class_declaration(&mut self, ast: &mut ClassDeclaration<'alloc>) {
        self.visit_binding_identifier(&mut ast.name);
        if let Some(item) = &mut ast.super_ {
            self.visit_expression(item);
        }
        for item in &mut ast.elements {
            self.visit_class_element(item);
        }
    }

    fn visit_class_element(&mut self, ast: &mut ClassElement<'alloc>) {
        self.visit_method_definition(&mut ast.method);
    }

    fn visit_module_items(&mut self, ast: &mut ModuleItems<'alloc>) {
        match ast {
            ModuleItems::ImportDeclaration(ast) => {
                self.visit_import_declaration(ast);
            }
            ModuleItems::ExportDeclaration(ast) => {
                self.visit_export_declaration(ast);
            }
            ModuleItems::Statement(ast) => {
                self.visit_statement(ast);
            }
        }
    }

    fn visit_module(&mut self, ast: &mut Module<'alloc>) {
        for item in &mut ast.directives {
            self.visit_directive(item);
        }
        for item in &mut ast.items {
            self.visit_module_items(item);
        }
    }

    fn visit_import(&mut self, ast: &mut Import<'alloc>) {
        if let Some(item) = &mut ast.default_binding {
            self.visit_binding_identifier(item);
        }
        for item in &mut ast.named_imports {
            self.visit_import_specifier(item);
        }
    }

    fn visit_import_namespace(&mut self, ast: &mut ImportNamespace<'alloc>) {
        if let Some(item) = &mut ast.default_binding {
            self.visit_binding_identifier(item);
        }
        self.visit_binding_identifier(&mut ast.namespace_binding);
    }

    fn visit_import_specifier(&mut self, ast: &mut ImportSpecifier<'alloc>) {
        if let Some(item) = &mut ast.name {
            self.visit_identifier_name(item);
        }
        self.visit_binding_identifier(&mut ast.binding);
    }

    fn visit_export_all_from(&mut self, ast: &mut ExportAllFrom<'alloc>) {}

    fn visit_export_from(&mut self, ast: &mut ExportFrom<'alloc>) {
        for item in &mut ast.named_exports {
            self.visit_export_from_specifier(item);
        }
    }

    fn visit_export_locals(&mut self, ast: &mut ExportLocals<'alloc>) {
        for item in &mut ast.named_exports {
            self.visit_export_local_specifier(item);
        }
    }

    fn visit_export(&mut self, ast: &mut Export<'alloc>) {
        match ast {
            Export::FunctionDeclaration(ast) => {
                self.visit_function(ast);
            }
            Export::ClassDeclaration(ast) => {
                self.visit_class_declaration(ast);
            }
            Export::VariableDeclaration(ast) => {
                self.visit_variable_declaration(ast);
            }
        }
    }

    fn visit_export_default(&mut self, ast: &mut ExportDefault<'alloc>) {
        match ast {
            ExportDefault::FunctionDeclaration(ast) => {
                self.visit_function(ast);
            }
            ExportDefault::ClassDeclaration(ast) => {
                self.visit_class_declaration(ast);
            }
            ExportDefault::Expression(ast) => {
                self.visit_expression(ast);
            }
        }
    }

    fn visit_export_from_specifier(&mut self, ast: &mut ExportFromSpecifier<'alloc>) {
        self.visit_identifier_name(&mut ast.name);
        if let Some(item) = &mut ast.exported_name {
            self.visit_identifier_name(item);
        }
    }

    fn visit_export_local_specifier(&mut self, ast: &mut ExportLocalSpecifier<'alloc>) {
        self.visit_identifier_expression(&mut ast.name);
        if let Some(item) = &mut ast.exported_name {
            self.visit_identifier_name(item);
        }
    }

    fn visit_method(&mut self, ast: &mut Method<'alloc>) {
        self.visit_property_name(&mut ast.name);
        self.visit_formal_parameters(&mut ast.params);
        self.visit_function_body(&mut ast.body);
    }

    fn visit_getter(&mut self, ast: &mut Getter<'alloc>) {
        self.visit_property_name(&mut ast.property_name);
        self.visit_function_body(&mut ast.body);
    }

    fn visit_setter(&mut self, ast: &mut Setter<'alloc>) {
        self.visit_property_name(&mut ast.property_name);
        self.visit_parameter(&mut ast.param);
        self.visit_function_body(&mut ast.body);
    }

    fn visit_data_property(&mut self, ast: &mut DataProperty<'alloc>) {
        self.visit_property_name(&mut ast.property_name);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_shorthand_property(&mut self, ast: &mut ShorthandProperty<'alloc>) {
        self.visit_identifier_expression(&mut ast.name);
    }

    fn visit_computed_property_name(&mut self, ast: &mut ComputedPropertyName<'alloc>) {
        self.visit_expression(&mut ast.expression);
    }

    fn visit_static_property_name(&mut self, ast: &mut StaticPropertyName<'alloc>) {}

    fn visit_array_expression_element(&mut self, ast: &mut ArrayExpressionElement<'alloc>) {
        match ast {
            ArrayExpressionElement::SpreadElement(ast) => {
                self.visit_expression(ast);
            }
            ArrayExpressionElement::Expression(ast) => {
                self.visit_expression(ast);
            }
            ArrayExpressionElement::Elision(_) => (),
        }
    }

    fn visit_array_expression(&mut self, ast: &mut ArrayExpression<'alloc>) {
        for item in &mut ast.elements {
            self.visit_array_expression_element(item);
        }
    }

    fn visit_arrow_expression_body(&mut self, ast: &mut ArrowExpressionBody<'alloc>) {
        match ast {
            ArrowExpressionBody::FunctionBody(ast) => {
                self.visit_function_body(ast);
            }
            ArrowExpressionBody::Expression(ast) => {
                self.visit_expression(ast);
            }
        }
    }

    fn visit_computed_member_expression(&mut self, ast: &mut ComputedMemberExpression<'alloc>) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_identifier_expression(&mut self, ast: &mut IdentifierExpression<'alloc>) {
        self.visit_identifier(&mut ast.name);
    }

    fn visit_object_expression(&mut self, ast: &mut ObjectExpression<'alloc>) {
        for item in &mut ast.properties {
            self.visit_object_property(item);
        }
    }

    fn visit_static_member_expression(&mut self, ast: &mut StaticMemberExpression<'alloc>) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_identifier_name(&mut ast.property);
    }

    fn visit_template_expression_element(&mut self, ast: &mut TemplateExpressionElement<'alloc>) {
        match ast {
            TemplateExpressionElement::Expression(ast) => {
                self.visit_expression(ast);
            }
            TemplateExpressionElement::TemplateElement(ast) => {
                self.visit_template_element(ast);
            }
        }
    }

    fn visit_template_expression(&mut self, ast: &mut TemplateExpression<'alloc>) {
        if let Some(item) = &mut ast.tag {
            self.visit_expression(item);
        }
        for item in &mut ast.elements {
            self.visit_template_expression_element(item);
        }
    }

    fn visit_variable_declaration_or_assignment_target(
        &mut self,
        ast: &mut VariableDeclarationOrAssignmentTarget<'alloc>,
    ) {
        match ast {
            VariableDeclarationOrAssignmentTarget::VariableDeclaration(ast) => {
                self.visit_variable_declaration(ast);
            }
            VariableDeclarationOrAssignmentTarget::AssignmentTarget(ast) => {
                self.visit_assignment_target(ast);
            }
        }
    }

    fn visit_variable_declaration_or_expression(
        &mut self,
        ast: &mut VariableDeclarationOrExpression<'alloc>,
    ) {
        match ast {
            VariableDeclarationOrExpression::VariableDeclaration(ast) => {
                self.visit_variable_declaration(ast);
            }
            VariableDeclarationOrExpression::Expression(ast) => {
                self.visit_expression(ast);
            }
        }
    }

    fn visit_block(&mut self, ast: &mut Block<'alloc>) {
        for item in &mut ast.statements {
            self.visit_statement(item);
        }
        if let Some(item) = &mut ast.declarations {
            for item in item {}
        }
    }

    fn visit_catch_clause(&mut self, ast: &mut CatchClause<'alloc>) {
        if let Some(item) = &mut ast.binding {
            self.visit_binding(item);
        }
        self.visit_block(&mut ast.body);
    }

    fn visit_directive(&mut self, ast: &mut Directive<'alloc>) {}

    fn visit_formal_parameters(&mut self, ast: &mut FormalParameters<'alloc>) {
        for item in &mut ast.items {
            self.visit_parameter(item);
        }
        if let Some(item) = &mut ast.rest {
            self.visit_binding(item);
        }
    }

    fn visit_function_body(&mut self, ast: &mut FunctionBody<'alloc>) {
        for item in &mut ast.directives {
            self.visit_directive(item);
        }
        for item in &mut ast.statements {
            self.visit_statement(item);
        }
    }

    fn visit_script(&mut self, ast: &mut Script<'alloc>) {
        for item in &mut ast.directives {
            self.visit_directive(item);
        }
        for item in &mut ast.statements {
            self.visit_statement(item);
        }
    }

    fn visit_switch_case(&mut self, ast: &mut SwitchCase<'alloc>) {
        self.visit_expression(&mut ast.test);
        for item in &mut ast.consequent {
            self.visit_statement(item);
        }
    }

    fn visit_switch_default(&mut self, ast: &mut SwitchDefault<'alloc>) {
        for item in &mut ast.consequent {
            self.visit_statement(item);
        }
    }

    fn visit_template_element(&mut self, ast: &mut TemplateElement<'alloc>) {}

    fn visit_variable_declaration(&mut self, ast: &mut VariableDeclaration<'alloc>) {
        self.visit_variable_declaration_kind(&mut ast.kind);
        for item in &mut ast.declarators {
            self.visit_variable_declarator(item);
        }
    }

    fn visit_variable_declarator(&mut self, ast: &mut VariableDeclarator<'alloc>) {
        self.visit_binding(&mut ast.binding);
        if let Some(item) = &mut ast.init {
            self.visit_expression(item);
        }
    }

    fn visit_cover_parenthesized(&mut self, ast: &mut CoverParenthesized<'alloc>) {
        match ast {
            CoverParenthesized::Expression { expression, .. } => {
                self.visit_expression(expression);
            }
            CoverParenthesized::Parameters(ast) => {
                self.visit_formal_parameters(ast);
            }
        }
    }
}

pub trait PostfixPassMonoid: Default {
    fn append(&mut self, other: Self);
}

pub trait PostfixPass<'alloc> {
    type Value: PostfixPassMonoid + 'alloc;
    fn visit_arguments(&self, args: arena::Vec<'alloc, Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        for item in args {
            result.append(item);
        }
        result
    }

    fn visit_identifier(&self, value: &mut &'alloc str) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_identifier_name(&self, value: &mut &'alloc str) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_label(&self, value: &mut &'alloc str) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_function(
        &self,
        name: Option<Self::Value>,
        is_async: &mut bool,
        is_generator: &mut bool,
        params: Self::Value,
        body: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = name {
            result.append(item);
        }
        result.append(params);
        result.append(body);
        result
    }

    fn visit_block_statement(&self, block: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(block);
        result
    }

    fn visit_break_statement(&self, label: Option<Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = label {
            result.append(item);
        }
        result
    }

    fn visit_continue_statement(&self, label: Option<Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = label {
            result.append(item);
        }
        result
    }

    fn visit_do_while_statement(&self, block: Self::Value, test: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(block);
        result.append(test);
        result
    }

    fn visit_for_in_statement(
        &self,
        left: Self::Value,
        right: Self::Value,
        block: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(left);
        result.append(right);
        result.append(block);
        result
    }

    fn visit_for_of_statement(
        &self,
        left: Self::Value,
        right: Self::Value,
        block: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(left);
        result.append(right);
        result.append(block);
        result
    }

    fn visit_for_statement(
        &self,
        init: Option<Self::Value>,
        test: Option<Self::Value>,
        update: Option<Self::Value>,
        block: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = init {
            result.append(item);
        }
        if let Some(item) = test {
            result.append(item);
        }
        if let Some(item) = update {
            result.append(item);
        }
        result.append(block);
        result
    }

    fn visit_if_statement(
        &self,
        test: Self::Value,
        consequent: Self::Value,
        alternate: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(test);
        result.append(consequent);
        if let Some(item) = alternate {
            result.append(item);
        }
        result
    }

    fn visit_labeled_statement(&self, label: Self::Value, body: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(label);
        result.append(body);
        result
    }

    fn visit_return_statement(&self, expression: Option<Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = expression {
            result.append(item);
        }
        result
    }

    fn visit_switch_statement(
        &self,
        discriminant: Self::Value,
        cases: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(discriminant);
        for item in cases {
            result.append(item);
        }
        result
    }

    fn visit_switch_statement_with_default(
        &self,
        discriminant: Self::Value,
        pre_default_cases: arena::Vec<'alloc, Self::Value>,
        default_case: Self::Value,
        post_default_cases: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(discriminant);
        for item in pre_default_cases {
            result.append(item);
        }
        result.append(default_case);
        for item in post_default_cases {
            result.append(item);
        }
        result
    }

    fn visit_throw_statement(&self, expression: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(expression);
        result
    }

    fn visit_try_catch_statement(
        &self,
        body: Self::Value,
        catch_clause: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(body);
        result.append(catch_clause);
        result
    }

    fn visit_try_finally_statement(
        &self,
        body: Self::Value,
        catch_clause: Option<Self::Value>,
        finalizer: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(body);
        if let Some(item) = catch_clause {
            result.append(item);
        }
        result.append(finalizer);
        result
    }

    fn visit_while_statement(&self, test: Self::Value, block: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(test);
        result.append(block);
        result
    }

    fn visit_with_statement(&self, object: Self::Value, body: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(object);
        result.append(body);
        result
    }

    fn visit_literal_boolean_expression(&self, value: &mut bool) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_literal_numeric_expression(&self, value: &mut f64) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_literal_reg_exp_expression(
        &self,
        pattern: &mut &'alloc str,
        global: &mut bool,
        ignore_case: &mut bool,
        multi_line: &mut bool,
        sticky: &mut bool,
        unicode: &mut bool,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_literal_string_expression(&self, value: &mut &'alloc str) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_arrow_expression(
        &self,
        is_async: &mut bool,
        params: Self::Value,
        body: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(params);
        result.append(body);
        result
    }

    fn visit_assignment_expression(
        &self,
        binding: Self::Value,
        expression: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(binding);
        result.append(expression);
        result
    }

    fn visit_binary_expression(
        &self,
        operator: Self::Value,
        left: Self::Value,
        right: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(operator);
        result.append(left);
        result.append(right);
        result
    }

    fn visit_call_expression(&self, callee: Self::Value, arguments: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(callee);
        result.append(arguments);
        result
    }

    fn visit_compound_assignment_expression(
        &self,
        operator: Self::Value,
        binding: Self::Value,
        expression: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(operator);
        result.append(binding);
        result.append(expression);
        result
    }

    fn visit_conditional_expression(
        &self,
        test: Self::Value,
        consequent: Self::Value,
        alternate: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(test);
        result.append(consequent);
        result.append(alternate);
        result
    }

    fn visit_new_expression(&self, callee: Self::Value, arguments: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(callee);
        result.append(arguments);
        result
    }

    fn visit_unary_expression(&self, operator: Self::Value, operand: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(operator);
        result.append(operand);
        result
    }

    fn visit_update_expression(
        &self,
        is_prefix: &mut bool,
        operator: Self::Value,
        operand: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(operator);
        result.append(operand);
        result
    }

    fn visit_yield_expression(&self, expression: Option<Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = expression {
            result.append(item);
        }
        result
    }

    fn visit_yield_generator_expression(&self, expression: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(expression);
        result
    }

    fn visit_await_expression(&self, expression: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(expression);
        result
    }

    fn visit_import_call_expression(&self, argument: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(argument);
        result
    }

    fn visit_binding_with_default(&self, binding: Self::Value, init: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(binding);
        result.append(init);
        result
    }

    fn visit_binding_identifier(&self, name: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        result
    }

    fn visit_assignment_target_identifier(&self, name: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        result
    }

    fn visit_computed_member_assignment_target(
        &self,
        object: Self::Value,
        expression: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(object);
        result.append(expression);
        result
    }

    fn visit_static_member_assignment_target(
        &self,
        object: Self::Value,
        property: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(object);
        result.append(property);
        result
    }

    fn visit_array_binding(
        &self,
        elements: arena::Vec<'alloc, Option<Self::Value>>,
        rest: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in elements {
            if let Some(item) = item {
                result.append(item);
            }
        }
        if let Some(item) = rest {
            result.append(item);
        }
        result
    }

    fn visit_object_binding(
        &self,
        properties: arena::Vec<'alloc, Self::Value>,
        rest: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in properties {
            result.append(item);
        }
        if let Some(item) = rest {
            result.append(item);
        }
        result
    }

    fn visit_binding_property_identifier(
        &self,
        binding: Self::Value,
        init: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(binding);
        if let Some(item) = init {
            result.append(item);
        }
        result
    }

    fn visit_binding_property_property(
        &self,
        name: Self::Value,
        binding: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        result.append(binding);
        result
    }

    fn visit_assignment_target_with_default(
        &self,
        binding: Self::Value,
        init: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(binding);
        result.append(init);
        result
    }

    fn visit_array_assignment_target(
        &self,
        elements: arena::Vec<'alloc, Option<Self::Value>>,
        rest: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in elements {
            if let Some(item) = item {
                result.append(item);
            }
        }
        if let Some(item) = rest {
            result.append(item);
        }
        result
    }

    fn visit_object_assignment_target(
        &self,
        properties: arena::Vec<'alloc, Self::Value>,
        rest: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in properties {
            result.append(item);
        }
        if let Some(item) = rest {
            result.append(item);
        }
        result
    }

    fn visit_assignment_target_property_identifier(
        &self,
        binding: Self::Value,
        init: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(binding);
        if let Some(item) = init {
            result.append(item);
        }
        result
    }

    fn visit_assignment_target_property_property(
        &self,
        name: Self::Value,
        binding: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        result.append(binding);
        result
    }

    fn visit_class_expression(
        &self,
        name: Option<Self::Value>,
        super_: Option<Self::Value>,
        elements: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = name {
            result.append(item);
        }
        if let Some(item) = super_ {
            result.append(item);
        }
        for item in elements {
            result.append(item);
        }
        result
    }

    fn visit_class_declaration(
        &self,
        name: Self::Value,
        super_: Option<Self::Value>,
        elements: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        if let Some(item) = super_ {
            result.append(item);
        }
        for item in elements {
            result.append(item);
        }
        result
    }

    fn visit_class_element(&self, is_static: &mut bool, method: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(method);
        result
    }

    fn visit_module(
        &self,
        directives: arena::Vec<'alloc, Self::Value>,
        items: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in directives {
            result.append(item);
        }
        for item in items {
            result.append(item);
        }
        result
    }

    fn visit_import(
        &self,
        module_specifier: &mut &'alloc str,
        default_binding: Option<Self::Value>,
        named_imports: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = default_binding {
            result.append(item);
        }
        for item in named_imports {
            result.append(item);
        }
        result
    }

    fn visit_import_namespace(
        &self,
        module_specifier: &mut &'alloc str,
        default_binding: Option<Self::Value>,
        namespace_binding: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = default_binding {
            result.append(item);
        }
        result.append(namespace_binding);
        result
    }

    fn visit_import_specifier(
        &self,
        name: Option<Self::Value>,
        binding: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = name {
            result.append(item);
        }
        result.append(binding);
        result
    }

    fn visit_export_all_from(&self, module_specifier: &mut &'alloc str) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_export_from(
        &self,
        named_exports: arena::Vec<'alloc, Self::Value>,
        module_specifier: &mut &'alloc str,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in named_exports {
            result.append(item);
        }
        result
    }

    fn visit_export_locals(&self, named_exports: arena::Vec<'alloc, Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        for item in named_exports {
            result.append(item);
        }
        result
    }

    fn visit_export_from_specifier(
        &self,
        name: Self::Value,
        exported_name: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        if let Some(item) = exported_name {
            result.append(item);
        }
        result
    }

    fn visit_export_local_specifier(
        &self,
        name: Self::Value,
        exported_name: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        if let Some(item) = exported_name {
            result.append(item);
        }
        result
    }

    fn visit_method(
        &self,
        name: Self::Value,
        is_async: &mut bool,
        is_generator: &mut bool,
        params: Self::Value,
        body: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        result.append(params);
        result.append(body);
        result
    }

    fn visit_getter(&self, property_name: Self::Value, body: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(property_name);
        result.append(body);
        result
    }

    fn visit_setter(
        &self,
        property_name: Self::Value,
        param: Self::Value,
        body: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(property_name);
        result.append(param);
        result.append(body);
        result
    }

    fn visit_data_property(
        &self,
        property_name: Self::Value,
        expression: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(property_name);
        result.append(expression);
        result
    }

    fn visit_shorthand_property(&self, name: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        result
    }

    fn visit_computed_property_name(&self, expression: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(expression);
        result
    }

    fn visit_static_property_name(&self, value: &mut &'alloc str) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_array_expression(&self, elements: arena::Vec<'alloc, Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        for item in elements {
            result.append(item);
        }
        result
    }

    fn visit_computed_member_expression(
        &self,
        object: Self::Value,
        expression: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(object);
        result.append(expression);
        result
    }

    fn visit_identifier_expression(&self, name: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        result
    }

    fn visit_object_expression(&self, properties: arena::Vec<'alloc, Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        for item in properties {
            result.append(item);
        }
        result
    }

    fn visit_static_member_expression(
        &self,
        object: Self::Value,
        property: Self::Value,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(object);
        result.append(property);
        result
    }

    fn visit_template_expression(
        &self,
        tag: Option<Self::Value>,
        elements: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = tag {
            result.append(item);
        }
        for item in elements {
            result.append(item);
        }
        result
    }

    fn visit_block(
        &self,
        statements: arena::Vec<'alloc, Self::Value>,
        declarations: &mut Option<arena::Vec<'alloc, &'alloc str>>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in statements {
            result.append(item);
        }
        if let Some(item) = declarations {
            for item in item {}
        }
        result
    }

    fn visit_catch_clause(&self, binding: Option<Self::Value>, body: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        if let Some(item) = binding {
            result.append(item);
        }
        result.append(body);
        result
    }

    fn visit_directive(&self, raw_value: &mut &'alloc str) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_formal_parameters(
        &self,
        items: arena::Vec<'alloc, Self::Value>,
        rest: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in items {
            result.append(item);
        }
        if let Some(item) = rest {
            result.append(item);
        }
        result
    }

    fn visit_function_body(
        &self,
        directives: arena::Vec<'alloc, Self::Value>,
        statements: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in directives {
            result.append(item);
        }
        for item in statements {
            result.append(item);
        }
        result
    }

    fn visit_script(
        &self,
        directives: arena::Vec<'alloc, Self::Value>,
        statements: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        for item in directives {
            result.append(item);
        }
        for item in statements {
            result.append(item);
        }
        result
    }

    fn visit_switch_case(
        &self,
        test: Self::Value,
        consequent: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(test);
        for item in consequent {
            result.append(item);
        }
        result
    }

    fn visit_switch_default(&self, consequent: arena::Vec<'alloc, Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        for item in consequent {
            result.append(item);
        }
        result
    }

    fn visit_template_element(&self, raw_value: &mut &'alloc str) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_variable_declaration(
        &self,
        kind: Self::Value,
        declarators: arena::Vec<'alloc, Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(kind);
        for item in declarators {
            result.append(item);
        }
        result
    }

    fn visit_variable_declarator(
        &self,
        binding: Self::Value,
        init: Option<Self::Value>,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(binding);
        if let Some(item) = init {
            result.append(item);
        }
        result
    }

    fn visit_expression(&self, expression: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(expression);
        result
    }
}

pub struct PostfixPassVisitor<'alloc, T: PostfixPass<'alloc>> {
    allocator: &'alloc bumpalo::Bump,
    pass: T,
}

impl<'alloc, T: PostfixPass<'alloc>> PostfixPassVisitor<'alloc, T> {
    pub fn new(allocator: &'alloc bumpalo::Bump, pass: T) -> Self {
        Self { allocator, pass }
    }

    pub fn visit_argument(&mut self, ast: &mut Argument<'alloc>) -> T::Value {
        match ast {
            Argument::SpreadElement(ast) => self.visit_expression(ast),
            Argument::Expression(ast) => self.visit_expression(ast),
        }
    }

    pub fn visit_arguments(&mut self, ast: &mut Arguments<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec((&mut ast.args), |item| self.visit_argument(item), allocator)
        };
        self.pass.visit_arguments(a0)
    }

    pub fn visit_identifier(&mut self, ast: &mut Identifier<'alloc>) -> T::Value {
        let a0 = &mut ast.value;
        self.pass.visit_identifier(a0)
    }

    pub fn visit_identifier_name(&mut self, ast: &mut IdentifierName<'alloc>) -> T::Value {
        let a0 = &mut ast.value;
        self.pass.visit_identifier_name(a0)
    }

    pub fn visit_label(&mut self, ast: &mut Label<'alloc>) -> T::Value {
        let a0 = &mut ast.value;
        self.pass.visit_label(a0)
    }

    pub fn visit_variable_declaration_kind(
        &mut self,
        ast: &mut VariableDeclarationKind,
    ) -> T::Value {
        match ast {
            VariableDeclarationKind::Var(_) => T::Value::default(),
            VariableDeclarationKind::Let(_) => T::Value::default(),
            VariableDeclarationKind::Const(_) => T::Value::default(),
        }
    }

    pub fn visit_compound_assignment_operator(
        &mut self,
        ast: &mut CompoundAssignmentOperator,
    ) -> T::Value {
        match ast {
            CompoundAssignmentOperator::Add(_) => T::Value::default(),
            CompoundAssignmentOperator::Sub(_) => T::Value::default(),
            CompoundAssignmentOperator::Mul(_) => T::Value::default(),
            CompoundAssignmentOperator::Div(_) => T::Value::default(),
            CompoundAssignmentOperator::Mod(_) => T::Value::default(),
            CompoundAssignmentOperator::Pow(_) => T::Value::default(),
            CompoundAssignmentOperator::LeftShift(_) => T::Value::default(),
            CompoundAssignmentOperator::RightShift(_) => T::Value::default(),
            CompoundAssignmentOperator::RightShiftExt(_) => T::Value::default(),
            CompoundAssignmentOperator::Or(_) => T::Value::default(),
            CompoundAssignmentOperator::Xor(_) => T::Value::default(),
            CompoundAssignmentOperator::And(_) => T::Value::default(),
        }
    }

    pub fn visit_binary_operator(&mut self, ast: &mut BinaryOperator) -> T::Value {
        match ast {
            BinaryOperator::Equals(_) => T::Value::default(),
            BinaryOperator::NotEquals(_) => T::Value::default(),
            BinaryOperator::StrictEquals(_) => T::Value::default(),
            BinaryOperator::StrictNotEquals(_) => T::Value::default(),
            BinaryOperator::LessThan(_) => T::Value::default(),
            BinaryOperator::LessThanOrEqual(_) => T::Value::default(),
            BinaryOperator::GreaterThan(_) => T::Value::default(),
            BinaryOperator::GreaterThanOrEqual(_) => T::Value::default(),
            BinaryOperator::In(_) => T::Value::default(),
            BinaryOperator::Instanceof(_) => T::Value::default(),
            BinaryOperator::LeftShift(_) => T::Value::default(),
            BinaryOperator::RightShift(_) => T::Value::default(),
            BinaryOperator::RightShiftExt(_) => T::Value::default(),
            BinaryOperator::Add(_) => T::Value::default(),
            BinaryOperator::Sub(_) => T::Value::default(),
            BinaryOperator::Mul(_) => T::Value::default(),
            BinaryOperator::Div(_) => T::Value::default(),
            BinaryOperator::Mod(_) => T::Value::default(),
            BinaryOperator::Pow(_) => T::Value::default(),
            BinaryOperator::Comma(_) => T::Value::default(),
            BinaryOperator::Coalesce(_) => T::Value::default(),
            BinaryOperator::LogicalOr(_) => T::Value::default(),
            BinaryOperator::LogicalAnd(_) => T::Value::default(),
            BinaryOperator::BitwiseOr(_) => T::Value::default(),
            BinaryOperator::BitwiseXor(_) => T::Value::default(),
            BinaryOperator::BitwiseAnd(_) => T::Value::default(),
        }
    }

    pub fn visit_unary_operator(&mut self, ast: &mut UnaryOperator) -> T::Value {
        match ast {
            UnaryOperator::Plus(_) => T::Value::default(),
            UnaryOperator::Minus(_) => T::Value::default(),
            UnaryOperator::LogicalNot(_) => T::Value::default(),
            UnaryOperator::BitwiseNot(_) => T::Value::default(),
            UnaryOperator::Typeof(_) => T::Value::default(),
            UnaryOperator::Void(_) => T::Value::default(),
            UnaryOperator::Delete(_) => T::Value::default(),
        }
    }

    pub fn visit_update_operator(&mut self, ast: &mut UpdateOperator) -> T::Value {
        match ast {
            UpdateOperator::Increment(_) => T::Value::default(),
            UpdateOperator::Decrement(_) => T::Value::default(),
        }
    }

    pub fn visit_function(&mut self, ast: &mut Function<'alloc>) -> T::Value {
        let a0 = (&mut ast.name)
            .as_mut()
            .map(|item| self.visit_binding_identifier(item));
        let a1 = &mut ast.is_async;
        let a2 = &mut ast.is_generator;
        let a3 = self.visit_formal_parameters((&mut ast.params));
        let a4 = self.visit_function_body((&mut ast.body));
        self.pass.visit_function(a0, a1, a2, a3, a4)
    }

    pub fn visit_program(&mut self, ast: &mut Program<'alloc>) -> T::Value {
        match ast {
            Program::Module(ast) => self.visit_module(ast),
            Program::Script(ast) => self.visit_script(ast),
        }
    }

    pub fn visit_statement(&mut self, ast: &mut Statement<'alloc>) -> T::Value {
        match ast {
            Statement::BlockStatement { block, .. } => {
                let a0 = self.visit_block((block));
                self.pass.visit_block_statement(a0)
            }
            Statement::BreakStatement { label, .. } => {
                let a0 = (label).as_mut().map(|item| self.visit_label(item));
                self.pass.visit_break_statement(a0)
            }
            Statement::ContinueStatement { label, .. } => {
                let a0 = (label).as_mut().map(|item| self.visit_label(item));
                self.pass.visit_continue_statement(a0)
            }
            Statement::DebuggerStatement(_) => T::Value::default(),
            Statement::DoWhileStatement { block, test, .. } => {
                let a0 = self.visit_statement((block));
                let a1 = self.visit_expression((test));
                self.pass.visit_do_while_statement(a0, a1)
            }
            Statement::EmptyStatement(_) => T::Value::default(),
            Statement::ExpressionStatement(ast) => self.visit_expression(ast),
            Statement::ForInStatement {
                left, right, block, ..
            } => {
                let a0 = self.visit_variable_declaration_or_assignment_target((left));
                let a1 = self.visit_expression((right));
                let a2 = self.visit_statement((block));
                self.pass.visit_for_in_statement(a0, a1, a2)
            }
            Statement::ForOfStatement {
                left, right, block, ..
            } => {
                let a0 = self.visit_variable_declaration_or_assignment_target((left));
                let a1 = self.visit_expression((right));
                let a2 = self.visit_statement((block));
                self.pass.visit_for_of_statement(a0, a1, a2)
            }
            Statement::ForStatement {
                init,
                test,
                update,
                block,
                ..
            } => {
                let a0 = (init)
                    .as_mut()
                    .map(|item| self.visit_variable_declaration_or_expression(item));
                let a1 = (test).as_mut().map(|item| self.visit_expression(item));
                let a2 = (update).as_mut().map(|item| self.visit_expression(item));
                let a3 = self.visit_statement((block));
                self.pass.visit_for_statement(a0, a1, a2, a3)
            }
            Statement::IfStatement {
                test,
                consequent,
                alternate,
                ..
            } => {
                let a0 = self.visit_expression((test));
                let a1 = self.visit_statement((consequent));
                let a2 = (alternate).as_mut().map(|item| self.visit_statement(item));
                self.pass.visit_if_statement(a0, a1, a2)
            }
            Statement::LabeledStatement { label, body, .. } => {
                let a0 = self.visit_label((label));
                let a1 = self.visit_statement((body));
                self.pass.visit_labeled_statement(a0, a1)
            }
            Statement::ReturnStatement { expression, .. } => {
                let a0 = (expression)
                    .as_mut()
                    .map(|item| self.visit_expression(item));
                self.pass.visit_return_statement(a0)
            }
            Statement::SwitchStatement {
                discriminant,
                cases,
                ..
            } => {
                let a0 = self.visit_expression((discriminant));
                let a1 = {
                    let allocator: &bumpalo::Bump = self.allocator;
                    arena::map_vec((cases), |item| self.visit_switch_case(item), allocator)
                };
                self.pass.visit_switch_statement(a0, a1)
            }
            Statement::SwitchStatementWithDefault {
                discriminant,
                pre_default_cases,
                default_case,
                post_default_cases,
                ..
            } => {
                let a0 = self.visit_expression((discriminant));
                let a1 = {
                    let allocator: &bumpalo::Bump = self.allocator;
                    arena::map_vec(
                        (pre_default_cases),
                        |item| self.visit_switch_case(item),
                        allocator,
                    )
                };
                let a2 = self.visit_switch_default((default_case));
                let a3 = {
                    let allocator: &bumpalo::Bump = self.allocator;
                    arena::map_vec(
                        (post_default_cases),
                        |item| self.visit_switch_case(item),
                        allocator,
                    )
                };
                self.pass
                    .visit_switch_statement_with_default(a0, a1, a2, a3)
            }
            Statement::ThrowStatement { expression, .. } => {
                let a0 = self.visit_expression((expression));
                self.pass.visit_throw_statement(a0)
            }
            Statement::TryCatchStatement {
                body, catch_clause, ..
            } => {
                let a0 = self.visit_block((body));
                let a1 = self.visit_catch_clause((catch_clause));
                self.pass.visit_try_catch_statement(a0, a1)
            }
            Statement::TryFinallyStatement {
                body,
                catch_clause,
                finalizer,
                ..
            } => {
                let a0 = self.visit_block((body));
                let a1 = (catch_clause)
                    .as_mut()
                    .map(|item| self.visit_catch_clause(item));
                let a2 = self.visit_block((finalizer));
                self.pass.visit_try_finally_statement(a0, a1, a2)
            }
            Statement::WhileStatement { test, block, .. } => {
                let a0 = self.visit_expression((test));
                let a1 = self.visit_statement((block));
                self.pass.visit_while_statement(a0, a1)
            }
            Statement::WithStatement { object, body, .. } => {
                let a0 = self.visit_expression((object));
                let a1 = self.visit_statement((body));
                self.pass.visit_with_statement(a0, a1)
            }
            Statement::VariableDeclarationStatement(ast) => self.visit_variable_declaration(ast),
            Statement::FunctionDeclaration(ast) => self.visit_function(ast),
            Statement::ClassDeclaration(ast) => self.visit_class_declaration(ast),
        }
    }

    pub fn visit_expression(&mut self, ast: &mut Expression<'alloc>) -> T::Value {
        match ast {
            Expression::MemberExpression(ast) => self.visit_member_expression(ast),
            Expression::ClassExpression(ast) => self.visit_class_expression(ast),
            Expression::LiteralBooleanExpression { value, .. } => {
                let a0 = value;
                self.pass.visit_literal_boolean_expression(a0)
            }
            Expression::LiteralInfinityExpression(_) => T::Value::default(),
            Expression::LiteralNullExpression(_) => T::Value::default(),
            Expression::LiteralNumericExpression { value, .. } => {
                let a0 = value;
                self.pass.visit_literal_numeric_expression(a0)
            }
            Expression::LiteralRegExpExpression {
                pattern,
                global,
                ignore_case,
                multi_line,
                sticky,
                unicode,
                ..
            } => {
                let a0 = pattern;
                let a1 = global;
                let a2 = ignore_case;
                let a3 = multi_line;
                let a4 = sticky;
                let a5 = unicode;
                self.pass
                    .visit_literal_reg_exp_expression(a0, a1, a2, a3, a4, a5)
            }
            Expression::LiteralStringExpression { value, .. } => {
                let a0 = value;
                self.pass.visit_literal_string_expression(a0)
            }
            Expression::ArrayExpression(ast) => self.visit_array_expression(ast),
            Expression::ArrowExpression {
                is_async,
                params,
                body,
                ..
            } => {
                let a0 = is_async;
                let a1 = self.visit_formal_parameters((params));
                let a2 = self.visit_arrow_expression_body((body));
                self.pass.visit_arrow_expression(a0, a1, a2)
            }
            Expression::AssignmentExpression {
                binding,
                expression,
                ..
            } => {
                let a0 = self.visit_assignment_target((binding));
                let a1 = self.visit_expression((expression));
                self.pass.visit_assignment_expression(a0, a1)
            }
            Expression::BinaryExpression {
                operator,
                left,
                right,
                ..
            } => {
                let a0 = self.visit_binary_operator((operator));
                let a1 = self.visit_expression((left));
                let a2 = self.visit_expression((right));
                self.pass.visit_binary_expression(a0, a1, a2)
            }
            Expression::CallExpression {
                callee, arguments, ..
            } => {
                let a0 = self.visit_expression_or_super((callee));
                let a1 = self.visit_arguments((arguments));
                self.pass.visit_call_expression(a0, a1)
            }
            Expression::CompoundAssignmentExpression {
                operator,
                binding,
                expression,
                ..
            } => {
                let a0 = self.visit_compound_assignment_operator((operator));
                let a1 = self.visit_simple_assignment_target((binding));
                let a2 = self.visit_expression((expression));
                self.pass.visit_compound_assignment_expression(a0, a1, a2)
            }
            Expression::ConditionalExpression {
                test,
                consequent,
                alternate,
                ..
            } => {
                let a0 = self.visit_expression((test));
                let a1 = self.visit_expression((consequent));
                let a2 = self.visit_expression((alternate));
                self.pass.visit_conditional_expression(a0, a1, a2)
            }
            Expression::FunctionExpression(ast) => self.visit_function(ast),
            Expression::IdentifierExpression(ast) => self.visit_identifier_expression(ast),
            Expression::NewExpression {
                callee, arguments, ..
            } => {
                let a0 = self.visit_expression((callee));
                let a1 = self.visit_arguments((arguments));
                self.pass.visit_new_expression(a0, a1)
            }
            Expression::NewTargetExpression(_) => T::Value::default(),
            Expression::ObjectExpression(ast) => self.visit_object_expression(ast),
            Expression::UnaryExpression {
                operator, operand, ..
            } => {
                let a0 = self.visit_unary_operator((operator));
                let a1 = self.visit_expression((operand));
                self.pass.visit_unary_expression(a0, a1)
            }
            Expression::TemplateExpression(ast) => self.visit_template_expression(ast),
            Expression::ThisExpression(_) => T::Value::default(),
            Expression::UpdateExpression {
                is_prefix,
                operator,
                operand,
                ..
            } => {
                let a0 = is_prefix;
                let a1 = self.visit_update_operator((operator));
                let a2 = self.visit_simple_assignment_target((operand));
                self.pass.visit_update_expression(a0, a1, a2)
            }
            Expression::YieldExpression { expression, .. } => {
                let a0 = (expression)
                    .as_mut()
                    .map(|item| self.visit_expression(item));
                self.pass.visit_yield_expression(a0)
            }
            Expression::YieldGeneratorExpression { expression, .. } => {
                let a0 = self.visit_expression((expression));
                self.pass.visit_yield_generator_expression(a0)
            }
            Expression::AwaitExpression { expression, .. } => {
                let a0 = self.visit_expression((expression));
                self.pass.visit_await_expression(a0)
            }
            Expression::ImportCallExpression { argument, .. } => {
                let a0 = self.visit_expression((argument));
                self.pass.visit_import_call_expression(a0)
            }
        }
    }

    pub fn visit_member_expression(&mut self, ast: &mut MemberExpression<'alloc>) -> T::Value {
        match ast {
            MemberExpression::ComputedMemberExpression(ast) => {
                self.visit_computed_member_expression(ast)
            }
            MemberExpression::StaticMemberExpression(ast) => {
                self.visit_static_member_expression(ast)
            }
        }
    }

    pub fn visit_property_name(&mut self, ast: &mut PropertyName<'alloc>) -> T::Value {
        match ast {
            PropertyName::ComputedPropertyName(ast) => self.visit_computed_property_name(ast),
            PropertyName::StaticPropertyName(ast) => self.visit_static_property_name(ast),
        }
    }

    pub fn visit_object_property(&mut self, ast: &mut ObjectProperty<'alloc>) -> T::Value {
        match ast {
            ObjectProperty::NamedObjectProperty(ast) => self.visit_named_object_property(ast),
            ObjectProperty::ShorthandProperty(ast) => self.visit_shorthand_property(ast),
            ObjectProperty::SpreadProperty(ast) => self.visit_expression(ast),
        }
    }

    pub fn visit_named_object_property(
        &mut self,
        ast: &mut NamedObjectProperty<'alloc>,
    ) -> T::Value {
        match ast {
            NamedObjectProperty::MethodDefinition(ast) => self.visit_method_definition(ast),
            NamedObjectProperty::DataProperty(ast) => self.visit_data_property(ast),
        }
    }

    pub fn visit_method_definition(&mut self, ast: &mut MethodDefinition<'alloc>) -> T::Value {
        match ast {
            MethodDefinition::Method(ast) => self.visit_method(ast),
            MethodDefinition::Getter(ast) => self.visit_getter(ast),
            MethodDefinition::Setter(ast) => self.visit_setter(ast),
        }
    }

    pub fn visit_import_declaration(&mut self, ast: &mut ImportDeclaration<'alloc>) -> T::Value {
        match ast {
            ImportDeclaration::Import(ast) => self.visit_import(ast),
            ImportDeclaration::ImportNamespace(ast) => self.visit_import_namespace(ast),
        }
    }

    pub fn visit_export_declaration(&mut self, ast: &mut ExportDeclaration<'alloc>) -> T::Value {
        match ast {
            ExportDeclaration::ExportAllFrom(ast) => self.visit_export_all_from(ast),
            ExportDeclaration::ExportFrom(ast) => self.visit_export_from(ast),
            ExportDeclaration::ExportLocals(ast) => self.visit_export_locals(ast),
            ExportDeclaration::Export(ast) => self.visit_export(ast),
            ExportDeclaration::ExportDefault(ast) => self.visit_export_default(ast),
        }
    }

    pub fn visit_variable_reference(&mut self, ast: &mut VariableReference<'alloc>) -> T::Value {
        match ast {
            VariableReference::BindingIdentifier(ast) => self.visit_binding_identifier(ast),
            VariableReference::AssignmentTargetIdentifier(ast) => {
                self.visit_assignment_target_identifier(ast)
            }
        }
    }

    pub fn visit_binding_pattern(&mut self, ast: &mut BindingPattern<'alloc>) -> T::Value {
        match ast {
            BindingPattern::ObjectBinding(ast) => self.visit_object_binding(ast),
            BindingPattern::ArrayBinding(ast) => self.visit_array_binding(ast),
        }
    }

    pub fn visit_binding(&mut self, ast: &mut Binding<'alloc>) -> T::Value {
        match ast {
            Binding::BindingPattern(ast) => self.visit_binding_pattern(ast),
            Binding::BindingIdentifier(ast) => self.visit_binding_identifier(ast),
        }
    }

    pub fn visit_simple_assignment_target(
        &mut self,
        ast: &mut SimpleAssignmentTarget<'alloc>,
    ) -> T::Value {
        match ast {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(ast) => {
                self.visit_assignment_target_identifier(ast)
            }
            SimpleAssignmentTarget::MemberAssignmentTarget(ast) => {
                self.visit_member_assignment_target(ast)
            }
        }
    }

    pub fn visit_assignment_target_pattern(
        &mut self,
        ast: &mut AssignmentTargetPattern<'alloc>,
    ) -> T::Value {
        match ast {
            AssignmentTargetPattern::ArrayAssignmentTarget(ast) => {
                self.visit_array_assignment_target(ast)
            }
            AssignmentTargetPattern::ObjectAssignmentTarget(ast) => {
                self.visit_object_assignment_target(ast)
            }
        }
    }

    pub fn visit_assignment_target(&mut self, ast: &mut AssignmentTarget<'alloc>) -> T::Value {
        match ast {
            AssignmentTarget::AssignmentTargetPattern(ast) => {
                self.visit_assignment_target_pattern(ast)
            }
            AssignmentTarget::SimpleAssignmentTarget(ast) => {
                self.visit_simple_assignment_target(ast)
            }
        }
    }

    pub fn visit_parameter(&mut self, ast: &mut Parameter<'alloc>) -> T::Value {
        match ast {
            Parameter::Binding(ast) => self.visit_binding(ast),
            Parameter::BindingWithDefault(ast) => self.visit_binding_with_default(ast),
        }
    }

    pub fn visit_binding_with_default(&mut self, ast: &mut BindingWithDefault<'alloc>) -> T::Value {
        let a0 = self.visit_binding((&mut ast.binding));
        let a1 = self.visit_expression((&mut ast.init));
        self.pass.visit_binding_with_default(a0, a1)
    }

    pub fn visit_binding_identifier(&mut self, ast: &mut BindingIdentifier<'alloc>) -> T::Value {
        let a0 = self.visit_identifier((&mut ast.name));
        self.pass.visit_binding_identifier(a0)
    }

    pub fn visit_assignment_target_identifier(
        &mut self,
        ast: &mut AssignmentTargetIdentifier<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_identifier((&mut ast.name));
        self.pass.visit_assignment_target_identifier(a0)
    }

    pub fn visit_expression_or_super(&mut self, ast: &mut ExpressionOrSuper<'alloc>) -> T::Value {
        match ast {
            ExpressionOrSuper::Expression(ast) => self.visit_expression(ast),
            ExpressionOrSuper::Super(_) => T::Value::default(),
        }
    }

    pub fn visit_member_assignment_target(
        &mut self,
        ast: &mut MemberAssignmentTarget<'alloc>,
    ) -> T::Value {
        match ast {
            MemberAssignmentTarget::ComputedMemberAssignmentTarget(ast) => {
                self.visit_computed_member_assignment_target(ast)
            }
            MemberAssignmentTarget::StaticMemberAssignmentTarget(ast) => {
                self.visit_static_member_assignment_target(ast)
            }
        }
    }

    pub fn visit_computed_member_assignment_target(
        &mut self,
        ast: &mut ComputedMemberAssignmentTarget<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression_or_super((&mut ast.object));
        let a1 = self.visit_expression((&mut ast.expression));
        self.pass.visit_computed_member_assignment_target(a0, a1)
    }

    pub fn visit_static_member_assignment_target(
        &mut self,
        ast: &mut StaticMemberAssignmentTarget<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression_or_super((&mut ast.object));
        let a1 = self.visit_identifier_name((&mut ast.property));
        self.pass.visit_static_member_assignment_target(a0, a1)
    }

    pub fn visit_array_binding(&mut self, ast: &mut ArrayBinding<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.elements),
                |item| item.as_mut().map(|item| self.visit_parameter(item)),
                allocator,
            )
        };
        let a1 = (&mut ast.rest)
            .as_mut()
            .map(|item| self.visit_binding(item));
        self.pass.visit_array_binding(a0, a1)
    }

    pub fn visit_object_binding(&mut self, ast: &mut ObjectBinding<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.properties),
                |item| self.visit_binding_property(item),
                allocator,
            )
        };
        let a1 = (&mut ast.rest)
            .as_mut()
            .map(|item| self.visit_binding_identifier(item));
        self.pass.visit_object_binding(a0, a1)
    }

    pub fn visit_binding_property(&mut self, ast: &mut BindingProperty<'alloc>) -> T::Value {
        match ast {
            BindingProperty::BindingPropertyIdentifier(ast) => {
                self.visit_binding_property_identifier(ast)
            }
            BindingProperty::BindingPropertyProperty(ast) => {
                self.visit_binding_property_property(ast)
            }
        }
    }

    pub fn visit_binding_property_identifier(
        &mut self,
        ast: &mut BindingPropertyIdentifier<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_binding_identifier((&mut ast.binding));
        let a1 = (&mut ast.init)
            .as_mut()
            .map(|item| self.visit_expression(item));
        self.pass.visit_binding_property_identifier(a0, a1)
    }

    pub fn visit_binding_property_property(
        &mut self,
        ast: &mut BindingPropertyProperty<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_property_name((&mut ast.name));
        let a1 = self.visit_parameter((&mut ast.binding));
        self.pass.visit_binding_property_property(a0, a1)
    }

    pub fn visit_assignment_target_with_default(
        &mut self,
        ast: &mut AssignmentTargetWithDefault<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_assignment_target((&mut ast.binding));
        let a1 = self.visit_expression((&mut ast.init));
        self.pass.visit_assignment_target_with_default(a0, a1)
    }

    pub fn visit_assignment_target_maybe_default(
        &mut self,
        ast: &mut AssignmentTargetMaybeDefault<'alloc>,
    ) -> T::Value {
        match ast {
            AssignmentTargetMaybeDefault::AssignmentTarget(ast) => {
                self.visit_assignment_target(ast)
            }
            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(ast) => {
                self.visit_assignment_target_with_default(ast)
            }
        }
    }

    pub fn visit_array_assignment_target(
        &mut self,
        ast: &mut ArrayAssignmentTarget<'alloc>,
    ) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.elements),
                |item| {
                    item.as_mut()
                        .map(|item| self.visit_assignment_target_maybe_default(item))
                },
                allocator,
            )
        };
        let a1 = (&mut ast.rest)
            .as_mut()
            .map(|item| self.visit_assignment_target(item));
        self.pass.visit_array_assignment_target(a0, a1)
    }

    pub fn visit_object_assignment_target(
        &mut self,
        ast: &mut ObjectAssignmentTarget<'alloc>,
    ) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.properties),
                |item| self.visit_assignment_target_property(item),
                allocator,
            )
        };
        let a1 = (&mut ast.rest)
            .as_mut()
            .map(|item| self.visit_assignment_target(item));
        self.pass.visit_object_assignment_target(a0, a1)
    }

    pub fn visit_assignment_target_property(
        &mut self,
        ast: &mut AssignmentTargetProperty<'alloc>,
    ) -> T::Value {
        match ast {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(ast) => {
                self.visit_assignment_target_property_identifier(ast)
            }
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(ast) => {
                self.visit_assignment_target_property_property(ast)
            }
        }
    }

    pub fn visit_assignment_target_property_identifier(
        &mut self,
        ast: &mut AssignmentTargetPropertyIdentifier<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_assignment_target_identifier((&mut ast.binding));
        let a1 = (&mut ast.init)
            .as_mut()
            .map(|item| self.visit_expression(item));
        self.pass
            .visit_assignment_target_property_identifier(a0, a1)
    }

    pub fn visit_assignment_target_property_property(
        &mut self,
        ast: &mut AssignmentTargetPropertyProperty<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_property_name((&mut ast.name));
        let a1 = self.visit_assignment_target_maybe_default((&mut ast.binding));
        self.pass.visit_assignment_target_property_property(a0, a1)
    }

    pub fn visit_class_expression(&mut self, ast: &mut ClassExpression<'alloc>) -> T::Value {
        let a0 = (&mut ast.name)
            .as_mut()
            .map(|item| self.visit_binding_identifier(item));
        let a1 = (&mut ast.super_)
            .as_mut()
            .map(|item| self.visit_expression(item));
        let a2 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.elements),
                |item| self.visit_class_element(item),
                allocator,
            )
        };
        self.pass.visit_class_expression(a0, a1, a2)
    }

    pub fn visit_class_declaration(&mut self, ast: &mut ClassDeclaration<'alloc>) -> T::Value {
        let a0 = self.visit_binding_identifier((&mut ast.name));
        let a1 = (&mut ast.super_)
            .as_mut()
            .map(|item| self.visit_expression(item));
        let a2 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.elements),
                |item| self.visit_class_element(item),
                allocator,
            )
        };
        self.pass.visit_class_declaration(a0, a1, a2)
    }

    pub fn visit_class_element(&mut self, ast: &mut ClassElement<'alloc>) -> T::Value {
        let a0 = &mut ast.is_static;
        let a1 = self.visit_method_definition((&mut ast.method));
        self.pass.visit_class_element(a0, a1)
    }

    pub fn visit_module_items(&mut self, ast: &mut ModuleItems<'alloc>) -> T::Value {
        match ast {
            ModuleItems::ImportDeclaration(ast) => self.visit_import_declaration(ast),
            ModuleItems::ExportDeclaration(ast) => self.visit_export_declaration(ast),
            ModuleItems::Statement(ast) => self.visit_statement(ast),
        }
    }

    pub fn visit_module(&mut self, ast: &mut Module<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.directives),
                |item| self.visit_directive(item),
                allocator,
            )
        };
        let a1 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.items),
                |item| self.visit_module_items(item),
                allocator,
            )
        };
        self.pass.visit_module(a0, a1)
    }

    pub fn visit_import(&mut self, ast: &mut Import<'alloc>) -> T::Value {
        let a0 = &mut ast.module_specifier;
        let a1 = (&mut ast.default_binding)
            .as_mut()
            .map(|item| self.visit_binding_identifier(item));
        let a2 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.named_imports),
                |item| self.visit_import_specifier(item),
                allocator,
            )
        };
        self.pass.visit_import(a0, a1, a2)
    }

    pub fn visit_import_namespace(&mut self, ast: &mut ImportNamespace<'alloc>) -> T::Value {
        let a0 = &mut ast.module_specifier;
        let a1 = (&mut ast.default_binding)
            .as_mut()
            .map(|item| self.visit_binding_identifier(item));
        let a2 = self.visit_binding_identifier((&mut ast.namespace_binding));
        self.pass.visit_import_namespace(a0, a1, a2)
    }

    pub fn visit_import_specifier(&mut self, ast: &mut ImportSpecifier<'alloc>) -> T::Value {
        let a0 = (&mut ast.name)
            .as_mut()
            .map(|item| self.visit_identifier_name(item));
        let a1 = self.visit_binding_identifier((&mut ast.binding));
        self.pass.visit_import_specifier(a0, a1)
    }

    pub fn visit_export_all_from(&mut self, ast: &mut ExportAllFrom<'alloc>) -> T::Value {
        let a0 = &mut ast.module_specifier;
        self.pass.visit_export_all_from(a0)
    }

    pub fn visit_export_from(&mut self, ast: &mut ExportFrom<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.named_exports),
                |item| self.visit_export_from_specifier(item),
                allocator,
            )
        };
        let a1 = &mut ast.module_specifier;
        self.pass.visit_export_from(a0, a1)
    }

    pub fn visit_export_locals(&mut self, ast: &mut ExportLocals<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.named_exports),
                |item| self.visit_export_local_specifier(item),
                allocator,
            )
        };
        self.pass.visit_export_locals(a0)
    }

    pub fn visit_export(&mut self, ast: &mut Export<'alloc>) -> T::Value {
        match ast {
            Export::FunctionDeclaration(ast) => self.visit_function(ast),
            Export::ClassDeclaration(ast) => self.visit_class_declaration(ast),
            Export::VariableDeclaration(ast) => self.visit_variable_declaration(ast),
        }
    }

    pub fn visit_export_default(&mut self, ast: &mut ExportDefault<'alloc>) -> T::Value {
        match ast {
            ExportDefault::FunctionDeclaration(ast) => self.visit_function(ast),
            ExportDefault::ClassDeclaration(ast) => self.visit_class_declaration(ast),
            ExportDefault::Expression(ast) => self.visit_expression(ast),
        }
    }

    pub fn visit_export_from_specifier(
        &mut self,
        ast: &mut ExportFromSpecifier<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_identifier_name((&mut ast.name));
        let a1 = (&mut ast.exported_name)
            .as_mut()
            .map(|item| self.visit_identifier_name(item));
        self.pass.visit_export_from_specifier(a0, a1)
    }

    pub fn visit_export_local_specifier(
        &mut self,
        ast: &mut ExportLocalSpecifier<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_identifier_expression((&mut ast.name));
        let a1 = (&mut ast.exported_name)
            .as_mut()
            .map(|item| self.visit_identifier_name(item));
        self.pass.visit_export_local_specifier(a0, a1)
    }

    pub fn visit_method(&mut self, ast: &mut Method<'alloc>) -> T::Value {
        let a0 = self.visit_property_name((&mut ast.name));
        let a1 = &mut ast.is_async;
        let a2 = &mut ast.is_generator;
        let a3 = self.visit_formal_parameters((&mut ast.params));
        let a4 = self.visit_function_body((&mut ast.body));
        self.pass.visit_method(a0, a1, a2, a3, a4)
    }

    pub fn visit_getter(&mut self, ast: &mut Getter<'alloc>) -> T::Value {
        let a0 = self.visit_property_name((&mut ast.property_name));
        let a1 = self.visit_function_body((&mut ast.body));
        self.pass.visit_getter(a0, a1)
    }

    pub fn visit_setter(&mut self, ast: &mut Setter<'alloc>) -> T::Value {
        let a0 = self.visit_property_name((&mut ast.property_name));
        let a1 = self.visit_parameter((&mut ast.param));
        let a2 = self.visit_function_body((&mut ast.body));
        self.pass.visit_setter(a0, a1, a2)
    }

    pub fn visit_data_property(&mut self, ast: &mut DataProperty<'alloc>) -> T::Value {
        let a0 = self.visit_property_name((&mut ast.property_name));
        let a1 = self.visit_expression((&mut ast.expression));
        self.pass.visit_data_property(a0, a1)
    }

    pub fn visit_shorthand_property(&mut self, ast: &mut ShorthandProperty<'alloc>) -> T::Value {
        let a0 = self.visit_identifier_expression((&mut ast.name));
        self.pass.visit_shorthand_property(a0)
    }

    pub fn visit_computed_property_name(
        &mut self,
        ast: &mut ComputedPropertyName<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression((&mut ast.expression));
        self.pass.visit_computed_property_name(a0)
    }

    pub fn visit_static_property_name(&mut self, ast: &mut StaticPropertyName<'alloc>) -> T::Value {
        let a0 = &mut ast.value;
        self.pass.visit_static_property_name(a0)
    }

    pub fn visit_array_expression_element(
        &mut self,
        ast: &mut ArrayExpressionElement<'alloc>,
    ) -> T::Value {
        match ast {
            ArrayExpressionElement::SpreadElement(ast) => self.visit_expression(ast),
            ArrayExpressionElement::Expression(ast) => self.visit_expression(ast),
            ArrayExpressionElement::Elision(_) => T::Value::default(),
        }
    }

    pub fn visit_array_expression(&mut self, ast: &mut ArrayExpression<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.elements),
                |item| self.visit_array_expression_element(item),
                allocator,
            )
        };
        self.pass.visit_array_expression(a0)
    }

    pub fn visit_arrow_expression_body(
        &mut self,
        ast: &mut ArrowExpressionBody<'alloc>,
    ) -> T::Value {
        match ast {
            ArrowExpressionBody::FunctionBody(ast) => self.visit_function_body(ast),
            ArrowExpressionBody::Expression(ast) => self.visit_expression(ast),
        }
    }

    pub fn visit_computed_member_expression(
        &mut self,
        ast: &mut ComputedMemberExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression_or_super((&mut ast.object));
        let a1 = self.visit_expression((&mut ast.expression));
        self.pass.visit_computed_member_expression(a0, a1)
    }

    pub fn visit_identifier_expression(
        &mut self,
        ast: &mut IdentifierExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_identifier((&mut ast.name));
        self.pass.visit_identifier_expression(a0)
    }

    pub fn visit_object_expression(&mut self, ast: &mut ObjectExpression<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.properties),
                |item| self.visit_object_property(item),
                allocator,
            )
        };
        self.pass.visit_object_expression(a0)
    }

    pub fn visit_static_member_expression(
        &mut self,
        ast: &mut StaticMemberExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression_or_super((&mut ast.object));
        let a1 = self.visit_identifier_name((&mut ast.property));
        self.pass.visit_static_member_expression(a0, a1)
    }

    pub fn visit_template_expression_element(
        &mut self,
        ast: &mut TemplateExpressionElement<'alloc>,
    ) -> T::Value {
        match ast {
            TemplateExpressionElement::Expression(ast) => self.visit_expression(ast),
            TemplateExpressionElement::TemplateElement(ast) => self.visit_template_element(ast),
        }
    }

    pub fn visit_template_expression(&mut self, ast: &mut TemplateExpression<'alloc>) -> T::Value {
        let a0 = (&mut ast.tag)
            .as_mut()
            .map(|item| self.visit_expression(item));
        let a1 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.elements),
                |item| self.visit_template_expression_element(item),
                allocator,
            )
        };
        self.pass.visit_template_expression(a0, a1)
    }

    pub fn visit_variable_declaration_or_assignment_target(
        &mut self,
        ast: &mut VariableDeclarationOrAssignmentTarget<'alloc>,
    ) -> T::Value {
        match ast {
            VariableDeclarationOrAssignmentTarget::VariableDeclaration(ast) => {
                self.visit_variable_declaration(ast)
            }
            VariableDeclarationOrAssignmentTarget::AssignmentTarget(ast) => {
                self.visit_assignment_target(ast)
            }
        }
    }

    pub fn visit_variable_declaration_or_expression(
        &mut self,
        ast: &mut VariableDeclarationOrExpression<'alloc>,
    ) -> T::Value {
        match ast {
            VariableDeclarationOrExpression::VariableDeclaration(ast) => {
                self.visit_variable_declaration(ast)
            }
            VariableDeclarationOrExpression::Expression(ast) => self.visit_expression(ast),
        }
    }

    pub fn visit_block(&mut self, ast: &mut Block<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.statements),
                |item| self.visit_statement(item),
                allocator,
            )
        };
        let a1 = &mut ast.declarations;
        self.pass.visit_block(a0, a1)
    }

    pub fn visit_catch_clause(&mut self, ast: &mut CatchClause<'alloc>) -> T::Value {
        let a0 = (&mut ast.binding)
            .as_mut()
            .map(|item| self.visit_binding(item));
        let a1 = self.visit_block((&mut ast.body));
        self.pass.visit_catch_clause(a0, a1)
    }

    pub fn visit_directive(&mut self, ast: &mut Directive<'alloc>) -> T::Value {
        let a0 = &mut ast.raw_value;
        self.pass.visit_directive(a0)
    }

    pub fn visit_formal_parameters(&mut self, ast: &mut FormalParameters<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.items),
                |item| self.visit_parameter(item),
                allocator,
            )
        };
        let a1 = (&mut ast.rest)
            .as_mut()
            .map(|item| self.visit_binding(item));
        self.pass.visit_formal_parameters(a0, a1)
    }

    pub fn visit_function_body(&mut self, ast: &mut FunctionBody<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.directives),
                |item| self.visit_directive(item),
                allocator,
            )
        };
        let a1 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.statements),
                |item| self.visit_statement(item),
                allocator,
            )
        };
        self.pass.visit_function_body(a0, a1)
    }

    pub fn visit_script(&mut self, ast: &mut Script<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.directives),
                |item| self.visit_directive(item),
                allocator,
            )
        };
        let a1 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.statements),
                |item| self.visit_statement(item),
                allocator,
            )
        };
        self.pass.visit_script(a0, a1)
    }

    pub fn visit_switch_case(&mut self, ast: &mut SwitchCase<'alloc>) -> T::Value {
        let a0 = self.visit_expression((&mut ast.test));
        let a1 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.consequent),
                |item| self.visit_statement(item),
                allocator,
            )
        };
        self.pass.visit_switch_case(a0, a1)
    }

    pub fn visit_switch_default(&mut self, ast: &mut SwitchDefault<'alloc>) -> T::Value {
        let a0 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.consequent),
                |item| self.visit_statement(item),
                allocator,
            )
        };
        self.pass.visit_switch_default(a0)
    }

    pub fn visit_template_element(&mut self, ast: &mut TemplateElement<'alloc>) -> T::Value {
        let a0 = &mut ast.raw_value;
        self.pass.visit_template_element(a0)
    }

    pub fn visit_variable_declaration(
        &mut self,
        ast: &mut VariableDeclaration<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_variable_declaration_kind((&mut ast.kind));
        let a1 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.declarators),
                |item| self.visit_variable_declarator(item),
                allocator,
            )
        };
        self.pass.visit_variable_declaration(a0, a1)
    }

    pub fn visit_variable_declarator(&mut self, ast: &mut VariableDeclarator<'alloc>) -> T::Value {
        let a0 = self.visit_binding((&mut ast.binding));
        let a1 = (&mut ast.init)
            .as_mut()
            .map(|item| self.visit_expression(item));
        self.pass.visit_variable_declarator(a0, a1)
    }

    pub fn visit_cover_parenthesized(&mut self, ast: &mut CoverParenthesized<'alloc>) -> T::Value {
        match ast {
            CoverParenthesized::Expression { expression, .. } => {
                let a0 = self.visit_expression((expression));
                self.pass.visit_expression(a0)
            }
            CoverParenthesized::Parameters(ast) => self.visit_formal_parameters(ast),
        }
    }
}
