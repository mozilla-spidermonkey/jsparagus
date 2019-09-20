// WARNING: This file is auto-generated.

use crate::Token;
use ast::arena;
use ast::types::*;

#[derive(Debug)]
pub enum StackValue<'alloc> {
    Argument(arena::Box<'alloc, Argument<'alloc>>),
    Arguments(arena::Box<'alloc, Arguments<'alloc>>),
    ArrayAssignmentTarget(arena::Box<'alloc, ArrayAssignmentTarget<'alloc>>),
    ArrayBinding(arena::Box<'alloc, ArrayBinding<'alloc>>),
    ArrayExpression(arena::Box<'alloc, ArrayExpression<'alloc>>),
    ArrayExpressionElement(arena::Box<'alloc, ArrayExpressionElement<'alloc>>),
    ArrowExpression(arena::Box<'alloc, ArrowExpression<'alloc>>),
    ArrowExpressionBody(arena::Box<'alloc, ArrowExpressionBody<'alloc>>),
    AssignmentExpression(arena::Box<'alloc, AssignmentExpression<'alloc>>),
    AssignmentTarget(arena::Box<'alloc, AssignmentTarget<'alloc>>),
    AssignmentTargetIdentifier(arena::Box<'alloc, AssignmentTargetIdentifier<'alloc>>),
    AssignmentTargetMaybeDefault(arena::Box<'alloc, AssignmentTargetMaybeDefault<'alloc>>),
    AssignmentTargetPattern(arena::Box<'alloc, AssignmentTargetPattern<'alloc>>),
    AssignmentTargetProperty(arena::Box<'alloc, AssignmentTargetProperty<'alloc>>),
    AssignmentTargetPropertyIdentifier(
        arena::Box<'alloc, AssignmentTargetPropertyIdentifier<'alloc>>,
    ),
    AssignmentTargetPropertyProperty(arena::Box<'alloc, AssignmentTargetPropertyProperty<'alloc>>),
    AssignmentTargetWithDefault(arena::Box<'alloc, AssignmentTargetWithDefault<'alloc>>),
    AwaitExpression(arena::Box<'alloc, AwaitExpression<'alloc>>),
    BinaryExpression(arena::Box<'alloc, BinaryExpression<'alloc>>),
    BinaryOperator(arena::Box<'alloc, BinaryOperator>),
    Binding(arena::Box<'alloc, Binding<'alloc>>),
    BindingIdentifier(arena::Box<'alloc, BindingIdentifier<'alloc>>),
    BindingPattern(arena::Box<'alloc, BindingPattern<'alloc>>),
    BindingProperty(arena::Box<'alloc, BindingProperty<'alloc>>),
    BindingPropertyIdentifier(arena::Box<'alloc, BindingPropertyIdentifier<'alloc>>),
    BindingPropertyProperty(arena::Box<'alloc, BindingPropertyProperty<'alloc>>),
    BindingWithDefault(arena::Box<'alloc, BindingWithDefault<'alloc>>),
    Block(arena::Box<'alloc, Block<'alloc>>),
    BlockStatement(arena::Box<'alloc, BlockStatement<'alloc>>),
    BreakStatement(arena::Box<'alloc, BreakStatement<'alloc>>),
    CallExpression(arena::Box<'alloc, CallExpression<'alloc>>),
    CatchClause(arena::Box<'alloc, CatchClause<'alloc>>),
    ClassDeclaration(arena::Box<'alloc, ClassDeclaration<'alloc>>),
    ClassElement(arena::Box<'alloc, ClassElement<'alloc>>),
    ClassExpression(arena::Box<'alloc, ClassExpression<'alloc>>),
    CompoundAssignmentExpression(arena::Box<'alloc, CompoundAssignmentExpression<'alloc>>),
    CompoundAssignmentOperator(arena::Box<'alloc, CompoundAssignmentOperator>),
    ComputedMemberAssignmentTarget(arena::Box<'alloc, ComputedMemberAssignmentTarget<'alloc>>),
    ComputedMemberExpression(arena::Box<'alloc, ComputedMemberExpression<'alloc>>),
    ComputedPropertyName(arena::Box<'alloc, ComputedPropertyName<'alloc>>),
    ConditionalExpression(arena::Box<'alloc, ConditionalExpression<'alloc>>),
    ContinueStatement(arena::Box<'alloc, ContinueStatement<'alloc>>),
    CoverParenthesized(arena::Box<'alloc, CoverParenthesized<'alloc>>),
    DataProperty(arena::Box<'alloc, DataProperty<'alloc>>),
    Directive(arena::Box<'alloc, Directive<'alloc>>),
    DoWhileStatement(arena::Box<'alloc, DoWhileStatement<'alloc>>),
    Export(arena::Box<'alloc, Export<'alloc>>),
    ExportAllFrom(arena::Box<'alloc, ExportAllFrom<'alloc>>),
    ExportDeclaration(arena::Box<'alloc, ExportDeclaration<'alloc>>),
    ExportDefault(arena::Box<'alloc, ExportDefault<'alloc>>),
    ExportFrom(arena::Box<'alloc, ExportFrom<'alloc>>),
    ExportFromSpecifier(arena::Box<'alloc, ExportFromSpecifier<'alloc>>),
    ExportLocalSpecifier(arena::Box<'alloc, ExportLocalSpecifier<'alloc>>),
    ExportLocals(arena::Box<'alloc, ExportLocals<'alloc>>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    ExpressionOrSuper(arena::Box<'alloc, ExpressionOrSuper<'alloc>>),
    ForInStatement(arena::Box<'alloc, ForInStatement<'alloc>>),
    ForOfStatement(arena::Box<'alloc, ForOfStatement<'alloc>>),
    ForStatement(arena::Box<'alloc, ForStatement<'alloc>>),
    FormalParameters(arena::Box<'alloc, FormalParameters<'alloc>>),
    Function(arena::Box<'alloc, Function<'alloc>>),
    FunctionBody(arena::Box<'alloc, FunctionBody<'alloc>>),
    Getter(arena::Box<'alloc, Getter<'alloc>>),
    Identifier(arena::Box<'alloc, Identifier<'alloc>>),
    IdentifierExpression(arena::Box<'alloc, IdentifierExpression<'alloc>>),
    IdentifierName(arena::Box<'alloc, IdentifierName<'alloc>>),
    IfStatement(arena::Box<'alloc, IfStatement<'alloc>>),
    Import(arena::Box<'alloc, Import<'alloc>>),
    ImportDeclaration(arena::Box<'alloc, ImportDeclaration<'alloc>>),
    ImportNamespace(arena::Box<'alloc, ImportNamespace<'alloc>>),
    ImportSpecifier(arena::Box<'alloc, ImportSpecifier<'alloc>>),
    Label(arena::Box<'alloc, Label<'alloc>>),
    LabeledStatement(arena::Box<'alloc, LabeledStatement<'alloc>>),
    LiteralBooleanExpression(arena::Box<'alloc, LiteralBooleanExpression>),
    LiteralNumericExpression(arena::Box<'alloc, LiteralNumericExpression>),
    LiteralRegExpExpression(arena::Box<'alloc, LiteralRegExpExpression<'alloc>>),
    LiteralStringExpression(arena::Box<'alloc, LiteralStringExpression<'alloc>>),
    MemberAssignmentTarget(arena::Box<'alloc, MemberAssignmentTarget<'alloc>>),
    MemberExpression(arena::Box<'alloc, MemberExpression<'alloc>>),
    Method(arena::Box<'alloc, Method<'alloc>>),
    MethodDefinition(arena::Box<'alloc, MethodDefinition<'alloc>>),
    Module(arena::Box<'alloc, Module<'alloc>>),
    ModuleItems(arena::Box<'alloc, ModuleItems<'alloc>>),
    NamedObjectProperty(arena::Box<'alloc, NamedObjectProperty<'alloc>>),
    NewExpression(arena::Box<'alloc, NewExpression<'alloc>>),
    ObjectAssignmentTarget(arena::Box<'alloc, ObjectAssignmentTarget<'alloc>>),
    ObjectBinding(arena::Box<'alloc, ObjectBinding<'alloc>>),
    ObjectExpression(arena::Box<'alloc, ObjectExpression<'alloc>>),
    ObjectProperty(arena::Box<'alloc, ObjectProperty<'alloc>>),
    Parameter(arena::Box<'alloc, Parameter<'alloc>>),
    Program(arena::Box<'alloc, Program<'alloc>>),
    PropertyName(arena::Box<'alloc, PropertyName<'alloc>>),
    ReturnStatement(arena::Box<'alloc, ReturnStatement<'alloc>>),
    Script(arena::Box<'alloc, Script<'alloc>>),
    Setter(arena::Box<'alloc, Setter<'alloc>>),
    ShorthandProperty(arena::Box<'alloc, ShorthandProperty<'alloc>>),
    SimpleAssignmentTarget(arena::Box<'alloc, SimpleAssignmentTarget<'alloc>>),
    Statement(arena::Box<'alloc, Statement<'alloc>>),
    StaticMemberAssignmentTarget(arena::Box<'alloc, StaticMemberAssignmentTarget<'alloc>>),
    StaticMemberExpression(arena::Box<'alloc, StaticMemberExpression<'alloc>>),
    StaticPropertyName(arena::Box<'alloc, StaticPropertyName<'alloc>>),
    SwitchCase(arena::Box<'alloc, SwitchCase<'alloc>>),
    SwitchDefault(arena::Box<'alloc, SwitchDefault<'alloc>>),
    SwitchStatement(arena::Box<'alloc, SwitchStatement<'alloc>>),
    SwitchStatementWithDefault(arena::Box<'alloc, SwitchStatementWithDefault<'alloc>>),
    TemplateElement(arena::Box<'alloc, TemplateElement<'alloc>>),
    TemplateExpression(arena::Box<'alloc, TemplateExpression<'alloc>>),
    TemplateExpressionElement(arena::Box<'alloc, TemplateExpressionElement<'alloc>>),
    ThrowStatement(arena::Box<'alloc, ThrowStatement<'alloc>>),
    Token(arena::Box<'alloc, Token<'alloc>>),
    TryCatchStatement(arena::Box<'alloc, TryCatchStatement<'alloc>>),
    TryFinallyStatement(arena::Box<'alloc, TryFinallyStatement<'alloc>>),
    UnaryExpression(arena::Box<'alloc, UnaryExpression<'alloc>>),
    UnaryOperator(arena::Box<'alloc, UnaryOperator>),
    UpdateExpression(arena::Box<'alloc, UpdateExpression<'alloc>>),
    UpdateOperator(arena::Box<'alloc, UpdateOperator>),
    VariableDeclaration(arena::Box<'alloc, VariableDeclaration<'alloc>>),
    VariableDeclarationKind(arena::Box<'alloc, VariableDeclarationKind>),
    VariableDeclarationOrAssignmentTarget(
        arena::Box<'alloc, VariableDeclarationOrAssignmentTarget<'alloc>>,
    ),
    VariableDeclarationOrExpression(arena::Box<'alloc, VariableDeclarationOrExpression<'alloc>>),
    VariableDeclarator(arena::Box<'alloc, VariableDeclarator<'alloc>>),
    VariableReference(arena::Box<'alloc, VariableReference<'alloc>>),
    VecArrayExpressionElement(
        arena::Box<'alloc, arena::Vec<'alloc, ArrayExpressionElement<'alloc>>>,
    ),
    VecBindingProperty(arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>>),
    VecClassElement(arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>>),
    VecOption(arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>>),
    VecStatement(arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>),
    VecSwitchCase(arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>>),
    VecVariableDeclarator(arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>),
    Void(arena::Box<'alloc, Void>),
    WhileStatement(arena::Box<'alloc, WhileStatement<'alloc>>),
    WithStatement(arena::Box<'alloc, WithStatement<'alloc>>),
    YieldExpression(arena::Box<'alloc, YieldExpression<'alloc>>),
    YieldGeneratorExpression(arena::Box<'alloc, YieldGeneratorExpression<'alloc>>),
}

impl<'alloc> StackValue<'alloc> {
    pub fn to_ast<T: StackValueItem<'alloc>>(self) -> arena::Box<'alloc, T> {
        T::to_ast(self)
    }
}

pub trait StackValueItem<'alloc>: Sized {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self>;
}

impl<'alloc> StackValueItem<'alloc> for Argument<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Argument(v) => v,
            _ => panic!("StackValue expected Argument, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Arguments<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Arguments(v) => v,
            _ => panic!("StackValue expected Arguments, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ArrayAssignmentTarget<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ArrayAssignmentTarget(v) => v,
            _ => panic!("StackValue expected ArrayAssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ArrayBinding<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ArrayBinding(v) => v,
            _ => panic!("StackValue expected ArrayBinding, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ArrayExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ArrayExpression(v) => v,
            _ => panic!("StackValue expected ArrayExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ArrayExpressionElement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ArrayExpressionElement(v) => v,
            _ => panic!("StackValue expected ArrayExpressionElement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ArrowExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ArrowExpression(v) => v,
            _ => panic!("StackValue expected ArrowExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ArrowExpressionBody<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ArrowExpressionBody(v) => v,
            _ => panic!("StackValue expected ArrowExpressionBody, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentExpression(v) => v,
            _ => panic!("StackValue expected AssignmentExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentTarget<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentTarget(v) => v,
            _ => panic!("StackValue expected AssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentTargetIdentifier<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentTargetIdentifier(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetIdentifier, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentTargetMaybeDefault<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentTargetMaybeDefault(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetMaybeDefault, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentTargetPattern<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentTargetPattern(v) => v,
            _ => panic!("StackValue expected AssignmentTargetPattern, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentTargetProperty<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentTargetProperty(v) => v,
            _ => panic!("StackValue expected AssignmentTargetProperty, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentTargetPropertyIdentifier<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentTargetPropertyIdentifier(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetPropertyIdentifier, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentTargetPropertyProperty<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentTargetPropertyProperty(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetPropertyProperty, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AssignmentTargetWithDefault<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AssignmentTargetWithDefault(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetWithDefault, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for AwaitExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::AwaitExpression(v) => v,
            _ => panic!("StackValue expected AwaitExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BinaryExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BinaryExpression(v) => v,
            _ => panic!("StackValue expected BinaryExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BinaryOperator {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BinaryOperator(v) => v,
            _ => panic!("StackValue expected BinaryOperator, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Binding<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Binding(v) => v,
            _ => panic!("StackValue expected Binding, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BindingIdentifier<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BindingIdentifier(v) => v,
            _ => panic!("StackValue expected BindingIdentifier, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BindingPattern<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BindingPattern(v) => v,
            _ => panic!("StackValue expected BindingPattern, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BindingProperty<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BindingProperty(v) => v,
            _ => panic!("StackValue expected BindingProperty, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BindingPropertyIdentifier<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BindingPropertyIdentifier(v) => v,
            _ => panic!(
                "StackValue expected BindingPropertyIdentifier, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BindingPropertyProperty<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BindingPropertyProperty(v) => v,
            _ => panic!("StackValue expected BindingPropertyProperty, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BindingWithDefault<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BindingWithDefault(v) => v,
            _ => panic!("StackValue expected BindingWithDefault, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Block<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Block(v) => v,
            _ => panic!("StackValue expected Block, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BlockStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BlockStatement(v) => v,
            _ => panic!("StackValue expected BlockStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for BreakStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::BreakStatement(v) => v,
            _ => panic!("StackValue expected BreakStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for CallExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::CallExpression(v) => v,
            _ => panic!("StackValue expected CallExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for CatchClause<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::CatchClause(v) => v,
            _ => panic!("StackValue expected CatchClause, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ClassDeclaration<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ClassDeclaration(v) => v,
            _ => panic!("StackValue expected ClassDeclaration, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ClassElement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ClassElement(v) => v,
            _ => panic!("StackValue expected ClassElement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ClassExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ClassExpression(v) => v,
            _ => panic!("StackValue expected ClassExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for CompoundAssignmentExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::CompoundAssignmentExpression(v) => v,
            _ => panic!(
                "StackValue expected CompoundAssignmentExpression, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for CompoundAssignmentOperator {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::CompoundAssignmentOperator(v) => v,
            _ => panic!(
                "StackValue expected CompoundAssignmentOperator, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ComputedMemberAssignmentTarget<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ComputedMemberAssignmentTarget(v) => v,
            _ => panic!(
                "StackValue expected ComputedMemberAssignmentTarget, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ComputedMemberExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ComputedMemberExpression(v) => v,
            _ => panic!("StackValue expected ComputedMemberExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ComputedPropertyName<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ComputedPropertyName(v) => v,
            _ => panic!("StackValue expected ComputedPropertyName, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ConditionalExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ConditionalExpression(v) => v,
            _ => panic!("StackValue expected ConditionalExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ContinueStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ContinueStatement(v) => v,
            _ => panic!("StackValue expected ContinueStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for CoverParenthesized<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::CoverParenthesized(v) => v,
            _ => panic!("StackValue expected CoverParenthesized, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for DataProperty<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::DataProperty(v) => v,
            _ => panic!("StackValue expected DataProperty, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Directive<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Directive(v) => v,
            _ => panic!("StackValue expected Directive, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for DoWhileStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::DoWhileStatement(v) => v,
            _ => panic!("StackValue expected DoWhileStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Export<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Export(v) => v,
            _ => panic!("StackValue expected Export, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ExportAllFrom<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ExportAllFrom(v) => v,
            _ => panic!("StackValue expected ExportAllFrom, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ExportDeclaration<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ExportDeclaration(v) => v,
            _ => panic!("StackValue expected ExportDeclaration, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ExportDefault<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ExportDefault(v) => v,
            _ => panic!("StackValue expected ExportDefault, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ExportFrom<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ExportFrom(v) => v,
            _ => panic!("StackValue expected ExportFrom, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ExportFromSpecifier<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ExportFromSpecifier(v) => v,
            _ => panic!("StackValue expected ExportFromSpecifier, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ExportLocalSpecifier<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ExportLocalSpecifier(v) => v,
            _ => panic!("StackValue expected ExportLocalSpecifier, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ExportLocals<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ExportLocals(v) => v,
            _ => panic!("StackValue expected ExportLocals, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Expression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Expression(v) => v,
            _ => panic!("StackValue expected Expression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ExpressionOrSuper<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ExpressionOrSuper(v) => v,
            _ => panic!("StackValue expected ExpressionOrSuper, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ForInStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ForInStatement(v) => v,
            _ => panic!("StackValue expected ForInStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ForOfStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ForOfStatement(v) => v,
            _ => panic!("StackValue expected ForOfStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ForStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ForStatement(v) => v,
            _ => panic!("StackValue expected ForStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for FormalParameters<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::FormalParameters(v) => v,
            _ => panic!("StackValue expected FormalParameters, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Function<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Function(v) => v,
            _ => panic!("StackValue expected Function, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for FunctionBody<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::FunctionBody(v) => v,
            _ => panic!("StackValue expected FunctionBody, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Getter<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Getter(v) => v,
            _ => panic!("StackValue expected Getter, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Identifier<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Identifier(v) => v,
            _ => panic!("StackValue expected Identifier, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for IdentifierExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::IdentifierExpression(v) => v,
            _ => panic!("StackValue expected IdentifierExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for IdentifierName<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::IdentifierName(v) => v,
            _ => panic!("StackValue expected IdentifierName, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for IfStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::IfStatement(v) => v,
            _ => panic!("StackValue expected IfStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Import<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Import(v) => v,
            _ => panic!("StackValue expected Import, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ImportDeclaration<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ImportDeclaration(v) => v,
            _ => panic!("StackValue expected ImportDeclaration, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ImportNamespace<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ImportNamespace(v) => v,
            _ => panic!("StackValue expected ImportNamespace, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ImportSpecifier<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ImportSpecifier(v) => v,
            _ => panic!("StackValue expected ImportSpecifier, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Label<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Label(v) => v,
            _ => panic!("StackValue expected Label, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for LabeledStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::LabeledStatement(v) => v,
            _ => panic!("StackValue expected LabeledStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for LiteralBooleanExpression {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::LiteralBooleanExpression(v) => v,
            _ => panic!("StackValue expected LiteralBooleanExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for LiteralNumericExpression {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::LiteralNumericExpression(v) => v,
            _ => panic!("StackValue expected LiteralNumericExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for LiteralRegExpExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::LiteralRegExpExpression(v) => v,
            _ => panic!("StackValue expected LiteralRegExpExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for LiteralStringExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::LiteralStringExpression(v) => v,
            _ => panic!("StackValue expected LiteralStringExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for MemberAssignmentTarget<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::MemberAssignmentTarget(v) => v,
            _ => panic!("StackValue expected MemberAssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for MemberExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::MemberExpression(v) => v,
            _ => panic!("StackValue expected MemberExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Method<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Method(v) => v,
            _ => panic!("StackValue expected Method, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for MethodDefinition<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::MethodDefinition(v) => v,
            _ => panic!("StackValue expected MethodDefinition, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Module<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Module(v) => v,
            _ => panic!("StackValue expected Module, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ModuleItems<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ModuleItems(v) => v,
            _ => panic!("StackValue expected ModuleItems, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for NamedObjectProperty<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::NamedObjectProperty(v) => v,
            _ => panic!("StackValue expected NamedObjectProperty, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for NewExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::NewExpression(v) => v,
            _ => panic!("StackValue expected NewExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ObjectAssignmentTarget<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ObjectAssignmentTarget(v) => v,
            _ => panic!("StackValue expected ObjectAssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ObjectBinding<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ObjectBinding(v) => v,
            _ => panic!("StackValue expected ObjectBinding, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ObjectExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ObjectExpression(v) => v,
            _ => panic!("StackValue expected ObjectExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ObjectProperty<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ObjectProperty(v) => v,
            _ => panic!("StackValue expected ObjectProperty, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Parameter<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Parameter(v) => v,
            _ => panic!("StackValue expected Parameter, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Program<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Program(v) => v,
            _ => panic!("StackValue expected Program, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for PropertyName<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::PropertyName(v) => v,
            _ => panic!("StackValue expected PropertyName, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ReturnStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ReturnStatement(v) => v,
            _ => panic!("StackValue expected ReturnStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Script<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Script(v) => v,
            _ => panic!("StackValue expected Script, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Setter<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Setter(v) => v,
            _ => panic!("StackValue expected Setter, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ShorthandProperty<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ShorthandProperty(v) => v,
            _ => panic!("StackValue expected ShorthandProperty, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for SimpleAssignmentTarget<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::SimpleAssignmentTarget(v) => v,
            _ => panic!("StackValue expected SimpleAssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Statement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Statement(v) => v,
            _ => panic!("StackValue expected Statement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for StaticMemberAssignmentTarget<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::StaticMemberAssignmentTarget(v) => v,
            _ => panic!(
                "StackValue expected StaticMemberAssignmentTarget, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for StaticMemberExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::StaticMemberExpression(v) => v,
            _ => panic!("StackValue expected StaticMemberExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for StaticPropertyName<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::StaticPropertyName(v) => v,
            _ => panic!("StackValue expected StaticPropertyName, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for SwitchCase<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::SwitchCase(v) => v,
            _ => panic!("StackValue expected SwitchCase, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for SwitchDefault<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::SwitchDefault(v) => v,
            _ => panic!("StackValue expected SwitchDefault, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for SwitchStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::SwitchStatement(v) => v,
            _ => panic!("StackValue expected SwitchStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for SwitchStatementWithDefault<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::SwitchStatementWithDefault(v) => v,
            _ => panic!(
                "StackValue expected SwitchStatementWithDefault, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for TemplateElement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::TemplateElement(v) => v,
            _ => panic!("StackValue expected TemplateElement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for TemplateExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::TemplateExpression(v) => v,
            _ => panic!("StackValue expected TemplateExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for TemplateExpressionElement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::TemplateExpressionElement(v) => v,
            _ => panic!(
                "StackValue expected TemplateExpressionElement, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for ThrowStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ThrowStatement(v) => v,
            _ => panic!("StackValue expected ThrowStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Token<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Token(v) => v,
            _ => panic!("StackValue expected Token, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for TryCatchStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::TryCatchStatement(v) => v,
            _ => panic!("StackValue expected TryCatchStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for TryFinallyStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::TryFinallyStatement(v) => v,
            _ => panic!("StackValue expected TryFinallyStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for UnaryExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::UnaryExpression(v) => v,
            _ => panic!("StackValue expected UnaryExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for UnaryOperator {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::UnaryOperator(v) => v,
            _ => panic!("StackValue expected UnaryOperator, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for UpdateExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::UpdateExpression(v) => v,
            _ => panic!("StackValue expected UpdateExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for UpdateOperator {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::UpdateOperator(v) => v,
            _ => panic!("StackValue expected UpdateOperator, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for VariableDeclaration<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VariableDeclaration(v) => v,
            _ => panic!("StackValue expected VariableDeclaration, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for VariableDeclarationKind {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VariableDeclarationKind(v) => v,
            _ => panic!("StackValue expected VariableDeclarationKind, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for VariableDeclarationOrAssignmentTarget<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VariableDeclarationOrAssignmentTarget(v) => v,
            _ => panic!(
                "StackValue expected VariableDeclarationOrAssignmentTarget, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for VariableDeclarationOrExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VariableDeclarationOrExpression(v) => v,
            _ => panic!(
                "StackValue expected VariableDeclarationOrExpression, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for VariableDeclarator<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VariableDeclarator(v) => v,
            _ => panic!("StackValue expected VariableDeclarator, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for VariableReference<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VariableReference(v) => v,
            _ => panic!("StackValue expected VariableReference, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for arena::Vec<'alloc, ArrayExpressionElement<'alloc>> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VecArrayExpressionElement(v) => v,
            _ => panic!(
                "StackValue expected Vec<ArrayExpressionElement>, got {:?}",
                sv
            ),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for arena::Vec<'alloc, BindingProperty<'alloc>> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VecBindingProperty(v) => v,
            _ => panic!("StackValue expected Vec<BindingProperty>, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for arena::Vec<'alloc, ClassElement<'alloc>> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VecClassElement(v) => v,
            _ => panic!("StackValue expected Vec<ClassElement>, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for arena::Vec<'alloc, Option<Parameter<'alloc>>> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VecOption(v) => v,
            _ => panic!("StackValue expected Vec<Option<Parameter>>, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for arena::Vec<'alloc, Statement<'alloc>> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VecStatement(v) => v,
            _ => panic!("StackValue expected Vec<Statement>, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for arena::Vec<'alloc, SwitchCase<'alloc>> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VecSwitchCase(v) => v,
            _ => panic!("StackValue expected Vec<SwitchCase>, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for arena::Vec<'alloc, VariableDeclarator<'alloc>> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::VecVariableDeclarator(v) => v,
            _ => panic!("StackValue expected Vec<VariableDeclarator>, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for Void {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::Void(v) => v,
            _ => panic!("StackValue expected Void, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for WhileStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::WhileStatement(v) => v,
            _ => panic!("StackValue expected WhileStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for WithStatement<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::WithStatement(v) => v,
            _ => panic!("StackValue expected WithStatement, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for YieldExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::YieldExpression(v) => v,
            _ => panic!("StackValue expected YieldExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> StackValueItem<'alloc> for YieldGeneratorExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::YieldGeneratorExpression(v) => v,
            _ => panic!("StackValue expected YieldGeneratorExpression, got {:?}", sv),
        }
    }
}

impl<'alloc> From<arena::Box<'alloc, Argument<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Argument<'alloc>>) -> StackValue<'alloc> {
        StackValue::Argument(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Arguments<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Arguments<'alloc>>) -> StackValue<'alloc> {
        StackValue::Arguments(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ArrayAssignmentTarget<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ArrayAssignmentTarget<'alloc>>) -> StackValue<'alloc> {
        StackValue::ArrayAssignmentTarget(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ArrayBinding<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ArrayBinding<'alloc>>) -> StackValue<'alloc> {
        StackValue::ArrayBinding(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ArrayExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ArrayExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::ArrayExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ArrayExpressionElement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ArrayExpressionElement<'alloc>>) -> StackValue<'alloc> {
        StackValue::ArrayExpressionElement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ArrowExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ArrowExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::ArrowExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ArrowExpressionBody<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ArrowExpressionBody<'alloc>>) -> StackValue<'alloc> {
        StackValue::ArrowExpressionBody(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, AssignmentExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::AssignmentExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentTarget<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, AssignmentTarget<'alloc>>) -> StackValue<'alloc> {
        StackValue::AssignmentTarget(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentTargetIdentifier<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, AssignmentTargetIdentifier<'alloc>>) -> StackValue<'alloc> {
        StackValue::AssignmentTargetIdentifier(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentTargetMaybeDefault<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, AssignmentTargetMaybeDefault<'alloc>>) -> StackValue<'alloc> {
        StackValue::AssignmentTargetMaybeDefault(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentTargetPattern<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, AssignmentTargetPattern<'alloc>>) -> StackValue<'alloc> {
        StackValue::AssignmentTargetPattern(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentTargetProperty<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, AssignmentTargetProperty<'alloc>>) -> StackValue<'alloc> {
        StackValue::AssignmentTargetProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentTargetPropertyIdentifier<'alloc>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, AssignmentTargetPropertyIdentifier<'alloc>>,
    ) -> StackValue<'alloc> {
        StackValue::AssignmentTargetPropertyIdentifier(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentTargetPropertyProperty<'alloc>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, AssignmentTargetPropertyProperty<'alloc>>,
    ) -> StackValue<'alloc> {
        StackValue::AssignmentTargetPropertyProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AssignmentTargetWithDefault<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, AssignmentTargetWithDefault<'alloc>>) -> StackValue<'alloc> {
        StackValue::AssignmentTargetWithDefault(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, AwaitExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, AwaitExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::AwaitExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BinaryExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BinaryExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::BinaryExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BinaryOperator>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BinaryOperator>) -> StackValue<'alloc> {
        StackValue::BinaryOperator(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Binding<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Binding<'alloc>>) -> StackValue<'alloc> {
        StackValue::Binding(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BindingIdentifier<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BindingIdentifier<'alloc>>) -> StackValue<'alloc> {
        StackValue::BindingIdentifier(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BindingPattern<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BindingPattern<'alloc>>) -> StackValue<'alloc> {
        StackValue::BindingPattern(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BindingProperty<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BindingProperty<'alloc>>) -> StackValue<'alloc> {
        StackValue::BindingProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BindingPropertyIdentifier<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BindingPropertyIdentifier<'alloc>>) -> StackValue<'alloc> {
        StackValue::BindingPropertyIdentifier(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BindingPropertyProperty<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BindingPropertyProperty<'alloc>>) -> StackValue<'alloc> {
        StackValue::BindingPropertyProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BindingWithDefault<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BindingWithDefault<'alloc>>) -> StackValue<'alloc> {
        StackValue::BindingWithDefault(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Block<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Block<'alloc>>) -> StackValue<'alloc> {
        StackValue::Block(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BlockStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BlockStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::BlockStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, BreakStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, BreakStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::BreakStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, CallExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, CallExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::CallExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, CatchClause<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, CatchClause<'alloc>>) -> StackValue<'alloc> {
        StackValue::CatchClause(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ClassDeclaration<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ClassDeclaration<'alloc>>) -> StackValue<'alloc> {
        StackValue::ClassDeclaration(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ClassElement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ClassElement<'alloc>>) -> StackValue<'alloc> {
        StackValue::ClassElement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ClassExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ClassExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::ClassExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, CompoundAssignmentExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, CompoundAssignmentExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::CompoundAssignmentExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, CompoundAssignmentOperator>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, CompoundAssignmentOperator>) -> StackValue<'alloc> {
        StackValue::CompoundAssignmentOperator(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ComputedMemberAssignmentTarget<'alloc>>>
    for StackValue<'alloc>
{
    fn from(val: arena::Box<'alloc, ComputedMemberAssignmentTarget<'alloc>>) -> StackValue<'alloc> {
        StackValue::ComputedMemberAssignmentTarget(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ComputedMemberExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ComputedMemberExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::ComputedMemberExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ComputedPropertyName<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ComputedPropertyName<'alloc>>) -> StackValue<'alloc> {
        StackValue::ComputedPropertyName(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ConditionalExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ConditionalExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::ConditionalExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ContinueStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ContinueStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::ContinueStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, CoverParenthesized<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, CoverParenthesized<'alloc>>) -> StackValue<'alloc> {
        StackValue::CoverParenthesized(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, DataProperty<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, DataProperty<'alloc>>) -> StackValue<'alloc> {
        StackValue::DataProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Directive<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Directive<'alloc>>) -> StackValue<'alloc> {
        StackValue::Directive(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, DoWhileStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, DoWhileStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::DoWhileStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Export<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Export<'alloc>>) -> StackValue<'alloc> {
        StackValue::Export(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ExportAllFrom<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ExportAllFrom<'alloc>>) -> StackValue<'alloc> {
        StackValue::ExportAllFrom(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ExportDeclaration<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ExportDeclaration<'alloc>>) -> StackValue<'alloc> {
        StackValue::ExportDeclaration(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ExportDefault<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ExportDefault<'alloc>>) -> StackValue<'alloc> {
        StackValue::ExportDefault(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ExportFrom<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ExportFrom<'alloc>>) -> StackValue<'alloc> {
        StackValue::ExportFrom(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ExportFromSpecifier<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ExportFromSpecifier<'alloc>>) -> StackValue<'alloc> {
        StackValue::ExportFromSpecifier(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ExportLocalSpecifier<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ExportLocalSpecifier<'alloc>>) -> StackValue<'alloc> {
        StackValue::ExportLocalSpecifier(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ExportLocals<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ExportLocals<'alloc>>) -> StackValue<'alloc> {
        StackValue::ExportLocals(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Expression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Expression<'alloc>>) -> StackValue<'alloc> {
        StackValue::Expression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ExpressionOrSuper<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ExpressionOrSuper<'alloc>>) -> StackValue<'alloc> {
        StackValue::ExpressionOrSuper(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ForInStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ForInStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::ForInStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ForOfStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ForOfStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::ForOfStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ForStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ForStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::ForStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, FormalParameters<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, FormalParameters<'alloc>>) -> StackValue<'alloc> {
        StackValue::FormalParameters(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Function<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Function<'alloc>>) -> StackValue<'alloc> {
        StackValue::Function(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, FunctionBody<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, FunctionBody<'alloc>>) -> StackValue<'alloc> {
        StackValue::FunctionBody(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Getter<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Getter<'alloc>>) -> StackValue<'alloc> {
        StackValue::Getter(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Identifier<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Identifier<'alloc>>) -> StackValue<'alloc> {
        StackValue::Identifier(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, IdentifierExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, IdentifierExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::IdentifierExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, IdentifierName<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, IdentifierName<'alloc>>) -> StackValue<'alloc> {
        StackValue::IdentifierName(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, IfStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, IfStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::IfStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Import<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Import<'alloc>>) -> StackValue<'alloc> {
        StackValue::Import(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ImportDeclaration<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ImportDeclaration<'alloc>>) -> StackValue<'alloc> {
        StackValue::ImportDeclaration(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ImportNamespace<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ImportNamespace<'alloc>>) -> StackValue<'alloc> {
        StackValue::ImportNamespace(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ImportSpecifier<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ImportSpecifier<'alloc>>) -> StackValue<'alloc> {
        StackValue::ImportSpecifier(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Label<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Label<'alloc>>) -> StackValue<'alloc> {
        StackValue::Label(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, LabeledStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, LabeledStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::LabeledStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, LiteralBooleanExpression>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, LiteralBooleanExpression>) -> StackValue<'alloc> {
        StackValue::LiteralBooleanExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, LiteralNumericExpression>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, LiteralNumericExpression>) -> StackValue<'alloc> {
        StackValue::LiteralNumericExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, LiteralRegExpExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, LiteralRegExpExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::LiteralRegExpExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, LiteralStringExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, LiteralStringExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::LiteralStringExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, MemberAssignmentTarget<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, MemberAssignmentTarget<'alloc>>) -> StackValue<'alloc> {
        StackValue::MemberAssignmentTarget(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, MemberExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, MemberExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::MemberExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Method<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Method<'alloc>>) -> StackValue<'alloc> {
        StackValue::Method(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, MethodDefinition<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, MethodDefinition<'alloc>>) -> StackValue<'alloc> {
        StackValue::MethodDefinition(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Module<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Module<'alloc>>) -> StackValue<'alloc> {
        StackValue::Module(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ModuleItems<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ModuleItems<'alloc>>) -> StackValue<'alloc> {
        StackValue::ModuleItems(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, NamedObjectProperty<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, NamedObjectProperty<'alloc>>) -> StackValue<'alloc> {
        StackValue::NamedObjectProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, NewExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, NewExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::NewExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ObjectAssignmentTarget<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ObjectAssignmentTarget<'alloc>>) -> StackValue<'alloc> {
        StackValue::ObjectAssignmentTarget(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ObjectBinding<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ObjectBinding<'alloc>>) -> StackValue<'alloc> {
        StackValue::ObjectBinding(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ObjectExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ObjectExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::ObjectExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ObjectProperty<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ObjectProperty<'alloc>>) -> StackValue<'alloc> {
        StackValue::ObjectProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Parameter<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Parameter<'alloc>>) -> StackValue<'alloc> {
        StackValue::Parameter(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Program<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Program<'alloc>>) -> StackValue<'alloc> {
        StackValue::Program(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, PropertyName<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, PropertyName<'alloc>>) -> StackValue<'alloc> {
        StackValue::PropertyName(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ReturnStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ReturnStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::ReturnStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Script<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Script<'alloc>>) -> StackValue<'alloc> {
        StackValue::Script(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Setter<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Setter<'alloc>>) -> StackValue<'alloc> {
        StackValue::Setter(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ShorthandProperty<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ShorthandProperty<'alloc>>) -> StackValue<'alloc> {
        StackValue::ShorthandProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, SimpleAssignmentTarget<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, SimpleAssignmentTarget<'alloc>>) -> StackValue<'alloc> {
        StackValue::SimpleAssignmentTarget(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Statement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Statement<'alloc>>) -> StackValue<'alloc> {
        StackValue::Statement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, StaticMemberAssignmentTarget<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, StaticMemberAssignmentTarget<'alloc>>) -> StackValue<'alloc> {
        StackValue::StaticMemberAssignmentTarget(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, StaticMemberExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, StaticMemberExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::StaticMemberExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, StaticPropertyName<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, StaticPropertyName<'alloc>>) -> StackValue<'alloc> {
        StackValue::StaticPropertyName(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, SwitchCase<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, SwitchCase<'alloc>>) -> StackValue<'alloc> {
        StackValue::SwitchCase(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, SwitchDefault<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, SwitchDefault<'alloc>>) -> StackValue<'alloc> {
        StackValue::SwitchDefault(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, SwitchStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, SwitchStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::SwitchStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, SwitchStatementWithDefault<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, SwitchStatementWithDefault<'alloc>>) -> StackValue<'alloc> {
        StackValue::SwitchStatementWithDefault(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, TemplateElement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, TemplateElement<'alloc>>) -> StackValue<'alloc> {
        StackValue::TemplateElement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, TemplateExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, TemplateExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::TemplateExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, TemplateExpressionElement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, TemplateExpressionElement<'alloc>>) -> StackValue<'alloc> {
        StackValue::TemplateExpressionElement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, ThrowStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, ThrowStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::ThrowStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Token<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Token<'alloc>>) -> StackValue<'alloc> {
        StackValue::Token(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, TryCatchStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, TryCatchStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::TryCatchStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, TryFinallyStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, TryFinallyStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::TryFinallyStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, UnaryExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, UnaryExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::UnaryExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, UnaryOperator>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, UnaryOperator>) -> StackValue<'alloc> {
        StackValue::UnaryOperator(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, UpdateExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, UpdateExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::UpdateExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, UpdateOperator>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, UpdateOperator>) -> StackValue<'alloc> {
        StackValue::UpdateOperator(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, VariableDeclaration<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, VariableDeclaration<'alloc>>) -> StackValue<'alloc> {
        StackValue::VariableDeclaration(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, VariableDeclarationKind>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, VariableDeclarationKind>) -> StackValue<'alloc> {
        StackValue::VariableDeclarationKind(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, VariableDeclarationOrAssignmentTarget<'alloc>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, VariableDeclarationOrAssignmentTarget<'alloc>>,
    ) -> StackValue<'alloc> {
        StackValue::VariableDeclarationOrAssignmentTarget(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, VariableDeclarationOrExpression<'alloc>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, VariableDeclarationOrExpression<'alloc>>,
    ) -> StackValue<'alloc> {
        StackValue::VariableDeclarationOrExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, VariableDeclarator<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, VariableDeclarator<'alloc>>) -> StackValue<'alloc> {
        StackValue::VariableDeclarator(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, VariableReference<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, VariableReference<'alloc>>) -> StackValue<'alloc> {
        StackValue::VariableReference(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, arena::Vec<'alloc, ArrayExpressionElement<'alloc>>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, arena::Vec<'alloc, ArrayExpressionElement<'alloc>>>,
    ) -> StackValue<'alloc> {
        StackValue::VecArrayExpressionElement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>>,
    ) -> StackValue<'alloc> {
        StackValue::VecBindingProperty(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>>,
    ) -> StackValue<'alloc> {
        StackValue::VecClassElement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>>,
    ) -> StackValue<'alloc> {
        StackValue::VecOption(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>>
    for StackValue<'alloc>
{
    fn from(val: arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>) -> StackValue<'alloc> {
        StackValue::VecStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>>>
    for StackValue<'alloc>
{
    fn from(val: arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>>) -> StackValue<'alloc> {
        StackValue::VecSwitchCase(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>>
    for StackValue<'alloc>
{
    fn from(
        val: arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>,
    ) -> StackValue<'alloc> {
        StackValue::VecVariableDeclarator(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, Void>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, Void>) -> StackValue<'alloc> {
        StackValue::Void(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, WhileStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, WhileStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::WhileStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, WithStatement<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, WithStatement<'alloc>>) -> StackValue<'alloc> {
        StackValue::WithStatement(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, YieldExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, YieldExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::YieldExpression(val)
    }
}

impl<'alloc> From<arena::Box<'alloc, YieldGeneratorExpression<'alloc>>> for StackValue<'alloc> {
    fn from(val: arena::Box<'alloc, YieldGeneratorExpression<'alloc>>) -> StackValue<'alloc> {
        StackValue::YieldGeneratorExpression(val)
    }
}
