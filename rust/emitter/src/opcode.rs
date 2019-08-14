pub struct Opcode {
    pub value: u8,
    pub name: &'static str,
}

// impl Opcode {
//     fn new(value: u8, name: &'static str) -> Self {
//         Self { value, name }
//     }
// }

pub const TABLE: [Opcode; 241] = [
    NOP,
    UNDEFINED,
    GETRVAL,
    ENTERWITH,
    LEAVEWITH,
    RETURN,
    GOTO,
    IFEQ,
    IFNE,
    ARGUMENTS,
    SWAP,
    POPN,
    DUP,
    DUP2,
    CHECKISOBJ,
    BITOR,
    BITXOR,
    BITAND,
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
    LSH,
    RSH,
    URSH,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    NOT,
    BITNOT,
    NEG,
    POS,
    DELNAME,
    DELPROP,
    DELELEM,
    TYPEOF,
    VOID,
    SPREADCALL,
    SPREADNEW,
    SPREADEVAL,
    DUPAT,
    SYMBOL,
    STRICTDELPROP,
    STRICTDELELEM,
    STRICTSETPROP,
    STRICTSETNAME,
    STRICTSPREADEVAL,
    CHECKCLASSHERITAGE,
    FUNWITHPROTO,
    GETPROP,
    SETPROP,
    GETELEM,
    SETELEM,
    STRICTSETELEM,
    CALL,
    GETNAME,
    DOUBLE,
    STRING,
    ZERO,
    ONE,
    NULL,
    IS_CONSTRUCTING,
    FALSE,
    TRUE,
    OR,
    AND,
    TABLESWITCH,
    UNUSED71,
    STRICTEQ,
    STRICTNE,
    THROWMSG,
    ITER,
    MOREITER,
    ISNOITER,
    ENDITER,
    FUNAPPLY,
    OBJECT,
    POP,
    NEW,
    OBJWITHPROTO,
    GETARG,
    SETARG,
    GETLOCAL,
    SETLOCAL,
    UINT16,
    NEWINIT,
    NEWARRAY,
    NEWOBJECT,
    INITHOMEOBJECT,
    INITPROP,
    INITELEM,
    INITELEM_INC,
    INITELEM_ARRAY,
    INITPROP_GETTER,
    INITPROP_SETTER,
    INITELEM_GETTER,
    INITELEM_SETTER,
    CALLSITEOBJ,
    NEWARRAY_COPYONWRITE,
    SUPERBASE,
    GETPROP_SUPER,
    STRICTSETPROP_SUPER,
    LABEL,
    SETPROP_SUPER,
    FUNCALL,
    LOOPHEAD,
    BINDNAME,
    SETNAME,
    THROW,
    IN,
    INSTANCEOF,
    DEBUGGER,
    GOSUB,
    RETSUB,
    EXCEPTION,
    LINENO,
    CONDSWITCH,
    CASE,
    DEFAULT,
    EVAL,
    STRICTEVAL,
    GETELEM_SUPER,
    RESUMEINDEX,
    DEFFUN,
    DEFCONST,
    DEFVAR,
    LAMBDA,
    LAMBDA_ARROW,
    CALLEE,
    PICK,
    TRY,
    FINALLY,
    GETALIASEDVAR,
    SETALIASEDVAR,
    CHECKLEXICAL,
    INITLEXICAL,
    CHECKALIASEDLEXICAL,
    INITALIASEDLEXICAL,
    UNINITIALIZED,
    GETINTRINSIC,
    SETINTRINSIC,
    CALLITER,
    INITLOCKEDPROP,
    INITHIDDENPROP,
    NEWTARGET,
    UNUSED149,
    POW,
    ASYNCAWAIT,
    SETRVAL,
    RETRVAL,
    GETGNAME,
    SETGNAME,
    STRICTSETGNAME,
    GIMPLICITTHIS,
    SETELEM_SUPER,
    STRICTSETELEM_SUPER,
    REGEXP,
    INITGLEXICAL,
    DEFLET,
    CHECKOBJCOERCIBLE,
    SUPERFUN,
    SUPERCALL,
    SPREADSUPERCALL,
    CLASSCONSTRUCTOR,
    DERIVEDCONSTRUCTOR,
    THROWSETCONST,
    THROWSETALIASEDCONST,
    INITHIDDENPROP_GETTER,
    INITHIDDENPROP_SETTER,
    INITHIDDENELEM_GETTER,
    INITHIDDENELEM_SETTER,
    INITHIDDENELEM,
    GETIMPORT,
    DEBUGCHECKSELFHOSTED,
    OPTIMIZE_SPREADCALL,
    THROWSETCALLEE,
    PUSHVARENV,
    POPVARENV,
    SETFUNNAME,
    UNPICK,
    CALLPROP,
    FUNCTIONTHIS,
    GLOBALTHIS,
    ISGENCLOSING,
    UINT24,
    CHECKTHIS,
    CHECKRETURN,
    CHECKTHISREINIT,
    ASYNCRESOLVE,
    CALLELEM,
    MUTATEPROTO,
    GETBOUNDNAME,
    TYPEOFEXPR,
    FRESHENLEXICALENV,
    RECREATELEXICALENV,
    PUSHLEXICALENV,
    POPLEXICALENV,
    DEBUGLEAVELEXICALENV,
    INITIALYIELD,
    YIELD,
    FINALYIELDRVAL,
    RESUME,
    ENVCALLEE,
    FORCEINTERPRETER,
    AFTERYIELD,
    AWAIT,
    TOASYNCITER,
    HASOWN,
    GENERATOR,
    BINDVAR,
    BINDGNAME,
    INT8,
    INT32,
    LENGTH,
    HOLE,
    CHECKISCALLABLE,
    TRY_DESTRUCTURING,
    BUILTINPROTO,
    ITERNEXT,
    TRYSKIPAWAIT,
    REST,
    TOID,
    IMPLICITTHIS,
    LOOPENTRY,
    TOSTRING,
    NOP_DESTRUCTURING,
    JUMPTARGET,
    CALL_IGNORES_RV,
    IMPORTMETA,
    DYNAMIC_IMPORT,
    INC,
    DEC,
    TONUMERIC,
    BIGINT,
    INSTRUMENTATION_ACTIVE,
    INSTRUMENTATION_CALLBACK,
    INSTRUMENTATION_SCRIPT_ID,
];

pub const js_undefined_str: &str = "undefined";
pub const js_typeof_str: &str = "typeof";
pub const js_void_str: &str = "void";
pub const js_null_str: &str = "null";
pub const js_false_str: &str = "false";
pub const js_true_str: &str = "true";
pub const js_throw_str: &str = "throw";
pub const js_in_str: &str = "in";
pub const js_instanceof_str: &str = "instanceof";

/*
 * No operation is performed.
 *
 *   Category: Other
 *   Operands:
 *   Stack: =>
 */
pub const NOP: Opcode = Opcode {
    value: 0,
    name: "nop",
};
/*
 * Pushes 'undefined' onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands:
 *   Stack: => undefined
 */
pub const UNDEFINED: Opcode = Opcode {
    value: 1,
    name: js_undefined_str,
};
/*
 * Pushes stack frame's 'rval' onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: => rval
 */
pub const GETRVAL: Opcode = Opcode {
    value: 2,
    name: "getrval",
};
/*
 * Pops the top of stack value, converts it to an object, and adds a
 * 'WithEnvironmentObject' wrapping that object to the environment chain.
 *
 * There is a matching JSOP_LEAVEWITH instruction later. All name
 * lookups between the two that may need to consult the With object
 * are deoptimized.
 *
 *   Category: Statements
 *   Type: With Statement
 *   Operands: uint32_t staticWithIndex
 *   Stack: val =>
 */
pub const ENTERWITH: Opcode = Opcode {
    value: 3,
    name: "enterwith",
};
/*
 * Pops the environment chain object pushed by JSOP_ENTERWITH.
 *
 *   Category: Statements
 *   Type: With Statement
 *   Operands:
 *   Stack: =>
 */
pub const LEAVEWITH: Opcode = Opcode {
    value: 4,
    name: "leavewith",
};
/*
 * Pops the top of stack value as 'rval', stops interpretation of current
 * script and returns 'rval'.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: rval =>
 */
pub const RETURN: Opcode = Opcode {
    value: 5,
    name: "return",
};
/*
 * Jumps to a 32-bit offset from the current bytecode.
 *
 *   Category: Statements
 *   Type: Jumps
 *   Operands: int32_t offset
 *   Stack: =>
 */
pub const GOTO: Opcode = Opcode {
    value: 6,
    name: "goto",
};
/*
 * Pops the top of stack value, converts it into a boolean, if the result
 * is 'false', jumps to a 32-bit offset from the current bytecode.
 *
 * The idea is that a sequence like
 * JSOP_ZERO; JSOP_ZERO; JSOP_EQ; JSOP_IFEQ; JSOP_RETURN;
 * reads like a nice linear sequence that will execute the return.
 *
 *   Category: Statements
 *   Type: Jumps
 *   Operands: int32_t offset
 *   Stack: cond =>
 */
pub const IFEQ: Opcode = Opcode {
    value: 7,
    name: "ifeq",
};
/*
 * Pops the top of stack value, converts it into a boolean, if the result
 * is 'true', jumps to a 32-bit offset from the current bytecode.
 *
 *   Category: Statements
 *   Type: Jumps
 *   Operands: int32_t offset
 *   Stack: cond =>
 */
pub const IFNE: Opcode = Opcode {
    value: 8,
    name: "ifne",
};
/*
 * Pushes the 'arguments' object for the current function activation.
 *
 * If 'JSScript' is not marked 'needsArgsObj', then a
 * JS_OPTIMIZED_ARGUMENTS magic value is pushed. Otherwise, a proper
 * arguments object is constructed and pushed.
 *
 * This opcode requires that the function does not have rest parameter.
 *
 *   Category: Variables and Scopes
 *   Type: Arguments
 *   Operands:
 *   Stack: => arguments
 */
pub const ARGUMENTS: Opcode = Opcode {
    value: 9,
    name: "arguments",
};
/*
 * Swaps the top two values on the stack. This is useful for things like
 * post-increment/decrement.
 *
 *   Category: Operators
 *   Type: Stack Operations
 *   Operands:
 *   Stack: v1, v2 => v2, v1
 */
pub const SWAP: Opcode = Opcode {
    value: 10,
    name: "swap",
};
/*
 * Pops the top 'n' values from the stack.
 *
 *   Category: Operators
 *   Type: Stack Operations
 *   Operands: uint16_t n
 *   Stack: v[n-1], ..., v[1], v[0] =>
 *   nuses: n
 */
pub const POPN: Opcode = Opcode {
    value: 11,
    name: "popn",
};
/*
 * Pushes a copy of the top value on the stack.
 *
 *   Category: Operators
 *   Type: Stack Operations
 *   Operands:
 *   Stack: v => v, v
 */
pub const DUP: Opcode = Opcode {
    value: 12,
    name: "dup",
};
/*
 * Duplicates the top two values on the stack.
 *
 *   Category: Operators
 *   Type: Stack Operations
 *   Operands:
 *   Stack: v1, v2 => v1, v2, v1, v2
 */
pub const DUP2: Opcode = Opcode {
    value: 13,
    name: "dup2",
};
/*
 * Checks that the top value on the stack is an object, and throws a
 * TypeError if not. The operand 'kind' is used only to generate an
 * appropriate error message.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands: uint8_t kind
 *   Stack: result => result
 */
pub const CHECKISOBJ: Opcode = Opcode {
    value: 14,
    name: "checkisobj",
};
/*
 * Pops the top two values 'lval' and 'rval' from the stack, then pushes
 * the result of the operation applied to the two operands, converting both
 * to 32-bit signed integers if necessary.
 *
 *   Category: Operators
 *   Type: Bitwise Logical Operators
 *   Operands:
 *   Stack: lval, rval => (lval OP rval)
 */
pub const BITOR: Opcode = Opcode {
    value: 15,
    name: "bitor",
};
pub const BITXOR: Opcode = Opcode {
    value: 16,
    name: "bitxor",
};
pub const BITAND: Opcode = Opcode {
    value: 17,
    name: "bitand",
};
/*
 * Pops the top two values from the stack and pushes the result of
 * comparing them.
 *
 *   Category: Operators
 *   Type: Comparison Operators
 *   Operands:
 *   Stack: lval, rval => (lval OP rval)
 */
pub const EQ: Opcode = Opcode {
    value: 18,
    name: "eq",
};
pub const NE: Opcode = Opcode {
    value: 19,
    name: "ne",
};
pub const LT: Opcode = Opcode {
    value: 20,
    name: "lt",
};
pub const LE: Opcode = Opcode {
    value: 21,
    name: "le",
};
pub const GT: Opcode = Opcode {
    value: 22,
    name: "gt",
};
pub const GE: Opcode = Opcode {
    value: 23,
    name: "ge",
};
/*
 * Pops the top two values 'lval' and 'rval' from the stack, then pushes
 * the result of the operation applied to the operands.
 *
 *   Category: Operators
 *   Type: Bitwise Shift Operators
 *   Operands:
 *   Stack: lval, rval => (lval OP rval)
 */
pub const LSH: Opcode = Opcode {
    value: 24,
    name: "lsh",
};
pub const RSH: Opcode = Opcode {
    value: 25,
    name: "rsh",
};
/*
 * Pops the top two values 'lval' and 'rval' from the stack, then pushes
 * 'lval >>> rval'.
 *
 *   Category: Operators
 *   Type: Bitwise Shift Operators
 *   Operands:
 *   Stack: lval, rval => (lval >>> rval)
 */
pub const URSH: Opcode = Opcode {
    value: 26,
    name: "ursh",
};
/*
 * Pops the top two values 'lval' and 'rval' from the stack, then pushes
 * the result of 'lval + rval'.
 *
 *   Category: Operators
 *   Type: Arithmetic Operators
 *   Operands:
 *   Stack: lval, rval => (lval + rval)
 */
pub const ADD: Opcode = Opcode {
    value: 27,
    name: "add",
};
/*
 * Pops the top two values 'lval' and 'rval' from the stack, then pushes
 * the result of applying the arithmetic operation to them.
 *
 *   Category: Operators
 *   Type: Arithmetic Operators
 *   Operands:
 *   Stack: lval, rval => (lval OP rval)
 */
pub const SUB: Opcode = Opcode {
    value: 28,
    name: "sub",
};
pub const MUL: Opcode = Opcode {
    value: 29,
    name: "mul",
};
pub const DIV: Opcode = Opcode {
    value: 30,
    name: "div",
};
pub const MOD: Opcode = Opcode {
    value: 31,
    name: "mod",
};
/*
 * Pops the value 'val' from the stack, then pushes '!val'.
 *
 *   Category: Operators
 *   Type: Logical Operators
 *   Operands:
 *   Stack: val => (!val)
 */
pub const NOT: Opcode = Opcode {
    value: 32,
    name: "not",
};
/*
 * Pops the value 'val' from the stack, then pushes '~val'.
 *
 *   Category: Operators
 *   Type: Bitwise Logical Operators
 *   Operands:
 *   Stack: val => (~val)
 */
pub const BITNOT: Opcode = Opcode {
    value: 33,
    name: "bitnot",
};
/*
 * Pops the value 'val' from the stack, then pushes '-val'.
 *
 *   Category: Operators
 *   Type: Arithmetic Operators
 *   Operands:
 *   Stack: val => (-val)
 */
pub const NEG: Opcode = Opcode {
    value: 34,
    name: "neg",
};
/*
 * Pops the value 'val' from the stack, then pushes '+val'.
 * ('+val' is the value converted to a number.)
 *
 *   Category: Operators
 *   Type: Arithmetic Operators
 *   Operands:
 *   Stack: val => (+val)
 */
pub const POS: Opcode = Opcode {
    value: 35,
    name: "pos",
};
/*
 * Looks up name on the environment chain and deletes it, pushes 'true'
 * onto the stack if succeeded (if the property was present and deleted or
 * if the property wasn't present in the first place), 'false' if not.
 *
 * Strict mode code should never contain this opcode.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: => succeeded
 */
pub const DELNAME: Opcode = Opcode {
    value: 36,
    name: "delname",
};
/*
 * Pops the top of stack value, deletes property from it, pushes 'true'
 * onto the stack if succeeded, 'false' if not.
 *
 *   Category: Operators
 *   Type: Special Operators
 *   Operands: uint32_t nameIndex
 *   Stack: obj => succeeded
 */
pub const DELPROP: Opcode = Opcode {
    value: 37,
    name: "delprop",
};
/*
 * Pops the top two values on the stack as 'propval' and 'obj', deletes
 * 'propval' property from 'obj', pushes 'true'  onto the stack if
 * succeeded, 'false' if not.
 *
 *   Category: Operators
 *   Type: Special Operators
 *   Operands:
 *   Stack: obj, propval => succeeded
 */
pub const DELELEM: Opcode = Opcode {
    value: 38,
    name: "delelem",
};
/*
 * Pops the value 'val' from the stack, then pushes 'typeof val'.
 *
 *   Category: Operators
 *   Type: Special Operators
 *   Operands:
 *   Stack: val => (typeof val)
 */
pub const TYPEOF: Opcode = Opcode {
    value: 39,
    name: js_typeof_str,
};
/*
 * Pops the top value on the stack and pushes 'undefined'.
 *
 *   Category: Operators
 *   Type: Special Operators
 *   Operands:
 *   Stack: val => undefined
 */
pub const VOID: Opcode = Opcode {
    value: 40,
    name: js_void_str,
};
/*
 * spreadcall variant of JSOP_CALL.
 *
 * Invokes 'callee' with 'this' and 'args', pushes the return value onto
 * the stack.
 *
 * 'args' is an Array object which contains actual arguments.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: callee, this, args => rval
 */
pub const SPREADCALL: Opcode = Opcode {
    value: 41,
    name: "spreadcall",
};
/*
 * spreadcall variant of JSOP_NEW
 *
 * Invokes 'callee' as a constructor with 'this' and 'args', pushes the
 * return value onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: callee, this, args, newTarget => rval
 */
pub const SPREADNEW: Opcode = Opcode {
    value: 42,
    name: "spreadnew",
};
/*
 * spreadcall variant of JSOP_EVAL
 *
 * Invokes 'eval' with 'args' and pushes the return value onto the stack.
 *
 * If 'eval' in global scope is not original one, invokes the function with
 * 'this' and 'args', and pushes return value onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: callee, this, args => rval
 */
pub const SPREADEVAL: Opcode = Opcode {
    value: 43,
    name: "spreadeval",
};
/*
 * Duplicates the Nth value from the top onto the stack.
 *
 *   Category: Operators
 *   Type: Stack Operations
 *   Operands: uint24_t n
 *   Stack: v[n], v[n-1], ..., v[1], v[0] =>
 *          v[n], v[n-1], ..., v[1], v[0], v[n]
 */
pub const DUPAT: Opcode = Opcode {
    value: 44,
    name: "dupat",
};
/*
 * Push a well-known symbol onto the operand stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands: uint8_t symbol (the JS::SymbolCode of the symbol to use)
 *   Stack: => symbol
 */
pub const SYMBOL: Opcode = Opcode {
    value: 45,
    name: "symbol",
};
/*
 * Pops the top of stack value and attempts to delete the given property
 * from it. Pushes 'true' onto success, else throws a TypeError per strict
 * mode property-deletion requirements.
 *
 *   Category: Operators
 *   Type: Special Operators
 *   Operands: uint32_t nameIndex
 *   Stack: obj => succeeded
 */
pub const STRICTDELPROP: Opcode = Opcode {
    value: 46,
    name: "strict-delprop",
};
/*
 * Pops the top two values on the stack as 'propval' and 'obj', and
 * attempts to delete 'propval' property from 'obj'. Pushes 'true' onto the
 * stack on success, else throws a TypeError per strict mode property
 * deletion requirements.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, propval => succeeded
 */
pub const STRICTDELELEM: Opcode = Opcode {
    value: 47,
    name: "strict-delelem",
};
/*
 * Pops the top two values on the stack as 'val' and 'obj', and performs
 * 'obj.prop = val', pushing 'val' back onto the stack. Throws a TypeError
 * if the set-operation failed (per strict mode semantics).
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => val
 */
pub const STRICTSETPROP: Opcode = Opcode {
    value: 48,
    name: "strict-setprop",
};
/*
 * Pops a environment and value from the stack, assigns value to the given
 * name, and pushes the value back on the stack. If the set failed, then
 * throw a TypeError, per usual strict mode semantics.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: env, val => val
 */
pub const STRICTSETNAME: Opcode = Opcode {
    value: 49,
    name: "strict-setname",
};
/*
 * spreadcall variant of JSOP_EVAL
 *
 * Invokes 'eval' with 'args' and pushes the return value onto the stack.
 *
 * If 'eval' in global scope is not original one, invokes the function with
 * 'this' and 'args', and pushes return value onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: callee, this, args => rval
 */
pub const STRICTSPREADEVAL: Opcode = Opcode {
    value: 50,
    name: "strict-spreadeval",
};
/*
 * Ensures the result of a class's heritage expression is either null or a
 * constructor.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: heritage => heritage
 */
pub const CHECKCLASSHERITAGE: Opcode = Opcode {
    value: 51,
    name: "checkclassheritage",
};
/*
 * Pushes a clone of a function with a given [[Prototype]] onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint32_t funcIndex
 *   Stack: proto => obj
 */
pub const FUNWITHPROTO: Opcode = Opcode {
    value: 52,
    name: "funwithproto",
};
/*
 * Pops the top of stack value, pushes property of it onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj => obj[name]
 */
pub const GETPROP: Opcode = Opcode {
    value: 53,
    name: "getprop",
};
/*
 * Pops the top two values on the stack as 'val' and 'obj' and performs
 * 'obj.prop = val', pushing 'val' back onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => val
 */
pub const SETPROP: Opcode = Opcode {
    value: 54,
    name: "setprop",
};
/*
 * Pops the top two values on the stack as 'propval' and 'obj', pushes
 * 'propval' property of 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, propval => obj[propval]
 */
pub const GETELEM: Opcode = Opcode {
    value: 55,
    name: "getelem",
};
/*
 * Pops the top three values on the stack as 'val', 'propval' and 'obj',
 * sets 'propval' property of 'obj' as 'val', pushes 'val' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, propval, val => val
 */
pub const SETELEM: Opcode = Opcode {
    value: 56,
    name: "setelem",
};
/*
 * Pops the top three values on the stack as 'val', 'propval' and 'obj',
 * sets 'propval' property of 'obj' as 'val', pushes 'val' onto the stack.
 * Throws a TypeError if the set fails, per strict mode semantics.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, propval, val => val
 */
pub const STRICTSETELEM: Opcode = Opcode {
    value: 57,
    name: "strict-setelem",
};
/*
 * Invokes 'callee' with 'this' and 'args', pushes return value onto the
 * stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc
 *   Stack: callee, this, args[0], ..., args[argc-1] => rval
 *   nuses: (argc+2)
 */
pub const CALL: Opcode = Opcode {
    value: 58,
    name: "call",
};
/*
 * Looks up name on the environment chain and pushes its value onto the
 * stack.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: => val
 */
pub const GETNAME: Opcode = Opcode {
    value: 59,
    name: "getname",
};
/*
 * Pushes numeric constant onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands: DoubleValue literal
 *   Stack: => val
 */
pub const DOUBLE: Opcode = Opcode {
    value: 60,
    name: "double",
};
/*
 * Pushes string constant onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands: uint32_t atomIndex
 *   Stack: => atom
 */
pub const STRING: Opcode = Opcode {
    value: 61,
    name: "string",
};
/*
 * Pushes '0' onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands:
 *   Stack: => 0
 */
pub const ZERO: Opcode = Opcode {
    value: 62,
    name: "zero",
};
/*
 * Pushes '1' onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands:
 *   Stack: => 1
 */
pub const ONE: Opcode = Opcode {
    value: 63,
    name: "one",
};
/*
 * Pushes 'null' onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands:
 *   Stack: => null
 */
pub const NULL: Opcode = Opcode {
    value: 64,
    name: js_null_str,
};
/*
 * Pushes 'JS_IS_CONSTRUCTING'
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands:
 *   Stack: => JS_IS_CONSTRUCTING
 */
pub const IS_CONSTRUCTING: Opcode = Opcode {
    value: 65,
    name: "is-constructing",
};
/*
 * Pushes boolean value onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands:
 *   Stack: => true/false
 */
pub const FALSE: Opcode = Opcode {
    value: 66,
    name: js_false_str,
};
pub const TRUE: Opcode = Opcode {
    value: 67,
    name: js_true_str,
};
/*
 * Converts the top of stack value into a boolean, if the result is 'true',
 * jumps to a 32-bit offset from the current bytecode.
 *
 *   Category: Statements
 *   Type: Jumps
 *   Operands: int32_t offset
 *   Stack: cond => cond
 */
pub const OR: Opcode = Opcode {
    value: 68,
    name: "or",
};
/*
 * Converts the top of stack value into a boolean, if the result is
 * 'false', jumps to a 32-bit offset from the current bytecode.
 *
 *   Category: Statements
 *   Type: Jumps
 *   Operands: int32_t offset
 *   Stack: cond => cond
 */
pub const AND: Opcode = Opcode {
    value: 69,
    name: "and",
};
/*
 * Pops the top of stack value as 'i', if 'low <= i <= high',
 * jumps to a 32-bit offset: offset is stored in the script's resumeOffsets
 *                           list at index 'firstResumeIndex + (i - low)'
 * jumps to a 32-bit offset: 'len' from the current bytecode otherwise
 *
 *   Category: Statements
 *   Type: Switch Statement
 *   Operands: int32_t len, int32_t low, int32_t high,
 *             uint24_t firstResumeIndex
 *   Stack: i =>
 *   len: len
 */
pub const TABLESWITCH: Opcode = Opcode {
    value: 70,
    name: "tableswitch",
};
/*
 */
pub const UNUSED71: Opcode = Opcode {
    value: 71,
    name: "unused71",
};
/*
 * Pops the top two values from the stack, then pushes the result of
 * applying the operator to the two values.
 *
 *   Category: Operators
 *   Type: Comparison Operators
 *   Operands:
 *   Stack: lval, rval => (lval OP rval)
 */
pub const STRICTEQ: Opcode = Opcode {
    value: 72,
    name: "stricteq",
};
pub const STRICTNE: Opcode = Opcode {
    value: 73,
    name: "strictne",
};
/*
 * Sometimes we know when emitting that an operation will always throw.
 *
 * Throws the indicated JSMSG.
 *
 *   Category: Statements
 *   Type: Exception Handling
 *   Operands: uint16_t msgNumber
 *   Stack: =>
 */
pub const THROWMSG: Opcode = Opcode {
    value: 74,
    name: "throwmsg",
};
/*
 * Sets up a for-in loop. It pops the top of stack value as 'val' and
 * pushes 'iter' which is an iterator for 'val'.
 *
 *   Category: Statements
 *   Type: For-In Statement
 *   Operands:
 *   Stack: val => iter
 */
pub const ITER: Opcode = Opcode {
    value: 75,
    name: "iter",
};
/*
 * Pushes the next iterated value onto the stack. If no value is available,
 * MagicValue(JS_NO_ITER_VALUE) is pushed.
 *
 *   Category: Statements
 *   Type: For-In Statement
 *   Operands:
 *   Stack: iter => iter, val
 */
pub const MOREITER: Opcode = Opcode {
    value: 76,
    name: "moreiter",
};
/*
 * Pushes a boolean indicating whether the value on top of the stack is
 * MagicValue(JS_NO_ITER_VALUE).
 *
 *   Category: Statements
 *   Type: For-In Statement
 *   Operands:
 *   Stack: val => val, res
 */
pub const ISNOITER: Opcode = Opcode {
    value: 77,
    name: "isnoiter",
};
/*
 * Exits a for-in loop by popping the iterator object from the stack and
 * closing it.
 *
 *   Category: Statements
 *   Type: For-In Statement
 *   Operands:
 *   Stack: iter =>
 */
pub const ENDITER: Opcode = Opcode {
    value: 78,
    name: "enditer",
};
/*
 * Invokes 'callee' with 'this' and 'args', pushes return value onto the
 * stack.
 *
 * This is for 'f.apply'.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc
 *   Stack: callee, this, args[0], ..., args[argc-1] => rval
 *   nuses: (argc+2)
 */
pub const FUNAPPLY: Opcode = Opcode {
    value: 79,
    name: "funapply",
};
/*
 * Pushes deep-cloned object literal or singleton onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t objectIndex
 *   Stack: => obj
 */
pub const OBJECT: Opcode = Opcode {
    value: 80,
    name: "object",
};
/*
 * Pops the top value off the stack.
 *
 *   Category: Operators
 *   Type: Stack Operations
 *   Operands:
 *   Stack: v =>
 */
pub const POP: Opcode = Opcode {
    value: 81,
    name: "pop",
};
/*
 * Invokes 'callee' as a constructor with 'this' and 'args', pushes return
 * value onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc
 *   Stack: callee, this, args[0], ..., args[argc-1], newTarget => rval
 *   nuses: (argc+3)
 */
pub const NEW: Opcode = Opcode {
    value: 82,
    name: "new",
};
/*
 * Pushes newly created object onto the stack with provided [[Prototype]].
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: proto => obj
 */
pub const OBJWITHPROTO: Opcode = Opcode {
    value: 83,
    name: "objwithproto",
};
/*
 * Fast get op for function arguments and local variables.
 *
 * Pushes 'arguments[argno]' onto the stack.
 *
 *   Category: Variables and Scopes
 *   Type: Arguments
 *   Operands: uint16_t argno
 *   Stack: => arguments[argno]
 */
pub const GETARG: Opcode = Opcode {
    value: 84,
    name: "getarg",
};
/*
 * Fast set op for function arguments and local variables.
 *
 * Sets 'arguments[argno]' as the top of stack value.
 *
 *   Category: Variables and Scopes
 *   Type: Arguments
 *   Operands: uint16_t argno
 *   Stack: v => v
 */
pub const SETARG: Opcode = Opcode {
    value: 85,
    name: "setarg",
};
/*
 * Pushes the value of local variable onto the stack.
 *
 *   Category: Variables and Scopes
 *   Type: Local Variables
 *   Operands: uint24_t localno
 *   Stack: => val
 */
pub const GETLOCAL: Opcode = Opcode {
    value: 86,
    name: "getlocal",
};
/*
 * Stores the top stack value to the given local.
 *
 *   Category: Variables and Scopes
 *   Type: Local Variables
 *   Operands: uint24_t localno
 *   Stack: v => v
 */
pub const SETLOCAL: Opcode = Opcode {
    value: 87,
    name: "setlocal",
};
/*
 * Pushes unsigned 16-bit int immediate integer operand onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands: uint16_t val
 *   Stack: => val
 */
pub const UINT16: Opcode = Opcode {
    value: 88,
    name: "uint16",
};
/*
 * Pushes newly created object onto the stack.
 *
 * This opcode has four extra bytes so it can be exchanged with
 * JSOP_NEWOBJECT during emit.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: (uint32_t extra)
 *   Stack: => obj
 */
pub const NEWINIT: Opcode = Opcode {
    value: 89,
    name: "newinit",
};
/*
 * Pushes newly created array onto the stack.
 *
 * This opcode takes the final length, which is preallocated.
 *
 *   Category: Literals
 *   Type: Array
 *   Operands: uint32_t length
 *   Stack: => obj
 */
pub const NEWARRAY: Opcode = Opcode {
    value: 90,
    name: "newarray",
};
/*
 * Pushes newly created object onto the stack.
 *
 * This opcode takes an object with the final shape, which can be set at
 * the start and slots then filled in directly.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t baseobjIndex
 *   Stack: => obj
 */
pub const NEWOBJECT: Opcode = Opcode {
    value: 91,
    name: "newobject",
};
/*
 * Initialize the home object for functions with super bindings.
 *
 * This opcode takes the function and the object to be the home object,
 * does the set, and leaves the function on the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: fun, homeObject => fun
 */
pub const INITHOMEOBJECT: Opcode = Opcode {
    value: 92,
    name: "inithomeobject",
};
/*
 * Initialize a named property in an object literal, like '{a: x}'.
 *
 * Pops the top two values on the stack as 'val' and 'obj', defines
 * 'nameIndex' property of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => obj
 */
pub const INITPROP: Opcode = Opcode {
    value: 93,
    name: "initprop",
};
/*
 * Initialize a numeric property in an object literal, like '{1: x}'.
 *
 * Pops the top three values on the stack as 'val', 'id' and 'obj', defines
 * 'id' property of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, id, val => obj
 */
pub const INITELEM: Opcode = Opcode {
    value: 94,
    name: "initelem",
};
/*
 * Pops the top three values on the stack as 'val', 'index' and 'obj', sets
 * 'index' property of 'obj' as 'val', pushes 'obj' and 'index + 1' onto
 * the stack.
 *
 * This opcode is used in Array literals with spread and spreadcall
 * arguments.
 *
 *   Category: Literals
 *   Type: Array
 *   Operands:
 *   Stack: obj, index, val => obj, (index + 1)
 */
pub const INITELEM_INC: Opcode = Opcode {
    value: 95,
    name: "initelem_inc",
};
/*
 * Initialize an array element.
 *
 * Pops the top two values on the stack as 'val' and 'obj', sets 'index'
 * property of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Array
 *   Operands: uint32_t index
 *   Stack: obj, val => obj
 */
pub const INITELEM_ARRAY: Opcode = Opcode {
    value: 96,
    name: "initelem_array",
};
/*
 * Initialize a getter in an object literal.
 *
 * Pops the top two values on the stack as 'val' and 'obj', defines getter
 * of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => obj
 */
pub const INITPROP_GETTER: Opcode = Opcode {
    value: 97,
    name: "initprop_getter",
};
/*
 * Initialize a setter in an object literal.
 *
 * Pops the top two values on the stack as 'val' and 'obj', defines setter
 * of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => obj
 */
pub const INITPROP_SETTER: Opcode = Opcode {
    value: 98,
    name: "initprop_setter",
};
/*
 * Initialize a numeric getter in an object literal like
 * '{get 2() {}}'.
 *
 * Pops the top three values on the stack as 'val', 'id' and 'obj', defines
 * 'id' getter of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, id, val => obj
 */
pub const INITELEM_GETTER: Opcode = Opcode {
    value: 99,
    name: "initelem_getter",
};
/*
 * Initialize a numeric setter in an object literal like
 * '{set 2(v) {}}'.
 *
 * Pops the top three values on the stack as 'val', 'id' and 'obj', defines
 * 'id' setter of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, id, val => obj
 */
pub const INITELEM_SETTER: Opcode = Opcode {
    value: 100,
    name: "initelem_setter",
};
/*
 * Pushes the call site object specified by objectIndex onto the stack.
 * Defines the raw property specified by objectIndex + 1 on the call site
 * object and freezes both the call site object as well as its raw
 * property.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t objectIndex
 *   Stack: => obj
 */
pub const CALLSITEOBJ: Opcode = Opcode {
    value: 101,
    name: "callsiteobj",
};
/*
 * Pushes a newly created array onto the stack, whose elements are the same
 * as that of a template object's copy on write elements.
 *
 *   Category: Literals
 *   Type: Array
 *   Operands: uint32_t objectIndex
 *   Stack: => obj
 */
pub const NEWARRAY_COPYONWRITE: Opcode = Opcode {
    value: 102,
    name: "newarray_copyonwrite",
};
/*
 * Pushes the prototype of the home object for |callee| onto the
 * stack.
 *
 *   Category: Variables and Scopes
 *   Type: Super
 *   Operands:
 *   Stack: callee => homeObjectProto
 */
pub const SUPERBASE: Opcode = Opcode {
    value: 103,
    name: "superbase",
};
/*
 * Pops the top two values, and pushes the property of one, using the other
 * as the receiver.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: receiver, obj => obj[name]
 */
pub const GETPROP_SUPER: Opcode = Opcode {
    value: 104,
    name: "getprop-super",
};
/*
 * Pops the top three values on the stack as 'val' and 'obj', and
 * 'receiver', and performs 'obj.prop = val', pushing 'val' back onto the
 * stack. Throws a TypeError if the set-operation failed (per strict mode
 * semantics).
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: receiver, obj, val => val
 */
pub const STRICTSETPROP_SUPER: Opcode = Opcode {
    value: 105,
    name: "strictsetprop-super",
};
/*
 * This opcode precedes every labeled statement. It's a no-op.
 *
 * 'offset' is the offset to the next instruction after this statement, the
 * one 'break LABEL;' would jump to. IonMonkey uses this.
 *
 *   Category: Statements
 *   Type: Jumps
 *   Operands: int32_t offset
 *   Stack: =>
 */
pub const LABEL: Opcode = Opcode {
    value: 106,
    name: "label",
};
/*
 * Pops the top three values on the stack as 'val', 'obj' and 'receiver',
 * and performs 'obj.prop = val', pushing 'val' back onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: receiver, obj, val => val
 */
pub const SETPROP_SUPER: Opcode = Opcode {
    value: 107,
    name: "setprop-super",
};
/*
 * Invokes 'callee' with 'this' and 'args', pushes return value onto the
 * stack.
 *
 * If 'callee' is determined to be the canonical 'Function.prototype.call'
 * function, then this operation is optimized to directly call 'callee'
 * with 'args[0]' as 'this', and the remaining arguments as formal args to
 * 'callee'.
 *
 * Like JSOP_FUNAPPLY but for 'f.call' instead of 'f.apply'.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc
 *   Stack: callee, this, args[0], ..., args[argc-1] => rval
 *   nuses: (argc+2)
 */
pub const FUNCALL: Opcode = Opcode {
    value: 108,
    name: "funcall",
};
/*
 * Another no-op.
 *
 * This opcode is the target of the backwards jump for some loop.
 * See JSOP_JUMPTARGET for the icIndex operand.
 *
 *   Category: Statements
 *   Type: Jumps
 *   Operands: uint32_t icIndex
 *   Stack: =>
 */
pub const LOOPHEAD: Opcode = Opcode {
    value: 109,
    name: "loophead",
};
/*
 * Looks up name on the environment chain and pushes the environment which
 * contains the name onto the stack. If not found, pushes global lexical
 * environment onto the stack.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: => env
 */
pub const BINDNAME: Opcode = Opcode {
    value: 110,
    name: "bindname",
};
/*
 * Pops an environment and value from the stack, assigns value to the given
 * name, and pushes the value back on the stack
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: env, val => val
 */
pub const SETNAME: Opcode = Opcode {
    value: 111,
    name: "setname",
};
/*
 * Pops the top of stack value as 'v', sets pending exception as 'v', then
 * raises error.
 *
 *   Category: Statements
 *   Type: Exception Handling
 *   Operands:
 *   Stack: v =>
 */
pub const THROW: Opcode = Opcode {
    value: 112,
    name: js_throw_str,
};
/*
 * Pops the top two values 'id' and 'obj' from the stack, then pushes
 * 'id in obj'. This will throw a 'TypeError' if 'obj' is not an object.
 *
 * Note that 'obj' is the top value.
 *
 *   Category: Operators
 *   Type: Special Operators
 *   Operands:
 *   Stack: id, obj => (id in obj)
 */
pub const IN: Opcode = Opcode {
    value: 113,
    name: js_in_str,
};
/*
 * Pops the top two values 'obj' and 'ctor' from the stack, then pushes
 * 'obj instanceof ctor'. This will throw a 'TypeError' if 'obj' is not an
 * object.
 *
 *   Category: Operators
 *   Type: Special Operators
 *   Operands:
 *   Stack: obj, ctor => (obj instanceof ctor)
 */
pub const INSTANCEOF: Opcode = Opcode {
    value: 114,
    name: js_instanceof_str,
};
/*
 * Invokes debugger.
 *
 *   Category: Statements
 *   Type: Debugger
 *   Operands:
 *   Stack: =>
 */
pub const DEBUGGER: Opcode = Opcode {
    value: 115,
    name: "debugger",
};
/*
 * This opcode is used for entering a 'finally' block. Jumps to a 32-bit
 * offset from the current pc.
 *
 * Note: this op doesn't actually push/pop any values, but it has a use
 * count of 2 (for the 'false' + resumeIndex values pushed by preceding
 * bytecode ops) because the 'finally' entry point does not expect these
 * values on the stack. See also JSOP_FINALLY (it has a def count of 2).
 *
 * When the execution resumes from 'finally' block, those stack values are
 * popped.
 *
 *   Category: Statements
 *   Type: Exception Handling
 *   Operands: int32_t offset
 *   Stack: false, resumeIndex =>
 */
pub const GOSUB: Opcode = Opcode {
    value: 116,
    name: "gosub",
};
/*
 * This opcode is used for returning from a 'finally' block.
 *
 * Pops the top two values on the stack as 'rval' and 'lval'. Then:
 * - If 'lval' is true, throws 'rval'.
 * - If 'lval' is false, jumps to the resumeIndex stored in 'lval'.
 *
 *   Category: Statements
 *   Type: Exception Handling
 *   Operands:
 *   Stack: lval, rval =>
 */
pub const RETSUB: Opcode = Opcode {
    value: 117,
    name: "retsub",
};
/*
 * Pushes the current pending exception onto the stack and clears the
 * pending exception. This is only emitted at the beginning of code for a
 * catch-block, so it is known that an exception is pending. It is used to
 * implement catch-blocks and 'yield*'.
 *
 *   Category: Statements
 *   Type: Exception Handling
 *   Operands:
 *   Stack: => exception
 */
pub const EXCEPTION: Opcode = Opcode {
    value: 118,
    name: "exception",
};
/*
 * Embedded lineno to speedup 'pc->line' mapping.
 *
 *   Category: Other
 *   Operands: uint32_t lineno
 *   Stack: =>
 */
pub const LINENO: Opcode = Opcode {
    value: 119,
    name: "lineno",
};
/*
 * This no-op appears after the bytecode for EXPR in 'switch (EXPR) {...}'
 * if the switch cannot be optimized using JSOP_TABLESWITCH.
 *
 * For a non-optimized switch statement like this:
 *
 *     switch (EXPR) {
 *       case V0:
 *         C0;
 *       ...
 *       default:
 *         D;
 *     }
 *
 * the bytecode looks like this:
 *
 *     (EXPR)
 *     condswitch
 *     (V0)
 *     case ->C0
 *     ...
 *     default ->D
 *     (C0)
 *     ...
 *     (D)
 *
 * Note that code for all case-labels is emitted first, then code for the
 * body of each case clause.
 *
 *   Category: Statements
 *   Type: Switch Statement
 *   Operands:
 *   Stack: =>
 */
pub const CONDSWITCH: Opcode = Opcode {
    value: 120,
    name: "condswitch",
};
/*
 * Pops the top two values on the stack as 'val' and 'cond'. If 'cond' is
 * 'true', jumps to a 32-bit offset from the current bytecode, re-pushes
 * 'val' onto the stack if 'false'.
 *
 *   Category: Statements
 *   Type: Switch Statement
 *   Operands: int32_t offset
 *   Stack: val, cond => val(if !cond)
 */
pub const CASE: Opcode = Opcode {
    value: 121,
    name: "case",
};
/*
 * This appears after all cases in a JSOP_CONDSWITCH, whether there is a
 * 'default:' label in the switch statement or not. Pop the switch operand
 * from the stack and jump to a 32-bit offset from the current bytecode.
 * offset from the current bytecode.
 *
 *   Category: Statements
 *   Type: Switch Statement
 *   Operands: int32_t offset
 *   Stack: lval =>
 */
pub const DEFAULT: Opcode = Opcode {
    value: 122,
    name: "default",
};
/*
 * Invokes 'eval' with 'args' and pushes return value onto the stack.
 *
 * If 'eval' in global scope is not original one, invokes the function with
 * 'this' and 'args', and pushes return value onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc
 *   Stack: callee, this, args[0], ..., args[argc-1] => rval
 *   nuses: (argc+2)
 */
pub const EVAL: Opcode = Opcode {
    value: 123,
    name: "eval",
};
/*
 * Invokes 'eval' with 'args' and pushes return value onto the stack.
 *
 * If 'eval' in global scope is not original one, invokes the function with
 * 'this' and 'args', and pushes return value onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc
 *   Stack: callee, this, args[0], ..., args[argc-1] => rval
 *   nuses: (argc+2)
 */
pub const STRICTEVAL: Opcode = Opcode {
    value: 124,
    name: "strict-eval",
};
/*
 * LIKE JSOP_GETELEM but takes receiver on stack, and the propval is
 * evaluated before the obj.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: receiver, propval, obj => obj[propval]
 */
pub const GETELEM_SUPER: Opcode = Opcode {
    value: 125,
    name: "getelem-super",
};
/*
 * Pushes a resumeIndex (stored as 24-bit operand) on the stack.
 *
 * Resume indexes are used for ops like JSOP_YIELD and JSOP_GOSUB.
 * JSScript and BaselineScript have lists of resume entries (one for each
 * resumeIndex); this lets the JIT resume at these ops from JIT code.
 *
 *   Category: Other
 *   Operands: uint24_t resumeIndex
 *   Stack: => resumeIndex
 */
pub const RESUMEINDEX: Opcode = Opcode {
    value: 126,
    name: "resume-index",
};
/*
 * Defines the given function on the current scope.
 *
 * This is used for global scripts and also in some cases for function
 * scripts where use of dynamic scoping inhibits optimization.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands:
 *   Stack: fun =>
 */
pub const DEFFUN: Opcode = Opcode {
    value: 127,
    name: "deffun",
};
/*
 * Defines the new constant binding on global lexical environment.
 *
 * Throws if a binding with the same name already exists on the
 * environment, or if a var binding with the same name exists on the
 * global.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: =>
 */
pub const DEFCONST: Opcode = Opcode {
    value: 128,
    name: "defconst",
};
/*
 * Defines the new binding on the frame's current variables-object (the
 * environment on the environment chain designated to receive new
 * variables).
 *
 * Throws if the current variables-object is the global object and a
 * binding with the same name exists on the global lexical environment.
 *
 * This is used for global scripts and also in some cases for function
 * scripts where use of dynamic scoping inhibits optimization.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: =>
 */
pub const DEFVAR: Opcode = Opcode {
    value: 129,
    name: "defvar",
};
/*
 * Pushes a closure for a named or anonymous function expression onto the
 * stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint32_t funcIndex
 *   Stack: => obj
 */
pub const LAMBDA: Opcode = Opcode {
    value: 130,
    name: "lambda",
};
/*
 * Pops the top of stack value as 'new.target', pushes an arrow function
 * with lexical 'new.target' onto the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint32_t funcIndex
 *   Stack: new.target => obj
 */
pub const LAMBDA_ARROW: Opcode = Opcode {
    value: 131,
    name: "lambda_arrow",
};
/*
 * Pushes current callee onto the stack.
 *
 * Used for named function expression self-naming, if lightweight.
 *
 *   Category: Variables and Scopes
 *   Type: Arguments
 *   Operands:
 *   Stack: => callee
 */
pub const CALLEE: Opcode = Opcode {
    value: 132,
    name: "callee",
};
/*
 * Picks the nth element from the stack and moves it to the top of the
 * stack.
 *
 *   Category: Operators
 *   Type: Stack Operations
 *   Operands: uint8_t n
 *   Stack: v[n], v[n-1], ..., v[1], v[0] => v[n-1], ..., v[1], v[0], v[n]
 */
pub const PICK: Opcode = Opcode {
    value: 133,
    name: "pick",
};
/*
 * This no-op appears at the top of the bytecode for a 'TryStatement'.
 *
 * Location information for catch/finally blocks is stored in a side table,
 * 'script->trynotes()'.
 *
 *   Category: Statements
 *   Type: Exception Handling
 *   Operands:
 *   Stack: =>
 */
pub const TRY: Opcode = Opcode {
    value: 134,
    name: "try",
};
/*
 * This opcode has a def count of 2, but these values are already on the
 * stack (they're pushed by JSOP_GOSUB).
 *
 *   Category: Statements
 *   Type: Exception Handling
 *   Operands:
 *   Stack: => false, resumeIndex
 */
pub const FINALLY: Opcode = Opcode {
    value: 135,
    name: "finally",
};
/*
 * Pushes aliased variable onto the stack.
 *
 * An "aliased variable" is a var, let, or formal arg that is aliased.
 * Sources of aliasing include: nested functions accessing the vars of an
 * enclosing function, function statements that are conditionally executed,
 * 'eval', 'with', and 'arguments'. All of these cases require creating a
 * CallObject to own the aliased variable.
 *
 * An ALIASEDVAR opcode contains the following immediates:
 *  uint8 hops: the number of environment objects to skip to find the
 *               EnvironmentObject containing the variable being accessed
 *  uint24 slot: the slot containing the variable in the EnvironmentObject
 *               (this 'slot' does not include RESERVED_SLOTS).
 *
 *   Category: Variables and Scopes
 *   Type: Aliased Variables
 *   Operands: uint8_t hops, uint24_t slot
 *   Stack: => aliasedVar
 */
pub const GETALIASEDVAR: Opcode = Opcode {
    value: 136,
    name: "getaliasedvar",
};
/*
 * Sets aliased variable as the top of stack value.
 *
 *   Category: Variables and Scopes
 *   Type: Aliased Variables
 *   Operands: uint8_t hops, uint24_t slot
 *   Stack: v => v
 */
pub const SETALIASEDVAR: Opcode = Opcode {
    value: 137,
    name: "setaliasedvar",
};
/*
 * Checks if the value of the local variable is the
 * JS_UNINITIALIZED_LEXICAL magic, throwing an error if so.
 *
 *   Category: Variables and Scopes
 *   Type: Local Variables
 *   Operands: uint24_t localno
 *   Stack: =>
 */
pub const CHECKLEXICAL: Opcode = Opcode {
    value: 138,
    name: "checklexical",
};
/*
 * Initializes an uninitialized local lexical binding with the top of stack
 * value.
 *
 *   Category: Variables and Scopes
 *   Type: Local Variables
 *   Operands: uint24_t localno
 *   Stack: v => v
 */
pub const INITLEXICAL: Opcode = Opcode {
    value: 139,
    name: "initlexical",
};
/*
 * Checks if the value of the aliased variable is the
 * JS_UNINITIALIZED_LEXICAL magic, throwing an error if so.
 *
 *   Category: Variables and Scopes
 *   Type: Aliased Variables
 *   Operands: uint8_t hops, uint24_t slot
 *   Stack: =>
 */
pub const CHECKALIASEDLEXICAL: Opcode = Opcode {
    value: 140,
    name: "checkaliasedlexical",
};
/*
 * Initializes an uninitialized aliased lexical binding with the top of
 * stack value.
 *
 *   Category: Variables and Scopes
 *   Type: Aliased Variables
 *   Operands: uint8_t hops, uint24_t slot
 *   Stack: v => v
 */
pub const INITALIASEDLEXICAL: Opcode = Opcode {
    value: 141,
    name: "initaliasedlexical",
};
/*
 * Pushes a JS_UNINITIALIZED_LEXICAL value onto the stack, representing an
 * uninitialized lexical binding.
 *
 * This opcode is used with the JSOP_INITLEXICAL opcode.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands:
 *   Stack: => uninitialized
 */
pub const UNINITIALIZED: Opcode = Opcode {
    value: 142,
    name: "uninitialized",
};
/*
 * Pushes the value of the intrinsic onto the stack.
 *
 * Intrinsic names are emitted instead of JSOP_*NAME ops when the
 * 'CompileOptions' flag 'selfHostingMode' is set.
 *
 * They are used in self-hosted code to access other self-hosted values and
 * intrinsic functions the runtime doesn't give client JS code access to.
 *
 *   Category: Variables and Scopes
 *   Type: Intrinsics
 *   Operands: uint32_t nameIndex
 *   Stack: => intrinsic[name]
 */
pub const GETINTRINSIC: Opcode = Opcode {
    value: 143,
    name: "getintrinsic",
};
/*
 * Stores the top stack value in the specified intrinsic.
 *
 *   Category: Variables and Scopes
 *   Type: Intrinsics
 *   Operands: uint32_t nameIndex
 *   Stack: val => val
 */
pub const SETINTRINSIC: Opcode = Opcode {
    value: 144,
    name: "setintrinsic",
};
/*
 * Like JSOP_CALL, but used as part of for-of and destructuring bytecode to
 * provide better error messages.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc (must be 0)
 *   Stack: callee, this => rval
 *   nuses: 2
 */
pub const CALLITER: Opcode = Opcode {
    value: 145,
    name: "calliter",
};
/*
 * Initialize a non-configurable, non-writable, non-enumerable
 * data-property on an object.
 *
 * Pops the top two values on the stack as 'val' and 'obj', defines
 * 'nameIndex' property of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => obj
 */
pub const INITLOCKEDPROP: Opcode = Opcode {
    value: 146,
    name: "initlockedprop",
};
/*
 * Initialize a non-enumerable data-property on an object.
 *
 * Pops the top two values on the stack as 'val' and 'obj', defines
 * 'nameIndex' property of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => obj
 */
pub const INITHIDDENPROP: Opcode = Opcode {
    value: 147,
    name: "inithiddenprop",
};
/*
 * Push "new.target"
 *
 *   Category: Variables and Scopes
 *   Type: Arguments
 *   Operands:
 *   Stack: => new.target
 */
pub const NEWTARGET: Opcode = Opcode {
    value: 148,
    name: "newtarget",
};
/*
 */
pub const UNUSED149: Opcode = Opcode {
    value: 149,
    name: "unused149",
};
/*
 * Pops the top two values 'lval' and 'rval' from the stack, then pushes
 * the result of 'Math.pow(lval, rval)'.
 *
 *   Category: Operators
 *   Type: Arithmetic Operators
 *   Operands:
 *   Stack: lval, rval => (lval ** rval)
 */
pub const POW: Opcode = Opcode {
    value: 150,
    name: "pow",
};
/*
 * Pops the top two values 'value' and 'gen' from the stack, then starts
 * "awaiting" for 'value' to be resolved, which will then resume the
 * execution of 'gen'. Pushes the async function promise on the stack, so
 * that it'll be returned to the caller on the very first "await".
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands:
 *   Stack: value, gen => promise
 */
pub const ASYNCAWAIT: Opcode = Opcode {
    value: 151,
    name: "async-await",
};
/*
 * Pops the top of stack value as 'rval', sets the return value in stack
 * frame as 'rval'.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: rval =>
 */
pub const SETRVAL: Opcode = Opcode {
    value: 152,
    name: "setrval",
};
/*
 * Stops interpretation and returns value set by JSOP_SETRVAL. When not
 * set, returns 'undefined'.
 *
 * Also emitted at end of script so interpreter don't need to check if
 * opcode is still in script range.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: =>
 */
pub const RETRVAL: Opcode = Opcode {
    value: 153,
    name: "retrval",
};
/*
 * Looks up name on global environment and pushes its value onto the stack,
 * unless the script has a non-syntactic global scope, in which case it
 * acts just like JSOP_NAME.
 *
 * Free variable references that must either be found on the global or a
 * ReferenceError.
 *
 *   Category: Variables and Scopes
 *   Type: Free Variables
 *   Operands: uint32_t nameIndex
 *   Stack: => val
 */
pub const GETGNAME: Opcode = Opcode {
    value: 154,
    name: "getgname",
};
/*
 * Pops the top two values on the stack as 'val' and 'env', sets property
 * of 'env' as 'val' and pushes 'val' back on the stack.
 *
 * 'env' should be the global lexical environment unless the script has a
 * non-syntactic global scope, in which case acts like JSOP_SETNAME.
 *
 *   Category: Variables and Scopes
 *   Type: Free Variables
 *   Operands: uint32_t nameIndex
 *   Stack: env, val => val
 */
pub const SETGNAME: Opcode = Opcode {
    value: 155,
    name: "setgname",
};
/*
 * Pops the top two values on the stack as 'val' and 'env', sets property
 * of 'env' as 'val' and pushes 'val' back on the stack. Throws a TypeError
 * if the set fails, per strict mode semantics.
 *
 * 'env' should be the global lexical environment unless the script has a
 * non-syntactic global scope, in which case acts like JSOP_STRICTSETNAME.
 *
 *   Category: Variables and Scopes
 *   Type: Free Variables
 *   Operands: uint32_t nameIndex
 *   Stack: env, val => val
 */
pub const STRICTSETGNAME: Opcode = Opcode {
    value: 156,
    name: "strict-setgname",
};
/*
 * Pushes the implicit 'this' value for calls to the associated name onto
 * the stack; only used when the implicit this might be derived from a
 * non-syntactic scope (instead of the global itself).
 *
 * Note that code evaluated via the Debugger API uses DebugEnvironmentProxy
 * objects on its scope chain, which are non-syntactic environments that
 * refer to syntactic environments. As a result, the binding we want may be
 * held by a syntactic environments such as CallObject or
 * VarEnvrionmentObject.
 *
 *   Category: Variables and Scopes
 *   Type: This
 *   Operands: uint32_t nameIndex
 *   Stack: => this
 */
pub const GIMPLICITTHIS: Opcode = Opcode {
    value: 157,
    name: "gimplicitthis",
};
/*
 * LIKE JSOP_SETELEM, but takes receiver on the stack, and the propval is
 * evaluated before the base.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: receiver, propval, obj, val => val
 */
pub const SETELEM_SUPER: Opcode = Opcode {
    value: 158,
    name: "setelem-super",
};
/*
 * LIKE JSOP_STRICTSETELEM, but takes receiver on the stack, and the
 * propval is evaluated before the base.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: receiver, propval, obj, val => val
 */
pub const STRICTSETELEM_SUPER: Opcode = Opcode {
    value: 159,
    name: "strict-setelem-super",
};
/*
 * Pushes a regular expression literal onto the stack. It requires special
 * "clone on exec" handling.
 *
 *   Category: Literals
 *   Type: RegExp
 *   Operands: uint32_t regexpIndex
 *   Stack: => regexp
 */
pub const REGEXP: Opcode = Opcode {
    value: 160,
    name: "regexp",
};
/*
 * Initializes an uninitialized global lexical binding with the top of
 * stack value.
 *
 *   Category: Variables and Scopes
 *   Type: Free Variables
 *   Operands: uint32_t nameIndex
 *   Stack: val => val
 */
pub const INITGLEXICAL: Opcode = Opcode {
    value: 161,
    name: "initglexical",
};
/*
 * Defines the new mutable binding on global lexical environment.
 *
 * Throws if a binding with the same name already exists on the
 * environment, or if a var binding with the same name exists on the
 * global.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: =>
 */
pub const DEFLET: Opcode = Opcode {
    value: 162,
    name: "deflet",
};
/*
 * Throw if the value on the stack is not coerscible to an object (is
 * |null| or |undefined|).
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: val => val
 */
pub const CHECKOBJCOERCIBLE: Opcode = Opcode {
    value: 163,
    name: "checkobjcoercible",
};
/*
 * Push the function to invoke with |super()|. This is the prototype of the
 * function passed in as |callee|.
 *
 *   Category: Variables and Scopes
 *   Type: Super
 *   Operands:
 *   Stack: callee => superFun
 */
pub const SUPERFUN: Opcode = Opcode {
    value: 164,
    name: "superfun",
};
/*
 * Behaves exactly like JSOP_NEW, but allows JITs to distinguish the two
 * cases.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc
 *   Stack: callee, this, args[0], ..., args[argc-1], newTarget => rval
 *   nuses: (argc+3)
 */
pub const SUPERCALL: Opcode = Opcode {
    value: 165,
    name: "supercall",
};
/*
 * spreadcall variant of JSOP_SUPERCALL.
 *
 * Behaves exactly like JSOP_SPREADNEW.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: callee, this, args, newTarget => rval
 */
pub const SPREADSUPERCALL: Opcode = Opcode {
    value: 166,
    name: "spreadsupercall",
};
/*
 * Push a default constructor for a base class literal.
 *
 *   Category: Literals
 *   Type: Class
 *   Operands: atom className
 *   Stack: => constructor
 */
pub const CLASSCONSTRUCTOR: Opcode = Opcode {
    value: 167,
    name: "classconstructor",
};
/*
 * Push a default constructor for a derived class literal.
 *
 *   Category: Literals
 *   Type: Class
 *   Operands: atom className
 *   Stack: proto => constructor
 */
pub const DERIVEDCONSTRUCTOR: Opcode = Opcode {
    value: 168,
    name: "derivedconstructor",
};
/*
 * Throws a runtime TypeError for invalid assignment to 'const'. The
 * localno is used for better error messages.
 *
 *   Category: Variables and Scopes
 *   Type: Local Variables
 *   Operands: uint24_t localno
 *   Stack: v => v
 */
pub const THROWSETCONST: Opcode = Opcode {
    value: 169,
    name: "throwsetconst",
};
/*
 * Throws a runtime TypeError for invalid assignment to 'const'. The
 * environment coordinate is used for better error messages.
 *
 *   Category: Variables and Scopes
 *   Type: Aliased Variables
 *   Operands: uint8_t hops, uint24_t slot
 *   Stack: v => v
 */
pub const THROWSETALIASEDCONST: Opcode = Opcode {
    value: 170,
    name: "throwsetaliasedconst",
};
/*
 * Initialize a non-enumerable getter in an object literal.
 *
 * Pops the top two values on the stack as 'val' and 'obj', defines getter
 * of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => obj
 */
pub const INITHIDDENPROP_GETTER: Opcode = Opcode {
    value: 171,
    name: "inithiddenprop_getter",
};
/*
 * Initialize a non-enumerable setter in an object literal.
 *
 * Pops the top two values on the stack as 'val' and 'obj', defines setter
 * of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj, val => obj
 */
pub const INITHIDDENPROP_SETTER: Opcode = Opcode {
    value: 172,
    name: "inithiddenprop_setter",
};
/*
 * Initialize a non-enumerable numeric getter in an object literal like
 * '{get 2() {}}'.
 *
 * Pops the top three values on the stack as 'val', 'id' and 'obj', defines
 * 'id' getter of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, id, val => obj
 */
pub const INITHIDDENELEM_GETTER: Opcode = Opcode {
    value: 173,
    name: "inithiddenelem_getter",
};
/*
 * Initialize a non-enumerable numeric setter in an object literal like
 * '{set 2(v) {}}'.
 *
 * Pops the top three values on the stack as 'val', 'id' and 'obj', defines
 * 'id' setter of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, id, val => obj
 */
pub const INITHIDDENELEM_SETTER: Opcode = Opcode {
    value: 174,
    name: "inithiddenelem_setter",
};
/*
 * Initialize a non-enumerable numeric property in an object literal, like
 * '{1: x}'.
 *
 * Pops the top three values on the stack as 'val', 'id' and 'obj', defines
 * 'id' property of 'obj' as 'val', pushes 'obj' onto the stack.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, id, val => obj
 */
pub const INITHIDDENELEM: Opcode = Opcode {
    value: 175,
    name: "inithiddenelem",
};
/*
 * Gets the value of a module import by name and pushes it onto the stack.
 *
 *   Category: Variables and Scopes
 *   Type: Variables
 *   Operands: uint32_t nameIndex
 *   Stack: => val
 */
pub const GETIMPORT: Opcode = Opcode {
    value: 176,
    name: "getimport",
};
/*
 * Examines the top stack value, asserting that it's either a self-hosted
 * function or a self-hosted intrinsic. This opcode does nothing in a
 * non-debug build.
 *
 *   Category: Other
 *   Operands:
 *   Stack: checkVal => checkVal
 */
pub const DEBUGCHECKSELFHOSTED: Opcode = Opcode {
    value: 177,
    name: "debug-checkselfhosted",
};
/*
 * Pops the top stack value, pushes the value and a boolean value that
 * indicates whether the spread operation for the value can be optimized in
 * spread call.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: arr => arr, optimized
 */
pub const OPTIMIZE_SPREADCALL: Opcode = Opcode {
    value: 178,
    name: "optimize-spreadcall",
};
/*
 * Throws a runtime TypeError for invalid assignment to the callee in a
 * named lambda, which is always a 'const' binding. This is a different
 * bytecode than JSOP_SETCONST because the named lambda callee, if not
 * closed over, does not have a frame slot to look up the name with for the
 * error message.
 *
 *   Category: Variables and Scopes
 *   Type: Local Variables
 *   Operands:
 *   Stack: v => v
 */
pub const THROWSETCALLEE: Opcode = Opcode {
    value: 179,
    name: "throwsetcallee",
};
/*
 * Pushes a var environment onto the env chain.
 *
 *   Category: Variables and Scopes
 *   Type: Var Scope
 *   Operands: uint32_t scopeIndex
 *   Stack: =>
 */
pub const PUSHVARENV: Opcode = Opcode {
    value: 180,
    name: "pushvarenv",
};
/*
 * Pops a var environment from the env chain.
 *
 *   Category: Variables and Scopes
 *   Type: Var Scope
 *   Operands:
 *   Stack: =>
 */
pub const POPVARENV: Opcode = Opcode {
    value: 181,
    name: "popvarenv",
};
/*
 * Pops the top two values on the stack as 'name' and 'fun', defines the
 * name of 'fun' to 'name' with prefix if any, and pushes 'fun' back onto
 * the stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint8_t prefixKind
 *   Stack: fun, name => fun
 */
pub const SETFUNNAME: Opcode = Opcode {
    value: 182,
    name: "setfunname",
};
/*
 * Moves the top of the stack value under the nth element of the stack.
 * Note: n must NOT be 0.
 *
 *   Category: Operators
 *   Type: Stack Operations
 *   Operands: uint8_t n
 *   Stack: v[n], v[n-1], ..., v[1], v[0] => v[0], v[n], v[n-1], ..., v[1]
 */
pub const UNPICK: Opcode = Opcode {
    value: 183,
    name: "unpick",
};
/*
 * Pops the top of stack value, pushes property of it onto the stack.
 * Requires the value under 'obj' to be the receiver of the following call.
 *
 * Like JSOP_GETPROP but for call context.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: obj => obj[name]
 */
pub const CALLPROP: Opcode = Opcode {
    value: 184,
    name: "callprop",
};
/*
 * Determines the 'this' value for current function frame and pushes it
 * onto the stack. Emitted in the prologue of functions with a
 * this-binding.
 *
 *   Category: Variables and Scopes
 *   Type: This
 *   Operands:
 *   Stack: => this
 */
pub const FUNCTIONTHIS: Opcode = Opcode {
    value: 185,
    name: "functionthis",
};
/*
 * Pushes 'this' value for current stack frame onto the stack. Emitted when
 * 'this' refers to the global 'this'.
 *
 *   Category: Variables and Scopes
 *   Type: This
 *   Operands:
 *   Stack: => this
 */
pub const GLOBALTHIS: Opcode = Opcode {
    value: 186,
    name: "globalthis",
};
/*
 * Pushes a boolean indicating whether the top of the stack is
 * MagicValue(JS_GENERATOR_CLOSING).
 *
 *   Category: Statements
 *   Type: For-In Statement
 *   Operands:
 *   Stack: val => val, res
 */
pub const ISGENCLOSING: Opcode = Opcode {
    value: 187,
    name: "isgenclosing",
};
/*
 * Pushes unsigned 24-bit int immediate integer operand onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands: uint24_t val
 *   Stack: => val
 */
pub const UINT24: Opcode = Opcode {
    value: 188,
    name: "uint24",
};
/*
 * Throw if the value on top of the stack is the TDZ MagicValue. Used in
 * derived class constructors.
 *
 *   Category: Variables and Scopes
 *   Type: This
 *   Operands:
 *   Stack: this => this
 */
pub const CHECKTHIS: Opcode = Opcode {
    value: 189,
    name: "checkthis",
};
/*
 * Check if a derived class constructor has a valid return value and 'this'
 * value before it returns. If the return value is not an object, stores
 * the 'this' value to the return value slot.
 *
 *   Category: Variables and Scopes
 *   Type: This
 *   Operands:
 *   Stack: this =>
 */
pub const CHECKRETURN: Opcode = Opcode {
    value: 190,
    name: "checkreturn",
};
/*
 * Throw an exception if the value on top of the stack is not the TDZ
 * MagicValue. Used in derived class constructors.
 *
 *   Category: Variables and Scopes
 *   Type: This
 *   Operands:
 *   Stack: this => this
 */
pub const CHECKTHISREINIT: Opcode = Opcode {
    value: 191,
    name: "checkthisreinit",
};
/*
 * Pops the top two values 'valueOrReason' and 'gen' from the stack, then
 * pushes the promise resolved with 'valueOrReason'. `gen` must be the
 * internal generator object created in async functions. The pushed promise
 * is the async function's result promise, which is stored in `gen`.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands: uint8_t fulfillOrReject
 *   Stack: valueOrReason, gen => promise
 */
pub const ASYNCRESOLVE: Opcode = Opcode {
    value: 192,
    name: "async-resolve",
};
/*
 * Pops the top two values on the stack as 'propval' and 'obj', pushes
 * 'propval' property of 'obj' onto the stack. Requires the value under
 * 'obj' to be the receiver of the following call.
 *
 * Like JSOP_GETELEM but for call context.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, propval => obj[propval]
 */
pub const CALLELEM: Opcode = Opcode {
    value: 193,
    name: "callelem",
};
/*
 * '__proto__: v' inside an object initializer.
 *
 * Pops the top two values on the stack as 'newProto' and 'obj', sets
 * prototype of 'obj' as 'newProto', pushes 'true' onto the stack if
 * succeeded, 'false' if not.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: obj, newProto => succeeded
 */
pub const MUTATEPROTO: Opcode = Opcode {
    value: 194,
    name: "mutateproto",
};
/*
 * Pops an environment, gets the value of a bound name on it. If the name
 * is not bound to the environment, throw a ReferenceError. Used in
 * conjunction with BINDNAME.
 *
 *   Category: Literals
 *   Type: Object
 *   Operands: uint32_t nameIndex
 *   Stack: env => v
 */
pub const GETBOUNDNAME: Opcode = Opcode {
    value: 195,
    name: "getboundname",
};
/*
 * Pops the top stack value as 'val' and pushes 'typeof val'. Note that
 * this opcode isn't used when, in the original source code, 'val' is a
 * name -- see 'JSOP_TYPEOF' for that.
 * (This is because 'typeof undefinedName === "undefined"'.)
 *
 *   Category: Operators
 *   Type: Special Operators
 *   Operands:
 *   Stack: val => (typeof val)
 */
pub const TYPEOFEXPR: Opcode = Opcode {
    value: 196,
    name: "typeofexpr",
};
/*
 * Replaces the current block on the env chain with a fresh block that
 * copies all the bindings in the block. This operation implements the
 * behavior of inducing a fresh lexical environment for every iteration of
 * a for(let ...; ...; ...) loop, if any declarations induced by such a
 * loop are captured within the loop.
 *
 *   Category: Variables and Scopes
 *   Type: Block-local Scope
 *   Operands:
 *   Stack: =>
 */
pub const FRESHENLEXICALENV: Opcode = Opcode {
    value: 197,
    name: "freshenlexicalenv",
};
/*
 * Recreates the current block on the env chain with a fresh block with
 * uninitialized bindings. This operation implements the behavior of
 * inducing a fresh lexical environment for every iteration of a for-in/of
 * loop whose loop-head has a (captured) lexical declaration.
 *
 *   Category: Variables and Scopes
 *   Type: Block-local Scope
 *   Operands:
 *   Stack: =>
 */
pub const RECREATELEXICALENV: Opcode = Opcode {
    value: 198,
    name: "recreatelexicalenv",
};
/*
 * Pushes lexical environment onto the env chain.
 *
 *   Category: Variables and Scopes
 *   Type: Block-local Scope
 *   Operands: uint32_t scopeIndex
 *   Stack: =>
 */
pub const PUSHLEXICALENV: Opcode = Opcode {
    value: 199,
    name: "pushlexicalenv",
};
/*
 * Pops lexical environment from the env chain.
 *
 *   Category: Variables and Scopes
 *   Type: Block-local Scope
 *   Operands:
 *   Stack: =>
 */
pub const POPLEXICALENV: Opcode = Opcode {
    value: 200,
    name: "poplexicalenv",
};
/*
 * The opcode to assist the debugger.
 *
 *   Category: Statements
 *   Type: Debugger
 *   Operands:
 *   Stack: =>
 */
pub const DEBUGLEAVELEXICALENV: Opcode = Opcode {
    value: 201,
    name: "debugleavelexicalenv",
};
/*
 * Pops the generator from the top of the stack, suspends it and stops
 * interpretation.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands: uint24_t resumeIndex
 *   Stack: generator => generator
 */
pub const INITIALYIELD: Opcode = Opcode {
    value: 202,
    name: "initialyield",
};
/*
 * Pops the generator and the return value 'rval1', stops interpretation
 * and returns 'rval1'. Pushes sent value from 'send()' onto the stack.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands: uint24_t resumeIndex
 *   Stack: rval1, gen => rval2
 */
pub const YIELD: Opcode = Opcode {
    value: 203,
    name: "yield",
};
/*
 * Pops the generator and suspends and closes it. Yields the value in the
 * frame's return value slot.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands:
 *   Stack: gen =>
 */
pub const FINALYIELDRVAL: Opcode = Opcode {
    value: 204,
    name: "finalyieldrval",
};
/*
 * Pops the generator and argument from the stack, pushes a new generator
 * frame and resumes execution of it. Pushes the return value after the
 * generator yields.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands: resume kind (AbstractGeneratorObject::ResumeKind)
 *   Stack: gen, val => rval
 */
pub const RESUME: Opcode = Opcode {
    value: 205,
    name: "resume",
};
/*
 * Load the callee stored in a CallObject on the environment chain. The
 * numHops operand is the number of environment objects to skip on the
 * environment chain.
 *
 *   Category: Variables and Scopes
 *   Type: Arguments
 *   Operands: uint8_t numHops
 *   Stack: => callee
 */
pub const ENVCALLEE: Opcode = Opcode {
    value: 206,
    name: "envcallee",
};
/*
 * No-op bytecode only emitted in some self-hosted functions. Not handled
 * by the JITs or Baseline Interpreter so the script always runs in the C++
 * interpreter.
 *
 *   Category: Other
 *   Operands:
 *   Stack: =>
 */
pub const FORCEINTERPRETER: Opcode = Opcode {
    value: 207,
    name: "forceinterpreter",
};
/*
 * Bytecode emitted after 'yield' expressions. This is useful for the
 * Debugger and AbstractGeneratorObject::isAfterYieldOrAwait. It's treated
 * as jump target op so that the Baseline Interpreter can efficiently
 * restore the frame's interpreterICEntry when resuming a generator.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands: uint32_t icIndex
 *   Stack: =>
 */
pub const AFTERYIELD: Opcode = Opcode {
    value: 208,
    name: "afteryield",
};
/*
 * Pops the generator and the return value 'promise', stops interpretation
 * and returns 'promise'. Pushes resolved value onto the stack.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands: uint24_t resumeIndex
 *   Stack: promise, gen => resolved
 */
pub const AWAIT: Opcode = Opcode {
    value: 209,
    name: "await",
};
/*
 * Pops the iterator and its next method from the top of the stack, and
 * create async iterator from it and push the async iterator back onto the
 * stack.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands:
 *   Stack: iter, next => asynciter
 */
pub const TOASYNCITER: Opcode = Opcode {
    value: 210,
    name: "toasynciter",
};
/*
 * Pops the top two values 'id' and 'obj' from the stack, then pushes
 * obj.hasOwnProperty(id)
 *
 * Note that 'obj' is the top value.
 *
 *   Category: Other
 *   Type:
 *   Operands:
 *   Stack: id, obj => (obj.hasOwnProperty(id))
 */
pub const HASOWN: Opcode = Opcode {
    value: 211,
    name: "hasown",
};
/*
 * Initializes generator frame, creates a generator and pushes it on the
 * stack.
 *
 *   Category: Statements
 *   Type: Generator
 *   Operands:
 *   Stack: => generator
 */
pub const GENERATOR: Opcode = Opcode {
    value: 212,
    name: "generator",
};
/*
 * Pushes the nearest 'var' environment.
 *
 *   Category: Variables and Scopes
 *   Type: Free Variables
 *   Operands:
 *   Stack: => env
 */
pub const BINDVAR: Opcode = Opcode {
    value: 213,
    name: "bindvar",
};
/*
 * Pushes the global environment onto the stack if the script doesn't have
 * a non-syntactic global scope. Otherwise will act like JSOP_BINDNAME.
 *
 * 'nameIndex' is only used when acting like JSOP_BINDNAME.
 *
 *   Category: Variables and Scopes
 *   Type: Free Variables
 *   Operands: uint32_t nameIndex
 *   Stack: => global
 */
pub const BINDGNAME: Opcode = Opcode {
    value: 214,
    name: "bindgname",
};
/*
 * Pushes 8-bit int immediate integer operand onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands: int8_t val
 *   Stack: => val
 */
pub const INT8: Opcode = Opcode {
    value: 215,
    name: "int8",
};
/*
 * Pushes 32-bit int immediate integer operand onto the stack.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands: int32_t val
 *   Stack: => val
 */
pub const INT32: Opcode = Opcode {
    value: 216,
    name: "int32",
};
/*
 * Pops the top of stack value, pushes the 'length' property of it onto the
 * stack.
 *
 *   Category: Literals
 *   Type: Array
 *   Operands: uint32_t nameIndex
 *   Stack: obj => obj['length']
 */
pub const LENGTH: Opcode = Opcode {
    value: 217,
    name: "length",
};
/*
 * Pushes a JS_ELEMENTS_HOLE value onto the stack, representing an omitted
 * property in an array literal (e.g. property 0 in the array '[, 1]').
 *
 * This opcode is used with the JSOP_NEWARRAY opcode.
 *
 *   Category: Literals
 *   Type: Array
 *   Operands:
 *   Stack: => hole
 */
pub const HOLE: Opcode = Opcode {
    value: 218,
    name: "hole",
};
/*
 * Checks that the top value on the stack is callable, and throws a
 * TypeError if not. The operand 'kind' is used only to generate an
 * appropriate error message.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint8_t kind
 *   Stack: obj => obj
 */
pub const CHECKISCALLABLE: Opcode = Opcode {
    value: 219,
    name: "checkiscallable",
};
/*
 * No-op used by the exception unwinder to determine the correct
 * environment to unwind to when performing IteratorClose due to
 * destructuring.
 *
 *   Category: Other
 *   Operands:
 *   Stack: =>
 */
pub const TRY_DESTRUCTURING: Opcode = Opcode {
    value: 220,
    name: "try-destructuring",
};
/*
 * Pushes the current global's builtin prototype for a given proto key.
 *
 *   Category: Literals
 *   Type: Constants
 *   Operands: uint8_t kind
 *   Stack: => %BuiltinPrototype%
 */
pub const BUILTINPROTO: Opcode = Opcode {
    value: 221,
    name: "builtinproto",
};
/*
 * NOP opcode to hint to IonBuilder that the value on top of the stack is
 * the (likely string) key in a for-in loop.
 *
 *   Category: Other
 *   Operands:
 *   Stack: val => val
 */
pub const ITERNEXT: Opcode = Opcode {
    value: 222,
    name: "iternext",
};
/*
 * Pops the top of stack value as 'value', checks if the await for 'value'
 * can be skipped. If the await operation can be skipped and the resolution
 * value for 'value' can be acquired, pushes the resolution value and
 * 'true' onto the stack. Otherwise, pushes 'value' and 'false' on the
 * stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands:
 *   Stack: value => value_or_resolved, canskip
 */
pub const TRYSKIPAWAIT: Opcode = Opcode {
    value: 223,
    name: "tryskipawait",
};
/*
 * Creates rest parameter array for current function call, and pushes it
 * onto the stack.
 *
 *   Category: Variables and Scopes
 *   Type: Arguments
 *   Operands:
 *   Stack: => rest
 */
pub const REST: Opcode = Opcode {
    value: 224,
    name: "rest",
};
/*
 * Replace the top-of-stack value propertyNameValue with
 * ToPropertyKey(propertyNameValue).
 *
 *   Category: Literals
 *   Type: Object
 *   Operands:
 *   Stack: propertyNameValue => propertyKey
 */
pub const TOID: Opcode = Opcode {
    value: 225,
    name: "toid",
};
/*
 * Pushes the implicit 'this' value for calls to the associated name onto
 * the stack.
 *
 *   Category: Variables and Scopes
 *   Type: This
 *   Operands: uint32_t nameIndex
 *   Stack: => this
 */
pub const IMPLICITTHIS: Opcode = Opcode {
    value: 226,
    name: "implicitthis",
};
/*
 * This opcode is the target of the entry jump for some loop. The uint8
 * argument is a bitfield. The lower 7 bits of the argument indicate the
 * loop depth. This value starts at 1 and is just a hint: deeply nested
 * loops all have the same value. The upper bit is set if Ion should be
 * able to OSR at this point, which is true unless there is non-loop state
 * on the stack. See JSOP_JUMPTARGET for the icIndex argument.
 *
 *   Category: Statements
 *   Type: Jumps
 *   Operands: uint32_t icIndex, uint8_t BITFIELD
 *   Stack: =>
 */
pub const LOOPENTRY: Opcode = Opcode {
    value: 227,
    name: "loopentry",
};
/*
 * Converts the value on the top of the stack to a String.
 *
 *   Category: Other
 *   Operands:
 *   Stack: val => ToString(val)
 */
pub const TOSTRING: Opcode = Opcode {
    value: 228,
    name: "tostring",
};
/*
 * No-op used by the decompiler to produce nicer error messages about
 * destructuring code.
 *
 *   Category: Other
 *   Operands:
 *   Stack: =>
 */
pub const NOP_DESTRUCTURING: Opcode = Opcode {
    value: 229,
    name: "nop-destructuring",
};
/*
 * This opcode is a no-op and it indicates the location of a jump
 * instruction target. Some other opcodes act as jump targets as well, see
 * BytecodeIsJumpTarget. The IC index is used by the Baseline interpreter.
 *
 *   Category: Other
 *   Operands: uint32_t icIndex
 *   Stack: =>
 */
pub const JUMPTARGET: Opcode = Opcode {
    value: 230,
    name: "jumptarget",
};
/*
 * Like JSOP_CALL, but tells the function that the return value is ignored.
 * stack.
 *
 *   Category: Statements
 *   Type: Function
 *   Operands: uint16_t argc
 *   Stack: callee, this, args[0], ..., args[argc-1] => rval
 *   nuses: (argc+2)
 */
pub const CALL_IGNORES_RV: Opcode = Opcode {
    value: 231,
    name: "call-ignores-rv",
};
/*
 * Push "import.meta"
 *
 *   Category: Variables and Scopes
 *   Type: Modules
 *   Operands:
 *   Stack: => import.meta
 */
pub const IMPORTMETA: Opcode = Opcode {
    value: 232,
    name: "importmeta",
};
/*
 * Dynamic import of the module specified by the string value on the top of
 * the stack.
 *
 *   Category: Variables and Scopes
 *   Type: Modules
 *   Operands:
 *   Stack: arg => rval
 */
pub const DYNAMIC_IMPORT: Opcode = Opcode {
    value: 233,
    name: "call-import",
};
/*
 * Pops the numeric value 'val' from the stack, then pushes 'val + 1'.
 *
 *   Category: Operators
 *   Type: Arithmetic Operators
 *   Operands:
 *   Stack: val => (val + 1)
 */
pub const INC: Opcode = Opcode {
    value: 234,
    name: "inc",
};
/*
 * Pops the numeric value 'val' from the stack, then pushes 'val - 1'.
 *
 *   Category: Operators
 *   Type: Arithmetic Operators
 *   Operands:
 *   Stack: val => (val - 1)
 */
pub const DEC: Opcode = Opcode {
    value: 235,
    name: "dec",
};
/*
 * Pop 'val' from the stack, then push the result of 'ToNumeric(val)'.
 *   Category: Operators
 *   Type: Arithmetic Operators
 *   Operands:
 *   Stack: val => ToNumeric(val)
 */
pub const TONUMERIC: Opcode = Opcode {
    value: 236,
    name: "tonumeric",
};
/*
 * Pushes a BigInt constant onto the stack.
 *   Category: Literals
 *   Type: Constants
 *   Operands: uint32_t constIndex
 *   Stack: => val
 */
pub const BIGINT: Opcode = Opcode {
    value: 237,
    name: "bigint",
};
/*
 * Pushes a boolean indicating if instrumentation is active.
 *   Category: Other
 *   Operands:
 *   Stack: => val
 */
pub const INSTRUMENTATION_ACTIVE: Opcode = Opcode {
    value: 238,
    name: "instrumentationActive",
};
/*
 * Pushes the instrumentation callback for the current realm.
 *   Category: Other
 *   Operands:
 *   Stack: => val
 */
pub const INSTRUMENTATION_CALLBACK: Opcode = Opcode {
    value: 239,
    name: "instrumentationCallback",
};
/*
 * Pushes the current script's instrumentation ID.
 *   Category: Other
 *   Operands:
 *   Stack: => val
 */
pub const INSTRUMENTATION_SCRIPT_ID: Opcode = Opcode {
    value: 240,
    name: "instrumentationScriptId",
};
