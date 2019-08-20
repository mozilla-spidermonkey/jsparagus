pub mod full {
    use super::super::pass::Pass;
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
}

pub mod postfix {
    use super::super::pass::{PostfixPass, PostfixPassMonoid, PostfixPassVisitor};
    use ast::*;

    pub fn pass(ast: &mut Program) {
        PostfixPassVisitor::new(ScopePass {}).visit_program(ast);
    }

    #[derive(Default, Debug)]
    struct ScopeInfo {
        declarations: Vec<String>,
        references: Vec<String>,
    }

    impl PostfixPassMonoid for ScopeInfo {
        fn append(&mut self, mut other: Self) {
            self.declarations.append(&mut other.declarations);
            self.references.append(&mut other.references);
        }
    }

    struct ScopePass {}

    impl PostfixPass for ScopePass {
        type Value = ScopeInfo;

        fn visit_identifier(&self, name: &mut String) -> ScopeInfo {
            ScopeInfo {
                declarations: Vec::new(),
                references: vec![name.clone()],
            }
        }

        fn visit_variable_declarator(
            &self,
            mut binding: ScopeInfo,
            init: Option<ScopeInfo>,
        ) -> Self::Value {
            // TODO: This is a little gross. We assume that the eventual Identifier contained
            // within VariableDeclarator->BindingIdentifier->Identifier is the only binding, and no
            // references can occur (e.g. can destructuring exprs contain ident references?)
            let mut result = ScopeInfo {
                declarations: binding.references,
                references: Vec::new(),
            };
            result.declarations.append(&mut binding.declarations);
            if let Some(item) = init {
                result.append(item);
            }
            result
        }

        fn visit_block(
            &self,
            mut statements: Vec<Self::Value>,
            declarations: &mut Option<Vec<String>>,
        ) -> ScopeInfo {
            let mut declared_here = Vec::new();
            let mut free_vars = Vec::new();
            let statements = statements
                .drain(..)
                .fold(ScopeInfo::default(), |mut sum, item| {
                    sum.append(item);
                    sum
                });
            for reference in statements.references {
                if statements.declarations.contains(&reference) {
                    declared_here.push(reference);
                } else {
                    free_vars.push(reference);
                }
            }
            *declarations = Some(declared_here);
            ScopeInfo {
                declarations: Vec::new(),
                references: free_vars,
            }
        }
    }
}
