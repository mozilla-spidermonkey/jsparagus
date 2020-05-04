use crate::context_stack::{
    BindingInfo, BindingKind, BindingsIndex, BreakOrContinueIndex, ContextMetadata, LabelIndex,
    LabelInfo, LabelKind,
};
use crate::declaration_kind::DeclarationKind;
use crate::early_errors::*;
use crate::error::{ParseError, Result};
use crate::Token;
use ast::arena;
use ast::source_atom_set::{SourceAtomSet, SourceAtomSetIndex};
use std::cell::RefCell;
use std::rc::Rc;

pub trait EarlyErrorChecker<'alloc> {
    fn context_metadata_mut(&mut self) -> &mut ContextMetadata;
    fn context_metadata(&self) -> &ContextMetadata;
    fn atoms(&self) -> &Rc<RefCell<SourceAtomSet<'alloc>>>;

    // Check Early Error for BindingIdentifier and note binding info to the
    // stack.
    fn on_binding_identifier(&mut self, token: &arena::Box<'alloc, Token>) -> Result<'alloc, ()> {
        let context = IdentifierEarlyErrorsContext::new();
        context.check_binding_identifier(token, &self.atoms().borrow())?;

        let name = token.value.as_atom();
        let offset = token.loc.start;

        if let Some(info) = self.context_metadata_mut().last_binding() {
            debug_assert!(info.offset < offset);
        }

        self.context_metadata_mut().push_binding(BindingInfo {
            name,
            offset,
            kind: BindingKind::Unknown,
        });

        Ok(())
    }

    // Check Early Error for IdentifierReference.
    fn on_identifier_reference(&self, token: &arena::Box<'alloc, Token>) -> Result<'alloc, ()> {
        let context = IdentifierEarlyErrorsContext::new();
        context.check_identifier_reference(token, &self.atoms().borrow())
    }

    // Check Early Error for LabelIdentifier and note binding info to the
    // stack
    fn on_label_identifier(&mut self, token: &arena::Box<'alloc, Token>) -> Result<'alloc, ()> {
        let context = IdentifierEarlyErrorsContext::new();

        let name = token.value.as_atom();
        let offset = token.loc.start;

        if let Some(info) = self.context_metadata_mut().last_binding() {
            debug_assert!(info.offset < offset);
        }

        // If the label is attached to a continue or break statement, its label info
        // is popped from the stack. See `continue_statement` and `break_statement` for more
        // information.
        self.context_metadata_mut().push_label(LabelInfo {
            name,
            offset,
            kind: LabelKind::Other,
        });

        context.check_label_identifier(token, &self.atoms().borrow())
    }

    fn check_labelled_statement(
        &mut self,
        name: SourceAtomSetIndex,
        label_offset: usize,
        statement_offset: usize,
    ) -> Result<'alloc, ()> {
        let label = self
            .context_metadata_mut()
            .find_label_at_offset(label_offset)
            .unwrap();

        let context = LabelledStatementEarlyErrorsContext::new(name, label.kind);
        let next_label_index = self
            .context_metadata_mut()
            .find_first_label(statement_offset);
        for info in self.context_metadata_mut().labels_from(next_label_index) {
            context.check_duplicate_label(info.name)?;
        }

        let break_or_continue_index = self
            .context_metadata_mut()
            .find_first_break_or_continue(label_offset);

        self.internal_check_unhandled_continue(context, break_or_continue_index)?;

        self.context_metadata_mut()
            .pop_labelled_breaks_and_continues_from_index(break_or_continue_index, name);
        Ok(())
    }

    // Static Semantics: Early Errors
    // https://tc39.es/ecma262/#sec-if-statement-static-semantics-early-errors
    // https://tc39.es/ecma262/#sec-semantics-static-semantics-early-errors
    // https://tc39.es/ecma262/#sec-with-statement-static-semantics-early-errors
    fn check_single_statement(&self, statement_start_offset: usize) -> Result<'alloc, ()> {
        // * It is a Syntax Error if IsLabelledFunction(Statement) is true.
        if self.is_labelled_function(statement_start_offset) {
            return Err(ParseError::LabelledFunctionDeclInSingleStatement.into());
        }
        Ok(())
    }

    // https://tc39.es/ecma262/#sec-islabelledfunction
    // Static Semantics: IsLabelledFunction ( stmt )
    //
    // Returns IsLabelledFunction of `stmt`.
    //
    // NOTE: For Syntax-only parsing (NYI), the stack value for Statement
    //       should contain this information.
    fn is_labelled_function(&self, statement_start_offset: usize) -> bool {
        // Step 1. If stmt is not a LabelledStatement , return false.
        if let Some(index) = self
            .context_metadata()
            .find_label_index_at_offset(statement_start_offset)
        {
            // Step 2. Let item be the LabelledItem of stmt.
            for label in self.context_metadata().labels_from(index) {
                match label.kind {
                    // Step 3. If item is LabelledItem : FunctionDeclaration,
                    // return true.
                    LabelKind::Function => {
                        return true;
                    }
                    // Step 4. Let subStmt be the Statement of item.
                    // Step 5. Return IsLabelledFunction(subStmt).
                    LabelKind::LabelledLabel => continue,
                    _ => break,
                }
            }
        }

        false
    }

    // Check bindings in Script. This is called after we have identified that
    // we are in a script. Any remaining bindings should be legal in this context.
    // Any labels within this context are only valid here, and can be popped.
    fn check_script_bindings(&mut self) -> Result<'alloc, ()> {
        let mut context = ScriptEarlyErrorsContext::new();
        let index = BindingsIndex { index: 0 };
        self.declare_script_or_function(&mut context, index)?;
        self.context_metadata_mut().pop_bindings_from(index);

        let label_index = LabelIndex { index: 0 };
        self.context_metadata_mut().pop_labels_from(label_index);

        self.check_unhandled_break_or_continue(context, 0)?;

        Ok(())
    }

    // Check bindings in Module. This is called after we have identified that
    // we are in a Module. Any remaining bindings should be legal in this context.
    // Any labels within this context are only valid here, and can be popped.
    fn check_module_bindings(&mut self) -> Result<'alloc, ()> {
        let mut context = ModuleEarlyErrorsContext::new();
        let index = BindingsIndex { index: 0 };
        self.declare_script_or_function(&mut context, index)?;
        self.context_metadata_mut().pop_bindings_from(index);

        let label_index = LabelIndex { index: 0 };
        self.context_metadata_mut().pop_labels_from(label_index);

        self.check_unhandled_break_or_continue(context, 0)?;

        Ok(())
    }

    /// Internal helper method:
    /// called From EarlyErrorChecker::check_labelled_statement, and not used by
    /// the struct implementing the trait. This means that
    /// LabelledStatementEarlyErrorsContext is allocated inside this trait.
    fn internal_check_unhandled_continue(
        &mut self,
        context: LabelledStatementEarlyErrorsContext,
        index: BreakOrContinueIndex,
    ) -> Result<'alloc, ()> {
        for info in self.context_metadata_mut().breaks_and_continues_from(index) {
            context.check_labelled_continue_to_non_loop(info)?;
        }

        Ok(())
    }

    /// Internal helper method:
    /// called From EarlyErrorChecker::check_script_bindings
    /// EarlyErrorChecker::check_module_bindings,
    /// EarlyErrorChecker::check_function_bindings, and not used by
    /// the struct implementing the trait. This means that
    /// the contexts associated with those methods are allocated inside this trait.
    fn check_unhandled_break_or_continue<T>(
        &mut self,
        context: T,
        offset: usize,
    ) -> Result<'alloc, ()>
    where
        T: ControlEarlyErrorsContext,
    {
        let index = self
            .context_metadata_mut()
            .find_first_break_or_continue(offset);
        if let Some(info) = self.context_metadata_mut().find_break_or_continue_at(index) {
            context.on_unhandled_break_or_continue(info)?;
        }

        Ok(())
    }

    /// Declare bindings in context_metadata to script-or-function-like context,
    /// where function declarations are body-level. This method is an internal
    /// helper for EarlyErrorChecker
    fn declare_script_or_function<T>(
        &mut self,
        context: &mut T,
        index: BindingsIndex,
    ) -> Result<'alloc, ()>
    where
        T: LexicalEarlyErrorsContext + VarEarlyErrorsContext,
    {
        for info in self.context_metadata().bindings_from(index) {
            match info.kind {
                BindingKind::Var => {
                    context.declare_var(
                        info.name,
                        DeclarationKind::Var,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Function | BindingKind::AsyncOrGenerator => {
                    context.declare_var(
                        info.name,
                        DeclarationKind::BodyLevelFunction,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Let => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::Let,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Const => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::Const,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Class => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::Class,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                _ => {
                    panic!("Unexpected binding found {:?}", info);
                }
            }
        }

        Ok(())
    }

    // Declare bindings to the parameter of function or catch.
    fn declare_param<T>(
        &self,
        context: &mut T,
        from: BindingsIndex,
        to: BindingsIndex,
    ) -> Result<'alloc, ()>
    where
        T: ParameterEarlyErrorsContext,
    {
        for info in self.context_metadata().bindings_from_to(from, to) {
            context.declare(info.name, info.offset, &self.atoms().borrow())?;
        }

        Ok(())
    }

    // Check bindings in function with FormalParameters.
    fn check_function_bindings(
        &mut self,
        is_simple: bool,
        start_of_param_offset: usize,
        end_of_param_offset: usize,
    ) -> Result<'alloc, ()> {
        let mut param_context = if is_simple {
            FormalParametersEarlyErrorsContext::new_simple()
        } else {
            FormalParametersEarlyErrorsContext::new_non_simple()
        };

        let param_index = self
            .context_metadata_mut()
            .find_first_binding(start_of_param_offset);
        let body_index = self
            .context_metadata_mut()
            .find_first_binding(end_of_param_offset);
        self.declare_param(&mut param_context, param_index, body_index)?;

        let mut body_context = FunctionBodyEarlyErrorsContext::new(param_context);
        self.declare_script_or_function(&mut body_context, body_index)?;

        self.check_unhandled_break_or_continue(body_context, end_of_param_offset)?;

        self.context_metadata_mut().pop_bindings_from(param_index);
        let label_index = self
            .context_metadata_mut()
            .find_first_label(start_of_param_offset);
        self.context_metadata_mut().pop_labels_from(label_index);

        Ok(())
    }

    // Check bindings in function with UniqueFormalParameters.
    fn check_unique_function_bindings(
        &mut self,
        start_of_param_offset: usize,
        end_of_param_offset: usize,
    ) -> Result<'alloc, ()> {
        let mut param_context = UniqueFormalParametersEarlyErrorsContext::new();

        let param_index = self
            .context_metadata_mut()
            .find_first_binding(start_of_param_offset);
        let body_index = self
            .context_metadata_mut()
            .find_first_binding(end_of_param_offset);
        self.declare_param(&mut param_context, param_index, body_index)?;

        let mut body_context = UniqueFunctionBodyEarlyErrorsContext::new(param_context);
        self.declare_script_or_function(&mut body_context, body_index)?;
        self.context_metadata_mut().pop_bindings_from(param_index);

        let label_index = self
            .context_metadata_mut()
            .find_first_label(start_of_param_offset);
        self.context_metadata_mut().pop_labels_from(label_index);

        self.check_unhandled_break_or_continue(body_context, end_of_param_offset)?;

        Ok(())
    }

    // Declare bindings to Block-like context, where function declarations
    // are lexical.
    fn declare_block<T>(&self, context: &mut T, index: BindingsIndex) -> Result<'alloc, ()>
    where
        T: LexicalEarlyErrorsContext + VarEarlyErrorsContext,
    {
        for info in self.context_metadata().bindings_from(index) {
            match info.kind {
                BindingKind::Var => {
                    context.declare_var(
                        info.name,
                        DeclarationKind::Var,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Function => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::LexicalFunction,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::AsyncOrGenerator => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::LexicalAsyncOrGenerator,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Let => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::Let,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Const => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::Const,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Class => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::Class,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                _ => {
                    panic!("Unexpected binding found {:?}", info);
                }
            }
        }

        Ok(())
    }

    // Check bindings in Block.
    fn check_block_bindings(&mut self, start_of_block_offset: usize) -> Result<'alloc, ()> {
        let mut context = BlockEarlyErrorsContext::new();
        let index = self
            .context_metadata_mut()
            .find_first_binding(start_of_block_offset);
        self.declare_block(&mut context, index)?;
        self.context_metadata_mut().pop_lexical_bindings_from(index);

        Ok(())
    }

    // Check bindings in CaseBlock of switch-statement.
    fn check_case_block_binding(&mut self, start_of_block_offset: usize) -> Result<'alloc, ()> {
        let mut context = CaseBlockEarlyErrorsContext::new();

        let index = self
            .context_metadata_mut()
            .find_first_binding(start_of_block_offset);
        // Check bindings in CaseBlock of switch-statement.
        self.declare_block(&mut context, index)?;
        self.context_metadata_mut().pop_lexical_bindings_from(index);

        self.context_metadata_mut()
            .pop_unlabelled_breaks_from(start_of_block_offset);

        Ok(())
    }

    // Check bindings in Catch and Block.
    fn check_catch_bindings(
        &mut self,
        is_simple: bool,
        start_loc: usize,
        end_loc: usize,
    ) -> Result<'alloc, ()> {
        let mut param_context = if is_simple {
            CatchParameterEarlyErrorsContext::new_with_binding_identifier()
        } else {
            CatchParameterEarlyErrorsContext::new_with_binding_pattern()
        };

        let param_index = self.context_metadata_mut().find_first_binding(start_loc);
        let body_index = self.context_metadata_mut().find_first_binding(end_loc);
        self.declare_param(&mut param_context, param_index, body_index)?;

        let mut block_context = CatchBlockEarlyErrorsContext::new(param_context);
        self.declare_block(&mut block_context, body_index)?;
        self.context_metadata_mut()
            .pop_lexical_bindings_from(param_index);

        Ok(())
    }

    // Check bindings in Catch with no parameter and Block.
    fn check_catch_no_param_bindings(&mut self, catch_offset: usize) -> Result<'alloc, ()> {
        let body_index = self.context_metadata_mut().find_first_binding(catch_offset);

        let param_context = CatchParameterEarlyErrorsContext::new_with_binding_identifier();
        let mut block_context = CatchBlockEarlyErrorsContext::new(param_context);
        self.declare_block(&mut block_context, body_index)?;
        self.context_metadata_mut()
            .pop_lexical_bindings_from(body_index);

        Ok(())
    }

    // Declare bindings to the head of lexical for-statement.
    fn declare_lexical_for_head(
        &self,
        context: &mut LexicalForHeadEarlyErrorsContext,
        from: BindingsIndex,
        to: BindingsIndex,
    ) -> Result<'alloc, ()> {
        for info in self.context_metadata().bindings_from_to(from, to) {
            match info.kind {
                BindingKind::Let => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::Let,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                BindingKind::Const => {
                    context.declare_lex(
                        info.name,
                        DeclarationKind::Const,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                _ => {
                    panic!("Unexpected binding found {:?}", info);
                }
            }
        }

        Ok(())
    }

    // Declare bindings to the body of lexical for-statement.
    fn declare_lexical_for_body(
        &self,
        context: &mut LexicalForBodyEarlyErrorsContext,
        index: BindingsIndex,
    ) -> Result<'alloc, ()> {
        for info in self.context_metadata().bindings_from(index) {
            match info.kind {
                BindingKind::Var => {
                    context.declare_var(
                        info.name,
                        DeclarationKind::Var,
                        info.offset,
                        &self.atoms().borrow(),
                    )?;
                }
                _ => {
                    panic!("Unexpected binding found {:?}", info);
                }
            }
        }

        Ok(())
    }

    // Check bindings in lexical for-statement.
    fn check_lexical_for_bindings(
        &mut self,
        start_loc: usize,
        end_loc: usize,
    ) -> Result<'alloc, ()> {
        let mut head_context = LexicalForHeadEarlyErrorsContext::new();

        let head_index = self.context_metadata_mut().find_first_binding(start_loc);
        let body_index = self.context_metadata_mut().find_first_binding(end_loc);
        self.declare_lexical_for_head(&mut head_context, head_index, body_index)?;

        let mut body_context = LexicalForBodyEarlyErrorsContext::new(head_context);
        self.declare_lexical_for_body(&mut body_context, body_index)?;
        self.context_metadata_mut()
            .pop_lexical_bindings_from(head_index);

        Ok(())
    }
}
