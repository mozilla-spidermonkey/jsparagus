// WARNING: This file is auto-generated.

#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]

use ast::*;
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
            VariableDeclarationKind::Var => (),
            VariableDeclarationKind::Let => (),
            VariableDeclarationKind::Const => (),
        }
    }

    fn visit_compound_assignment_operator(&mut self, ast: &mut CompoundAssignmentOperator) {
        match ast {
            CompoundAssignmentOperator::Add => (),
            CompoundAssignmentOperator::Sub => (),
            CompoundAssignmentOperator::Mul => (),
            CompoundAssignmentOperator::Div => (),
            CompoundAssignmentOperator::Mod => (),
            CompoundAssignmentOperator::Pow => (),
            CompoundAssignmentOperator::LeftShift => (),
            CompoundAssignmentOperator::RightShift => (),
            CompoundAssignmentOperator::RightShiftExt => (),
            CompoundAssignmentOperator::Or => (),
            CompoundAssignmentOperator::Xor => (),
            CompoundAssignmentOperator::And => (),
        }
    }

    fn visit_binary_operator(&mut self, ast: &mut BinaryOperator) {
        match ast {
            BinaryOperator::Equals => (),
            BinaryOperator::NotEquals => (),
            BinaryOperator::StrictEquals => (),
            BinaryOperator::StrictNotEquals => (),
            BinaryOperator::LessThan => (),
            BinaryOperator::LessThanOrEqual => (),
            BinaryOperator::GreaterThan => (),
            BinaryOperator::GreaterThanOrEqual => (),
            BinaryOperator::In => (),
            BinaryOperator::Instanceof => (),
            BinaryOperator::LeftShift => (),
            BinaryOperator::RightShift => (),
            BinaryOperator::RightShiftExt => (),
            BinaryOperator::Add => (),
            BinaryOperator::Sub => (),
            BinaryOperator::Mul => (),
            BinaryOperator::Div => (),
            BinaryOperator::Mod => (),
            BinaryOperator::Pow => (),
            BinaryOperator::Comma => (),
            BinaryOperator::LogicalOr => (),
            BinaryOperator::LogicalAnd => (),
            BinaryOperator::BitwiseOr => (),
            BinaryOperator::BitwiseXor => (),
            BinaryOperator::BitwiseAnd => (),
        }
    }

    fn visit_unary_operator(&mut self, ast: &mut UnaryOperator) {
        match ast {
            UnaryOperator::Plus => (),
            UnaryOperator::Minus => (),
            UnaryOperator::LogicalNot => (),
            UnaryOperator::BitwiseNot => (),
            UnaryOperator::Typeof => (),
            UnaryOperator::Void => (),
            UnaryOperator::Delete => (),
        }
    }

    fn visit_update_operator(&mut self, ast: &mut UpdateOperator) {
        match ast {
            UpdateOperator::Increment => (),
            UpdateOperator::Decrement => (),
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
            Statement::BlockStatement(ast) => {
                self.visit_block_statement(ast);
            }
            Statement::BreakStatement(ast) => {
                self.visit_break_statement(ast);
            }
            Statement::ContinueStatement(ast) => {
                self.visit_continue_statement(ast);
            }
            Statement::DebuggerStatement => (),
            Statement::DoWhileStatement(ast) => {
                self.visit_do_while_statement(ast);
            }
            Statement::EmptyStatement => (),
            Statement::ExpressionStatement(ast) => {
                self.visit_expression(ast);
            }
            Statement::ForInStatement(ast) => {
                self.visit_for_in_statement(ast);
            }
            Statement::ForOfStatement(ast) => {
                self.visit_for_of_statement(ast);
            }
            Statement::ForStatement(ast) => {
                self.visit_for_statement(ast);
            }
            Statement::IfStatement(ast) => {
                self.visit_if_statement(ast);
            }
            Statement::LabeledStatement(ast) => {
                self.visit_labeled_statement(ast);
            }
            Statement::ReturnStatement(ast) => {
                self.visit_return_statement(ast);
            }
            Statement::SwitchStatement(ast) => {
                self.visit_switch_statement(ast);
            }
            Statement::SwitchStatementWithDefault(ast) => {
                self.visit_switch_statement_with_default(ast);
            }
            Statement::ThrowStatement(ast) => {
                self.visit_throw_statement(ast);
            }
            Statement::TryCatchStatement(ast) => {
                self.visit_try_catch_statement(ast);
            }
            Statement::TryFinallyStatement(ast) => {
                self.visit_try_finally_statement(ast);
            }
            Statement::VariableDeclarationStatement(ast) => {
                self.visit_variable_declaration(ast);
            }
            Statement::WhileStatement(ast) => {
                self.visit_while_statement(ast);
            }
            Statement::WithStatement(ast) => {
                self.visit_with_statement(ast);
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
            Expression::LiteralBooleanExpression(ast) => {
                self.visit_literal_boolean_expression(ast);
            }
            Expression::LiteralInfinityExpression => (),
            Expression::LiteralNullExpression => (),
            Expression::LiteralNumericExpression(ast) => {
                self.visit_literal_numeric_expression(ast);
            }
            Expression::LiteralRegExpExpression(ast) => {
                self.visit_literal_reg_exp_expression(ast);
            }
            Expression::LiteralStringExpression(ast) => {
                self.visit_literal_string_expression(ast);
            }
            Expression::ArrayExpression(ast) => {
                self.visit_array_expression(ast);
            }
            Expression::ArrowExpression(ast) => {
                self.visit_arrow_expression(ast);
            }
            Expression::AssignmentExpression(ast) => {
                self.visit_assignment_expression(ast);
            }
            Expression::BinaryExpression(ast) => {
                self.visit_binary_expression(ast);
            }
            Expression::CallExpression(ast) => {
                self.visit_call_expression(ast);
            }
            Expression::CompoundAssignmentExpression(ast) => {
                self.visit_compound_assignment_expression(ast);
            }
            Expression::ConditionalExpression(ast) => {
                self.visit_conditional_expression(ast);
            }
            Expression::FunctionExpression(ast) => {
                self.visit_function(ast);
            }
            Expression::IdentifierExpression(ast) => {
                self.visit_identifier_expression(ast);
            }
            Expression::NewExpression(ast) => {
                self.visit_new_expression(ast);
            }
            Expression::NewTargetExpression => (),
            Expression::ObjectExpression(ast) => {
                self.visit_object_expression(ast);
            }
            Expression::UnaryExpression(ast) => {
                self.visit_unary_expression(ast);
            }
            Expression::TemplateExpression(ast) => {
                self.visit_template_expression(ast);
            }
            Expression::ThisExpression => (),
            Expression::UpdateExpression(ast) => {
                self.visit_update_expression(ast);
            }
            Expression::YieldExpression(ast) => {
                self.visit_yield_expression(ast);
            }
            Expression::YieldGeneratorExpression(ast) => {
                self.visit_yield_generator_expression(ast);
            }
            Expression::AwaitExpression(ast) => {
                self.visit_await_expression(ast);
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
            ExpressionOrSuper::Super => (),
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

    fn visit_literal_boolean_expression(&mut self, ast: &mut LiteralBooleanExpression) {}

    fn visit_literal_numeric_expression(&mut self, ast: &mut LiteralNumericExpression) {}

    fn visit_literal_reg_exp_expression(&mut self, ast: &mut LiteralRegExpExpression<'alloc>) {}

    fn visit_literal_string_expression(&mut self, ast: &mut LiteralStringExpression<'alloc>) {}

    fn visit_array_expression_element(&mut self, ast: &mut ArrayExpressionElement<'alloc>) {
        match ast {
            ArrayExpressionElement::SpreadElement(ast) => {
                self.visit_expression(ast);
            }
            ArrayExpressionElement::Expression(ast) => {
                self.visit_expression(ast);
            }
            ArrayExpressionElement::Elision => (),
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

    fn visit_arrow_expression(&mut self, ast: &mut ArrowExpression<'alloc>) {
        self.visit_formal_parameters(&mut ast.params);
        self.visit_arrow_expression_body(&mut ast.body);
    }

    fn visit_assignment_expression(&mut self, ast: &mut AssignmentExpression<'alloc>) {
        self.visit_assignment_target(&mut ast.binding);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_binary_expression(&mut self, ast: &mut BinaryExpression<'alloc>) {
        self.visit_binary_operator(&mut ast.operator);
        self.visit_expression(&mut ast.left);
        self.visit_expression(&mut ast.right);
    }

    fn visit_call_expression(&mut self, ast: &mut CallExpression<'alloc>) {
        self.visit_expression_or_super(&mut ast.callee);
        self.visit_arguments(&mut ast.arguments);
    }

    fn visit_compound_assignment_expression(
        &mut self,
        ast: &mut CompoundAssignmentExpression<'alloc>,
    ) {
        self.visit_compound_assignment_operator(&mut ast.operator);
        self.visit_simple_assignment_target(&mut ast.binding);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_computed_member_expression(&mut self, ast: &mut ComputedMemberExpression<'alloc>) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_conditional_expression(&mut self, ast: &mut ConditionalExpression<'alloc>) {
        self.visit_expression(&mut ast.test);
        self.visit_expression(&mut ast.consequent);
        self.visit_expression(&mut ast.alternate);
    }

    fn visit_identifier_expression(&mut self, ast: &mut IdentifierExpression<'alloc>) {
        self.visit_identifier(&mut ast.name);
    }

    fn visit_new_expression(&mut self, ast: &mut NewExpression<'alloc>) {
        self.visit_expression(&mut ast.callee);
        self.visit_arguments(&mut ast.arguments);
    }

    fn visit_object_expression(&mut self, ast: &mut ObjectExpression<'alloc>) {
        for item in &mut ast.properties {
            self.visit_object_property(item);
        }
    }

    fn visit_unary_expression(&mut self, ast: &mut UnaryExpression<'alloc>) {
        self.visit_unary_operator(&mut ast.operator);
        self.visit_expression(&mut ast.operand);
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

    fn visit_update_expression(&mut self, ast: &mut UpdateExpression<'alloc>) {
        self.visit_update_operator(&mut ast.operator);
        self.visit_simple_assignment_target(&mut ast.operand);
    }

    fn visit_yield_expression(&mut self, ast: &mut YieldExpression<'alloc>) {
        if let Some(item) = &mut ast.expression {
            self.visit_expression(item);
        }
    }

    fn visit_yield_generator_expression(&mut self, ast: &mut YieldGeneratorExpression<'alloc>) {
        self.visit_expression(&mut ast.expression);
    }

    fn visit_await_expression(&mut self, ast: &mut AwaitExpression<'alloc>) {
        self.visit_expression(&mut ast.expression);
    }

    fn visit_block_statement(&mut self, ast: &mut BlockStatement<'alloc>) {
        self.visit_block(&mut ast.block);
    }

    fn visit_break_statement(&mut self, ast: &mut BreakStatement<'alloc>) {
        if let Some(item) = &mut ast.label {
            self.visit_label(item);
        }
    }

    fn visit_continue_statement(&mut self, ast: &mut ContinueStatement<'alloc>) {
        if let Some(item) = &mut ast.label {
            self.visit_label(item);
        }
    }

    fn visit_do_while_statement(&mut self, ast: &mut DoWhileStatement<'alloc>) {
        self.visit_statement(&mut ast.block);
        self.visit_expression(&mut ast.test);
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

    fn visit_for_in_statement(&mut self, ast: &mut ForInStatement<'alloc>) {
        self.visit_variable_declaration_or_assignment_target(&mut ast.left);
        self.visit_expression(&mut ast.right);
        self.visit_statement(&mut ast.block);
    }

    fn visit_for_of_statement(&mut self, ast: &mut ForOfStatement<'alloc>) {
        self.visit_variable_declaration_or_assignment_target(&mut ast.left);
        self.visit_expression(&mut ast.right);
        self.visit_statement(&mut ast.block);
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

    fn visit_for_statement(&mut self, ast: &mut ForStatement<'alloc>) {
        if let Some(item) = &mut ast.init {
            self.visit_variable_declaration_or_expression(item);
        }
        if let Some(item) = &mut ast.test {
            self.visit_expression(item);
        }
        if let Some(item) = &mut ast.update {
            self.visit_expression(item);
        }
        self.visit_statement(&mut ast.block);
    }

    fn visit_if_statement(&mut self, ast: &mut IfStatement<'alloc>) {
        self.visit_expression(&mut ast.test);
        self.visit_statement(&mut ast.consequent);
        if let Some(item) = &mut ast.alternate {
            self.visit_statement(item);
        }
    }

    fn visit_labeled_statement(&mut self, ast: &mut LabeledStatement<'alloc>) {
        self.visit_label(&mut ast.label);
        self.visit_statement(&mut ast.body);
    }

    fn visit_return_statement(&mut self, ast: &mut ReturnStatement<'alloc>) {
        if let Some(item) = &mut ast.expression {
            self.visit_expression(item);
        }
    }

    fn visit_switch_statement(&mut self, ast: &mut SwitchStatement<'alloc>) {
        self.visit_expression(&mut ast.discriminant);
        for item in &mut ast.cases {
            self.visit_switch_case(item);
        }
    }

    fn visit_switch_statement_with_default(
        &mut self,
        ast: &mut SwitchStatementWithDefault<'alloc>,
    ) {
        self.visit_expression(&mut ast.discriminant);
        for item in &mut ast.pre_default_cases {
            self.visit_switch_case(item);
        }
        self.visit_switch_default(&mut ast.default_case);
        for item in &mut ast.post_default_cases {
            self.visit_switch_case(item);
        }
    }

    fn visit_throw_statement(&mut self, ast: &mut ThrowStatement<'alloc>) {
        self.visit_expression(&mut ast.expression);
    }

    fn visit_try_catch_statement(&mut self, ast: &mut TryCatchStatement<'alloc>) {
        self.visit_block(&mut ast.body);
        self.visit_catch_clause(&mut ast.catch_clause);
    }

    fn visit_try_finally_statement(&mut self, ast: &mut TryFinallyStatement<'alloc>) {
        self.visit_block(&mut ast.body);
        if let Some(item) = &mut ast.catch_clause {
            self.visit_catch_clause(item);
        }
        self.visit_block(&mut ast.finalizer);
    }

    fn visit_while_statement(&mut self, ast: &mut WhileStatement<'alloc>) {
        self.visit_expression(&mut ast.test);
        self.visit_statement(&mut ast.block);
    }

    fn visit_with_statement(&mut self, ast: &mut WithStatement<'alloc>) {
        self.visit_expression(&mut ast.object);
        self.visit_statement(&mut ast.body);
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
        self.visit_binding(&mut ast.binding);
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
            CoverParenthesized::Expression(ast) => {
                self.visit_expression(ast);
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

    fn visit_identifier(&self, value: &mut arena::String<'alloc>) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_identifier_name(&self, value: &mut arena::String<'alloc>) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_label(&self, value: &mut arena::String<'alloc>) -> Self::Value {
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
        module_specifier: &mut arena::String<'alloc>,
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
        module_specifier: &mut arena::String<'alloc>,
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

    fn visit_export_all_from(&self, module_specifier: &mut arena::String<'alloc>) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_export_from(
        &self,
        named_exports: arena::Vec<'alloc, Self::Value>,
        module_specifier: &mut arena::String<'alloc>,
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

    fn visit_static_property_name(&self, value: &mut arena::String<'alloc>) -> Self::Value {
        let mut result = Self::Value::default();
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
        pattern: &mut arena::String<'alloc>,
        global: &mut bool,
        ignore_case: &mut bool,
        multi_line: &mut bool,
        sticky: &mut bool,
        unicode: &mut bool,
    ) -> Self::Value {
        let mut result = Self::Value::default();
        result
    }

    fn visit_literal_string_expression(&self, value: &mut arena::String<'alloc>) -> Self::Value {
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

    fn visit_identifier_expression(&self, name: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(name);
        result
    }

    fn visit_new_expression(&self, callee: Self::Value, arguments: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(callee);
        result.append(arguments);
        result
    }

    fn visit_object_expression(&self, properties: arena::Vec<'alloc, Self::Value>) -> Self::Value {
        let mut result = Self::Value::default();
        for item in properties {
            result.append(item);
        }
        result
    }

    fn visit_unary_expression(&self, operator: Self::Value, operand: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(operator);
        result.append(operand);
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

    fn visit_block(
        &self,
        statements: arena::Vec<'alloc, Self::Value>,
        declarations: &mut Option<arena::Vec<'alloc, arena::String<'alloc>>>,
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

    fn visit_catch_clause(&self, binding: Self::Value, body: Self::Value) -> Self::Value {
        let mut result = Self::Value::default();
        result.append(binding);
        result.append(body);
        result
    }

    fn visit_directive(&self, raw_value: &mut arena::String<'alloc>) -> Self::Value {
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

    fn visit_template_element(&self, raw_value: &mut arena::String<'alloc>) -> Self::Value {
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
            VariableDeclarationKind::Var => T::Value::default(),
            VariableDeclarationKind::Let => T::Value::default(),
            VariableDeclarationKind::Const => T::Value::default(),
        }
    }

    pub fn visit_compound_assignment_operator(
        &mut self,
        ast: &mut CompoundAssignmentOperator,
    ) -> T::Value {
        match ast {
            CompoundAssignmentOperator::Add => T::Value::default(),
            CompoundAssignmentOperator::Sub => T::Value::default(),
            CompoundAssignmentOperator::Mul => T::Value::default(),
            CompoundAssignmentOperator::Div => T::Value::default(),
            CompoundAssignmentOperator::Mod => T::Value::default(),
            CompoundAssignmentOperator::Pow => T::Value::default(),
            CompoundAssignmentOperator::LeftShift => T::Value::default(),
            CompoundAssignmentOperator::RightShift => T::Value::default(),
            CompoundAssignmentOperator::RightShiftExt => T::Value::default(),
            CompoundAssignmentOperator::Or => T::Value::default(),
            CompoundAssignmentOperator::Xor => T::Value::default(),
            CompoundAssignmentOperator::And => T::Value::default(),
        }
    }

    pub fn visit_binary_operator(&mut self, ast: &mut BinaryOperator) -> T::Value {
        match ast {
            BinaryOperator::Equals => T::Value::default(),
            BinaryOperator::NotEquals => T::Value::default(),
            BinaryOperator::StrictEquals => T::Value::default(),
            BinaryOperator::StrictNotEquals => T::Value::default(),
            BinaryOperator::LessThan => T::Value::default(),
            BinaryOperator::LessThanOrEqual => T::Value::default(),
            BinaryOperator::GreaterThan => T::Value::default(),
            BinaryOperator::GreaterThanOrEqual => T::Value::default(),
            BinaryOperator::In => T::Value::default(),
            BinaryOperator::Instanceof => T::Value::default(),
            BinaryOperator::LeftShift => T::Value::default(),
            BinaryOperator::RightShift => T::Value::default(),
            BinaryOperator::RightShiftExt => T::Value::default(),
            BinaryOperator::Add => T::Value::default(),
            BinaryOperator::Sub => T::Value::default(),
            BinaryOperator::Mul => T::Value::default(),
            BinaryOperator::Div => T::Value::default(),
            BinaryOperator::Mod => T::Value::default(),
            BinaryOperator::Pow => T::Value::default(),
            BinaryOperator::Comma => T::Value::default(),
            BinaryOperator::LogicalOr => T::Value::default(),
            BinaryOperator::LogicalAnd => T::Value::default(),
            BinaryOperator::BitwiseOr => T::Value::default(),
            BinaryOperator::BitwiseXor => T::Value::default(),
            BinaryOperator::BitwiseAnd => T::Value::default(),
        }
    }

    pub fn visit_unary_operator(&mut self, ast: &mut UnaryOperator) -> T::Value {
        match ast {
            UnaryOperator::Plus => T::Value::default(),
            UnaryOperator::Minus => T::Value::default(),
            UnaryOperator::LogicalNot => T::Value::default(),
            UnaryOperator::BitwiseNot => T::Value::default(),
            UnaryOperator::Typeof => T::Value::default(),
            UnaryOperator::Void => T::Value::default(),
            UnaryOperator::Delete => T::Value::default(),
        }
    }

    pub fn visit_update_operator(&mut self, ast: &mut UpdateOperator) -> T::Value {
        match ast {
            UpdateOperator::Increment => T::Value::default(),
            UpdateOperator::Decrement => T::Value::default(),
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
            Statement::BlockStatement(ast) => self.visit_block_statement(ast),
            Statement::BreakStatement(ast) => self.visit_break_statement(ast),
            Statement::ContinueStatement(ast) => self.visit_continue_statement(ast),
            Statement::DebuggerStatement => T::Value::default(),
            Statement::DoWhileStatement(ast) => self.visit_do_while_statement(ast),
            Statement::EmptyStatement => T::Value::default(),
            Statement::ExpressionStatement(ast) => self.visit_expression(ast),
            Statement::ForInStatement(ast) => self.visit_for_in_statement(ast),
            Statement::ForOfStatement(ast) => self.visit_for_of_statement(ast),
            Statement::ForStatement(ast) => self.visit_for_statement(ast),
            Statement::IfStatement(ast) => self.visit_if_statement(ast),
            Statement::LabeledStatement(ast) => self.visit_labeled_statement(ast),
            Statement::ReturnStatement(ast) => self.visit_return_statement(ast),
            Statement::SwitchStatement(ast) => self.visit_switch_statement(ast),
            Statement::SwitchStatementWithDefault(ast) => {
                self.visit_switch_statement_with_default(ast)
            }
            Statement::ThrowStatement(ast) => self.visit_throw_statement(ast),
            Statement::TryCatchStatement(ast) => self.visit_try_catch_statement(ast),
            Statement::TryFinallyStatement(ast) => self.visit_try_finally_statement(ast),
            Statement::VariableDeclarationStatement(ast) => self.visit_variable_declaration(ast),
            Statement::WhileStatement(ast) => self.visit_while_statement(ast),
            Statement::WithStatement(ast) => self.visit_with_statement(ast),
            Statement::FunctionDeclaration(ast) => self.visit_function(ast),
            Statement::ClassDeclaration(ast) => self.visit_class_declaration(ast),
        }
    }

    pub fn visit_expression(&mut self, ast: &mut Expression<'alloc>) -> T::Value {
        match ast {
            Expression::MemberExpression(ast) => self.visit_member_expression(ast),
            Expression::ClassExpression(ast) => self.visit_class_expression(ast),
            Expression::LiteralBooleanExpression(ast) => self.visit_literal_boolean_expression(ast),
            Expression::LiteralInfinityExpression => T::Value::default(),
            Expression::LiteralNullExpression => T::Value::default(),
            Expression::LiteralNumericExpression(ast) => self.visit_literal_numeric_expression(ast),
            Expression::LiteralRegExpExpression(ast) => self.visit_literal_reg_exp_expression(ast),
            Expression::LiteralStringExpression(ast) => self.visit_literal_string_expression(ast),
            Expression::ArrayExpression(ast) => self.visit_array_expression(ast),
            Expression::ArrowExpression(ast) => self.visit_arrow_expression(ast),
            Expression::AssignmentExpression(ast) => self.visit_assignment_expression(ast),
            Expression::BinaryExpression(ast) => self.visit_binary_expression(ast),
            Expression::CallExpression(ast) => self.visit_call_expression(ast),
            Expression::CompoundAssignmentExpression(ast) => {
                self.visit_compound_assignment_expression(ast)
            }
            Expression::ConditionalExpression(ast) => self.visit_conditional_expression(ast),
            Expression::FunctionExpression(ast) => self.visit_function(ast),
            Expression::IdentifierExpression(ast) => self.visit_identifier_expression(ast),
            Expression::NewExpression(ast) => self.visit_new_expression(ast),
            Expression::NewTargetExpression => T::Value::default(),
            Expression::ObjectExpression(ast) => self.visit_object_expression(ast),
            Expression::UnaryExpression(ast) => self.visit_unary_expression(ast),
            Expression::TemplateExpression(ast) => self.visit_template_expression(ast),
            Expression::ThisExpression => T::Value::default(),
            Expression::UpdateExpression(ast) => self.visit_update_expression(ast),
            Expression::YieldExpression(ast) => self.visit_yield_expression(ast),
            Expression::YieldGeneratorExpression(ast) => self.visit_yield_generator_expression(ast),
            Expression::AwaitExpression(ast) => self.visit_await_expression(ast),
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
            ExpressionOrSuper::Super => T::Value::default(),
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

    pub fn visit_literal_boolean_expression(
        &mut self,
        ast: &mut LiteralBooleanExpression,
    ) -> T::Value {
        let a0 = &mut ast.value;
        self.pass.visit_literal_boolean_expression(a0)
    }

    pub fn visit_literal_numeric_expression(
        &mut self,
        ast: &mut LiteralNumericExpression,
    ) -> T::Value {
        let a0 = &mut ast.value;
        self.pass.visit_literal_numeric_expression(a0)
    }

    pub fn visit_literal_reg_exp_expression(
        &mut self,
        ast: &mut LiteralRegExpExpression<'alloc>,
    ) -> T::Value {
        let a0 = &mut ast.pattern;
        let a1 = &mut ast.global;
        let a2 = &mut ast.ignore_case;
        let a3 = &mut ast.multi_line;
        let a4 = &mut ast.sticky;
        let a5 = &mut ast.unicode;
        self.pass
            .visit_literal_reg_exp_expression(a0, a1, a2, a3, a4, a5)
    }

    pub fn visit_literal_string_expression(
        &mut self,
        ast: &mut LiteralStringExpression<'alloc>,
    ) -> T::Value {
        let a0 = &mut ast.value;
        self.pass.visit_literal_string_expression(a0)
    }

    pub fn visit_array_expression_element(
        &mut self,
        ast: &mut ArrayExpressionElement<'alloc>,
    ) -> T::Value {
        match ast {
            ArrayExpressionElement::SpreadElement(ast) => self.visit_expression(ast),
            ArrayExpressionElement::Expression(ast) => self.visit_expression(ast),
            ArrayExpressionElement::Elision => T::Value::default(),
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

    pub fn visit_arrow_expression(&mut self, ast: &mut ArrowExpression<'alloc>) -> T::Value {
        let a0 = &mut ast.is_async;
        let a1 = self.visit_formal_parameters((&mut ast.params));
        let a2 = self.visit_arrow_expression_body((&mut ast.body));
        self.pass.visit_arrow_expression(a0, a1, a2)
    }

    pub fn visit_assignment_expression(
        &mut self,
        ast: &mut AssignmentExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_assignment_target((&mut ast.binding));
        let a1 = self.visit_expression((&mut ast.expression));
        self.pass.visit_assignment_expression(a0, a1)
    }

    pub fn visit_binary_expression(&mut self, ast: &mut BinaryExpression<'alloc>) -> T::Value {
        let a0 = self.visit_binary_operator((&mut ast.operator));
        let a1 = self.visit_expression((&mut ast.left));
        let a2 = self.visit_expression((&mut ast.right));
        self.pass.visit_binary_expression(a0, a1, a2)
    }

    pub fn visit_call_expression(&mut self, ast: &mut CallExpression<'alloc>) -> T::Value {
        let a0 = self.visit_expression_or_super((&mut ast.callee));
        let a1 = self.visit_arguments((&mut ast.arguments));
        self.pass.visit_call_expression(a0, a1)
    }

    pub fn visit_compound_assignment_expression(
        &mut self,
        ast: &mut CompoundAssignmentExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_compound_assignment_operator((&mut ast.operator));
        let a1 = self.visit_simple_assignment_target((&mut ast.binding));
        let a2 = self.visit_expression((&mut ast.expression));
        self.pass.visit_compound_assignment_expression(a0, a1, a2)
    }

    pub fn visit_computed_member_expression(
        &mut self,
        ast: &mut ComputedMemberExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression_or_super((&mut ast.object));
        let a1 = self.visit_expression((&mut ast.expression));
        self.pass.visit_computed_member_expression(a0, a1)
    }

    pub fn visit_conditional_expression(
        &mut self,
        ast: &mut ConditionalExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression((&mut ast.test));
        let a1 = self.visit_expression((&mut ast.consequent));
        let a2 = self.visit_expression((&mut ast.alternate));
        self.pass.visit_conditional_expression(a0, a1, a2)
    }

    pub fn visit_identifier_expression(
        &mut self,
        ast: &mut IdentifierExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_identifier((&mut ast.name));
        self.pass.visit_identifier_expression(a0)
    }

    pub fn visit_new_expression(&mut self, ast: &mut NewExpression<'alloc>) -> T::Value {
        let a0 = self.visit_expression((&mut ast.callee));
        let a1 = self.visit_arguments((&mut ast.arguments));
        self.pass.visit_new_expression(a0, a1)
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

    pub fn visit_unary_expression(&mut self, ast: &mut UnaryExpression<'alloc>) -> T::Value {
        let a0 = self.visit_unary_operator((&mut ast.operator));
        let a1 = self.visit_expression((&mut ast.operand));
        self.pass.visit_unary_expression(a0, a1)
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

    pub fn visit_update_expression(&mut self, ast: &mut UpdateExpression<'alloc>) -> T::Value {
        let a0 = &mut ast.is_prefix;
        let a1 = self.visit_update_operator((&mut ast.operator));
        let a2 = self.visit_simple_assignment_target((&mut ast.operand));
        self.pass.visit_update_expression(a0, a1, a2)
    }

    pub fn visit_yield_expression(&mut self, ast: &mut YieldExpression<'alloc>) -> T::Value {
        let a0 = (&mut ast.expression)
            .as_mut()
            .map(|item| self.visit_expression(item));
        self.pass.visit_yield_expression(a0)
    }

    pub fn visit_yield_generator_expression(
        &mut self,
        ast: &mut YieldGeneratorExpression<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression((&mut ast.expression));
        self.pass.visit_yield_generator_expression(a0)
    }

    pub fn visit_await_expression(&mut self, ast: &mut AwaitExpression<'alloc>) -> T::Value {
        let a0 = self.visit_expression((&mut ast.expression));
        self.pass.visit_await_expression(a0)
    }

    pub fn visit_block_statement(&mut self, ast: &mut BlockStatement<'alloc>) -> T::Value {
        let a0 = self.visit_block((&mut ast.block));
        self.pass.visit_block_statement(a0)
    }

    pub fn visit_break_statement(&mut self, ast: &mut BreakStatement<'alloc>) -> T::Value {
        let a0 = (&mut ast.label).as_mut().map(|item| self.visit_label(item));
        self.pass.visit_break_statement(a0)
    }

    pub fn visit_continue_statement(&mut self, ast: &mut ContinueStatement<'alloc>) -> T::Value {
        let a0 = (&mut ast.label).as_mut().map(|item| self.visit_label(item));
        self.pass.visit_continue_statement(a0)
    }

    pub fn visit_do_while_statement(&mut self, ast: &mut DoWhileStatement<'alloc>) -> T::Value {
        let a0 = self.visit_statement((&mut ast.block));
        let a1 = self.visit_expression((&mut ast.test));
        self.pass.visit_do_while_statement(a0, a1)
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

    pub fn visit_for_in_statement(&mut self, ast: &mut ForInStatement<'alloc>) -> T::Value {
        let a0 = self.visit_variable_declaration_or_assignment_target((&mut ast.left));
        let a1 = self.visit_expression((&mut ast.right));
        let a2 = self.visit_statement((&mut ast.block));
        self.pass.visit_for_in_statement(a0, a1, a2)
    }

    pub fn visit_for_of_statement(&mut self, ast: &mut ForOfStatement<'alloc>) -> T::Value {
        let a0 = self.visit_variable_declaration_or_assignment_target((&mut ast.left));
        let a1 = self.visit_expression((&mut ast.right));
        let a2 = self.visit_statement((&mut ast.block));
        self.pass.visit_for_of_statement(a0, a1, a2)
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

    pub fn visit_for_statement(&mut self, ast: &mut ForStatement<'alloc>) -> T::Value {
        let a0 = (&mut ast.init)
            .as_mut()
            .map(|item| self.visit_variable_declaration_or_expression(item));
        let a1 = (&mut ast.test)
            .as_mut()
            .map(|item| self.visit_expression(item));
        let a2 = (&mut ast.update)
            .as_mut()
            .map(|item| self.visit_expression(item));
        let a3 = self.visit_statement((&mut ast.block));
        self.pass.visit_for_statement(a0, a1, a2, a3)
    }

    pub fn visit_if_statement(&mut self, ast: &mut IfStatement<'alloc>) -> T::Value {
        let a0 = self.visit_expression((&mut ast.test));
        let a1 = self.visit_statement((&mut ast.consequent));
        let a2 = (&mut ast.alternate)
            .as_mut()
            .map(|item| self.visit_statement(item));
        self.pass.visit_if_statement(a0, a1, a2)
    }

    pub fn visit_labeled_statement(&mut self, ast: &mut LabeledStatement<'alloc>) -> T::Value {
        let a0 = self.visit_label((&mut ast.label));
        let a1 = self.visit_statement((&mut ast.body));
        self.pass.visit_labeled_statement(a0, a1)
    }

    pub fn visit_return_statement(&mut self, ast: &mut ReturnStatement<'alloc>) -> T::Value {
        let a0 = (&mut ast.expression)
            .as_mut()
            .map(|item| self.visit_expression(item));
        self.pass.visit_return_statement(a0)
    }

    pub fn visit_switch_statement(&mut self, ast: &mut SwitchStatement<'alloc>) -> T::Value {
        let a0 = self.visit_expression((&mut ast.discriminant));
        let a1 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.cases),
                |item| self.visit_switch_case(item),
                allocator,
            )
        };
        self.pass.visit_switch_statement(a0, a1)
    }

    pub fn visit_switch_statement_with_default(
        &mut self,
        ast: &mut SwitchStatementWithDefault<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_expression((&mut ast.discriminant));
        let a1 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.pre_default_cases),
                |item| self.visit_switch_case(item),
                allocator,
            )
        };
        let a2 = self.visit_switch_default((&mut ast.default_case));
        let a3 = {
            let allocator: &bumpalo::Bump = self.allocator;
            arena::map_vec(
                (&mut ast.post_default_cases),
                |item| self.visit_switch_case(item),
                allocator,
            )
        };
        self.pass
            .visit_switch_statement_with_default(a0, a1, a2, a3)
    }

    pub fn visit_throw_statement(&mut self, ast: &mut ThrowStatement<'alloc>) -> T::Value {
        let a0 = self.visit_expression((&mut ast.expression));
        self.pass.visit_throw_statement(a0)
    }

    pub fn visit_try_catch_statement(&mut self, ast: &mut TryCatchStatement<'alloc>) -> T::Value {
        let a0 = self.visit_block((&mut ast.body));
        let a1 = self.visit_catch_clause((&mut ast.catch_clause));
        self.pass.visit_try_catch_statement(a0, a1)
    }

    pub fn visit_try_finally_statement(
        &mut self,
        ast: &mut TryFinallyStatement<'alloc>,
    ) -> T::Value {
        let a0 = self.visit_block((&mut ast.body));
        let a1 = (&mut ast.catch_clause)
            .as_mut()
            .map(|item| self.visit_catch_clause(item));
        let a2 = self.visit_block((&mut ast.finalizer));
        self.pass.visit_try_finally_statement(a0, a1, a2)
    }

    pub fn visit_while_statement(&mut self, ast: &mut WhileStatement<'alloc>) -> T::Value {
        let a0 = self.visit_expression((&mut ast.test));
        let a1 = self.visit_statement((&mut ast.block));
        self.pass.visit_while_statement(a0, a1)
    }

    pub fn visit_with_statement(&mut self, ast: &mut WithStatement<'alloc>) -> T::Value {
        let a0 = self.visit_expression((&mut ast.object));
        let a1 = self.visit_statement((&mut ast.body));
        self.pass.visit_with_statement(a0, a1)
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
        let a0 = self.visit_binding((&mut ast.binding));
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
            CoverParenthesized::Expression(ast) => self.visit_expression(ast),
            CoverParenthesized::Parameters(ast) => self.visit_formal_parameters(ast),
        }
    }
}
