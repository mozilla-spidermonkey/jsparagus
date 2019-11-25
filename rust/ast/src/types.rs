// WARNING: This file is auto-generated.

use crate::arena;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
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
    Coalesce,
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Void,
    Delete,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    BlockStatement {
        block: Block<'alloc>,
    },
    BreakStatement {
        label: Option<Label<'alloc>>,
    },
    ContinueStatement {
        label: Option<Label<'alloc>>,
    },
    DebuggerStatement,
    DoWhileStatement {
        block: arena::Box<'alloc, Statement<'alloc>>,
        test: arena::Box<'alloc, Expression<'alloc>>,
    },
    EmptyStatement,
    ExpressionStatement(arena::Box<'alloc, Expression<'alloc>>),
    ForInStatement {
        left: VariableDeclarationOrAssignmentTarget<'alloc>,
        right: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    },
    ForOfStatement {
        left: VariableDeclarationOrAssignmentTarget<'alloc>,
        right: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    },
    ForStatement {
        init: Option<VariableDeclarationOrExpression<'alloc>>,
        test: Option<arena::Box<'alloc, Expression<'alloc>>>,
        update: Option<arena::Box<'alloc, Expression<'alloc>>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    },
    IfStatement {
        test: arena::Box<'alloc, Expression<'alloc>>,
        consequent: arena::Box<'alloc, Statement<'alloc>>,
        alternate: Option<arena::Box<'alloc, Statement<'alloc>>>,
    },
    LabeledStatement {
        label: Label<'alloc>,
        body: arena::Box<'alloc, Statement<'alloc>>,
    },
    ReturnStatement {
        expression: Option<arena::Box<'alloc, Expression<'alloc>>>,
    },
    SwitchStatement {
        discriminant: arena::Box<'alloc, Expression<'alloc>>,
        cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
    },
    SwitchStatementWithDefault {
        discriminant: arena::Box<'alloc, Expression<'alloc>>,
        pre_default_cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
        default_case: SwitchDefault<'alloc>,
        post_default_cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
    },
    ThrowStatement {
        expression: arena::Box<'alloc, Expression<'alloc>>,
    },
    TryCatchStatement {
        body: Block<'alloc>,
        catch_clause: CatchClause<'alloc>,
    },
    TryFinallyStatement {
        body: Block<'alloc>,
        catch_clause: Option<CatchClause<'alloc>>,
        finalizer: Block<'alloc>,
    },
    WhileStatement {
        test: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    },
    WithStatement {
        object: arena::Box<'alloc, Expression<'alloc>>,
        body: arena::Box<'alloc, Statement<'alloc>>,
    },
    VariableDeclarationStatement(VariableDeclaration<'alloc>),
    FunctionDeclaration(Function<'alloc>),
    ClassDeclaration(ClassDeclaration<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum Expression<'alloc> {
    MemberExpression(MemberExpression<'alloc>),
    ClassExpression(ClassExpression<'alloc>),
    LiteralBooleanExpression {
        value: bool,
    },
    LiteralInfinityExpression,
    LiteralNullExpression,
    LiteralNumericExpression {
        value: f64,
    },
    LiteralRegExpExpression {
        pattern: &'alloc str,
        global: bool,
        ignore_case: bool,
        multi_line: bool,
        sticky: bool,
        unicode: bool,
    },
    LiteralStringExpression {
        value: &'alloc str,
    },
    ArrayExpression(ArrayExpression<'alloc>),
    ArrowExpression {
        is_async: bool,
        params: FormalParameters<'alloc>,
        body: ArrowExpressionBody<'alloc>,
    },
    AssignmentExpression {
        binding: AssignmentTarget<'alloc>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    },
    BinaryExpression {
        operator: BinaryOperator,
        left: arena::Box<'alloc, Expression<'alloc>>,
        right: arena::Box<'alloc, Expression<'alloc>>,
    },
    CallExpression {
        callee: ExpressionOrSuper<'alloc>,
        arguments: Arguments<'alloc>,
    },
    CompoundAssignmentExpression {
        operator: CompoundAssignmentOperator,
        binding: SimpleAssignmentTarget<'alloc>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    },
    ConditionalExpression {
        test: arena::Box<'alloc, Expression<'alloc>>,
        consequent: arena::Box<'alloc, Expression<'alloc>>,
        alternate: arena::Box<'alloc, Expression<'alloc>>,
    },
    FunctionExpression(Function<'alloc>),
    IdentifierExpression(IdentifierExpression<'alloc>),
    NewExpression {
        callee: arena::Box<'alloc, Expression<'alloc>>,
        arguments: Arguments<'alloc>,
    },
    NewTargetExpression,
    ObjectExpression(ObjectExpression<'alloc>),
    UnaryExpression {
        operator: UnaryOperator,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    },
    TemplateExpression(TemplateExpression<'alloc>),
    ThisExpression,
    UpdateExpression {
        is_prefix: bool,
        operator: UpdateOperator,
        operand: SimpleAssignmentTarget<'alloc>,
    },
    YieldExpression {
        expression: Option<arena::Box<'alloc, Expression<'alloc>>>,
    },
    YieldGeneratorExpression {
        expression: arena::Box<'alloc, Expression<'alloc>>,
    },
    AwaitExpression {
        expression: arena::Box<'alloc, Expression<'alloc>>,
    },
    ImportCallExpression {
        argument: arena::Box<'alloc, Expression<'alloc>>,
    },
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
pub struct ComputedMemberExpression<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression<'alloc> {
    pub name: Identifier<'alloc>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectExpression<'alloc> {
    pub properties: arena::Vec<'alloc, arena::Box<'alloc, ObjectProperty<'alloc>>>,
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
pub enum VariableDeclarationOrAssignmentTarget<'alloc> {
    VariableDeclaration(VariableDeclaration<'alloc>),
    AssignmentTarget(AssignmentTarget<'alloc>),
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationOrExpression<'alloc> {
    VariableDeclaration(VariableDeclaration<'alloc>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug, PartialEq)]
pub struct Block<'alloc> {
    pub statements: arena::Vec<'alloc, Statement<'alloc>>,
    pub declarations: Option<arena::Vec<'alloc, &'alloc str>>,
}

#[derive(Debug, PartialEq)]
pub struct CatchClause<'alloc> {
    pub binding: Option<arena::Box<'alloc, Binding<'alloc>>>,
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
