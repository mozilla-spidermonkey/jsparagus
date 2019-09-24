// WARNING: This file is auto-generated.

use crate::arena;

#[derive(Debug, PartialEq)]
pub enum Void {}

#[derive(Debug, PartialEq)]
pub enum Argument<'alloc> {
    SpreadElement(arena::Box<'alloc, Expression<'alloc>>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug, PartialEq)]
pub struct Arguments<'alloc> {
    pub args: arena::Vec<'alloc, Argument<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct Identifier<'alloc> {
    pub value: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierName<'alloc> {
    pub value: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub struct Label<'alloc> {
    pub value: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, PartialEq)]
pub enum CompoundAssignmentOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    LeftShift,
    RightShift,
    RightShiftExt,
    Or,
    Xor,
    And,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Equals,
    NotEquals,
    StrictEquals,
    StrictNotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    In,
    Instanceof,
    LeftShift,
    RightShift,
    RightShiftExt,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Comma,
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Void,
    Delete,
}

#[derive(Debug, PartialEq)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq)]
pub struct Function<'alloc> {
    pub name: Option<BindingIdentifier<'alloc>>,
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters<'alloc>,
    pub body: FunctionBody<'alloc>,
}

#[derive(Debug, PartialEq)]
pub enum Program<'alloc> {
    Module(Module<'alloc>),
    Script(Script<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum Statement<'alloc> {
    BlockStatement(BlockStatement<'alloc>),
    BreakStatement(BreakStatement<'alloc>),
    ContinueStatement(ContinueStatement<'alloc>),
    DebuggerStatement,
    DoWhileStatement(DoWhileStatement<'alloc>),
    EmptyStatement,
    ExpressionStatement(arena::Box<'alloc, Expression<'alloc>>),
    ForInStatement(ForInStatement<'alloc>),
    ForOfStatement(ForOfStatement<'alloc>),
    ForStatement(ForStatement<'alloc>),
    IfStatement(IfStatement<'alloc>),
    LabeledStatement(LabeledStatement<'alloc>),
    ReturnStatement(ReturnStatement<'alloc>),
    SwitchStatement(SwitchStatement<'alloc>),
    SwitchStatementWithDefault(SwitchStatementWithDefault<'alloc>),
    ThrowStatement(ThrowStatement<'alloc>),
    TryCatchStatement(TryCatchStatement<'alloc>),
    TryFinallyStatement(TryFinallyStatement<'alloc>),
    VariableDeclarationStatement(VariableDeclaration<'alloc>),
    WhileStatement(WhileStatement<'alloc>),
    WithStatement(WithStatement<'alloc>),
    FunctionDeclaration(Function<'alloc>),
    ClassDeclaration(ClassDeclaration<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum Expression<'alloc> {
    MemberExpression(MemberExpression<'alloc>),
    ClassExpression(ClassExpression<'alloc>),
    LiteralBooleanExpression(LiteralBooleanExpression),
    LiteralInfinityExpression,
    LiteralNullExpression,
    LiteralNumericExpression(LiteralNumericExpression),
    LiteralRegExpExpression(LiteralRegExpExpression<'alloc>),
    LiteralStringExpression(LiteralStringExpression<'alloc>),
    ArrayExpression(ArrayExpression<'alloc>),
    ArrowExpression(ArrowExpression<'alloc>),
    AssignmentExpression(AssignmentExpression<'alloc>),
    BinaryExpression(BinaryExpression<'alloc>),
    CallExpression(CallExpression<'alloc>),
    CompoundAssignmentExpression(CompoundAssignmentExpression<'alloc>),
    ConditionalExpression(ConditionalExpression<'alloc>),
    FunctionExpression(Function<'alloc>),
    IdentifierExpression(IdentifierExpression<'alloc>),
    NewExpression(NewExpression<'alloc>),
    NewTargetExpression,
    ObjectExpression(ObjectExpression<'alloc>),
    UnaryExpression(UnaryExpression<'alloc>),
    TemplateExpression(TemplateExpression<'alloc>),
    ThisExpression,
    UpdateExpression(UpdateExpression<'alloc>),
    YieldExpression(YieldExpression<'alloc>),
    YieldGeneratorExpression(YieldGeneratorExpression<'alloc>),
    AwaitExpression(AwaitExpression<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum MemberExpression<'alloc> {
    ComputedMemberExpression(ComputedMemberExpression<'alloc>),
    StaticMemberExpression(StaticMemberExpression<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum PropertyName<'alloc> {
    ComputedPropertyName(ComputedPropertyName<'alloc>),
    StaticPropertyName(StaticPropertyName<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum ObjectProperty<'alloc> {
    NamedObjectProperty(NamedObjectProperty<'alloc>),
    ShorthandProperty(ShorthandProperty<'alloc>),
    SpreadProperty(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug, PartialEq)]
pub enum NamedObjectProperty<'alloc> {
    MethodDefinition(MethodDefinition<'alloc>),
    DataProperty(DataProperty<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum MethodDefinition<'alloc> {
    Method(Method<'alloc>),
    Getter(Getter<'alloc>),
    Setter(Setter<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum ImportDeclaration<'alloc> {
    Import(Import<'alloc>),
    ImportNamespace(ImportNamespace<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum ExportDeclaration<'alloc> {
    ExportAllFrom(ExportAllFrom<'alloc>),
    ExportFrom(ExportFrom<'alloc>),
    ExportLocals(ExportLocals<'alloc>),
    Export(Export<'alloc>),
    ExportDefault(ExportDefault<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum VariableReference<'alloc> {
    BindingIdentifier(BindingIdentifier<'alloc>),
    AssignmentTargetIdentifier(AssignmentTargetIdentifier<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum BindingPattern<'alloc> {
    ObjectBinding(ObjectBinding<'alloc>),
    ArrayBinding(ArrayBinding<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum Binding<'alloc> {
    BindingPattern(BindingPattern<'alloc>),
    BindingIdentifier(BindingIdentifier<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum SimpleAssignmentTarget<'alloc> {
    AssignmentTargetIdentifier(AssignmentTargetIdentifier<'alloc>),
    MemberAssignmentTarget(MemberAssignmentTarget<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum AssignmentTargetPattern<'alloc> {
    ArrayAssignmentTarget(ArrayAssignmentTarget<'alloc>),
    ObjectAssignmentTarget(ObjectAssignmentTarget<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum AssignmentTarget<'alloc> {
    AssignmentTargetPattern(AssignmentTargetPattern<'alloc>),
    SimpleAssignmentTarget(SimpleAssignmentTarget<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum Parameter<'alloc> {
    Binding(Binding<'alloc>),
    BindingWithDefault(BindingWithDefault<'alloc>),
}

#[derive(Debug, PartialEq)]
pub struct BindingWithDefault<'alloc> {
    pub binding: Binding<'alloc>,
    pub init: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct BindingIdentifier<'alloc> {
    pub name: Identifier<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct AssignmentTargetIdentifier<'alloc> {
    pub name: Identifier<'alloc>,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionOrSuper<'alloc> {
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    Super,
}

#[derive(Debug, PartialEq)]
pub enum MemberAssignmentTarget<'alloc> {
    ComputedMemberAssignmentTarget(ComputedMemberAssignmentTarget<'alloc>),
    StaticMemberAssignmentTarget(StaticMemberAssignmentTarget<'alloc>),
}

#[derive(Debug, PartialEq)]
pub struct ComputedMemberAssignmentTarget<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct StaticMemberAssignmentTarget<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub property: IdentifierName<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayBinding<'alloc> {
    pub elements: arena::Vec<'alloc, Option<Parameter<'alloc>>>,
    pub rest: Option<arena::Box<'alloc, Binding<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectBinding<'alloc> {
    pub properties: arena::Vec<'alloc, BindingProperty<'alloc>>,
    pub rest: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub enum BindingProperty<'alloc> {
    BindingPropertyIdentifier(BindingPropertyIdentifier<'alloc>),
    BindingPropertyProperty(BindingPropertyProperty<'alloc>),
}

#[derive(Debug, PartialEq)]
pub struct BindingPropertyIdentifier<'alloc> {
    pub binding: BindingIdentifier<'alloc>,
    pub init: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub struct BindingPropertyProperty<'alloc> {
    pub name: PropertyName<'alloc>,
    pub binding: Parameter<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct AssignmentTargetWithDefault<'alloc> {
    pub binding: AssignmentTarget<'alloc>,
    pub init: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentTargetMaybeDefault<'alloc> {
    AssignmentTarget(AssignmentTarget<'alloc>),
    AssignmentTargetWithDefault(AssignmentTargetWithDefault<'alloc>),
}

#[derive(Debug, PartialEq)]
pub struct ArrayAssignmentTarget<'alloc> {
    pub elements: arena::Vec<'alloc, Option<AssignmentTargetMaybeDefault<'alloc>>>,
    pub rest: Option<arena::Box<'alloc, AssignmentTarget<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectAssignmentTarget<'alloc> {
    pub properties: arena::Vec<'alloc, AssignmentTargetProperty<'alloc>>,
    pub rest: Option<arena::Box<'alloc, AssignmentTarget<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentTargetProperty<'alloc> {
    AssignmentTargetPropertyIdentifier(AssignmentTargetPropertyIdentifier<'alloc>),
    AssignmentTargetPropertyProperty(AssignmentTargetPropertyProperty<'alloc>),
}

#[derive(Debug, PartialEq)]
pub struct AssignmentTargetPropertyIdentifier<'alloc> {
    pub binding: AssignmentTargetIdentifier<'alloc>,
    pub init: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub struct AssignmentTargetPropertyProperty<'alloc> {
    pub name: PropertyName<'alloc>,
    pub binding: AssignmentTargetMaybeDefault<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct ClassExpression<'alloc> {
    pub name: Option<BindingIdentifier<'alloc>>,
    pub super_: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub elements: arena::Vec<'alloc, ClassElement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ClassDeclaration<'alloc> {
    pub name: BindingIdentifier<'alloc>,
    pub super_: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub elements: arena::Vec<'alloc, ClassElement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ClassElement<'alloc> {
    pub is_static: bool,
    pub method: MethodDefinition<'alloc>,
}

#[derive(Debug, PartialEq)]
pub enum ModuleItems<'alloc> {
    ImportDeclaration(ImportDeclaration<'alloc>),
    ExportDeclaration(ExportDeclaration<'alloc>),
    Statement(arena::Box<'alloc, Statement<'alloc>>),
}

#[derive(Debug, PartialEq)]
pub struct Module<'alloc> {
    pub directives: arena::Vec<'alloc, Directive<'alloc>>,
    pub items: arena::Vec<'alloc, ModuleItems<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct Import<'alloc> {
    pub module_specifier: &'alloc str,
    pub default_binding: Option<BindingIdentifier<'alloc>>,
    pub named_imports: arena::Vec<'alloc, ImportSpecifier<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ImportNamespace<'alloc> {
    pub module_specifier: &'alloc str,
    pub default_binding: Option<BindingIdentifier<'alloc>>,
    pub namespace_binding: BindingIdentifier<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct ImportSpecifier<'alloc> {
    pub name: Option<IdentifierName<'alloc>>,
    pub binding: BindingIdentifier<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct ExportAllFrom<'alloc> {
    pub module_specifier: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub struct ExportFrom<'alloc> {
    pub named_exports: arena::Vec<'alloc, ExportFromSpecifier<'alloc>>,
    pub module_specifier: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub struct ExportLocals<'alloc> {
    pub named_exports: arena::Vec<'alloc, ExportLocalSpecifier<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub enum Export<'alloc> {
    FunctionDeclaration(Function<'alloc>),
    ClassDeclaration(ClassDeclaration<'alloc>),
    VariableDeclaration(VariableDeclaration<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum ExportDefault<'alloc> {
    FunctionDeclaration(Function<'alloc>),
    ClassDeclaration(ClassDeclaration<'alloc>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug, PartialEq)]
pub struct ExportFromSpecifier<'alloc> {
    pub name: IdentifierName<'alloc>,
    pub exported_name: Option<IdentifierName<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ExportLocalSpecifier<'alloc> {
    pub name: IdentifierExpression<'alloc>,
    pub exported_name: Option<IdentifierName<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct Method<'alloc> {
    pub name: PropertyName<'alloc>,
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters<'alloc>,
    pub body: FunctionBody<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct Getter<'alloc> {
    pub property_name: PropertyName<'alloc>,
    pub body: FunctionBody<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct Setter<'alloc> {
    pub property_name: PropertyName<'alloc>,
    pub param: Parameter<'alloc>,
    pub body: FunctionBody<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct DataProperty<'alloc> {
    pub property_name: PropertyName<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ShorthandProperty<'alloc> {
    pub name: IdentifierExpression<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct ComputedPropertyName<'alloc> {
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct StaticPropertyName<'alloc> {
    pub value: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub struct LiteralBooleanExpression {
    pub value: bool,
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumericExpression {
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub struct LiteralRegExpExpression<'alloc> {
    pub pattern: &'alloc str,
    pub global: bool,
    pub ignore_case: bool,
    pub multi_line: bool,
    pub sticky: bool,
    pub unicode: bool,
}

#[derive(Debug, PartialEq)]
pub struct LiteralStringExpression<'alloc> {
    pub value: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub enum ArrayExpressionElement<'alloc> {
    SpreadElement(arena::Box<'alloc, Expression<'alloc>>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    Elision,
}

#[derive(Debug, PartialEq)]
pub struct ArrayExpression<'alloc> {
    pub elements: arena::Vec<'alloc, ArrayExpressionElement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub enum ArrowExpressionBody<'alloc> {
    FunctionBody(FunctionBody<'alloc>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug, PartialEq)]
pub struct ArrowExpression<'alloc> {
    pub is_async: bool,
    pub params: FormalParameters<'alloc>,
    pub body: ArrowExpressionBody<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression<'alloc> {
    pub binding: AssignmentTarget<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression<'alloc> {
    pub operator: BinaryOperator,
    pub left: arena::Box<'alloc, Expression<'alloc>>,
    pub right: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct CallExpression<'alloc> {
    pub callee: ExpressionOrSuper<'alloc>,
    pub arguments: Arguments<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct CompoundAssignmentExpression<'alloc> {
    pub operator: CompoundAssignmentOperator,
    pub binding: SimpleAssignmentTarget<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ComputedMemberExpression<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ConditionalExpression<'alloc> {
    pub test: arena::Box<'alloc, Expression<'alloc>>,
    pub consequent: arena::Box<'alloc, Expression<'alloc>>,
    pub alternate: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression<'alloc> {
    pub name: Identifier<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct NewExpression<'alloc> {
    pub callee: arena::Box<'alloc, Expression<'alloc>>,
    pub arguments: Arguments<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectExpression<'alloc> {
    pub properties: arena::Vec<'alloc, arena::Box<'alloc, ObjectProperty<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression<'alloc> {
    pub operator: UnaryOperator,
    pub operand: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct StaticMemberExpression<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub property: IdentifierName<'alloc>,
}

#[derive(Debug, PartialEq)]
pub enum TemplateExpressionElement<'alloc> {
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    TemplateElement(TemplateElement<'alloc>),
}

#[derive(Debug, PartialEq)]
pub struct TemplateExpression<'alloc> {
    pub tag: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub elements: arena::Vec<'alloc, TemplateExpressionElement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct UpdateExpression<'alloc> {
    pub is_prefix: bool,
    pub operator: UpdateOperator,
    pub operand: SimpleAssignmentTarget<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct YieldExpression<'alloc> {
    pub expression: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub struct YieldGeneratorExpression<'alloc> {
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct AwaitExpression<'alloc> {
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement<'alloc> {
    pub block: Block<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct BreakStatement<'alloc> {
    pub label: Option<Label<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ContinueStatement<'alloc> {
    pub label: Option<Label<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct DoWhileStatement<'alloc> {
    pub block: arena::Box<'alloc, Statement<'alloc>>,
    pub test: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationOrAssignmentTarget<'alloc> {
    VariableDeclaration(VariableDeclaration<'alloc>),
    AssignmentTarget(AssignmentTarget<'alloc>),
}

#[derive(Debug, PartialEq)]
pub struct ForInStatement<'alloc> {
    pub left: VariableDeclarationOrAssignmentTarget<'alloc>,
    pub right: arena::Box<'alloc, Expression<'alloc>>,
    pub block: arena::Box<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ForOfStatement<'alloc> {
    pub left: VariableDeclarationOrAssignmentTarget<'alloc>,
    pub right: arena::Box<'alloc, Expression<'alloc>>,
    pub block: arena::Box<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationOrExpression<'alloc> {
    VariableDeclaration(VariableDeclaration<'alloc>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug, PartialEq)]
pub struct ForStatement<'alloc> {
    pub init: Option<VariableDeclarationOrExpression<'alloc>>,
    pub test: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub update: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub block: arena::Box<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement<'alloc> {
    pub test: arena::Box<'alloc, Expression<'alloc>>,
    pub consequent: arena::Box<'alloc, Statement<'alloc>>,
    pub alternate: Option<arena::Box<'alloc, Statement<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub struct LabeledStatement<'alloc> {
    pub label: Label<'alloc>,
    pub body: arena::Box<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement<'alloc> {
    pub expression: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub struct SwitchStatement<'alloc> {
    pub discriminant: arena::Box<'alloc, Expression<'alloc>>,
    pub cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct SwitchStatementWithDefault<'alloc> {
    pub discriminant: arena::Box<'alloc, Expression<'alloc>>,
    pub pre_default_cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
    pub default_case: SwitchDefault<'alloc>,
    pub post_default_cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct ThrowStatement<'alloc> {
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct TryCatchStatement<'alloc> {
    pub body: Block<'alloc>,
    pub catch_clause: CatchClause<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct TryFinallyStatement<'alloc> {
    pub body: Block<'alloc>,
    pub catch_clause: Option<CatchClause<'alloc>>,
    pub finalizer: Block<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement<'alloc> {
    pub test: arena::Box<'alloc, Expression<'alloc>>,
    pub block: arena::Box<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct WithStatement<'alloc> {
    pub object: arena::Box<'alloc, Expression<'alloc>>,
    pub body: arena::Box<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct Block<'alloc> {
    pub statements: arena::Vec<'alloc, Statement<'alloc>>,
    pub declarations: Option<arena::Vec<'alloc, &'alloc str>>,
}

#[derive(Debug, PartialEq)]
pub struct CatchClause<'alloc> {
    pub binding: Binding<'alloc>,
    pub body: Block<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct Directive<'alloc> {
    pub raw_value: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub struct FormalParameters<'alloc> {
    pub items: arena::Vec<'alloc, Parameter<'alloc>>,
    pub rest: Option<Binding<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionBody<'alloc> {
    pub directives: arena::Vec<'alloc, Directive<'alloc>>,
    pub statements: arena::Vec<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct Script<'alloc> {
    pub directives: arena::Vec<'alloc, Directive<'alloc>>,
    pub statements: arena::Vec<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct SwitchCase<'alloc> {
    pub test: arena::Box<'alloc, Expression<'alloc>>,
    pub consequent: arena::Vec<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct SwitchDefault<'alloc> {
    pub consequent: arena::Vec<'alloc, Statement<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct TemplateElement<'alloc> {
    pub raw_value: &'alloc str,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration<'alloc> {
    pub kind: VariableDeclarationKind,
    pub declarators: arena::Vec<'alloc, VariableDeclarator<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclarator<'alloc> {
    pub binding: Binding<'alloc>,
    pub init: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

#[derive(Debug, PartialEq)]
pub enum CoverParenthesized<'alloc> {
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    Parameters(arena::Box<'alloc, FormalParameters<'alloc>>),
}
