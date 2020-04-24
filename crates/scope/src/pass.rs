//! Code to traverse the AST and drive the rest of scope analysis.
//!
//! This module is responsible for walking the AST. Other modules do the actual
//! analysis.
//!
//! Scope analysis is currently a second pass, after parsing, using the AST,
//! but the goal is to do this analysis as part of the parse phase, even when
//! no AST is built. So we try to keep AST use separate from the analysis code.

use crate::context::ScopeContext;
use crate::data::ScopeDataMap;
use ast::associated_data::AssociatedData;
use ast::{types::*, visit::Pass};

/// The result of scope analysis.
pub struct ScopeDataMapAndFunctionMap<'alloc> {
    pub scope_data_map: ScopeDataMap,
    pub function_map: AssociatedData<&'alloc Function<'alloc>>,
}

/// The top-level struct responsible for extracting the necessary information
/// from the AST. The analysis itself is done mainly by the `ScopeContext`,
/// which has very limited interaction with the AST.
///
/// FIXME: This should be rewritten as a grammar extension.
#[derive(Debug)]
pub struct ScopePass<'alloc> {
    context: ScopeContext,
    function_map: AssociatedData<&'alloc Function<'alloc>>,
}

impl<'alloc> ScopePass<'alloc> {
    pub fn new() -> Self {
        Self {
            context: ScopeContext::new(),
            function_map: AssociatedData::new(),
        }
    }
}

impl<'alloc> From<ScopePass<'alloc>> for ScopeDataMapAndFunctionMap<'alloc> {
    fn from(pass: ScopePass<'alloc>) -> ScopeDataMapAndFunctionMap<'alloc> {
        ScopeDataMapAndFunctionMap {
            scope_data_map: pass.context.into(),
            function_map: pass.function_map,
        }
    }
}

impl<'alloc> Pass<'alloc> for ScopePass<'alloc> {
    fn enter_script(&mut self, _ast: &'alloc Script<'alloc>) {
        self.context.before_script();
    }

    fn leave_script(&mut self, _ast: &'alloc Script<'alloc>) {
        self.context.after_script();
    }

    fn enter_enum_statement_variant_block_statement(&mut self, block: &'alloc Block<'alloc>) {
        self.context.before_block_statement(block);
    }

    fn leave_enum_statement_variant_block_statement(&mut self, _block: &'alloc Block<'alloc>) {
        self.context.after_block_statement();
    }

    fn enter_variable_declaration(&mut self, ast: &'alloc VariableDeclaration<'alloc>) {
        match ast.kind {
            VariableDeclarationKind::Var { .. } => {
                self.context.before_var_declaration();
            }
            VariableDeclarationKind::Let { .. } => {
                self.context.before_let_declaration();
            }
            VariableDeclarationKind::Const { .. } => {
                self.context.before_const_declaration();
            }
        }
    }

    fn leave_variable_declaration(&mut self, ast: &'alloc VariableDeclaration<'alloc>) {
        match ast.kind {
            VariableDeclarationKind::Var { .. } => {
                self.context.after_var_declaration();
            }
            VariableDeclarationKind::Let { .. } => {
                self.context.after_let_declaration();
            }
            VariableDeclarationKind::Const { .. } => {
                self.context.after_const_declaration();
            }
        }
    }

    fn visit_binding_identifier(&mut self, ast: &'alloc BindingIdentifier) {
        // NOTE: The following should be handled at the parent node,
        // without visiting BindingIdentifier field:
        //   * Function.name
        //   * ClassExpression.name
        //   * ClassDeclaration.name
        //   * Import.default_binding
        //   * ImportNamespace.default_binding
        //   * ImportNamespace.namespace_binding
        //   * ImportSpecifier.binding
        self.context.on_binding_identifier(ast.name.value);
    }

    // Given we override `visit_binding_identifier` above,
    // visit_identifier is not called for Identifier inside BindingIdentifier,
    // and this is called only for references, either
    // IdentifierExpression or AssignmentTargetIdentifier.
    fn visit_identifier(&mut self, ast: &'alloc Identifier) {
        self.context.on_non_binding_identifier(ast.value);
    }

    fn enter_enum_statement_variant_function_declaration(&mut self, ast: &'alloc Function<'alloc>) {
        let name = if let Some(ref name) = ast.name {
            name.name.value
        } else {
            panic!("FunctionDeclaration should have name");
        };
        self.context.before_function_declaration(name, ast);
        self.function_map.insert(ast, ast);
    }
}
