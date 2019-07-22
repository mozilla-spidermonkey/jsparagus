// Derived from https://github.com/shapesecurity/shift-spec/blob/es2017/spec.idl

/*
  INFO: This specification is currently divided into the following sections:
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

enum Argument {
    SpreadElement(SpreadElement),
    Expression(Box<Expression>),
}

struct Arguments {
    args: Vec<Argument>,
}

// typedef DOMString string;

// typedef string Identifier;
struct Identifier {
    value: String,
}

// typedef string IdentifierName;
struct IdentifierName {
    value: String,
}

// typedef string Label;
struct Label {
    value: String,
}

enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

enum CompoundAssignmentOperator {
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

enum BinaryOperator {
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

enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Void,
    Delete,
}

enum UpdateOperator {
    Increment,
    Decrement,
}

// `FunctionExpression`, `FunctionDeclaration`, `GeneratorExpression`, `GeneratorDeclaration`, `AsyncFunctionExpression`, `AsyncFunctionDeclaration`
struct Function {
    // True for `AsyncFunctionExpression` and `AsyncFunctionDeclaration`, false otherwise.
    isAsync: bool,
    // True for `GeneratorExpression` and `GeneratorDeclaration`, false otherwise.
    isGenerator: bool,
    params: FormalParameters,
    body: FunctionBody,
}

// node classes

// interface Node {
//   [TypeIndicator] readonly attribute Type type;
// };

enum Node {
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

enum Program {
    Module(Module),
    Script(Script),
}

// interface Statement : Node { };

enum Statement {
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

enum IterationStatement {
    DoWhileStatement(DoWhileStatement, Box<Statement>),
    ForInStatement(ForInStatement, Box<Statement>),
    ForOfStatement(ForOfStatement, Box<Statement>),
    ForStatement(ForStatement, Box<Statement>),
    WhileStatement(WhileStatement, Box<Statement>),
}

// interface Expression : Node { };

enum Expression {
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

enum MemberExpression {
    ComputedMemberExpression(ComputedMemberExpression, ExpressionOrSuper),
    StaticMemberExpression(StaticMemberExpression, ExpressionOrSuper),
}

// `[ Expression ]`, `. IdentifierName`
// interface PropertyName : Node { };

enum PropertyName {
    ComputedPropertyName(ComputedPropertyName),
    StaticPropertyName(StaticPropertyName),
}

// `PropertyDefinition`
// interface ObjectProperty : Node { };

enum ObjectProperty {
    NamedObjectProperty(NamedObjectProperty),
    ShorthandProperty(ShorthandProperty),
}

// `PropertyName : AssignmentExpression`, `MethodDefinition`
// interface NamedObjectProperty : ObjectProperty {
//   attribute PropertyName name;
// };

enum NamedObjectProperty {
    MethodDefinition(MethodDefinition, PropertyName),
    DataProperty(DataProperty, PropertyName),
}

// interface MethodDefinition : NamedObjectProperty {
//   attribute FunctionBody body;
// };

enum MethodDefinition {
    Method(Method),
    Getter(Getter),
    Setter(Setter),
}

// interface ImportDeclaration : Node {
//   attribute string moduleSpecifier;
// };

enum ImportDeclaration {
    Import(Import, String),
    ImportNamespace(ImportNamespace, String),
}

// interface ExportDeclaration : Node { };

enum ExportDeclaration {
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

enum VariableReference {
    BindingIdentifier(BindingIdentifier, Identifier),
    AssignmentTargetIdentifier(AssignmentTargetIdentifier, Identifier),
}

// bindings

//typedef (ObjectBinding or ArrayBinding) BindingPattern;

enum BindingPattern {
    ObjectBinding(ObjectBinding),
    ArrayBinding(ArrayBinding),
}

// typedef (BindingPattern or BindingIdentifier) Binding;

enum Binding {
    BindingPattern(BindingPattern),
    BindingIdentifier(BindingIdentifier),
}

//typedef (AssignmentTargetIdentifier or MemberAssignmentTarget) SimpleAssignmentTarget;

enum SimpleAssignmentTarget {
    AssignmentTargetIdentifier(AssignmentTargetIdentifier),
    MemberAssignmentTarget(MemberAssignmentTarget),
}

// typedef (ObjectAssignmentTarget or ArrayAssignmentTarget) AssignmentTargetPattern;

enum AssignmentTargetPattern {
    ArrayAssignmentTarget(ArrayAssignmentTarget),
    ObjectAssignmentTarget(ObjectAssignmentTarget),
}

// `DestructuringAssignmentTarget`
// typedef (AssignmentTargetPattern or SimpleAssignmentTarget) AssignmentTarget;

enum AssignmentTarget {
    AssignmentTargetPattern(AssignmentTargetPattern),
    SimpleAssignmentTarget(SimpleAssignmentTarget),
}

// `FormalParameter`
// typedef (Binding or BindingWithDefault) Parameter;

enum Parameter {
    Binding(Binding),
    BindingWithDefault(BindingWithDefault),
}

// interface BindingWithDefault : Node {
//   attribute Binding binding;
//   attribute Expression init;
// };

struct BindingWithDefault {
    binding: Binding,
    init: Box<Expression>,
}

// interface BindingIdentifier : VariableReference { };

struct BindingIdentifier {}

// interface AssignmentTargetIdentifier : VariableReference { };

struct AssignmentTargetIdentifier {}

enum ExpressionOrSuper {
    Expression(Box<Expression>),
    Super(Super),
}

// interface MemberAssignmentTarget : Node {
//   // The object whose property is being assigned.
//   attribute (Expression or Super) _object;
// };

enum MemberAssignmentTarget {
    ComputedMemberAssignmentTarget(ComputedMemberAssignmentTarget, ExpressionOrSuper),
    StaticMemberAssignmentTarget(StaticMemberAssignmentTarget, ExpressionOrSuper),
}

// interface ComputedMemberAssignmentTarget : MemberAssignmentTarget {
//   // The expression resolving to the name of the property to be accessed.
//   attribute Expression expression;
// };

struct ComputedMemberAssignmentTarget {
    expression: Box<Expression>,
}

// interface StaticMemberAssignmentTarget : MemberAssignmentTarget {
//   // The name of the property to be accessed.
//   attribute IdentifierName property;
// };

struct StaticMemberAssignmentTarget {
    property: IdentifierName,
}

// `ArrayBindingPattern`
// interface ArrayBinding : Node {
//   // The elements of the array pattern; a null value represents an elision.
//   attribute (Binding or BindingWithDefault)?[] elements;
//   attribute Binding? rest;
// };

struct ArrayBinding {
    elements: Vec<Option<Parameter>>,
    rest: Option<Box<Binding>>,
}

//interface ObjectBinding : Node {
//  attribute BindingProperty[] properties;
//};

struct ObjectBinding {
    properties: Vec<BindingProperty>,
}

// interface BindingProperty : Node { };

enum BindingProperty {
    BindingPropertyIdentifier(BindingPropertyIdentifier),
    BindingPropertyProperty(BindingPropertyProperty),
}

// `SingleNameBinding`
// interface BindingPropertyIdentifier : BindingProperty {
//   attribute BindingIdentifier binding;
//   attribute Expression? init;
// };

struct BindingPropertyIdentifier {
    binding: BindingIdentifier,
    init: Option<Box<Expression>>,
}

// `BindingProperty :: PropertyName : BindingElement`
// interface BindingPropertyProperty : BindingProperty {
//   attribute PropertyName name;
//   attribute (Binding or BindingWithDefault) binding;
// };

struct BindingPropertyProperty {
    name: PropertyName,
    binding: Parameter,
}

// This interface represents the case where the initializer is present in `AssignmentElement :: DestructuringAssignmentTarget Initializer_opt`.
// interface AssignmentTargetWithDefault : Node {
//   attribute AssignmentTarget binding;
//   attribute Expression init;
// };

struct AssignmentTargetWithDefault {
    binding: AssignmentTarget,
    init: Box<Expression>,
}

// `ArrayAssignmentPattern`
// interface ArrayAssignmentTarget : Node {
//   // The elements of the array pattern; a null value represents an elision.
//   attribute (AssignmentTarget or AssignmentTargetWithDefault)?[] elements;
//   attribute AssignmentTarget? rest;
// };

enum AssignmentTargetMaybeDefault {
    AssignmentTarget(AssignmentTarget),
    AssignmentTargetWithDefault(AssignmentTargetWithDefault),
}

struct ArrayAssignmentTarget {
    elements: Vec<Option<AssignmentTargetMaybeDefault>>,
    rest: Option<Box<AssignmentTarget>>,
}

// `ObjectAssignmentPattern`
// interface ObjectAssignmentTarget : Node {
//   attribute AssignmentTargetProperty[] properties;
// };

struct ObjectAssignmentTarget {
    properties: Vec<AssignmentTargetProperty>,
}

// `AssignmentProperty`
// interface AssignmentTargetProperty : Node { };

enum AssignmentTargetProperty {
    AssignmentTargetPropertyIdentifier(AssignmentTargetPropertyIdentifier),
    AssignmentTargetPropertyProperty(AssignmentTargetPropertyProperty),
}

// `AssignmentProperty :: IdentifierReference Initializer_opt`
// interface AssignmentTargetPropertyIdentifier : AssignmentTargetProperty {
//   attribute AssignmentTargetIdentifier binding;
//   attribute Expression? init;
// };

struct AssignmentTargetPropertyIdentifier {
    binding: AssignmentTargetIdentifier,
    init: Option<Box<Expression>>,
}

// `AssignmentProperty :: PropertyName : AssignmentElement`
// interface AssignmentTargetPropertyProperty : AssignmentTargetProperty {
//   attribute PropertyName name;
//   attribute (AssignmentTarget or AssignmentTargetWithDefault) binding;
// };

struct AssignmentTargetPropertyProperty {
    name: PropertyName,
    binding: AssignmentTargetMaybeDefault,
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

struct ClassExpression {
    name: Option<BindingIdentifier>,
    super_: Option<Box<Expression>>,
    elements: Vec<ClassElement>,
}

// interface ClassDeclaration : Statement {
//   attribute BindingIdentifier name;
// };
// ClassDeclaration implements Class;

struct ClassDeclaration {
    name: BindingIdentifier,
    super_: Option<Box<Expression>>,
    elements: Vec<ClassElement>,
}

// interface ClassElement : Node {
//   // True iff `IsStatic` of ClassElement is true.
//   attribute boolean isStatic;
//   attribute MethodDefinition method;
// };

struct ClassElement {
    isStatic: bool,
    method: MethodDefinition,
}

// modules

// interface Module : Program {
//   attribute Directive[] directives;
//   attribute (ImportDeclaration or ExportDeclaration or Statement)[] items;
// };

enum ModuleItems {
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    Statement(Box<Statement>),
}

struct Module {
    directives: Vec<Directive>,
    items: Vec<ModuleItems>,
}

// An `ImportDeclaration` not including a namespace import.
// interface Import : ImportDeclaration {
//   // `ImportedDefaultBinding`, if present.
//   attribute BindingIdentifier? defaultBinding;
//   attribute ImportSpecifier[] namedImports;
// };

struct Import {
    defaultBinding: Option<BindingIdentifier>,
    namedImports: Vec<ImportSpecifier>,
}

// An `ImportDeclaration` including a namespace import.
// interface ImportNamespace : ImportDeclaration {
//   // `ImportedDefaultBinding`, if present.
//   attribute BindingIdentifier? defaultBinding;
//   attribute BindingIdentifier namespaceBinding;
// };

struct ImportNamespace {
    defaultBinding: Option<BindingIdentifier>,
    namespaceBinding: BindingIdentifier,
}

// interface ImportSpecifier : Node {
//   // The `IdentifierName` in the production `ImportSpecifier :: IdentifierName as ImportedBinding`; absent if this specifier represents the production `ImportSpecifier :: ImportedBinding`.
//   attribute IdentifierName? name;
//   attribute BindingIdentifier binding;
// };

struct ImportSpecifier {
    name: Option<IdentifierName>,
    binding: BindingIdentifier,
}

// `export * FromClause;`
// interface ExportAllFrom : ExportDeclaration {
//   attribute string moduleSpecifier;
// };

struct ExportAllFrom {
    moduleSpecifier: String,
}

// `export ExportClause FromClause;`
// interface ExportFrom : ExportDeclaration {
//   attribute ExportFromSpecifier[] namedExports;
//   attribute string moduleSpecifier;
// };

struct ExportFrom {
    namedExports: Vec<ExportFromSpecifier>,
    moduleSpecifier: String,
}

// `export ExportClause;`
// interface ExportLocals : ExportDeclaration {
//   attribute ExportLocalSpecifier[] namedExports;
// };

struct ExportLocals {
    namedExports: Vec<ExportLocalSpecifier>,
}

// `export VariableStatement`, `export Declaration`
// interface Export : ExportDeclaration {
//   attribute (FunctionDeclaration or ClassDeclaration or VariableDeclaration) declaration;
// };

enum Export {
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    VariableDeclaration(VariableDeclaration),
}

// `export default HoistableDeclaration`, `export default ClassDeclaration`, `export default AssignmentExpression`
// interface ExportDefault : ExportDeclaration {
//   attribute (FunctionDeclaration or ClassDeclaration or Expression) body;
// };

enum ExportDefault {
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

struct ExportFromSpecifier {
    name: IdentifierName,
    exportedName: Option<IdentifierName>,
}

// `ExportSpecifier`, as part of an `ExportLocals`.
// interface ExportLocalSpecifier : Node {
//   // The only `IdentifierName in `ExportSpecifier :: IdentifierName`, or the first in `ExportSpecifier :: IdentifierName as IdentifierName`.
//   attribute IdentifierExpression name;
//   // The second `IdentifierName` in `ExportSpecifier :: IdentifierName as IdentifierName`, if present.
//   attribute IdentifierName? exportedName;
// };

struct ExportLocalSpecifier {
    name: IdentifierExpression,
    exportedName: Option<IdentifierName>,
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

struct Method {
    isAsync: bool,
    isGenerator: bool,
    params: FormalParameters,
}

// `get PropertyName ( ) { FunctionBody }`
// interface Getter : MethodDefinition { };

struct Getter {}

// `set PropertyName ( PropertySetParameterList ) { FunctionBody }`
// interface Setter : MethodDefinition {
//   // The `PropertySetParameterList`.
//   attribute Parameter param;
// };

struct Setter {
    param: Parameter,
}

// `PropertyDefinition :: PropertyName : AssignmentExpression`
// interface DataProperty : NamedObjectProperty {
//   // The `AssignmentExpression`.
//   attribute Expression expression;
// };

struct DataProperty {
    expression: Box<Expression>,
}

// `PropertyDefinition :: IdentifierReference`
// interface ShorthandProperty : ObjectProperty {
//   // The `IdentifierReference`.
//   attribute IdentifierExpression name;
// };

struct ShorthandProperty {
    name: IdentifierExpression,
}

// interface ComputedPropertyName : PropertyName {
//   attribute Expression expression;
// };

struct ComputedPropertyName {
    expression: Box<Expression>,
}

// `LiteralPropertyName`
// interface StaticPropertyName : PropertyName {
//   attribute string value;
// };

struct StaticPropertyName {
    value: String,
}

// literals

// `BooleanLiteral`
// interface LiteralBooleanExpression : Expression {
//   attribute boolean value;
// };

struct LiteralBooleanExpression {
    value: bool,
}

// A `NumericLiteral` for which the Number value of its MV is positive infinity.
// interface LiteralInfinityExpression : Expression { };

struct LiteralInfinityExpression {}

// `NullLiteral`
// interface LiteralNullExpression : Expression { };

struct LiteralNullExpression {}

// `NumericLiteral`
// interface LiteralNumericExpression : Expression {
//   attribute double value;
// };

struct LiteralNumericExpression {
    value: f64,
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

struct LiteralRegExpExpression {
    pattern: String,
    global: bool,
    ignoreCase: bool,
    multiLine: bool,
    sticky: bool,
    unicode: bool,
}

// `StringLiteral`
// interface LiteralStringExpression : Expression {
//   attribute string value;
// };

struct LiteralStringExpression {
    value: String,
}

// other expressions

// `ArrayLiteral`
// interface ArrayExpression : Expression {
//   // The elements of the array literal; a null value represents an elision.
//   attribute (SpreadElement or Expression)?[] elements;
// };

enum ArrayExpressionElement {
    SpreadElement(SpreadElement),
    Expression(Box<Expression>),
    Elision,
}

struct ArrayExpression {
    elements: Vec<ArrayExpressionElement>,
}

// `ArrowFunction`, `AsyncArrowFunction`
// interface ArrowExpression : Expression {
//   // True for `AsyncArrowFunction`, false otherwise.
//   attribute boolean isAsync;
//   attribute FormalParameters params;
//   attribute (FunctionBody or Expression) body;
// };

enum ArrowExpressionBody {
    FunctionBody(FunctionBody),
    Expression(Box<Expression>),
}

struct ArrowExpression {
    isAsync: bool,
    params: FormalParameters,
    body: ArrowExpressionBody,
}

// `AssignmentExpression :: LeftHandSideExpression = AssignmentExpression`
// interface AssignmentExpression : Expression {
//   // The `LeftHandSideExpression`.
//   attribute AssignmentTarget binding;
//   // The `AssignmentExpression` following the `=`.
//   attribute Expression expression;
// };

struct AssignmentExpression {
    binding: AssignmentTarget,
    expression: Box<Expression>,
}

// `ExponentiationExpression`, `MultiplicativeExpression`, `AdditiveExpression`, `ShiftExpression`, `RelationalExpression`, `EqualityExpression`, `BitwiseANDExpression`, `BitwiseXORExpression`, `BitwiseORExpression`, `LogicalANDExpression`, `LogicalORExpression`
// interface BinaryExpression : Expression {
//   attribute BinaryOperator operator;
//   // The expression before the operator.
//   attribute Expression left;
//   // The expression after the operator.
//   attribute Expression right;
// };

struct BinaryExpression {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}

// interface CallExpression : Expression {
//   attribute (Expression or Super) callee;
//   attribute Arguments arguments;
// };

struct CallExpression {
    callee: ExpressionOrSuper,
    arguments: Arguments,
}

// `AssignmentExpression :: LeftHandSideExpression AssignmentOperator AssignmentExpression`
// interface CompoundAssignmentExpression : Expression {
//   attribute CompoundAssignmentOperator operator;
//   // The `LeftHandSideExpression`.
//   attribute SimpleAssignmentTarget binding;
//   // The `AssignmentExpression`.
//   attribute Expression expression;
// };

struct CompoundAssignmentExpression {
    operator: CompoundAssignmentOperator,
    binding: SimpleAssignmentTarget,
    expression: Box<Expression>,
}

// interface ComputedMemberExpression : MemberExpression {
//   // The expression resolving to the name of the property to be accessed.
//   attribute Expression expression;
// };

struct ComputedMemberExpression {
    expression: Box<Expression>,
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

struct ConditionalExpression {
    test: Box<Expression>,
    consequent: Box<Expression>,
    alternate: Box<Expression>,
}

// interface FunctionExpression : Expression {
//   attribute BindingIdentifier? name;
// };
// FunctionExpression implements Function;

struct FunctionExpression {
    name: Option<BindingIdentifier>,
    // True for `AsyncFunctionExpression` and `AsyncFunctionDeclaration`, false otherwise.
    isAsync: bool,
    // True for `GeneratorExpression` and `GeneratorDeclaration`, false otherwise.
    isGenerator: bool,
    params: FormalParameters,
    body: FunctionBody,
}

// `IdentifierReference`
// interface IdentifierExpression : Expression { };
// IdentifierExpression implements VariableReference;

// TODO
struct IdentifierExpression {
    var: VariableReference,
}

// interface NewExpression : Expression {
//   attribute Expression callee;
//   attribute Arguments arguments;
// };

struct NewExpression {
    callee: Box<Expression>,
    arguments: Arguments,
}

// interface NewTargetExpression : Expression { };

struct NewTargetExpression {}

// interface ObjectExpression : Expression {
//   attribute ObjectProperty[] properties;
// };

struct ObjectExpression {
    properties: Vec<ObjectProperty>,
}

// interface UnaryExpression : Expression {
//   attribute UnaryOperator operator;
//   attribute Expression operand;
// };

struct UnaryExpression {
    operator: UnaryOperator,
    operand: Box<Expression>,
}

// interface StaticMemberExpression : MemberExpression {
//   // The name of the property to be accessed.
//   attribute IdentifierName property;
// };

struct StaticMemberExpression {
    property: IdentifierName,
}

// `TemplateLiteral`, `MemberExpression :: MemberExpression TemplateLiteral`, `CallExpression : CallExpression TemplateLiteral`
// interface TemplateExpression : Expression {
//   // The second `MemberExpression` or `CallExpression`, if present.
//   attribute Expression? tag;
//   // The contents of the template. This list must be alternating TemplateElements and Expressions, beginning and ending with TemplateElement.
//   attribute (Expression or TemplateElement)[] elements;
// };

enum TemplateExpressionElement {
    Expression(Box<Expression>),
    TemplateElement(TemplateElement),
}

struct TemplateExpression {
    tag: Option<Box<Expression>>,
    elements: Vec<TemplateExpressionElement>,
}

// `PrimaryExpression :: this`
// interface ThisExpression : Expression { };

struct ThisExpression {}

// `UpdateExpression :: LeftHandSideExpression ++`, `UpdateExpression :: LeftHandSideExpression --`, `UpdateExpression :: ++ LeftHandSideExpression`, ``UpdateExpression :: -- LeftHandSideExpression`
// interface UpdateExpression : Expression {
//   // True for `UpdateExpression :: ++ LeftHandSideExpression` and `UpdateExpression :: -- LeftHandSideExpression`, false otherwise.
//   attribute boolean isPrefix;
//   attribute UpdateOperator operator;
//   attribute SimpleAssignmentTarget operand;
// };

struct UpdateExpression {
    isPrefix: bool,
    operator: UpdateOperator,
    operand: SimpleAssignmentTarget,
}

// `YieldExpression :: yield`, `YieldExpression :: yield AssignmentExpression`
// interface YieldExpression : Expression {
//   // The `AssignmentExpression`, if present.
//   attribute Expression? expression;
// };

struct YieldExpression {
    expression: Option<Box<Expression>>,
}

// `YieldExpression :: yield * AssignmentExpression`
//interface YieldGeneratorExpression : Expression {
//  attribute Expression expression;
//};

struct YieldGeneratorExpression {
    expression: Box<Expression>,
}

// interface AwaitExpression : Expression {
//   attribute Expression expression;
// };

struct AwaitExpression {
    expression: Box<Expression>,
}

// other statements

// interface BlockStatement : Statement {
//   attribute Block block;
// };

struct BlockStatement {
    block: Block,
}

// interface BreakStatement : Statement {
//   attribute Label? label;
// };

struct BreakStatement {
    label: Option<Label>,
}

// interface ContinueStatement : Statement {
//   attribute Label? label;
// };

struct ContinueStatement {
    label: Option<Label>,
}

// interface DebuggerStatement : Statement { };

struct DebuggerStatement {}

// interface DoWhileStatement : IterationStatement {
//   attribute Expression test;
// };

struct DoWhileStatement {
    test: Box<Expression>,
}

// interface EmptyStatement : Statement { };

struct EmptyStatement {}

//interface ExpressionStatement : Statement {
//  attribute Expression expression;
//};

struct ExpressionStatement {
    expression: Box<Expression>,
}

// `for ( LeftHandSideExpression in Expression ) Statement`, `for ( var ForBinding in Expression ) Statement`, `for ( ForDeclaration in Expression ) Statement`, `for ( var BindingIdentifier Initializer in Expression ) Statement`
// interface ForInStatement : IterationStatement {
//   // The expression or declaration before `in`.
//   attribute (VariableDeclaration or AssignmentTarget) left;
//   // The expression after `in`.
//   attribute Expression right;
// };

enum VariableDeclarationOrAssignmentTarget {
    VariableDeclaration(VariableDeclaration),
    AssignmentTarget(AssignmentTarget),
}

struct ForInStatement {
    left: VariableDeclarationOrAssignmentTarget,
    right: Box<Expression>,
}

// `for ( LeftHandSideExpression of Expression ) Statement`, `for ( var ForBinding of Expression ) Statement`, `for ( ForDeclaration of Expression ) Statement`
// interface ForOfStatement : IterationStatement {
//   // The expression or declaration before `of`.
//   attribute (VariableDeclaration or AssignmentTarget) left;
//   // The expression after `of`.
//   attribute Expression right;
// };

struct ForOfStatement {
    left: VariableDeclarationOrAssignmentTarget,
    right: Box<Expression>,
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

enum VariableDeclarationOrExpression {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<Expression>),
}

struct ForStatement {
    init: Option<VariableDeclarationOrExpression>,
    test: Option<Box<Expression>>,
    update: Option<Box<Expression>>,
}

// `if ( Expression ) Statement`, `if ( Expression ) Statement else Statement`,
// interface IfStatement : Statement {
//   attribute Expression test;
//   // The first `Statement`.
//   attribute Statement consequent;
//   // The second `Statement`, if present.
//   attribute Statement? alternate;
// };

struct IfStatement {
    test: Box<Expression>,
    consequent: Box<Statement>,
    alternate: Option<Box<Statement>>,
}

// interface LabeledStatement : Statement {
//   attribute Label label;
//   attribute Statement body;
// };

struct LabeledStatement {
    label: Label,
    body: Box<Statement>,
}

// interface ReturnStatement : Statement {
//   attribute Expression? expression;
// };

struct ReturnStatement {
    expression: Option<Box<Expression>>,
}

// A `SwitchStatement` whose `CaseBlock` is `CaseBlock :: { CaseClauses }`.
// interface SwitchStatement : Statement {
//   attribute Expression discriminant;
//   attribute SwitchCase[] cases;
// };

struct SwitchStatement {
    discriminant: Box<Expression>,
    cases: Vec<SwitchCase>,
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

struct SwitchStatementWithDefault {
    discriminant: Box<Expression>,
    preDefaultCases: Vec<SwitchCase>,
    defaultCase: SwitchDefault,
    postDefaultCases: Vec<SwitchCase>,
}

// interface ThrowStatement : Statement {
//   attribute Expression expression;
// };

struct ThrowStatement {
    expression: Box<Expression>,
}

// `TryStatement :: try Block Catch`
// interface TryCatchStatement : Statement {
//   attribute Block body;
//   attribute CatchClause catchClause;
// };

struct TryCatchStatement {
    body: Block,
    catchClause: CatchClause,
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

struct TryFinallyStatement {
    body: Block,
    catchClause: Option<CatchClause>,
    finalizer: Block,
}

// interface VariableDeclarationStatement : Statement {
//   attribute VariableDeclaration declaration;
// };

struct VariableDeclarationStatement {
    declaration: VariableDeclaration,
}

// interface WhileStatement : IterationStatement {
//   attribute Expression test;
// };

struct WhileStatement {
    test: Box<Expression>,
}

// interface WithStatement : Statement {
//   attribute Expression _object;
//   attribute Statement body;
// };

struct WithStatement {
    object: Box<Expression>,
    body: Box<Statement>,
}

// other nodes

// interface Block : Node {
//   attribute Statement[] statements;
// };

struct Block {
    statements: Vec<Statement>,
}

// `Catch`
// interface CatchClause : Node {
//   attribute Binding binding;
//   attribute Block body;
// };

struct CatchClause {
    binding: Binding,
    body: Block,
}

// An item in a `DirectivePrologue`
// interface Directive : Node {
//   attribute string rawValue;
// };

struct Directive {
    rawValue: String,
}

// interface FormalParameters : Node {
//   attribute Parameter[] items;
//   attribute Binding? rest;
// };

struct FormalParameters {
    items: Vec<Parameter>,
    rest: Option<Binding>,
}

// interface FunctionBody : Node {
//   attribute Directive[] directives;
//   attribute Statement[] statements;
// };

struct FunctionBody {
    directives: Vec<Directive>,
    statements: Vec<Statement>,
}

// interface FunctionDeclaration : Statement {
//   attribute BindingIdentifier name;
// };
// FunctionDeclaration implements Function;

struct FunctionDeclaration {
    name: BindingIdentifier,
    // True for `AsyncFunctionExpression` and `AsyncFunctionDeclaration`, false otherwise.
    isAsync: bool,
    // True for `GeneratorExpression` and `GeneratorDeclaration`, false otherwise.
    isGenerator: bool,
    params: FormalParameters,
    body: FunctionBody,
}

// interface Script : Program {
//   attribute Directive[] directives;
//   attribute Statement[] statements;
// };

struct Script {
    directives: Vec<Directive>,
    statements: Vec<Statement>,
}

// interface SpreadElement : Node {
//   attribute Expression expression;
// };

struct SpreadElement {
    expression: Box<Expression>,
}

// `super`
// interface Super : Node { };

struct Super {}

// `CaseClause`
// interface SwitchCase : Node {
//   attribute Expression test;
//   attribute Statement[] consequent;
// };

struct SwitchCase {
    test: Box<Expression>,
    consequent: Vec<Statement>,
}

// `DefaultClause`
// interface SwitchDefault : Node {
//   attribute Statement[] consequent;
// };

struct SwitchDefault {
    consequent: Vec<Statement>,
}

// `TemplateCharacters`
// interface TemplateElement : Node {
//   attribute string rawValue;
// };

struct TemplateElement {
    rawValue: String,
}

// interface VariableDeclaration : Node {
//   attribute VariableDeclarationKind kind;
//   [NonEmpty] attribute VariableDeclarator[] declarators;
// };

struct VariableDeclaration {
    kind: VariableDeclarationKind,
    declarators: Vec<VariableDeclarator>,
}

// interface VariableDeclarator : Node {
//   attribute Binding binding;
//   attribute Expression? init;
// };

struct VariableDeclarator {
    binding: Binding,
    init: Option<Box<Expression>>,
}
