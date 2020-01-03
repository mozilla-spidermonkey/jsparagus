use std::convert::TryFrom;

macro_rules! using_opcode_database {
    ( $macro:ident ! ( ) ) => {
        $macro! {
            [
                (Undefined, js_undefined_str, "", 1, 0, 1, JOF_BYTE),
                (Null, js_null_str, js_null_str, 1, 0, 1, JOF_BYTE),
                (False, js_false_str, js_false_str, 1, 0, 1, JOF_BYTE),
                (True, js_true_str, js_true_str, 1, 0, 1, JOF_BYTE),
                (Int32, "int32", NULL, 5, 0, 1, JOF_INT32),
                (Zero, "zero", "0", 1, 0, 1, JOF_BYTE),
                (One, "one", "1", 1, 0, 1, JOF_BYTE),
                (Int8, "int8", NULL, 2, 0, 1, JOF_INT8),
                (Uint16, "uint16", NULL, 3, 0, 1, JOF_UINT16),
                (Uint24, "uint24", NULL, 4, 0, 1, JOF_UINT24),
                (Double, "double", NULL, 9, 0, 1, JOF_DOUBLE),
                (BigInt, "bigint", NULL, 5, 0, 1, JOF_BIGINT),
                (String, "string", NULL, 5, 0, 1, JOF_ATOM),
                (Symbol, "symbol", NULL, 2, 0, 1, JOF_UINT8),
                (Void, js_void_str, NULL, 1, 1, 1, JOF_BYTE),
                (Typeof, js_typeof_str, NULL, 1, 1, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (TypeofExpr, "typeofexpr", NULL, 1, 1, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (Pos, "pos", "+ ", 1, 1, 1, JOF_BYTE),
                (Neg, "neg", "- ", 1, 1, 1, JOF_BYTE|JOF_IC),
                (BitNot, "bitnot", "~", 1, 1, 1, JOF_BYTE|JOF_IC),
                (Not, "not", "!", 1, 1, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (BitOr, "bitor", "|",  1, 2, 1, JOF_BYTE|JOF_IC),
                (BitXor, "bitxor", "^", 1, 2, 1, JOF_BYTE|JOF_IC),
                (BitAnd, "bitand", "&", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Eq, "eq", "==", 1, 2, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (Ne, "ne", "!=", 1, 2, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (StrictEq, "stricteq", "===", 1, 2, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (StrictNe, "strictne", "!==", 1, 2, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (Lt, "lt", "<",  1, 2, 1, JOF_BYTE|JOF_IC),
                (Gt, "gt", ">",  1, 2, 1, JOF_BYTE|JOF_IC),
                (Le, "le", "<=", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Ge, "ge", ">=", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Instanceof, js_instanceof_str, js_instanceof_str, 1, 2, 1, JOF_BYTE|JOF_IC),
                (In, js_in_str, js_in_str, 1, 2, 1, JOF_BYTE|JOF_IC),
                (Lsh, "lsh", "<<", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Rsh, "rsh", ">>", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Ursh, "ursh", ">>>", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Add, "add", "+", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Sub, "sub", "-", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Inc, "inc", NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (Dec, "dec", NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (Mul, "mul", "*", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Div, "div", "/", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Mod, "mod", "%", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Pow, "pow", "**", 1, 2, 1, JOF_BYTE|JOF_IC),
                (ToId, "toid", NULL, 1, 1, 1, JOF_BYTE),
                (ToNumeric, "tonumeric", NULL, 1, 1, 1, JOF_BYTE),
                (ToString, "tostring", NULL, 1, 1, 1, JOF_BYTE),
                (GlobalThis, "globalthis", NULL, 1, 0, 1, JOF_BYTE),
                (NewTarget, "newtarget", NULL, 1, 0, 1, JOF_BYTE),
                (DynamicImport, "dynamic-import", NULL, 1, 1, 1, JOF_BYTE),
                (ImportMeta, "importmeta", NULL, 1, 0, 1, JOF_BYTE),
                (NewInit, "newinit", NULL, 5, 0, 1, JOF_UINT32|JOF_IC),
                (NewObject, "newobject", NULL, 5, 0, 1, JOF_OBJECT|JOF_IC),
                (NewObjectWithGroup, "newobjectwithgroup", NULL, 5, 0, 1, JOF_OBJECT|JOF_IC),
                (Object, "object", NULL, 5, 0, 1, JOF_OBJECT),
                (ObjWithProto, "objwithproto", NULL, 1, 1, 1, JOF_BYTE),
                (InitProp, "initprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (InitHiddenProp, "inithiddenprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (InitLockedProp, "initlockedprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (InitElem, "initelem", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (InitHiddenElem, "inithiddenelem", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (InitPropGetter, "initprop_getter", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING),
                (InitHiddenPropGetter, "inithiddenprop_getter", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING),
                (InitElemGetter, "initelem_getter", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING),
                (InitHiddenElemGetter, "inithiddenelem_getter", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING),
                (InitPropSetter, "initprop_setter", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING),
                (InitHiddenPropSetter, "inithiddenprop_setter", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING),
                (InitElemSetter, "initelem_setter", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING),
                (InitHiddenElemSetter, "inithiddenelem_setter", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING),
                (GetProp, "getprop", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_TYPESET|JOF_IC),
                (CallProp, "callprop", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_TYPESET|JOF_IC),
                (GetElem, "getelem", NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_TYPESET|JOF_IC),
                (CallElem, "callelem", NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_TYPESET|JOF_IC),
                (Length, "length", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_TYPESET|JOF_IC),
                (SetProp, "setprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY|JOF_IC),
                (StrictSetProp, "strict-setprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT|JOF_IC),
                (SetElem, "setelem", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY|JOF_IC),
                (StrictSetElem, "strict-setelem", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT|JOF_IC),
                (DelProp, "delprop", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_CHECKSLOPPY),
                (StrictDelProp, "strict-delprop", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_CHECKSTRICT),
                (DelElem, "delelem", NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_CHECKSLOPPY),
                (StrictDelElem, "strict-delelem", NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_CHECKSTRICT),
                (HasOwn, "hasown", NULL, 1, 2, 1, JOF_BYTE|JOF_IC),
                (SuperBase, "superbase", NULL, 1, 1, 1, JOF_BYTE),
                (GetPropSuper, "getprop-super", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_TYPESET|JOF_IC),
                (GetElemSuper, "getelem-super", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_TYPESET|JOF_IC),
                (SetPropSuper, "setprop-super", NULL, 5, 3, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY),
                (StrictSetPropSuper, "strictsetprop-super", NULL, 5, 3, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT),
                (SetElemSuper, "setelem-super", NULL, 1, 4, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY),
                (StrictSetElemSuper, "strict-setelem-super", NULL, 1, 4, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT),
                (Iter, "iter", NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (MoreIter, "moreiter", NULL, 1, 1, 2, JOF_BYTE),
                (IsNoIter, "isnoiter", NULL, 1, 1, 2, JOF_BYTE),
                (IterNext, "iternext", NULL, 1, 1, 1, JOF_BYTE),
                (EndIter, "enditer", NULL, 1, 2, 0, JOF_BYTE),
                (CheckIsObj, "checkisobj", NULL, 2, 1, 1, JOF_UINT8),
                (CheckIsCallable, "checkiscallable", NULL, 2, 1, 1, JOF_UINT8),
                (CheckObjCoercible, "checkobjcoercible", NULL, 1, 1, 1, JOF_BYTE),
                (ToAsyncIter, "toasynciter", NULL, 1, 2, 1, JOF_BYTE),
                (MutateProto, "mutateproto", NULL, 1, 2, 1, JOF_BYTE),
                (NewArray, "newarray", NULL, 5, 0, 1, JOF_UINT32|JOF_IC),
                (InitElemArray, "initelem_array", NULL, 5, 2, 1, JOF_UINT32|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (InitElemInc, "initelem_inc", NULL, 1, 3, 2, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_IC),
                (Hole, "hole", NULL, 1, 0, 1, JOF_BYTE),
                (NewArrayCopyOnWrite, "newarray_copyonwrite", NULL, 5, 0, 1, JOF_OBJECT),
                (RegExp, "regexp", NULL, 5, 0, 1, JOF_REGEXP),
                (Lambda, "lambda", NULL, 5, 0, 1, JOF_OBJECT),
                (LambdaArrow, "lambda_arrow", NULL, 5, 1, 1, JOF_OBJECT),
                (SetFunName, "setfunname", NULL, 2, 2, 1, JOF_UINT8),
                (InitHomeObject, "inithomeobject", NULL, 1, 2, 1, JOF_BYTE),
                (CheckClassHeritage, "checkclassheritage", NULL, 1, 1, 1, JOF_BYTE),
                (FunWithProto, "funwithproto", NULL, 5, 1, 1, JOF_OBJECT),
                (ClassConstructor, "classconstructor", NULL, 13, 0, 1, JOF_CLASS_CTOR),
                (DerivedConstructor, "derivedconstructor", NULL, 13, 1, 1, JOF_CLASS_CTOR),
                (BuiltinProto, "builtinproto", NULL, 2, 0, 1, JOF_UINT8),
                (Call, "call", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (CallIter, "calliter", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (FunApply, "funapply", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (FunCall, "funcall", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (CallIgnoresRv, "call-ignores-rv", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (SpreadCall, "spreadcall", NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (OptimizeSpreadCall, "optimize-spreadcall", NULL, 1, 1, 2, JOF_BYTE),
                (Eval, "eval", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_CHECKSLOPPY|JOF_IC),
                (SpreadEval, "spreadeval", NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_CHECKSLOPPY|JOF_IC),
                (StrictEval, "strict-eval", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_CHECKSTRICT|JOF_IC),
                (StrictSpreadEval, "strict-spreadeval", NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_CHECKSTRICT|JOF_IC),
                (ImplicitThis, "implicitthis", "", 5, 0, 1, JOF_ATOM),
                (GImplicitThis, "gimplicitthis", "", 5, 0, 1, JOF_ATOM),
                (CallSiteObj, "callsiteobj", NULL, 5, 0, 1, JOF_OBJECT),
                (IsConstructing, "is-constructing", NULL, 1, 0, 1, JOF_BYTE),
                (New, "new", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC|JOF_IC),
                (SpreadNew, "spreadnew", NULL, 1, 4, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (SuperFun, "superfun", NULL, 1, 1, 1, JOF_BYTE),
                (SuperCall, "supercall", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (SpreadSuperCall, "spreadsupercall", NULL, 1, 4, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (CheckThisReinit, "checkthisreinit", NULL, 1, 1, 1, JOF_BYTE),
                (Generator, "generator", NULL, 1, 0, 1, JOF_BYTE),
                (InitialYield, "initialyield", NULL, 4, 1, 1, JOF_RESUMEINDEX),
                (AfterYield, "afteryield", NULL, 5, 0, 0, JOF_ICINDEX),
                (FinalYieldRval, "finalyieldrval", NULL, 1, 1, 0, JOF_BYTE),
                (Yield, "yield", NULL, 4, 2, 1, JOF_RESUMEINDEX),
                (IsGenClosing, "isgenclosing", NULL, 1, 1, 2, JOF_BYTE),
                (AsyncAwait, "async-await", NULL, 1, 2, 1, JOF_BYTE),
                (AsyncResolve, "async-resolve", NULL, 2, 2, 1, JOF_UINT8),
                (Await, "await", NULL, 4, 2, 1, JOF_RESUMEINDEX),
                (TrySkipAwait, "tryskipawait", NULL, 1, 1, 2, JOF_BYTE),
                (Resume, "resume", NULL, 2, 2, 1, JOF_UINT8|JOF_INVOKE),
                (JumpTarget, "jumptarget", NULL, 5, 0, 0, JOF_ICINDEX),
                (LoopHead, "loophead", NULL, 6, 0, 0, JOF_LOOPHEAD),
                (Goto, "goto", NULL, 5, 0, 0, JOF_JUMP),
                (IfEq, "ifeq", NULL, 5, 1, 0, JOF_JUMP|JOF_DETECTING|JOF_IC),
                (IfNe, "ifne", NULL, 5, 1, 0, JOF_JUMP|JOF_IC),
                (And, "and", NULL, 5, 1, 1, JOF_JUMP|JOF_DETECTING|JOF_IC),
                (Or, "or", NULL, 5, 1, 1, JOF_JUMP|JOF_DETECTING|JOF_IC),
                (Coalesce, "coalesce", NULL, 5, 1, 1, JOF_JUMP|JOF_DETECTING),
                (Case, "case", NULL, 5, 2, 1, JOF_JUMP),
                (Default, "default", NULL, 5, 1, 0, JOF_JUMP),
                (TableSwitch, "tableswitch", NULL, 16, 1, 0, JOF_TABLESWITCH|JOF_DETECTING),
                (Return, "return", NULL, 1, 1, 0, JOF_BYTE),
                (GetRval, "getrval", NULL, 1, 0, 1, JOF_BYTE),
                (SetRval, "setrval", NULL, 1, 1, 0, JOF_BYTE),
                (RetRval, "retrval", NULL, 1, 0, 0, JOF_BYTE),
                (CheckReturn, "checkreturn", NULL, 1, 1, 0, JOF_BYTE),
                (Throw, js_throw_str, NULL, 1, 1, 0, JOF_BYTE),
                (ThrowMsg, "throwmsg", NULL, 3, 0, 0, JOF_UINT16),
                (ThrowSetAliasedConst, "throwsetaliasedconst", NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME|JOF_DETECTING),
                (ThrowSetCallee, "throwsetcallee", NULL, 1, 1, 1, JOF_BYTE),
                (ThrowSetConst, "throwsetconst", NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME|JOF_DETECTING),
                (Try, "try", NULL, 1, 0, 0, JOF_BYTE),
                (TryDestructuring, "try-destructuring", NULL, 1, 0, 0, JOF_BYTE),
                (Exception, "exception", NULL, 1, 0, 1, JOF_BYTE),
                (ResumeIndex, "resume-index", NULL, 4, 0, 1, JOF_RESUMEINDEX),
                (Gosub, "gosub", NULL, 5, 2, 0, JOF_JUMP),
                (Finally, "finally", NULL, 1, 0, 2, JOF_BYTE),
                (Retsub, "retsub", NULL, 1, 2, 0, JOF_BYTE),
                (Uninitialized, "uninitialized", NULL, 1, 0, 1, JOF_BYTE),
                (InitLexical, "initlexical", NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME|JOF_DETECTING),
                (InitGLexical, "initglexical", NULL, 5, 1, 1, JOF_ATOM|JOF_NAME|JOF_PROPINIT|JOF_GNAME|JOF_IC),
                (InitAliasedLexical, "initaliasedlexical", NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME|JOF_PROPINIT|JOF_DETECTING),
                (CheckLexical, "checklexical", NULL, 4, 0, 0, JOF_LOCAL|JOF_NAME),
                (CheckAliasedLexical, "checkaliasedlexical", NULL, 5, 0, 0, JOF_ENVCOORD|JOF_NAME),
                (CheckThis, "checkthis", NULL, 1, 1, 1, JOF_BYTE),
                (BindGname, "bindgname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_GNAME|JOF_IC),
                (BindName, "bindname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_IC),
                (GetName, "getname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_IC),
                (GetGname, "getgname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_GNAME|JOF_IC),
                (GetArg, "getarg", NULL, 3, 0, 1, JOF_QARG|JOF_NAME),
                (GetLocal, "getlocal", NULL, 4, 0, 1, JOF_LOCAL|JOF_NAME),
                (GetAliasedVar, "getaliasedvar", NULL, 5, 0, 1, JOF_ENVCOORD|JOF_NAME|JOF_TYPESET|JOF_IC),
                (GetImport, "getimport", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_IC),
                (GetBoundName, "getboundname", NULL, 5, 1, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_IC),
                (GetIntrinsic, "getintrinsic", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_IC),
                (Callee, "callee", NULL, 1, 0, 1, JOF_BYTE),
                (EnvCallee, "envcallee", NULL, 2, 0, 1, JOF_UINT8),
                (SetName, "setname", NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY|JOF_IC),
                (StrictSetName, "strict-setname", NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT|JOF_IC),
                (SetGname, "setgname", NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_DETECTING|JOF_GNAME|JOF_CHECKSLOPPY|JOF_IC),
                (StrictSetGname, "strict-setgname", NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_DETECTING|JOF_GNAME|JOF_CHECKSTRICT|JOF_IC),
                (SetArg, "setarg", NULL, 3, 1, 1, JOF_QARG|JOF_NAME),
                (SetLocal, "setlocal", NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME|JOF_DETECTING),
                (SetAliasedVar, "setaliasedvar", NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME|JOF_PROPSET|JOF_DETECTING),
                (SetIntrinsic, "setintrinsic", NULL, 5, 1, 1, JOF_ATOM|JOF_NAME|JOF_DETECTING),
                (PushLexicalEnv, "pushlexicalenv", NULL, 5, 0, 0, JOF_SCOPE),
                (PopLexicalEnv, "poplexicalenv", NULL, 1, 0, 0, JOF_BYTE),
                (DebugLeaveLexicalEnv, "debugleavelexicalenv", NULL, 1, 0, 0, JOF_BYTE),
                (RecreateLexicalEnv, "recreatelexicalenv", NULL, 1, 0, 0, JOF_BYTE),
                (FreshenLexicalEnv, "freshenlexicalenv", NULL, 1, 0, 0, JOF_BYTE),
                (PushVarEnv, "pushvarenv", NULL, 5, 0, 0, JOF_SCOPE),
                (PopVarEnv, "popvarenv", NULL, 1, 0, 0, JOF_BYTE),
                (EnterWith, "enterwith", NULL, 5, 1, 0, JOF_SCOPE),
                (LeaveWith, "leavewith", NULL, 1, 0, 0, JOF_BYTE),
                (BindVar, "bindvar", NULL, 1, 0, 1, JOF_BYTE),
                (DefVar, "defvar", NULL, 5, 0, 0, JOF_ATOM),
                (DefFun, "deffun", NULL, 1, 1, 0, JOF_BYTE),
                (DefLet, "deflet", NULL, 5, 0, 0, JOF_ATOM),
                (DefConst, "defconst", NULL, 5, 0, 0, JOF_ATOM),
                (DelName, "delname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_CHECKSLOPPY),
                (Arguments, "arguments", NULL, 1, 0, 1, JOF_BYTE),
                (Rest, "rest", NULL, 1, 0, 1, JOF_BYTE|JOF_TYPESET|JOF_IC),
                (FunctionThis, "functionthis", NULL, 1, 0, 1, JOF_BYTE),
                (Pop, "pop", NULL, 1, 1, 0, JOF_BYTE),
                (PopN, "popn", NULL, 3, -1, 0, JOF_UINT16),
                (Dup, "dup", NULL, 1, 1, 2, JOF_BYTE),
                (Dup2, "dup2", NULL, 1, 2, 4, JOF_BYTE),
                (DupAt, "dupat", NULL, 4, 0, 1, JOF_UINT24),
                (Swap, "swap", NULL, 1, 2, 2, JOF_BYTE),
                (Pick, "pick", NULL, 2, 0, 0, JOF_UINT8),
                (Unpick, "unpick", NULL, 2, 0, 0, JOF_UINT8),
                (Nop, "nop", NULL, 1, 0, 0, JOF_BYTE),
                (Lineno, "lineno", NULL, 5, 0, 0, JOF_UINT32),
                (NopDestructuring, "nop-destructuring", NULL, 1, 0, 0, JOF_BYTE),
                (ForceInterpreter, "forceinterpreter", NULL, 1, 0, 0, JOF_BYTE),
                (DebugCheckSelfHosted, "debug-checkselfhosted", NULL, 1, 1, 1, JOF_BYTE),
                (InstrumentationActive, "instrumentationActive", NULL, 1, 0, 1, JOF_BYTE),
                (InstrumentationCallback, "instrumentationCallback", NULL, 1, 0, 1, JOF_BYTE),
                (InstrumentationScriptId, "instrumentationScriptId", NULL, 1, 0, 1, JOF_BYTE),
                (Debugger, "debugger", NULL, 1, 0, 0, JOF_BYTE),
            ]
        }
    }
}

macro_rules! define_opcode_enum {
    ( [ $( ( $id:ident , $( $etc:tt )* ), )* ] ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        #[repr(u8)]
        pub enum Opcode {
            $( $id, )*
        }
    }
}

using_opcode_database!(define_opcode_enum!());

macro_rules! count_rows {
    ( [ $( $ignored_row:tt , )* ] ) => {
        0 $( + count_rows!(one for $ignored_row) )*
    };

    ( one for $ignored_row:tt ) => {
        1
    }
}

const LIMIT: usize = using_opcode_database!(count_rows!());

/// single bytecode, no immediates
const JOF_BYTE: u32 = 0;

/// unspecified uint8_t argument
const JOF_UINT8: u32 = 1;

/// unspecified uint16_t argument
const JOF_UINT16: u32 = 2;

/// unspecified uint24_t argument
const JOF_UINT24: u32 = 3;

/// unspecified uint32_t argument
const JOF_UINT32: u32 = 4;

/// int8_t literal
const JOF_INT8: u32 = 5;

/// int32_t literal
const JOF_INT32: u32 = 6;

/// int32_t jump offset
const JOF_JUMP: u32 = 7;

/// table switch
const JOF_TABLESWITCH: u32 = 8;

/// embedded ScopeCoordinate immediate
const JOF_ENVCOORD: u32 = 9;

/// uint16_t argument count
const JOF_ARGC: u32 = 10;

/// function argument index
const JOF_QARG: u32 = 11;

/// var or block-local variable
const JOF_LOCAL: u32 = 12;

/// yield, await, or gosub resume index
const JOF_RESUMEINDEX: u32 = 13;

/// uint32_t constant index
const JOF_ATOM: u32 = 14;

/// uint32_t object index
const JOF_OBJECT: u32 = 15;

/// uint32_t regexp index
const JOF_REGEXP: u32 = 16;

/// inline DoubleValue
const JOF_DOUBLE: u32 = 17;

/// uint32_t scope index
const JOF_SCOPE: u32 = 18;

/// uint32_t IC index
const JOF_ICINDEX: u32 = 19;

/// Opcode::Loophead, combines JOF_ICINDEX and JOF_UINT8
const JOF_LOOPHEAD: u32 = 20;

/// uint32_t index for BigInt value
const JOF_BIGINT: u32 = 21;

/// uint32_t atom index, sourceStart, sourceEnd
const JOF_CLASS_CTOR: u32 = 22;

/// mask for above immediate types
const JOF_TYPEMASK: u32 = 0x001f;


/// name operation
const JOF_NAME: u32 = 1 << 5;

/// obj.prop operation
const JOF_PROP: u32 = 2 << 5;

/// obj[index] operation
const JOF_ELEM: u32 = 3 << 5;

// /// mask for above addressing modes
// const JOF_MODEMASK: u32 = 3 << 5;


/// property/element/name set operation
const JOF_PROPSET: u32 = 1 << 7;

/// property/element/name init operation
const JOF_PROPINIT: u32 = 1 << 8;

/// object detection for warning-quelling
const JOF_DETECTING: u32 = 1 << 9;

/// op can only be generated in sloppy mode
const JOF_CHECKSLOPPY: u32 = 1 << 10;

/// op can only be generated in strict mode
const JOF_CHECKSTRICT: u32 = 1 << 11;

/// call, construct, or spreadcall instruction
const JOF_INVOKE: u32 = 1 << 12;

/// predicted global name
const JOF_GNAME: u32 = 1 << 13;

/// has an entry in a script's type sets
const JOF_TYPESET: u32 = 1 << 14;

/// baseline may use an IC for this op
const JOF_IC: u32 = 1 << 15;




impl Opcode {
    /// Return the numeric bytecode value for this opcode, as understood by the
    /// SpiderMonkey interpreter and the rest of the VM.
    pub fn to_byte(&self) -> u8 {
        *self as u8
    }

    /// True if this opcode takes no operands (is a one-byte opcode) and
    /// implements a unary operator that operates on one stack value.
    ///
    /// The operators `typeof` and `delete` have different bytecode patterns
    /// and thus are not considered "simple"; `new` is considered a special
    /// kind of function call, not a unary operator.
    pub fn is_simple_unary_operator(self) -> bool {
        self == Opcode::Pos
            || self == Opcode::Neg
            || self == Opcode::Not
            || self == Opcode::BitNot
            || self == Opcode::Void
    }

    /// True if this opcode takes no operands (is a one-byte opcode) and
    /// implements a binary operator that operates on two stack values.
    ///
    /// The operators `||`, `&&`, and `,` have different bytecode patterns
    /// and thus are not considered "simple".
    pub fn is_simple_binary_operator(self) -> bool {
        self == Opcode::BitOr
            || self == Opcode::BitXor
            || self == Opcode::BitAnd
            || self == Opcode::Eq
            || self == Opcode::Ne
            || self == Opcode::StrictEq
            || self == Opcode::StrictNe
            || self == Opcode::Lt
            || self == Opcode::Gt
            || self == Opcode::Le
            || self == Opcode::Ge
            || self == Opcode::Instanceof
            || self == Opcode::In
            || self == Opcode::Lsh
            || self == Opcode::Rsh
            || self == Opcode::Ursh
            || self == Opcode::Add
            || self == Opcode::Sub
            || self == Opcode::Mul
            || self == Opcode::Div
            || self == Opcode::Mod
            || self == Opcode::Pow
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) -> Result<Opcode, ()> {
        if (value as usize) < LIMIT {
            Ok(TABLE[value as usize])
        } else {
            Err(())
        }
    }
}

macro_rules! define_table {
    ( [ $( ( $name:ident , $str:expr , $str2:expr , $length:expr , $nuses:expr , $ndefs:expr , $flags:expr ) , )* ] ) => {
        pub static TABLE: [Opcode; LIMIT] = [
            $( Opcode::$name , )*
        ];
    }
}

using_opcode_database!(define_table!());

impl Opcode {
    /// Length of this instruction, in bytes.
    pub fn instruction_length(self) -> usize {
        macro_rules! select_length {
            ( [ $( ( $name:ident , $str:expr , $str2:expr , $length:expr , $nuses:expr , $ndefs:expr , $flags:expr ) , )* ] ) => {
                match self {
                    $( Opcode::$name => $length , )*
                }
            }
        }

        using_opcode_database!(select_length!())
    }

    /// Number of stack slots consumed by this instruction, or -1 for variadic
    /// instructions.
    pub fn nuses(self) -> isize {
        macro_rules! select_nuses {
            ( [ $( ( $name:ident , $str:expr , $str2:expr , $length:expr , $nuses:expr , $ndefs:expr , $format:expr ) , )* ] ) => {
                match self {
                    $( Opcode::$name => $nuses , )*
                }
            }
        }

        using_opcode_database!(select_nuses!())
    }

    pub fn ndefs(self) -> usize {
        macro_rules! select_ndefs {
            ( [ $( ( $name:ident , $str:expr , $str2:expr , $length:expr , $nuses:expr , $ndefs:expr , $format:expr ) , )* ] ) => {
                match self {
                    $( Opcode::$name => $ndefs , )*
                }
            }
        }

        using_opcode_database!(select_ndefs!())
    }

    fn format_bits(self) -> u32 {
        macro_rules! select_format {
            ( [ $( ( $name:ident , $str:expr , $str2:expr , $length:expr , $nuses:expr , $ndefs:expr , $format:expr ) , )* ] ) => {
                match self {
                    $( Opcode::$name => $format , )*
                }
            }
        }

        using_opcode_database!(select_format!())
    }

    pub fn has_ic_entry(self) -> bool {
        self.format_bits() & JOF_IC != 0
    }

    pub fn has_ic_index(self) -> bool {
        match self.format_bits() & JOF_TYPEMASK {
            JOF_LOOPHEAD | JOF_ICINDEX => true,
            _ => false,
        }
    }

    pub fn has_argc(self) -> bool {
        self.format_bits() & JOF_TYPEMASK == JOF_ARGC
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_matches() {
        for (i, t) in TABLE.iter().enumerate() {
            assert_eq!(i, t.to_byte() as usize);
        }
    }
}
