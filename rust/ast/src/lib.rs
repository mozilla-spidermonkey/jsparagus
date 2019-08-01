// Derived from https://github.com/shapesecurity/shift-spec/blob/es2017/spec.idl

/*
pub INFO: This specification is currently divided into the following sections:
  * supporting types
  * node classes
  * bindings
  * classes
  * modules
  * functions
  * object expressions
  * literals
  * other expressions
  * other statements
  * directives
  * other nodes
*/

// supporting types

// typedef (SpreadElement or Expression)[] Arguments;

pub enum Argument {
    SpreadElement(SpreadElement),
    Expression(Box<Expression>),
}

pub struct Arguments {
    pub args: Vec<Argument>,
}

impl Arguments {
    pub fn new(args: Vec<Argument>) -> Self {
        Self { args }
    }
}

// typedef DOMString string;

// typedef string Identifier;
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

// typedef string IdentifierName;
pub struct IdentifierName {
    pub value: String,
}

impl IdentifierName {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

// typedef string Label;
pub struct Label {
    pub value: String,
}

impl Label {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

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

pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Void,
    Delete,
}

pub enum UpdateOperator {
    Increment,
    Decrement,
}

// `FunctionExpression`, `FunctionDeclaration`, `GeneratorExpression`, `GeneratorDeclaration`, `AsyncFunctionExpression`, `AsyncFunctionDeclaration`
pub struct Function {
    // True for `AsyncFunctionExpression` and `AsyncFunctionDeclaration`, false otherwise.
    pub is_async: bool,
    // True for `GeneratorExpression` and `GeneratorDeclaration`, false otherwise.
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

// node classes

// interface Node {
//   [TypeIndicator] readonly attribute Type type;
// };

pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
    PropertyName(PropertyName),
    ObjectProperty(ObjectProperty),
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    VariableReference(VariableReference),
    BindingWithDefault(BindingWithDefault),
    MemberAssignmentTarget(MemberAssignmentTarget),
    ArrayBinding(ArrayBinding),
    ObjectBinding(ObjectBinding),
    BindingProperty(BindingProperty),
    AssignmentTargetWithDefault(AssignmentTargetWithDefault),
    ArrayAssignmentTarget(ArrayAssignmentTarget),
    ObjectAssignmentTarget(ObjectAssignmentTarget),
    AssignmentTargetProperty(AssignmentTargetProperty),
    ClassElement(ClassElement),
    ImportSpecifier(ImportSpecifier),
    ExportFromSpecifier(ExportFromSpecifier),
    ExportLocalSpecifier(ExportLocalSpecifier),
    Block(Block),
    CatchClause(CatchClause),
    Directive(Directive),
    FormalParameters(FormalParameters),
    FunctionBody(FunctionBody),
    SpreadElement(SpreadElement),
    Super(Super),
    SwitchCase(SwitchCase),
    SwitchDefault(SwitchDefault),
    TemplateElement(TemplateElement),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarator(VariableDeclarator),
}

// `Script`, `Module`

// interface Program : Node { };

pub enum Program {
    Module(Module),
    Script(Script),
}

// interface Statement : Node { };

pub enum Statement {
    IterationStatement(IterationStatement),
    ClassDeclaration(ClassDeclaration),
    BlockStatement(BlockStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    DebuggerStatement(DebuggerStatement),
    EmptyStatement(EmptyStatement),
    ExpressionStatement(ExpressionStatement),
    IfStatement(IfStatement),
    LabeledStatement(LabeledStatement),
    ReturnStatement(ReturnStatement),
    SwitchStatement(SwitchStatement),
    SwitchStatementWithDefault(SwitchStatementWithDefault),
    ThrowStatement(ThrowStatement),
    TryCatchStatement(TryCatchStatement),
    TryFinallyStatement(TryFinallyStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WithStatement(WithStatement),
    FunctionDeclaration(FunctionDeclaration),
}

// interface IterationStatement : Statement {
//   attribute Statement body;
// };

pub enum IterationStatement {
    DoWhileStatement(DoWhileStatement),
    ForInStatement(ForInStatement),
    ForOfStatement(ForOfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
}

// interface Expression : Node { };

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

// `MemberExpression`, `SuperProperty`
// interface MemberExpression : Expression {
//   // The object whose property is being accessed.
//   attribute (Expression or Super) _object;
// };

pub enum MemberExpression {
    ComputedMemberExpression(ComputedMemberExpression),
    StaticMemberExpression(StaticMemberExpression),
}

// `[ Expression ]`, `. IdentifierName`
// interface PropertyName : Node { };

pub enum PropertyName {
    ComputedPropertyName(ComputedPropertyName),
    StaticPropertyName(StaticPropertyName),
}

// `PropertyDefinition`
// interface ObjectProperty : Node { };

pub enum ObjectProperty {
    NamedObjectProperty(NamedObjectProperty),
    ShorthandProperty(ShorthandProperty),
}

// `PropertyName : AssignmentExpression`, `MethodDefinition`
// interface NamedObjectProperty : ObjectProperty {
//   attribute PropertyName name;
// };

pub enum NamedObjectProperty {
    MethodDefinition(MethodDefinition),
    DataProperty(DataProperty),
}

// interface MethodDefinition : NamedObjectProperty {
//   attribute FunctionBody body;
// };

pub enum MethodDefinition {
    Method(Method),
    Getter(Getter),
    Setter(Setter),
}

// interface ImportDeclaration : Node {
//   attribute string moduleSpecifier;
// };

pub enum ImportDeclaration {
    Import(Import),
    ImportNamespace(ImportNamespace),
}

// interface ExportDeclaration : Node { };

pub enum ExportDeclaration {
    ExportAllFrom(ExportAllFrom),
    ExportFrom(ExportFrom),
    ExportLocals(ExportLocals),
    Export(Export),
    ExportDefault(ExportDefault),
}

// `IdentifierReference`, `BindingIdentifier`
// interface VariableReference : Node {
//   attribute Identifier name;
// };

pub enum VariableReference {
    BindingIdentifier(BindingIdentifier),
    AssignmentTargetIdentifier(AssignmentTargetIdentifier),
}

// bindings

//typedef (ObjectBinding or ArrayBinding) BindingPattern;

pub enum BindingPattern {
    ObjectBinding(ObjectBinding),
    ArrayBinding(ArrayBinding),
}

// typedef (BindingPattern or BindingIdentifier) Binding;

pub enum Binding {
    BindingPattern(BindingPattern),
    BindingIdentifier(BindingIdentifier),
}

//typedef (AssignmentTargetIdentifier or MemberAssignmentTarget) SimpleAssignmentTarget;

pub enum SimpleAssignmentTarget {
    AssignmentTargetIdentifier(AssignmentTargetIdentifier),
    MemberAssignmentTarget(MemberAssignmentTarget),
}

// typedef (ObjectAssignmentTarget or ArrayAssignmentTarget) AssignmentTargetPattern;

pub enum AssignmentTargetPattern {
    ArrayAssignmentTarget(ArrayAssignmentTarget),
    ObjectAssignmentTarget(ObjectAssignmentTarget),
}

// `DestructuringAssignmentTarget`
// typedef (AssignmentTargetPattern or SimpleAssignmentTarget) AssignmentTarget;

pub enum AssignmentTarget {
    AssignmentTargetPattern(AssignmentTargetPattern),
    SimpleAssignmentTarget(SimpleAssignmentTarget),
}

// `FormalParameter`
// typedef (Binding or BindingWithDefault) Parameter;

pub enum Parameter {
    Binding(Binding),
    BindingWithDefault(BindingWithDefault),
}

// interface BindingWithDefault : Node {
//   attribute Binding binding;
//   attribute Expression init;
// };

pub struct BindingWithDefault {
    pub binding: Binding,
    pub init: Box<Expression>,
}

impl BindingWithDefault {
    pub fn new(binding: Binding, init: Box<Expression>) -> Self {
        Self { binding, init }
    }
}

// interface BindingIdentifier : VariableReference { };

pub struct BindingIdentifier {
    pub name: Identifier,
}

impl BindingIdentifier {
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }
}

// interface AssignmentTargetIdentifier : VariableReference { };

pub struct AssignmentTargetIdentifier {
    pub name: Identifier,
}

impl AssignmentTargetIdentifier {
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }
}

pub enum ExpressionOrSuper {
    Expression(Box<Expression>),
    Super(Super),
}

// interface MemberAssignmentTarget : Node {
//   // The object whose property is being assigned.
//   attribute (Expression or Super) _object;
// };

pub enum MemberAssignmentTarget {
    ComputedMemberAssignmentTarget(ComputedMemberAssignmentTarget),
    StaticMemberAssignmentTarget(StaticMemberAssignmentTarget),
}

// interface ComputedMemberAssignmentTarget : MemberAssignmentTarget {
//   // The expression resolving to the name of the property to be accessed.
//   attribute Expression expression;
// };

pub struct ComputedMemberAssignmentTarget {
    pub object: ExpressionOrSuper,
    pub expression: Box<Expression>,
}

impl ComputedMemberAssignmentTarget {
    pub fn new(object: ExpressionOrSuper, expression: Box<Expression>) -> Self {
        Self { object, expression }
    }
}

// interface StaticMemberAssignmentTarget : MemberAssignmentTarget {
//   // The name of the property to be accessed.
//   attribute IdentifierName property;
// };

pub struct StaticMemberAssignmentTarget {
    pub object: ExpressionOrSuper,
    pub property: IdentifierName,
}

impl StaticMemberAssignmentTarget {
    pub fn new(object: ExpressionOrSuper, property: IdentifierName) -> Self {
        Self { object, property }
    }
}

// `ArrayBindingPattern`
// interface ArrayBinding : Node {
//   // The elements of the array pattern; a null value represents an elision.
//   attribute (Binding or BindingWithDefault)?[] elements;
//   attribute Binding? rest;
// };

pub struct ArrayBinding {
    pub elements: Vec<Option<Parameter>>,
    pub rest: Option<Box<Binding>>,
}

impl ArrayBinding {
    pub fn new(elements: Vec<Option<Parameter>>, rest: Option<Box<Binding>>) -> Self {
        Self { elements, rest }
    }
}

//interface ObjectBinding : Node {
//  attribute BindingProperty[] properties;
//};

pub struct ObjectBinding {
    pub properties: Vec<BindingProperty>,
}

impl ObjectBinding {
    pub fn new(properties: Vec<BindingProperty>) -> Self {
        Self { properties }
    }
}

// interface BindingProperty : Node { };

pub enum BindingProperty {
    BindingPropertyIdentifier(BindingPropertyIdentifier),
    BindingPropertyProperty(BindingPropertyProperty),
}

// `SingleNameBinding`
// interface BindingPropertyIdentifier : BindingProperty {
//   attribute BindingIdentifier binding;
//   attribute Expression? init;
// };

pub struct BindingPropertyIdentifier {
    pub binding: BindingIdentifier,
    pub init: Option<Box<Expression>>,
}

impl BindingPropertyIdentifier {
    pub fn new(binding: BindingIdentifier, init: Option<Box<Expression>>) -> Self {
        Self { binding, init }
    }
}

// `BindingProperty :: PropertyName : BindingElement`
// interface BindingPropertyProperty : BindingProperty {
//   attribute PropertyName name;
//   attribute (Binding or BindingWithDefault) binding;
// };

pub struct BindingPropertyProperty {
    pub name: PropertyName,
    pub binding: Parameter,
}

impl BindingPropertyProperty {
    pub fn new(name: PropertyName, binding: Parameter) -> Self {
        Self { name, binding }
    }
}

// This interface represents the case where the initializer is present in `AssignmentElement :: DestructuringAssignmentTarget Initializer_opt`.
// interface AssignmentTargetWithDefault : Node {
//   attribute AssignmentTarget binding;
//   attribute Expression init;
// };

pub struct AssignmentTargetWithDefault {
    pub binding: AssignmentTarget,
    pub init: Box<Expression>,
}

impl AssignmentTargetWithDefault {
    pub fn new(binding: AssignmentTarget, init: Box<Expression>) -> Self {
        Self { binding, init }
    }
}

// `ArrayAssignmentPattern`
// interface ArrayAssignmentTarget : Node {
//   // The elements of the array pattern; a null value represents an elision.
//   attribute (AssignmentTarget or AssignmentTargetWithDefault)?[] elements;
//   attribute AssignmentTarget? rest;
// };

pub enum AssignmentTargetMaybeDefault {
    AssignmentTarget(AssignmentTarget),
    AssignmentTargetWithDefault(AssignmentTargetWithDefault),
}

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

// `ObjectAssignmentPattern`
// interface ObjectAssignmentTarget : Node {
//   attribute AssignmentTargetProperty[] properties;
// };

pub struct ObjectAssignmentTarget {
    pub properties: Vec<AssignmentTargetProperty>,
}

impl ObjectAssignmentTarget {
    pub fn new(properties: Vec<AssignmentTargetProperty>) -> Self {
        Self { properties }
    }
}

// `AssignmentProperty`
// interface AssignmentTargetProperty : Node { };

pub enum AssignmentTargetProperty {
    AssignmentTargetPropertyIdentifier(AssignmentTargetPropertyIdentifier),
    AssignmentTargetPropertyProperty(AssignmentTargetPropertyProperty),
}

// `AssignmentProperty :: IdentifierReference Initializer_opt`
// interface AssignmentTargetPropertyIdentifier : AssignmentTargetProperty {
//   attribute AssignmentTargetIdentifier binding;
//   attribute Expression? init;
// };

pub struct AssignmentTargetPropertyIdentifier {
    pub binding: AssignmentTargetIdentifier,
    pub init: Option<Box<Expression>>,
}

impl AssignmentTargetPropertyIdentifier {
    pub fn new(binding: AssignmentTargetIdentifier, init: Option<Box<Expression>>) -> Self {
        Self { binding, init }
    }
}

// `AssignmentProperty :: PropertyName : AssignmentElement`
// interface AssignmentTargetPropertyProperty : AssignmentTargetProperty {
//   attribute PropertyName name;
//   attribute (AssignmentTarget or AssignmentTargetWithDefault) binding;
// };

pub struct AssignmentTargetPropertyProperty {
    pub name: PropertyName,
    pub binding: AssignmentTargetMaybeDefault,
}

impl AssignmentTargetPropertyProperty {
    pub fn new(name: PropertyName, binding: AssignmentTargetMaybeDefault) -> Self {
        Self { name, binding }
    }
}

// classes

// interface Class {
//   attribute Expression? super;
//   attribute ClassElement[] elements;
// };

// struct Class {
//     super_: Option<Expression>,
//     elements: Vec<ClassElement>,
// }

// interface ClassExpression : Expression {
//   attribute BindingIdentifier? name;
// };
// ClassExpression implements Class;

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

// interface ClassDeclaration : Statement {
//   attribute BindingIdentifier name;
// };
// ClassDeclaration implements Class;

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

// interface ClassElement : Node {
//   // True iff `IsStatic` of ClassElement is true.
//   attribute boolean isStatic;
//   attribute MethodDefinition method;
// };

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

// modules

// interface Module : Program {
//   attribute Directive[] directives;
//   attribute (ImportDeclaration or ExportDeclaration or Statement)[] items;
// };

pub enum ModuleItems {
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    Statement(Box<Statement>),
}

pub struct Module {
    pub directives: Vec<Directive>,
    pub items: Vec<ModuleItems>,
}

impl Module {
    pub fn new(directives: Vec<Directive>, items: Vec<ModuleItems>) -> Self {
        Self { directives, items }
    }
}

// An `ImportDeclaration` not including a namespace import.
// interface Import : ImportDeclaration {
//   // `ImportedDefaultBinding`, if present.
//   attribute BindingIdentifier? defaultBinding;
//   attribute ImportSpecifier[] namedImports;
// };

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

// An `ImportDeclaration` including a namespace import.
// interface ImportNamespace : ImportDeclaration {
//   // `ImportedDefaultBinding`, if present.
//   attribute BindingIdentifier? defaultBinding;
//   attribute BindingIdentifier namespaceBinding;
// };

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

// interface ImportSpecifier : Node {
//   // The `IdentifierName` in the production `ImportSpecifier :: IdentifierName as ImportedBinding`; absent if this specifier represents the production `ImportSpecifier :: ImportedBinding`.
//   attribute IdentifierName? name;
//   attribute BindingIdentifier binding;
// };

pub struct ImportSpecifier {
    pub name: Option<IdentifierName>,
    pub binding: BindingIdentifier,
}

impl ImportSpecifier {
    pub fn new(name: Option<IdentifierName>, binding: BindingIdentifier) -> Self {
        Self { name, binding }
    }
}

// `export * FromClause;`
// interface ExportAllFrom : ExportDeclaration {
//   attribute string moduleSpecifier;
// };

pub struct ExportAllFrom {
    pub module_specifier: String,
}

impl ExportAllFrom {
    pub fn new(module_specifier: String) -> Self {
        Self { module_specifier }
    }
}

// `export ExportClause FromClause;`
// interface ExportFrom : ExportDeclaration {
//   attribute ExportFromSpecifier[] namedExports;
//   attribute string moduleSpecifier;
// };

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

// `export ExportClause;`
// interface ExportLocals : ExportDeclaration {
//   attribute ExportLocalSpecifier[] namedExports;
// };

pub struct ExportLocals {
    pub named_exports: Vec<ExportLocalSpecifier>,
}

impl ExportLocals {
    pub fn new(named_exports: Vec<ExportLocalSpecifier>) -> Self {
        Self { named_exports }
    }
}

// `export VariableStatement`, `export Declaration`
// interface Export : ExportDeclaration {
//   attribute (FunctionDeclaration or ClassDeclaration or VariableDeclaration) declaration;
// };

pub enum Export {
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    VariableDeclaration(VariableDeclaration),
}

// `export default HoistableDeclaration`, `export default ClassDeclaration`, `export default AssignmentExpression`
// interface ExportDefault : ExportDeclaration {
//   attribute (FunctionDeclaration or ClassDeclaration or Expression) body;
// };

pub enum ExportDefault {
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    Expression(Box<Expression>),
}

// `ExportSpecifier`, as part of an `ExportFrom`.
// interface ExportFromSpecifier : Node {
//   // The only `IdentifierName in `ExportSpecifier :: IdentifierName`, or the first in `ExportSpecifier :: IdentifierName as IdentifierName`.
//   attribute IdentifierName name;
//   // The second `IdentifierName` in `ExportSpecifier :: IdentifierName as IdentifierName`, if that is the production represented.
//   attribute IdentifierName? exportedName;
// };

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

// `ExportSpecifier`, as part of an `ExportLocals`.
// interface ExportLocalSpecifier : Node {
//   // The only `IdentifierName in `ExportSpecifier :: IdentifierName`, or the first in `ExportSpecifier :: IdentifierName as IdentifierName`.
//   attribute IdentifierExpression name;
//   // The second `IdentifierName` in `ExportSpecifier :: IdentifierName as IdentifierName`, if present.
//   attribute IdentifierName? exportedName;
// };

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

// property definition

// `MethodDefinition :: PropertyName ( UniqueFormalParameters ) { FunctionBody }`, `GeneratorMethod :: * PropertyName ( UniqueFormalParameters ) { GeneratorBody }`, `AsyncMethod :: async PropertyName ( UniqueFormalParameters ) { AsyncFunctionBody }`
// interface Method : MethodDefinition {
//   // True for `AsyncMethod`, false otherwise.
//   attribute boolean isAsync;
//   // True for `GeneratorMethod`, false otherwise.
//   attribute boolean isGenerator;
//   // The `UniqueFormalParameters`.
//   attribute FormalParameters params;
// };

pub struct Method {
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters,
}

impl Method {
    pub fn new(is_async: bool, is_generator: bool, params: FormalParameters) -> Self {
        Self {
            is_async,
            is_generator,
            params,
        }
    }
}

// `get PropertyName ( ) { FunctionBody }`
// interface Getter : MethodDefinition { };

pub struct Getter {
    pub property_name: PropertyName,
}

impl Getter {
    pub fn new(property_name: PropertyName) -> Self {
        Self { property_name }
    }
}

// `set PropertyName ( PropertySetParameterList ) { FunctionBody }`
// interface Setter : MethodDefinition {
//   // The `PropertySetParameterList`.
//   attribute Parameter param;
// };

pub struct Setter {
    pub property_name: PropertyName,
    pub param: Parameter,
}

impl Setter {
    pub fn new(property_name: PropertyName, param: Parameter) -> Self {
        Self {
            property_name,
            param,
        }
    }
}

// `PropertyDefinition :: PropertyName : AssignmentExpression`
// interface DataProperty : NamedObjectProperty {
//   // The `AssignmentExpression`.
//   attribute Expression expression;
// };

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

// `PropertyDefinition :: IdentifierReference`
// interface ShorthandProperty : ObjectProperty {
//   // The `IdentifierReference`.
//   attribute IdentifierExpression name;
// };

pub struct ShorthandProperty {
    pub name: IdentifierExpression,
}

impl ShorthandProperty {
    pub fn new(name: IdentifierExpression) -> Self {
        Self { name }
    }
}

// interface ComputedPropertyName : PropertyName {
//   attribute Expression expression;
// };

pub struct ComputedPropertyName {
    pub expression: Box<Expression>,
}

impl ComputedPropertyName {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

// `LiteralPropertyName`
// interface StaticPropertyName : PropertyName {
//   attribute string value;
// };

pub struct StaticPropertyName {
    pub value: String,
}

impl StaticPropertyName {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

// literals

// `BooleanLiteral`
// interface LiteralBooleanExpression : Expression {
//   attribute boolean value;
// };

pub struct LiteralBooleanExpression {
    pub value: bool,
}

impl LiteralBooleanExpression {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

// A `NumericLiteral` for which the Number value of its MV is positive infinity.
// interface LiteralInfinityExpression : Expression { };

#[derive(Default)]
pub struct LiteralInfinityExpression {}

impl LiteralInfinityExpression {
    pub fn new() -> Self {
        Self {}
    }
}

// `NullLiteral`
// interface LiteralNullExpression : Expression { };

#[derive(Default)]
pub struct LiteralNullExpression {}

impl LiteralNullExpression {
    pub fn new() -> Self {
        Self {}
    }
}

// `NumericLiteral`
// interface LiteralNumericExpression : Expression {
//   attribute double value;
// };

pub struct LiteralNumericExpression {
    pub value: f64,
}

impl LiteralNumericExpression {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

// `RegularExpressionLiteral`
// interface LiteralRegExpExpression : Expression {
//   attribute string pattern;
//   // Whether the `g` flag is present.
//   attribute boolean global;
//   // Whether the `i` flag is present.
//   attribute boolean ignoreCase;
//   // Whether the `m` flag is present.
//   attribute boolean multiLine;
//   // Whether the `y` flag is present.
//   attribute boolean sticky;
//   // Whether the `u` flag is present.
//   attribute boolean unicode;
// };

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

// `StringLiteral`
// interface LiteralStringExpression : Expression {
//   attribute string value;
// };

pub struct LiteralStringExpression {
    pub value: String,
}

impl LiteralStringExpression {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

// other expressions

// `ArrayLiteral`
// interface ArrayExpression : Expression {
//   // The elements of the array literal; a null value represents an elision.
//   attribute (SpreadElement or Expression)?[] elements;
// };

pub enum ArrayExpressionElement {
    SpreadElement(SpreadElement),
    Expression(Box<Expression>),
    Elision,
}

pub struct ArrayExpression {
    pub elements: Vec<ArrayExpressionElement>,
}

impl ArrayExpression {
    pub fn new(elements: Vec<ArrayExpressionElement>) -> Self {
        Self { elements }
    }
}

// `ArrowFunction`, `AsyncArrowFunction`
// interface ArrowExpression : Expression {
//   // True for `AsyncArrowFunction`, false otherwise.
//   attribute boolean isAsync;
//   attribute FormalParameters params;
//   attribute (FunctionBody or Expression) body;
// };

pub enum ArrowExpressionBody {
    FunctionBody(FunctionBody),
    Expression(Box<Expression>),
}

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

// `AssignmentExpression :: LeftHandSideExpression = AssignmentExpression`
// interface AssignmentExpression : Expression {
//   // The `LeftHandSideExpression`.
//   attribute AssignmentTarget binding;
//   // The `AssignmentExpression` following the `=`.
//   attribute Expression expression;
// };

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

// `ExponentiationExpression`, `MultiplicativeExpression`, `AdditiveExpression`, `ShiftExpression`, `RelationalExpression`, `EqualityExpression`, `BitwiseANDExpression`, `BitwiseXORExpression`, `BitwiseORExpression`, `LogicalANDExpression`, `LogicalORExpression`
// interface BinaryExpression : Expression {
//   attribute BinaryOperator operator;
//   // The expression before the operator.
//   attribute Expression left;
//   // The expression after the operator.
//   attribute Expression right;
// };

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

// interface CallExpression : Expression {
//   attribute (Expression or Super) callee;
//   attribute Arguments arguments;
// };

pub struct CallExpression {
    pub callee: ExpressionOrSuper,
    pub arguments: Arguments,
}

impl CallExpression {
    pub fn new(callee: ExpressionOrSuper, arguments: Arguments) -> Self {
        Self { callee, arguments }
    }
}

// `AssignmentExpression :: LeftHandSideExpression AssignmentOperator AssignmentExpression`
// interface CompoundAssignmentExpression : Expression {
//   attribute CompoundAssignmentOperator operator;
//   // The `LeftHandSideExpression`.
//   attribute SimpleAssignmentTarget binding;
//   // The `AssignmentExpression`.
//   attribute Expression expression;
// };

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

// interface ComputedMemberExpression : MemberExpression {
//   // The expression resolving to the name of the property to be accessed.
//   attribute Expression expression;
// };

pub struct ComputedMemberExpression {
    pub object: ExpressionOrSuper,
    pub expression: Box<Expression>,
}

impl ComputedMemberExpression {
    pub fn new(object: ExpressionOrSuper, expression: Box<Expression>) -> Self {
        Self { object, expression }
    }
}

// `ConditionalExpression :: LogicalORExpression ? AssignmentExpression : AssignmentExpression`
// interface ConditionalExpression : Expression {
//   // The `LogicalORExpression`.
//   attribute Expression test;
//   // The first `AssignmentExpression`.
//   attribute Expression consequent;
//   // The second `AssignmentExpression`.
//   attribute Expression alternate;
// };

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

// interface FunctionExpression : Expression {
//   attribute BindingIdentifier? name;
// };
// FunctionExpression implements Function;

pub struct FunctionExpression {
    pub name: Option<BindingIdentifier>,
    // True for `AsyncFunctionExpression` and `AsyncFunctionDeclaration`, false otherwise.
    pub is_async: bool,
    // True for `GeneratorExpression` and `GeneratorDeclaration`, false otherwise.
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

// `IdentifierReference`
// interface IdentifierExpression : Expression { };
// IdentifierExpression implements VariableReference;

pub struct IdentifierExpression {
    pub var: VariableReference,
}

impl IdentifierExpression {
    pub fn new(var: VariableReference) -> Self {
        Self { var }
    }
}

// interface NewExpression : Expression {
//   attribute Expression callee;
//   attribute Arguments arguments;
// };

pub struct NewExpression {
    pub callee: Box<Expression>,
    pub arguments: Arguments,
}

impl NewExpression {
    pub fn new(callee: Box<Expression>, arguments: Arguments) -> Self {
        Self { callee, arguments }
    }
}

// interface NewTargetExpression : Expression { };

#[derive(Default)]
pub struct NewTargetExpression {}

impl NewTargetExpression {
    pub fn new() -> Self {
        Self {}
    }
}

// interface ObjectExpression : Expression {
//   attribute ObjectProperty[] properties;
// };

pub struct ObjectExpression {
    pub properties: Vec<ObjectProperty>,
}

impl ObjectExpression {
    pub fn new(properties: Vec<ObjectProperty>) -> Self {
        Self { properties }
    }
}

// interface UnaryExpression : Expression {
//   attribute UnaryOperator operator;
//   attribute Expression operand;
// };

pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: UnaryOperator, operand: Box<Expression>) -> Self {
        Self { operator, operand }
    }
}

// interface StaticMemberExpression : MemberExpression {
//   // The name of the property to be accessed.
//   attribute IdentifierName property;
// };

pub struct StaticMemberExpression {
    pub object: ExpressionOrSuper,
    pub property: IdentifierName,
}

impl StaticMemberExpression {
    pub fn new(object: ExpressionOrSuper, property: IdentifierName) -> Self {
        Self { object, property }
    }
}

// `TemplateLiteral`, `MemberExpression :: MemberExpression TemplateLiteral`, `CallExpression : CallExpression TemplateLiteral`
// interface TemplateExpression : Expression {
//   // The second `MemberExpression` or `CallExpression`, if present.
//   attribute Expression? tag;
//   // The contents of the template. This list must be alternating TemplateElements and Expressions, beginning and ending with TemplateElement.
//   attribute (Expression or TemplateElement)[] elements;
// };

pub enum TemplateExpressionElement {
    Expression(Box<Expression>),
    TemplateElement(TemplateElement),
}

pub struct TemplateExpression {
    pub tag: Option<Box<Expression>>,
    pub elements: Vec<TemplateExpressionElement>,
}

impl TemplateExpression {
    pub fn new(tag: Option<Box<Expression>>, elements: Vec<TemplateExpressionElement>) -> Self {
        Self { tag, elements }
    }
}

// `PrimaryExpression :: this`
// interface ThisExpression : Expression { };

#[derive(Default)]
pub struct ThisExpression {}

impl ThisExpression {
    pub fn new() -> Self {
        Self {}
    }
}

// `UpdateExpression :: LeftHandSideExpression ++`, `UpdateExpression :: LeftHandSideExpression --`, `UpdateExpression :: ++ LeftHandSideExpression`, ``UpdateExpression :: -- LeftHandSideExpression`
// interface UpdateExpression : Expression {
//   // True for `UpdateExpression :: ++ LeftHandSideExpression` and `UpdateExpression :: -- LeftHandSideExpression`, false otherwise.
//   attribute boolean isPrefix;
//   attribute UpdateOperator operator;
//   attribute SimpleAssignmentTarget operand;
// };

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

// `YieldExpression :: yield`, `YieldExpression :: yield AssignmentExpression`
// interface YieldExpression : Expression {
//   // The `AssignmentExpression`, if present.
//   attribute Expression? expression;
// };

pub struct YieldExpression {
    pub expression: Option<Box<Expression>>,
}

impl YieldExpression {
    pub fn new(expression: Option<Box<Expression>>) -> Self {
        Self { expression }
    }
}

// `YieldExpression :: yield * AssignmentExpression`
//interface YieldGeneratorExpression : Expression {
//  attribute Expression expression;
//};

pub struct YieldGeneratorExpression {
    pub expression: Box<Expression>,
}

impl YieldGeneratorExpression {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

// interface AwaitExpression : Expression {
//   attribute Expression expression;
// };

pub struct AwaitExpression {
    pub expression: Box<Expression>,
}

impl AwaitExpression {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

// other statements

// interface BlockStatement : Statement {
//   attribute Block block;
// };

pub struct BlockStatement {
    pub block: Block,
}

impl BlockStatement {
    pub fn new(block: Block) -> Self {
        Self { block }
    }
}

// interface BreakStatement : Statement {
//   attribute Label? label;
// };

pub struct BreakStatement {
    pub label: Option<Label>,
}

impl BreakStatement {
    pub fn new(label: Option<Label>) -> Self {
        Self { label }
    }
}

// interface ContinueStatement : Statement {
//   attribute Label? label;
// };

pub struct ContinueStatement {
    pub label: Option<Label>,
}

impl ContinueStatement {
    pub fn new(label: Option<Label>) -> Self {
        Self { label }
    }
}

// interface DebuggerStatement : Statement { };

#[derive(Default)]
pub struct DebuggerStatement {}

impl DebuggerStatement {
    pub fn new() -> Self {
        Self {}
    }
}

// interface DoWhileStatement : IterationStatement {
//   attribute Expression test;
// };

pub struct DoWhileStatement {
    pub block: Box<Statement>,
    pub test: Box<Expression>,
}

impl DoWhileStatement {
    pub fn new(block: Box<Statement>, test: Box<Expression>) -> Self {
        Self { block, test }
    }
}

// interface EmptyStatement : Statement { };

#[derive(Default)]
pub struct EmptyStatement {}

impl EmptyStatement {
    pub fn new() -> Self {
        Self {}
    }
}

//interface ExpressionStatement : Statement {
//  attribute Expression expression;
//};

pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}

impl ExpressionStatement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

// `for ( LeftHandSideExpression in Expression ) Statement`, `for ( var ForBinding in Expression ) Statement`, `for ( ForDeclaration in Expression ) Statement`, `for ( var BindingIdentifier Initializer in Expression ) Statement`
// interface ForInStatement : IterationStatement {
//   // The expression or declaration before `in`.
//   attribute (VariableDeclaration or AssignmentTarget) left;
//   // The expression after `in`.
//   attribute Expression right;
// };

pub enum VariableDeclarationOrAssignmentTarget {
    VariableDeclaration(VariableDeclaration),
    AssignmentTarget(AssignmentTarget),
}

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

// `for ( LeftHandSideExpression of Expression ) Statement`, `for ( var ForBinding of Expression ) Statement`, `for ( ForDeclaration of Expression ) Statement`
// interface ForOfStatement : IterationStatement {
//   // The expression or declaration before `of`.
//   attribute (VariableDeclaration or AssignmentTarget) left;
//   // The expression after `of`.
//   attribute Expression right;
// };

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

// `for ( Expression ; Expression ; Expression ) Statement`, `for ( var VariableDeclarationlist ; Expression ; Expression ) Statement`
// interface ForStatement : IterationStatement {
//   // The expression or declaration before the first `;`, if present.
//   attribute (VariableDeclaration or Expression)? init;
//   // The expression before the second `;`, if present
//   attribute Expression? test;
//   // The expression after the second `;`, if present
//   attribute Expression? update;
// };

pub enum VariableDeclarationOrExpression {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<Expression>),
}

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

// `if ( Expression ) Statement`, `if ( Expression ) Statement else Statement`,
// interface IfStatement : Statement {
//   attribute Expression test;
//   // The first `Statement`.
//   attribute Statement consequent;
//   // The second `Statement`, if present.
//   attribute Statement? alternate;
// };

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

// interface LabeledStatement : Statement {
//   attribute Label label;
//   attribute Statement body;
// };

pub struct LabeledStatement {
    pub label: Label,
    pub body: Box<Statement>,
}

impl LabeledStatement {
    pub fn new(label: Label, body: Box<Statement>) -> Self {
        Self { label, body }
    }
}

// interface ReturnStatement : Statement {
//   attribute Expression? expression;
// };

pub struct ReturnStatement {
    pub expression: Option<Box<Expression>>,
}

impl ReturnStatement {
    pub fn new(expression: Option<Box<Expression>>) -> Self {
        Self { expression }
    }
}

// A `SwitchStatement` whose `CaseBlock` is `CaseBlock :: { CaseClauses }`.
// interface SwitchStatement : Statement {
//   attribute Expression discriminant;
//   attribute SwitchCase[] cases;
// };

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

// A `SwitchStatement` whose `CaseBlock` is `CaseBlock :: { CaseClauses DefaultClause CaseClauses }`.
// interface SwitchStatementWithDefault : Statement {
//   attribute Expression discriminant;
//   // The `CaseClauses` before the `DefaultClause`.
//   attribute SwitchCase[] preDefaultCases;
//   // The `DefaultClause`.
//   attribute SwitchDefault defaultCase;
//   // The `CaseClauses` after the `DefaultClause`.
//   attribute SwitchCase[] postDefaultCases;
// };

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

// interface ThrowStatement : Statement {
//   attribute Expression expression;
// };

pub struct ThrowStatement {
    pub expression: Box<Expression>,
}

impl ThrowStatement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

// `TryStatement :: try Block Catch`
// interface TryCatchStatement : Statement {
//   attribute Block body;
//   attribute CatchClause catchClause;
// };

pub struct TryCatchStatement {
    pub body: Block,
    pub catch_clause: CatchClause,
}

impl TryCatchStatement {
    pub fn new(body: Block, catch_clause: CatchClause) -> Self {
        Self { body, catch_clause }
    }
}

// `TryStatement :: try Block Finally`, `TryStatement :: try Block Catch Finally`
// interface TryFinallyStatement : Statement {
//   // The `Block`.
//   attribute Block body;
//   // The `Catch`, if present.
//   attribute CatchClause? catchClause;
//   // The `Finally`.
//   attribute Block finalizer;
// };

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

// interface VariableDeclarationStatement : Statement {
//   attribute VariableDeclaration declaration;
// };

pub struct VariableDeclarationStatement {
    pub declaration: VariableDeclaration,
}

impl VariableDeclarationStatement {
    pub fn new(declaration: VariableDeclaration) -> Self {
        Self { declaration }
    }
}

// interface WhileStatement : IterationStatement {
//   attribute Expression test;
// };

pub struct WhileStatement {
    pub test: Box<Expression>,
    pub block: Box<Statement>,
}

impl WhileStatement {
    pub fn new(test: Box<Expression>, block: Box<Statement>) -> Self {
        Self { test, block }
    }
}

// interface WithStatement : Statement {
//   attribute Expression _object;
//   attribute Statement body;
// };

pub struct WithStatement {
    pub object: Box<Expression>,
    pub body: Box<Statement>,
}

impl WithStatement {
    pub fn new(object: Box<Expression>, body: Box<Statement>) -> Self {
        Self { object, body }
    }
}

// other nodes

// interface Block : Node {
//   attribute Statement[] statements;
// };

pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

// `Catch`
// interface CatchClause : Node {
//   attribute Binding binding;
//   attribute Block body;
// };

pub struct CatchClause {
    pub binding: Binding,
    pub body: Block,
}

impl CatchClause {
    pub fn new(binding: Binding, body: Block) -> Self {
        Self { binding, body }
    }
}

// An item in a `DirectivePrologue`
// interface Directive : Node {
//   attribute string rawValue;
// };

pub struct Directive {
    pub raw_value: String,
}

impl Directive {
    pub fn new(raw_value: String) -> Self {
        Self { raw_value }
    }
}

// interface FormalParameters : Node {
//   attribute Parameter[] items;
//   attribute Binding? rest;
// };

pub struct FormalParameters {
    pub items: Vec<Parameter>,
    pub rest: Option<Binding>,
}

impl FormalParameters {
    pub fn new(items: Vec<Parameter>, rest: Option<Binding>) -> Self {
        Self { items, rest }
    }
}

// interface FunctionBody : Node {
//   attribute Directive[] directives;
//   attribute Statement[] statements;
// };

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

// interface FunctionDeclaration : Statement {
//   attribute BindingIdentifier name;
// };
// FunctionDeclaration implements Function;

pub struct FunctionDeclaration {
    pub name: BindingIdentifier,
    // True for `AsyncFunctionExpression` and `AsyncFunctionDeclaration`, false otherwise.
    pub is_async: bool,
    // True for `GeneratorExpression` and `GeneratorDeclaration`, false otherwise.
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

// interface Script : Program {
//   attribute Directive[] directives;
//   attribute Statement[] statements;
// };

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

// interface SpreadElement : Node {
//   attribute Expression expression;
// };

pub struct SpreadElement {
    pub expression: Box<Expression>,
}

impl SpreadElement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

// `super`
// interface Super : Node { };

#[derive(Default)]
pub struct Super {}

impl Super {
    pub fn new() -> Self {
        Self {}
    }
}

// `CaseClause`
// interface SwitchCase : Node {
//   attribute Expression test;
//   attribute Statement[] consequent;
// };

pub struct SwitchCase {
    pub test: Box<Expression>,
    pub consequent: Vec<Statement>,
}

impl SwitchCase {
    pub fn new(test: Box<Expression>, consequent: Vec<Statement>) -> Self {
        Self { test, consequent }
    }
}

// `DefaultClause`
// interface SwitchDefault : Node {
//   attribute Statement[] consequent;
// };

pub struct SwitchDefault {
    pub consequent: Vec<Statement>,
}

impl SwitchDefault {
    pub fn new(consequent: Vec<Statement>) -> Self {
        Self { consequent }
    }
}

// `TemplateCharacters`
// interface TemplateElement : Node {
//   attribute string rawValue;
// };

pub struct TemplateElement {
    pub raw_value: String,
}

impl TemplateElement {
    pub fn new(raw_value: String) -> Self {
        Self { raw_value }
    }
}

// interface VariableDeclaration : Node {
//   attribute VariableDeclarationKind kind;
//   [NonEmpty] attribute VariableDeclarator[] declarators;
// };

pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub declarators: Vec<VariableDeclarator>,
}

impl VariableDeclaration {
    pub fn new(kind: VariableDeclarationKind, declarators: Vec<VariableDeclarator>) -> Self {
        Self { kind, declarators }
    }
}

// interface VariableDeclarator : Node {
//   attribute Binding binding;
//   attribute Expression? init;
// };

pub struct VariableDeclarator {
    pub binding: Binding,
    pub init: Option<Box<Expression>>,
}

impl VariableDeclarator {
    pub fn new(binding: Binding, init: Option<Box<Expression>>) -> Self {
        Self { binding, init }
    }
}
