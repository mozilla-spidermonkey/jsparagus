use std::convert::TryFrom;

macro_rules! using_opcode_database {
    ( $macro:ident ! ( ) ) => {
        $macro! {
            [
                // WARNING
                // The following section is generated by update_stencil.py.
                // Do mot modify manually.
                //
                // @@@@ BEGIN OPCODES @@@@
                (Undefined, undefined, "", 1, 0, 1, JOF_BYTE),
                (Null, null, "null", 1, 0, 1, JOF_BYTE),
                (False, false_, "false", 1, 0, 1, JOF_BYTE),
                (True, true_, "true", 1, 0, 1, JOF_BYTE),
                (Int32, int32, NULL, 5, 0, 1, JOF_INT32),
                (Zero, zero, "0", 1, 0, 1, JOF_BYTE),
                (One, one, "1", 1, 0, 1, JOF_BYTE),
                (Int8, int8, NULL, 2, 0, 1, JOF_INT8),
                (Uint16, uint16, NULL, 3, 0, 1, JOF_UINT16),
                (Uint24, uint24, NULL, 4, 0, 1, JOF_UINT24),
                (Double, double_, NULL, 9, 0, 1, JOF_DOUBLE),
                (BigInt, big_int, NULL, 5, 0, 1, JOF_BIGINT),
                (String, string, NULL, 5, 0, 1, JOF_STRING),
                (Symbol, symbol, NULL, 2, 0, 1, JOF_UINT8),
                (Void, void_, NULL, 1, 1, 1, JOF_BYTE),
                (Typeof, typeof_, NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (TypeofExpr, typeof_expr, NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (Pos, pos, "+ ", 1, 1, 1, JOF_BYTE|JOF_IC),
                (Neg, neg, "- ", 1, 1, 1, JOF_BYTE|JOF_IC),
                (BitNot, bit_not, "~", 1, 1, 1, JOF_BYTE|JOF_IC),
                (Not, not_, "!", 1, 1, 1, JOF_BYTE|JOF_IC),
                (BitOr, bit_or, "|",  1, 2, 1, JOF_BYTE|JOF_IC),
                (BitXor, bit_xor, "^", 1, 2, 1, JOF_BYTE|JOF_IC),
                (BitAnd, bit_and, "&", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Eq, eq, "==", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Ne, ne, "!=", 1, 2, 1, JOF_BYTE|JOF_IC),
                (StrictEq, strict_eq, "===", 1, 2, 1, JOF_BYTE|JOF_IC),
                (StrictNe, strict_ne, "!==", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Lt, lt, "<",  1, 2, 1, JOF_BYTE|JOF_IC),
                (Gt, gt, ">",  1, 2, 1, JOF_BYTE|JOF_IC),
                (Le, le, "<=", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Ge, ge, ">=", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Instanceof, instanceof, "instanceof", 1, 2, 1, JOF_BYTE|JOF_IC),
                (In, in_, "in", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Lsh, lsh, "<<", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Rsh, rsh, ">>", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Ursh, ursh, ">>>", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Add, add, "+", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Sub, sub, "-", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Inc, inc, NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (Dec, dec, NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (Mul, mul, "*", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Div, div, "/", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Mod, mod, "%", 1, 2, 1, JOF_BYTE|JOF_IC),
                (Pow, pow, "**", 1, 2, 1, JOF_BYTE|JOF_IC),
                (ToPropertyKey, to_property_key, NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (ToNumeric, to_numeric, NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (ToString, to_string, NULL, 1, 1, 1, JOF_BYTE),
                (IsNullOrUndefined, is_null_or_undefined, NULL, 1, 1, 2, JOF_BYTE),
                (GlobalThis, global_this, NULL, 1, 0, 1, JOF_BYTE),
                (NonSyntacticGlobalThis, non_syntactic_global_this, NULL, 1, 0, 1, JOF_BYTE),
                (NewTarget, new_target, NULL, 1, 0, 1, JOF_BYTE),
                (DynamicImport, dynamic_import, NULL, 1, 2, 1, JOF_BYTE),
                (ImportMeta, import_meta, NULL, 1, 0, 1, JOF_BYTE),
                (NewInit, new_init, NULL, 1, 0, 1, JOF_BYTE|JOF_IC),
                (NewObject, new_object, NULL, 5, 0, 1, JOF_SHAPE|JOF_IC),
                (Object, object, NULL, 5, 0, 1, JOF_OBJECT),
                (ObjWithProto, obj_with_proto, NULL, 1, 1, 1, JOF_BYTE),
                (InitProp, init_prop, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_IC),
                (InitHiddenProp, init_hidden_prop, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_IC),
                (InitLockedProp, init_locked_prop, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT|JOF_IC),
                (InitElem, init_elem, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_IC),
                (InitHiddenElem, init_hidden_elem, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_IC),
                (InitLockedElem, init_locked_elem, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_IC),
                (InitPropGetter, init_prop_getter, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT),
                (InitHiddenPropGetter, init_hidden_prop_getter, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT),
                (InitElemGetter, init_elem_getter, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT),
                (InitHiddenElemGetter, init_hidden_elem_getter, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT),
                (InitPropSetter, init_prop_setter, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT),
                (InitHiddenPropSetter, init_hidden_prop_setter, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPINIT),
                (InitElemSetter, init_elem_setter, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT),
                (InitHiddenElemSetter, init_hidden_elem_setter, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPINIT),
                (GetProp, get_prop, NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_IC),
                (GetElem, get_elem, NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_IC),
                (SetProp, set_prop, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_CHECKSLOPPY|JOF_IC),
                (StrictSetProp, strict_set_prop, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_CHECKSTRICT|JOF_IC),
                (SetElem, set_elem, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_CHECKSLOPPY|JOF_IC),
                (StrictSetElem, strict_set_elem, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_CHECKSTRICT|JOF_IC),
                (DelProp, del_prop, NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_CHECKSLOPPY),
                (StrictDelProp, strict_del_prop, NULL, 5, 1, 1, JOF_ATOM|JOF_PROP|JOF_CHECKSTRICT),
                (DelElem, del_elem, NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_CHECKSLOPPY),
                (StrictDelElem, strict_del_elem, NULL, 1, 2, 1, JOF_BYTE|JOF_ELEM|JOF_CHECKSTRICT),
                (HasOwn, has_own, NULL, 1, 2, 1, JOF_BYTE|JOF_IC),
                (CheckPrivateField, check_private_field, NULL, 3, 2, 3, JOF_TWO_UINT8|JOF_CHECKSTRICT|JOF_IC),
                (NewPrivateName, new_private_name, NULL, 5, 0, 1, JOF_ATOM),
                (SuperBase, super_base, NULL, 1, 1, 1, JOF_BYTE),
                (GetPropSuper, get_prop_super, NULL, 5, 2, 1, JOF_ATOM|JOF_PROP|JOF_IC),
                (GetElemSuper, get_elem_super, NULL, 1, 3, 1, JOF_BYTE|JOF_ELEM|JOF_IC),
                (SetPropSuper, set_prop_super, NULL, 5, 3, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_CHECKSLOPPY),
                (StrictSetPropSuper, strict_set_prop_super, NULL, 5, 3, 1, JOF_ATOM|JOF_PROP|JOF_PROPSET|JOF_CHECKSTRICT),
                (SetElemSuper, set_elem_super, NULL, 1, 4, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_CHECKSLOPPY),
                (StrictSetElemSuper, strict_set_elem_super, NULL, 1, 4, 1, JOF_BYTE|JOF_ELEM|JOF_PROPSET|JOF_CHECKSTRICT),
                (Iter, iter, NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (MoreIter, more_iter, NULL, 1, 1, 2, JOF_BYTE),
                (IsNoIter, is_no_iter, NULL, 1, 1, 2, JOF_BYTE),
                (EndIter, end_iter, NULL, 1, 2, 0, JOF_BYTE),
                (CheckIsObj, check_is_obj, NULL, 2, 1, 1, JOF_UINT8),
                (CheckObjCoercible, check_obj_coercible, NULL, 1, 1, 1, JOF_BYTE),
                (ToAsyncIter, to_async_iter, NULL, 1, 2, 1, JOF_BYTE),
                (MutateProto, mutate_proto, NULL, 1, 2, 1, JOF_BYTE),
                (NewArray, new_array, NULL, 5, 0, 1, JOF_UINT32|JOF_IC),
                (InitElemArray, init_elem_array, NULL, 5, 2, 1, JOF_UINT32|JOF_ELEM|JOF_PROPINIT),
                (InitElemInc, init_elem_inc, NULL, 1, 3, 2, JOF_BYTE|JOF_ELEM|JOF_PROPINIT|JOF_IC),
                (Hole, hole, NULL, 1, 0, 1, JOF_BYTE),
                (RegExp, reg_exp, NULL, 5, 0, 1, JOF_REGEXP),
                (Lambda, lambda, NULL, 5, 0, 1, JOF_OBJECT),
                (SetFunName, set_fun_name, NULL, 2, 2, 1, JOF_UINT8),
                (InitHomeObject, init_home_object, NULL, 1, 2, 1, JOF_BYTE),
                (CheckClassHeritage, check_class_heritage, NULL, 1, 1, 1, JOF_BYTE),
                (FunWithProto, fun_with_proto, NULL, 5, 1, 1, JOF_OBJECT),
                (BuiltinObject, builtin_object, NULL, 2, 0, 1, JOF_UINT8),
                (Call, call, NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_IC),
                (CallIter, call_iter, NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_IC),
                (CallIgnoresRv, call_ignores_rv, NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_IC),
                (SpreadCall, spread_call, NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_SPREAD|JOF_IC),
                (OptimizeSpreadCall, optimize_spread_call, NULL, 1, 1, 1, JOF_BYTE|JOF_IC),
                (Eval, eval, NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_CHECKSLOPPY|JOF_IC),
                (SpreadEval, spread_eval, NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_SPREAD|JOF_CHECKSLOPPY|JOF_IC),
                (StrictEval, strict_eval, NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_CHECKSTRICT|JOF_IC),
                (StrictSpreadEval, strict_spread_eval, NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE|JOF_SPREAD|JOF_CHECKSTRICT|JOF_IC),
                (ImplicitThis, implicit_this, "", 5, 0, 1, JOF_ATOM),
                (CallSiteObj, call_site_obj, NULL, 5, 0, 1, JOF_OBJECT),
                (IsConstructing, is_constructing, NULL, 1, 0, 1, JOF_BYTE),
                (New, new_, NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_CONSTRUCT|JOF_IC),
                (SuperCall, super_call, NULL, 3, -1, 1, JOF_ARGC|JOF_INVOKE|JOF_CONSTRUCT|JOF_IC),
                (SpreadNew, spread_new, NULL, 1, 4, 1, JOF_BYTE|JOF_INVOKE|JOF_CONSTRUCT|JOF_SPREAD|JOF_IC),
                (SpreadSuperCall, spread_super_call, NULL, 1, 4, 1, JOF_BYTE|JOF_INVOKE|JOF_CONSTRUCT|JOF_SPREAD|JOF_IC),
                (SuperFun, super_fun, NULL, 1, 1, 1, JOF_BYTE),
                (CheckThisReinit, check_this_reinit, NULL, 1, 1, 1, JOF_BYTE),
                (Generator, generator, NULL, 1, 0, 1, JOF_BYTE),
                (InitialYield, initial_yield, NULL, 4, 1, 3, JOF_RESUMEINDEX),
                (AfterYield, after_yield, NULL, 5, 0, 0, JOF_ICINDEX),
                (FinalYieldRval, final_yield_rval, NULL, 1, 1, 0, JOF_BYTE),
                (Yield, yield, NULL, 4, 2, 3, JOF_RESUMEINDEX),
                (IsGenClosing, is_gen_closing, NULL, 1, 1, 2, JOF_BYTE),
                (AsyncAwait, async_await, NULL, 1, 2, 1, JOF_BYTE),
                (AsyncResolve, async_resolve, NULL, 2, 2, 1, JOF_UINT8),
                (Await, await, NULL, 4, 2, 3, JOF_RESUMEINDEX),
                (CanSkipAwait, can_skip_await, NULL, 1, 1, 2, JOF_BYTE),
                (MaybeExtractAwaitValue, maybe_extract_await_value, NULL, 1, 2, 2, JOF_BYTE),
                (ResumeKind, resume_kind, NULL, 2, 0, 1, JOF_UINT8),
                (CheckResumeKind, check_resume_kind, NULL, 1, 3, 1, JOF_BYTE),
                (Resume, resume, NULL, 1, 3, 1, JOF_BYTE|JOF_INVOKE),
                (JumpTarget, jump_target, NULL, 5, 0, 0, JOF_ICINDEX),
                (LoopHead, loop_head, NULL, 6, 0, 0, JOF_LOOPHEAD),
                (Goto, goto_, NULL, 5, 0, 0, JOF_JUMP),
                (JumpIfFalse, jump_if_false, NULL, 5, 1, 0, JOF_JUMP|JOF_IC),
                (JumpIfTrue, jump_if_true, NULL, 5, 1, 0, JOF_JUMP|JOF_IC),
                (And, and_, NULL, 5, 1, 1, JOF_JUMP|JOF_IC),
                (Or, or_, NULL, 5, 1, 1, JOF_JUMP|JOF_IC),
                (Coalesce, coalesce, NULL, 5, 1, 1, JOF_JUMP),
                (Case, case_, NULL, 5, 2, 1, JOF_JUMP),
                (Default, default_, NULL, 5, 1, 0, JOF_JUMP),
                (TableSwitch, table_switch, NULL, 16, 1, 0, JOF_TABLESWITCH),
                (Return, return_, NULL, 1, 1, 0, JOF_BYTE),
                (GetRval, get_rval, NULL, 1, 0, 1, JOF_BYTE),
                (SetRval, set_rval, NULL, 1, 1, 0, JOF_BYTE),
                (RetRval, ret_rval, NULL, 1, 0, 0, JOF_BYTE),
                (CheckReturn, check_return, NULL, 1, 1, 1, JOF_BYTE),
                (Throw, throw_, NULL, 1, 1, 0, JOF_BYTE),
                (ThrowMsg, throw_msg, NULL, 2, 0, 0, JOF_UINT8),
                (ThrowSetConst, throw_set_const, NULL, 5, 0, 0, JOF_ATOM|JOF_NAME),
                (Try, try_, NULL, 1, 0, 0, JOF_BYTE),
                (TryDestructuring, try_destructuring, NULL, 1, 0, 0, JOF_BYTE),
                (Exception, exception, NULL, 1, 0, 1, JOF_BYTE),
                (ResumeIndex, resume_index, NULL, 4, 0, 1, JOF_RESUMEINDEX),
                (Finally, finally, NULL, 1, 0, 0, JOF_BYTE),
                (Retsub, retsub, NULL, 1, 1, 0, JOF_BYTE),
                (Uninitialized, uninitialized, NULL, 1, 0, 1, JOF_BYTE),
                (InitLexical, init_lexical, NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME),
                (InitGLexical, init_g_lexical, NULL, 5, 1, 1, JOF_ATOM|JOF_NAME|JOF_PROPINIT|JOF_GNAME|JOF_IC),
                (InitAliasedLexical, init_aliased_lexical, NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME|JOF_PROPINIT),
                (CheckLexical, check_lexical, NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME),
                (CheckAliasedLexical, check_aliased_lexical, NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME),
                (CheckThis, check_this, NULL, 1, 1, 1, JOF_BYTE),
                (BindGName, bind_g_name, NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_GNAME|JOF_IC),
                (BindName, bind_name, NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_IC),
                (GetName, get_name, NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_IC),
                (GetGName, get_g_name, NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_GNAME|JOF_IC),
                (GetArg, get_arg, NULL, 3, 0, 1, JOF_QARG|JOF_NAME),
                (GetLocal, get_local, NULL, 4, 0, 1, JOF_LOCAL|JOF_NAME),
                (GetAliasedVar, get_aliased_var, NULL, 5, 0, 1, JOF_ENVCOORD|JOF_NAME),
                (GetAliasedDebugVar, get_aliased_debug_var, NULL, 5, 0, 1, JOF_DEBUGCOORD|JOF_NAME),
                (GetImport, get_import, NULL, 5, 0, 1, JOF_ATOM|JOF_NAME),
                (GetBoundName, get_bound_name, NULL, 5, 1, 1, JOF_ATOM|JOF_NAME|JOF_IC),
                (GetIntrinsic, get_intrinsic, NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_IC),
                (Callee, callee, NULL, 1, 0, 1, JOF_BYTE),
                (EnvCallee, env_callee, NULL, 2, 0, 1, JOF_UINT8),
                (SetName, set_name, NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_CHECKSLOPPY|JOF_IC),
                (StrictSetName, strict_set_name, NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_CHECKSTRICT|JOF_IC),
                (SetGName, set_g_name, NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_GNAME|JOF_CHECKSLOPPY|JOF_IC),
                (StrictSetGName, strict_set_g_name, NULL, 5, 2, 1, JOF_ATOM|JOF_NAME|JOF_PROPSET|JOF_GNAME|JOF_CHECKSTRICT|JOF_IC),
                (SetArg, set_arg, NULL, 3, 1, 1, JOF_QARG|JOF_NAME),
                (SetLocal, set_local, NULL, 4, 1, 1, JOF_LOCAL|JOF_NAME),
                (SetAliasedVar, set_aliased_var, NULL, 5, 1, 1, JOF_ENVCOORD|JOF_NAME|JOF_PROPSET),
                (SetIntrinsic, set_intrinsic, NULL, 5, 1, 1, JOF_ATOM|JOF_NAME),
                (PushLexicalEnv, push_lexical_env, NULL, 5, 0, 0, JOF_SCOPE),
                (PopLexicalEnv, pop_lexical_env, NULL, 1, 0, 0, JOF_BYTE),
                (DebugLeaveLexicalEnv, debug_leave_lexical_env, NULL, 1, 0, 0, JOF_BYTE),
                (RecreateLexicalEnv, recreate_lexical_env, NULL, 1, 0, 0, JOF_BYTE),
                (FreshenLexicalEnv, freshen_lexical_env, NULL, 1, 0, 0, JOF_BYTE),
                (PushClassBodyEnv, push_class_body_env, NULL, 5, 0, 0, JOF_SCOPE),
                (PushVarEnv, push_var_env, NULL, 5, 0, 0, JOF_SCOPE),
                (EnterWith, enter_with, NULL, 5, 1, 0, JOF_SCOPE),
                (LeaveWith, leave_with, NULL, 1, 0, 0, JOF_BYTE),
                (BindVar, bind_var, NULL, 1, 0, 1, JOF_BYTE),
                (GlobalOrEvalDeclInstantiation, global_or_eval_decl_instantiation, NULL, 5, 0, 0, JOF_GCTHING),
                (DelName, del_name, NULL, 5, 0, 1, JOF_ATOM|JOF_NAME|JOF_CHECKSLOPPY),
                (Arguments, arguments, NULL, 1, 0, 1, JOF_BYTE),
                (Rest, rest, NULL, 1, 0, 1, JOF_BYTE|JOF_IC),
                (FunctionThis, function_this, NULL, 1, 0, 1, JOF_BYTE),
                (Pop, pop, NULL, 1, 1, 0, JOF_BYTE),
                (PopN, pop_n, NULL, 3, -1, 0, JOF_UINT16),
                (Dup, dup, NULL, 1, 1, 2, JOF_BYTE),
                (Dup2, dup2, NULL, 1, 2, 4, JOF_BYTE),
                (DupAt, dup_at, NULL, 4, 0, 1, JOF_UINT24),
                (Swap, swap, NULL, 1, 2, 2, JOF_BYTE),
                (Pick, pick, NULL, 2, 0, 0, JOF_UINT8),
                (Unpick, unpick, NULL, 2, 0, 0, JOF_UINT8),
                (Nop, nop, NULL, 1, 0, 0, JOF_BYTE),
                (Lineno, lineno, NULL, 5, 0, 0, JOF_UINT32),
                (NopDestructuring, nop_destructuring, NULL, 1, 0, 0, JOF_BYTE),
                (ForceInterpreter, force_interpreter, NULL, 1, 0, 0, JOF_BYTE),
                (DebugCheckSelfHosted, debug_check_self_hosted, NULL, 1, 1, 1, JOF_BYTE),
                (Debugger, debugger, NULL, 1, 0, 0, JOF_BYTE),
                // @@@@ END OPCODES @@@@
            ]
        }
    }
}

macro_rules! define_opcode_enum {
    ( [ $( ( $op:ident , $( $_etc:tt )* ), )* ] ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        #[repr(u8)]
        pub enum Opcode {
            $( $op, )*
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

// WARNING
// The following section is generated by update_stencil.py.
// Do mot modify manually.
//
// @@@@ BEGIN FLAGS @@@@
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

/// yield, await, or retsub resume index
const JOF_RESUMEINDEX: u32 = 13;

/// inline DoubleValue
const JOF_DOUBLE: u32 = 14;

/// uint32_t generic gc-thing index
const JOF_GCTHING: u32 = 15;

/// uint32_t constant index
const JOF_ATOM: u32 = 16;

/// uint32_t object index
const JOF_OBJECT: u32 = 17;

/// uint32_t regexp index
const JOF_REGEXP: u32 = 18;

/// uint32_t scope index
const JOF_SCOPE: u32 = 19;

/// uint32_t index for BigInt value
const JOF_BIGINT: u32 = 20;

/// uint32_t IC index
const JOF_ICINDEX: u32 = 21;

/// JSOp::LoopHead, combines JOF_ICINDEX and JOF_UINT8
const JOF_LOOPHEAD: u32 = 22;

/// A pair of unspecified uint8_t arguments
const JOF_TWO_UINT8: u32 = 23;

/// An embedded ScopeCoordinate immediate that may traverse DebugEnvironmentProxies
const JOF_DEBUGCOORD: u32 = 24;

/// uint32_t shape index
const JOF_SHAPE: u32 = 25;

/// uint32_t constant index
const JOF_STRING: u32 = 26;

/// mask for above immediate types
const JOF_TYPEMASK: u32 = 0xFF;

/// name operation
const JOF_NAME: u32 = 1 << 8;

/// obj.prop operation
const JOF_PROP: u32 = 2 << 8;

/// obj[index] operation
const JOF_ELEM: u32 = 3 << 8;

/// property/element/name set operation
const JOF_PROPSET: u32 = 1 << 16;

/// property/element/name init operation
const JOF_PROPINIT: u32 = 1 << 17;

/// op can only be generated in sloppy mode
const JOF_CHECKSLOPPY: u32 = 1 << 18;

/// op can only be generated in strict mode
const JOF_CHECKSTRICT: u32 = 1 << 19;

/// any call, construct, or eval instruction
const JOF_INVOKE: u32 = 1 << 20;

/// invoke instruction using [[Construct]] entry
const JOF_CONSTRUCT: u32 = 1 << 21;

/// invoke instruction using spread argument
const JOF_SPREAD: u32 = 1 << 22;

/// predicted global name
const JOF_GNAME: u32 = 1 << 23;

/// baseline may use an IC for this op
const JOF_IC: u32 = 1 << 24;

// @@@@ END FLAGS @@@@

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

    pub fn is_jump_target(self) -> bool {
        self == Opcode::JumpTarget || self == Opcode::LoopHead || self == Opcode::AfterYield
    }

    pub fn is_jump(self) -> bool {
        self == Opcode::Goto
            || self == Opcode::JumpIfFalse
            || self == Opcode::JumpIfTrue
            || self == Opcode::Or
            || self == Opcode::And
            || self == Opcode::Coalesce
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
    ( [ $(
        (
            $op:ident , $op_snake:ident , $str:expr ,
            $length:expr , $nuses:expr , $ndefs:expr , $format:expr
        ) ,
    )* ] ) => {
        pub static TABLE: [Opcode; JSOP_LIMIT] = [
            $( Opcode::$op , )*
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
                    $op:ident , $op_snake:ident , $str:expr ,
                    $length:expr , $nuses:expr , $ndefs:expr , $format:expr
                ) ,
            )* ] ) => {
                match self {
                    $( Opcode::$op => $length , )*
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
                    $op:ident , $op_snake:ident , $str:expr ,
                    $length:expr , $nuses:expr , $ndefs:expr , $format:expr
                ) ,
            )* ] ) => {
                match self {
                    $( Opcode::$op => $nuses , )*
                }
            }
        }

        using_opcode_database!(select_nuses!())
    }

    pub fn ndefs(self) -> usize {
        macro_rules! select_ndefs {
            ( [ $(
                (
                    $op:ident , $op_snake:ident , $str:expr ,
                    $length:expr , $nuses:expr , $ndefs:expr , $format:expr
                ) ,
            )* ] ) => {
                match self {
                    $( Opcode::$op => $ndefs , )*
                }
            }
        }

        using_opcode_database!(select_ndefs!())
    }

    fn format_bits(self) -> u32 {
        macro_rules! select_format {
            ( [ $(
                (
                    $op:ident , $op_snake:ident , $str:expr ,
                    $length:expr , $nuses:expr , $ndefs:expr , $format:expr
                ) ,
            )* ] ) => {
                match self {
                    $( Opcode::$op => $format , )*
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
