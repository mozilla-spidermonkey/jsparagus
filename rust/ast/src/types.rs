// WARNING: This file is auto-generated.

use crate::arena;

#[derive(Debug)]
pub enum Void {}

#[derive(Debug)]
pub enum Argument<'alloc> {
    SpreadElement(arena::Box<'alloc, Expression<'alloc>>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug)]
pub struct Arguments<'alloc> {
    pub args: arena::Vec<'alloc, Argument<'alloc>>,
}

impl<'alloc> Arguments<'alloc> {
    pub fn new(args: arena::Vec<'alloc, Argument<'alloc>>) -> Self {
        Self { args }
    }
}

#[derive(Debug)]
pub struct Identifier<'alloc> {
    pub value: arena::String<'alloc>,
}

impl<'alloc> Identifier<'alloc> {
    pub fn new(value: arena::String<'alloc>) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct IdentifierName<'alloc> {
    pub value: arena::String<'alloc>,
}

impl<'alloc> IdentifierName<'alloc> {
    pub fn new(value: arena::String<'alloc>) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct Label<'alloc> {
    pub value: arena::String<'alloc>,
}

impl<'alloc> Label<'alloc> {
    pub fn new(value: arena::String<'alloc>) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Void,
    Delete,
}

#[derive(Debug)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

#[derive(Debug)]
pub struct Function<'alloc> {
    pub name: Option<BindingIdentifier<'alloc>>,
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters<'alloc>,
    pub body: FunctionBody<'alloc>,
}

impl<'alloc> Function<'alloc> {
    pub fn new(
        name: Option<BindingIdentifier<'alloc>>,
        is_async: bool,
        is_generator: bool,
        params: FormalParameters<'alloc>,
        body: FunctionBody<'alloc>,
    ) -> Self {
        Self {
            name,
            is_async,
            is_generator,
            params,
            body,
        }
    }
}

#[derive(Debug)]
pub enum Program<'alloc> {
    Module(Module<'alloc>),
    Script(Script<'alloc>),
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum MemberExpression<'alloc> {
    ComputedMemberExpression(ComputedMemberExpression<'alloc>),
    StaticMemberExpression(StaticMemberExpression<'alloc>),
}

#[derive(Debug)]
pub enum PropertyName<'alloc> {
    ComputedPropertyName(ComputedPropertyName<'alloc>),
    StaticPropertyName(StaticPropertyName<'alloc>),
}

#[derive(Debug)]
pub enum ObjectProperty<'alloc> {
    NamedObjectProperty(NamedObjectProperty<'alloc>),
    ShorthandProperty(ShorthandProperty<'alloc>),
    SpreadProperty(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug)]
pub enum NamedObjectProperty<'alloc> {
    MethodDefinition(MethodDefinition<'alloc>),
    DataProperty(DataProperty<'alloc>),
}

#[derive(Debug)]
pub enum MethodDefinition<'alloc> {
    Method(Method<'alloc>),
    Getter(Getter<'alloc>),
    Setter(Setter<'alloc>),
}

#[derive(Debug)]
pub enum ImportDeclaration<'alloc> {
    Import(Import<'alloc>),
    ImportNamespace(ImportNamespace<'alloc>),
}

#[derive(Debug)]
pub enum ExportDeclaration<'alloc> {
    ExportAllFrom(ExportAllFrom<'alloc>),
    ExportFrom(ExportFrom<'alloc>),
    ExportLocals(ExportLocals<'alloc>),
    Export(Export<'alloc>),
    ExportDefault(ExportDefault<'alloc>),
}

#[derive(Debug)]
pub enum VariableReference<'alloc> {
    BindingIdentifier(BindingIdentifier<'alloc>),
    AssignmentTargetIdentifier(AssignmentTargetIdentifier<'alloc>),
}

#[derive(Debug)]
pub enum BindingPattern<'alloc> {
    ObjectBinding(ObjectBinding<'alloc>),
    ArrayBinding(ArrayBinding<'alloc>),
}

#[derive(Debug)]
pub enum Binding<'alloc> {
    BindingPattern(BindingPattern<'alloc>),
    BindingIdentifier(BindingIdentifier<'alloc>),
}

#[derive(Debug)]
pub enum SimpleAssignmentTarget<'alloc> {
    AssignmentTargetIdentifier(AssignmentTargetIdentifier<'alloc>),
    MemberAssignmentTarget(MemberAssignmentTarget<'alloc>),
}

#[derive(Debug)]
pub enum AssignmentTargetPattern<'alloc> {
    ArrayAssignmentTarget(ArrayAssignmentTarget<'alloc>),
    ObjectAssignmentTarget(ObjectAssignmentTarget<'alloc>),
}

#[derive(Debug)]
pub enum AssignmentTarget<'alloc> {
    AssignmentTargetPattern(AssignmentTargetPattern<'alloc>),
    SimpleAssignmentTarget(SimpleAssignmentTarget<'alloc>),
}

#[derive(Debug)]
pub enum Parameter<'alloc> {
    Binding(Binding<'alloc>),
    BindingWithDefault(BindingWithDefault<'alloc>),
}

#[derive(Debug)]
pub struct BindingWithDefault<'alloc> {
    pub binding: Binding<'alloc>,
    pub init: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> BindingWithDefault<'alloc> {
    pub fn new(binding: Binding<'alloc>, init: arena::Box<'alloc, Expression<'alloc>>) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug)]
pub struct BindingIdentifier<'alloc> {
    pub name: Identifier<'alloc>,
}

impl<'alloc> BindingIdentifier<'alloc> {
    pub fn new(name: Identifier<'alloc>) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct AssignmentTargetIdentifier<'alloc> {
    pub name: Identifier<'alloc>,
}

impl<'alloc> AssignmentTargetIdentifier<'alloc> {
    pub fn new(name: Identifier<'alloc>) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub enum ExpressionOrSuper<'alloc> {
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    Super,
}

#[derive(Debug)]
pub enum MemberAssignmentTarget<'alloc> {
    ComputedMemberAssignmentTarget(ComputedMemberAssignmentTarget<'alloc>),
    StaticMemberAssignmentTarget(StaticMemberAssignmentTarget<'alloc>),
}

#[derive(Debug)]
pub struct ComputedMemberAssignmentTarget<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> ComputedMemberAssignmentTarget<'alloc> {
    pub fn new(
        object: ExpressionOrSuper<'alloc>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self { object, expression }
    }
}

#[derive(Debug)]
pub struct StaticMemberAssignmentTarget<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub property: IdentifierName<'alloc>,
}

impl<'alloc> StaticMemberAssignmentTarget<'alloc> {
    pub fn new(object: ExpressionOrSuper<'alloc>, property: IdentifierName<'alloc>) -> Self {
        Self { object, property }
    }
}

#[derive(Debug)]
pub struct ArrayBinding<'alloc> {
    pub elements: arena::Vec<'alloc, Option<Parameter<'alloc>>>,
    pub rest: Option<arena::Box<'alloc, Binding<'alloc>>>,
}

impl<'alloc> ArrayBinding<'alloc> {
    pub fn new(
        elements: arena::Vec<'alloc, Option<Parameter<'alloc>>>,
        rest: Option<arena::Box<'alloc, Binding<'alloc>>>,
    ) -> Self {
        Self { elements, rest }
    }
}

#[derive(Debug)]
pub struct ObjectBinding<'alloc> {
    pub properties: arena::Vec<'alloc, BindingProperty<'alloc>>,
    pub rest: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
}

impl<'alloc> ObjectBinding<'alloc> {
    pub fn new(
        properties: arena::Vec<'alloc, BindingProperty<'alloc>>,
        rest: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
    ) -> Self {
        Self { properties, rest }
    }
}

#[derive(Debug)]
pub enum BindingProperty<'alloc> {
    BindingPropertyIdentifier(BindingPropertyIdentifier<'alloc>),
    BindingPropertyProperty(BindingPropertyProperty<'alloc>),
}

#[derive(Debug)]
pub struct BindingPropertyIdentifier<'alloc> {
    pub binding: BindingIdentifier<'alloc>,
    pub init: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

impl<'alloc> BindingPropertyIdentifier<'alloc> {
    pub fn new(
        binding: BindingIdentifier<'alloc>,
        init: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug)]
pub struct BindingPropertyProperty<'alloc> {
    pub name: PropertyName<'alloc>,
    pub binding: Parameter<'alloc>,
}

impl<'alloc> BindingPropertyProperty<'alloc> {
    pub fn new(name: PropertyName<'alloc>, binding: Parameter<'alloc>) -> Self {
        Self { name, binding }
    }
}

#[derive(Debug)]
pub struct AssignmentTargetWithDefault<'alloc> {
    pub binding: AssignmentTarget<'alloc>,
    pub init: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> AssignmentTargetWithDefault<'alloc> {
    pub fn new(
        binding: AssignmentTarget<'alloc>,
        init: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug)]
pub enum AssignmentTargetMaybeDefault<'alloc> {
    AssignmentTarget(AssignmentTarget<'alloc>),
    AssignmentTargetWithDefault(AssignmentTargetWithDefault<'alloc>),
}

#[derive(Debug)]
pub struct ArrayAssignmentTarget<'alloc> {
    pub elements: arena::Vec<'alloc, Option<AssignmentTargetMaybeDefault<'alloc>>>,
    pub rest: Option<arena::Box<'alloc, AssignmentTarget<'alloc>>>,
}

impl<'alloc> ArrayAssignmentTarget<'alloc> {
    pub fn new(
        elements: arena::Vec<'alloc, Option<AssignmentTargetMaybeDefault<'alloc>>>,
        rest: Option<arena::Box<'alloc, AssignmentTarget<'alloc>>>,
    ) -> Self {
        Self { elements, rest }
    }
}

#[derive(Debug)]
pub struct ObjectAssignmentTarget<'alloc> {
    pub properties: arena::Vec<'alloc, AssignmentTargetProperty<'alloc>>,
    pub rest: Option<arena::Box<'alloc, AssignmentTarget<'alloc>>>,
}

impl<'alloc> ObjectAssignmentTarget<'alloc> {
    pub fn new(
        properties: arena::Vec<'alloc, AssignmentTargetProperty<'alloc>>,
        rest: Option<arena::Box<'alloc, AssignmentTarget<'alloc>>>,
    ) -> Self {
        Self { properties, rest }
    }
}

#[derive(Debug)]
pub enum AssignmentTargetProperty<'alloc> {
    AssignmentTargetPropertyIdentifier(AssignmentTargetPropertyIdentifier<'alloc>),
    AssignmentTargetPropertyProperty(AssignmentTargetPropertyProperty<'alloc>),
}

#[derive(Debug)]
pub struct AssignmentTargetPropertyIdentifier<'alloc> {
    pub binding: AssignmentTargetIdentifier<'alloc>,
    pub init: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

impl<'alloc> AssignmentTargetPropertyIdentifier<'alloc> {
    pub fn new(
        binding: AssignmentTargetIdentifier<'alloc>,
        init: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug)]
pub struct AssignmentTargetPropertyProperty<'alloc> {
    pub name: PropertyName<'alloc>,
    pub binding: AssignmentTargetMaybeDefault<'alloc>,
}

impl<'alloc> AssignmentTargetPropertyProperty<'alloc> {
    pub fn new(name: PropertyName<'alloc>, binding: AssignmentTargetMaybeDefault<'alloc>) -> Self {
        Self { name, binding }
    }
}

#[derive(Debug)]
pub struct ClassExpression<'alloc> {
    pub name: Option<BindingIdentifier<'alloc>>,
    pub super_: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub elements: arena::Vec<'alloc, ClassElement<'alloc>>,
}

impl<'alloc> ClassExpression<'alloc> {
    pub fn new(
        name: Option<BindingIdentifier<'alloc>>,
        super_: Option<arena::Box<'alloc, Expression<'alloc>>>,
        elements: arena::Vec<'alloc, ClassElement<'alloc>>,
    ) -> Self {
        Self {
            name,
            super_,
            elements,
        }
    }
}

#[derive(Debug)]
pub struct ClassDeclaration<'alloc> {
    pub name: BindingIdentifier<'alloc>,
    pub super_: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub elements: arena::Vec<'alloc, ClassElement<'alloc>>,
}

impl<'alloc> ClassDeclaration<'alloc> {
    pub fn new(
        name: BindingIdentifier<'alloc>,
        super_: Option<arena::Box<'alloc, Expression<'alloc>>>,
        elements: arena::Vec<'alloc, ClassElement<'alloc>>,
    ) -> Self {
        Self {
            name,
            super_,
            elements,
        }
    }
}

#[derive(Debug)]
pub struct ClassElement<'alloc> {
    pub is_static: bool,
    pub method: MethodDefinition<'alloc>,
}

impl<'alloc> ClassElement<'alloc> {
    pub fn new(is_static: bool, method: MethodDefinition<'alloc>) -> Self {
        Self { is_static, method }
    }
}

#[derive(Debug)]
pub enum ModuleItems<'alloc> {
    ImportDeclaration(ImportDeclaration<'alloc>),
    ExportDeclaration(ExportDeclaration<'alloc>),
    Statement(arena::Box<'alloc, Statement<'alloc>>),
}

#[derive(Debug)]
pub struct Module<'alloc> {
    pub directives: arena::Vec<'alloc, Directive<'alloc>>,
    pub items: arena::Vec<'alloc, ModuleItems<'alloc>>,
}

impl<'alloc> Module<'alloc> {
    pub fn new(
        directives: arena::Vec<'alloc, Directive<'alloc>>,
        items: arena::Vec<'alloc, ModuleItems<'alloc>>,
    ) -> Self {
        Self { directives, items }
    }
}

#[derive(Debug)]
pub struct Import<'alloc> {
    pub module_specifier: arena::String<'alloc>,
    pub default_binding: Option<BindingIdentifier<'alloc>>,
    pub named_imports: arena::Vec<'alloc, ImportSpecifier<'alloc>>,
}

impl<'alloc> Import<'alloc> {
    pub fn new(
        module_specifier: arena::String<'alloc>,
        default_binding: Option<BindingIdentifier<'alloc>>,
        named_imports: arena::Vec<'alloc, ImportSpecifier<'alloc>>,
    ) -> Self {
        Self {
            module_specifier,
            default_binding,
            named_imports,
        }
    }
}

#[derive(Debug)]
pub struct ImportNamespace<'alloc> {
    pub module_specifier: arena::String<'alloc>,
    pub default_binding: Option<BindingIdentifier<'alloc>>,
    pub namespace_binding: BindingIdentifier<'alloc>,
}

impl<'alloc> ImportNamespace<'alloc> {
    pub fn new(
        module_specifier: arena::String<'alloc>,
        default_binding: Option<BindingIdentifier<'alloc>>,
        namespace_binding: BindingIdentifier<'alloc>,
    ) -> Self {
        Self {
            module_specifier,
            default_binding,
            namespace_binding,
        }
    }
}

#[derive(Debug)]
pub struct ImportSpecifier<'alloc> {
    pub name: Option<IdentifierName<'alloc>>,
    pub binding: BindingIdentifier<'alloc>,
}

impl<'alloc> ImportSpecifier<'alloc> {
    pub fn new(name: Option<IdentifierName<'alloc>>, binding: BindingIdentifier<'alloc>) -> Self {
        Self { name, binding }
    }
}

#[derive(Debug)]
pub struct ExportAllFrom<'alloc> {
    pub module_specifier: arena::String<'alloc>,
}

impl<'alloc> ExportAllFrom<'alloc> {
    pub fn new(module_specifier: arena::String<'alloc>) -> Self {
        Self { module_specifier }
    }
}

#[derive(Debug)]
pub struct ExportFrom<'alloc> {
    pub named_exports: arena::Vec<'alloc, ExportFromSpecifier<'alloc>>,
    pub module_specifier: arena::String<'alloc>,
}

impl<'alloc> ExportFrom<'alloc> {
    pub fn new(
        named_exports: arena::Vec<'alloc, ExportFromSpecifier<'alloc>>,
        module_specifier: arena::String<'alloc>,
    ) -> Self {
        Self {
            named_exports,
            module_specifier,
        }
    }
}

#[derive(Debug)]
pub struct ExportLocals<'alloc> {
    pub named_exports: arena::Vec<'alloc, ExportLocalSpecifier<'alloc>>,
}

impl<'alloc> ExportLocals<'alloc> {
    pub fn new(named_exports: arena::Vec<'alloc, ExportLocalSpecifier<'alloc>>) -> Self {
        Self { named_exports }
    }
}

#[derive(Debug)]
pub enum Export<'alloc> {
    FunctionDeclaration(Function<'alloc>),
    ClassDeclaration(ClassDeclaration<'alloc>),
    VariableDeclaration(VariableDeclaration<'alloc>),
}

#[derive(Debug)]
pub enum ExportDefault<'alloc> {
    FunctionDeclaration(Function<'alloc>),
    ClassDeclaration(ClassDeclaration<'alloc>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug)]
pub struct ExportFromSpecifier<'alloc> {
    pub name: IdentifierName<'alloc>,
    pub exported_name: Option<IdentifierName<'alloc>>,
}

impl<'alloc> ExportFromSpecifier<'alloc> {
    pub fn new(
        name: IdentifierName<'alloc>,
        exported_name: Option<IdentifierName<'alloc>>,
    ) -> Self {
        Self {
            name,
            exported_name,
        }
    }
}

#[derive(Debug)]
pub struct ExportLocalSpecifier<'alloc> {
    pub name: IdentifierExpression<'alloc>,
    pub exported_name: Option<IdentifierName<'alloc>>,
}

impl<'alloc> ExportLocalSpecifier<'alloc> {
    pub fn new(
        name: IdentifierExpression<'alloc>,
        exported_name: Option<IdentifierName<'alloc>>,
    ) -> Self {
        Self {
            name,
            exported_name,
        }
    }
}

#[derive(Debug)]
pub struct Method<'alloc> {
    pub name: PropertyName<'alloc>,
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters<'alloc>,
    pub body: FunctionBody<'alloc>,
}

impl<'alloc> Method<'alloc> {
    pub fn new(
        name: PropertyName<'alloc>,
        is_async: bool,
        is_generator: bool,
        params: FormalParameters<'alloc>,
        body: FunctionBody<'alloc>,
    ) -> Self {
        Self {
            name,
            is_async,
            is_generator,
            params,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Getter<'alloc> {
    pub property_name: PropertyName<'alloc>,
    pub body: FunctionBody<'alloc>,
}

impl<'alloc> Getter<'alloc> {
    pub fn new(property_name: PropertyName<'alloc>, body: FunctionBody<'alloc>) -> Self {
        Self {
            property_name,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Setter<'alloc> {
    pub property_name: PropertyName<'alloc>,
    pub param: Parameter<'alloc>,
    pub body: FunctionBody<'alloc>,
}

impl<'alloc> Setter<'alloc> {
    pub fn new(
        property_name: PropertyName<'alloc>,
        param: Parameter<'alloc>,
        body: FunctionBody<'alloc>,
    ) -> Self {
        Self {
            property_name,
            param,
            body,
        }
    }
}

#[derive(Debug)]
pub struct DataProperty<'alloc> {
    pub property_name: PropertyName<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> DataProperty<'alloc> {
    pub fn new(
        property_name: PropertyName<'alloc>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self {
            property_name,
            expression,
        }
    }
}

#[derive(Debug)]
pub struct ShorthandProperty<'alloc> {
    pub name: IdentifierExpression<'alloc>,
}

impl<'alloc> ShorthandProperty<'alloc> {
    pub fn new(name: IdentifierExpression<'alloc>) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct ComputedPropertyName<'alloc> {
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> ComputedPropertyName<'alloc> {
    pub fn new(expression: arena::Box<'alloc, Expression<'alloc>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct StaticPropertyName<'alloc> {
    pub value: arena::String<'alloc>,
}

impl<'alloc> StaticPropertyName<'alloc> {
    pub fn new(value: arena::String<'alloc>) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct LiteralBooleanExpression {
    pub value: bool,
}

impl LiteralBooleanExpression {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct LiteralNumericExpression {
    pub value: f64,
}

impl LiteralNumericExpression {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct LiteralRegExpExpression<'alloc> {
    pub pattern: arena::String<'alloc>,
    pub global: bool,
    pub ignore_case: bool,
    pub multi_line: bool,
    pub sticky: bool,
    pub unicode: bool,
}

impl<'alloc> LiteralRegExpExpression<'alloc> {
    pub fn new(
        pattern: arena::String<'alloc>,
        global: bool,
        ignore_case: bool,
        multi_line: bool,
        sticky: bool,
        unicode: bool,
    ) -> Self {
        Self {
            pattern,
            global,
            ignore_case,
            multi_line,
            sticky,
            unicode,
        }
    }
}

#[derive(Debug)]
pub struct LiteralStringExpression<'alloc> {
    pub value: arena::String<'alloc>,
}

impl<'alloc> LiteralStringExpression<'alloc> {
    pub fn new(value: arena::String<'alloc>) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum ArrayExpressionElement<'alloc> {
    SpreadElement(arena::Box<'alloc, Expression<'alloc>>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    Elision,
}

#[derive(Debug)]
pub struct ArrayExpression<'alloc> {
    pub elements: arena::Vec<'alloc, ArrayExpressionElement<'alloc>>,
}

impl<'alloc> ArrayExpression<'alloc> {
    pub fn new(elements: arena::Vec<'alloc, ArrayExpressionElement<'alloc>>) -> Self {
        Self { elements }
    }
}

#[derive(Debug)]
pub enum ArrowExpressionBody<'alloc> {
    FunctionBody(FunctionBody<'alloc>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug)]
pub struct ArrowExpression<'alloc> {
    pub is_async: bool,
    pub params: FormalParameters<'alloc>,
    pub body: ArrowExpressionBody<'alloc>,
}

impl<'alloc> ArrowExpression<'alloc> {
    pub fn new(
        is_async: bool,
        params: FormalParameters<'alloc>,
        body: ArrowExpressionBody<'alloc>,
    ) -> Self {
        Self {
            is_async,
            params,
            body,
        }
    }
}

#[derive(Debug)]
pub struct AssignmentExpression<'alloc> {
    pub binding: AssignmentTarget<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> AssignmentExpression<'alloc> {
    pub fn new(
        binding: AssignmentTarget<'alloc>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self {
            binding,
            expression,
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpression<'alloc> {
    pub operator: BinaryOperator,
    pub left: arena::Box<'alloc, Expression<'alloc>>,
    pub right: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> BinaryExpression<'alloc> {
    pub fn new(
        operator: BinaryOperator,
        left: arena::Box<'alloc, Expression<'alloc>>,
        right: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }
}

#[derive(Debug)]
pub struct CallExpression<'alloc> {
    pub callee: ExpressionOrSuper<'alloc>,
    pub arguments: Arguments<'alloc>,
}

impl<'alloc> CallExpression<'alloc> {
    pub fn new(callee: ExpressionOrSuper<'alloc>, arguments: Arguments<'alloc>) -> Self {
        Self { callee, arguments }
    }
}

#[derive(Debug)]
pub struct CompoundAssignmentExpression<'alloc> {
    pub operator: CompoundAssignmentOperator,
    pub binding: SimpleAssignmentTarget<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> CompoundAssignmentExpression<'alloc> {
    pub fn new(
        operator: CompoundAssignmentOperator,
        binding: SimpleAssignmentTarget<'alloc>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self {
            operator,
            binding,
            expression,
        }
    }
}

#[derive(Debug)]
pub struct ComputedMemberExpression<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> ComputedMemberExpression<'alloc> {
    pub fn new(
        object: ExpressionOrSuper<'alloc>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self { object, expression }
    }
}

#[derive(Debug)]
pub struct ConditionalExpression<'alloc> {
    pub test: arena::Box<'alloc, Expression<'alloc>>,
    pub consequent: arena::Box<'alloc, Expression<'alloc>>,
    pub alternate: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> ConditionalExpression<'alloc> {
    pub fn new(
        test: arena::Box<'alloc, Expression<'alloc>>,
        consequent: arena::Box<'alloc, Expression<'alloc>>,
        alternate: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self {
            test,
            consequent,
            alternate,
        }
    }
}

#[derive(Debug)]
pub struct IdentifierExpression<'alloc> {
    pub name: Identifier<'alloc>,
}

impl<'alloc> IdentifierExpression<'alloc> {
    pub fn new(name: Identifier<'alloc>) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct NewExpression<'alloc> {
    pub callee: arena::Box<'alloc, Expression<'alloc>>,
    pub arguments: Arguments<'alloc>,
}

impl<'alloc> NewExpression<'alloc> {
    pub fn new(
        callee: arena::Box<'alloc, Expression<'alloc>>,
        arguments: Arguments<'alloc>,
    ) -> Self {
        Self { callee, arguments }
    }
}

#[derive(Debug)]
pub struct ObjectExpression<'alloc> {
    pub properties: arena::Vec<'alloc, arena::Box<'alloc, ObjectProperty<'alloc>>>,
}

impl<'alloc> ObjectExpression<'alloc> {
    pub fn new(properties: arena::Vec<'alloc, arena::Box<'alloc, ObjectProperty<'alloc>>>) -> Self {
        Self { properties }
    }
}

#[derive(Debug)]
pub struct UnaryExpression<'alloc> {
    pub operator: UnaryOperator,
    pub operand: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> UnaryExpression<'alloc> {
    pub fn new(operator: UnaryOperator, operand: arena::Box<'alloc, Expression<'alloc>>) -> Self {
        Self { operator, operand }
    }
}

#[derive(Debug)]
pub struct StaticMemberExpression<'alloc> {
    pub object: ExpressionOrSuper<'alloc>,
    pub property: IdentifierName<'alloc>,
}

impl<'alloc> StaticMemberExpression<'alloc> {
    pub fn new(object: ExpressionOrSuper<'alloc>, property: IdentifierName<'alloc>) -> Self {
        Self { object, property }
    }
}

#[derive(Debug)]
pub enum TemplateExpressionElement<'alloc> {
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    TemplateElement(TemplateElement<'alloc>),
}

#[derive(Debug)]
pub struct TemplateExpression<'alloc> {
    pub tag: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub elements: arena::Vec<'alloc, TemplateExpressionElement<'alloc>>,
}

impl<'alloc> TemplateExpression<'alloc> {
    pub fn new(
        tag: Option<arena::Box<'alloc, Expression<'alloc>>>,
        elements: arena::Vec<'alloc, TemplateExpressionElement<'alloc>>,
    ) -> Self {
        Self { tag, elements }
    }
}

#[derive(Debug)]
pub struct UpdateExpression<'alloc> {
    pub is_prefix: bool,
    pub operator: UpdateOperator,
    pub operand: SimpleAssignmentTarget<'alloc>,
}

impl<'alloc> UpdateExpression<'alloc> {
    pub fn new(
        is_prefix: bool,
        operator: UpdateOperator,
        operand: SimpleAssignmentTarget<'alloc>,
    ) -> Self {
        Self {
            is_prefix,
            operator,
            operand,
        }
    }
}

#[derive(Debug)]
pub struct YieldExpression<'alloc> {
    pub expression: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

impl<'alloc> YieldExpression<'alloc> {
    pub fn new(expression: Option<arena::Box<'alloc, Expression<'alloc>>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct YieldGeneratorExpression<'alloc> {
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> YieldGeneratorExpression<'alloc> {
    pub fn new(expression: arena::Box<'alloc, Expression<'alloc>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct AwaitExpression<'alloc> {
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> AwaitExpression<'alloc> {
    pub fn new(expression: arena::Box<'alloc, Expression<'alloc>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct BlockStatement<'alloc> {
    pub block: Block<'alloc>,
}

impl<'alloc> BlockStatement<'alloc> {
    pub fn new(block: Block<'alloc>) -> Self {
        Self { block }
    }
}

#[derive(Debug)]
pub struct BreakStatement<'alloc> {
    pub label: Option<Label<'alloc>>,
}

impl<'alloc> BreakStatement<'alloc> {
    pub fn new(label: Option<Label<'alloc>>) -> Self {
        Self { label }
    }
}

#[derive(Debug)]
pub struct ContinueStatement<'alloc> {
    pub label: Option<Label<'alloc>>,
}

impl<'alloc> ContinueStatement<'alloc> {
    pub fn new(label: Option<Label<'alloc>>) -> Self {
        Self { label }
    }
}

#[derive(Debug)]
pub struct DoWhileStatement<'alloc> {
    pub block: arena::Box<'alloc, Statement<'alloc>>,
    pub test: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> DoWhileStatement<'alloc> {
    pub fn new(
        block: arena::Box<'alloc, Statement<'alloc>>,
        test: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Self {
        Self { block, test }
    }
}

#[derive(Debug)]
pub enum VariableDeclarationOrAssignmentTarget<'alloc> {
    VariableDeclaration(VariableDeclaration<'alloc>),
    AssignmentTarget(AssignmentTarget<'alloc>),
}

#[derive(Debug)]
pub struct ForInStatement<'alloc> {
    pub left: VariableDeclarationOrAssignmentTarget<'alloc>,
    pub right: arena::Box<'alloc, Expression<'alloc>>,
    pub block: arena::Box<'alloc, Statement<'alloc>>,
}

impl<'alloc> ForInStatement<'alloc> {
    pub fn new(
        left: VariableDeclarationOrAssignmentTarget<'alloc>,
        right: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> Self {
        Self { left, right, block }
    }
}

#[derive(Debug)]
pub struct ForOfStatement<'alloc> {
    pub left: VariableDeclarationOrAssignmentTarget<'alloc>,
    pub right: arena::Box<'alloc, Expression<'alloc>>,
    pub block: arena::Box<'alloc, Statement<'alloc>>,
}

impl<'alloc> ForOfStatement<'alloc> {
    pub fn new(
        left: VariableDeclarationOrAssignmentTarget<'alloc>,
        right: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> Self {
        Self { left, right, block }
    }
}

#[derive(Debug)]
pub enum VariableDeclarationOrExpression<'alloc> {
    VariableDeclaration(VariableDeclaration<'alloc>),
    Expression(arena::Box<'alloc, Expression<'alloc>>),
}

#[derive(Debug)]
pub struct ForStatement<'alloc> {
    pub init: Option<VariableDeclarationOrExpression<'alloc>>,
    pub test: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub update: Option<arena::Box<'alloc, Expression<'alloc>>>,
    pub block: arena::Box<'alloc, Statement<'alloc>>,
}

impl<'alloc> ForStatement<'alloc> {
    pub fn new(
        init: Option<VariableDeclarationOrExpression<'alloc>>,
        test: Option<arena::Box<'alloc, Expression<'alloc>>>,
        update: Option<arena::Box<'alloc, Expression<'alloc>>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> Self {
        Self {
            init,
            test,
            update,
            block,
        }
    }
}

#[derive(Debug)]
pub struct IfStatement<'alloc> {
    pub test: arena::Box<'alloc, Expression<'alloc>>,
    pub consequent: arena::Box<'alloc, Statement<'alloc>>,
    pub alternate: Option<arena::Box<'alloc, Statement<'alloc>>>,
}

impl<'alloc> IfStatement<'alloc> {
    pub fn new(
        test: arena::Box<'alloc, Expression<'alloc>>,
        consequent: arena::Box<'alloc, Statement<'alloc>>,
        alternate: Option<arena::Box<'alloc, Statement<'alloc>>>,
    ) -> Self {
        Self {
            test,
            consequent,
            alternate,
        }
    }
}

#[derive(Debug)]
pub struct LabeledStatement<'alloc> {
    pub label: Label<'alloc>,
    pub body: arena::Box<'alloc, Statement<'alloc>>,
}

impl<'alloc> LabeledStatement<'alloc> {
    pub fn new(label: Label<'alloc>, body: arena::Box<'alloc, Statement<'alloc>>) -> Self {
        Self { label, body }
    }
}

#[derive(Debug)]
pub struct ReturnStatement<'alloc> {
    pub expression: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

impl<'alloc> ReturnStatement<'alloc> {
    pub fn new(expression: Option<arena::Box<'alloc, Expression<'alloc>>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct SwitchStatement<'alloc> {
    pub discriminant: arena::Box<'alloc, Expression<'alloc>>,
    pub cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
}

impl<'alloc> SwitchStatement<'alloc> {
    pub fn new(
        discriminant: arena::Box<'alloc, Expression<'alloc>>,
        cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
    ) -> Self {
        Self {
            discriminant,
            cases,
        }
    }
}

#[derive(Debug)]
pub struct SwitchStatementWithDefault<'alloc> {
    pub discriminant: arena::Box<'alloc, Expression<'alloc>>,
    pub pre_default_cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
    pub default_case: SwitchDefault<'alloc>,
    pub post_default_cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
}

impl<'alloc> SwitchStatementWithDefault<'alloc> {
    pub fn new(
        discriminant: arena::Box<'alloc, Expression<'alloc>>,
        pre_default_cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
        default_case: SwitchDefault<'alloc>,
        post_default_cases: arena::Vec<'alloc, SwitchCase<'alloc>>,
    ) -> Self {
        Self {
            discriminant,
            pre_default_cases,
            default_case,
            post_default_cases,
        }
    }
}

#[derive(Debug)]
pub struct ThrowStatement<'alloc> {
    pub expression: arena::Box<'alloc, Expression<'alloc>>,
}

impl<'alloc> ThrowStatement<'alloc> {
    pub fn new(expression: arena::Box<'alloc, Expression<'alloc>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct TryCatchStatement<'alloc> {
    pub body: Block<'alloc>,
    pub catch_clause: CatchClause<'alloc>,
}

impl<'alloc> TryCatchStatement<'alloc> {
    pub fn new(body: Block<'alloc>, catch_clause: CatchClause<'alloc>) -> Self {
        Self { body, catch_clause }
    }
}

#[derive(Debug)]
pub struct TryFinallyStatement<'alloc> {
    pub body: Block<'alloc>,
    pub catch_clause: Option<CatchClause<'alloc>>,
    pub finalizer: Block<'alloc>,
}

impl<'alloc> TryFinallyStatement<'alloc> {
    pub fn new(
        body: Block<'alloc>,
        catch_clause: Option<CatchClause<'alloc>>,
        finalizer: Block<'alloc>,
    ) -> Self {
        Self {
            body,
            catch_clause,
            finalizer,
        }
    }
}

#[derive(Debug)]
pub struct WhileStatement<'alloc> {
    pub test: arena::Box<'alloc, Expression<'alloc>>,
    pub block: arena::Box<'alloc, Statement<'alloc>>,
}

impl<'alloc> WhileStatement<'alloc> {
    pub fn new(
        test: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> Self {
        Self { test, block }
    }
}

#[derive(Debug)]
pub struct WithStatement<'alloc> {
    pub object: arena::Box<'alloc, Expression<'alloc>>,
    pub body: arena::Box<'alloc, Statement<'alloc>>,
}

impl<'alloc> WithStatement<'alloc> {
    pub fn new(
        object: arena::Box<'alloc, Expression<'alloc>>,
        body: arena::Box<'alloc, Statement<'alloc>>,
    ) -> Self {
        Self { object, body }
    }
}

#[derive(Debug)]
pub struct Block<'alloc> {
    pub statements: arena::Vec<'alloc, Statement<'alloc>>,
    pub declarations: Option<arena::Vec<'alloc, arena::String<'alloc>>>,
}

impl<'alloc> Block<'alloc> {
    pub fn new(
        statements: arena::Vec<'alloc, Statement<'alloc>>,
        declarations: Option<arena::Vec<'alloc, arena::String<'alloc>>>,
    ) -> Self {
        Self {
            statements,
            declarations,
        }
    }
}

#[derive(Debug)]
pub struct CatchClause<'alloc> {
    pub binding: Binding<'alloc>,
    pub body: Block<'alloc>,
}

impl<'alloc> CatchClause<'alloc> {
    pub fn new(binding: Binding<'alloc>, body: Block<'alloc>) -> Self {
        Self { binding, body }
    }
}

#[derive(Debug)]
pub struct Directive<'alloc> {
    pub raw_value: arena::String<'alloc>,
}

impl<'alloc> Directive<'alloc> {
    pub fn new(raw_value: arena::String<'alloc>) -> Self {
        Self { raw_value }
    }
}

#[derive(Debug)]
pub struct FormalParameters<'alloc> {
    pub items: arena::Vec<'alloc, Parameter<'alloc>>,
    pub rest: Option<Binding<'alloc>>,
}

impl<'alloc> FormalParameters<'alloc> {
    pub fn new(
        items: arena::Vec<'alloc, Parameter<'alloc>>,
        rest: Option<Binding<'alloc>>,
    ) -> Self {
        Self { items, rest }
    }
}

#[derive(Debug)]
pub struct FunctionBody<'alloc> {
    pub directives: arena::Vec<'alloc, Directive<'alloc>>,
    pub statements: arena::Vec<'alloc, Statement<'alloc>>,
}

impl<'alloc> FunctionBody<'alloc> {
    pub fn new(
        directives: arena::Vec<'alloc, Directive<'alloc>>,
        statements: arena::Vec<'alloc, Statement<'alloc>>,
    ) -> Self {
        Self {
            directives,
            statements,
        }
    }
}

#[derive(Debug)]
pub struct Script<'alloc> {
    pub directives: arena::Vec<'alloc, Directive<'alloc>>,
    pub statements: arena::Vec<'alloc, Statement<'alloc>>,
}

impl<'alloc> Script<'alloc> {
    pub fn new(
        directives: arena::Vec<'alloc, Directive<'alloc>>,
        statements: arena::Vec<'alloc, Statement<'alloc>>,
    ) -> Self {
        Self {
            directives,
            statements,
        }
    }
}

#[derive(Debug)]
pub struct SwitchCase<'alloc> {
    pub test: arena::Box<'alloc, Expression<'alloc>>,
    pub consequent: arena::Vec<'alloc, Statement<'alloc>>,
}

impl<'alloc> SwitchCase<'alloc> {
    pub fn new(
        test: arena::Box<'alloc, Expression<'alloc>>,
        consequent: arena::Vec<'alloc, Statement<'alloc>>,
    ) -> Self {
        Self { test, consequent }
    }
}

#[derive(Debug)]
pub struct SwitchDefault<'alloc> {
    pub consequent: arena::Vec<'alloc, Statement<'alloc>>,
}

impl<'alloc> SwitchDefault<'alloc> {
    pub fn new(consequent: arena::Vec<'alloc, Statement<'alloc>>) -> Self {
        Self { consequent }
    }
}

#[derive(Debug)]
pub struct TemplateElement<'alloc> {
    pub raw_value: arena::String<'alloc>,
}

impl<'alloc> TemplateElement<'alloc> {
    pub fn new(raw_value: arena::String<'alloc>) -> Self {
        Self { raw_value }
    }
}

#[derive(Debug)]
pub struct VariableDeclaration<'alloc> {
    pub kind: VariableDeclarationKind,
    pub declarators: arena::Vec<'alloc, VariableDeclarator<'alloc>>,
}

impl<'alloc> VariableDeclaration<'alloc> {
    pub fn new(
        kind: VariableDeclarationKind,
        declarators: arena::Vec<'alloc, VariableDeclarator<'alloc>>,
    ) -> Self {
        Self { kind, declarators }
    }
}

#[derive(Debug)]
pub struct VariableDeclarator<'alloc> {
    pub binding: Binding<'alloc>,
    pub init: Option<arena::Box<'alloc, Expression<'alloc>>>,
}

impl<'alloc> VariableDeclarator<'alloc> {
    pub fn new(
        binding: Binding<'alloc>,
        init: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug)]
pub enum CoverParenthesized<'alloc> {
    Expression(arena::Box<'alloc, Expression<'alloc>>),
    Parameters(arena::Box<'alloc, FormalParameters<'alloc>>),
}
