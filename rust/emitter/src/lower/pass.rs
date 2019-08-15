// WARNING: This file is auto-generated.

use ast::*;

pub trait Pass {
    fn visit_argument(&mut self, ast: &mut Argument) {
        match ast {
            Argument::SpreadElement(ast) => {
                self.visit_expression(ast);
            }
            Argument::Expression(ast) => {
                self.visit_expression(ast);
            }
        }
    }

    fn visit_arguments(&mut self, ast: &mut Arguments) {
        for item in &mut ast.args {
            self.visit_argument(item);
        }
    }

    fn visit_identifier(&mut self, ast: &mut Identifier) {
    }

    fn visit_identifier_name(&mut self, ast: &mut IdentifierName) {
    }

    fn visit_label(&mut self, ast: &mut Label) {
    }

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

    fn visit_function(&mut self, ast: &mut Function) {
        if let Some(item) = &mut ast.name {
            self.visit_binding_identifier(item);
        }
        self.visit_formal_parameters(&mut ast.params);
        self.visit_function_body(&mut ast.body);
    }

    fn visit_program(&mut self, ast: &mut Program) {
        match ast {
            Program::Module(ast) => {
                self.visit_module(ast);
            }
            Program::Script(ast) => {
                self.visit_script(ast);
            }
        }
    }

    fn visit_statement(&mut self, ast: &mut Statement) {
        match ast {
            Statement::IterationStatement(ast) => {
                self.visit_iteration_statement(ast);
            }
            Statement::ClassDeclaration(ast) => {
                self.visit_class_declaration(ast);
            }
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
            Statement::EmptyStatement => (),
            Statement::ExpressionStatement(ast) => {
                self.visit_expression(ast);
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
            Statement::WithStatement(ast) => {
                self.visit_with_statement(ast);
            }
            Statement::FunctionDeclaration(ast) => {
                self.visit_function(ast);
            }
        }
    }

    fn visit_iteration_statement(&mut self, ast: &mut IterationStatement) {
        match ast {
            IterationStatement::DoWhileStatement(ast) => {
                self.visit_do_while_statement(ast);
            }
            IterationStatement::ForInStatement(ast) => {
                self.visit_for_in_statement(ast);
            }
            IterationStatement::ForOfStatement(ast) => {
                self.visit_for_of_statement(ast);
            }
            IterationStatement::ForStatement(ast) => {
                self.visit_for_statement(ast);
            }
            IterationStatement::WhileStatement(ast) => {
                self.visit_while_statement(ast);
            }
        }
    }

    fn visit_expression(&mut self, ast: &mut Expression) {
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

    fn visit_member_expression(&mut self, ast: &mut MemberExpression) {
        match ast {
            MemberExpression::ComputedMemberExpression(ast) => {
                self.visit_computed_member_expression(ast);
            }
            MemberExpression::StaticMemberExpression(ast) => {
                self.visit_static_member_expression(ast);
            }
        }
    }

    fn visit_property_name(&mut self, ast: &mut PropertyName) {
        match ast {
            PropertyName::ComputedPropertyName(ast) => {
                self.visit_computed_property_name(ast);
            }
            PropertyName::StaticPropertyName(ast) => {
                self.visit_static_property_name(ast);
            }
        }
    }

    fn visit_object_property(&mut self, ast: &mut ObjectProperty) {
        match ast {
            ObjectProperty::NamedObjectProperty(ast) => {
                self.visit_named_object_property(ast);
            }
            ObjectProperty::ShorthandProperty(ast) => {
                self.visit_shorthand_property(ast);
            }
        }
    }

    fn visit_named_object_property(&mut self, ast: &mut NamedObjectProperty) {
        match ast {
            NamedObjectProperty::MethodDefinition(ast) => {
                self.visit_method_definition(ast);
            }
            NamedObjectProperty::DataProperty(ast) => {
                self.visit_data_property(ast);
            }
        }
    }

    fn visit_method_definition(&mut self, ast: &mut MethodDefinition) {
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

    fn visit_import_declaration(&mut self, ast: &mut ImportDeclaration) {
        match ast {
            ImportDeclaration::Import(ast) => {
                self.visit_import(ast);
            }
            ImportDeclaration::ImportNamespace(ast) => {
                self.visit_import_namespace(ast);
            }
        }
    }

    fn visit_export_declaration(&mut self, ast: &mut ExportDeclaration) {
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

    fn visit_variable_reference(&mut self, ast: &mut VariableReference) {
        match ast {
            VariableReference::BindingIdentifier(ast) => {
                self.visit_binding_identifier(ast);
            }
            VariableReference::AssignmentTargetIdentifier(ast) => {
                self.visit_assignment_target_identifier(ast);
            }
        }
    }

    fn visit_binding_pattern(&mut self, ast: &mut BindingPattern) {
        match ast {
            BindingPattern::ObjectBinding(ast) => {
                self.visit_object_binding(ast);
            }
            BindingPattern::ArrayBinding(ast) => {
                self.visit_array_binding(ast);
            }
        }
    }

    fn visit_binding(&mut self, ast: &mut Binding) {
        match ast {
            Binding::BindingPattern(ast) => {
                self.visit_binding_pattern(ast);
            }
            Binding::BindingIdentifier(ast) => {
                self.visit_binding_identifier(ast);
            }
        }
    }

    fn visit_simple_assignment_target(&mut self, ast: &mut SimpleAssignmentTarget) {
        match ast {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(ast) => {
                self.visit_assignment_target_identifier(ast);
            }
            SimpleAssignmentTarget::MemberAssignmentTarget(ast) => {
                self.visit_member_assignment_target(ast);
            }
        }
    }

    fn visit_assignment_target_pattern(&mut self, ast: &mut AssignmentTargetPattern) {
        match ast {
            AssignmentTargetPattern::ArrayAssignmentTarget(ast) => {
                self.visit_array_assignment_target(ast);
            }
            AssignmentTargetPattern::ObjectAssignmentTarget(ast) => {
                self.visit_object_assignment_target(ast);
            }
        }
    }

    fn visit_assignment_target(&mut self, ast: &mut AssignmentTarget) {
        match ast {
            AssignmentTarget::AssignmentTargetPattern(ast) => {
                self.visit_assignment_target_pattern(ast);
            }
            AssignmentTarget::SimpleAssignmentTarget(ast) => {
                self.visit_simple_assignment_target(ast);
            }
        }
    }

    fn visit_parameter(&mut self, ast: &mut Parameter) {
        match ast {
            Parameter::Binding(ast) => {
                self.visit_binding(ast);
            }
            Parameter::BindingWithDefault(ast) => {
                self.visit_binding_with_default(ast);
            }
        }
    }

    fn visit_binding_with_default(&mut self, ast: &mut BindingWithDefault) {
        self.visit_binding(&mut ast.binding);
        self.visit_expression(&mut ast.init);
    }

    fn visit_binding_identifier(&mut self, ast: &mut BindingIdentifier) {
        self.visit_identifier(&mut ast.name);
    }

    fn visit_assignment_target_identifier(&mut self, ast: &mut AssignmentTargetIdentifier) {
        self.visit_identifier(&mut ast.name);
    }

    fn visit_expression_or_super(&mut self, ast: &mut ExpressionOrSuper) {
        match ast {
            ExpressionOrSuper::Expression(ast) => {
                self.visit_expression(ast);
            }
            ExpressionOrSuper::Super => (),
        }
    }

    fn visit_member_assignment_target(&mut self, ast: &mut MemberAssignmentTarget) {
        match ast {
            MemberAssignmentTarget::ComputedMemberAssignmentTarget(ast) => {
                self.visit_computed_member_assignment_target(ast);
            }
            MemberAssignmentTarget::StaticMemberAssignmentTarget(ast) => {
                self.visit_static_member_assignment_target(ast);
            }
        }
    }

    fn visit_computed_member_assignment_target(&mut self, ast: &mut ComputedMemberAssignmentTarget) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_static_member_assignment_target(&mut self, ast: &mut StaticMemberAssignmentTarget) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_identifier_name(&mut ast.property);
    }

    fn visit_array_binding(&mut self, ast: &mut ArrayBinding) {
        for item in &mut ast.elements {
            if let Some(item) = item {
                self.visit_parameter(item);
            }
        }
        if let Some(item) = &mut ast.rest {
            self.visit_binding(item);
        }
    }

    fn visit_object_binding(&mut self, ast: &mut ObjectBinding) {
        for item in &mut ast.properties {
            self.visit_binding_property(item);
        }
    }

    fn visit_binding_property(&mut self, ast: &mut BindingProperty) {
        match ast {
            BindingProperty::BindingPropertyIdentifier(ast) => {
                self.visit_binding_property_identifier(ast);
            }
            BindingProperty::BindingPropertyProperty(ast) => {
                self.visit_binding_property_property(ast);
            }
        }
    }

    fn visit_binding_property_identifier(&mut self, ast: &mut BindingPropertyIdentifier) {
        self.visit_binding_identifier(&mut ast.binding);
        if let Some(item) = &mut ast.init {
            self.visit_expression(item);
        }
    }

    fn visit_binding_property_property(&mut self, ast: &mut BindingPropertyProperty) {
        self.visit_property_name(&mut ast.name);
        self.visit_parameter(&mut ast.binding);
    }

    fn visit_assignment_target_with_default(&mut self, ast: &mut AssignmentTargetWithDefault) {
        self.visit_assignment_target(&mut ast.binding);
        self.visit_expression(&mut ast.init);
    }

    fn visit_assignment_target_maybe_default(&mut self, ast: &mut AssignmentTargetMaybeDefault) {
        match ast {
            AssignmentTargetMaybeDefault::AssignmentTarget(ast) => {
                self.visit_assignment_target(ast);
            }
            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(ast) => {
                self.visit_assignment_target_with_default(ast);
            }
        }
    }

    fn visit_array_assignment_target(&mut self, ast: &mut ArrayAssignmentTarget) {
        for item in &mut ast.elements {
            if let Some(item) = item {
                self.visit_assignment_target_maybe_default(item);
            }
        }
        if let Some(item) = &mut ast.rest {
            self.visit_assignment_target(item);
        }
    }

    fn visit_object_assignment_target(&mut self, ast: &mut ObjectAssignmentTarget) {
        for item in &mut ast.properties {
            self.visit_assignment_target_property(item);
        }
    }

    fn visit_assignment_target_property(&mut self, ast: &mut AssignmentTargetProperty) {
        match ast {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(ast) => {
                self.visit_assignment_target_property_identifier(ast);
            }
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(ast) => {
                self.visit_assignment_target_property_property(ast);
            }
        }
    }

    fn visit_assignment_target_property_identifier(&mut self, ast: &mut AssignmentTargetPropertyIdentifier) {
        self.visit_assignment_target_identifier(&mut ast.binding);
        if let Some(item) = &mut ast.init {
            self.visit_expression(item);
        }
    }

    fn visit_assignment_target_property_property(&mut self, ast: &mut AssignmentTargetPropertyProperty) {
        self.visit_property_name(&mut ast.name);
        self.visit_assignment_target_maybe_default(&mut ast.binding);
    }

    fn visit_class_expression(&mut self, ast: &mut ClassExpression) {
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

    fn visit_class_declaration(&mut self, ast: &mut ClassDeclaration) {
        self.visit_binding_identifier(&mut ast.name);
        if let Some(item) = &mut ast.super_ {
            self.visit_expression(item);
        }
        for item in &mut ast.elements {
            self.visit_class_element(item);
        }
    }

    fn visit_class_element(&mut self, ast: &mut ClassElement) {
        self.visit_property_name(&mut ast.property_name);
        self.visit_method_definition(&mut ast.method);
    }

    fn visit_module_items(&mut self, ast: &mut ModuleItems) {
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

    fn visit_module(&mut self, ast: &mut Module) {
        for item in &mut ast.directives {
            self.visit_directive(item);
        }
        for item in &mut ast.items {
            self.visit_module_items(item);
        }
    }

    fn visit_import(&mut self, ast: &mut Import) {
        if let Some(item) = &mut ast.default_binding {
            self.visit_binding_identifier(item);
        }
        for item in &mut ast.named_imports {
            self.visit_import_specifier(item);
        }
    }

    fn visit_import_namespace(&mut self, ast: &mut ImportNamespace) {
        if let Some(item) = &mut ast.default_binding {
            self.visit_binding_identifier(item);
        }
        self.visit_binding_identifier(&mut ast.namespace_binding);
    }

    fn visit_import_specifier(&mut self, ast: &mut ImportSpecifier) {
        if let Some(item) = &mut ast.name {
            self.visit_identifier_name(item);
        }
        self.visit_binding_identifier(&mut ast.binding);
    }

    fn visit_export_all_from(&mut self, ast: &mut ExportAllFrom) {
    }

    fn visit_export_from(&mut self, ast: &mut ExportFrom) {
        for item in &mut ast.named_exports {
            self.visit_export_from_specifier(item);
        }
    }

    fn visit_export_locals(&mut self, ast: &mut ExportLocals) {
        for item in &mut ast.named_exports {
            self.visit_export_local_specifier(item);
        }
    }

    fn visit_export(&mut self, ast: &mut Export) {
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

    fn visit_export_default(&mut self, ast: &mut ExportDefault) {
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

    fn visit_export_from_specifier(&mut self, ast: &mut ExportFromSpecifier) {
        self.visit_identifier_name(&mut ast.name);
        if let Some(item) = &mut ast.exported_name {
            self.visit_identifier_name(item);
        }
    }

    fn visit_export_local_specifier(&mut self, ast: &mut ExportLocalSpecifier) {
        self.visit_identifier_expression(&mut ast.name);
        if let Some(item) = &mut ast.exported_name {
            self.visit_identifier_name(item);
        }
    }

    fn visit_method(&mut self, ast: &mut Method) {
        self.visit_property_name(&mut ast.name);
        self.visit_formal_parameters(&mut ast.params);
        self.visit_function_body(&mut ast.body);
    }

    fn visit_getter(&mut self, ast: &mut Getter) {
        self.visit_property_name(&mut ast.property_name);
        self.visit_function_body(&mut ast.body);
    }

    fn visit_setter(&mut self, ast: &mut Setter) {
        self.visit_property_name(&mut ast.property_name);
        self.visit_parameter(&mut ast.param);
        self.visit_function_body(&mut ast.body);
    }

    fn visit_data_property(&mut self, ast: &mut DataProperty) {
        self.visit_property_name(&mut ast.property_name);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_shorthand_property(&mut self, ast: &mut ShorthandProperty) {
        self.visit_identifier_expression(&mut ast.name);
    }

    fn visit_computed_property_name(&mut self, ast: &mut ComputedPropertyName) {
        self.visit_expression(&mut ast.expression);
    }

    fn visit_static_property_name(&mut self, ast: &mut StaticPropertyName) {
    }

    fn visit_literal_boolean_expression(&mut self, ast: &mut LiteralBooleanExpression) {
    }

    fn visit_literal_numeric_expression(&mut self, ast: &mut LiteralNumericExpression) {
    }

    fn visit_literal_reg_exp_expression(&mut self, ast: &mut LiteralRegExpExpression) {
    }

    fn visit_literal_string_expression(&mut self, ast: &mut LiteralStringExpression) {
    }

    fn visit_array_expression_element(&mut self, ast: &mut ArrayExpressionElement) {
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

    fn visit_array_expression(&mut self, ast: &mut ArrayExpression) {
        for item in &mut ast.elements {
            self.visit_array_expression_element(item);
        }
    }

    fn visit_arrow_expression_body(&mut self, ast: &mut ArrowExpressionBody) {
        match ast {
            ArrowExpressionBody::FunctionBody(ast) => {
                self.visit_function_body(ast);
            }
            ArrowExpressionBody::Expression(ast) => {
                self.visit_expression(ast);
            }
        }
    }

    fn visit_arrow_expression(&mut self, ast: &mut ArrowExpression) {
        self.visit_formal_parameters(&mut ast.params);
        self.visit_arrow_expression_body(&mut ast.body);
    }

    fn visit_assignment_expression(&mut self, ast: &mut AssignmentExpression) {
        self.visit_assignment_target(&mut ast.binding);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_binary_expression(&mut self, ast: &mut BinaryExpression) {
        self.visit_binary_operator(&mut ast.operator);
        self.visit_expression(&mut ast.left);
        self.visit_expression(&mut ast.right);
    }

    fn visit_call_expression(&mut self, ast: &mut CallExpression) {
        self.visit_expression_or_super(&mut ast.callee);
        self.visit_arguments(&mut ast.arguments);
    }

    fn visit_compound_assignment_expression(&mut self, ast: &mut CompoundAssignmentExpression) {
        self.visit_compound_assignment_operator(&mut ast.operator);
        self.visit_simple_assignment_target(&mut ast.binding);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_computed_member_expression(&mut self, ast: &mut ComputedMemberExpression) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_expression(&mut ast.expression);
    }

    fn visit_conditional_expression(&mut self, ast: &mut ConditionalExpression) {
        self.visit_expression(&mut ast.test);
        self.visit_expression(&mut ast.consequent);
        self.visit_expression(&mut ast.alternate);
    }

    fn visit_identifier_expression(&mut self, ast: &mut IdentifierExpression) {
        self.visit_variable_reference(&mut ast.var);
    }

    fn visit_new_expression(&mut self, ast: &mut NewExpression) {
        self.visit_expression(&mut ast.callee);
        self.visit_arguments(&mut ast.arguments);
    }

    fn visit_object_expression(&mut self, ast: &mut ObjectExpression) {
        for item in &mut ast.properties {
            self.visit_object_property(item);
        }
    }

    fn visit_unary_expression(&mut self, ast: &mut UnaryExpression) {
        self.visit_unary_operator(&mut ast.operator);
        self.visit_expression(&mut ast.operand);
    }

    fn visit_static_member_expression(&mut self, ast: &mut StaticMemberExpression) {
        self.visit_expression_or_super(&mut ast.object);
        self.visit_identifier_name(&mut ast.property);
    }

    fn visit_template_expression_element(&mut self, ast: &mut TemplateExpressionElement) {
        match ast {
            TemplateExpressionElement::Expression(ast) => {
                self.visit_expression(ast);
            }
            TemplateExpressionElement::TemplateElement(ast) => {
                self.visit_template_element(ast);
            }
        }
    }

    fn visit_template_expression(&mut self, ast: &mut TemplateExpression) {
        if let Some(item) = &mut ast.tag {
            self.visit_expression(item);
        }
        for item in &mut ast.elements {
            self.visit_template_expression_element(item);
        }
    }

    fn visit_update_expression(&mut self, ast: &mut UpdateExpression) {
        self.visit_update_operator(&mut ast.operator);
        self.visit_simple_assignment_target(&mut ast.operand);
    }

    fn visit_yield_expression(&mut self, ast: &mut YieldExpression) {
        if let Some(item) = &mut ast.expression {
            self.visit_expression(item);
        }
    }

    fn visit_yield_generator_expression(&mut self, ast: &mut YieldGeneratorExpression) {
        self.visit_expression(&mut ast.expression);
    }

    fn visit_await_expression(&mut self, ast: &mut AwaitExpression) {
        self.visit_expression(&mut ast.expression);
    }

    fn visit_block_statement(&mut self, ast: &mut BlockStatement) {
        self.visit_block(&mut ast.block);
    }

    fn visit_break_statement(&mut self, ast: &mut BreakStatement) {
        if let Some(item) = &mut ast.label {
            self.visit_label(item);
        }
    }

    fn visit_continue_statement(&mut self, ast: &mut ContinueStatement) {
        if let Some(item) = &mut ast.label {
            self.visit_label(item);
        }
    }

    fn visit_do_while_statement(&mut self, ast: &mut DoWhileStatement) {
        self.visit_statement(&mut ast.block);
        self.visit_expression(&mut ast.test);
    }

    fn visit_variable_declaration_or_assignment_target(&mut self, ast: &mut VariableDeclarationOrAssignmentTarget) {
        match ast {
            VariableDeclarationOrAssignmentTarget::VariableDeclaration(ast) => {
                self.visit_variable_declaration(ast);
            }
            VariableDeclarationOrAssignmentTarget::AssignmentTarget(ast) => {
                self.visit_assignment_target(ast);
            }
        }
    }

    fn visit_for_in_statement(&mut self, ast: &mut ForInStatement) {
        self.visit_variable_declaration_or_assignment_target(&mut ast.left);
        self.visit_expression(&mut ast.right);
        self.visit_statement(&mut ast.block);
    }

    fn visit_for_of_statement(&mut self, ast: &mut ForOfStatement) {
        self.visit_variable_declaration_or_assignment_target(&mut ast.left);
        self.visit_expression(&mut ast.right);
        self.visit_statement(&mut ast.block);
    }

    fn visit_variable_declaration_or_expression(&mut self, ast: &mut VariableDeclarationOrExpression) {
        match ast {
            VariableDeclarationOrExpression::VariableDeclaration(ast) => {
                self.visit_variable_declaration(ast);
            }
            VariableDeclarationOrExpression::Expression(ast) => {
                self.visit_expression(ast);
            }
        }
    }

    fn visit_for_statement(&mut self, ast: &mut ForStatement) {
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

    fn visit_if_statement(&mut self, ast: &mut IfStatement) {
        self.visit_expression(&mut ast.test);
        self.visit_statement(&mut ast.consequent);
        if let Some(item) = &mut ast.alternate {
            self.visit_statement(item);
        }
    }

    fn visit_labeled_statement(&mut self, ast: &mut LabeledStatement) {
        self.visit_label(&mut ast.label);
        self.visit_statement(&mut ast.body);
    }

    fn visit_return_statement(&mut self, ast: &mut ReturnStatement) {
        if let Some(item) = &mut ast.expression {
            self.visit_expression(item);
        }
    }

    fn visit_switch_statement(&mut self, ast: &mut SwitchStatement) {
        self.visit_expression(&mut ast.discriminant);
        for item in &mut ast.cases {
            self.visit_switch_case(item);
        }
    }

    fn visit_switch_statement_with_default(&mut self, ast: &mut SwitchStatementWithDefault) {
        self.visit_expression(&mut ast.discriminant);
        for item in &mut ast.pre_default_cases {
            self.visit_switch_case(item);
        }
        self.visit_switch_default(&mut ast.default_case);
        for item in &mut ast.post_default_cases {
            self.visit_switch_case(item);
        }
    }

    fn visit_throw_statement(&mut self, ast: &mut ThrowStatement) {
        self.visit_expression(&mut ast.expression);
    }

    fn visit_try_catch_statement(&mut self, ast: &mut TryCatchStatement) {
        self.visit_block(&mut ast.body);
        self.visit_catch_clause(&mut ast.catch_clause);
    }

    fn visit_try_finally_statement(&mut self, ast: &mut TryFinallyStatement) {
        self.visit_block(&mut ast.body);
        if let Some(item) = &mut ast.catch_clause {
            self.visit_catch_clause(item);
        }
        self.visit_block(&mut ast.finalizer);
    }

    fn visit_while_statement(&mut self, ast: &mut WhileStatement) {
        self.visit_expression(&mut ast.test);
        self.visit_statement(&mut ast.block);
    }

    fn visit_with_statement(&mut self, ast: &mut WithStatement) {
        self.visit_expression(&mut ast.object);
        self.visit_statement(&mut ast.body);
    }

    fn visit_block(&mut self, ast: &mut Block) {
        for item in &mut ast.statements {
            self.visit_statement(item);
        }
        if let Some(item) = &mut ast.declarations {
            for item in item {
            }
        }
    }

    fn visit_catch_clause(&mut self, ast: &mut CatchClause) {
        self.visit_binding(&mut ast.binding);
        self.visit_block(&mut ast.body);
    }

    fn visit_directive(&mut self, ast: &mut Directive) {
    }

    fn visit_formal_parameters(&mut self, ast: &mut FormalParameters) {
        for item in &mut ast.items {
            self.visit_parameter(item);
        }
        if let Some(item) = &mut ast.rest {
            self.visit_binding(item);
        }
    }

    fn visit_function_body(&mut self, ast: &mut FunctionBody) {
        for item in &mut ast.directives {
            self.visit_directive(item);
        }
        for item in &mut ast.statements {
            self.visit_statement(item);
        }
    }

    fn visit_script(&mut self, ast: &mut Script) {
        for item in &mut ast.directives {
            self.visit_directive(item);
        }
        for item in &mut ast.statements {
            self.visit_statement(item);
        }
    }

    fn visit_switch_case(&mut self, ast: &mut SwitchCase) {
        self.visit_expression(&mut ast.test);
        for item in &mut ast.consequent {
            self.visit_statement(item);
        }
    }

    fn visit_switch_default(&mut self, ast: &mut SwitchDefault) {
        for item in &mut ast.consequent {
            self.visit_statement(item);
        }
    }

    fn visit_template_element(&mut self, ast: &mut TemplateElement) {
    }

    fn visit_variable_declaration(&mut self, ast: &mut VariableDeclaration) {
        self.visit_variable_declaration_kind(&mut ast.kind);
        for item in &mut ast.declarators {
            self.visit_variable_declarator(item);
        }
    }

    fn visit_variable_declarator(&mut self, ast: &mut VariableDeclarator) {
        self.visit_binding(&mut ast.binding);
        if let Some(item) = &mut ast.init {
            self.visit_expression(item);
        }
    }

}

