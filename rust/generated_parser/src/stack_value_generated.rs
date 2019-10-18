// WARNING: This file is auto-generated.

use crate::token::Token;
use ast::arena;
use ast::types::*;
use std::convert::Infallible;

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
    ImportCallExpression(arena::Box<'alloc, ImportCallExpression<'alloc>>),
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

/// Values that can be converted to StackValues, fallibly.
pub trait TryIntoStack<'alloc> {
    type Error;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Self::Error>;
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

impl<'alloc> StackValueItem<'alloc> for ImportCallExpression<'alloc> {
    fn to_ast(sv: StackValue<'alloc>) -> arena::Box<'alloc, Self> {
        match sv {
            StackValue::ImportCallExpression(v) => v,
            _ => panic!("StackValue expected ImportCallExpression, got {:?}", sv),
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

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Argument<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Argument(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Arguments<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Arguments(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ArrayAssignmentTarget<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ArrayAssignmentTarget(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ArrayBinding<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ArrayBinding(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ArrayExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ArrayExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ArrayExpressionElement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ArrayExpressionElement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ArrowExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ArrowExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ArrowExpressionBody<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ArrowExpressionBody(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AssignmentExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AssignmentTarget<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentTarget(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AssignmentTargetIdentifier<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentTargetIdentifier(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AssignmentTargetMaybeDefault<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentTargetMaybeDefault(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AssignmentTargetPattern<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentTargetPattern(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AssignmentTargetProperty<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentTargetProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc>
    for arena::Box<'alloc, AssignmentTargetPropertyIdentifier<'alloc>>
{
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentTargetPropertyIdentifier(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AssignmentTargetPropertyProperty<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentTargetPropertyProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AssignmentTargetWithDefault<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AssignmentTargetWithDefault(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, AwaitExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::AwaitExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BinaryExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BinaryExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BinaryOperator> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BinaryOperator(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Binding<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Binding(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BindingIdentifier<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BindingIdentifier(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BindingPattern<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BindingPattern(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BindingProperty<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BindingProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BindingPropertyIdentifier<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BindingPropertyIdentifier(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BindingPropertyProperty<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BindingPropertyProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BindingWithDefault<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BindingWithDefault(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Block<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Block(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BlockStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BlockStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, BreakStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::BreakStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, CallExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::CallExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, CatchClause<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::CatchClause(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ClassDeclaration<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ClassDeclaration(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ClassElement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ClassElement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ClassExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ClassExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, CompoundAssignmentExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::CompoundAssignmentExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, CompoundAssignmentOperator> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::CompoundAssignmentOperator(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ComputedMemberAssignmentTarget<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ComputedMemberAssignmentTarget(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ComputedMemberExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ComputedMemberExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ComputedPropertyName<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ComputedPropertyName(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ConditionalExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ConditionalExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ContinueStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ContinueStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, CoverParenthesized<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::CoverParenthesized(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, DataProperty<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::DataProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Directive<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Directive(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, DoWhileStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::DoWhileStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Export<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Export(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ExportAllFrom<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ExportAllFrom(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ExportDeclaration<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ExportDeclaration(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ExportDefault<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ExportDefault(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ExportFrom<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ExportFrom(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ExportFromSpecifier<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ExportFromSpecifier(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ExportLocalSpecifier<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ExportLocalSpecifier(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ExportLocals<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ExportLocals(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Expression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Expression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ExpressionOrSuper<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ExpressionOrSuper(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ForInStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ForInStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ForOfStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ForOfStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ForStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ForStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, FormalParameters<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::FormalParameters(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Function<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Function(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, FunctionBody<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::FunctionBody(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Getter<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Getter(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Identifier<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Identifier(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, IdentifierExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::IdentifierExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, IdentifierName<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::IdentifierName(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, IfStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::IfStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Import<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Import(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ImportCallExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ImportCallExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ImportDeclaration<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ImportDeclaration(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ImportNamespace<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ImportNamespace(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ImportSpecifier<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ImportSpecifier(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Label<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Label(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, LabeledStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::LabeledStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, LiteralBooleanExpression> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::LiteralBooleanExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, LiteralNumericExpression> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::LiteralNumericExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, LiteralRegExpExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::LiteralRegExpExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, LiteralStringExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::LiteralStringExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, MemberAssignmentTarget<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::MemberAssignmentTarget(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, MemberExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::MemberExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Method<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Method(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, MethodDefinition<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::MethodDefinition(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Module<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Module(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ModuleItems<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ModuleItems(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, NamedObjectProperty<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::NamedObjectProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, NewExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::NewExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ObjectAssignmentTarget<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ObjectAssignmentTarget(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ObjectBinding<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ObjectBinding(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ObjectExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ObjectExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ObjectProperty<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ObjectProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Parameter<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Parameter(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Program<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Program(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, PropertyName<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::PropertyName(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ReturnStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ReturnStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Script<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Script(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Setter<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Setter(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ShorthandProperty<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ShorthandProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, SimpleAssignmentTarget<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::SimpleAssignmentTarget(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Statement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Statement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, StaticMemberAssignmentTarget<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::StaticMemberAssignmentTarget(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, StaticMemberExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::StaticMemberExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, StaticPropertyName<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::StaticPropertyName(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, SwitchCase<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::SwitchCase(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, SwitchDefault<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::SwitchDefault(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, SwitchStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::SwitchStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, SwitchStatementWithDefault<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::SwitchStatementWithDefault(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, TemplateElement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::TemplateElement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, TemplateExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::TemplateExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, TemplateExpressionElement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::TemplateExpressionElement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, ThrowStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::ThrowStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Token<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Token(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, TryCatchStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::TryCatchStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, TryFinallyStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::TryFinallyStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, UnaryExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::UnaryExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, UnaryOperator> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::UnaryOperator(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, UpdateExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::UpdateExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, UpdateOperator> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::UpdateOperator(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, VariableDeclaration<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VariableDeclaration(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, VariableDeclarationKind> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VariableDeclarationKind(self))
    }
}

impl<'alloc> TryIntoStack<'alloc>
    for arena::Box<'alloc, VariableDeclarationOrAssignmentTarget<'alloc>>
{
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VariableDeclarationOrAssignmentTarget(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, VariableDeclarationOrExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VariableDeclarationOrExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, VariableDeclarator<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VariableDeclarator(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, VariableReference<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VariableReference(self))
    }
}

impl<'alloc> TryIntoStack<'alloc>
    for arena::Box<'alloc, arena::Vec<'alloc, ArrayExpressionElement<'alloc>>>
{
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VecArrayExpressionElement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc>
    for arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>>
{
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VecBindingProperty(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VecClassElement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc>
    for arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>>
{
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VecOption(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VecStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VecSwitchCase(self))
    }
}

impl<'alloc> TryIntoStack<'alloc>
    for arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>
{
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::VecVariableDeclarator(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, Void> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::Void(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, WhileStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::WhileStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, WithStatement<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::WithStatement(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, YieldExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::YieldExpression(self))
    }
}

impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, YieldGeneratorExpression<'alloc>> {
    type Error = Infallible;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {
        Ok(StackValue::YieldGeneratorExpression(self))
    }
}

impl<'alloc, T, E> TryIntoStack<'alloc> for Result<T, E>
where
    T: TryIntoStack<'alloc>,
    E: From<T::Error>,
{
    type Error = E;
    fn try_into_stack(self) -> Result<StackValue<'alloc>, E> {
        Ok(self?.try_into_stack()?)
    }
}
