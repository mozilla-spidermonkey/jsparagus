use crate::context_stack::{BreakOrContinueIndex, ContextMetadata, LabelKind};
use crate::early_errors::*;
use crate::error::{ParseError, Result};
use ast::source_atom_set::SourceAtomSetIndex;

pub trait EarlyErrorChecker<'alloc> {
    fn context_metadata(&mut self) -> &mut ContextMetadata;
    fn context_metadata_immutable(&self) -> &ContextMetadata;
    fn check_labelled_statement(
        &mut self,
        name: SourceAtomSetIndex,
        start_label_offset: usize,
        end_label_offset: usize,
    ) -> Result<'alloc, ()> {
        let label = self
            .context_metadata()
            .find_label_at_offset(start_label_offset)
            .unwrap();

        let context = LabelledStatementEarlyErrorsContext::new(name, label.kind);
        let next_label_index = self.context_metadata().find_first_label(end_label_offset);
        for info in self.context_metadata().labels_from(next_label_index) {
            context.check_duplicate_label(info.name)?;
        }

        let break_or_continue_index = self
            .context_metadata()
            .find_first_break_or_continue(start_label_offset);

        self.check_unhandled_continue(context, break_or_continue_index)?;

        self.context_metadata()
            .pop_labelled_breaks_and_continues_from_index(break_or_continue_index, name);
        Ok(())
    }

    fn check_unhandled_continue(
        &mut self,
        context: LabelledStatementEarlyErrorsContext,
        index: BreakOrContinueIndex,
    ) -> Result<'alloc, ()> {
        for info in self.context_metadata().breaks_and_continues_from(index) {
            context.check_labelled_continue_to_non_loop(info)?;
        }

        Ok(())
    }

    fn check_unhandled_break_or_continue<T>(
        &mut self,
        context: T,
        offset: usize,
    ) -> Result<'alloc, ()>
    where
        T: ControlEarlyErrorsContext,
    {
        let index = self.context_metadata().find_first_break_or_continue(offset);
        if let Some(info) = self.context_metadata().find_break_or_continue_at(index) {
            context.on_unhandled_break_or_continue(info)?;
        }

        Ok(())
    }

    // Static Semantics: Early Errors
    // https://tc39.es/ecma262/#sec-if-statement-static-semantics-early-errors
    // https://tc39.es/ecma262/#sec-semantics-static-semantics-early-errors
    // https://tc39.es/ecma262/#sec-with-statement-static-semantics-early-errors
    fn check_single_statement(&self, offset: usize) -> Result<'alloc, ()> {
        // * It is a Syntax Error if IsLabelledFunction(Statement) is true.
        if self.is_labelled_function(offset) {
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
    fn is_labelled_function(&self, offset: usize) -> bool {
        // Step 1. If stmt is not a LabelledStatement , return false.
        if let Some(index) = self
            .context_metadata_immutable()
            .find_label_index_at_offset(offset)
        {
            // Step 2. Let item be the LabelledItem of stmt.
            for label in self.context_metadata_immutable().labels_from(index) {
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
}
