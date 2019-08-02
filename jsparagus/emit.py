"""Emit code pelled the saor parser tables in either Python or Rust. """

import re
import unicodedata

from .runtime import (ERROR, ErrorToken)
from .ordered import OrderedSet

from .grammar import (InitNt, CallMethod, Some, is_concrete_element, Nt,
                      Optional)

from . import types


def write_python_parser(out, parser_states):
    grammar = parser_states.grammar
    states = parser_states.states
    prods = parser_states.prods
    init_state_map = parser_states.init_state_map

    out.write("from jsparagus import runtime\n")
    if any(isinstance(key, Nt) for key in grammar.nonterminals):
        out.write("from jsparagus.runtime import Nt, ErrorToken\n")
    out.write("\n")

    out.write("actions = [\n")
    for i, state in enumerate(states):
        out.write("    # {}. {}\n".format(i, state.traceback() or "<empty>"))
        # for item in state._lr_items:
        #     out.write("    #       {}\n".format(grammar.lr_item_to_str(prods, item)))
        out.write("    " + repr(state.action_row) + ",\n")
        out.write("\n")
    out.write("]\n\n")
    out.write("ctns = [\n")
    for state in states:
        row = {
            nt.pretty(): state_id
            for nt, state_id in state.ctn_row.items()
        }
        out.write("    " + repr(row) + ",\n")
    out.write("]\n\n")
    out.write("error_codes = [\n")
    SLICE_LEN = 16
    for i in range(0, len(states), SLICE_LEN):
        slice = states[i:i + SLICE_LEN]
        out.write("    {}\n".format(
            " ".join(repr(e.error_code) + "," for e in slice)))
    out.write("]\n\n")

    def action(a):
        """Compile a reduce expression to Python"""
        if isinstance(a, CallMethod):
            method_name = a.method.replace(" ", "_P")
            return "builder.{}({})".format(method_name, ', '.join(map(action, a.args)))
        elif isinstance(a, Some):
            return action(a.inner)
        elif a is None:
            return "None"
        else:
            # can't be 'accept' because we filter out InitNt productions
            assert isinstance(a, int)
            return "x{}".format(a)

    out.write("reductions = [\n")
    for prod_index, prod in enumerate(prods):
        if isinstance(prod.nt.name, InitNt):
            continue
        nparams = sum(1 for e in prod.rhs if is_concrete_element(e))
        names = ["x" + str(i) for i in range(nparams)]
        fn = ("lambda builder, "
              + ", ".join(names)
              + ": " + action(prod.action))
        out.write("    # {}. {}\n".format(
            prod_index,
            grammar.production_to_str(prod.nt, prod.rhs, prod.action)))
        out.write("    ({!r}, {!r}, {}),\n".format(prod.nt.pretty(), len(names), fn))
    out.write("]\n\n\n")  # two blank lines before class.

    out.write("class DefaultBuilder:\n")
    for tag, method_type in grammar.methods.items():
        method_name = tag.replace(' ', '_P')
        args = ", ".join("x{}".format(i)
                         for i in range(len(method_type.argument_types)))
        out.write("    def {}(self, {}): return ({!r}, {})\n"
                  .format(method_name, args, tag, args))
    out.write("\n\n")

    out.write("goal_nt_to_init_state = {\n")
    for init_nt, index in init_state_map.items():
        out.write("    {!r}: {!r},\n".format(init_nt.name, index))
    out.write("}\n\n")

    if len(init_state_map) == 1:
        init_nt = next(iter(init_state_map.keys()))
        default_goal = '=' + repr(init_nt.name)
    else:
        default_goal = ''
    out.write("class Parser(runtime.Parser):\n")
    out.write("    def __init__(self, goal{}, builder=None):\n".format(default_goal))
    out.write("        if builder is None:\n")
    out.write("            builder = DefaultBuilder()\n")
    out.write("        super().__init__(actions, ctns, reductions, error_codes,\n")
    out.write("                         goal_nt_to_init_state[goal], builder)\n")
    out.write("\n")


TERMINAL_NAMES = {
    "=>": "Arrow",
}


class RustParserWriter:
    def __init__(self, out, parser_states):
        self.out = out
        self.grammar = parser_states.grammar
        self.prods = parser_states.prods
        self.states = parser_states.states
        self.init_state_map = parser_states.init_state_map
        self.terminals = list(OrderedSet(
            t for state in self.states for t in state.action_row))
        self.nonterminals = list(OrderedSet(
            nt for state in self.states for nt in state.ctn_row))

    def emit(self):
        self.header()
        self.terminal_id()
        self.token()
        self.actions()
        self.error_codes()
        self.check_camel_case()
        # self.nt_node()
        # self.nt_node_impl()
        self.nonterminal_id()
        self.goto()
        # self.stack_value()
        self.reduce()
        self.reduce_simulator()
        self.entry()

    def write(self, indentation, string, *format_args):
        if len(format_args) == 0:
            formatted = string
        else:
            formatted = string.format(*format_args)
        self.out.write("    " * indentation + formatted + "\n")

    def header(self):
        self.write(0, "// THIS FILE IS AUTOGENERATED -- HAHAHAHA")
        self.write(0, "")
        self.write(0, "extern crate ast;")
        self.write(0, "")
        self.write(0, "mod ast_builder;")
        self.write(0, "mod stack_value;")
        self.write(0, "")
        self.write(0, "pub use ast_builder::*;")
        self.write(0, "pub use stack_value::*;")
        self.write(0, "")
        self.write(0, "const ERROR: i64 = {};", hex(ERROR))
        self.write(0, "")

    def terminal_name(self, value):
        if value is None:
            return "End"
        elif value is ErrorToken:
            return "ErrorToken"
        elif value in TERMINAL_NAMES:
            return TERMINAL_NAMES[value]
        elif value.isalpha():
            if value.islower():
                return value.capitalize()
            else:
                return value
        else:
            raw_name = " ".join((unicodedata.name(c) for c in value))
            snake_case = raw_name.replace("-", " ").replace(" ", "_").lower()
            camel_case = self.to_camel_case(snake_case)
            return camel_case

    def terminal_name_camel(self, value):
        return self.to_camel_case(self.terminal_name(value))

    def terminal_id(self):
        self.write(0, "#[derive(Copy, Clone, Debug, PartialEq)]")
        self.write(0, "pub enum TerminalId {")
        for i, t in enumerate(self.terminals):
            name = self.terminal_name(t)
            self.write(1, "{} = {}, // {}", name, i, repr(t))
        self.write(0, "}")
        self.write(0, "")

    def token(self):
        self.write(0, "#[derive(Clone, Debug, PartialEq)]")
        self.write(0, "pub struct Token {")
        self.write(1, "pub terminal_id: TerminalId,")
        self.write(1, "pub saw_newline: bool,")
        self.write(1, "pub value: Option<String>,")
        self.write(0, "}")
        self.write(0, "")

        self.write(0, "impl Token {")
        self.write(1, "pub fn basic_token(terminal_id: TerminalId) -> Self {")
        self.write(2, "Self {")
        self.write(3, "terminal_id,")
        self.write(3, "saw_newline: false,")
        self.write(3, "value: None,")
        self.write(2, "}")
        self.write(1, "}")
        self.write(0, "}")
        self.write(0, "")

    def actions(self):
        self.write(0, "static ACTIONS: [i64; {}] = [",
                   len(self.states) * len(self.terminals))
        for i, state in enumerate(self.states):
            self.write(1, "// {}. {}", i, state.traceback() or "<empty>")
            self.write(1, "{}",
                       ' '.join("{},".format(state.action_row.get(t, "ERROR")) for t in self.terminals))
            if i < len(self.states) - 1:
                self.write(0, "")
        self.write(0, "];")
        self.write(0, "")

    def error_codes(self):
        self.write(0, "#[derive(Clone, Debug, PartialEq)]")
        self.write(0, "pub enum ErrorCode {")
        for error_code in OrderedSet(s.error_code for s in self.states):
            if error_code is not None:
                self.write(1, "{},", self.to_camel_case(error_code))
        self.write(0, "}")
        self.write(0, "")

        self.write(0, "static STATE_TO_ERROR_CODE: [Option<ErrorCode>; {}] = [",
                   len(self.states))
        for i, state in enumerate(self.states):
            self.write(1, "// {}. {}", i, state.traceback() or "<empty>")
            if state.error_code is None:
                self.write(1, "None,")
            else:
                self.write(1, "Some(ErrorCode::{}),",
                           self.to_camel_case(state.error_code))
        self.write(0, "];")
        self.write(0, "")

    def nonterminal_to_snake(self, ident):
        if isinstance(ident, Nt):
            base_name = self.to_snek_case(ident.name)
            args = ''.join((("_" + self.to_snek_case(name))
                            for name, value in ident.args if value))
            return base_name + args
        else:
            assert isinstance(ident, str)
            return self.to_snek_case(ident)

    def nonterminal_to_camel(self, nt):
        return self.to_camel_case(self.nonterminal_to_snake(nt))

    def to_camel_case(self, ident):
        if '_' in ident:
            return ''.join(word.capitalize() for word in ident.split('_'))
        elif ident.islower():
            return ident.capitalize()
        else:
            return ident

    def check_camel_case(self):
        seen = {}
        for nt in self.nonterminals:
            cc = self.nonterminal_to_camel(nt)
            if cc in seen:
                raise ValueError("{} and {} have the same camel-case spelling ({})".format(
                    seen[cc], nt, cc))
            seen[cc] = nt

    def to_snek_case(self, ident):
        # https://stackoverflow.com/questions/1175208
        s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', ident)
        return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

    def method_name_to_rust(self, name):
        """Convert jsparagus's internal method name to idiomatic Rust."""
        nt_name, space, number = name.partition(' ')
        name = self.nonterminal_to_snake(nt_name)
        if space:
            name += "_p" + str(number)
        return name

    def get_associated_type_names(self):
        names = OrderedSet()

        def visit_type(ty):
            if isinstance(ty, types.NtType):
                names.add(ty.name)
            elif isinstance(ty, types.OptionType):
                visit_type(ty.t)

        for ty in self.grammar.nt_types:
            visit_type(ty)
        for method in self.grammar.methods.values():
            visit_type(method.return_type)
        return names

    def type_to_rust(self, ty, namespace, boxed=False):
        """
        Convert a jsparagus type (see types.py) to Rust.

        Pass boxed=True if the type needs to be boxed.
        """
        if ty is types.UnitType:
            return '()'
        elif ty == 'str':
            return 'String'
        elif ty == 'bool':
            return 'bool'
        elif isinstance(ty, types.NtType):
            if namespace == "":
                rty = ty.name
            else:
                rty = namespace + '::' + ty.name
            if boxed:
                return 'Box<{}>'.format(rty)
            else:
                return rty
        elif isinstance(ty, types.OptionType):
            return 'Option<{}>'.format(self.type_to_rust(ty.t, namespace, boxed))
        else:
            raise TypeError("unexpected type: {!r}".format(ty))

    def handler_trait(self):
        # NOTE: unused, code kept if we need it later
        self.write(0, "pub trait Handler {")

        for name in self.get_associated_type_names():
            self.write(1, "type {};", name)

        for tag, method in self.grammar.methods.items():
            method_name = self.method_name_to_rust(tag)
            arg_types = [
                self.type_to_rust(ty, "Self")
                for ty in method.argument_types
                if ty != types.UnitType
            ]
            if method.return_type == types.UnitType:
                return_type_tag = ''
            else:
                return_type_tag = ' -> ' + \
                    self.type_to_rust(method.return_type, "Self")

            args = ", ".join(("a{}: {}".format(i, t)
                              for i, t in enumerate(arg_types)))
            self.write(1, "fn {}(&self, {}){};",
                       method_name, args, return_type_tag)
        self.write(0, "}")
        self.write(0, "")

    def nt_node(self):
        self.write(0, "pub mod concrete {")
        for name in self.get_associated_type_names():
            self.write(0, "#[derive(Debug, PartialEq)]")
            self.write(0, "pub enum {} {{", name)
            for tag, method in self.grammar.methods.items():
                # TODO: Make this check better
                if method.return_type.name != name:
                    continue
                method_name = self.to_camel_case(self.method_name_to_rust(tag))
                arg_types = [
                    self.type_to_rust(ty, "", boxed=True)
                    for ty in method.argument_types
                    if ty != types.UnitType
                ]
                self.write(1, "{}({}),", method_name, ", ".join(arg_types))
            self.write(0, "}")
            self.write(0, "")
        self.write(0, "}")
        self.write(0, "")

    def nt_node_impl(self):
        method_to_prod = {}
        for i, prod in enumerate(self.prods):
            if prod.nt in self.nonterminals:
                def find_first_method(expr):
                    """Compile a reduce expression to Rust"""
                    if isinstance(expr, CallMethod):
                        return self.method_name_to_rust(expr.method)
                    elif isinstance(expr, Some):
                        return find_first_method
                    elif expr is None:
                        return None
                    else:
                        assert isinstance(expr, int)
                        return None

                method_name = find_first_method(prod.action)
                if method_name is not None:
                    method_to_prod[method_name] = self.grammar.production_to_str(
                        prod.nt, prod.rhs, prod.action)

        # for method in self.grammar.methods.items():
        # if prod.nt in self.nonterminals:
        #    self.write(2, "{} => {{", i)
        #    self.write(3, "// {}",

        self.write(0, "pub struct DefaultHandler {}")
        self.write(0, "")
        self.write(0, "impl DefaultHandler {")

        for tag, method in self.grammar.methods.items():
            method_name = self.method_name_to_rust(tag)
            if method_name in method_to_prod:
                prod_name = method_to_prod[method_name]
            else:
                prod_name = "<unknown production>"
            method_name_camel = self.to_camel_case(method_name)
            arg_types = [
                self.type_to_rust(ty, "concrete", boxed=True)
                for ty in method.argument_types
                if ty != types.UnitType
            ]
            if method.return_type == types.UnitType:
                return_type_tag = ''
            else:
                return_type_tag = ' -> ' + \
                    self.type_to_rust(method.return_type, "concrete", boxed=True)

            args = "".join(", a{}: {}".format(i, t)
                           for i, t in enumerate(arg_types))
            params = ", ".join("a{}".format(i)
                               for i, t in enumerate(arg_types))

            self.write(1, "// {}", prod_name)
            self.write(1, "fn {}(&self{}){} {{",
                       method_name, args, return_type_tag)
            self.write(2, "Box::new(concrete::{}::{}({}))",
                       method.return_type.name, method_name_camel, params)
            self.write(1, "}")
        self.write(0, "}")
        self.write(0, "")

    def nonterminal_id(self):
        self.write(0, "#[derive(Clone, Copy, Debug, PartialEq)]")
        self.write(0, "pub enum NonterminalId {")
        for i, nt in enumerate(self.nonterminals):
            self.write(1, "{} = {},", self.nonterminal_to_camel(nt), i)
        self.write(0, "}")
        self.write(0, "")

    def goto(self):
        self.write(0, "static GOTO: [u16; {}] = [",
                   len(self.states) * len(self.nonterminals))
        for state in self.states:
            row = state.ctn_row
            self.write(1, "{}", ' '.join("{},".format(row.get(nt, 0))
                                         for nt in self.nonterminals))
        self.write(0, "];")
        self.write(0, "")

    def element_type(self, e):
        # Mostly duplicated from types.py. :(
        if isinstance(e, str):
            if e in self.grammar.nonterminals:
                return self.grammar.nt_types[e]
            elif e in self.grammar.variable_terminals:
                return 'str'
            else:
                # constant terminal
                return types.UnitType
        elif isinstance(e, Optional):
            return types.OptionType(self.element_type(e.inner))
        elif isinstance(e, Nt):
            return self.grammar.nt_types[e.name]
        else:
            assert False, "unexpected element type: {!r}".format(e)

    def stack_value(self):
        self.write(0, "#[derive(Debug, PartialEq)]")
        self.write(0, "pub enum StackValue {")
        return_types = set((self.type_to_rust(method.return_type, ""),
                            self.type_to_rust(method.return_type, "concrete", boxed=True))
                           for tag, method in self.grammar.methods.items())
        return_types.add(("Token", "Token"))
        for return_type_plain, return_type_boxed in return_types:
            assert return_type_plain != "()"
            # This might happen, but doesn't happen in practice, so just assert
            # and deal with it later.
            assert "Option" not in return_type_plain
            self.write(0, "{}({}),", return_type_plain, return_type_boxed)
        self.write(0, "}")
        self.write(0, "")

        self.write(0, "impl StackValue {")
        for return_type_plain, return_type_boxed in return_types:
            self.write(1, "fn unwrap_{}(self) -> {} {{",
                       self.to_snek_case(return_type_plain), return_type_boxed)
            self.write(2, "match self {")
            self.write(3, "StackValue::{}(v) => v,", return_type_plain)
            self.write(3, "_ => panic!(\"StackValue expected {}, got {{:?}}\", self)", return_type_plain)
            self.write(2, "}")
            self.write(1, "}")
        self.write(0, "}")
        self.write(0, "")

        for return_type_plain, return_type_boxed in return_types:
            self.write(0, "impl From<{}> for StackValue {{", return_type_boxed)
            self.write(1, "fn from(val: {}) -> StackValue {{", return_type_boxed)
            self.write(2, "StackValue::{}(val)", return_type_plain)
            self.write(1, "}")
            self.write(0, "}")
            self.write(0, "")

    def reduce(self):
        self.write(
            0,
            "pub fn reduce(handler: &AstBuilder, prod: usize, "
            "stack: &mut Vec<StackValue>) -> NonterminalId {")
        self.write(1, "match prod {")
        for i, prod in enumerate(self.prods):
            # If prod.nt is not in nonterminals, that means it's a goal
            # nonterminal, only accepted, never reduced.
            if prod.nt in self.nonterminals:
                self.write(2, "{} => {{", i)
                self.write(3, "// {}",
                           self.grammar.production_to_str(prod.nt, prod.rhs, prod.action))

                elements = [e for e in prod.rhs if is_concrete_element(e)]
                variable_used = [False] * len(elements)
                method_used = False

                def compile_reduce_expr(expr):
                    nonlocal method_used
                    nonlocal variable_used
                    """Compile a reduce expression to Rust"""
                    if isinstance(expr, CallMethod):
                        method_used = True
                        method_type = self.grammar.methods[expr.method]
                        method_name = self.method_name_to_rust(expr.method)
                        assert len(method_type.argument_types) == len(expr.args)
                        args = ', '.join(
                            compile_reduce_expr(arg)
                            for ty, arg in zip(method_type.argument_types,
                                               expr.args)
                            if ty != types.UnitType)
                        call = "handler.{}({})".format(method_name, args)
                        return "{}".format(call)
                    elif isinstance(expr, Some):
                        return "Some({})".format(compile_reduce_expr(expr.inner))
                    elif expr is None:
                        return "None"
                    else:
                        # can't be 'accept' because we filter out InitNt productions
                        assert isinstance(expr, int)
                        variable_used[expr] = True
                        return "x{}".format(expr)

                compiled_expr = compile_reduce_expr(prod.action)

                for index, e in reversed(list(enumerate(elements))):
                    # ty = self.element_type(e)
                    # rust_ty = self.type_to_rust(ty, "")
                    if variable_used[index]:
                        if method_used:
                            self.write(3, "let x{} = stack.pop().unwrap().to_ast();", index)
                        else:
                            self.write(3, "let x{} = stack.pop().unwrap();", index)
                    else:
                        self.write(3, "stack.pop();", index)

                if method_used:
                    self.write(3, "stack.push(StackValue::from({}));", compiled_expr)
                else:
                    self.write(3, "stack.push({});", compiled_expr)
                self.write(3, "NonterminalId::{}",
                           self.nonterminal_to_camel(prod.nt))
                self.write(2, "}")
        self.write(2, '_ => panic!("no such production: {}", prod),')
        self.write(1, "}")
        self.write(0, "}")
        self.write(0, "")

    def reduce_simulator(self):
        prods = [prod for prod in self.prods if prod.nt in self.nonterminals]
        self.write(0, "static REDUCE_SIMULATOR: [(usize, NonterminalId); {}] = [", len(prods))
        for prod in prods:
            elements = [e for e in prod.rhs if is_concrete_element(e)]
            self.write(1, "({}, NonterminalId::{}),", len(elements), self.nonterminal_to_camel(prod.nt))
        self.write(0, "];")
        self.write(0, "")

    def entry(self):
        self.write(0, "#[derive(Clone, Copy)]")
        self.write(0, "pub struct ParserTables<'a> {")
        self.write(1, "pub state_count: usize,")
        self.write(1, "pub action_table: &'a [i64],")
        self.write(1, "pub action_width: usize,")
        self.write(1, "pub error_codes: &'a [Option<ErrorCode>],")
        self.write(1, "pub reduce_simulator: &'a [(usize, NonterminalId)],")
        self.write(1, "pub goto_table: &'a [u16],")
        self.write(1, "pub goto_width: usize,")
        self.write(0, "}")
        self.write(0, "")

        self.write(0, "impl<'a> ParserTables<'a> {")
        self.write(1, "pub fn check(&self) {")
        self.write(2, "assert_eq!(")
        self.write(3, "self.action_table.len(),")
        self.write(3, "(self.state_count * self.action_width) as usize")
        self.write(2, ");")
        self.write(2, "assert_eq!(self.goto_table.len(), (self.state_count * self.goto_width) as usize);")
        self.write(1, "}")
        self.write(0, "}")
        self.write(0, "")

        self.write(0, "pub static TABLES: ParserTables<'static> = ParserTables {")
        self.write(1, "state_count: {},", len(self.states))
        self.write(1, "action_table: &ACTIONS,")
        self.write(1, "action_width: {},", len(self.terminals))
        self.write(1, "error_codes: &STATE_TO_ERROR_CODE,")
        self.write(1, "reduce_simulator: &REDUCE_SIMULATOR,")
        self.write(1, "goto_table: &GOTO,")
        self.write(1, "goto_width: {},".format(len(self.nonterminals)))
        self.write(0, "};")
        self.write(0, "")

        for init_nt, index in self.init_state_map.items():
            assert init_nt.args == ()
            # result_type_jsparagus = self.grammar.nt_types[init_nt.name]
            # result_type_name = self.type_to_rust(result_type_jsparagus, "")
            # result_type_concrete = self.type_to_rust(result_type_jsparagus, "concrete", boxed=True)
            self.write(0, "pub static START_STATE_{}: usize = {};",
                       self.to_snek_case(init_nt.name).upper(), index)
            self.write(0, "")
            # self.write(0, "pub fn get_result_{}(node: StackValue) -> {} {{",
            #            self.to_snek_case(init_nt.name), result_type_concrete)
            # self.write(1, "node.to_ast()")
            # self.write(0, "}")
            # self.write(0, "")


def write_rust_parser(out, parser_states):
    RustParserWriter(out, parser_states).emit()
