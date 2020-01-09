use std::convert::TryFrom;

macro_rules! using_opcode_database {
    ( $macro:ident ! ( ) ) => {
        $macro! {
            [
                (JSOP_UNDEFINED, Undefined, undefined, js_undefined_str, "", 1, 0, 1, JOF_BYTE),
                (JSOP_NULL, Null, null, js_null_str, js_null_str, 1, 0, 1, JOF_BYTE),
                (JSOP_FALSE, False, false_, js_false_str, js_false_str, 1, 0, 1, JOF_BYTE),
                (JSOP_TRUE, True, true_, js_true_str, js_true_str, 1, 0, 1, JOF_BYTE),
                (JSOP_INT32, Int32, int32, "int32", NULL, 5, 0, 1, JOF_INT32),
                (JSOP_ZERO, Zero, zero, "zero", "0", 1, 0, 1, JOF_BYTE),
                (JSOP_ONE, One, one, "one", "1", 1, 0, 1, JOF_BYTE),
                (JSOP_INT8, Int8, int8, "int8", NULL, 2, 0, 1, JOF_INT8),
                (JSOP_UINT16, Uint16, uint16, "uint16", NULL, 3, 0, 1, JOF_UINT16),
                (JSOP_UINT24, Uint24, uint24, "uint24", NULL, 4, 0, 1, JOF_UINT24),
                (JSOP_DOUBLE, Double, double_, "double", NULL, 9, 0, 1, JOF_DOUBLE),
                (JSOP_BIGINT, BigInt, big_int, "bigint", NULL, 5, 0, 1, JOF_BIGINT),
                (JSOP_STRING, String, string, "string", NULL, 5, 0, 1, JOF_ATOM),
                (JSOP_SYMBOL, Symbol, symbol, "symbol", NULL, 2, 0, 1, JOF_UINT8),
                (JSOP_VOID, Void, void_, js_void_str, NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_TYPEOF, Typeof, typeof, js_typeof_str, NULL, 1, 1, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (JSOP_TYPEOFEXPR, TypeofExpr, typeof_expr, "typeofexpr", NULL, 1, 1, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (JSOP_POS, Pos, pos, "pos", "+ ", 1, 1, 1, JOF_BYTE),
                (JSOP_NEG, Neg, neg, "neg", "- ", 1, 1, 1, JOF_BYTE|JOF_IC),
                (JSOP_BITNOT, BitNot, bit_not, "bitnot", "~", 1, 1, 1, JOF_BYTE|JOF_IC),
                (JSOP_NOT, Not, not_, "not", "!", 1, 1, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (JSOP_BITOR, BitOr, bit_or, "bitor", "|",  1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_BITXOR, BitXor, bit_xor, "bitxor", "^", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_BITAND, BitAnd, bit_and, "bitand", "&", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_EQ, Eq, eq, "eq", "==", 1, 2, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (JSOP_NE, Ne, ne, "ne", "!=", 1, 2, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (JSOP_STRICTEQ, StrictEq, strict_eq, "stricteq", "===", 1, 2, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (JSOP_STRICTNE, StrictNe, strict_ne, "strictne", "!==", 1, 2, 1, JOF_BYTE|JOF_DETECTING|JOF_IC),
                (JSOP_LT, Lt, lt, "lt", "<",  1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_GT, Gt, gt, "gt", ">",  1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_LE, Le, le, "le", "<=", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_GE, Ge, ge, "ge", ">=", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_INSTANCEOF, Instanceof, instanceof, js_instanceof_str, js_instanceof_str, 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_IN, In, in_, js_in_str, js_in_str, 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_LSH, Lsh, lsh, "lsh", "<<", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_RSH, Rsh, rsh, "rsh", ">>", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_URSH, Ursh, ursh, "ursh", ">>>", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_ADD, Add, add, "add", "+", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_SUB, Sub, sub, "sub", "-", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_INC, Inc, inc, "inc", NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (JSOP_DEC, Dec, dec, "dec", NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (JSOP_MUL, Mul, mul, "mul", "*", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_DIV, Div, div, "div", "/", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_MOD, Mod, mod, "mod", "%", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_POW, Pow, pow, "pow", "**", 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_TOID, ToId, to_id, "toid", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_TONUMERIC, ToNumeric, to_numeric, "tonumeric", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_TOSTRING, ToString, to_string, "tostring", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_GLOBALTHIS, GlobalThis, global_this, "globalthis", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_NEWTARGET, NewTarget, new_target, "newtarget", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_DYNAMIC_IMPORT, DynamicImport, dynamic_import, "dynamic-import", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_IMPORTMETA, ImportMeta, import_meta, "importmeta", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_NEWINIT, NewInit, new_init, "newinit", NULL, 5, 0, 1, JOF_UINT32|JOF_IC),
                (JSOP_NEWOBJECT, NewObject, new_object, "newobject", NULL, 5, 0, 1, JOF_OBJECT|JOF_IC),
                (JSOP_NEWOBJECT_WITHGROUP, NewObjectWithGroup, new_object_with_group, "newobjectwithgroup", NULL, 5, 0, 1, JOF_OBJECT|JOF_IC),
                (JSOP_OBJECT, Object, object, "object", NULL, 5, 0, 1, JOF_OBJECT),
                (JSOP_OBJWITHPROTO, ObjWithProto, obj_with_proto, "objwithproto", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_INITPROP, InitProp, init_prop, "initprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (JSOP_INITHIDDENPROP, InitHiddenProp, init_hidden_prop, "inithiddenprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (JSOP_INITLOCKEDPROP, InitLockedProp, init_locked_prop, "initlockedprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (JSOP_INITELEM, InitElem, init_elem, "initelem", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (JSOP_INITHIDDENELEM, InitHiddenElem, init_hidden_elem, "inithiddenelem", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (JSOP_INITPROP_GETTER, InitPropGetter, init_prop_getter, "initprop_getter", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_INITHIDDENPROP_GETTER, InitHiddenPropGetter, init_hidden_prop_getter, "inithiddenprop_getter", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_INITELEM_GETTER, InitElemGetter, init_elem_getter, "initelem_getter", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_INITHIDDENELEM_GETTER, InitHiddenElemGetter, init_hidden_elem_getter, "inithiddenelem_getter", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_INITPROP_SETTER, InitPropSetter, init_prop_setter, "initprop_setter", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_INITHIDDENPROP_SETTER, InitHiddenPropSetter, init_hidden_prop_setter, "inithiddenprop_setter", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_INITELEM_SETTER, InitElemSetter, init_elem_setter, "initelem_setter", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_INITHIDDENELEM_SETTER, InitHiddenElemSetter, init_hidden_elem_setter, "inithiddenelem_setter", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_GETPROP, GetProp, get_prop, "getprop", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_TYPESET|JOF_IC),
                (JSOP_CALLPROP, CallProp, call_prop, "callprop", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_TYPESET|JOF_IC),
                (JSOP_GETELEM, GetElem, get_elem, "getelem", NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_TYPESET|JOF_IC),
                (JSOP_CALLELEM, CallElem, call_elem, "callelem", NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_TYPESET|JOF_IC),
                (JSOP_LENGTH, Length, length, "length", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_TYPESET|JOF_IC),
                (JSOP_SETPROP, SetProp, set_prop, "setprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY|JOF_IC),
                (JSOP_STRICTSETPROP, StrictSetProp, strict_set_prop, "strict-setprop", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT|JOF_IC),
                (JSOP_SETELEM, SetElem, set_elem, "setelem", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY|JOF_IC),
                (JSOP_STRICTSETELEM, StrictSetElem, strict_set_elem, "strict-setelem", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT|JOF_IC),
                (JSOP_DELPROP, DelProp, del_prop, "delprop", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_CHECKSLOPPY),
                (JSOP_STRICTDELPROP, StrictDelProp, strict_del_prop, "strict-delprop", NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_CHECKSTRICT),
                (JSOP_DELELEM, DelElem, del_elem, "delelem", NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_CHECKSLOPPY),
                (JSOP_STRICTDELELEM, StrictDelElem, strict_del_elem, "strict-delelem", NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_CHECKSTRICT),
                (JSOP_HASOWN, HasOwn, has_own, "hasown", NULL, 1, 2, 1, JOF_BYTE|JOF_IC),
                (JSOP_SUPERBASE, SuperBase, super_base, "superbase", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_GETPROP_SUPER, GetPropSuper, get_prop_super, "getprop-super", NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_TYPESET|JOF_IC),
                (JSOP_GETELEM_SUPER, GetElemSuper, get_elem_super, "getelem-super", NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_TYPESET|JOF_IC),
                (JSOP_SETPROP_SUPER, SetPropSuper, set_prop_super, "setprop-super", NULL, 5, 3, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY),
                (JSOP_STRICTSETPROP_SUPER, StrictSetPropSuper, strict_set_prop_super, "strictsetprop-super", NULL, 5, 3, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT),
                (JSOP_SETELEM_SUPER, SetElemSuper, set_elem_super, "setelem-super", NULL, 1, 4, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY),
                (JSOP_STRICTSETELEM_SUPER, StrictSetElemSuper, strict_set_elem_super, "strict-setelem-super", NULL, 1, 4, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT),
                (JSOP_ITER, Iter, iter, "iter", NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (JSOP_MOREITER, MoreIter, more_iter, "moreiter", NULL, 1, 1, 2, JOF_BYTE),
                (JSOP_ISNOITER, IsNoIter, is_no_iter, "isnoiter", NULL, 1, 1, 2, JOF_BYTE),
                (JSOP_ITERNEXT, IterNext, iter_next, "iternext", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_ENDITER, EndIter, end_iter, "enditer", NULL, 1, 2, 0, JOF_BYTE),
                (JSOP_CHECKISOBJ, CheckIsObj, check_is_obj, "checkisobj", NULL, 2, 1, 1, JOF_UINT8),
                (JSOP_CHECKISCALLABLE, CheckIsCallable, check_is_callable, "checkiscallable", NULL, 2, 1, 1, JOF_UINT8),
                (JSOP_CHECKOBJCOERCIBLE, CheckObjCoercible, check_obj_coercible, "checkobjcoercible", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_TOASYNCITER, ToAsyncIter, to_async_iter, "toasynciter", NULL, 1, 2, 1, JOF_BYTE),
                (JSOP_MUTATEPROTO, MutateProto, mutate_proto, "mutateproto", NULL, 1, 2, 1, JOF_BYTE),
                (JSOP_NEWARRAY, NewArray, new_array, "newarray", NULL, 5, 0, 1, JOF_UINT32|JOF_IC),
                (JSOP_INITELEM_ARRAY, InitElemArray, init_elem_array, "initelem_array", NULL, 5, 2, 1, JOF_UINT32|JOF_ELEM|JOF_PROPINIT|JOF_DETECTING|JOF_IC),
                (JSOP_INITELEM_INC, InitElemInc, init_elem_inc, "initelem_inc", NULL, 1, 3, 2, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_IC),
                (JSOP_HOLE, Hole, hole, "hole", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_NEWARRAY_COPYONWRITE, NewArrayCopyOnWrite, new_array_copy_on_write, "newarray_copyonwrite", NULL, 5, 0, 1, JOF_OBJECT),
                (JSOP_REGEXP, RegExp, reg_exp, "regexp", NULL, 5, 0, 1, JOF_REGEXP),
                (JSOP_LAMBDA, Lambda, lambda, "lambda", NULL, 5, 0, 1, JOF_OBJECT),
                (JSOP_LAMBDA_ARROW, LambdaArrow, lambda_arrow, "lambda_arrow", NULL, 5, 1, 1, JOF_OBJECT),
                (JSOP_SETFUNNAME, SetFunName, set_fun_name, "setfunname", NULL, 2, 2, 1, JOF_UINT8),
                (JSOP_INITHOMEOBJECT, InitHomeObject, init_home_object, "inithomeobject", NULL, 1, 2, 1, JOF_BYTE),
                (JSOP_CHECKCLASSHERITAGE, CheckClassHeritage, check_class_heritage, "checkclassheritage", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_FUNWITHPROTO, FunWithProto, fun_with_proto, "funwithproto", NULL, 5, 1, 1, JOF_OBJECT),
                (JSOP_CLASSCONSTRUCTOR, ClassConstructor, class_constructor, "classconstructor", NULL, 13, 0, 1, JOF_CLASS_CTOR),
                (JSOP_DERIVEDCONSTRUCTOR, DerivedConstructor, derived_constructor, "derivedconstructor", NULL, 13, 1, 1, JOF_CLASS_CTOR),
                (JSOP_BUILTINPROTO, BuiltinProto, builtin_proto, "builtinproto", NULL, 2, 0, 1, JOF_UINT8),
                (JSOP_CALL, Call, call, "call", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_CALLITER, CallIter, call_iter, "calliter", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_FUNAPPLY, FunApply, fun_apply, "funapply", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_FUNCALL, FunCall, fun_call, "funcall", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_CALL_IGNORES_RV, CallIgnoresRv, call_ignores_rv, "call-ignores-rv", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_SPREADCALL, SpreadCall, spread_call, "spreadcall", NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_OPTIMIZE_SPREADCALL, OptimizeSpreadCall, optimize_spread_call, "optimize-spreadcall", NULL, 1, 1, 2, JOF_BYTE),
                (JSOP_EVAL, Eval, eval, "eval", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_CHECKSLOPPY|JOF_IC),
                (JSOP_SPREADEVAL, SpreadEval, spread_eval, "spreadeval", NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_CHECKSLOPPY|JOF_IC),
                (JSOP_STRICTEVAL, StrictEval, strict_eval, "strict-eval", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_CHECKSTRICT|JOF_IC),
                (JSOP_STRICTSPREADEVAL, StrictSpreadEval, strict_spread_eval, "strict-spreadeval", NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_CHECKSTRICT|JOF_IC),
                (JSOP_IMPLICITTHIS, ImplicitThis, implicit_this, "implicitthis", "", 5, 0, 1, JOF_ATOM),
                (JSOP_GIMPLICITTHIS, GImplicitThis, g_implicit_this, "gimplicitthis", "", 5, 0, 1, JOF_ATOM),
                (JSOP_CALLSITEOBJ, CallSiteObj, call_site_obj, "callsiteobj", NULL, 5, 0, 1, JOF_OBJECT),
                (JSOP_IS_CONSTRUCTING, IsConstructing, is_constructing, "is-constructing", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_NEW, New, new_, "new", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC|JOF_IC),
                (JSOP_SUPERCALL, SuperCall, super_call, "supercall", NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_SPREADNEW, SpreadNew, spread_new, "spreadnew", NULL, 1, 4, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_SPREADSUPERCALL, SpreadSuperCall, spread_super_call, "spreadsupercall", NULL, 1, 4, 1, JOF_BYTE|JOF_INVOKE|JOF_TYPESET|JOF_IC),
                (JSOP_SUPERFUN, SuperFun, super_fun, "superfun", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_CHECKTHISREINIT, CheckThisReinit, check_this_reinit, "checkthisreinit", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_GENERATOR, Generator, generator, "generator", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_INITIALYIELD, InitialYield, initial_yield, "initialyield", NULL, 4, 1, 3, JOF_RESUMEINDEX),
                (JSOP_AFTERYIELD, AfterYield, after_yield, "afteryield", NULL, 5, 0, 0, JOF_ICINDEX),
                (JSOP_FINALYIELDRVAL, FinalYieldRval, final_yield_rval, "finalyieldrval", NULL, 1, 1, 0, JOF_BYTE),
                (JSOP_YIELD, Yield, yield, "yield", NULL, 4, 2, 3, JOF_RESUMEINDEX),
                (JSOP_ISGENCLOSING, IsGenClosing, is_gen_closing, "isgenclosing", NULL, 1, 1, 2, JOF_BYTE),
                (JSOP_ASYNCAWAIT, AsyncAwait, async_await, "async-await", NULL, 1, 2, 1, JOF_BYTE),
                (JSOP_ASYNCRESOLVE, AsyncResolve, async_resolve, "async-resolve", NULL, 2, 2, 1, JOF_UINT8),
                (JSOP_AWAIT, Await, await, "await", NULL, 4, 2, 3, JOF_RESUMEINDEX),
                (JSOP_TRYSKIPAWAIT, TrySkipAwait, try_skip_await, "tryskipawait", NULL, 1, 1, 2, JOF_BYTE),
                (JSOP_RESUMEKIND, ResumeKind, resume_kind, "resumekind", NULL, 2, 0, 1, JOF_UINT8),
                (JSOP_CHECK_RESUMEKIND, CheckResumeKind, check_resume_kind, "check-resumekind", NULL, 1, 3, 1, JOF_BYTE),
                (JSOP_RESUME, Resume, resume, "resume", NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE),
                (JSOP_JUMPTARGET, JumpTarget, jump_target, "jumptarget", NULL, 5, 0, 0, JOF_ICINDEX),
                (JSOP_LOOPHEAD, LoopHead, loop_head, "loophead", NULL, 6, 0, 0, JOF_LOOPHEAD),
                (JSOP_GOTO, Goto, goto_, "goto", NULL, 5, 0, 0, JOF_JUMP),
                (JSOP_IFEQ, IfEq, if_eq, "ifeq", NULL, 5, 1, 0, JOF_JUMP|JOF_DETECTING|JOF_IC),
                (JSOP_IFNE, IfNe, if_ne, "ifne", NULL, 5, 1, 0, JOF_JUMP|JOF_IC),
                (JSOP_AND, And, and_, "and", NULL, 5, 1, 1, JOF_JUMP|JOF_DETECTING|JOF_IC),
                (JSOP_OR, Or, or_, "or", NULL, 5, 1, 1, JOF_JUMP|JOF_DETECTING|JOF_IC),
                (JSOP_COALESCE, Coalesce, coalesce, "coalesce", NULL, 5, 1, 1, JOF_JUMP|JOF_DETECTING),
                (JSOP_CASE, Case, case_, "case", NULL, 5, 2, 1, JOF_JUMP),
                (JSOP_DEFAULT, Default, default_, "default", NULL, 5, 1, 0, JOF_JUMP),
                (JSOP_TABLESWITCH, TableSwitch, table_switch, "tableswitch", NULL, 16, 1, 0, JOF_TABLESWITCH|JOF_DETECTING),
                (JSOP_RETURN, Return, return_, "return", NULL, 1, 1, 0, JOF_BYTE),
                (JSOP_GETRVAL, GetRval, get_rval, "getrval", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_SETRVAL, SetRval, set_rval, "setrval", NULL, 1, 1, 0, JOF_BYTE),
                (JSOP_RETRVAL, RetRval, ret_rval, "retrval", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_CHECKRETURN, CheckReturn, check_return, "checkreturn", NULL, 1, 1, 0, JOF_BYTE),
                (JSOP_THROW, Throw, throw_, js_throw_str, NULL, 1, 1, 0, JOF_BYTE),
                (JSOP_THROWMSG, ThrowMsg, throw_msg, "throwmsg", NULL, 3, 0, 0, JOF_UINT16),
                (JSOP_THROWSETALIASEDCONST, ThrowSetAliasedConst, throw_set_aliased_const, "throwsetaliasedconst", NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME|JOF_DETECTING),
                (JSOP_THROWSETCALLEE, ThrowSetCallee, throw_set_callee, "throwsetcallee", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_THROWSETCONST, ThrowSetConst, throw_set_const, "throwsetconst", NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME|JOF_DETECTING),
                (JSOP_TRY, Try, try_, "try", NULL, 5, 0, 0, JOF_CODE_OFFSET),
                (JSOP_TRY_DESTRUCTURING, TryDestructuring, try_destructuring, "try-destructuring", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_EXCEPTION, Exception, exception, "exception", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_RESUMEINDEX, ResumeIndex, resume_index, "resume-index", NULL, 4, 0, 1, JOF_RESUMEINDEX),
                (JSOP_GOSUB, Gosub, gosub, "gosub", NULL, 5, 2, 0, JOF_JUMP),
                (JSOP_FINALLY, Finally, finally, "finally", NULL, 1, 0, 2, JOF_BYTE),
                (JSOP_RETSUB, Retsub, retsub, "retsub", NULL, 1, 2, 0, JOF_BYTE),
                (JSOP_UNINITIALIZED, Uninitialized, uninitialized, "uninitialized", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_INITLEXICAL, InitLexical, init_lexical, "initlexical", NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME|JOF_DETECTING),
                (JSOP_INITGLEXICAL, InitGLexical, init_g_lexical, "initglexical", NULL, 5, 1, 1, JOF_ATOM|JOF_NAME|JOF_PROPINIT|JOF_GNAME|JOF_IC),
                (JSOP_INITALIASEDLEXICAL, InitAliasedLexical, init_aliased_lexical, "initaliasedlexical", NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME|JOF_PROPINIT|JOF_DETECTING),
                (JSOP_CHECKLEXICAL, CheckLexical, check_lexical, "checklexical", NULL, 4, 0, 0, JOF_LOCAL|JOF_NAME),
                (JSOP_CHECKALIASEDLEXICAL, CheckAliasedLexical, check_aliased_lexical, "checkaliasedlexical", NULL, 5, 0, 0, JOF_ENVCOORD|JOF_NAME),
                (JSOP_CHECKTHIS, CheckThis, check_this, "checkthis", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_BINDGNAME, BindGName, bind_g_name, "bindgname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_GNAME|JOF_IC),
                (JSOP_BINDNAME, BindName, bind_name, "bindname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_IC),
                (JSOP_GETNAME, GetName, get_name, "getname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_IC),
                (JSOP_GETGNAME, GetGName, get_g_name, "getgname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_GNAME|JOF_IC),
                (JSOP_GETARG, GetArg, get_arg, "getarg", NULL, 3, 0, 1, JOF_QARG|JOF_NAME),
                (JSOP_GETLOCAL, GetLocal, get_local, "getlocal", NULL, 4, 0, 1, JOF_LOCAL|JOF_NAME),
                (JSOP_GETALIASEDVAR, GetAliasedVar, get_aliased_var, "getaliasedvar", NULL, 5, 0, 1, JOF_ENVCOORD|JOF_NAME|JOF_TYPESET|JOF_IC),
                (JSOP_GETIMPORT, GetImport, get_import, "getimport", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_IC),
                (JSOP_GETBOUNDNAME, GetBoundName, get_bound_name, "getboundname", NULL, 5, 1, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_IC),
                (JSOP_GETINTRINSIC, GetIntrinsic, get_intrinsic, "getintrinsic", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_TYPESET|JOF_IC),
                (JSOP_CALLEE, Callee, callee, "callee", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_ENVCALLEE, EnvCallee, env_callee, "envcallee", NULL, 2, 0, 1, JOF_UINT8),
                (JSOP_SETNAME, SetName, set_name, "setname", NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSLOPPY|JOF_IC),
                (JSOP_STRICTSETNAME, StrictSetName, strict_set_name, "strict-setname", NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_DETECTING|JOF_CHECKSTRICT|JOF_IC),
                (JSOP_SETGNAME, SetGName, set_g_name, "setgname", NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_DETECTING|JOF_GNAME|JOF_CHECKSLOPPY|JOF_IC),
                (JSOP_STRICTSETGNAME, StrictSetGName, strict_set_g_name, "strict-setgname", NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_DETECTING|JOF_GNAME|JOF_CHECKSTRICT|JOF_IC),
                (JSOP_SETARG, SetArg, set_arg, "setarg", NULL, 3, 1, 1, JOF_QARG|JOF_NAME),
                (JSOP_SETLOCAL, SetLocal, set_local, "setlocal", NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME|JOF_DETECTING),
                (JSOP_SETALIASEDVAR, SetAliasedVar, set_aliased_var, "setaliasedvar", NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME|JOF_PROPSET|JOF_DETECTING),
                (JSOP_SETINTRINSIC, SetIntrinsic, set_intrinsic, "setintrinsic", NULL, 5, 1, 1, JOF_ATOM|JOF_NAME|JOF_DETECTING),
                (JSOP_PUSHLEXICALENV, PushLexicalEnv, push_lexical_env, "pushlexicalenv", NULL, 5, 0, 0, JOF_SCOPE),
                (JSOP_POPLEXICALENV, PopLexicalEnv, pop_lexical_env, "poplexicalenv", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_DEBUGLEAVELEXICALENV, DebugLeaveLexicalEnv, debug_leave_lexical_env, "debugleavelexicalenv", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_RECREATELEXICALENV, RecreateLexicalEnv, recreate_lexical_env, "recreatelexicalenv", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_FRESHENLEXICALENV, FreshenLexicalEnv, freshen_lexical_env, "freshenlexicalenv", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_PUSHVARENV, PushVarEnv, push_var_env, "pushvarenv", NULL, 5, 0, 0, JOF_SCOPE),
                (JSOP_POPVARENV, PopVarEnv, pop_var_env, "popvarenv", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_ENTERWITH, EnterWith, enter_with, "enterwith", NULL, 5, 1, 0, JOF_SCOPE),
                (JSOP_LEAVEWITH, LeaveWith, leave_with, "leavewith", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_BINDVAR, BindVar, bind_var, "bindvar", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_DEFVAR, DefVar, def_var, "defvar", NULL, 5, 0, 0, JOF_ATOM),
                (JSOP_DEFFUN, DefFun, def_fun, "deffun", NULL, 1, 1, 0, JOF_BYTE),
                (JSOP_DEFLET, DefLet, def_let, "deflet", NULL, 5, 0, 0, JOF_ATOM),
                (JSOP_DEFCONST, DefConst, def_const, "defconst", NULL, 5, 0, 0, JOF_ATOM),
                (JSOP_DELNAME, DelName, del_name, "delname", NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_CHECKSLOPPY),
                (JSOP_ARGUMENTS, Arguments, arguments, "arguments", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_REST, Rest, rest, "rest", NULL, 1, 0, 1, JOF_BYTE|JOF_TYPESET|JOF_IC),
                (JSOP_FUNCTIONTHIS, FunctionThis, function_this, "functionthis", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_POP, Pop, pop, "pop", NULL, 1, 1, 0, JOF_BYTE),
                (JSOP_POPN, PopN, pop_n, "popn", NULL, 3, -1, 0, JOF_UINT16),
                (JSOP_DUP, Dup, dup, "dup", NULL, 1, 1, 2, JOF_BYTE),
                (JSOP_DUP2, Dup2, dup2, "dup2", NULL, 1, 2, 4, JOF_BYTE),
                (JSOP_DUPAT, DupAt, dup_at, "dupat", NULL, 4, 0, 1, JOF_UINT24),
                (JSOP_SWAP, Swap, swap, "swap", NULL, 1, 2, 2, JOF_BYTE),
                (JSOP_PICK, Pick, pick, "pick", NULL, 2, 0, 0, JOF_UINT8),
                (JSOP_UNPICK, Unpick, unpick, "unpick", NULL, 2, 0, 0, JOF_UINT8),
                (JSOP_NOP, Nop, nop, "nop", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_LINENO, Lineno, lineno, "lineno", NULL, 5, 0, 0, JOF_UINT32),
                (JSOP_NOP_DESTRUCTURING, NopDestructuring, nop_destructuring, "nop-destructuring", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_FORCEINTERPRETER, ForceInterpreter, force_interpreter, "forceinterpreter", NULL, 1, 0, 0, JOF_BYTE),
                (JSOP_DEBUGCHECKSELFHOSTED, DebugCheckSelfHosted, debug_check_self_hosted, "debug-checkselfhosted", NULL, 1, 1, 1, JOF_BYTE),
                (JSOP_INSTRUMENTATION_ACTIVE, InstrumentationActive, instrumentation_active, "instrumentationActive", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_INSTRUMENTATION_CALLBACK, InstrumentationCallback, instrumentation_callback, "instrumentationCallback", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_INSTRUMENTATION_SCRIPT_ID, InstrumentationScriptId, instrumentation_script_id, "instrumentationScriptId", NULL, 1, 0, 1, JOF_BYTE),
                (JSOP_DEBUGGER, Debugger, debugger, "debugger", NULL, 1, 0, 0, JOF_BYTE),
            ]
        }
    }
}

macro_rules! define_opcode_enum {
    ( [ $( ( $_op_upper:ident , $op_camel:ident , $( $_etc:tt )* ), )* ] ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        #[repr(u8)]
        pub enum Opcode {
            $( $op_camel, )*
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

const JSOP_LIMIT: usize = using_opcode_database!(count_rows!());

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

/// int32_t bytecode offset
const JOF_CODE_OFFSET: u32 = 23;

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
        if (value as usize) < JSOP_LIMIT {
            Ok(TABLE[value as usize])
        } else {
            Err(())
        }
    }
}

macro_rules! define_table {
    ( [ $( ( $op_upper:ident , $op_camel:ident , $( $_etc:tt )* ) , )* ] ) => {
        pub static TABLE: [Opcode; JSOP_LIMIT] = [
            $( Opcode::$op_camel , )*
        ];
    }
}

using_opcode_database!(define_table!());

impl Opcode {
    /// Length of this instruction, in bytes.
    pub fn instruction_length(self) -> usize {
        macro_rules! select_length {
            ( [ $(
                (
                    $op_upper:ident , $op_camel:ident , $op_snake:ident , $str:expr , $str2:expr ,
                    $length:expr , $nuses:expr , $ndefs:expr , $format:expr
                ) ,
            )* ] ) => {
                match self {
                    $( Opcode::$op_camel => $length , )*
                }
            }
        }

        using_opcode_database!(select_length!())
    }

    /// Number of stack slots consumed by this instruction, or -1 for variadic
    /// instructions.
    pub fn nuses(self) -> isize {
        macro_rules! select_nuses {
            ( [ $(
                (
                    $op_upper:ident , $op_camel:ident , $op_snake:ident , $str:expr , $str2:expr ,
                    $length:expr , $nuses:expr , $ndefs:expr , $format:expr
                ) ,
            )* ] ) => {
                match self {
                    $( Opcode::$op_camel => $nuses , )*
                }
            }
        }

        using_opcode_database!(select_nuses!())
    }

    pub fn ndefs(self) -> usize {
        macro_rules! select_ndefs {
            ( [ $(
                (
                    $op_upper:ident , $op_camel:ident , $op_snake:ident , $str:expr , $str2:expr ,
                    $length:expr , $nuses:expr , $ndefs:expr , $format:expr
                ) ,
            )* ] ) => {
                match self {
                    $( Opcode::$op_camel => $ndefs , )*
                }
            }
        }

        using_opcode_database!(select_ndefs!())
    }

    fn format_bits(self) -> u32 {
        macro_rules! select_format {
            ( [ $(
                (
                    $op_upper:ident , $op_camel:ident , $op_snake:ident , $str:expr , $str2:expr ,
                    $length:expr , $nuses:expr , $ndefs:expr , $format:expr
                ) ,
            )* ] ) => {
                match self {
                    $( Opcode::$op_camel => $format , )*
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
