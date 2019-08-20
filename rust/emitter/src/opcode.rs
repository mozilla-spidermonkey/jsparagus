
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    /// No operation is performed.
    ///
    ///   Category: Other
    ///   Operands:
    ///   Stack: =>
    Nop = 0,  // JSOP_NOP

    /// Pushes 'undefined' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands:
    ///   Stack: => undefined
    Undefined = 1,  // JSOP_UNDEFINED

    /// Pushes stack frame's 'rval' onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: => rval
    GetRval = 2,  // JSOP_GETRVAL

    /// Pops the top of stack value, converts it to an object, and adds a
    /// 'WithEnvironmentObject' wrapping that object to the environment chain.
    ///
    /// There is a matching JSOP_LEAVEWITH instruction later. All name
    /// lookups between the two that may need to consult the With object
    /// are deoptimized.
    ///
    ///   Category: Statements
    ///   Type: With Statement
    ///   Operands: uint32_t staticWithIndex
    ///   Stack: val =>
    EnterWith = 3,  // JSOP_ENTERWITH

    /// Pops the environment chain object pushed by JSOP_ENTERWITH.
    ///
    ///   Category: Statements
    ///   Type: With Statement
    ///   Operands:
    ///   Stack: =>
    LeaveWith = 4,  // JSOP_LEAVEWITH

    /// Pops the top of stack value as 'rval', stops interpretation of current
    /// script and returns 'rval'.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: rval =>
    Return = 5,  // JSOP_RETURN

    /// Jumps to a 32-bit offset from the current bytecode.
    ///
    ///   Category: Statements
    ///   Type: Jumps
    ///   Operands: int32_t offset
    ///   Stack: =>
    Goto = 6,  // JSOP_GOTO

    /// Pops the top of stack value, converts it into a boolean, if the result
    /// is 'false', jumps to a 32-bit offset from the current bytecode.
    ///
    /// The idea is that a sequence like
    /// JSOP_ZERO; JSOP_ZERO; JSOP_EQ; JSOP_IFEQ; JSOP_RETURN;
    /// reads like a nice linear sequence that will execute the return.
    ///
    ///   Category: Statements
    ///   Type: Jumps
    ///   Operands: int32_t offset
    ///   Stack: cond =>
    IfEq = 7,  // JSOP_IFEQ

    /// Pops the top of stack value, converts it into a boolean, if the result
    /// is 'true', jumps to a 32-bit offset from the current bytecode.
    ///
    ///   Category: Statements
    ///   Type: Jumps
    ///   Operands: int32_t offset
    ///   Stack: cond =>
    IfNe = 8,  // JSOP_IFNE

    /// Pushes the 'arguments' object for the current function activation.
    ///
    /// If 'JSScript' is not marked 'needsArgsObj', then a
    /// JS_OPTIMIZED_ARGUMENTS magic value is pushed. Otherwise, a proper
    /// arguments object is constructed and pushed.
    ///
    /// This opcode requires that the function does not have rest parameter.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Arguments
    ///   Operands:
    ///   Stack: => arguments
    Arguments = 9,  // JSOP_ARGUMENTS

    /// Swaps the top two values on the stack. This is useful for things like
    /// post-increment/decrement.
    ///
    ///   Category: Operators
    ///   Type: Stack Operations
    ///   Operands:
    ///   Stack: v1, v2 => v2, v1
    Swap = 10,  // JSOP_SWAP

    /// Pops the top 'n' values from the stack.
    ///
    ///   Category: Operators
    ///   Type: Stack Operations
    ///   Operands: uint16_t n
    ///   Stack: v[n-1], ..., v[1], v[0] =>
    ///   nuses: n
    PopN = 11,  // JSOP_POPN

    /// Pushes a copy of the top value on the stack.
    ///
    ///   Category: Operators
    ///   Type: Stack Operations
    ///   Operands:
    ///   Stack: v => v, v
    Dup = 12,  // JSOP_DUP

    /// Duplicates the top two values on the stack.
    ///
    ///   Category: Operators
    ///   Type: Stack Operations
    ///   Operands:
    ///   Stack: v1, v2 => v1, v2, v1, v2
    Dup2 = 13,  // JSOP_DUP2

    /// Checks that the top value on the stack is an object, and throws a
    /// TypeError if not. The operand 'kind' is used only to generate an
    /// appropriate error message.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands: uint8_t kind
    ///   Stack: result => result
    CheckIsObj = 14,  // JSOP_CHECKISOBJ

    /// Pops the top two values 'lval' and 'rval' from the stack, then pushes
    /// the result of the operation applied to the two operands, converting both
    /// to 32-bit signed integers if necessary.
    ///
    ///   Category: Operators
    ///   Type: Bitwise Logical Operators
    ///   Operands:
    ///   Stack: lval, rval => (lval OP rval)
    BitOr = 15,  // JSOP_BITOR
    BitXor = 16,  // JSOP_BITXOR
    BitAnd = 17,  // JSOP_BITAND

    /// Pops the top two values from the stack and pushes the result of
    /// comparing them.
    ///
    ///   Category: Operators
    ///   Type: Comparison Operators
    ///   Operands:
    ///   Stack: lval, rval => (lval OP rval)
    Eq = 18,  // JSOP_EQ
    Ne = 19,  // JSOP_NE
    Lt = 20,  // JSOP_LT
    Le = 21,  // JSOP_LE
    Gt = 22,  // JSOP_GT
    Ge = 23,  // JSOP_GE

    /// Pops the top two values 'lval' and 'rval' from the stack, then pushes
    /// the result of the operation applied to the operands.
    ///
    ///   Category: Operators
    ///   Type: Bitwise Shift Operators
    ///   Operands:
    ///   Stack: lval, rval => (lval OP rval)
    Lsh = 24,  // JSOP_LSH
    Rsh = 25,  // JSOP_RSH

    /// Pops the top two values 'lval' and 'rval' from the stack, then pushes
    /// 'lval >>> rval'.
    ///
    ///   Category: Operators
    ///   Type: Bitwise Shift Operators
    ///   Operands:
    ///   Stack: lval, rval => (lval >>> rval)
    Ursh = 26,  // JSOP_URSH

    /// Pops the top two values 'lval' and 'rval' from the stack, then pushes
    /// the result of 'lval + rval'.
    ///
    ///   Category: Operators
    ///   Type: Arithmetic Operators
    ///   Operands:
    ///   Stack: lval, rval => (lval + rval)
    Add = 27,  // JSOP_ADD

    /// Pops the top two values 'lval' and 'rval' from the stack, then pushes
    /// the result of applying the arithmetic operation to them.
    ///
    ///   Category: Operators
    ///   Type: Arithmetic Operators
    ///   Operands:
    ///   Stack: lval, rval => (lval OP rval)
    Sub = 28,  // JSOP_SUB
    Mul = 29,  // JSOP_MUL
    Div = 30,  // JSOP_DIV
    Mod = 31,  // JSOP_MOD

    /// Pops the value 'val' from the stack, then pushes '!val'.
    ///
    ///   Category: Operators
    ///   Type: Logical Operators
    ///   Operands:
    ///   Stack: val => (!val)
    Not = 32,  // JSOP_NOT

    /// Pops the value 'val' from the stack, then pushes '~val'.
    ///
    ///   Category: Operators
    ///   Type: Bitwise Logical Operators
    ///   Operands:
    ///   Stack: val => (~val)
    BitNot = 33,  // JSOP_BITNOT

    /// Pops the value 'val' from the stack, then pushes '-val'.
    ///
    ///   Category: Operators
    ///   Type: Arithmetic Operators
    ///   Operands:
    ///   Stack: val => (-val)
    Neg = 34,  // JSOP_NEG

    /// Pops the value 'val' from the stack, then pushes '+val'.
    /// ('+val' is the value converted to a number.)
    ///
    ///   Category: Operators
    ///   Type: Arithmetic Operators
    ///   Operands:
    ///   Stack: val => (+val)
    Pos = 35,  // JSOP_POS

    /// Looks up name on the environment chain and deletes it, pushes 'true'
    /// onto the stack if succeeded (if the property was present and deleted or
    /// if the property wasn't present in the first place), 'false' if not.
    ///
    /// Strict mode code should never contain this opcode.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: => succeeded
    DelName = 36,  // JSOP_DELNAME

    /// Pops the top of stack value, deletes property from it, pushes 'true'
    /// onto the stack if succeeded, 'false' if not.
    ///
    ///   Category: Operators
    ///   Type: Special Operators
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj => succeeded
    DelProp = 37,  // JSOP_DELPROP

    /// Pops the top two values on the stack as 'propval' and 'obj', deletes
    /// 'propval' property from 'obj', pushes 'true'  onto the stack if
    /// succeeded, 'false' if not.
    ///
    ///   Category: Operators
    ///   Type: Special Operators
    ///   Operands:
    ///   Stack: obj, propval => succeeded
    DelElem = 38,  // JSOP_DELELEM

    /// Pops the value 'val' from the stack, then pushes 'typeof val'.
    ///
    ///   Category: Operators
    ///   Type: Special Operators
    ///   Operands:
    ///   Stack: val => (typeof val)
    Typeof = 39,  // JSOP_TYPEOF

    /// Pops the top value on the stack and pushes 'undefined'.
    ///
    ///   Category: Operators
    ///   Type: Special Operators
    ///   Operands:
    ///   Stack: val => undefined
    Void = 40,  // JSOP_VOID

    /// spreadcall variant of JSOP_CALL.
    ///
    /// Invokes 'callee' with 'this' and 'args', pushes the return value onto
    /// the stack.
    ///
    /// 'args' is an Array object which contains actual arguments.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: callee, this, args => rval
    SpreadCall = 41,  // JSOP_SPREADCALL

    /// spreadcall variant of JSOP_NEW
    ///
    /// Invokes 'callee' as a constructor with 'this' and 'args', pushes the
    /// return value onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: callee, this, args, newTarget => rval
    SpreadNew = 42,  // JSOP_SPREADNEW

    /// spreadcall variant of JSOP_EVAL
    ///
    /// Invokes 'eval' with 'args' and pushes the return value onto the stack.
    ///
    /// If 'eval' in global scope is not original one, invokes the function with
    /// 'this' and 'args', and pushes return value onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: callee, this, args => rval
    SpreadEval = 43,  // JSOP_SPREADEVAL

    /// Duplicates the Nth value from the top onto the stack.
    ///
    ///   Category: Operators
    ///   Type: Stack Operations
    ///   Operands: uint24_t n
    ///   Stack: v[n], v[n-1], ..., v[1], v[0] =>
    ///          v[n], v[n-1], ..., v[1], v[0], v[n]
    DupAt = 44,  // JSOP_DUPAT

    /// Push a well-known symbol onto the operand stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: uint8_t symbol (the JS::SymbolCode of the symbol to use)
    ///   Stack: => symbol
    Symbol = 45,  // JSOP_SYMBOL

    /// Pops the top of stack value and attempts to delete the given property
    /// from it. Pushes 'true' onto success, else throws a TypeError per strict
    /// mode property-deletion requirements.
    ///
    ///   Category: Operators
    ///   Type: Special Operators
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj => succeeded
    StrictDelProp = 46,  // JSOP_STRICTDELPROP

    /// Pops the top two values on the stack as 'propval' and 'obj', and
    /// attempts to delete 'propval' property from 'obj'. Pushes 'true' onto the
    /// stack on success, else throws a TypeError per strict mode property
    /// deletion requirements.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, propval => succeeded
    StrictDelElem = 47,  // JSOP_STRICTDELELEM

    /// Pops the top two values on the stack as 'val' and 'obj', and performs
    /// 'obj.prop = val', pushing 'val' back onto the stack. Throws a TypeError
    /// if the set-operation failed (per strict mode semantics).
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => val
    StrictSetProp = 48,  // JSOP_STRICTSETPROP

    /// Pops a environment and value from the stack, assigns value to the given
    /// name, and pushes the value back on the stack. If the set failed, then
    /// throw a TypeError, per usual strict mode semantics.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: env, val => val
    StrictSetName = 49,  // JSOP_STRICTSETNAME

    /// spreadcall variant of JSOP_EVAL
    ///
    /// Invokes 'eval' with 'args' and pushes the return value onto the stack.
    ///
    /// If 'eval' in global scope is not original one, invokes the function with
    /// 'this' and 'args', and pushes return value onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: callee, this, args => rval
    StrictSpreadEval = 50,  // JSOP_STRICTSPREADEVAL

    /// Ensures the result of a class's heritage expression is either null or a
    /// constructor.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: heritage => heritage
    CheckClassHeritage = 51,  // JSOP_CHECKCLASSHERITAGE

    /// Pushes a clone of a function with a given [[Prototype]] onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint32_t funcIndex
    ///   Stack: proto => obj
    FunWithProto = 52,  // JSOP_FUNWITHPROTO

    /// Pops the top of stack value, pushes property of it onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj => obj[name]
    GetProp = 53,  // JSOP_GETPROP

    /// Pops the top two values on the stack as 'val' and 'obj' and performs
    /// 'obj.prop = val', pushing 'val' back onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => val
    SetProp = 54,  // JSOP_SETPROP

    /// Pops the top two values on the stack as 'propval' and 'obj', pushes
    /// 'propval' property of 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, propval => obj[propval]
    GetElem = 55,  // JSOP_GETELEM

    /// Pops the top three values on the stack as 'val', 'propval' and 'obj',
    /// sets 'propval' property of 'obj' as 'val', pushes 'val' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, propval, val => val
    SetElem = 56,  // JSOP_SETELEM

    /// Pops the top three values on the stack as 'val', 'propval' and 'obj',
    /// sets 'propval' property of 'obj' as 'val', pushes 'val' onto the stack.
    /// Throws a TypeError if the set fails, per strict mode semantics.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, propval, val => val
    StrictSetElem = 57,  // JSOP_STRICTSETELEM

    /// Invokes 'callee' with 'this' and 'args', pushes return value onto the
    /// stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc
    ///   Stack: callee, this, args[0], ..., args[argc-1] => rval
    ///   nuses: (argc+2)
    Call = 58,  // JSOP_CALL

    /// Looks up name on the environment chain and pushes its value onto the
    /// stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: => val
    GetName = 59,  // JSOP_GETNAME

    /// Pushes numeric constant onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: DoubleValue literal
    ///   Stack: => val
    Double = 60,  // JSOP_DOUBLE

    /// Pushes string constant onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: uint32_t atomIndex
    ///   Stack: => atom
    String = 61,  // JSOP_STRING

    /// Pushes '0' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands:
    ///   Stack: => 0
    Zero = 62,  // JSOP_ZERO

    /// Pushes '1' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands:
    ///   Stack: => 1
    One = 63,  // JSOP_ONE

    /// Pushes 'null' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands:
    ///   Stack: => null
    Null = 64,  // JSOP_NULL

    /// Pushes 'JS_IS_CONSTRUCTING'
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands:
    ///   Stack: => JS_IS_CONSTRUCTING
    IsConstructing = 65,  // JSOP_IS_CONSTRUCTING

    /// Pushes boolean value onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands:
    ///   Stack: => true/false
    False = 66,  // JSOP_FALSE
    True = 67,  // JSOP_TRUE

    /// Converts the top of stack value into a boolean, if the result is 'true',
    /// jumps to a 32-bit offset from the current bytecode.
    ///
    ///   Category: Statements
    ///   Type: Jumps
    ///   Operands: int32_t offset
    ///   Stack: cond => cond
    Or = 68,  // JSOP_OR

    /// Converts the top of stack value into a boolean, if the result is
    /// 'false', jumps to a 32-bit offset from the current bytecode.
    ///
    ///   Category: Statements
    ///   Type: Jumps
    ///   Operands: int32_t offset
    ///   Stack: cond => cond
    And = 69,  // JSOP_AND

    /// Pops the top of stack value as 'i', if 'low <= i <= high',
    /// jumps to a 32-bit offset: offset is stored in the script's resumeOffsets
    ///                           list at index 'firstResumeIndex + (i - low)'
    /// jumps to a 32-bit offset: 'len' from the current bytecode otherwise
    ///
    ///   Category: Statements
    ///   Type: Switch Statement
    ///   Operands: int32_t len, int32_t low, int32_t high,
    ///             uint24_t firstResumeIndex
    ///   Stack: i =>
    ///   len: len
    TableSwitch = 70,  // JSOP_TABLESWITCH

    Unused71 = 71,  // JSOP_UNUSED71

    /// Pops the top two values from the stack, then pushes the result of
    /// applying the operator to the two values.
    ///
    ///   Category: Operators
    ///   Type: Comparison Operators
    ///   Operands:
    ///   Stack: lval, rval => (lval OP rval)
    StrictEq = 72,  // JSOP_STRICTEQ
    StrictNe = 73,  // JSOP_STRICTNE

    /// Sometimes we know when emitting that an operation will always throw.
    ///
    /// Throws the indicated JSMSG.
    ///
    ///   Category: Statements
    ///   Type: Exception Handling
    ///   Operands: uint16_t msgNumber
    ///   Stack: =>
    ThrowMsg = 74,  // JSOP_THROWMSG

    /// Sets up a for-in loop. It pops the top of stack value as 'val' and
    /// pushes 'iter' which is an iterator for 'val'.
    ///
    ///   Category: Statements
    ///   Type: For-In Statement
    ///   Operands:
    ///   Stack: val => iter
    Iter = 75,  // JSOP_ITER

    /// Pushes the next iterated value onto the stack. If no value is available,
    /// MagicValue(JS_NO_ITER_VALUE) is pushed.
    ///
    ///   Category: Statements
    ///   Type: For-In Statement
    ///   Operands:
    ///   Stack: iter => iter, val
    MoreIter = 76,  // JSOP_MOREITER

    /// Pushes a boolean indicating whether the value on top of the stack is
    /// MagicValue(JS_NO_ITER_VALUE).
    ///
    ///   Category: Statements
    ///   Type: For-In Statement
    ///   Operands:
    ///   Stack: val => val, res
    IsNoIter = 77,  // JSOP_ISNOITER

    /// Exits a for-in loop by popping the iterator object from the stack and
    /// closing it.
    ///
    ///   Category: Statements
    ///   Type: For-In Statement
    ///   Operands:
    ///   Stack: iter =>
    EndIter = 78,  // JSOP_ENDITER

    /// Invokes 'callee' with 'this' and 'args', pushes return value onto the
    /// stack.
    ///
    /// This is for 'f.apply'.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc
    ///   Stack: callee, this, args[0], ..., args[argc-1] => rval
    ///   nuses: (argc+2)
    FunApply = 79,  // JSOP_FUNAPPLY

    /// Pushes deep-cloned object literal or singleton onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t objectIndex
    ///   Stack: => obj
    Object = 80,  // JSOP_OBJECT

    /// Pops the top value off the stack.
    ///
    ///   Category: Operators
    ///   Type: Stack Operations
    ///   Operands:
    ///   Stack: v =>
    Pop = 81,  // JSOP_POP

    /// Invokes 'callee' as a constructor with 'this' and 'args', pushes return
    /// value onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc
    ///   Stack: callee, this, args[0], ..., args[argc-1], newTarget => rval
    ///   nuses: (argc+3)
    New = 82,  // JSOP_NEW

    /// Pushes newly created object onto the stack with provided [[Prototype]].
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: proto => obj
    ObjWithProto = 83,  // JSOP_OBJWITHPROTO

    /// Fast get op for function arguments and local variables.
    ///
    /// Pushes 'arguments[argno]' onto the stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Arguments
    ///   Operands: uint16_t argno
    ///   Stack: => arguments[argno]
    GetArg = 84,  // JSOP_GETARG

    /// Fast set op for function arguments and local variables.
    ///
    /// Sets 'arguments[argno]' as the top of stack value.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Arguments
    ///   Operands: uint16_t argno
    ///   Stack: v => v
    SetArg = 85,  // JSOP_SETARG

    /// Pushes the value of local variable onto the stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Local Variables
    ///   Operands: uint24_t localno
    ///   Stack: => val
    GetLocal = 86,  // JSOP_GETLOCAL

    /// Stores the top stack value to the given local.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Local Variables
    ///   Operands: uint24_t localno
    ///   Stack: v => v
    SetLocal = 87,  // JSOP_SETLOCAL

    /// Pushes unsigned 16-bit int immediate integer operand onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: uint16_t val
    ///   Stack: => val
    Uint16 = 88,  // JSOP_UINT16

    /// Pushes newly created object onto the stack.
    ///
    /// This opcode has four extra bytes so it can be exchanged with
    /// JSOP_NEWOBJECT during emit.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: (uint32_t extra)
    ///   Stack: => obj
    NewInit = 89,  // JSOP_NEWINIT

    /// Pushes newly created array onto the stack.
    ///
    /// This opcode takes the final length, which is preallocated.
    ///
    ///   Category: Literals
    ///   Type: Array
    ///   Operands: uint32_t length
    ///   Stack: => obj
    NewArray = 90,  // JSOP_NEWARRAY

    /// Pushes newly created object onto the stack.
    ///
    /// This opcode takes an object with the final shape, which can be set at
    /// the start and slots then filled in directly.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t baseobjIndex
    ///   Stack: => obj
    NewObject = 91,  // JSOP_NEWOBJECT

    /// Initialize the home object for functions with super bindings.
    ///
    /// This opcode takes the function and the object to be the home object,
    /// does the set, and leaves the function on the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: fun, homeObject => fun
    Inithomeobject = 92,  // JSOP_INITHOMEOBJECT

    /// Initialize a named property in an object literal, like '{a: x}'.
    ///
    /// Pops the top two values on the stack as 'val' and 'obj', defines
    /// 'nameIndex' property of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => obj
    InitProp = 93,  // JSOP_INITPROP

    /// Initialize a numeric property in an object literal, like '{1: x}'.
    ///
    /// Pops the top three values on the stack as 'val', 'id' and 'obj', defines
    /// 'id' property of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, id, val => obj
    InitElem = 94,  // JSOP_INITELEM

    /// Pops the top three values on the stack as 'val', 'index' and 'obj', sets
    /// 'index' property of 'obj' as 'val', pushes 'obj' and 'index + 1' onto
    /// the stack.
    ///
    /// This opcode is used in Array literals with spread and spreadcall
    /// arguments.
    ///
    ///   Category: Literals
    ///   Type: Array
    ///   Operands:
    ///   Stack: obj, index, val => obj, (index + 1)
    InitElemInc = 95,  // JSOP_INITELEM_INC

    /// Initialize an array element.
    ///
    /// Pops the top two values on the stack as 'val' and 'obj', sets 'index'
    /// property of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Array
    ///   Operands: uint32_t index
    ///   Stack: obj, val => obj
    InitElemArray = 96,  // JSOP_INITELEM_ARRAY

    /// Initialize a getter in an object literal.
    ///
    /// Pops the top two values on the stack as 'val' and 'obj', defines getter
    /// of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => obj
    InitPropGetter = 97,  // JSOP_INITPROP_GETTER

    /// Initialize a setter in an object literal.
    ///
    /// Pops the top two values on the stack as 'val' and 'obj', defines setter
    /// of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => obj
    InitPropSetter = 98,  // JSOP_INITPROP_SETTER

    /// Initialize a numeric getter in an object literal like
    /// '{get 2() {}}'.
    ///
    /// Pops the top three values on the stack as 'val', 'id' and 'obj', defines
    /// 'id' getter of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, id, val => obj
    InitElemGetter = 99,  // JSOP_INITELEM_GETTER

    /// Initialize a numeric setter in an object literal like
    /// '{set 2(v) {}}'.
    ///
    /// Pops the top three values on the stack as 'val', 'id' and 'obj', defines
    /// 'id' setter of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, id, val => obj
    InitElemSetter = 100,  // JSOP_INITELEM_SETTER

    /// Pushes the call site object specified by objectIndex onto the stack.
    /// Defines the raw property specified by objectIndex + 1 on the call site
    /// object and freezes both the call site object as well as its raw
    /// property.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t objectIndex
    ///   Stack: => obj
    CallSiteObj = 101,  // JSOP_CALLSITEOBJ

    /// Pushes a newly created array onto the stack, whose elements are the same
    /// as that of a template object's copy on write elements.
    ///
    ///   Category: Literals
    ///   Type: Array
    ///   Operands: uint32_t objectIndex
    ///   Stack: => obj
    NewArrayCopyOnWrite = 102,  // JSOP_NEWARRAY_COPYONWRITE

    /// Pushes the prototype of the home object for |callee| onto the
    /// stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Super
    ///   Operands:
    ///   Stack: callee => homeObjectProto
    SuperBase = 103,  // JSOP_SUPERBASE

    /// Pops the top two values, and pushes the property of one, using the other
    /// as the receiver.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: receiver, obj => obj[name]
    GetPropSuper = 104,  // JSOP_GETPROP_SUPER

    /// Pops the top three values on the stack as 'val' and 'obj', and
    /// 'receiver', and performs 'obj.prop = val', pushing 'val' back onto the
    /// stack. Throws a TypeError if the set-operation failed (per strict mode
    /// semantics).
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: receiver, obj, val => val
    StrictSetPropSuper = 105,  // JSOP_STRICTSETPROP_SUPER

    /// This opcode precedes every labeled statement. It's a no-op.
    ///
    /// 'offset' is the offset to the next instruction after this statement, the
    /// one 'break LABEL;' would jump to. IonMonkey uses this.
    ///
    ///   Category: Statements
    ///   Type: Jumps
    ///   Operands: int32_t offset
    ///   Stack: =>
    Label = 106,  // JSOP_LABEL

    /// Pops the top three values on the stack as 'val', 'obj' and 'receiver',
    /// and performs 'obj.prop = val', pushing 'val' back onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: receiver, obj, val => val
    SetPropSuper = 107,  // JSOP_SETPROP_SUPER

    /// Invokes 'callee' with 'this' and 'args', pushes return value onto the
    /// stack.
    ///
    /// If 'callee' is determined to be the canonical 'Function.prototype.call'
    /// function, then this operation is optimized to directly call 'callee'
    /// with 'args[0]' as 'this', and the remaining arguments as formal args to
    /// 'callee'.
    ///
    /// Like JSOP_FUNAPPLY but for 'f.call' instead of 'f.apply'.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc
    ///   Stack: callee, this, args[0], ..., args[argc-1] => rval
    ///   nuses: (argc+2)
    FunCall = 108,  // JSOP_FUNCALL

    /// Another no-op.
    ///
    /// This opcode is the target of the backwards jump for some loop.
    /// See JSOP_JUMPTARGET for the icIndex operand.
    ///
    ///   Category: Statements
    ///   Type: Jumps
    ///   Operands: uint32_t icIndex
    ///   Stack: =>
    LoopHead = 109,  // JSOP_LOOPHEAD

    /// Looks up name on the environment chain and pushes the environment which
    /// contains the name onto the stack. If not found, pushes global lexical
    /// environment onto the stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: => env
    BindName = 110,  // JSOP_BINDNAME

    /// Pops an environment and value from the stack, assigns value to the given
    /// name, and pushes the value back on the stack
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: env, val => val
    SetName = 111,  // JSOP_SETNAME

    /// Pops the top of stack value as 'v', sets pending exception as 'v', then
    /// raises error.
    ///
    ///   Category: Statements
    ///   Type: Exception Handling
    ///   Operands:
    ///   Stack: v =>
    Throw = 112,  // JSOP_THROW

    /// Pops the top two values 'id' and 'obj' from the stack, then pushes
    /// 'id in obj'. This will throw a 'TypeError' if 'obj' is not an object.
    ///
    /// Note that 'obj' is the top value.
    ///
    ///   Category: Operators
    ///   Type: Special Operators
    ///   Operands:
    ///   Stack: id, obj => (id in obj)
    In = 113,  // JSOP_IN

    /// Pops the top two values 'obj' and 'ctor' from the stack, then pushes
    /// 'obj instanceof ctor'. This will throw a 'TypeError' if 'obj' is not an
    /// object.
    ///
    ///   Category: Operators
    ///   Type: Special Operators
    ///   Operands:
    ///   Stack: obj, ctor => (obj instanceof ctor)
    Instanceof = 114,  // JSOP_INSTANCEOF

    /// Invokes debugger.
    ///
    ///   Category: Statements
    ///   Type: Debugger
    ///   Operands:
    ///   Stack: =>
    Debugger = 115,  // JSOP_DEBUGGER

    /// This opcode is used for entering a 'finally' block. Jumps to a 32-bit
    /// offset from the current pc.
    ///
    /// Note: this op doesn't actually push/pop any values, but it has a use
    /// count of 2 (for the 'false' + resumeIndex values pushed by preceding
    /// bytecode ops) because the 'finally' entry point does not expect these
    /// values on the stack. See also JSOP_FINALLY (it has a def count of 2).
    ///
    /// When the execution resumes from 'finally' block, those stack values are
    /// popped.
    ///
    ///   Category: Statements
    ///   Type: Exception Handling
    ///   Operands: int32_t offset
    ///   Stack: false, resumeIndex =>
    Gosub = 116,  // JSOP_GOSUB

    /// This opcode is used for returning from a 'finally' block.
    ///
    /// Pops the top two values on the stack as 'rval' and 'lval'. Then:
    /// - If 'lval' is true, throws 'rval'.
    /// - If 'lval' is false, jumps to the resumeIndex stored in 'lval'.
    ///
    ///   Category: Statements
    ///   Type: Exception Handling
    ///   Operands:
    ///   Stack: lval, rval =>
    Retsub = 117,  // JSOP_RETSUB

    /// Pushes the current pending exception onto the stack and clears the
    /// pending exception. This is only emitted at the beginning of code for a
    /// catch-block, so it is known that an exception is pending. It is used to
    /// implement catch-blocks and 'yield*'.
    ///
    ///   Category: Statements
    ///   Type: Exception Handling
    ///   Operands:
    ///   Stack: => exception
    Exception = 118,  // JSOP_EXCEPTION

    /// Embedded lineno to speedup 'pc->line' mapping.
    ///
    ///   Category: Other
    ///   Operands: uint32_t lineno
    ///   Stack: =>
    Lineno = 119,  // JSOP_LINENO

    /// This no-op appears after the bytecode for EXPR in 'switch (EXPR) {...}'
    /// if the switch cannot be optimized using JSOP_TABLESWITCH.
    ///
    /// For a non-optimized switch statement like this:
    ///
    /// ```js
    /// switch (EXPR) {
    ///   case V0:
    ///     C0;
    ///   ...
    ///   default:
    ///     D;
    /// }
    /// ```
    ///
    /// the bytecode looks like this:
    ///
    /// ```ignore
    /// (EXPR)
    /// condswitch
    /// (V0)
    /// case ->C0
    /// ...
    /// default ->D
    /// (C0)
    /// ...
    /// (D)
    /// ```
    ///
    /// Note that code for all case-labels is emitted first, then code for the
    /// body of each case clause.
    ///
    ///   Category: Statements
    ///   Type: Switch Statement
    ///   Operands:
    ///   Stack: =>
    CondSwitch = 120,  // JSOP_CONDSWITCH

    /// Pops the top two values on the stack as 'val' and 'cond'. If 'cond' is
    /// 'true', jumps to a 32-bit offset from the current bytecode, re-pushes
    /// 'val' onto the stack if 'false'.
    ///
    ///   Category: Statements
    ///   Type: Switch Statement
    ///   Operands: int32_t offset
    ///   Stack: val, cond => val(if !cond)
    Case = 121,  // JSOP_CASE

    /// This appears after all cases in a JSOP_CONDSWITCH, whether there is a
    /// 'default:' label in the switch statement or not. Pop the switch operand
    /// from the stack and jump to a 32-bit offset from the current bytecode.
    /// offset from the current bytecode.
    ///
    ///   Category: Statements
    ///   Type: Switch Statement
    ///   Operands: int32_t offset
    ///   Stack: lval =>
    Default = 122,  // JSOP_DEFAULT

    /// Invokes 'eval' with 'args' and pushes return value onto the stack.
    ///
    /// If 'eval' in global scope is not original one, invokes the function with
    /// 'this' and 'args', and pushes return value onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc
    ///   Stack: callee, this, args[0], ..., args[argc-1] => rval
    ///   nuses: (argc+2)
    Eval = 123,  // JSOP_EVAL

    /// Invokes 'eval' with 'args' and pushes return value onto the stack.
    ///
    /// If 'eval' in global scope is not original one, invokes the function with
    /// 'this' and 'args', and pushes return value onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc
    ///   Stack: callee, this, args[0], ..., args[argc-1] => rval
    ///   nuses: (argc+2)
    StrictEval = 124,  // JSOP_STRICTEVAL

    /// LIKE JSOP_GETELEM but takes receiver on stack, and the propval is
    /// evaluated before the obj.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: receiver, propval, obj => obj[propval]
    GetElemSuper = 125,  // JSOP_GETELEM_SUPER

    /// Pushes a resumeIndex (stored as 24-bit operand) on the stack.
    ///
    /// Resume indexes are used for ops like JSOP_YIELD and JSOP_GOSUB.
    /// JSScript and BaselineScript have lists of resume entries (one for each
    /// resumeIndex); this lets the JIT resume at these ops from JIT code.
    ///
    ///   Category: Other
    ///   Operands: uint24_t resumeIndex
    ///   Stack: => resumeIndex
    ResumeIndex = 126,  // JSOP_RESUMEINDEX

    /// Defines the given function on the current scope.
    ///
    /// This is used for global scripts and also in some cases for function
    /// scripts where use of dynamic scoping inhibits optimization.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands:
    ///   Stack: fun =>
    DefFun = 127,  // JSOP_DEFFUN

    /// Defines the new constant binding on global lexical environment.
    ///
    /// Throws if a binding with the same name already exists on the
    /// environment, or if a var binding with the same name exists on the
    /// global.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: =>
    DefConst = 128,  // JSOP_DEFCONST

    /// Defines the new binding on the frame's current variables-object (the
    /// environment on the environment chain designated to receive new
    /// variables).
    ///
    /// Throws if the current variables-object is the global object and a
    /// binding with the same name exists on the global lexical environment.
    ///
    /// This is used for global scripts and also in some cases for function
    /// scripts where use of dynamic scoping inhibits optimization.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: =>
    DefVar = 129,  // JSOP_DEFVAR

    /// Pushes a closure for a named or anonymous function expression onto the
    /// stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint32_t funcIndex
    ///   Stack: => obj
    Lambda = 130,  // JSOP_LAMBDA

    /// Pops the top of stack value as 'new.target', pushes an arrow function
    /// with lexical 'new.target' onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint32_t funcIndex
    ///   Stack: new.target => obj
    LambdaArrow = 131,  // JSOP_LAMBDA_ARROW

    /// Pushes current callee onto the stack.
    ///
    /// Used for named function expression self-naming, if lightweight.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Arguments
    ///   Operands:
    ///   Stack: => callee
    Callee = 132,  // JSOP_CALLEE

    /// Picks the nth element from the stack and moves it to the top of the
    /// stack.
    ///
    ///   Category: Operators
    ///   Type: Stack Operations
    ///   Operands: uint8_t n
    ///   Stack: v[n], v[n-1], ..., v[1], v[0] => v[n-1], ..., v[1], v[0], v[n]
    Pick = 133,  // JSOP_PICK

    /// This no-op appears at the top of the bytecode for a 'TryStatement'.
    ///
    /// Location information for catch/finally blocks is stored in a side table,
    /// 'script->trynotes()'.
    ///
    ///   Category: Statements
    ///   Type: Exception Handling
    ///   Operands:
    ///   Stack: =>
    Try = 134,  // JSOP_TRY

    /// This opcode has a def count of 2, but these values are already on the
    /// stack (they're pushed by JSOP_GOSUB).
    ///
    ///   Category: Statements
    ///   Type: Exception Handling
    ///   Operands:
    ///   Stack: => false, resumeIndex
    Finally = 135,  // JSOP_FINALLY

    /// Pushes aliased variable onto the stack.
    ///
    /// An "aliased variable" is a var, let, or formal arg that is aliased.
    /// Sources of aliasing include: nested functions accessing the vars of an
    /// enclosing function, function statements that are conditionally executed,
    /// 'eval', 'with', and 'arguments'. All of these cases require creating a
    /// CallObject to own the aliased variable.
    ///
    /// An ALIASEDVAR opcode contains the following immediates:
    ///  uint8 hops: the number of environment objects to skip to find the
    ///               EnvironmentObject containing the variable being accessed
    ///  uint24 slot: the slot containing the variable in the EnvironmentObject
    ///               (this 'slot' does not include RESERVED_SLOTS).
    ///
    ///   Category: Variables and Scopes
    ///   Type: Aliased Variables
    ///   Operands: uint8_t hops, uint24_t slot
    ///   Stack: => aliasedVar
    GetAliasedVar = 136,  // JSOP_GETALIASEDVAR

    /// Sets aliased variable as the top of stack value.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Aliased Variables
    ///   Operands: uint8_t hops, uint24_t slot
    ///   Stack: v => v
    SetAliasedVar = 137,  // JSOP_SETALIASEDVAR

    /// Checks if the value of the local variable is the
    /// JS_UNINITIALIZED_LEXICAL magic, throwing an error if so.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Local Variables
    ///   Operands: uint24_t localno
    ///   Stack: =>
    CheckLexical = 138,  // JSOP_CHECKLEXICAL

    /// Initializes an uninitialized local lexical binding with the top of stack
    /// value.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Local Variables
    ///   Operands: uint24_t localno
    ///   Stack: v => v
    InitLexical = 139,  // JSOP_INITLEXICAL

    /// Checks if the value of the aliased variable is the
    /// JS_UNINITIALIZED_LEXICAL magic, throwing an error if so.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Aliased Variables
    ///   Operands: uint8_t hops, uint24_t slot
    ///   Stack: =>
    CheckAliasedLexical = 140,  // JSOP_CHECKALIASEDLEXICAL

    /// Initializes an uninitialized aliased lexical binding with the top of
    /// stack value.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Aliased Variables
    ///   Operands: uint8_t hops, uint24_t slot
    ///   Stack: v => v
    InitAliasedLexical = 141,  // JSOP_INITALIASEDLEXICAL

    /// Pushes a JS_UNINITIALIZED_LEXICAL value onto the stack, representing an
    /// uninitialized lexical binding.
    ///
    /// This opcode is used with the JSOP_INITLEXICAL opcode.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands:
    ///   Stack: => uninitialized
    Uninitialized = 142,  // JSOP_UNINITIALIZED

    /// Pushes the value of the intrinsic onto the stack.
    ///
    /// Intrinsic names are emitted instead of JSOP_*NAME ops when the
    /// 'CompileOptions' flag 'selfHostingMode' is set.
    ///
    /// They are used in self-hosted code to access other self-hosted values and
    /// intrinsic functions the runtime doesn't give client JS code access to.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Intrinsics
    ///   Operands: uint32_t nameIndex
    ///   Stack: => intrinsic[name]
    GetIntrinsic = 143,  // JSOP_GETINTRINSIC

    /// Stores the top stack value in the specified intrinsic.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Intrinsics
    ///   Operands: uint32_t nameIndex
    ///   Stack: val => val
    SetIntrinsic = 144,  // JSOP_SETINTRINSIC

    /// Like JSOP_CALL, but used as part of for-of and destructuring bytecode to
    /// provide better error messages.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc (must be 0)
    ///   Stack: callee, this => rval
    ///   nuses: 2
    CallIter = 145,  // JSOP_CALLITER

    /// Initialize a non-configurable, non-writable, non-enumerable
    /// data-property on an object.
    ///
    /// Pops the top two values on the stack as 'val' and 'obj', defines
    /// 'nameIndex' property of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => obj
    InitLockedProp = 146,  // JSOP_INITLOCKEDPROP

    /// Initialize a non-enumerable data-property on an object.
    ///
    /// Pops the top two values on the stack as 'val' and 'obj', defines
    /// 'nameIndex' property of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => obj
    InitHiddenProp = 147,  // JSOP_INITHIDDENPROP

    /// Push "new.target"
    ///
    ///   Category: Variables and Scopes
    ///   Type: Arguments
    ///   Operands:
    ///   Stack: => new.target
    NewTarget = 148,  // JSOP_NEWTARGET

    Unused149 = 149,  // JSOP_UNUSED149

    /// Pops the top two values 'lval' and 'rval' from the stack, then pushes
    /// the result of 'Math.pow(lval, rval)'.
    ///
    ///   Category: Operators
    ///   Type: Arithmetic Operators
    ///   Operands:
    ///   Stack: lval, rval => (lval ** rval)
    Pow = 150,  // JSOP_POW

    /// Pops the top two values 'value' and 'gen' from the stack, then starts
    /// "awaiting" for 'value' to be resolved, which will then resume the
    /// execution of 'gen'. Pushes the async function promise on the stack, so
    /// that it'll be returned to the caller on the very first "await".
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands:
    ///   Stack: value, gen => promise
    AsyncAwait = 151,  // JSOP_ASYNCAWAIT

    /// Pops the top of stack value as 'rval', sets the return value in stack
    /// frame as 'rval'.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: rval =>
    SetRval = 152,  // JSOP_SETRVAL

    /// Stops interpretation and returns value set by JSOP_SETRVAL. When not
    /// set, returns 'undefined'.
    ///
    /// Also emitted at end of script so interpreter don't need to check if
    /// opcode is still in script range.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: =>
    RetRval = 153,  // JSOP_RETRVAL

    /// Looks up name on global environment and pushes its value onto the stack,
    /// unless the script has a non-syntactic global scope, in which case it
    /// acts just like JSOP_NAME.
    ///
    /// Free variable references that must either be found on the global or a
    /// ReferenceError.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Free Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: => val
    GetGname = 154,  // JSOP_GETGNAME

    /// Pops the top two values on the stack as 'val' and 'env', sets property
    /// of 'env' as 'val' and pushes 'val' back on the stack.
    ///
    /// 'env' should be the global lexical environment unless the script has a
    /// non-syntactic global scope, in which case acts like JSOP_SETNAME.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Free Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: env, val => val
    SetGname = 155,  // JSOP_SETGNAME

    /// Pops the top two values on the stack as 'val' and 'env', sets property
    /// of 'env' as 'val' and pushes 'val' back on the stack. Throws a TypeError
    /// if the set fails, per strict mode semantics.
    ///
    /// 'env' should be the global lexical environment unless the script has a
    /// non-syntactic global scope, in which case acts like JSOP_STRICTSETNAME.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Free Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: env, val => val
    StrictSetGname = 156,  // JSOP_STRICTSETGNAME

    /// Pushes the implicit 'this' value for calls to the associated name onto
    /// the stack; only used when the implicit this might be derived from a
    /// non-syntactic scope (instead of the global itself).
    ///
    /// Note that code evaluated via the Debugger API uses DebugEnvironmentProxy
    /// objects on its scope chain, which are non-syntactic environments that
    /// refer to syntactic environments. As a result, the binding we want may be
    /// held by a syntactic environments such as CallObject or
    /// VarEnvrionmentObject.
    ///
    ///   Category: Variables and Scopes
    ///   Type: This
    ///   Operands: uint32_t nameIndex
    ///   Stack: => this
    GImplicitThis = 157,  // JSOP_GIMPLICITTHIS

    /// LIKE JSOP_SETELEM, but takes receiver on the stack, and the propval is
    /// evaluated before the base.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: receiver, propval, obj, val => val
    SetElemSuper = 158,  // JSOP_SETELEM_SUPER

    /// LIKE JSOP_STRICTSETELEM, but takes receiver on the stack, and the
    /// propval is evaluated before the base.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: receiver, propval, obj, val => val
    StrictSetElemSuper = 159,  // JSOP_STRICTSETELEM_SUPER

    /// Pushes a regular expression literal onto the stack. It requires special
    /// "clone on exec" handling.
    ///
    ///   Category: Literals
    ///   Type: RegExp
    ///   Operands: uint32_t regexpIndex
    ///   Stack: => regexp
    RegExp = 160,  // JSOP_REGEXP

    /// Initializes an uninitialized global lexical binding with the top of
    /// stack value.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Free Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: val => val
    InitGLexical = 161,  // JSOP_INITGLEXICAL

    /// Defines the new mutable binding on global lexical environment.
    ///
    /// Throws if a binding with the same name already exists on the
    /// environment, or if a var binding with the same name exists on the
    /// global.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: =>
    DefLet = 162,  // JSOP_DEFLET

    /// Throw if the value on the stack is not coerscible to an object (is
    /// |null| or |undefined|).
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: val => val
    CheckObjCoercible = 163,  // JSOP_CHECKOBJCOERCIBLE

    /// Push the function to invoke with |super()|. This is the prototype of the
    /// function passed in as |callee|.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Super
    ///   Operands:
    ///   Stack: callee => superFun
    SuperFun = 164,  // JSOP_SUPERFUN

    /// Behaves exactly like JSOP_NEW, but allows JITs to distinguish the two
    /// cases.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc
    ///   Stack: callee, this, args[0], ..., args[argc-1], newTarget => rval
    ///   nuses: (argc+3)
    SuperCall = 165,  // JSOP_SUPERCALL

    /// spreadcall variant of JSOP_SUPERCALL.
    ///
    /// Behaves exactly like JSOP_SPREADNEW.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: callee, this, args, newTarget => rval
    SpreadSuperCall = 166,  // JSOP_SPREADSUPERCALL

    /// Push a default constructor for a base class literal.
    ///
    ///   Category: Literals
    ///   Type: Class
    ///   Operands: atom className
    ///   Stack: => constructor
    ClassConstructor = 167,  // JSOP_CLASSCONSTRUCTOR

    /// Push a default constructor for a derived class literal.
    ///
    ///   Category: Literals
    ///   Type: Class
    ///   Operands: atom className
    ///   Stack: proto => constructor
    DerivedConstructor = 168,  // JSOP_DERIVEDCONSTRUCTOR

    /// Throws a runtime TypeError for invalid assignment to 'const'. The
    /// localno is used for better error messages.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Local Variables
    ///   Operands: uint24_t localno
    ///   Stack: v => v
    ThrowSetConst = 169,  // JSOP_THROWSETCONST

    /// Throws a runtime TypeError for invalid assignment to 'const'. The
    /// environment coordinate is used for better error messages.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Aliased Variables
    ///   Operands: uint8_t hops, uint24_t slot
    ///   Stack: v => v
    ThrowSetAliasedConst = 170,  // JSOP_THROWSETALIASEDCONST

    /// Initialize a non-enumerable getter in an object literal.
    ///
    /// Pops the top two values on the stack as 'val' and 'obj', defines getter
    /// of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => obj
    InitHiddenPropGetter = 171,  // JSOP_INITHIDDENPROP_GETTER

    /// Initialize a non-enumerable setter in an object literal.
    ///
    /// Pops the top two values on the stack as 'val' and 'obj', defines setter
    /// of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj, val => obj
    InitHiddenPropSetter = 172,  // JSOP_INITHIDDENPROP_SETTER

    /// Initialize a non-enumerable numeric getter in an object literal like
    /// '{get 2() {}}'.
    ///
    /// Pops the top three values on the stack as 'val', 'id' and 'obj', defines
    /// 'id' getter of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, id, val => obj
    InitHiddenElemGetter = 173,  // JSOP_INITHIDDENELEM_GETTER

    /// Initialize a non-enumerable numeric setter in an object literal like
    /// '{set 2(v) {}}'.
    ///
    /// Pops the top three values on the stack as 'val', 'id' and 'obj', defines
    /// 'id' setter of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, id, val => obj
    InitHiddenElemSetter = 174,  // JSOP_INITHIDDENELEM_SETTER

    /// Initialize a non-enumerable numeric property in an object literal, like
    /// '{1: x}'.
    ///
    /// Pops the top three values on the stack as 'val', 'id' and 'obj', defines
    /// 'id' property of 'obj' as 'val', pushes 'obj' onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, id, val => obj
    InitHiddenElem = 175,  // JSOP_INITHIDDENELEM

    /// Gets the value of a module import by name and pushes it onto the stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: => val
    GetImport = 176,  // JSOP_GETIMPORT

    /// Examines the top stack value, asserting that it's either a self-hosted
    /// function or a self-hosted intrinsic. This opcode does nothing in a
    /// non-debug build.
    ///
    ///   Category: Other
    ///   Operands:
    ///   Stack: checkVal => checkVal
    DebugCheckSelfHosted = 177,  // JSOP_DEBUGCHECKSELFHOSTED

    /// Pops the top stack value, pushes the value and a boolean value that
    /// indicates whether the spread operation for the value can be optimized in
    /// spread call.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: arr => arr, optimized
    OptimizeSpreadCall = 178,  // JSOP_OPTIMIZE_SPREADCALL

    /// Throws a runtime TypeError for invalid assignment to the callee in a
    /// named lambda, which is always a 'const' binding. This is a different
    /// bytecode than JSOP_SETCONST because the named lambda callee, if not
    /// closed over, does not have a frame slot to look up the name with for the
    /// error message.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Local Variables
    ///   Operands:
    ///   Stack: v => v
    ThrowSetCallee = 179,  // JSOP_THROWSETCALLEE

    /// Pushes a var environment onto the env chain.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Var Scope
    ///   Operands: uint32_t scopeIndex
    ///   Stack: =>
    PushVarEnv = 180,  // JSOP_PUSHVARENV

    /// Pops a var environment from the env chain.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Var Scope
    ///   Operands:
    ///   Stack: =>
    PopVarEnv = 181,  // JSOP_POPVARENV

    /// Pops the top two values on the stack as 'name' and 'fun', defines the
    /// name of 'fun' to 'name' with prefix if any, and pushes 'fun' back onto
    /// the stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint8_t prefixKind
    ///   Stack: fun, name => fun
    SetFunName = 182,  // JSOP_SETFUNNAME

    /// Moves the top of the stack value under the nth element of the stack.
    /// Note: n must NOT be 0.
    ///
    ///   Category: Operators
    ///   Type: Stack Operations
    ///   Operands: uint8_t n
    ///   Stack: v[n], v[n-1], ..., v[1], v[0] => v[0], v[n], v[n-1], ..., v[1]
    Unpick = 183,  // JSOP_UNPICK

    /// Pops the top of stack value, pushes property of it onto the stack.
    /// Requires the value under 'obj' to be the receiver of the following call.
    ///
    /// Like JSOP_GETPROP but for call context.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj => obj[name]
    CallProp = 184,  // JSOP_CALLPROP

    /// Determines the 'this' value for current function frame and pushes it
    /// onto the stack. Emitted in the prologue of functions with a
    /// this-binding.
    ///
    ///   Category: Variables and Scopes
    ///   Type: This
    ///   Operands:
    ///   Stack: => this
    FunctionThis = 185,  // JSOP_FUNCTIONTHIS

    /// Pushes 'this' value for current stack frame onto the stack. Emitted when
    /// 'this' refers to the global 'this'.
    ///
    ///   Category: Variables and Scopes
    ///   Type: This
    ///   Operands:
    ///   Stack: => this
    GlobalThis = 186,  // JSOP_GLOBALTHIS

    /// Pushes a boolean indicating whether the top of the stack is
    /// MagicValue(JS_GENERATOR_CLOSING).
    ///
    ///   Category: Statements
    ///   Type: For-In Statement
    ///   Operands:
    ///   Stack: val => val, res
    IsGenClosing = 187,  // JSOP_ISGENCLOSING

    /// Pushes unsigned 24-bit int immediate integer operand onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: uint24_t val
    ///   Stack: => val
    Uint24 = 188,  // JSOP_UINT24

    /// Throw if the value on top of the stack is the TDZ MagicValue. Used in
    /// derived class constructors.
    ///
    ///   Category: Variables and Scopes
    ///   Type: This
    ///   Operands:
    ///   Stack: this => this
    CheckThis = 189,  // JSOP_CHECKTHIS

    /// Check if a derived class constructor has a valid return value and 'this'
    /// value before it returns. If the return value is not an object, stores
    /// the 'this' value to the return value slot.
    ///
    ///   Category: Variables and Scopes
    ///   Type: This
    ///   Operands:
    ///   Stack: this =>
    CheckReturn = 190,  // JSOP_CHECKRETURN

    /// Throw an exception if the value on top of the stack is not the TDZ
    /// MagicValue. Used in derived class constructors.
    ///
    ///   Category: Variables and Scopes
    ///   Type: This
    ///   Operands:
    ///   Stack: this => this
    CheckThisReinit = 191,  // JSOP_CHECKTHISREINIT

    /// Pops the top two values 'valueOrReason' and 'gen' from the stack, then
    /// pushes the promise resolved with 'valueOrReason'. `gen` must be the
    /// internal generator object created in async functions. The pushed promise
    /// is the async function's result promise, which is stored in `gen`.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands: uint8_t fulfillOrReject
    ///   Stack: valueOrReason, gen => promise
    AsyncResolve = 192,  // JSOP_ASYNCRESOLVE

    /// Pops the top two values on the stack as 'propval' and 'obj', pushes
    /// 'propval' property of 'obj' onto the stack. Requires the value under
    /// 'obj' to be the receiver of the following call.
    ///
    /// Like JSOP_GETELEM but for call context.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, propval => obj[propval]
    CallElem = 193,  // JSOP_CALLELEM

    /// '__proto__: v' inside an object initializer.
    ///
    /// Pops the top two values on the stack as 'newProto' and 'obj', sets
    /// prototype of 'obj' as 'newProto', pushes 'true' onto the stack if
    /// succeeded, 'false' if not.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: obj, newProto => succeeded
    MutateProto = 194,  // JSOP_MUTATEPROTO

    /// Pops an environment, gets the value of a bound name on it. If the name
    /// is not bound to the environment, throw a ReferenceError. Used in
    /// conjunction with BINDNAME.
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands: uint32_t nameIndex
    ///   Stack: env => v
    GetBoundName = 195,  // JSOP_GETBOUNDNAME

    /// Pops the top stack value as 'val' and pushes 'typeof val'. Note that
    /// this opcode isn't used when, in the original source code, 'val' is a
    /// name -- see 'JSOP_TYPEOF' for that.
    /// (This is because 'typeof undefinedName === "undefined"'.)
    ///
    ///   Category: Operators
    ///   Type: Special Operators
    ///   Operands:
    ///   Stack: val => (typeof val)
    TypeofExpr = 196,  // JSOP_TYPEOFEXPR

    /// Replaces the current block on the env chain with a fresh block that
    /// copies all the bindings in the block. This operation implements the
    /// behavior of inducing a fresh lexical environment for every iteration of
    /// a for(let ...; ...; ...) loop, if any declarations induced by such a
    /// loop are captured within the loop.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Block-local Scope
    ///   Operands:
    ///   Stack: =>
    FreshenLexicalEnv = 197,  // JSOP_FRESHENLEXICALENV

    /// Recreates the current block on the env chain with a fresh block with
    /// uninitialized bindings. This operation implements the behavior of
    /// inducing a fresh lexical environment for every iteration of a for-in/of
    /// loop whose loop-head has a (captured) lexical declaration.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Block-local Scope
    ///   Operands:
    ///   Stack: =>
    RecreateLexicalEnv = 198,  // JSOP_RECREATELEXICALENV

    /// Pushes lexical environment onto the env chain.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Block-local Scope
    ///   Operands: uint32_t scopeIndex
    ///   Stack: =>
    PushLexicalEnv = 199,  // JSOP_PUSHLEXICALENV

    /// Pops lexical environment from the env chain.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Block-local Scope
    ///   Operands:
    ///   Stack: =>
    PopLexicalEnv = 200,  // JSOP_POPLEXICALENV

    /// The opcode to assist the debugger.
    ///
    ///   Category: Statements
    ///   Type: Debugger
    ///   Operands:
    ///   Stack: =>
    DebugLeaveLexicalEnv = 201,  // JSOP_DEBUGLEAVELEXICALENV

    /// Pops the generator from the top of the stack, suspends it and stops
    /// interpretation.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands: uint24_t resumeIndex
    ///   Stack: generator => generator
    InitialYield = 202,  // JSOP_INITIALYIELD

    /// Pops the generator and the return value 'rval1', stops interpretation
    /// and returns 'rval1'. Pushes sent value from 'send()' onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands: uint24_t resumeIndex
    ///   Stack: rval1, gen => rval2
    Yield = 203,  // JSOP_YIELD

    /// Pops the generator and suspends and closes it. Yields the value in the
    /// frame's return value slot.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands:
    ///   Stack: gen =>
    FinalYieldRval = 204,  // JSOP_FINALYIELDRVAL

    /// Pops the generator and argument from the stack, pushes a new generator
    /// frame and resumes execution of it. Pushes the return value after the
    /// generator yields.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands: resume kind (AbstractGeneratorObject::ResumeKind)
    ///   Stack: gen, val => rval
    Resume = 205,  // JSOP_RESUME

    /// Load the callee stored in a CallObject on the environment chain. The
    /// numHops operand is the number of environment objects to skip on the
    /// environment chain.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Arguments
    ///   Operands: uint8_t numHops
    ///   Stack: => callee
    EnvCallee = 206,  // JSOP_ENVCALLEE

    /// No-op bytecode only emitted in some self-hosted functions. Not handled
    /// by the JITs or Baseline Interpreter so the script always runs in the C++
    /// interpreter.
    ///
    ///   Category: Other
    ///   Operands:
    ///   Stack: =>
    ForceInterpreter = 207,  // JSOP_FORCEINTERPRETER

    /// Bytecode emitted after 'yield' expressions. This is useful for the
    /// Debugger and AbstractGeneratorObject::isAfterYieldOrAwait. It's treated
    /// as jump target op so that the Baseline Interpreter can efficiently
    /// restore the frame's interpreterICEntry when resuming a generator.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands: uint32_t icIndex
    ///   Stack: =>
    AfterYield = 208,  // JSOP_AFTERYIELD

    /// Pops the generator and the return value 'promise', stops interpretation
    /// and returns 'promise'. Pushes resolved value onto the stack.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands: uint24_t resumeIndex
    ///   Stack: promise, gen => resolved
    Await = 209,  // JSOP_AWAIT

    /// Pops the iterator and its next method from the top of the stack, and
    /// create async iterator from it and push the async iterator back onto the
    /// stack.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands:
    ///   Stack: iter, next => asynciter
    ToAsyncIter = 210,  // JSOP_TOASYNCITER

    /// Pops the top two values 'id' and 'obj' from the stack, then pushes
    /// obj.hasOwnProperty(id)
    ///
    /// Note that 'obj' is the top value.
    ///
    ///   Category: Other
    ///   Type:
    ///   Operands:
    ///   Stack: id, obj => (obj.hasOwnProperty(id))
    HasOwn = 211,  // JSOP_HASOWN

    /// Initializes generator frame, creates a generator and pushes it on the
    /// stack.
    ///
    ///   Category: Statements
    ///   Type: Generator
    ///   Operands:
    ///   Stack: => generator
    Generator = 212,  // JSOP_GENERATOR

    /// Pushes the nearest 'var' environment.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Free Variables
    ///   Operands:
    ///   Stack: => env
    BindVar = 213,  // JSOP_BINDVAR

    /// Pushes the global environment onto the stack if the script doesn't have
    /// a non-syntactic global scope. Otherwise will act like JSOP_BINDNAME.
    ///
    /// 'nameIndex' is only used when acting like JSOP_BINDNAME.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Free Variables
    ///   Operands: uint32_t nameIndex
    ///   Stack: => global
    BindGname = 214,  // JSOP_BINDGNAME

    /// Pushes 8-bit int immediate integer operand onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: int8_t val
    ///   Stack: => val
    Int8 = 215,  // JSOP_INT8

    /// Pushes 32-bit int immediate integer operand onto the stack.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: int32_t val
    ///   Stack: => val
    Int32 = 216,  // JSOP_INT32

    /// Pops the top of stack value, pushes the 'length' property of it onto the
    /// stack.
    ///
    ///   Category: Literals
    ///   Type: Array
    ///   Operands: uint32_t nameIndex
    ///   Stack: obj => obj['length']
    Length = 217,  // JSOP_LENGTH

    /// Pushes a JS_ELEMENTS_HOLE value onto the stack, representing an omitted
    /// property in an array literal (e.g. property 0 in the array '[, 1]').
    ///
    /// This opcode is used with the JSOP_NEWARRAY opcode.
    ///
    ///   Category: Literals
    ///   Type: Array
    ///   Operands:
    ///   Stack: => hole
    Hole = 218,  // JSOP_HOLE

    /// Checks that the top value on the stack is callable, and throws a
    /// TypeError if not. The operand 'kind' is used only to generate an
    /// appropriate error message.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint8_t kind
    ///   Stack: obj => obj
    CheckIsCallable = 219,  // JSOP_CHECKISCALLABLE

    /// No-op used by the exception unwinder to determine the correct
    /// environment to unwind to when performing IteratorClose due to
    /// destructuring.
    ///
    ///   Category: Other
    ///   Operands:
    ///   Stack: =>
    TryDestructuring = 220,  // JSOP_TRY_DESTRUCTURING

    /// Pushes the current global's builtin prototype for a given proto key.
    ///
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: uint8_t kind
    ///   Stack: => %BuiltinPrototype%
    BuiltinProto = 221,  // JSOP_BUILTINPROTO

    /// NOP opcode to hint to IonBuilder that the value on top of the stack is
    /// the (likely string) key in a for-in loop.
    ///
    ///   Category: Other
    ///   Operands:
    ///   Stack: val => val
    IterNext = 222,  // JSOP_ITERNEXT

    /// Pops the top of stack value as 'value', checks if the await for 'value'
    /// can be skipped. If the await operation can be skipped and the resolution
    /// value for 'value' can be acquired, pushes the resolution value and
    /// 'true' onto the stack. Otherwise, pushes 'value' and 'false' on the
    /// stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands:
    ///   Stack: value => value_or_resolved, canskip
    TrySkipAwait = 223,  // JSOP_TRYSKIPAWAIT

    /// Creates rest parameter array for current function call, and pushes it
    /// onto the stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Arguments
    ///   Operands:
    ///   Stack: => rest
    Rest = 224,  // JSOP_REST

    /// Replace the top-of-stack value propertyNameValue with
    /// ToPropertyKey(propertyNameValue).
    ///
    ///   Category: Literals
    ///   Type: Object
    ///   Operands:
    ///   Stack: propertyNameValue => propertyKey
    Toid = 225,  // JSOP_TOID

    /// Pushes the implicit 'this' value for calls to the associated name onto
    /// the stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: This
    ///   Operands: uint32_t nameIndex
    ///   Stack: => this
    ImplicitThis = 226,  // JSOP_IMPLICITTHIS

    /// This opcode is the target of the entry jump for some loop. The uint8
    /// argument is a bitfield. The lower 7 bits of the argument indicate the
    /// loop depth. This value starts at 1 and is just a hint: deeply nested
    /// loops all have the same value. The upper bit is set if Ion should be
    /// able to OSR at this point, which is true unless there is non-loop state
    /// on the stack. See JSOP_JUMPTARGET for the icIndex argument.
    ///
    ///   Category: Statements
    ///   Type: Jumps
    ///   Operands: uint32_t icIndex, uint8_t BITFIELD
    ///   Stack: =>
    LoopEntry = 227,  // JSOP_LOOPENTRY

    /// Converts the value on the top of the stack to a String.
    ///
    ///   Category: Other
    ///   Operands:
    ///   Stack: val => ToString(val)
    ToString = 228,  // JSOP_TOSTRING

    /// No-op used by the decompiler to produce nicer error messages about
    /// destructuring code.
    ///
    ///   Category: Other
    ///   Operands:
    ///   Stack: =>
    NopDestructuring = 229,  // JSOP_NOP_DESTRUCTURING

    /// This opcode is a no-op and it indicates the location of a jump
    /// instruction target. Some other opcodes act as jump targets as well, see
    /// BytecodeIsJumpTarget. The IC index is used by the Baseline interpreter.
    ///
    ///   Category: Other
    ///   Operands: uint32_t icIndex
    ///   Stack: =>
    JumpTarget = 230,  // JSOP_JUMPTARGET

    /// Like JSOP_CALL, but tells the function that the return value is ignored.
    /// stack.
    ///
    ///   Category: Statements
    ///   Type: Function
    ///   Operands: uint16_t argc
    ///   Stack: callee, this, args[0], ..., args[argc-1] => rval
    ///   nuses: (argc+2)
    CallIgnoresRv = 231,  // JSOP_CALL_IGNORES_RV

    /// Push "import.meta"
    ///
    ///   Category: Variables and Scopes
    ///   Type: Modules
    ///   Operands:
    ///   Stack: => import.meta
    ImportMeta = 232,  // JSOP_IMPORTMETA

    /// Dynamic import of the module specified by the string value on the top of
    /// the stack.
    ///
    ///   Category: Variables and Scopes
    ///   Type: Modules
    ///   Operands:
    ///   Stack: arg => rval
    DynamicImport = 233,  // JSOP_DYNAMIC_IMPORT

    /// Pops the numeric value 'val' from the stack, then pushes 'val + 1'.
    ///
    ///   Category: Operators
    ///   Type: Arithmetic Operators
    ///   Operands:
    ///   Stack: val => (val + 1)
    Inc = 234,  // JSOP_INC

    /// Pops the numeric value 'val' from the stack, then pushes 'val - 1'.
    ///
    ///   Category: Operators
    ///   Type: Arithmetic Operators
    ///   Operands:
    ///   Stack: val => (val - 1)
    Dec = 235,  // JSOP_DEC

    /// Pop 'val' from the stack, then push the result of 'ToNumeric(val)'.
    ///   Category: Operators
    ///   Type: Arithmetic Operators
    ///   Operands:
    ///   Stack: val => ToNumeric(val)
    ToNumeric = 236,  // JSOP_TONUMERIC

    /// Pushes a BigInt constant onto the stack.
    ///   Category: Literals
    ///   Type: Constants
    ///   Operands: uint32_t constIndex
    ///   Stack: => val
    BigInt = 237,  // JSOP_BIGINT

    /// Pushes a boolean indicating if instrumentation is active.
    ///   Category: Other
    ///   Operands:
    ///   Stack: => val
    InstrumentationActive = 238,  // JSOP_INSTRUMENTATION_ACTIVE

    /// Pushes the instrumentation callback for the current realm.
    ///   Category: Other
    ///   Operands:
    ///   Stack: => val
    InstrumentationCallback = 239,  // JSOP_INSTRUMENTATION_CALLBACK

    /// Pushes the current script's instrumentation ID.
    ///   Category: Other
    ///   Operands:
    ///   Stack: => val
    InstrumentationScriptId = 240,  // JSOP_INSTRUMENTATION_SCRIPT_ID
}

const LIMIT: usize = 241;

impl Opcode {
    /// Return the numeric bytecode value for this opcode, as understood by the
    /// SpiderMonkey interpreter and the rest of the VM.
    pub fn to_byte(&self) -> u8 {
        *self as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) ->  Result<Opcode, ()> {
        if (value as usize) < LIMIT {
            Ok(TABLE[value as usize])
        } else {
            Err(())
        }
    }
}

pub static TABLE: [Opcode; LIMIT] = [
    Opcode::Nop,
    Opcode::Undefined,
    Opcode::GetRval,
    Opcode::EnterWith,
    Opcode::LeaveWith,
    Opcode::Return,
    Opcode::Goto,
    Opcode::IfEq,
    Opcode::IfNe,
    Opcode::Arguments,
    Opcode::Swap,
    Opcode::PopN,
    Opcode::Dup,
    Opcode::Dup2,
    Opcode::CheckIsObj,
    Opcode::BitOr,
    Opcode::BitXor,
    Opcode::BitAnd,
    Opcode::Eq,
    Opcode::Ne,
    Opcode::Lt,
    Opcode::Le,
    Opcode::Gt,
    Opcode::Ge,
    Opcode::Lsh,
    Opcode::Rsh,
    Opcode::Ursh,
    Opcode::Add,
    Opcode::Sub,
    Opcode::Mul,
    Opcode::Div,
    Opcode::Mod,
    Opcode::Not,
    Opcode::BitNot,
    Opcode::Neg,
    Opcode::Pos,
    Opcode::DelName,
    Opcode::DelProp,
    Opcode::DelElem,
    Opcode::Typeof,
    Opcode::Void,
    Opcode::SpreadCall,
    Opcode::SpreadNew,
    Opcode::SpreadEval,
    Opcode::DupAt,
    Opcode::Symbol,
    Opcode::StrictDelProp,
    Opcode::StrictDelElem,
    Opcode::StrictSetProp,
    Opcode::StrictSetName,
    Opcode::StrictSpreadEval,
    Opcode::CheckClassHeritage,
    Opcode::FunWithProto,
    Opcode::GetProp,
    Opcode::SetProp,
    Opcode::GetElem,
    Opcode::SetElem,
    Opcode::StrictSetElem,
    Opcode::Call,
    Opcode::GetName,
    Opcode::Double,
    Opcode::String,
    Opcode::Zero,
    Opcode::One,
    Opcode::Null,
    Opcode::IsConstructing,
    Opcode::False,
    Opcode::True,
    Opcode::Or,
    Opcode::And,
    Opcode::TableSwitch,
    Opcode::Unused71,
    Opcode::StrictEq,
    Opcode::StrictNe,
    Opcode::ThrowMsg,
    Opcode::Iter,
    Opcode::MoreIter,
    Opcode::IsNoIter,
    Opcode::EndIter,
    Opcode::FunApply,
    Opcode::Object,
    Opcode::Pop,
    Opcode::New,
    Opcode::ObjWithProto,
    Opcode::GetArg,
    Opcode::SetArg,
    Opcode::GetLocal,
    Opcode::SetLocal,
    Opcode::Uint16,
    Opcode::NewInit,
    Opcode::NewArray,
    Opcode::NewObject,
    Opcode::Inithomeobject,
    Opcode::InitProp,
    Opcode::InitElem,
    Opcode::InitElemInc,
    Opcode::InitElemArray,
    Opcode::InitPropGetter,
    Opcode::InitPropSetter,
    Opcode::InitElemGetter,
    Opcode::InitElemSetter,
    Opcode::CallSiteObj,
    Opcode::NewArrayCopyOnWrite,
    Opcode::SuperBase,
    Opcode::GetPropSuper,
    Opcode::StrictSetPropSuper,
    Opcode::Label,
    Opcode::SetPropSuper,
    Opcode::FunCall,
    Opcode::LoopHead,
    Opcode::BindName,
    Opcode::SetName,
    Opcode::Throw,
    Opcode::In,
    Opcode::Instanceof,
    Opcode::Debugger,
    Opcode::Gosub,
    Opcode::Retsub,
    Opcode::Exception,
    Opcode::Lineno,
    Opcode::CondSwitch,
    Opcode::Case,
    Opcode::Default,
    Opcode::Eval,
    Opcode::StrictEval,
    Opcode::GetElemSuper,
    Opcode::ResumeIndex,
    Opcode::DefFun,
    Opcode::DefConst,
    Opcode::DefVar,
    Opcode::Lambda,
    Opcode::LambdaArrow,
    Opcode::Callee,
    Opcode::Pick,
    Opcode::Try,
    Opcode::Finally,
    Opcode::GetAliasedVar,
    Opcode::SetAliasedVar,
    Opcode::CheckLexical,
    Opcode::InitLexical,
    Opcode::CheckAliasedLexical,
    Opcode::InitAliasedLexical,
    Opcode::Uninitialized,
    Opcode::GetIntrinsic,
    Opcode::SetIntrinsic,
    Opcode::CallIter,
    Opcode::InitLockedProp,
    Opcode::InitHiddenProp,
    Opcode::NewTarget,
    Opcode::Unused149,
    Opcode::Pow,
    Opcode::AsyncAwait,
    Opcode::SetRval,
    Opcode::RetRval,
    Opcode::GetGname,
    Opcode::SetGname,
    Opcode::StrictSetGname,
    Opcode::GImplicitThis,
    Opcode::SetElemSuper,
    Opcode::StrictSetElemSuper,
    Opcode::RegExp,
    Opcode::InitGLexical,
    Opcode::DefLet,
    Opcode::CheckObjCoercible,
    Opcode::SuperFun,
    Opcode::SuperCall,
    Opcode::SpreadSuperCall,
    Opcode::ClassConstructor,
    Opcode::DerivedConstructor,
    Opcode::ThrowSetConst,
    Opcode::ThrowSetAliasedConst,
    Opcode::InitHiddenPropGetter,
    Opcode::InitHiddenPropSetter,
    Opcode::InitHiddenElemGetter,
    Opcode::InitHiddenElemSetter,
    Opcode::InitHiddenElem,
    Opcode::GetImport,
    Opcode::DebugCheckSelfHosted,
    Opcode::OptimizeSpreadCall,
    Opcode::ThrowSetCallee,
    Opcode::PushVarEnv,
    Opcode::PopVarEnv,
    Opcode::SetFunName,
    Opcode::Unpick,
    Opcode::CallProp,
    Opcode::FunctionThis,
    Opcode::GlobalThis,
    Opcode::IsGenClosing,
    Opcode::Uint24,
    Opcode::CheckThis,
    Opcode::CheckReturn,
    Opcode::CheckThisReinit,
    Opcode::AsyncResolve,
    Opcode::CallElem,
    Opcode::MutateProto,
    Opcode::GetBoundName,
    Opcode::TypeofExpr,
    Opcode::FreshenLexicalEnv,
    Opcode::RecreateLexicalEnv,
    Opcode::PushLexicalEnv,
    Opcode::PopLexicalEnv,
    Opcode::DebugLeaveLexicalEnv,
    Opcode::InitialYield,
    Opcode::Yield,
    Opcode::FinalYieldRval,
    Opcode::Resume,
    Opcode::EnvCallee,
    Opcode::ForceInterpreter,
    Opcode::AfterYield,
    Opcode::Await,
    Opcode::ToAsyncIter,
    Opcode::HasOwn,
    Opcode::Generator,
    Opcode::BindVar,
    Opcode::BindGname,
    Opcode::Int8,
    Opcode::Int32,
    Opcode::Length,
    Opcode::Hole,
    Opcode::CheckIsCallable,
    Opcode::TryDestructuring,
    Opcode::BuiltinProto,
    Opcode::IterNext,
    Opcode::TrySkipAwait,
    Opcode::Rest,
    Opcode::Toid,
    Opcode::ImplicitThis,
    Opcode::LoopEntry,
    Opcode::ToString,
    Opcode::NopDestructuring,
    Opcode::JumpTarget,
    Opcode::CallIgnoresRv,
    Opcode::ImportMeta,
    Opcode::DynamicImport,
    Opcode::Inc,
    Opcode::Dec,
    Opcode::ToNumeric,
    Opcode::BigInt,
    Opcode::InstrumentationActive,
    Opcode::InstrumentationCallback,
    Opcode::InstrumentationScriptId,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_matches() {
        for (i, t) in TABLE.iter().enumerate() {
            assert!(i == t.to_byte() as usize);
        }
    }
}
