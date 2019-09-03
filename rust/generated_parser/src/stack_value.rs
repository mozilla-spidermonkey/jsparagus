// WARNING: This file is auto-generated.

use crate::Token;
use ast::*;

#[derive(Debug, PartialEq)]
pub enum StackValue<'a> {
    Argument(Box<Argument>),
    Arguments(Box<Arguments>),
    ArrayAssignmentTarget(Box<ArrayAssignmentTarget>),
    ArrayBinding(Box<ArrayBinding>),
    ArrayExpression(Box<ArrayExpression>),
    ArrayExpressionElement(Box<ArrayExpressionElement>),
    ArrowExpression(Box<ArrowExpression>),
    ArrowExpressionBody(Box<ArrowExpressionBody>),
    AssignmentExpression(Box<AssignmentExpression>),
    AssignmentTarget(Box<AssignmentTarget>),
    AssignmentTargetIdentifier(Box<AssignmentTargetIdentifier>),
    AssignmentTargetMaybeDefault(Box<AssignmentTargetMaybeDefault>),
    AssignmentTargetPattern(Box<AssignmentTargetPattern>),
    AssignmentTargetProperty(Box<AssignmentTargetProperty>),
    AssignmentTargetPropertyIdentifier(Box<AssignmentTargetPropertyIdentifier>),
    AssignmentTargetPropertyProperty(Box<AssignmentTargetPropertyProperty>),
    AssignmentTargetWithDefault(Box<AssignmentTargetWithDefault>),
    AwaitExpression(Box<AwaitExpression>),
    BinaryExpression(Box<BinaryExpression>),
    BinaryOperator(Box<BinaryOperator>),
    Binding(Box<Binding>),
    BindingIdentifier(Box<BindingIdentifier>),
    BindingPattern(Box<BindingPattern>),
    BindingProperty(Box<BindingProperty>),
    BindingPropertyIdentifier(Box<BindingPropertyIdentifier>),
    BindingPropertyProperty(Box<BindingPropertyProperty>),
    BindingWithDefault(Box<BindingWithDefault>),
    Block(Box<Block>),
    BlockStatement(Box<BlockStatement>),
    BreakStatement(Box<BreakStatement>),
    CallExpression(Box<CallExpression>),
    CatchClause(Box<CatchClause>),
    ClassDeclaration(Box<ClassDeclaration>),
    ClassElement(Box<ClassElement>),
    ClassExpression(Box<ClassExpression>),
    CompoundAssignmentExpression(Box<CompoundAssignmentExpression>),
    CompoundAssignmentOperator(Box<CompoundAssignmentOperator>),
    ComputedMemberAssignmentTarget(Box<ComputedMemberAssignmentTarget>),
    ComputedMemberExpression(Box<ComputedMemberExpression>),
    ComputedPropertyName(Box<ComputedPropertyName>),
    ConditionalExpression(Box<ConditionalExpression>),
    ContinueStatement(Box<ContinueStatement>),
    CoverParenthesized(Box<CoverParenthesized>),
    DataProperty(Box<DataProperty>),
    Directive(Box<Directive>),
    DoWhileStatement(Box<DoWhileStatement>),
    Export(Box<Export>),
    ExportAllFrom(Box<ExportAllFrom>),
    ExportDeclaration(Box<ExportDeclaration>),
    ExportDefault(Box<ExportDefault>),
    ExportFrom(Box<ExportFrom>),
    ExportFromSpecifier(Box<ExportFromSpecifier>),
    ExportLocalSpecifier(Box<ExportLocalSpecifier>),
    ExportLocals(Box<ExportLocals>),
    Expression(Box<Expression>),
    ExpressionOrSuper(Box<ExpressionOrSuper>),
    ForInStatement(Box<ForInStatement>),
    ForOfStatement(Box<ForOfStatement>),
    ForStatement(Box<ForStatement>),
    FormalParameters(Box<FormalParameters>),
    Function(Box<Function>),
    FunctionBody(Box<FunctionBody>),
    Getter(Box<Getter>),
    Identifier(Box<Identifier>),
    IdentifierExpression(Box<IdentifierExpression>),
    IdentifierName(Box<IdentifierName>),
    IfStatement(Box<IfStatement>),
    Import(Box<Import>),
    ImportDeclaration(Box<ImportDeclaration>),
    ImportNamespace(Box<ImportNamespace>),
    ImportSpecifier(Box<ImportSpecifier>),
    Label(Box<Label>),
    LabeledStatement(Box<LabeledStatement>),
    LiteralBooleanExpression(Box<LiteralBooleanExpression>),
    LiteralNumericExpression(Box<LiteralNumericExpression>),
    LiteralRegExpExpression(Box<LiteralRegExpExpression>),
    LiteralStringExpression(Box<LiteralStringExpression>),
    MemberAssignmentTarget(Box<MemberAssignmentTarget>),
    MemberExpression(Box<MemberExpression>),
    Method(Box<Method>),
    MethodDefinition(Box<MethodDefinition>),
    Module(Box<Module>),
    ModuleItems(Box<ModuleItems>),
    NamedObjectProperty(Box<NamedObjectProperty>),
    NewExpression(Box<NewExpression>),
    ObjectAssignmentTarget(Box<ObjectAssignmentTarget>),
    ObjectBinding(Box<ObjectBinding>),
    ObjectExpression(Box<ObjectExpression>),
    ObjectProperty(Box<ObjectProperty>),
    Parameter(Box<Parameter>),
    Program(Box<Program>),
    PropertyName(Box<PropertyName>),
    ReturnStatement(Box<ReturnStatement>),
    Script(Box<Script>),
    Setter(Box<Setter>),
    ShorthandProperty(Box<ShorthandProperty>),
    SimpleAssignmentTarget(Box<SimpleAssignmentTarget>),
    Statement(Box<Statement>),
    StaticMemberAssignmentTarget(Box<StaticMemberAssignmentTarget>),
    StaticMemberExpression(Box<StaticMemberExpression>),
    StaticPropertyName(Box<StaticPropertyName>),
    SwitchCase(Box<SwitchCase>),
    SwitchDefault(Box<SwitchDefault>),
    SwitchStatement(Box<SwitchStatement>),
    SwitchStatementWithDefault(Box<SwitchStatementWithDefault>),
    TemplateElement(Box<TemplateElement>),
    TemplateExpression(Box<TemplateExpression>),
    TemplateExpressionElement(Box<TemplateExpressionElement>),
    ThrowStatement(Box<ThrowStatement>),
    Token(Box<Token<'a>>),
    TryCatchStatement(Box<TryCatchStatement>),
    TryFinallyStatement(Box<TryFinallyStatement>),
    UnaryExpression(Box<UnaryExpression>),
    UnaryOperator(Box<UnaryOperator>),
    UpdateExpression(Box<UpdateExpression>),
    UpdateOperator(Box<UpdateOperator>),
    VariableDeclaration(Box<VariableDeclaration>),
    VariableDeclarationKind(Box<VariableDeclarationKind>),
    VariableDeclarationOrAssignmentTarget(Box<VariableDeclarationOrAssignmentTarget>),
    VariableDeclarationOrExpression(Box<VariableDeclarationOrExpression>),
    VariableDeclarator(Box<VariableDeclarator>),
    VariableReference(Box<VariableReference>),
    VecArrayExpressionElement(Box<Vec<ArrayExpressionElement>>),
    VecBindingProperty(Box<Vec<BindingProperty>>),
    VecClassElement(Box<Vec<ClassElement>>),
    VecParameter(Box<Vec<Option<Parameter>>>),
    VecStatement(Box<Vec<Statement>>),
    VecSwitchCase(Box<Vec<SwitchCase>>),
    VecVariableDeclarator(Box<Vec<VariableDeclarator>>),
    Void(Box<Void>),
    WhileStatement(Box<WhileStatement>),
    WithStatement(Box<WithStatement>),
    YieldExpression(Box<YieldExpression>),
    YieldGeneratorExpression(Box<YieldGeneratorExpression>),
}

impl<'a> StackValue<'a> {
    pub fn to_ast<T: StackValueItem<'a>>(self) -> Box<T> {
        T::to_ast(self)
    }
}

pub trait StackValueItem<'a> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self>;
}

impl<'a> StackValueItem<'a> for Argument {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Argument(v) => v,
            _ => panic!("StackValue expected Argument, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Arguments {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Arguments(v) => v,
            _ => panic!("StackValue expected Arguments, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ArrayAssignmentTarget {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ArrayAssignmentTarget(v) => v,
            _ => panic!("StackValue expected ArrayAssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ArrayBinding {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ArrayBinding(v) => v,
            _ => panic!("StackValue expected ArrayBinding, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ArrayExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ArrayExpression(v) => v,
            _ => panic!("StackValue expected ArrayExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ArrayExpressionElement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ArrayExpressionElement(v) => v,
            _ => panic!("StackValue expected ArrayExpressionElement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ArrowExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ArrowExpression(v) => v,
            _ => panic!("StackValue expected ArrowExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ArrowExpressionBody {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ArrowExpressionBody(v) => v,
            _ => panic!("StackValue expected ArrowExpressionBody, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentExpression(v) => v,
            _ => panic!("StackValue expected AssignmentExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentTarget {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentTarget(v) => v,
            _ => panic!("StackValue expected AssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentTargetIdentifier {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetIdentifier(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetIdentifier, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentTargetMaybeDefault {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetMaybeDefault(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetMaybeDefault, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentTargetPattern {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetPattern(v) => v,
            _ => panic!("StackValue expected AssignmentTargetPattern, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentTargetProperty {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetProperty(v) => v,
            _ => panic!("StackValue expected AssignmentTargetProperty, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentTargetPropertyIdentifier {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetPropertyIdentifier(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetPropertyIdentifier, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentTargetPropertyProperty {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetPropertyProperty(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetPropertyProperty, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for AssignmentTargetWithDefault {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetWithDefault(v) => v,
            _ => panic!(
                "StackValue expected AssignmentTargetWithDefault, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for AwaitExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::AwaitExpression(v) => v,
            _ => panic!("StackValue expected AwaitExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BinaryExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BinaryExpression(v) => v,
            _ => panic!("StackValue expected BinaryExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BinaryOperator {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BinaryOperator(v) => v,
            _ => panic!("StackValue expected BinaryOperator, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Binding {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Binding(v) => v,
            _ => panic!("StackValue expected Binding, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BindingIdentifier {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BindingIdentifier(v) => v,
            _ => panic!("StackValue expected BindingIdentifier, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BindingPattern {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BindingPattern(v) => v,
            _ => panic!("StackValue expected BindingPattern, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BindingProperty {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BindingProperty(v) => v,
            _ => panic!("StackValue expected BindingProperty, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BindingPropertyIdentifier {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BindingPropertyIdentifier(v) => v,
            _ => panic!(
                "StackValue expected BindingPropertyIdentifier, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for BindingPropertyProperty {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BindingPropertyProperty(v) => v,
            _ => panic!("StackValue expected BindingPropertyProperty, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BindingWithDefault {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BindingWithDefault(v) => v,
            _ => panic!("StackValue expected BindingWithDefault, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Block {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Block(v) => v,
            _ => panic!("StackValue expected Block, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BlockStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BlockStatement(v) => v,
            _ => panic!("StackValue expected BlockStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for BreakStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::BreakStatement(v) => v,
            _ => panic!("StackValue expected BreakStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for CallExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::CallExpression(v) => v,
            _ => panic!("StackValue expected CallExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for CatchClause {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::CatchClause(v) => v,
            _ => panic!("StackValue expected CatchClause, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ClassDeclaration {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ClassDeclaration(v) => v,
            _ => panic!("StackValue expected ClassDeclaration, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ClassElement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ClassElement(v) => v,
            _ => panic!("StackValue expected ClassElement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ClassExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ClassExpression(v) => v,
            _ => panic!("StackValue expected ClassExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for CompoundAssignmentExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::CompoundAssignmentExpression(v) => v,
            _ => panic!(
                "StackValue expected CompoundAssignmentExpression, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for CompoundAssignmentOperator {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::CompoundAssignmentOperator(v) => v,
            _ => panic!(
                "StackValue expected CompoundAssignmentOperator, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for ComputedMemberAssignmentTarget {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ComputedMemberAssignmentTarget(v) => v,
            _ => panic!(
                "StackValue expected ComputedMemberAssignmentTarget, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for ComputedMemberExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ComputedMemberExpression(v) => v,
            _ => panic!("StackValue expected ComputedMemberExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ComputedPropertyName {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ComputedPropertyName(v) => v,
            _ => panic!("StackValue expected ComputedPropertyName, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ConditionalExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ConditionalExpression(v) => v,
            _ => panic!("StackValue expected ConditionalExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ContinueStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ContinueStatement(v) => v,
            _ => panic!("StackValue expected ContinueStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for CoverParenthesized {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::CoverParenthesized(v) => v,
            _ => panic!("StackValue expected CoverParenthesized, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for DataProperty {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::DataProperty(v) => v,
            _ => panic!("StackValue expected DataProperty, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Directive {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Directive(v) => v,
            _ => panic!("StackValue expected Directive, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for DoWhileStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::DoWhileStatement(v) => v,
            _ => panic!("StackValue expected DoWhileStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Export {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Export(v) => v,
            _ => panic!("StackValue expected Export, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ExportAllFrom {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ExportAllFrom(v) => v,
            _ => panic!("StackValue expected ExportAllFrom, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ExportDeclaration {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ExportDeclaration(v) => v,
            _ => panic!("StackValue expected ExportDeclaration, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ExportDefault {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ExportDefault(v) => v,
            _ => panic!("StackValue expected ExportDefault, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ExportFrom {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ExportFrom(v) => v,
            _ => panic!("StackValue expected ExportFrom, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ExportFromSpecifier {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ExportFromSpecifier(v) => v,
            _ => panic!("StackValue expected ExportFromSpecifier, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ExportLocalSpecifier {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ExportLocalSpecifier(v) => v,
            _ => panic!("StackValue expected ExportLocalSpecifier, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ExportLocals {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ExportLocals(v) => v,
            _ => panic!("StackValue expected ExportLocals, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Expression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Expression(v) => v,
            _ => panic!("StackValue expected Expression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ExpressionOrSuper {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ExpressionOrSuper(v) => v,
            _ => panic!("StackValue expected ExpressionOrSuper, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ForInStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ForInStatement(v) => v,
            _ => panic!("StackValue expected ForInStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ForOfStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ForOfStatement(v) => v,
            _ => panic!("StackValue expected ForOfStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ForStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ForStatement(v) => v,
            _ => panic!("StackValue expected ForStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for FormalParameters {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::FormalParameters(v) => v,
            _ => panic!("StackValue expected FormalParameters, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Function {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Function(v) => v,
            _ => panic!("StackValue expected Function, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for FunctionBody {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::FunctionBody(v) => v,
            _ => panic!("StackValue expected FunctionBody, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Getter {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Getter(v) => v,
            _ => panic!("StackValue expected Getter, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Identifier {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Identifier(v) => v,
            _ => panic!("StackValue expected Identifier, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for IdentifierExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::IdentifierExpression(v) => v,
            _ => panic!("StackValue expected IdentifierExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for IdentifierName {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::IdentifierName(v) => v,
            _ => panic!("StackValue expected IdentifierName, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for IfStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::IfStatement(v) => v,
            _ => panic!("StackValue expected IfStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Import {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Import(v) => v,
            _ => panic!("StackValue expected Import, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ImportDeclaration {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ImportDeclaration(v) => v,
            _ => panic!("StackValue expected ImportDeclaration, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ImportNamespace {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ImportNamespace(v) => v,
            _ => panic!("StackValue expected ImportNamespace, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ImportSpecifier {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ImportSpecifier(v) => v,
            _ => panic!("StackValue expected ImportSpecifier, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Label {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Label(v) => v,
            _ => panic!("StackValue expected Label, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for LabeledStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::LabeledStatement(v) => v,
            _ => panic!("StackValue expected LabeledStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for LiteralBooleanExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::LiteralBooleanExpression(v) => v,
            _ => panic!("StackValue expected LiteralBooleanExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for LiteralNumericExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::LiteralNumericExpression(v) => v,
            _ => panic!("StackValue expected LiteralNumericExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for LiteralRegExpExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::LiteralRegExpExpression(v) => v,
            _ => panic!("StackValue expected LiteralRegExpExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for LiteralStringExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::LiteralStringExpression(v) => v,
            _ => panic!("StackValue expected LiteralStringExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for MemberAssignmentTarget {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::MemberAssignmentTarget(v) => v,
            _ => panic!("StackValue expected MemberAssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for MemberExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::MemberExpression(v) => v,
            _ => panic!("StackValue expected MemberExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Method {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Method(v) => v,
            _ => panic!("StackValue expected Method, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for MethodDefinition {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::MethodDefinition(v) => v,
            _ => panic!("StackValue expected MethodDefinition, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Module {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Module(v) => v,
            _ => panic!("StackValue expected Module, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ModuleItems {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ModuleItems(v) => v,
            _ => panic!("StackValue expected ModuleItems, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for NamedObjectProperty {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::NamedObjectProperty(v) => v,
            _ => panic!("StackValue expected NamedObjectProperty, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for NewExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::NewExpression(v) => v,
            _ => panic!("StackValue expected NewExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ObjectAssignmentTarget {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ObjectAssignmentTarget(v) => v,
            _ => panic!("StackValue expected ObjectAssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ObjectBinding {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ObjectBinding(v) => v,
            _ => panic!("StackValue expected ObjectBinding, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ObjectExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ObjectExpression(v) => v,
            _ => panic!("StackValue expected ObjectExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ObjectProperty {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ObjectProperty(v) => v,
            _ => panic!("StackValue expected ObjectProperty, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Parameter {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Parameter(v) => v,
            _ => panic!("StackValue expected Parameter, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Program {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Program(v) => v,
            _ => panic!("StackValue expected Program, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for PropertyName {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::PropertyName(v) => v,
            _ => panic!("StackValue expected PropertyName, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ReturnStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ReturnStatement(v) => v,
            _ => panic!("StackValue expected ReturnStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Script {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Script(v) => v,
            _ => panic!("StackValue expected Script, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Setter {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Setter(v) => v,
            _ => panic!("StackValue expected Setter, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for ShorthandProperty {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ShorthandProperty(v) => v,
            _ => panic!("StackValue expected ShorthandProperty, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for SimpleAssignmentTarget {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::SimpleAssignmentTarget(v) => v,
            _ => panic!("StackValue expected SimpleAssignmentTarget, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Statement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Statement(v) => v,
            _ => panic!("StackValue expected Statement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for StaticMemberAssignmentTarget {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::StaticMemberAssignmentTarget(v) => v,
            _ => panic!(
                "StackValue expected StaticMemberAssignmentTarget, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for StaticMemberExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::StaticMemberExpression(v) => v,
            _ => panic!("StackValue expected StaticMemberExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for StaticPropertyName {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::StaticPropertyName(v) => v,
            _ => panic!("StackValue expected StaticPropertyName, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for SwitchCase {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::SwitchCase(v) => v,
            _ => panic!("StackValue expected SwitchCase, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for SwitchDefault {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::SwitchDefault(v) => v,
            _ => panic!("StackValue expected SwitchDefault, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for SwitchStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::SwitchStatement(v) => v,
            _ => panic!("StackValue expected SwitchStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for SwitchStatementWithDefault {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::SwitchStatementWithDefault(v) => v,
            _ => panic!(
                "StackValue expected SwitchStatementWithDefault, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for TemplateElement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::TemplateElement(v) => v,
            _ => panic!("StackValue expected TemplateElement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for TemplateExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::TemplateExpression(v) => v,
            _ => panic!("StackValue expected TemplateExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for TemplateExpressionElement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::TemplateExpressionElement(v) => v,
            _ => panic!(
                "StackValue expected TemplateExpressionElement, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for ThrowStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::ThrowStatement(v) => v,
            _ => panic!("StackValue expected ThrowStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Token<'a> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Token(v) => v,
            _ => panic!("StackValue expected Token<'a>, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for TryCatchStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::TryCatchStatement(v) => v,
            _ => panic!("StackValue expected TryCatchStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for TryFinallyStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::TryFinallyStatement(v) => v,
            _ => panic!("StackValue expected TryFinallyStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for UnaryExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::UnaryExpression(v) => v,
            _ => panic!("StackValue expected UnaryExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for UnaryOperator {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::UnaryOperator(v) => v,
            _ => panic!("StackValue expected UnaryOperator, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for UpdateExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::UpdateExpression(v) => v,
            _ => panic!("StackValue expected UpdateExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for UpdateOperator {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::UpdateOperator(v) => v,
            _ => panic!("StackValue expected UpdateOperator, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for VariableDeclaration {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VariableDeclaration(v) => v,
            _ => panic!("StackValue expected VariableDeclaration, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for VariableDeclarationKind {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VariableDeclarationKind(v) => v,
            _ => panic!("StackValue expected VariableDeclarationKind, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for VariableDeclarationOrAssignmentTarget {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VariableDeclarationOrAssignmentTarget(v) => v,
            _ => panic!(
                "StackValue expected VariableDeclarationOrAssignmentTarget, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for VariableDeclarationOrExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VariableDeclarationOrExpression(v) => v,
            _ => panic!(
                "StackValue expected VariableDeclarationOrExpression, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for VariableDeclarator {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VariableDeclarator(v) => v,
            _ => panic!("StackValue expected VariableDeclarator, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for VariableReference {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VariableReference(v) => v,
            _ => panic!("StackValue expected VariableReference, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Vec<ArrayExpressionElement> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VecArrayExpressionElement(v) => v,
            _ => panic!(
                "StackValue expected Vec<ArrayExpressionElement>, got {:?}",
                sv
            ),
        }
    }
}

impl<'a> StackValueItem<'a> for Vec<BindingProperty> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VecBindingProperty(v) => v,
            _ => panic!("StackValue expected Vec<BindingProperty>, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Vec<ClassElement> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VecClassElement(v) => v,
            _ => panic!("StackValue expected Vec<ClassElement>, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Vec<Option<Parameter>> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VecParameter(v) => v,
            _ => panic!("StackValue expected Vec<Option<Parameter>>, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Vec<Statement> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VecStatement(v) => v,
            _ => panic!("StackValue expected Vec<Statement>, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Vec<SwitchCase> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VecSwitchCase(v) => v,
            _ => panic!("StackValue expected Vec<SwitchCase>, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Vec<VariableDeclarator> {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::VecVariableDeclarator(v) => v,
            _ => panic!("StackValue expected Vec<VariableDeclarator>, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for Void {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::Void(v) => v,
            _ => panic!("StackValue expected Void, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for WhileStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::WhileStatement(v) => v,
            _ => panic!("StackValue expected WhileStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for WithStatement {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::WithStatement(v) => v,
            _ => panic!("StackValue expected WithStatement, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for YieldExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::YieldExpression(v) => v,
            _ => panic!("StackValue expected YieldExpression, got {:?}", sv),
        }
    }
}

impl<'a> StackValueItem<'a> for YieldGeneratorExpression {
    fn to_ast(sv: StackValue<'a>) -> Box<Self> {
        match sv {
            StackValue::YieldGeneratorExpression(v) => v,
            _ => panic!("StackValue expected YieldGeneratorExpression, got {:?}", sv),
        }
    }
}

impl From<Box<Argument>> for StackValue<'static> {
    fn from(val: Box<Argument>) -> StackValue<'static> {
        StackValue::Argument(val)
    }
}

impl From<Box<Arguments>> for StackValue<'static> {
    fn from(val: Box<Arguments>) -> StackValue<'static> {
        StackValue::Arguments(val)
    }
}

impl From<Box<ArrayAssignmentTarget>> for StackValue<'static> {
    fn from(val: Box<ArrayAssignmentTarget>) -> StackValue<'static> {
        StackValue::ArrayAssignmentTarget(val)
    }
}

impl From<Box<ArrayBinding>> for StackValue<'static> {
    fn from(val: Box<ArrayBinding>) -> StackValue<'static> {
        StackValue::ArrayBinding(val)
    }
}

impl From<Box<ArrayExpression>> for StackValue<'static> {
    fn from(val: Box<ArrayExpression>) -> StackValue<'static> {
        StackValue::ArrayExpression(val)
    }
}

impl From<Box<ArrayExpressionElement>> for StackValue<'static> {
    fn from(val: Box<ArrayExpressionElement>) -> StackValue<'static> {
        StackValue::ArrayExpressionElement(val)
    }
}

impl From<Box<ArrowExpression>> for StackValue<'static> {
    fn from(val: Box<ArrowExpression>) -> StackValue<'static> {
        StackValue::ArrowExpression(val)
    }
}

impl From<Box<ArrowExpressionBody>> for StackValue<'static> {
    fn from(val: Box<ArrowExpressionBody>) -> StackValue<'static> {
        StackValue::ArrowExpressionBody(val)
    }
}

impl From<Box<AssignmentExpression>> for StackValue<'static> {
    fn from(val: Box<AssignmentExpression>) -> StackValue<'static> {
        StackValue::AssignmentExpression(val)
    }
}

impl From<Box<AssignmentTarget>> for StackValue<'static> {
    fn from(val: Box<AssignmentTarget>) -> StackValue<'static> {
        StackValue::AssignmentTarget(val)
    }
}

impl From<Box<AssignmentTargetIdentifier>> for StackValue<'static> {
    fn from(val: Box<AssignmentTargetIdentifier>) -> StackValue<'static> {
        StackValue::AssignmentTargetIdentifier(val)
    }
}

impl From<Box<AssignmentTargetMaybeDefault>> for StackValue<'static> {
    fn from(val: Box<AssignmentTargetMaybeDefault>) -> StackValue<'static> {
        StackValue::AssignmentTargetMaybeDefault(val)
    }
}

impl From<Box<AssignmentTargetPattern>> for StackValue<'static> {
    fn from(val: Box<AssignmentTargetPattern>) -> StackValue<'static> {
        StackValue::AssignmentTargetPattern(val)
    }
}

impl From<Box<AssignmentTargetProperty>> for StackValue<'static> {
    fn from(val: Box<AssignmentTargetProperty>) -> StackValue<'static> {
        StackValue::AssignmentTargetProperty(val)
    }
}

impl From<Box<AssignmentTargetPropertyIdentifier>> for StackValue<'static> {
    fn from(val: Box<AssignmentTargetPropertyIdentifier>) -> StackValue<'static> {
        StackValue::AssignmentTargetPropertyIdentifier(val)
    }
}

impl From<Box<AssignmentTargetPropertyProperty>> for StackValue<'static> {
    fn from(val: Box<AssignmentTargetPropertyProperty>) -> StackValue<'static> {
        StackValue::AssignmentTargetPropertyProperty(val)
    }
}

impl From<Box<AssignmentTargetWithDefault>> for StackValue<'static> {
    fn from(val: Box<AssignmentTargetWithDefault>) -> StackValue<'static> {
        StackValue::AssignmentTargetWithDefault(val)
    }
}

impl From<Box<AwaitExpression>> for StackValue<'static> {
    fn from(val: Box<AwaitExpression>) -> StackValue<'static> {
        StackValue::AwaitExpression(val)
    }
}

impl From<Box<BinaryExpression>> for StackValue<'static> {
    fn from(val: Box<BinaryExpression>) -> StackValue<'static> {
        StackValue::BinaryExpression(val)
    }
}

impl From<Box<BinaryOperator>> for StackValue<'static> {
    fn from(val: Box<BinaryOperator>) -> StackValue<'static> {
        StackValue::BinaryOperator(val)
    }
}

impl From<Box<Binding>> for StackValue<'static> {
    fn from(val: Box<Binding>) -> StackValue<'static> {
        StackValue::Binding(val)
    }
}

impl From<Box<BindingIdentifier>> for StackValue<'static> {
    fn from(val: Box<BindingIdentifier>) -> StackValue<'static> {
        StackValue::BindingIdentifier(val)
    }
}

impl From<Box<BindingPattern>> for StackValue<'static> {
    fn from(val: Box<BindingPattern>) -> StackValue<'static> {
        StackValue::BindingPattern(val)
    }
}

impl From<Box<BindingProperty>> for StackValue<'static> {
    fn from(val: Box<BindingProperty>) -> StackValue<'static> {
        StackValue::BindingProperty(val)
    }
}

impl From<Box<BindingPropertyIdentifier>> for StackValue<'static> {
    fn from(val: Box<BindingPropertyIdentifier>) -> StackValue<'static> {
        StackValue::BindingPropertyIdentifier(val)
    }
}

impl From<Box<BindingPropertyProperty>> for StackValue<'static> {
    fn from(val: Box<BindingPropertyProperty>) -> StackValue<'static> {
        StackValue::BindingPropertyProperty(val)
    }
}

impl From<Box<BindingWithDefault>> for StackValue<'static> {
    fn from(val: Box<BindingWithDefault>) -> StackValue<'static> {
        StackValue::BindingWithDefault(val)
    }
}

impl From<Box<Block>> for StackValue<'static> {
    fn from(val: Box<Block>) -> StackValue<'static> {
        StackValue::Block(val)
    }
}

impl From<Box<BlockStatement>> for StackValue<'static> {
    fn from(val: Box<BlockStatement>) -> StackValue<'static> {
        StackValue::BlockStatement(val)
    }
}

impl From<Box<BreakStatement>> for StackValue<'static> {
    fn from(val: Box<BreakStatement>) -> StackValue<'static> {
        StackValue::BreakStatement(val)
    }
}

impl From<Box<CallExpression>> for StackValue<'static> {
    fn from(val: Box<CallExpression>) -> StackValue<'static> {
        StackValue::CallExpression(val)
    }
}

impl From<Box<CatchClause>> for StackValue<'static> {
    fn from(val: Box<CatchClause>) -> StackValue<'static> {
        StackValue::CatchClause(val)
    }
}

impl From<Box<ClassDeclaration>> for StackValue<'static> {
    fn from(val: Box<ClassDeclaration>) -> StackValue<'static> {
        StackValue::ClassDeclaration(val)
    }
}

impl From<Box<ClassElement>> for StackValue<'static> {
    fn from(val: Box<ClassElement>) -> StackValue<'static> {
        StackValue::ClassElement(val)
    }
}

impl From<Box<ClassExpression>> for StackValue<'static> {
    fn from(val: Box<ClassExpression>) -> StackValue<'static> {
        StackValue::ClassExpression(val)
    }
}

impl From<Box<CompoundAssignmentExpression>> for StackValue<'static> {
    fn from(val: Box<CompoundAssignmentExpression>) -> StackValue<'static> {
        StackValue::CompoundAssignmentExpression(val)
    }
}

impl From<Box<CompoundAssignmentOperator>> for StackValue<'static> {
    fn from(val: Box<CompoundAssignmentOperator>) -> StackValue<'static> {
        StackValue::CompoundAssignmentOperator(val)
    }
}

impl From<Box<ComputedMemberAssignmentTarget>> for StackValue<'static> {
    fn from(val: Box<ComputedMemberAssignmentTarget>) -> StackValue<'static> {
        StackValue::ComputedMemberAssignmentTarget(val)
    }
}

impl From<Box<ComputedMemberExpression>> for StackValue<'static> {
    fn from(val: Box<ComputedMemberExpression>) -> StackValue<'static> {
        StackValue::ComputedMemberExpression(val)
    }
}

impl From<Box<ComputedPropertyName>> for StackValue<'static> {
    fn from(val: Box<ComputedPropertyName>) -> StackValue<'static> {
        StackValue::ComputedPropertyName(val)
    }
}

impl From<Box<ConditionalExpression>> for StackValue<'static> {
    fn from(val: Box<ConditionalExpression>) -> StackValue<'static> {
        StackValue::ConditionalExpression(val)
    }
}

impl From<Box<ContinueStatement>> for StackValue<'static> {
    fn from(val: Box<ContinueStatement>) -> StackValue<'static> {
        StackValue::ContinueStatement(val)
    }
}

impl From<Box<CoverParenthesized>> for StackValue<'static> {
    fn from(val: Box<CoverParenthesized>) -> StackValue<'static> {
        StackValue::CoverParenthesized(val)
    }
}

impl From<Box<DataProperty>> for StackValue<'static> {
    fn from(val: Box<DataProperty>) -> StackValue<'static> {
        StackValue::DataProperty(val)
    }
}

impl From<Box<Directive>> for StackValue<'static> {
    fn from(val: Box<Directive>) -> StackValue<'static> {
        StackValue::Directive(val)
    }
}

impl From<Box<DoWhileStatement>> for StackValue<'static> {
    fn from(val: Box<DoWhileStatement>) -> StackValue<'static> {
        StackValue::DoWhileStatement(val)
    }
}

impl From<Box<Export>> for StackValue<'static> {
    fn from(val: Box<Export>) -> StackValue<'static> {
        StackValue::Export(val)
    }
}

impl From<Box<ExportAllFrom>> for StackValue<'static> {
    fn from(val: Box<ExportAllFrom>) -> StackValue<'static> {
        StackValue::ExportAllFrom(val)
    }
}

impl From<Box<ExportDeclaration>> for StackValue<'static> {
    fn from(val: Box<ExportDeclaration>) -> StackValue<'static> {
        StackValue::ExportDeclaration(val)
    }
}

impl From<Box<ExportDefault>> for StackValue<'static> {
    fn from(val: Box<ExportDefault>) -> StackValue<'static> {
        StackValue::ExportDefault(val)
    }
}

impl From<Box<ExportFrom>> for StackValue<'static> {
    fn from(val: Box<ExportFrom>) -> StackValue<'static> {
        StackValue::ExportFrom(val)
    }
}

impl From<Box<ExportFromSpecifier>> for StackValue<'static> {
    fn from(val: Box<ExportFromSpecifier>) -> StackValue<'static> {
        StackValue::ExportFromSpecifier(val)
    }
}

impl From<Box<ExportLocalSpecifier>> for StackValue<'static> {
    fn from(val: Box<ExportLocalSpecifier>) -> StackValue<'static> {
        StackValue::ExportLocalSpecifier(val)
    }
}

impl From<Box<ExportLocals>> for StackValue<'static> {
    fn from(val: Box<ExportLocals>) -> StackValue<'static> {
        StackValue::ExportLocals(val)
    }
}

impl From<Box<Expression>> for StackValue<'static> {
    fn from(val: Box<Expression>) -> StackValue<'static> {
        StackValue::Expression(val)
    }
}

impl From<Box<ExpressionOrSuper>> for StackValue<'static> {
    fn from(val: Box<ExpressionOrSuper>) -> StackValue<'static> {
        StackValue::ExpressionOrSuper(val)
    }
}

impl From<Box<ForInStatement>> for StackValue<'static> {
    fn from(val: Box<ForInStatement>) -> StackValue<'static> {
        StackValue::ForInStatement(val)
    }
}

impl From<Box<ForOfStatement>> for StackValue<'static> {
    fn from(val: Box<ForOfStatement>) -> StackValue<'static> {
        StackValue::ForOfStatement(val)
    }
}

impl From<Box<ForStatement>> for StackValue<'static> {
    fn from(val: Box<ForStatement>) -> StackValue<'static> {
        StackValue::ForStatement(val)
    }
}

impl From<Box<FormalParameters>> for StackValue<'static> {
    fn from(val: Box<FormalParameters>) -> StackValue<'static> {
        StackValue::FormalParameters(val)
    }
}

impl From<Box<Function>> for StackValue<'static> {
    fn from(val: Box<Function>) -> StackValue<'static> {
        StackValue::Function(val)
    }
}

impl From<Box<FunctionBody>> for StackValue<'static> {
    fn from(val: Box<FunctionBody>) -> StackValue<'static> {
        StackValue::FunctionBody(val)
    }
}

impl From<Box<Getter>> for StackValue<'static> {
    fn from(val: Box<Getter>) -> StackValue<'static> {
        StackValue::Getter(val)
    }
}

impl From<Box<Identifier>> for StackValue<'static> {
    fn from(val: Box<Identifier>) -> StackValue<'static> {
        StackValue::Identifier(val)
    }
}

impl From<Box<IdentifierExpression>> for StackValue<'static> {
    fn from(val: Box<IdentifierExpression>) -> StackValue<'static> {
        StackValue::IdentifierExpression(val)
    }
}

impl From<Box<IdentifierName>> for StackValue<'static> {
    fn from(val: Box<IdentifierName>) -> StackValue<'static> {
        StackValue::IdentifierName(val)
    }
}

impl From<Box<IfStatement>> for StackValue<'static> {
    fn from(val: Box<IfStatement>) -> StackValue<'static> {
        StackValue::IfStatement(val)
    }
}

impl From<Box<Import>> for StackValue<'static> {
    fn from(val: Box<Import>) -> StackValue<'static> {
        StackValue::Import(val)
    }
}

impl From<Box<ImportDeclaration>> for StackValue<'static> {
    fn from(val: Box<ImportDeclaration>) -> StackValue<'static> {
        StackValue::ImportDeclaration(val)
    }
}

impl From<Box<ImportNamespace>> for StackValue<'static> {
    fn from(val: Box<ImportNamespace>) -> StackValue<'static> {
        StackValue::ImportNamespace(val)
    }
}

impl From<Box<ImportSpecifier>> for StackValue<'static> {
    fn from(val: Box<ImportSpecifier>) -> StackValue<'static> {
        StackValue::ImportSpecifier(val)
    }
}

impl From<Box<Label>> for StackValue<'static> {
    fn from(val: Box<Label>) -> StackValue<'static> {
        StackValue::Label(val)
    }
}

impl From<Box<LabeledStatement>> for StackValue<'static> {
    fn from(val: Box<LabeledStatement>) -> StackValue<'static> {
        StackValue::LabeledStatement(val)
    }
}

impl From<Box<LiteralBooleanExpression>> for StackValue<'static> {
    fn from(val: Box<LiteralBooleanExpression>) -> StackValue<'static> {
        StackValue::LiteralBooleanExpression(val)
    }
}

impl From<Box<LiteralNumericExpression>> for StackValue<'static> {
    fn from(val: Box<LiteralNumericExpression>) -> StackValue<'static> {
        StackValue::LiteralNumericExpression(val)
    }
}

impl From<Box<LiteralRegExpExpression>> for StackValue<'static> {
    fn from(val: Box<LiteralRegExpExpression>) -> StackValue<'static> {
        StackValue::LiteralRegExpExpression(val)
    }
}

impl From<Box<LiteralStringExpression>> for StackValue<'static> {
    fn from(val: Box<LiteralStringExpression>) -> StackValue<'static> {
        StackValue::LiteralStringExpression(val)
    }
}

impl From<Box<MemberAssignmentTarget>> for StackValue<'static> {
    fn from(val: Box<MemberAssignmentTarget>) -> StackValue<'static> {
        StackValue::MemberAssignmentTarget(val)
    }
}

impl From<Box<MemberExpression>> for StackValue<'static> {
    fn from(val: Box<MemberExpression>) -> StackValue<'static> {
        StackValue::MemberExpression(val)
    }
}

impl From<Box<Method>> for StackValue<'static> {
    fn from(val: Box<Method>) -> StackValue<'static> {
        StackValue::Method(val)
    }
}

impl From<Box<MethodDefinition>> for StackValue<'static> {
    fn from(val: Box<MethodDefinition>) -> StackValue<'static> {
        StackValue::MethodDefinition(val)
    }
}

impl From<Box<Module>> for StackValue<'static> {
    fn from(val: Box<Module>) -> StackValue<'static> {
        StackValue::Module(val)
    }
}

impl From<Box<ModuleItems>> for StackValue<'static> {
    fn from(val: Box<ModuleItems>) -> StackValue<'static> {
        StackValue::ModuleItems(val)
    }
}

impl From<Box<NamedObjectProperty>> for StackValue<'static> {
    fn from(val: Box<NamedObjectProperty>) -> StackValue<'static> {
        StackValue::NamedObjectProperty(val)
    }
}

impl From<Box<NewExpression>> for StackValue<'static> {
    fn from(val: Box<NewExpression>) -> StackValue<'static> {
        StackValue::NewExpression(val)
    }
}

impl From<Box<ObjectAssignmentTarget>> for StackValue<'static> {
    fn from(val: Box<ObjectAssignmentTarget>) -> StackValue<'static> {
        StackValue::ObjectAssignmentTarget(val)
    }
}

impl From<Box<ObjectBinding>> for StackValue<'static> {
    fn from(val: Box<ObjectBinding>) -> StackValue<'static> {
        StackValue::ObjectBinding(val)
    }
}

impl From<Box<ObjectExpression>> for StackValue<'static> {
    fn from(val: Box<ObjectExpression>) -> StackValue<'static> {
        StackValue::ObjectExpression(val)
    }
}

impl From<Box<ObjectProperty>> for StackValue<'static> {
    fn from(val: Box<ObjectProperty>) -> StackValue<'static> {
        StackValue::ObjectProperty(val)
    }
}

impl From<Box<Parameter>> for StackValue<'static> {
    fn from(val: Box<Parameter>) -> StackValue<'static> {
        StackValue::Parameter(val)
    }
}

impl From<Box<Program>> for StackValue<'static> {
    fn from(val: Box<Program>) -> StackValue<'static> {
        StackValue::Program(val)
    }
}

impl From<Box<PropertyName>> for StackValue<'static> {
    fn from(val: Box<PropertyName>) -> StackValue<'static> {
        StackValue::PropertyName(val)
    }
}

impl From<Box<ReturnStatement>> for StackValue<'static> {
    fn from(val: Box<ReturnStatement>) -> StackValue<'static> {
        StackValue::ReturnStatement(val)
    }
}

impl From<Box<Script>> for StackValue<'static> {
    fn from(val: Box<Script>) -> StackValue<'static> {
        StackValue::Script(val)
    }
}

impl From<Box<Setter>> for StackValue<'static> {
    fn from(val: Box<Setter>) -> StackValue<'static> {
        StackValue::Setter(val)
    }
}

impl From<Box<ShorthandProperty>> for StackValue<'static> {
    fn from(val: Box<ShorthandProperty>) -> StackValue<'static> {
        StackValue::ShorthandProperty(val)
    }
}

impl From<Box<SimpleAssignmentTarget>> for StackValue<'static> {
    fn from(val: Box<SimpleAssignmentTarget>) -> StackValue<'static> {
        StackValue::SimpleAssignmentTarget(val)
    }
}

impl From<Box<Statement>> for StackValue<'static> {
    fn from(val: Box<Statement>) -> StackValue<'static> {
        StackValue::Statement(val)
    }
}

impl From<Box<StaticMemberAssignmentTarget>> for StackValue<'static> {
    fn from(val: Box<StaticMemberAssignmentTarget>) -> StackValue<'static> {
        StackValue::StaticMemberAssignmentTarget(val)
    }
}

impl From<Box<StaticMemberExpression>> for StackValue<'static> {
    fn from(val: Box<StaticMemberExpression>) -> StackValue<'static> {
        StackValue::StaticMemberExpression(val)
    }
}

impl From<Box<StaticPropertyName>> for StackValue<'static> {
    fn from(val: Box<StaticPropertyName>) -> StackValue<'static> {
        StackValue::StaticPropertyName(val)
    }
}

impl From<Box<SwitchCase>> for StackValue<'static> {
    fn from(val: Box<SwitchCase>) -> StackValue<'static> {
        StackValue::SwitchCase(val)
    }
}

impl From<Box<SwitchDefault>> for StackValue<'static> {
    fn from(val: Box<SwitchDefault>) -> StackValue<'static> {
        StackValue::SwitchDefault(val)
    }
}

impl From<Box<SwitchStatement>> for StackValue<'static> {
    fn from(val: Box<SwitchStatement>) -> StackValue<'static> {
        StackValue::SwitchStatement(val)
    }
}

impl From<Box<SwitchStatementWithDefault>> for StackValue<'static> {
    fn from(val: Box<SwitchStatementWithDefault>) -> StackValue<'static> {
        StackValue::SwitchStatementWithDefault(val)
    }
}

impl From<Box<TemplateElement>> for StackValue<'static> {
    fn from(val: Box<TemplateElement>) -> StackValue<'static> {
        StackValue::TemplateElement(val)
    }
}

impl From<Box<TemplateExpression>> for StackValue<'static> {
    fn from(val: Box<TemplateExpression>) -> StackValue<'static> {
        StackValue::TemplateExpression(val)
    }
}

impl From<Box<TemplateExpressionElement>> for StackValue<'static> {
    fn from(val: Box<TemplateExpressionElement>) -> StackValue<'static> {
        StackValue::TemplateExpressionElement(val)
    }
}

impl From<Box<ThrowStatement>> for StackValue<'static> {
    fn from(val: Box<ThrowStatement>) -> StackValue<'static> {
        StackValue::ThrowStatement(val)
    }
}

impl<'a> From<Box<Token<'a>>> for StackValue<'a> {
    fn from(val: Box<Token<'a>>) -> StackValue<'a> {
        StackValue::Token(val)
    }
}

impl From<Box<TryCatchStatement>> for StackValue<'static> {
    fn from(val: Box<TryCatchStatement>) -> StackValue<'static> {
        StackValue::TryCatchStatement(val)
    }
}

impl From<Box<TryFinallyStatement>> for StackValue<'static> {
    fn from(val: Box<TryFinallyStatement>) -> StackValue<'static> {
        StackValue::TryFinallyStatement(val)
    }
}

impl From<Box<UnaryExpression>> for StackValue<'static> {
    fn from(val: Box<UnaryExpression>) -> StackValue<'static> {
        StackValue::UnaryExpression(val)
    }
}

impl From<Box<UnaryOperator>> for StackValue<'static> {
    fn from(val: Box<UnaryOperator>) -> StackValue<'static> {
        StackValue::UnaryOperator(val)
    }
}

impl From<Box<UpdateExpression>> for StackValue<'static> {
    fn from(val: Box<UpdateExpression>) -> StackValue<'static> {
        StackValue::UpdateExpression(val)
    }
}

impl From<Box<UpdateOperator>> for StackValue<'static> {
    fn from(val: Box<UpdateOperator>) -> StackValue<'static> {
        StackValue::UpdateOperator(val)
    }
}

impl From<Box<VariableDeclaration>> for StackValue<'static> {
    fn from(val: Box<VariableDeclaration>) -> StackValue<'static> {
        StackValue::VariableDeclaration(val)
    }
}

impl From<Box<VariableDeclarationKind>> for StackValue<'static> {
    fn from(val: Box<VariableDeclarationKind>) -> StackValue<'static> {
        StackValue::VariableDeclarationKind(val)
    }
}

impl From<Box<VariableDeclarationOrAssignmentTarget>> for StackValue<'static> {
    fn from(val: Box<VariableDeclarationOrAssignmentTarget>) -> StackValue<'static> {
        StackValue::VariableDeclarationOrAssignmentTarget(val)
    }
}

impl From<Box<VariableDeclarationOrExpression>> for StackValue<'static> {
    fn from(val: Box<VariableDeclarationOrExpression>) -> StackValue<'static> {
        StackValue::VariableDeclarationOrExpression(val)
    }
}

impl From<Box<VariableDeclarator>> for StackValue<'static> {
    fn from(val: Box<VariableDeclarator>) -> StackValue<'static> {
        StackValue::VariableDeclarator(val)
    }
}

impl From<Box<VariableReference>> for StackValue<'static> {
    fn from(val: Box<VariableReference>) -> StackValue<'static> {
        StackValue::VariableReference(val)
    }
}

impl From<Box<Vec<ArrayExpressionElement>>> for StackValue<'static> {
    fn from(val: Box<Vec<ArrayExpressionElement>>) -> StackValue<'static> {
        StackValue::VecArrayExpressionElement(val)
    }
}

impl From<Box<Vec<BindingProperty>>> for StackValue<'static> {
    fn from(val: Box<Vec<BindingProperty>>) -> StackValue<'static> {
        StackValue::VecBindingProperty(val)
    }
}

impl From<Box<Vec<ClassElement>>> for StackValue<'static> {
    fn from(val: Box<Vec<ClassElement>>) -> StackValue<'static> {
        StackValue::VecClassElement(val)
    }
}

impl From<Box<Vec<Option<Parameter>>>> for StackValue<'static> {
    fn from(val: Box<Vec<Option<Parameter>>>) -> StackValue<'static> {
        StackValue::VecParameter(val)
    }
}

impl From<Box<Vec<Statement>>> for StackValue<'static> {
    fn from(val: Box<Vec<Statement>>) -> StackValue<'static> {
        StackValue::VecStatement(val)
    }
}

impl From<Box<Vec<SwitchCase>>> for StackValue<'static> {
    fn from(val: Box<Vec<SwitchCase>>) -> StackValue<'static> {
        StackValue::VecSwitchCase(val)
    }
}

impl From<Box<Vec<VariableDeclarator>>> for StackValue<'static> {
    fn from(val: Box<Vec<VariableDeclarator>>) -> StackValue<'static> {
        StackValue::VecVariableDeclarator(val)
    }
}

impl From<Box<Void>> for StackValue<'static> {
    fn from(val: Box<Void>) -> StackValue<'static> {
        StackValue::Void(val)
    }
}

impl From<Box<WhileStatement>> for StackValue<'static> {
    fn from(val: Box<WhileStatement>) -> StackValue<'static> {
        StackValue::WhileStatement(val)
    }
}

impl From<Box<WithStatement>> for StackValue<'static> {
    fn from(val: Box<WithStatement>) -> StackValue<'static> {
        StackValue::WithStatement(val)
    }
}

impl From<Box<YieldExpression>> for StackValue<'static> {
    fn from(val: Box<YieldExpression>) -> StackValue<'static> {
        StackValue::YieldExpression(val)
    }
}

impl From<Box<YieldGeneratorExpression>> for StackValue<'static> {
    fn from(val: Box<YieldGeneratorExpression>) -> StackValue<'static> {
        StackValue::YieldGeneratorExpression(val)
    }
}
