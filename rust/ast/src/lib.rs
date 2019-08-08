// WARNING: This file is auto-generated.

#[derive(Debug, PartialEq)]
pub enum Void {}

#[derive(Debug, PartialEq)]
pub enum Argument {
    SpreadElement(SpreadElement),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Arguments {
    pub args: Vec<Argument>,
}

impl Arguments {
    pub fn new(args: Vec<Argument>) -> Self {
        Self { args }
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierName {
    pub value: String,
}

impl IdentifierName {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
pub struct Label {
    pub value: String,
}

impl Label {
    pub fn new(value: String) -> Self {
        Self { value }
    }
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
pub struct Function {
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters,
    pub body: FunctionBody,
}

impl Function {
    pub fn new(
        is_async: bool,
        is_generator: bool,
        params: FormalParameters,
        body: FunctionBody,
    ) -> Self {
        Self {
            is_async,
            is_generator,
            params,
            body,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Program {
    Module(Module),
    Script(Script),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    IterationStatement(IterationStatement),
    ClassDeclaration(ClassDeclaration),
    BlockStatement(BlockStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    DebuggerStatement(DebuggerStatement),
    EmptyStatement,
    ExpressionStatement(Box<Expression>),
    IfStatement(IfStatement),
    LabeledStatement(LabeledStatement),
    ReturnStatement(ReturnStatement),
    SwitchStatement(SwitchStatement),
    SwitchStatementWithDefault(SwitchStatementWithDefault),
    ThrowStatement(ThrowStatement),
    TryCatchStatement(TryCatchStatement),
    TryFinallyStatement(TryFinallyStatement),
    VariableDeclarationStatement(VariableDeclaration),
    WithStatement(WithStatement),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug, PartialEq)]
pub enum IterationStatement {
    DoWhileStatement(DoWhileStatement),
    ForInStatement(ForInStatement),
    ForOfStatement(ForOfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    MemberExpression(MemberExpression),
    ClassExpression(ClassExpression),
    LiteralBooleanExpression(LiteralBooleanExpression),
    LiteralInfinityExpression(LiteralInfinityExpression),
    LiteralNullExpression(LiteralNullExpression),
    LiteralNumericExpression(LiteralNumericExpression),
    LiteralRegExpExpression(LiteralRegExpExpression),
    LiteralStringExpression(LiteralStringExpression),
    ArrayExpression(ArrayExpression),
    ArrowExpression(ArrowExpression),
    AssignmentExpression(AssignmentExpression),
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
    CompoundAssignmentExpression(CompoundAssignmentExpression),
    ConditionalExpression(ConditionalExpression),
    FunctionExpression(FunctionExpression),
    IdentifierExpression(IdentifierExpression),
    NewExpression(NewExpression),
    NewTargetExpression(NewTargetExpression),
    ObjectExpression(ObjectExpression),
    UnaryExpression(UnaryExpression),
    TemplateExpression(TemplateExpression),
    ThisExpression(ThisExpression),
    UpdateExpression(UpdateExpression),
    YieldExpression(YieldExpression),
    YieldGeneratorExpression(YieldGeneratorExpression),
    AwaitExpression(AwaitExpression),
}

#[derive(Debug, PartialEq)]
pub enum MemberExpression {
    ComputedMemberExpression(ComputedMemberExpression),
    StaticMemberExpression(StaticMemberExpression),
}

#[derive(Debug, PartialEq)]
pub enum PropertyName {
    ComputedPropertyName(ComputedPropertyName),
    StaticPropertyName(StaticPropertyName),
}

#[derive(Debug, PartialEq)]
pub enum ObjectProperty {
    NamedObjectProperty(NamedObjectProperty),
    ShorthandProperty(ShorthandProperty),
}

#[derive(Debug, PartialEq)]
pub enum NamedObjectProperty {
    MethodDefinition(MethodDefinition),
    DataProperty(DataProperty),
}

#[derive(Debug, PartialEq)]
pub enum MethodDefinition {
    Method(Method),
    Getter(Getter),
    Setter(Setter),
}

#[derive(Debug, PartialEq)]
pub enum ImportDeclaration {
    Import(Import),
    ImportNamespace(ImportNamespace),
}

#[derive(Debug, PartialEq)]
pub enum ExportDeclaration {
    ExportAllFrom(ExportAllFrom),
    ExportFrom(ExportFrom),
    ExportLocals(ExportLocals),
    Export(Export),
    ExportDefault(ExportDefault),
}

#[derive(Debug, PartialEq)]
pub enum VariableReference {
    BindingIdentifier(BindingIdentifier),
    AssignmentTargetIdentifier(AssignmentTargetIdentifier),
}

#[derive(Debug, PartialEq)]
pub enum BindingPattern {
    ObjectBinding(ObjectBinding),
    ArrayBinding(ArrayBinding),
}

#[derive(Debug, PartialEq)]
pub enum Binding {
    BindingPattern(BindingPattern),
    BindingIdentifier(BindingIdentifier),
}

#[derive(Debug, PartialEq)]
pub enum SimpleAssignmentTarget {
    AssignmentTargetIdentifier(AssignmentTargetIdentifier),
    MemberAssignmentTarget(MemberAssignmentTarget),
}

#[derive(Debug, PartialEq)]
pub enum AssignmentTargetPattern {
    ArrayAssignmentTarget(ArrayAssignmentTarget),
    ObjectAssignmentTarget(ObjectAssignmentTarget),
}

#[derive(Debug, PartialEq)]
pub enum AssignmentTarget {
    AssignmentTargetPattern(AssignmentTargetPattern),
    SimpleAssignmentTarget(SimpleAssignmentTarget),
}

#[derive(Debug, PartialEq)]
pub enum Parameter {
    Binding(Binding),
    BindingWithDefault(BindingWithDefault),
}

#[derive(Debug, PartialEq)]
pub struct BindingWithDefault {
    pub binding: Binding,
    pub init: Box<Expression>,
}

impl BindingWithDefault {
    pub fn new(binding: Binding, init: Box<Expression>) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug, PartialEq)]
pub struct BindingIdentifier {
    pub name: Identifier,
}

impl BindingIdentifier {
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignmentTargetIdentifier {
    pub name: Identifier,
}

impl AssignmentTargetIdentifier {
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExpressionOrSuper {
    Expression(Box<Expression>),
    Super(Super),
}

#[derive(Debug, PartialEq)]
pub enum MemberAssignmentTarget {
    ComputedMemberAssignmentTarget(ComputedMemberAssignmentTarget),
    StaticMemberAssignmentTarget(StaticMemberAssignmentTarget),
}

#[derive(Debug, PartialEq)]
pub struct ComputedMemberAssignmentTarget {
    pub object: ExpressionOrSuper,
    pub expression: Box<Expression>,
}

impl ComputedMemberAssignmentTarget {
    pub fn new(object: ExpressionOrSuper, expression: Box<Expression>) -> Self {
        Self { object, expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct StaticMemberAssignmentTarget {
    pub object: ExpressionOrSuper,
    pub property: IdentifierName,
}

impl StaticMemberAssignmentTarget {
    pub fn new(object: ExpressionOrSuper, property: IdentifierName) -> Self {
        Self { object, property }
    }
}

#[derive(Debug, PartialEq)]
pub struct ArrayBinding {
    pub elements: Vec<Option<Parameter>>,
    pub rest: Option<Box<Binding>>,
}

impl ArrayBinding {
    pub fn new(elements: Vec<Option<Parameter>>, rest: Option<Box<Binding>>) -> Self {
        Self { elements, rest }
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectBinding {
    pub properties: Vec<BindingProperty>,
}

impl ObjectBinding {
    pub fn new(properties: Vec<BindingProperty>) -> Self {
        Self { properties }
    }
}

#[derive(Debug, PartialEq)]
pub enum BindingProperty {
    BindingPropertyIdentifier(BindingPropertyIdentifier),
    BindingPropertyProperty(BindingPropertyProperty),
}

#[derive(Debug, PartialEq)]
pub struct BindingPropertyIdentifier {
    pub binding: BindingIdentifier,
    pub init: Option<Box<Expression>>,
}

impl BindingPropertyIdentifier {
    pub fn new(binding: BindingIdentifier, init: Option<Box<Expression>>) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug, PartialEq)]
pub struct BindingPropertyProperty {
    pub name: PropertyName,
    pub binding: Parameter,
}

impl BindingPropertyProperty {
    pub fn new(name: PropertyName, binding: Parameter) -> Self {
        Self { name, binding }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignmentTargetWithDefault {
    pub binding: AssignmentTarget,
    pub init: Box<Expression>,
}

impl AssignmentTargetWithDefault {
    pub fn new(binding: AssignmentTarget, init: Box<Expression>) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug, PartialEq)]
pub enum AssignmentTargetMaybeDefault {
    AssignmentTarget(AssignmentTarget),
    AssignmentTargetWithDefault(AssignmentTargetWithDefault),
}

#[derive(Debug, PartialEq)]
pub struct ArrayAssignmentTarget {
    pub elements: Vec<Option<AssignmentTargetMaybeDefault>>,
    pub rest: Option<Box<AssignmentTarget>>,
}

impl ArrayAssignmentTarget {
    pub fn new(
        elements: Vec<Option<AssignmentTargetMaybeDefault>>,
        rest: Option<Box<AssignmentTarget>>,
    ) -> Self {
        Self { elements, rest }
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectAssignmentTarget {
    pub properties: Vec<AssignmentTargetProperty>,
}

impl ObjectAssignmentTarget {
    pub fn new(properties: Vec<AssignmentTargetProperty>) -> Self {
        Self { properties }
    }
}

#[derive(Debug, PartialEq)]
pub enum AssignmentTargetProperty {
    AssignmentTargetPropertyIdentifier(AssignmentTargetPropertyIdentifier),
    AssignmentTargetPropertyProperty(AssignmentTargetPropertyProperty),
}

#[derive(Debug, PartialEq)]
pub struct AssignmentTargetPropertyIdentifier {
    pub binding: AssignmentTargetIdentifier,
    pub init: Option<Box<Expression>>,
}

impl AssignmentTargetPropertyIdentifier {
    pub fn new(binding: AssignmentTargetIdentifier, init: Option<Box<Expression>>) -> Self {
        Self { binding, init }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignmentTargetPropertyProperty {
    pub name: PropertyName,
    pub binding: AssignmentTargetMaybeDefault,
}

impl AssignmentTargetPropertyProperty {
    pub fn new(name: PropertyName, binding: AssignmentTargetMaybeDefault) -> Self {
        Self { name, binding }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClassExpression {
    pub name: Option<BindingIdentifier>,
    pub super_: Option<Box<Expression>>,
    pub elements: Vec<ClassElement>,
}

impl ClassExpression {
    pub fn new(
        name: Option<BindingIdentifier>,
        super_: Option<Box<Expression>>,
        elements: Vec<ClassElement>,
    ) -> Self {
        Self {
            name,
            super_,
            elements,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClassDeclaration {
    pub name: BindingIdentifier,
    pub super_: Option<Box<Expression>>,
    pub elements: Vec<ClassElement>,
}

impl ClassDeclaration {
    pub fn new(
        name: BindingIdentifier,
        super_: Option<Box<Expression>>,
        elements: Vec<ClassElement>,
    ) -> Self {
        Self {
            name,
            super_,
            elements,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClassElement {
    pub property_name: PropertyName,
    pub is_static: bool,
    pub method: MethodDefinition,
}

impl ClassElement {
    pub fn new(property_name: PropertyName, is_static: bool, method: MethodDefinition) -> Self {
        Self {
            property_name,
            is_static,
            method,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ModuleItems {
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    Statement(Box<Statement>),
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub directives: Vec<Directive>,
    pub items: Vec<ModuleItems>,
}

impl Module {
    pub fn new(directives: Vec<Directive>, items: Vec<ModuleItems>) -> Self {
        Self { directives, items }
    }
}

#[derive(Debug, PartialEq)]
pub struct Import {
    pub module_specifier: String,
    pub default_binding: Option<BindingIdentifier>,
    pub named_imports: Vec<ImportSpecifier>,
}

impl Import {
    pub fn new(
        module_specifier: String,
        default_binding: Option<BindingIdentifier>,
        named_imports: Vec<ImportSpecifier>,
    ) -> Self {
        Self {
            module_specifier,
            default_binding,
            named_imports,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ImportNamespace {
    pub module_specifier: String,
    pub default_binding: Option<BindingIdentifier>,
    pub namespace_binding: BindingIdentifier,
}

impl ImportNamespace {
    pub fn new(
        module_specifier: String,
        default_binding: Option<BindingIdentifier>,
        namespace_binding: BindingIdentifier,
    ) -> Self {
        Self {
            module_specifier,
            default_binding,
            namespace_binding,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ImportSpecifier {
    pub name: Option<IdentifierName>,
    pub binding: BindingIdentifier,
}

impl ImportSpecifier {
    pub fn new(name: Option<IdentifierName>, binding: BindingIdentifier) -> Self {
        Self { name, binding }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExportAllFrom {
    pub module_specifier: String,
}

impl ExportAllFrom {
    pub fn new(module_specifier: String) -> Self {
        Self { module_specifier }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExportFrom {
    pub named_exports: Vec<ExportFromSpecifier>,
    pub module_specifier: String,
}

impl ExportFrom {
    pub fn new(named_exports: Vec<ExportFromSpecifier>, module_specifier: String) -> Self {
        Self {
            named_exports,
            module_specifier,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExportLocals {
    pub named_exports: Vec<ExportLocalSpecifier>,
}

impl ExportLocals {
    pub fn new(named_exports: Vec<ExportLocalSpecifier>) -> Self {
        Self { named_exports }
    }
}

#[derive(Debug, PartialEq)]
pub enum Export {
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, PartialEq)]
pub enum ExportDefault {
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct ExportFromSpecifier {
    pub name: IdentifierName,
    pub exported_name: Option<IdentifierName>,
}

impl ExportFromSpecifier {
    pub fn new(name: IdentifierName, exported_name: Option<IdentifierName>) -> Self {
        Self {
            name,
            exported_name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExportLocalSpecifier {
    pub name: IdentifierExpression,
    pub exported_name: Option<IdentifierName>,
}

impl ExportLocalSpecifier {
    pub fn new(name: IdentifierExpression, exported_name: Option<IdentifierName>) -> Self {
        Self {
            name,
            exported_name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Method {
    pub name: PropertyName,
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters,
    pub body: FunctionBody,
}

impl Method {
    pub fn new(
        name: PropertyName,
        is_async: bool,
        is_generator: bool,
        params: FormalParameters,
        body: FunctionBody,
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

#[derive(Debug, PartialEq)]
pub struct Getter {
    pub property_name: PropertyName,
    pub body: FunctionBody,
}

impl Getter {
    pub fn new(property_name: PropertyName, body: FunctionBody) -> Self {
        Self {
            property_name,
            body,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Setter {
    pub property_name: PropertyName,
    pub param: Parameter,
    pub body: FunctionBody,
}

impl Setter {
    pub fn new(property_name: PropertyName, param: Parameter, body: FunctionBody) -> Self {
        Self {
            property_name,
            param,
            body,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DataProperty {
    pub property_name: PropertyName,
    pub expression: Box<Expression>,
}

impl DataProperty {
    pub fn new(property_name: PropertyName, expression: Box<Expression>) -> Self {
        Self {
            property_name,
            expression,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ShorthandProperty {
    pub name: IdentifierExpression,
}

impl ShorthandProperty {
    pub fn new(name: IdentifierExpression) -> Self {
        Self { name }
    }
}

#[derive(Debug, PartialEq)]
pub struct ComputedPropertyName {
    pub expression: Box<Expression>,
}

impl ComputedPropertyName {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct StaticPropertyName {
    pub value: String,
}

impl StaticPropertyName {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
pub struct LiteralBooleanExpression {
    pub value: bool,
}

impl LiteralBooleanExpression {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct LiteralInfinityExpression {}

impl LiteralInfinityExpression {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct LiteralNullExpression {}

impl LiteralNullExpression {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumericExpression {
    pub value: f64,
}

impl LiteralNumericExpression {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
pub struct LiteralRegExpExpression {
    pub pattern: String,
    pub global: bool,
    pub ignore_case: bool,
    pub multi_line: bool,
    pub sticky: bool,
    pub unicode: bool,
}

impl LiteralRegExpExpression {
    pub fn new(
        pattern: String,
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

#[derive(Debug, PartialEq)]
pub struct LiteralStringExpression {
    pub value: String,
}

impl LiteralStringExpression {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArrayExpressionElement {
    SpreadElement(SpreadElement),
    Expression(Box<Expression>),
    Elision,
}

#[derive(Debug, PartialEq)]
pub struct ArrayExpression {
    pub elements: Vec<ArrayExpressionElement>,
}

impl ArrayExpression {
    pub fn new(elements: Vec<ArrayExpressionElement>) -> Self {
        Self { elements }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArrowExpressionBody {
    FunctionBody(FunctionBody),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct ArrowExpression {
    pub is_async: bool,
    pub params: FormalParameters,
    pub body: ArrowExpressionBody,
}

impl ArrowExpression {
    pub fn new(is_async: bool, params: FormalParameters, body: ArrowExpressionBody) -> Self {
        Self {
            is_async,
            params,
            body,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression {
    pub binding: AssignmentTarget,
    pub expression: Box<Expression>,
}

impl AssignmentExpression {
    pub fn new(binding: AssignmentTarget, expression: Box<Expression>) -> Self {
        Self {
            binding,
            expression,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(operator: BinaryOperator, left: Box<Expression>, right: Box<Expression>) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub callee: ExpressionOrSuper,
    pub arguments: Arguments,
}

impl CallExpression {
    pub fn new(callee: ExpressionOrSuper, arguments: Arguments) -> Self {
        Self { callee, arguments }
    }
}

#[derive(Debug, PartialEq)]
pub struct CompoundAssignmentExpression {
    pub operator: CompoundAssignmentOperator,
    pub binding: SimpleAssignmentTarget,
    pub expression: Box<Expression>,
}

impl CompoundAssignmentExpression {
    pub fn new(
        operator: CompoundAssignmentOperator,
        binding: SimpleAssignmentTarget,
        expression: Box<Expression>,
    ) -> Self {
        Self {
            operator,
            binding,
            expression,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ComputedMemberExpression {
    pub object: ExpressionOrSuper,
    pub expression: Box<Expression>,
}

impl ComputedMemberExpression {
    pub fn new(object: ExpressionOrSuper, expression: Box<Expression>) -> Self {
        Self { object, expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConditionalExpression {
    pub test: Box<Expression>,
    pub consequent: Box<Expression>,
    pub alternate: Box<Expression>,
}

impl ConditionalExpression {
    pub fn new(
        test: Box<Expression>,
        consequent: Box<Expression>,
        alternate: Box<Expression>,
    ) -> Self {
        Self {
            test,
            consequent,
            alternate,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionExpression {
    pub name: Option<BindingIdentifier>,
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters,
    pub body: FunctionBody,
}

impl FunctionExpression {
    pub fn new(
        name: Option<BindingIdentifier>,
        is_async: bool,
        is_generator: bool,
        params: FormalParameters,
        body: FunctionBody,
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

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression {
    pub var: VariableReference,
}

impl IdentifierExpression {
    pub fn new(var: VariableReference) -> Self {
        Self { var }
    }
}

#[derive(Debug, PartialEq)]
pub struct NewExpression {
    pub callee: Box<Expression>,
    pub arguments: Arguments,
}

impl NewExpression {
    pub fn new(callee: Box<Expression>, arguments: Arguments) -> Self {
        Self { callee, arguments }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct NewTargetExpression {}

impl NewTargetExpression {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectProperty>,
}

impl ObjectExpression {
    pub fn new(properties: Vec<ObjectProperty>) -> Self {
        Self { properties }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: UnaryOperator, operand: Box<Expression>) -> Self {
        Self { operator, operand }
    }
}

#[derive(Debug, PartialEq)]
pub struct StaticMemberExpression {
    pub object: ExpressionOrSuper,
    pub property: IdentifierName,
}

impl StaticMemberExpression {
    pub fn new(object: ExpressionOrSuper, property: IdentifierName) -> Self {
        Self { object, property }
    }
}

#[derive(Debug, PartialEq)]
pub enum TemplateExpressionElement {
    Expression(Box<Expression>),
    TemplateElement(TemplateElement),
}

#[derive(Debug, PartialEq)]
pub struct TemplateExpression {
    pub tag: Option<Box<Expression>>,
    pub elements: Vec<TemplateExpressionElement>,
}

impl TemplateExpression {
    pub fn new(tag: Option<Box<Expression>>, elements: Vec<TemplateExpressionElement>) -> Self {
        Self { tag, elements }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ThisExpression {}

impl ThisExpression {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq)]
pub struct UpdateExpression {
    pub is_prefix: bool,
    pub operator: UpdateOperator,
    pub operand: SimpleAssignmentTarget,
}

impl UpdateExpression {
    pub fn new(is_prefix: bool, operator: UpdateOperator, operand: SimpleAssignmentTarget) -> Self {
        Self {
            is_prefix,
            operator,
            operand,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct YieldExpression {
    pub expression: Option<Box<Expression>>,
}

impl YieldExpression {
    pub fn new(expression: Option<Box<Expression>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct YieldGeneratorExpression {
    pub expression: Box<Expression>,
}

impl YieldGeneratorExpression {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct AwaitExpression {
    pub expression: Box<Expression>,
}

impl AwaitExpression {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub block: Block,
}

impl BlockStatement {
    pub fn new(block: Block) -> Self {
        Self { block }
    }
}

#[derive(Debug, PartialEq)]
pub struct BreakStatement {
    pub label: Option<Label>,
}

impl BreakStatement {
    pub fn new(label: Option<Label>) -> Self {
        Self { label }
    }
}

#[derive(Debug, PartialEq)]
pub struct ContinueStatement {
    pub label: Option<Label>,
}

impl ContinueStatement {
    pub fn new(label: Option<Label>) -> Self {
        Self { label }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct DebuggerStatement {}

impl DebuggerStatement {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq)]
pub struct DoWhileStatement {
    pub block: Box<Statement>,
    pub test: Box<Expression>,
}

impl DoWhileStatement {
    pub fn new(block: Box<Statement>, test: Box<Expression>) -> Self {
        Self { block, test }
    }
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationOrAssignmentTarget {
    VariableDeclaration(VariableDeclaration),
    AssignmentTarget(AssignmentTarget),
}

#[derive(Debug, PartialEq)]
pub struct ForInStatement {
    pub left: VariableDeclarationOrAssignmentTarget,
    pub right: Box<Expression>,
    pub block: Box<Statement>,
}

impl ForInStatement {
    pub fn new(
        left: VariableDeclarationOrAssignmentTarget,
        right: Box<Expression>,
        block: Box<Statement>,
    ) -> Self {
        Self { left, right, block }
    }
}

#[derive(Debug, PartialEq)]
pub struct ForOfStatement {
    pub left: VariableDeclarationOrAssignmentTarget,
    pub right: Box<Expression>,
    pub block: Box<Statement>,
}

impl ForOfStatement {
    pub fn new(
        left: VariableDeclarationOrAssignmentTarget,
        right: Box<Expression>,
        block: Box<Statement>,
    ) -> Self {
        Self { left, right, block }
    }
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationOrExpression {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct ForStatement {
    pub init: Option<VariableDeclarationOrExpression>,
    pub test: Option<Box<Expression>>,
    pub update: Option<Box<Expression>>,
    pub block: Box<Statement>,
}

impl ForStatement {
    pub fn new(
        init: Option<VariableDeclarationOrExpression>,
        test: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        block: Box<Statement>,
    ) -> Self {
        Self {
            init,
            test,
            update,
            block,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub test: Box<Expression>,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
}

impl IfStatement {
    pub fn new(
        test: Box<Expression>,
        consequent: Box<Statement>,
        alternate: Option<Box<Statement>>,
    ) -> Self {
        Self {
            test,
            consequent,
            alternate,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LabeledStatement {
    pub label: Label,
    pub body: Box<Statement>,
}

impl LabeledStatement {
    pub fn new(label: Label, body: Box<Statement>) -> Self {
        Self { label, body }
    }
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub expression: Option<Box<Expression>>,
}

impl ReturnStatement {
    pub fn new(expression: Option<Box<Expression>>) -> Self {
        Self { expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct SwitchStatement {
    pub discriminant: Box<Expression>,
    pub cases: Vec<SwitchCase>,
}

impl SwitchStatement {
    pub fn new(discriminant: Box<Expression>, cases: Vec<SwitchCase>) -> Self {
        Self {
            discriminant,
            cases,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SwitchStatementWithDefault {
    pub discriminant: Box<Expression>,
    pub pre_default_cases: Vec<SwitchCase>,
    pub default_case: SwitchDefault,
    pub post_default_cases: Vec<SwitchCase>,
}

impl SwitchStatementWithDefault {
    pub fn new(
        discriminant: Box<Expression>,
        pre_default_cases: Vec<SwitchCase>,
        default_case: SwitchDefault,
        post_default_cases: Vec<SwitchCase>,
    ) -> Self {
        Self {
            discriminant,
            pre_default_cases,
            default_case,
            post_default_cases,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ThrowStatement {
    pub expression: Box<Expression>,
}

impl ThrowStatement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct TryCatchStatement {
    pub body: Block,
    pub catch_clause: CatchClause,
}

impl TryCatchStatement {
    pub fn new(body: Block, catch_clause: CatchClause) -> Self {
        Self { body, catch_clause }
    }
}

#[derive(Debug, PartialEq)]
pub struct TryFinallyStatement {
    pub body: Block,
    pub catch_clause: Option<CatchClause>,
    pub finalizer: Block,
}

impl TryFinallyStatement {
    pub fn new(body: Block, catch_clause: Option<CatchClause>, finalizer: Block) -> Self {
        Self {
            body,
            catch_clause,
            finalizer,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement {
    pub test: Box<Expression>,
    pub block: Box<Statement>,
}

impl WhileStatement {
    pub fn new(test: Box<Expression>, block: Box<Statement>) -> Self {
        Self { test, block }
    }
}

#[derive(Debug, PartialEq)]
pub struct WithStatement {
    pub object: Box<Expression>,
    pub body: Box<Statement>,
}

impl WithStatement {
    pub fn new(object: Box<Expression>, body: Box<Statement>) -> Self {
        Self { object, body }
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, PartialEq)]
pub struct CatchClause {
    pub binding: Binding,
    pub body: Block,
}

impl CatchClause {
    pub fn new(binding: Binding, body: Block) -> Self {
        Self { binding, body }
    }
}

#[derive(Debug, PartialEq)]
pub struct Directive {
    pub raw_value: String,
}

impl Directive {
    pub fn new(raw_value: String) -> Self {
        Self { raw_value }
    }
}

#[derive(Debug, PartialEq)]
pub struct FormalParameters {
    pub items: Vec<Parameter>,
    pub rest: Option<Binding>,
}

impl FormalParameters {
    pub fn new(items: Vec<Parameter>, rest: Option<Binding>) -> Self {
        Self { items, rest }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionBody {
    pub directives: Vec<Directive>,
    pub statements: Vec<Statement>,
}

impl FunctionBody {
    pub fn new(directives: Vec<Directive>, statements: Vec<Statement>) -> Self {
        Self {
            directives,
            statements,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: BindingIdentifier,
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters,
    pub body: FunctionBody,
}

impl FunctionDeclaration {
    pub fn new(
        name: BindingIdentifier,
        is_async: bool,
        is_generator: bool,
        params: FormalParameters,
        body: FunctionBody,
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

#[derive(Debug, PartialEq)]
pub struct Script {
    pub directives: Vec<Directive>,
    pub statements: Vec<Statement>,
}

impl Script {
    pub fn new(directives: Vec<Directive>, statements: Vec<Statement>) -> Self {
        Self {
            directives,
            statements,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SpreadElement {
    pub expression: Box<Expression>,
}

impl SpreadElement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Super {}

impl Super {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq)]
pub struct SwitchCase {
    pub test: Box<Expression>,
    pub consequent: Vec<Statement>,
}

impl SwitchCase {
    pub fn new(test: Box<Expression>, consequent: Vec<Statement>) -> Self {
        Self { test, consequent }
    }
}

#[derive(Debug, PartialEq)]
pub struct SwitchDefault {
    pub consequent: Vec<Statement>,
}

impl SwitchDefault {
    pub fn new(consequent: Vec<Statement>) -> Self {
        Self { consequent }
    }
}

#[derive(Debug, PartialEq)]
pub struct TemplateElement {
    pub raw_value: String,
}

impl TemplateElement {
    pub fn new(raw_value: String) -> Self {
        Self { raw_value }
    }
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub declarators: Vec<VariableDeclarator>,
}

impl VariableDeclaration {
    pub fn new(kind: VariableDeclarationKind, declarators: Vec<VariableDeclarator>) -> Self {
        Self { kind, declarators }
    }
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclarator {
    pub binding: Binding,
    pub init: Option<Box<Expression>>,
}

impl VariableDeclarator {
    pub fn new(binding: Binding, init: Option<Box<Expression>>) -> Self {
        Self { binding, init }
    }
}
