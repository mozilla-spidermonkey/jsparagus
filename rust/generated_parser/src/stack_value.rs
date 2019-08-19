// WARNING: This file is auto-generated.

#![rustfmt::skip]

use crate::Token;
use ast::*;

#[derive(Debug, PartialEq)]
pub enum StackValue {
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
    Token(Box<Token>),
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
    VecStatement(Box<Vec<Statement>>),
    VecSwitchCase(Box<Vec<SwitchCase>>),
    VecVariableDeclarator(Box<Vec<VariableDeclarator>>),
    Void(Box<Void>),
    WhileStatement(Box<WhileStatement>),
    WithStatement(Box<WithStatement>),
    YieldExpression(Box<YieldExpression>),
    YieldGeneratorExpression(Box<YieldGeneratorExpression>),
}

impl StackValue {
    pub fn to_ast<T: StackValueItem>(self) -> Box<T> {
        T::to_ast(self)
    }
}

pub trait StackValueItem {
    fn to_ast(sv: StackValue) -> Box<Self>;
}

impl StackValueItem for Argument {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Argument(v) => v,
            _ => panic!("StackValue expected Argument, got {:?}", sv),
        }
    }
}

impl StackValueItem for Arguments {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Arguments(v) => v,
            _ => panic!("StackValue expected Arguments, got {:?}", sv),
        }
    }
}

impl StackValueItem for ArrayAssignmentTarget {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ArrayAssignmentTarget(v) => v,
            _ => panic!("StackValue expected ArrayAssignmentTarget, got {:?}", sv),
        }
    }
}

impl StackValueItem for ArrayBinding {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ArrayBinding(v) => v,
            _ => panic!("StackValue expected ArrayBinding, got {:?}", sv),
        }
    }
}

impl StackValueItem for ArrayExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ArrayExpression(v) => v,
            _ => panic!("StackValue expected ArrayExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for ArrayExpressionElement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ArrayExpressionElement(v) => v,
            _ => panic!("StackValue expected ArrayExpressionElement, got {:?}", sv),
        }
    }
}

impl StackValueItem for ArrowExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ArrowExpression(v) => v,
            _ => panic!("StackValue expected ArrowExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for ArrowExpressionBody {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ArrowExpressionBody(v) => v,
            _ => panic!("StackValue expected ArrowExpressionBody, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentExpression(v) => v,
            _ => panic!("StackValue expected AssignmentExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentTarget {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentTarget(v) => v,
            _ => panic!("StackValue expected AssignmentTarget, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentTargetIdentifier {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetIdentifier(v) => v,
            _ => panic!("StackValue expected AssignmentTargetIdentifier, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentTargetMaybeDefault {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetMaybeDefault(v) => v,
            _ => panic!("StackValue expected AssignmentTargetMaybeDefault, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentTargetPattern {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetPattern(v) => v,
            _ => panic!("StackValue expected AssignmentTargetPattern, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentTargetProperty {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetProperty(v) => v,
            _ => panic!("StackValue expected AssignmentTargetProperty, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentTargetPropertyIdentifier {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetPropertyIdentifier(v) => v,
            _ => panic!("StackValue expected AssignmentTargetPropertyIdentifier, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentTargetPropertyProperty {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetPropertyProperty(v) => v,
            _ => panic!("StackValue expected AssignmentTargetPropertyProperty, got {:?}", sv),
        }
    }
}

impl StackValueItem for AssignmentTargetWithDefault {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AssignmentTargetWithDefault(v) => v,
            _ => panic!("StackValue expected AssignmentTargetWithDefault, got {:?}", sv),
        }
    }
}

impl StackValueItem for AwaitExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::AwaitExpression(v) => v,
            _ => panic!("StackValue expected AwaitExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for BinaryExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BinaryExpression(v) => v,
            _ => panic!("StackValue expected BinaryExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for BinaryOperator {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BinaryOperator(v) => v,
            _ => panic!("StackValue expected BinaryOperator, got {:?}", sv),
        }
    }
}

impl StackValueItem for Binding {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Binding(v) => v,
            _ => panic!("StackValue expected Binding, got {:?}", sv),
        }
    }
}

impl StackValueItem for BindingIdentifier {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BindingIdentifier(v) => v,
            _ => panic!("StackValue expected BindingIdentifier, got {:?}", sv),
        }
    }
}

impl StackValueItem for BindingPattern {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BindingPattern(v) => v,
            _ => panic!("StackValue expected BindingPattern, got {:?}", sv),
        }
    }
}

impl StackValueItem for BindingProperty {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BindingProperty(v) => v,
            _ => panic!("StackValue expected BindingProperty, got {:?}", sv),
        }
    }
}

impl StackValueItem for BindingPropertyIdentifier {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BindingPropertyIdentifier(v) => v,
            _ => panic!("StackValue expected BindingPropertyIdentifier, got {:?}", sv),
        }
    }
}

impl StackValueItem for BindingPropertyProperty {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BindingPropertyProperty(v) => v,
            _ => panic!("StackValue expected BindingPropertyProperty, got {:?}", sv),
        }
    }
}

impl StackValueItem for BindingWithDefault {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BindingWithDefault(v) => v,
            _ => panic!("StackValue expected BindingWithDefault, got {:?}", sv),
        }
    }
}

impl StackValueItem for Block {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Block(v) => v,
            _ => panic!("StackValue expected Block, got {:?}", sv),
        }
    }
}

impl StackValueItem for BlockStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BlockStatement(v) => v,
            _ => panic!("StackValue expected BlockStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for BreakStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::BreakStatement(v) => v,
            _ => panic!("StackValue expected BreakStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for CallExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::CallExpression(v) => v,
            _ => panic!("StackValue expected CallExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for CatchClause {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::CatchClause(v) => v,
            _ => panic!("StackValue expected CatchClause, got {:?}", sv),
        }
    }
}

impl StackValueItem for ClassDeclaration {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ClassDeclaration(v) => v,
            _ => panic!("StackValue expected ClassDeclaration, got {:?}", sv),
        }
    }
}

impl StackValueItem for ClassElement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ClassElement(v) => v,
            _ => panic!("StackValue expected ClassElement, got {:?}", sv),
        }
    }
}

impl StackValueItem for ClassExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ClassExpression(v) => v,
            _ => panic!("StackValue expected ClassExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for CompoundAssignmentExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::CompoundAssignmentExpression(v) => v,
            _ => panic!("StackValue expected CompoundAssignmentExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for CompoundAssignmentOperator {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::CompoundAssignmentOperator(v) => v,
            _ => panic!("StackValue expected CompoundAssignmentOperator, got {:?}", sv),
        }
    }
}

impl StackValueItem for ComputedMemberAssignmentTarget {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ComputedMemberAssignmentTarget(v) => v,
            _ => panic!("StackValue expected ComputedMemberAssignmentTarget, got {:?}", sv),
        }
    }
}

impl StackValueItem for ComputedMemberExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ComputedMemberExpression(v) => v,
            _ => panic!("StackValue expected ComputedMemberExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for ComputedPropertyName {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ComputedPropertyName(v) => v,
            _ => panic!("StackValue expected ComputedPropertyName, got {:?}", sv),
        }
    }
}

impl StackValueItem for ConditionalExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ConditionalExpression(v) => v,
            _ => panic!("StackValue expected ConditionalExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for ContinueStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ContinueStatement(v) => v,
            _ => panic!("StackValue expected ContinueStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for DataProperty {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::DataProperty(v) => v,
            _ => panic!("StackValue expected DataProperty, got {:?}", sv),
        }
    }
}

impl StackValueItem for Directive {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Directive(v) => v,
            _ => panic!("StackValue expected Directive, got {:?}", sv),
        }
    }
}

impl StackValueItem for DoWhileStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::DoWhileStatement(v) => v,
            _ => panic!("StackValue expected DoWhileStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for Export {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Export(v) => v,
            _ => panic!("StackValue expected Export, got {:?}", sv),
        }
    }
}

impl StackValueItem for ExportAllFrom {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ExportAllFrom(v) => v,
            _ => panic!("StackValue expected ExportAllFrom, got {:?}", sv),
        }
    }
}

impl StackValueItem for ExportDeclaration {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ExportDeclaration(v) => v,
            _ => panic!("StackValue expected ExportDeclaration, got {:?}", sv),
        }
    }
}

impl StackValueItem for ExportDefault {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ExportDefault(v) => v,
            _ => panic!("StackValue expected ExportDefault, got {:?}", sv),
        }
    }
}

impl StackValueItem for ExportFrom {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ExportFrom(v) => v,
            _ => panic!("StackValue expected ExportFrom, got {:?}", sv),
        }
    }
}

impl StackValueItem for ExportFromSpecifier {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ExportFromSpecifier(v) => v,
            _ => panic!("StackValue expected ExportFromSpecifier, got {:?}", sv),
        }
    }
}

impl StackValueItem for ExportLocalSpecifier {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ExportLocalSpecifier(v) => v,
            _ => panic!("StackValue expected ExportLocalSpecifier, got {:?}", sv),
        }
    }
}

impl StackValueItem for ExportLocals {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ExportLocals(v) => v,
            _ => panic!("StackValue expected ExportLocals, got {:?}", sv),
        }
    }
}

impl StackValueItem for Expression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Expression(v) => v,
            _ => panic!("StackValue expected Expression, got {:?}", sv),
        }
    }
}

impl StackValueItem for ExpressionOrSuper {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ExpressionOrSuper(v) => v,
            _ => panic!("StackValue expected ExpressionOrSuper, got {:?}", sv),
        }
    }
}

impl StackValueItem for ForInStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ForInStatement(v) => v,
            _ => panic!("StackValue expected ForInStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for ForOfStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ForOfStatement(v) => v,
            _ => panic!("StackValue expected ForOfStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for ForStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ForStatement(v) => v,
            _ => panic!("StackValue expected ForStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for FormalParameters {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::FormalParameters(v) => v,
            _ => panic!("StackValue expected FormalParameters, got {:?}", sv),
        }
    }
}

impl StackValueItem for Function {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Function(v) => v,
            _ => panic!("StackValue expected Function, got {:?}", sv),
        }
    }
}

impl StackValueItem for FunctionBody {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::FunctionBody(v) => v,
            _ => panic!("StackValue expected FunctionBody, got {:?}", sv),
        }
    }
}

impl StackValueItem for Getter {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Getter(v) => v,
            _ => panic!("StackValue expected Getter, got {:?}", sv),
        }
    }
}

impl StackValueItem for Identifier {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Identifier(v) => v,
            _ => panic!("StackValue expected Identifier, got {:?}", sv),
        }
    }
}

impl StackValueItem for IdentifierExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::IdentifierExpression(v) => v,
            _ => panic!("StackValue expected IdentifierExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for IdentifierName {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::IdentifierName(v) => v,
            _ => panic!("StackValue expected IdentifierName, got {:?}", sv),
        }
    }
}

impl StackValueItem for IfStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::IfStatement(v) => v,
            _ => panic!("StackValue expected IfStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for Import {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Import(v) => v,
            _ => panic!("StackValue expected Import, got {:?}", sv),
        }
    }
}

impl StackValueItem for ImportDeclaration {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ImportDeclaration(v) => v,
            _ => panic!("StackValue expected ImportDeclaration, got {:?}", sv),
        }
    }
}

impl StackValueItem for ImportNamespace {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ImportNamespace(v) => v,
            _ => panic!("StackValue expected ImportNamespace, got {:?}", sv),
        }
    }
}

impl StackValueItem for ImportSpecifier {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ImportSpecifier(v) => v,
            _ => panic!("StackValue expected ImportSpecifier, got {:?}", sv),
        }
    }
}

impl StackValueItem for Label {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Label(v) => v,
            _ => panic!("StackValue expected Label, got {:?}", sv),
        }
    }
}

impl StackValueItem for LabeledStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::LabeledStatement(v) => v,
            _ => panic!("StackValue expected LabeledStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for LiteralBooleanExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::LiteralBooleanExpression(v) => v,
            _ => panic!("StackValue expected LiteralBooleanExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for LiteralNumericExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::LiteralNumericExpression(v) => v,
            _ => panic!("StackValue expected LiteralNumericExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for LiteralRegExpExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::LiteralRegExpExpression(v) => v,
            _ => panic!("StackValue expected LiteralRegExpExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for LiteralStringExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::LiteralStringExpression(v) => v,
            _ => panic!("StackValue expected LiteralStringExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for MemberAssignmentTarget {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::MemberAssignmentTarget(v) => v,
            _ => panic!("StackValue expected MemberAssignmentTarget, got {:?}", sv),
        }
    }
}

impl StackValueItem for MemberExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::MemberExpression(v) => v,
            _ => panic!("StackValue expected MemberExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for Method {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Method(v) => v,
            _ => panic!("StackValue expected Method, got {:?}", sv),
        }
    }
}

impl StackValueItem for MethodDefinition {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::MethodDefinition(v) => v,
            _ => panic!("StackValue expected MethodDefinition, got {:?}", sv),
        }
    }
}

impl StackValueItem for Module {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Module(v) => v,
            _ => panic!("StackValue expected Module, got {:?}", sv),
        }
    }
}

impl StackValueItem for ModuleItems {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ModuleItems(v) => v,
            _ => panic!("StackValue expected ModuleItems, got {:?}", sv),
        }
    }
}

impl StackValueItem for NamedObjectProperty {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::NamedObjectProperty(v) => v,
            _ => panic!("StackValue expected NamedObjectProperty, got {:?}", sv),
        }
    }
}

impl StackValueItem for NewExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::NewExpression(v) => v,
            _ => panic!("StackValue expected NewExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for ObjectAssignmentTarget {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ObjectAssignmentTarget(v) => v,
            _ => panic!("StackValue expected ObjectAssignmentTarget, got {:?}", sv),
        }
    }
}

impl StackValueItem for ObjectBinding {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ObjectBinding(v) => v,
            _ => panic!("StackValue expected ObjectBinding, got {:?}", sv),
        }
    }
}

impl StackValueItem for ObjectExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ObjectExpression(v) => v,
            _ => panic!("StackValue expected ObjectExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for ObjectProperty {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ObjectProperty(v) => v,
            _ => panic!("StackValue expected ObjectProperty, got {:?}", sv),
        }
    }
}

impl StackValueItem for Parameter {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Parameter(v) => v,
            _ => panic!("StackValue expected Parameter, got {:?}", sv),
        }
    }
}

impl StackValueItem for Program {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Program(v) => v,
            _ => panic!("StackValue expected Program, got {:?}", sv),
        }
    }
}

impl StackValueItem for PropertyName {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::PropertyName(v) => v,
            _ => panic!("StackValue expected PropertyName, got {:?}", sv),
        }
    }
}

impl StackValueItem for ReturnStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ReturnStatement(v) => v,
            _ => panic!("StackValue expected ReturnStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for Script {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Script(v) => v,
            _ => panic!("StackValue expected Script, got {:?}", sv),
        }
    }
}

impl StackValueItem for Setter {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Setter(v) => v,
            _ => panic!("StackValue expected Setter, got {:?}", sv),
        }
    }
}

impl StackValueItem for ShorthandProperty {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ShorthandProperty(v) => v,
            _ => panic!("StackValue expected ShorthandProperty, got {:?}", sv),
        }
    }
}

impl StackValueItem for SimpleAssignmentTarget {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::SimpleAssignmentTarget(v) => v,
            _ => panic!("StackValue expected SimpleAssignmentTarget, got {:?}", sv),
        }
    }
}

impl StackValueItem for Statement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Statement(v) => v,
            _ => panic!("StackValue expected Statement, got {:?}", sv),
        }
    }
}

impl StackValueItem for StaticMemberAssignmentTarget {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::StaticMemberAssignmentTarget(v) => v,
            _ => panic!("StackValue expected StaticMemberAssignmentTarget, got {:?}", sv),
        }
    }
}

impl StackValueItem for StaticMemberExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::StaticMemberExpression(v) => v,
            _ => panic!("StackValue expected StaticMemberExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for StaticPropertyName {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::StaticPropertyName(v) => v,
            _ => panic!("StackValue expected StaticPropertyName, got {:?}", sv),
        }
    }
}

impl StackValueItem for SwitchCase {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::SwitchCase(v) => v,
            _ => panic!("StackValue expected SwitchCase, got {:?}", sv),
        }
    }
}

impl StackValueItem for SwitchDefault {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::SwitchDefault(v) => v,
            _ => panic!("StackValue expected SwitchDefault, got {:?}", sv),
        }
    }
}

impl StackValueItem for SwitchStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::SwitchStatement(v) => v,
            _ => panic!("StackValue expected SwitchStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for SwitchStatementWithDefault {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::SwitchStatementWithDefault(v) => v,
            _ => panic!("StackValue expected SwitchStatementWithDefault, got {:?}", sv),
        }
    }
}

impl StackValueItem for TemplateElement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::TemplateElement(v) => v,
            _ => panic!("StackValue expected TemplateElement, got {:?}", sv),
        }
    }
}

impl StackValueItem for TemplateExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::TemplateExpression(v) => v,
            _ => panic!("StackValue expected TemplateExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for TemplateExpressionElement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::TemplateExpressionElement(v) => v,
            _ => panic!("StackValue expected TemplateExpressionElement, got {:?}", sv),
        }
    }
}

impl StackValueItem for ThrowStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::ThrowStatement(v) => v,
            _ => panic!("StackValue expected ThrowStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for Token {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Token(v) => v,
            _ => panic!("StackValue expected Token, got {:?}", sv),
        }
    }
}

impl StackValueItem for TryCatchStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::TryCatchStatement(v) => v,
            _ => panic!("StackValue expected TryCatchStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for TryFinallyStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::TryFinallyStatement(v) => v,
            _ => panic!("StackValue expected TryFinallyStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for UnaryExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::UnaryExpression(v) => v,
            _ => panic!("StackValue expected UnaryExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for UnaryOperator {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::UnaryOperator(v) => v,
            _ => panic!("StackValue expected UnaryOperator, got {:?}", sv),
        }
    }
}

impl StackValueItem for UpdateExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::UpdateExpression(v) => v,
            _ => panic!("StackValue expected UpdateExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for UpdateOperator {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::UpdateOperator(v) => v,
            _ => panic!("StackValue expected UpdateOperator, got {:?}", sv),
        }
    }
}

impl StackValueItem for VariableDeclaration {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VariableDeclaration(v) => v,
            _ => panic!("StackValue expected VariableDeclaration, got {:?}", sv),
        }
    }
}

impl StackValueItem for VariableDeclarationKind {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VariableDeclarationKind(v) => v,
            _ => panic!("StackValue expected VariableDeclarationKind, got {:?}", sv),
        }
    }
}

impl StackValueItem for VariableDeclarationOrAssignmentTarget {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VariableDeclarationOrAssignmentTarget(v) => v,
            _ => panic!("StackValue expected VariableDeclarationOrAssignmentTarget, got {:?}", sv),
        }
    }
}

impl StackValueItem for VariableDeclarationOrExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VariableDeclarationOrExpression(v) => v,
            _ => panic!("StackValue expected VariableDeclarationOrExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for VariableDeclarator {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VariableDeclarator(v) => v,
            _ => panic!("StackValue expected VariableDeclarator, got {:?}", sv),
        }
    }
}

impl StackValueItem for VariableReference {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VariableReference(v) => v,
            _ => panic!("StackValue expected VariableReference, got {:?}", sv),
        }
    }
}

impl StackValueItem for Vec<ArrayExpressionElement> {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VecArrayExpressionElement(v) => v,
            _ => panic!("StackValue expected Vec<ArrayExpressionElement>, got {:?}", sv),
        }
    }
}

impl StackValueItem for Vec<Statement> {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VecStatement(v) => v,
            _ => panic!("StackValue expected Vec<Statement>, got {:?}", sv),
        }
    }
}

impl StackValueItem for Vec<SwitchCase> {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VecSwitchCase(v) => v,
            _ => panic!("StackValue expected Vec<SwitchCase>, got {:?}", sv),
        }
    }
}

impl StackValueItem for Vec<VariableDeclarator> {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::VecVariableDeclarator(v) => v,
            _ => panic!("StackValue expected Vec<VariableDeclarator>, got {:?}", sv),
        }
    }
}

impl StackValueItem for Void {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::Void(v) => v,
            _ => panic!("StackValue expected Void, got {:?}", sv),
        }
    }
}

impl StackValueItem for WhileStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::WhileStatement(v) => v,
            _ => panic!("StackValue expected WhileStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for WithStatement {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::WithStatement(v) => v,
            _ => panic!("StackValue expected WithStatement, got {:?}", sv),
        }
    }
}

impl StackValueItem for YieldExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::YieldExpression(v) => v,
            _ => panic!("StackValue expected YieldExpression, got {:?}", sv),
        }
    }
}

impl StackValueItem for YieldGeneratorExpression {
    fn to_ast(sv: StackValue) -> Box<Self> {
        match sv {
            StackValue::YieldGeneratorExpression(v) => v,
            _ => panic!("StackValue expected YieldGeneratorExpression, got {:?}", sv),
        }
    }
}

impl From<Box<Argument>> for StackValue {
    fn from(val: Box<Argument>) -> StackValue {
        StackValue::Argument(val)
    }
}

impl From<Box<Arguments>> for StackValue {
    fn from(val: Box<Arguments>) -> StackValue {
        StackValue::Arguments(val)
    }
}

impl From<Box<ArrayAssignmentTarget>> for StackValue {
    fn from(val: Box<ArrayAssignmentTarget>) -> StackValue {
        StackValue::ArrayAssignmentTarget(val)
    }
}

impl From<Box<ArrayBinding>> for StackValue {
    fn from(val: Box<ArrayBinding>) -> StackValue {
        StackValue::ArrayBinding(val)
    }
}

impl From<Box<ArrayExpression>> for StackValue {
    fn from(val: Box<ArrayExpression>) -> StackValue {
        StackValue::ArrayExpression(val)
    }
}

impl From<Box<ArrayExpressionElement>> for StackValue {
    fn from(val: Box<ArrayExpressionElement>) -> StackValue {
        StackValue::ArrayExpressionElement(val)
    }
}

impl From<Box<ArrowExpression>> for StackValue {
    fn from(val: Box<ArrowExpression>) -> StackValue {
        StackValue::ArrowExpression(val)
    }
}

impl From<Box<ArrowExpressionBody>> for StackValue {
    fn from(val: Box<ArrowExpressionBody>) -> StackValue {
        StackValue::ArrowExpressionBody(val)
    }
}

impl From<Box<AssignmentExpression>> for StackValue {
    fn from(val: Box<AssignmentExpression>) -> StackValue {
        StackValue::AssignmentExpression(val)
    }
}

impl From<Box<AssignmentTarget>> for StackValue {
    fn from(val: Box<AssignmentTarget>) -> StackValue {
        StackValue::AssignmentTarget(val)
    }
}

impl From<Box<AssignmentTargetIdentifier>> for StackValue {
    fn from(val: Box<AssignmentTargetIdentifier>) -> StackValue {
        StackValue::AssignmentTargetIdentifier(val)
    }
}

impl From<Box<AssignmentTargetMaybeDefault>> for StackValue {
    fn from(val: Box<AssignmentTargetMaybeDefault>) -> StackValue {
        StackValue::AssignmentTargetMaybeDefault(val)
    }
}

impl From<Box<AssignmentTargetPattern>> for StackValue {
    fn from(val: Box<AssignmentTargetPattern>) -> StackValue {
        StackValue::AssignmentTargetPattern(val)
    }
}

impl From<Box<AssignmentTargetProperty>> for StackValue {
    fn from(val: Box<AssignmentTargetProperty>) -> StackValue {
        StackValue::AssignmentTargetProperty(val)
    }
}

impl From<Box<AssignmentTargetPropertyIdentifier>> for StackValue {
    fn from(val: Box<AssignmentTargetPropertyIdentifier>) -> StackValue {
        StackValue::AssignmentTargetPropertyIdentifier(val)
    }
}

impl From<Box<AssignmentTargetPropertyProperty>> for StackValue {
    fn from(val: Box<AssignmentTargetPropertyProperty>) -> StackValue {
        StackValue::AssignmentTargetPropertyProperty(val)
    }
}

impl From<Box<AssignmentTargetWithDefault>> for StackValue {
    fn from(val: Box<AssignmentTargetWithDefault>) -> StackValue {
        StackValue::AssignmentTargetWithDefault(val)
    }
}

impl From<Box<AwaitExpression>> for StackValue {
    fn from(val: Box<AwaitExpression>) -> StackValue {
        StackValue::AwaitExpression(val)
    }
}

impl From<Box<BinaryExpression>> for StackValue {
    fn from(val: Box<BinaryExpression>) -> StackValue {
        StackValue::BinaryExpression(val)
    }
}

impl From<Box<BinaryOperator>> for StackValue {
    fn from(val: Box<BinaryOperator>) -> StackValue {
        StackValue::BinaryOperator(val)
    }
}

impl From<Box<Binding>> for StackValue {
    fn from(val: Box<Binding>) -> StackValue {
        StackValue::Binding(val)
    }
}

impl From<Box<BindingIdentifier>> for StackValue {
    fn from(val: Box<BindingIdentifier>) -> StackValue {
        StackValue::BindingIdentifier(val)
    }
}

impl From<Box<BindingPattern>> for StackValue {
    fn from(val: Box<BindingPattern>) -> StackValue {
        StackValue::BindingPattern(val)
    }
}

impl From<Box<BindingProperty>> for StackValue {
    fn from(val: Box<BindingProperty>) -> StackValue {
        StackValue::BindingProperty(val)
    }
}

impl From<Box<BindingPropertyIdentifier>> for StackValue {
    fn from(val: Box<BindingPropertyIdentifier>) -> StackValue {
        StackValue::BindingPropertyIdentifier(val)
    }
}

impl From<Box<BindingPropertyProperty>> for StackValue {
    fn from(val: Box<BindingPropertyProperty>) -> StackValue {
        StackValue::BindingPropertyProperty(val)
    }
}

impl From<Box<BindingWithDefault>> for StackValue {
    fn from(val: Box<BindingWithDefault>) -> StackValue {
        StackValue::BindingWithDefault(val)
    }
}

impl From<Box<Block>> for StackValue {
    fn from(val: Box<Block>) -> StackValue {
        StackValue::Block(val)
    }
}

impl From<Box<BlockStatement>> for StackValue {
    fn from(val: Box<BlockStatement>) -> StackValue {
        StackValue::BlockStatement(val)
    }
}

impl From<Box<BreakStatement>> for StackValue {
    fn from(val: Box<BreakStatement>) -> StackValue {
        StackValue::BreakStatement(val)
    }
}

impl From<Box<CallExpression>> for StackValue {
    fn from(val: Box<CallExpression>) -> StackValue {
        StackValue::CallExpression(val)
    }
}

impl From<Box<CatchClause>> for StackValue {
    fn from(val: Box<CatchClause>) -> StackValue {
        StackValue::CatchClause(val)
    }
}

impl From<Box<ClassDeclaration>> for StackValue {
    fn from(val: Box<ClassDeclaration>) -> StackValue {
        StackValue::ClassDeclaration(val)
    }
}

impl From<Box<ClassElement>> for StackValue {
    fn from(val: Box<ClassElement>) -> StackValue {
        StackValue::ClassElement(val)
    }
}

impl From<Box<ClassExpression>> for StackValue {
    fn from(val: Box<ClassExpression>) -> StackValue {
        StackValue::ClassExpression(val)
    }
}

impl From<Box<CompoundAssignmentExpression>> for StackValue {
    fn from(val: Box<CompoundAssignmentExpression>) -> StackValue {
        StackValue::CompoundAssignmentExpression(val)
    }
}

impl From<Box<CompoundAssignmentOperator>> for StackValue {
    fn from(val: Box<CompoundAssignmentOperator>) -> StackValue {
        StackValue::CompoundAssignmentOperator(val)
    }
}

impl From<Box<ComputedMemberAssignmentTarget>> for StackValue {
    fn from(val: Box<ComputedMemberAssignmentTarget>) -> StackValue {
        StackValue::ComputedMemberAssignmentTarget(val)
    }
}

impl From<Box<ComputedMemberExpression>> for StackValue {
    fn from(val: Box<ComputedMemberExpression>) -> StackValue {
        StackValue::ComputedMemberExpression(val)
    }
}

impl From<Box<ComputedPropertyName>> for StackValue {
    fn from(val: Box<ComputedPropertyName>) -> StackValue {
        StackValue::ComputedPropertyName(val)
    }
}

impl From<Box<ConditionalExpression>> for StackValue {
    fn from(val: Box<ConditionalExpression>) -> StackValue {
        StackValue::ConditionalExpression(val)
    }
}

impl From<Box<ContinueStatement>> for StackValue {
    fn from(val: Box<ContinueStatement>) -> StackValue {
        StackValue::ContinueStatement(val)
    }
}

impl From<Box<DataProperty>> for StackValue {
    fn from(val: Box<DataProperty>) -> StackValue {
        StackValue::DataProperty(val)
    }
}

impl From<Box<Directive>> for StackValue {
    fn from(val: Box<Directive>) -> StackValue {
        StackValue::Directive(val)
    }
}

impl From<Box<DoWhileStatement>> for StackValue {
    fn from(val: Box<DoWhileStatement>) -> StackValue {
        StackValue::DoWhileStatement(val)
    }
}

impl From<Box<Export>> for StackValue {
    fn from(val: Box<Export>) -> StackValue {
        StackValue::Export(val)
    }
}

impl From<Box<ExportAllFrom>> for StackValue {
    fn from(val: Box<ExportAllFrom>) -> StackValue {
        StackValue::ExportAllFrom(val)
    }
}

impl From<Box<ExportDeclaration>> for StackValue {
    fn from(val: Box<ExportDeclaration>) -> StackValue {
        StackValue::ExportDeclaration(val)
    }
}

impl From<Box<ExportDefault>> for StackValue {
    fn from(val: Box<ExportDefault>) -> StackValue {
        StackValue::ExportDefault(val)
    }
}

impl From<Box<ExportFrom>> for StackValue {
    fn from(val: Box<ExportFrom>) -> StackValue {
        StackValue::ExportFrom(val)
    }
}

impl From<Box<ExportFromSpecifier>> for StackValue {
    fn from(val: Box<ExportFromSpecifier>) -> StackValue {
        StackValue::ExportFromSpecifier(val)
    }
}

impl From<Box<ExportLocalSpecifier>> for StackValue {
    fn from(val: Box<ExportLocalSpecifier>) -> StackValue {
        StackValue::ExportLocalSpecifier(val)
    }
}

impl From<Box<ExportLocals>> for StackValue {
    fn from(val: Box<ExportLocals>) -> StackValue {
        StackValue::ExportLocals(val)
    }
}

impl From<Box<Expression>> for StackValue {
    fn from(val: Box<Expression>) -> StackValue {
        StackValue::Expression(val)
    }
}

impl From<Box<ExpressionOrSuper>> for StackValue {
    fn from(val: Box<ExpressionOrSuper>) -> StackValue {
        StackValue::ExpressionOrSuper(val)
    }
}

impl From<Box<ForInStatement>> for StackValue {
    fn from(val: Box<ForInStatement>) -> StackValue {
        StackValue::ForInStatement(val)
    }
}

impl From<Box<ForOfStatement>> for StackValue {
    fn from(val: Box<ForOfStatement>) -> StackValue {
        StackValue::ForOfStatement(val)
    }
}

impl From<Box<ForStatement>> for StackValue {
    fn from(val: Box<ForStatement>) -> StackValue {
        StackValue::ForStatement(val)
    }
}

impl From<Box<FormalParameters>> for StackValue {
    fn from(val: Box<FormalParameters>) -> StackValue {
        StackValue::FormalParameters(val)
    }
}

impl From<Box<Function>> for StackValue {
    fn from(val: Box<Function>) -> StackValue {
        StackValue::Function(val)
    }
}

impl From<Box<FunctionBody>> for StackValue {
    fn from(val: Box<FunctionBody>) -> StackValue {
        StackValue::FunctionBody(val)
    }
}

impl From<Box<Getter>> for StackValue {
    fn from(val: Box<Getter>) -> StackValue {
        StackValue::Getter(val)
    }
}

impl From<Box<Identifier>> for StackValue {
    fn from(val: Box<Identifier>) -> StackValue {
        StackValue::Identifier(val)
    }
}

impl From<Box<IdentifierExpression>> for StackValue {
    fn from(val: Box<IdentifierExpression>) -> StackValue {
        StackValue::IdentifierExpression(val)
    }
}

impl From<Box<IdentifierName>> for StackValue {
    fn from(val: Box<IdentifierName>) -> StackValue {
        StackValue::IdentifierName(val)
    }
}

impl From<Box<IfStatement>> for StackValue {
    fn from(val: Box<IfStatement>) -> StackValue {
        StackValue::IfStatement(val)
    }
}

impl From<Box<Import>> for StackValue {
    fn from(val: Box<Import>) -> StackValue {
        StackValue::Import(val)
    }
}

impl From<Box<ImportDeclaration>> for StackValue {
    fn from(val: Box<ImportDeclaration>) -> StackValue {
        StackValue::ImportDeclaration(val)
    }
}

impl From<Box<ImportNamespace>> for StackValue {
    fn from(val: Box<ImportNamespace>) -> StackValue {
        StackValue::ImportNamespace(val)
    }
}

impl From<Box<ImportSpecifier>> for StackValue {
    fn from(val: Box<ImportSpecifier>) -> StackValue {
        StackValue::ImportSpecifier(val)
    }
}

impl From<Box<Label>> for StackValue {
    fn from(val: Box<Label>) -> StackValue {
        StackValue::Label(val)
    }
}

impl From<Box<LabeledStatement>> for StackValue {
    fn from(val: Box<LabeledStatement>) -> StackValue {
        StackValue::LabeledStatement(val)
    }
}

impl From<Box<LiteralBooleanExpression>> for StackValue {
    fn from(val: Box<LiteralBooleanExpression>) -> StackValue {
        StackValue::LiteralBooleanExpression(val)
    }
}

impl From<Box<LiteralNumericExpression>> for StackValue {
    fn from(val: Box<LiteralNumericExpression>) -> StackValue {
        StackValue::LiteralNumericExpression(val)
    }
}

impl From<Box<LiteralRegExpExpression>> for StackValue {
    fn from(val: Box<LiteralRegExpExpression>) -> StackValue {
        StackValue::LiteralRegExpExpression(val)
    }
}

impl From<Box<LiteralStringExpression>> for StackValue {
    fn from(val: Box<LiteralStringExpression>) -> StackValue {
        StackValue::LiteralStringExpression(val)
    }
}

impl From<Box<MemberAssignmentTarget>> for StackValue {
    fn from(val: Box<MemberAssignmentTarget>) -> StackValue {
        StackValue::MemberAssignmentTarget(val)
    }
}

impl From<Box<MemberExpression>> for StackValue {
    fn from(val: Box<MemberExpression>) -> StackValue {
        StackValue::MemberExpression(val)
    }
}

impl From<Box<Method>> for StackValue {
    fn from(val: Box<Method>) -> StackValue {
        StackValue::Method(val)
    }
}

impl From<Box<MethodDefinition>> for StackValue {
    fn from(val: Box<MethodDefinition>) -> StackValue {
        StackValue::MethodDefinition(val)
    }
}

impl From<Box<Module>> for StackValue {
    fn from(val: Box<Module>) -> StackValue {
        StackValue::Module(val)
    }
}

impl From<Box<ModuleItems>> for StackValue {
    fn from(val: Box<ModuleItems>) -> StackValue {
        StackValue::ModuleItems(val)
    }
}

impl From<Box<NamedObjectProperty>> for StackValue {
    fn from(val: Box<NamedObjectProperty>) -> StackValue {
        StackValue::NamedObjectProperty(val)
    }
}

impl From<Box<NewExpression>> for StackValue {
    fn from(val: Box<NewExpression>) -> StackValue {
        StackValue::NewExpression(val)
    }
}

impl From<Box<ObjectAssignmentTarget>> for StackValue {
    fn from(val: Box<ObjectAssignmentTarget>) -> StackValue {
        StackValue::ObjectAssignmentTarget(val)
    }
}

impl From<Box<ObjectBinding>> for StackValue {
    fn from(val: Box<ObjectBinding>) -> StackValue {
        StackValue::ObjectBinding(val)
    }
}

impl From<Box<ObjectExpression>> for StackValue {
    fn from(val: Box<ObjectExpression>) -> StackValue {
        StackValue::ObjectExpression(val)
    }
}

impl From<Box<ObjectProperty>> for StackValue {
    fn from(val: Box<ObjectProperty>) -> StackValue {
        StackValue::ObjectProperty(val)
    }
}

impl From<Box<Parameter>> for StackValue {
    fn from(val: Box<Parameter>) -> StackValue {
        StackValue::Parameter(val)
    }
}

impl From<Box<Program>> for StackValue {
    fn from(val: Box<Program>) -> StackValue {
        StackValue::Program(val)
    }
}

impl From<Box<PropertyName>> for StackValue {
    fn from(val: Box<PropertyName>) -> StackValue {
        StackValue::PropertyName(val)
    }
}

impl From<Box<ReturnStatement>> for StackValue {
    fn from(val: Box<ReturnStatement>) -> StackValue {
        StackValue::ReturnStatement(val)
    }
}

impl From<Box<Script>> for StackValue {
    fn from(val: Box<Script>) -> StackValue {
        StackValue::Script(val)
    }
}

impl From<Box<Setter>> for StackValue {
    fn from(val: Box<Setter>) -> StackValue {
        StackValue::Setter(val)
    }
}

impl From<Box<ShorthandProperty>> for StackValue {
    fn from(val: Box<ShorthandProperty>) -> StackValue {
        StackValue::ShorthandProperty(val)
    }
}

impl From<Box<SimpleAssignmentTarget>> for StackValue {
    fn from(val: Box<SimpleAssignmentTarget>) -> StackValue {
        StackValue::SimpleAssignmentTarget(val)
    }
}

impl From<Box<Statement>> for StackValue {
    fn from(val: Box<Statement>) -> StackValue {
        StackValue::Statement(val)
    }
}

impl From<Box<StaticMemberAssignmentTarget>> for StackValue {
    fn from(val: Box<StaticMemberAssignmentTarget>) -> StackValue {
        StackValue::StaticMemberAssignmentTarget(val)
    }
}

impl From<Box<StaticMemberExpression>> for StackValue {
    fn from(val: Box<StaticMemberExpression>) -> StackValue {
        StackValue::StaticMemberExpression(val)
    }
}

impl From<Box<StaticPropertyName>> for StackValue {
    fn from(val: Box<StaticPropertyName>) -> StackValue {
        StackValue::StaticPropertyName(val)
    }
}

impl From<Box<SwitchCase>> for StackValue {
    fn from(val: Box<SwitchCase>) -> StackValue {
        StackValue::SwitchCase(val)
    }
}

impl From<Box<SwitchDefault>> for StackValue {
    fn from(val: Box<SwitchDefault>) -> StackValue {
        StackValue::SwitchDefault(val)
    }
}

impl From<Box<SwitchStatement>> for StackValue {
    fn from(val: Box<SwitchStatement>) -> StackValue {
        StackValue::SwitchStatement(val)
    }
}

impl From<Box<SwitchStatementWithDefault>> for StackValue {
    fn from(val: Box<SwitchStatementWithDefault>) -> StackValue {
        StackValue::SwitchStatementWithDefault(val)
    }
}

impl From<Box<TemplateElement>> for StackValue {
    fn from(val: Box<TemplateElement>) -> StackValue {
        StackValue::TemplateElement(val)
    }
}

impl From<Box<TemplateExpression>> for StackValue {
    fn from(val: Box<TemplateExpression>) -> StackValue {
        StackValue::TemplateExpression(val)
    }
}

impl From<Box<TemplateExpressionElement>> for StackValue {
    fn from(val: Box<TemplateExpressionElement>) -> StackValue {
        StackValue::TemplateExpressionElement(val)
    }
}

impl From<Box<ThrowStatement>> for StackValue {
    fn from(val: Box<ThrowStatement>) -> StackValue {
        StackValue::ThrowStatement(val)
    }
}

impl From<Box<Token>> for StackValue {
    fn from(val: Box<Token>) -> StackValue {
        StackValue::Token(val)
    }
}

impl From<Box<TryCatchStatement>> for StackValue {
    fn from(val: Box<TryCatchStatement>) -> StackValue {
        StackValue::TryCatchStatement(val)
    }
}

impl From<Box<TryFinallyStatement>> for StackValue {
    fn from(val: Box<TryFinallyStatement>) -> StackValue {
        StackValue::TryFinallyStatement(val)
    }
}

impl From<Box<UnaryExpression>> for StackValue {
    fn from(val: Box<UnaryExpression>) -> StackValue {
        StackValue::UnaryExpression(val)
    }
}

impl From<Box<UnaryOperator>> for StackValue {
    fn from(val: Box<UnaryOperator>) -> StackValue {
        StackValue::UnaryOperator(val)
    }
}

impl From<Box<UpdateExpression>> for StackValue {
    fn from(val: Box<UpdateExpression>) -> StackValue {
        StackValue::UpdateExpression(val)
    }
}

impl From<Box<UpdateOperator>> for StackValue {
    fn from(val: Box<UpdateOperator>) -> StackValue {
        StackValue::UpdateOperator(val)
    }
}

impl From<Box<VariableDeclaration>> for StackValue {
    fn from(val: Box<VariableDeclaration>) -> StackValue {
        StackValue::VariableDeclaration(val)
    }
}

impl From<Box<VariableDeclarationKind>> for StackValue {
    fn from(val: Box<VariableDeclarationKind>) -> StackValue {
        StackValue::VariableDeclarationKind(val)
    }
}

impl From<Box<VariableDeclarationOrAssignmentTarget>> for StackValue {
    fn from(val: Box<VariableDeclarationOrAssignmentTarget>) -> StackValue {
        StackValue::VariableDeclarationOrAssignmentTarget(val)
    }
}

impl From<Box<VariableDeclarationOrExpression>> for StackValue {
    fn from(val: Box<VariableDeclarationOrExpression>) -> StackValue {
        StackValue::VariableDeclarationOrExpression(val)
    }
}

impl From<Box<VariableDeclarator>> for StackValue {
    fn from(val: Box<VariableDeclarator>) -> StackValue {
        StackValue::VariableDeclarator(val)
    }
}

impl From<Box<VariableReference>> for StackValue {
    fn from(val: Box<VariableReference>) -> StackValue {
        StackValue::VariableReference(val)
    }
}

impl From<Box<Vec<ArrayExpressionElement>>> for StackValue {
    fn from(val: Box<Vec<ArrayExpressionElement>>) -> StackValue {
        StackValue::VecArrayExpressionElement(val)
    }
}

impl From<Box<Vec<Statement>>> for StackValue {
    fn from(val: Box<Vec<Statement>>) -> StackValue {
        StackValue::VecStatement(val)
    }
}

impl From<Box<Vec<SwitchCase>>> for StackValue {
    fn from(val: Box<Vec<SwitchCase>>) -> StackValue {
        StackValue::VecSwitchCase(val)
    }
}

impl From<Box<Vec<VariableDeclarator>>> for StackValue {
    fn from(val: Box<Vec<VariableDeclarator>>) -> StackValue {
        StackValue::VecVariableDeclarator(val)
    }
}

impl From<Box<Void>> for StackValue {
    fn from(val: Box<Void>) -> StackValue {
        StackValue::Void(val)
    }
}

impl From<Box<WhileStatement>> for StackValue {
    fn from(val: Box<WhileStatement>) -> StackValue {
        StackValue::WhileStatement(val)
    }
}

impl From<Box<WithStatement>> for StackValue {
    fn from(val: Box<WithStatement>) -> StackValue {
        StackValue::WithStatement(val)
    }
}

impl From<Box<YieldExpression>> for StackValue {
    fn from(val: Box<YieldExpression>) -> StackValue {
        StackValue::YieldExpression(val)
    }
}

impl From<Box<YieldGeneratorExpression>> for StackValue {
    fn from(val: Box<YieldGeneratorExpression>) -> StackValue {
        StackValue::YieldGeneratorExpression(val)
    }
}

