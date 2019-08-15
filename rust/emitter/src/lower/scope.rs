use super::pass::Pass;
use ast::*;
use std::mem::replace;

pub fn pass(ast: &mut Program) {
    ScopePass {
        top_scope: Scope {
            declarations: Vec::new(),
        },
    }
    .visit_program(ast);
}

struct Scope {
    declarations: Vec<String>,
}

struct ScopePass {
    top_scope: Scope,
}

impl Pass for ScopePass {
    fn visit_binding_identifier(&mut self, ast: &mut BindingIdentifier) {
        self.visit_identifier(&mut ast.name);
    }

    fn visit_block(&mut self, ast: &mut Block) {
        let old_scope = replace(
            &mut self.top_scope,
            Scope {
                declarations: Vec::new(),
            },
        );
        for item in &mut ast.statements {
            self.visit_statement(item);
        }
        let this_scope = replace(&mut self.top_scope, old_scope);
        ast.declarations = Some(this_scope.declarations);
    }
}
