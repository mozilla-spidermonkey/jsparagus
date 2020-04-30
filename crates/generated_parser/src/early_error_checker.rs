use crate::context_stack::{BreakOrContinueIndex, ContextMetadata};
use crate::early_errors::*;
use crate::error::Result;
use ast::source_atom_set::SourceAtomSetIndex;

pub trait EarlyErrorChecker<'alloc> {
    fn context_metadata(&mut self) -> &mut ContextMetadata;
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
        &self,
        context_metadata: &mut ContextMetadata,
        context: T,
        offset: usize,
    ) -> Result<'alloc, ()>
    where
        T: ControlEarlyErrorsContext,
    {
        let index = context_metadata.find_first_break_or_continue(offset);
        if let Some(info) = context_metadata.find_break_or_continue_at(index) {
            context.on_unhandled_break_or_continue(info)?;
        }

        Ok(())
    }
}
