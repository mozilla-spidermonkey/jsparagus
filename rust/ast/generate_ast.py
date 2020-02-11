#!/usr/bin/env python3

import json
import os
import re
import subprocess
import collections


RUST_BUILTIN_TYPES = {
    'bool',
    'f64',
    'String',
}

RUST_PARAMETERIZED_TYPES = {
    'Option',
    'Vec',
}


# name is a string; params is a tuple of 0 or more Types.
TypeBase = collections.namedtuple("Type", "name params")


class Type(TypeBase):
    def __new__(cls, name, params=()):
        params = tuple(params)
        for p in params:
            if not isinstance(p, Type):
                raise ValueError("type parameters must be types, got {!r}".format(p))
        return TypeBase.__new__(cls, name, params)

    def __str__(self):
        if self.params == ():
            return self.name
        return self.name + "<{}>".format(", ".join(map(str, self.params)))

    def __repr__(self):
        if self.params == ():
            return 'Type({!r})'.format(self.name)
        return 'Type({!r}, {!r})'.format(self.name, list(self.params))

    def to_rust_type(self, ast):
        params_str = ", ".join(p.to_rust_type(ast) for p in self.params)
        if self.name == 'String':
            return "&'alloc str"
        if self.name == 'Option':
            return "Option<{}>".format(params_str)
        if self.name == 'Box':
            return "arena::Box<'alloc, {}>".format(params_str)
        if self.name == 'Vec':
            return "arena::Vec<'alloc, {}>".format(params_str)
        if self.name in RUST_PARAMETERIZED_TYPES:
            return "{}<{}>".format(self.name, params_str)
        if self.params:
            return "{}<'alloc, {}>".format(self.name, params_str)
        if self.name in RUST_BUILTIN_TYPES:
            return self.name
        if self.name == 'Token':
            return "Token<'alloc>"
        if self.name in ast.type_decls and ast.type_decls[self.name].has_lifetime:
            return "{}<'alloc>".format(self.name)
        return self.name

    def rust_variant_name(self):
        if self.name == 'Vec':
            return 'Vec' + self.params[0].rust_variant_name()
        if self.name == 'Box':
            return self.params[0].rust_variant_name()
        return self.name


def parse_type(ty):
    """Parse a type, in the minilanguage used by ast.json, into a Type object.

    A simple type like String parses as `Type("String", ())`; a parameterized type
    like `Vec<String>` parses as `Type("Vec", ("String",))`;
    nested parameterized types parse as nested Type objects.
    """
    ident_re = re.compile(r'^(?:\w|_)+$')
    token_re = re.compile(r'(?s)\s*((?:\w|_)+|.)\s*')
    tokens = token_re.finditer(ty)

    current = None

    def consume(token=None):
        nonlocal current
        assert token is None or token == current
        current = next(tokens, None)
        if current is not None:
            current = current.group(1)

    consume(None)  # load the first token into `current`

    def is_ident():
        """True if the current token is an identifier"""
        return current is not None and ident_re.match(current) is not None

    def parse_params():
        params = []
        while current != '>':
            params.append(parse_ty())
            if current == ',':
                consume(',')
        return params

    def parse_ty():
        if not is_ident():
            raise ValueError("parse error in type {!r}".format(ty))
        name = current
        consume()
        if current == '<':
            consume('<')
            params = parse_params()
            if current != '>':
                raise ValueError("parse error in type {!r} (expected `>`)".format(ty))
            consume('>')
            return Type(name, params)
        return Type(name)

    result = parse_ty()
    if current is not None:
        raise ValueError("parse error in type {!r} (extra stuff at end)".format(ty))
    return result


assert parse_type('Statement') == Type('Statement')
assert parse_type('Box<T>') == Type('Box', [Type('T')])
assert parse_type("Vec<Box<Expression>>") == Type('Vec', [Type('Box', [Type('Expression')])])


def write_impl(f, indentation, string, *format_args):
    if len(format_args) == 0:
        formatted = string
    else:
        formatted = string.format(*format_args)
    f.write("    " * indentation + formatted + "\n")


def to_snek_case(ident):
    # https://stackoverflow.com/questions/1175208
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', ident)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


EXTRA_STACK_VALUE_TYPE_NAMES = [
    "Token",
    "Vec<SwitchCase>",
    "Vec<Statement>",
    "Vec<VariableDeclarator>",
    "Vec<ArrayExpressionElement>",
    "Vec<Box<ClassElement>>",
    "Vec<BindingProperty>",
    "Vec<Option<Parameter>>",
]


def collect_stack_value_types(ast):
    types = {}
    for name, type_decl in ast.type_decls.items():
        ty = parse_type(name)
        if ty in types:
            raise ValueError("type occurs twice with different spellings: {!r} and {!r}"
                             .format(name, types[ty]))
        types[ty] = name

    types = set(types.keys())
    for name in EXTRA_STACK_VALUE_TYPE_NAMES:
        types.add(parse_type(name))

    return sorted(types)


def stack_value(ast):
    types = collect_stack_value_types(ast)
    with open("../generated_parser/src/stack_value_generated.rs", "w+") as f:
        def write(*args):
            write_impl(f, *args)
        write(0, "// WARNING: This file is auto-generated by rust/ast/generate_ast.py.")
        write(0, "")
        write(0, "use crate::token::Token;")
        write(0, "use ast::arena;")
        write(0, "use ast::types::*;")
        write(0, "use std::convert::Infallible;")
        write(0, "")
        write(0, "pub type AstError = String;")
        write(0, "type AstResult<'alloc, T> = Result<arena::Box<'alloc, T>, AstError>;")
        write(0, "")
        write(0, "#[derive(Debug)]")
        write(0, "pub enum StackValue<'alloc> {")
        for ty in types:
            write(1, "{}({}),", ty.rust_variant_name(), Type('Box', [ty]).to_rust_type(ast))
        write(0, "}")
        write(0, "")
        write(0, "impl<'alloc> StackValue<'alloc> {")
        write(1, "pub fn to_ast<T: StackValueItem<'alloc>>(self) -> AstResult<'alloc, T> {")
        write(2, "T::to_ast(self)")
        write(1, "}")
        write(0, "}")
        write(0, "")
        write(0, "pub trait StackValueItem<'alloc>: Sized {")
        write(1, "fn to_ast(sv: StackValue<'alloc>) -> AstResult<'alloc, Self>;")
        write(0, "}")
        write(0, "")
        write(0, "/// Values that can be converted to StackValues, fallibly.")
        write(0, "pub trait TryIntoStack<'alloc> {")
        write(1, "type Error;")
        write(1, "fn try_into_stack(self) -> Result<StackValue<'alloc>, Self::Error>;")
        write(0, "}")
        write(0, "")
        for ty in types:
            write(0, "impl<'alloc> StackValueItem<'alloc> for {} {{", ty.to_rust_type(ast))
            write(1, "fn to_ast(sv: StackValue<'alloc>) -> AstResult<'alloc, Self> {")
            write(2, "match sv {")
            write(3, "StackValue::{}(v) => Ok(v),", ty.rust_variant_name())
            write(3, "_ => Err(format!(\"StackValue expected {}, got {{:?}}\", sv)),", ty)
            write(2, "}")
            write(1, "}")
            write(0, "}")
            write(0, "")
        for ty in types:
            rust_ty = ty.to_rust_type(ast)
            write(0, "impl<'alloc> TryIntoStack<'alloc> for arena::Box<'alloc, {}> {{", rust_ty)
            write(1, "type Error = Infallible;")
            write(1, "fn try_into_stack(self) -> Result<StackValue<'alloc>, Infallible> {{", rust_ty)
            write(2, "Ok(StackValue::{}(self))".format(ty.rust_variant_name()))
            write(1, "}")
            write(0, "}")
            write(0, "")
        write(0, "impl<'alloc, T, E> TryIntoStack<'alloc> for Result<T, E>")
        write(0, "where")
        write(1, "T: TryIntoStack<'alloc>,")
        write(1, "E: From<T::Error>,")
        write(0, "{")
        write(1, "type Error = E;")
        write(1, "fn try_into_stack(self) -> Result<StackValue<'alloc>, E> {")
        write(2, "Ok(self?.try_into_stack()?)")
        write(1, "}")
        write(0, "}")


def loc_trait(ast):
    types = collect_stack_value_types(ast)
    with open("src/source_location_accessor_generated.rs", "w+") as f:
        def write(*args):
            write_impl(f, *args)
        write(0, "// WARNING: This file is auto-generated by rust/ast/generate_ast.py.")
        write(0, "")
        write(0, "use crate::SourceLocation;")
        write(0, "use crate::arena;")
        write(0, "use crate::types::*;")
        write(0, "use std::borrow::{Borrow, BorrowMut};")
        write(0, "")
        write(0, "pub trait SourceLocationAccessor {")
        write(1, "fn set_loc(&mut self, start: SourceLocation, end: SourceLocation);")
        write(1, "fn get_loc(&self) -> SourceLocation;")
        write(0, "}")
        write(0, "")

        extra_types = []


        def define_accessor(ty):
            if ty.name in ['Box', 'Token', 'Vec', 'Void']:
                return

            if ty.name not in ast.type_decls:
                raise ValueError("unhandlable type {!r}".format(ty.name))

            rust_ty = ty.to_rust_type(ast)
            decl = ast.type_decls[ty.name]

            write(0, "impl<'alloc> SourceLocationAccessor for {} {{", rust_ty)
            if isinstance(decl, Struct):
                write(1, "fn set_loc(&mut self, start: SourceLocation, end: SourceLocation) {")
                write(2, "self.loc.start = start.start;")
                write(2, "self.loc.end = end.end;")
                write(1, "}")
                write(0, "")
                write(1, "fn get_loc(&self) -> SourceLocation {")
                write(2, "self.loc")
                write(1, "}")
            elif isinstance(decl, Enum):
                write(1, "fn set_loc(&mut self, start: SourceLocation, end: SourceLocation) {")
                write(2, "match self {")
                for variant_name, variant_ty in decl.variants.items():
                    if variant_ty is None:
                        write(3, "{}::{} {{ mut loc }} => {{", ty.name, variant_name)
                        write(4, "loc.start = start.start;")
                        write(4, "loc.end = end.end;")
                        write(3, "}}", ty.name, variant_name)
                    elif isinstance(variant_ty, dict):
                        write(3, "{}::{} {{ mut loc, .. }} => {{", ty.name, variant_name)
                        write(4, "loc.start = start.start;")
                        write(4, "loc.end = end.end;")
                        write(3, "}")
                    else:
                        write(3, "{}::{}(content) => {{ content.set_loc(start, end) }}", ty.name, variant_name)
                        if variant_ty not in extra_types and variant_ty not in types:
                            extra_types.append(variant_ty)
                write(2, "}")
                write(1, "}")
                write(0, "")
                write(1, "fn get_loc(&self) -> SourceLocation {")
                write(2, "match self {")
                for variant_name, variant_ty in decl.variants.items():
                    if variant_ty is None:
                        write(3, "{}::{} {{ loc }} => {{", ty.name, variant_name)
                        write(4, "*loc")
                        write(3, "}}", ty.name, variant_name)
                    elif isinstance(variant_ty, dict):
                        write(3, "{}::{} {{ loc, .. }} => {{", ty.name, variant_name)
                        write(4, "*loc")
                        write(3, "}")
                    else:
                        write(3, "{}::{}(content) => {{ content.get_loc() }}", ty.name, variant_name)
                write(2, "}")
                write(1, "}")
            else:
                raise ValueError("unhandlable type {!r}".format(types[ty]))

            write(0, "}")
            write(0, "")

        for ty in types:
            define_accessor(ty)

        for ty in extra_types:
            define_accessor(ty)

        write(0, "impl<'alloc, T> SourceLocationAccessor for arena::Box<'alloc, T>")
        write(0, "where")
        write(1, "T: SourceLocationAccessor,")
        write(0, "{")
        write(1, "fn set_loc(&mut self, start: SourceLocation, end: SourceLocation) {")
        write(2, "(self.borrow_mut() as &mut T).set_loc(start, end)")
        write(1, "}")
        write(0, "")
        write(1, "fn get_loc(&self) -> SourceLocation {")
        write(2, "(self.borrow() as &T).get_loc()")
        write(1, "}")
        write(0, "}")


def pass_(ast):
    with open("src/visit_generated.rs", "w+") as f:
        def write(*args):
            write_impl(f, *args)

        def to_method_name(name):
            return "visit_{}".format(to_snek_case(name))

        def to_enum_method_name(enum_name, variant_name):
            return "visit_enum_{}_variant_{}".format(to_snek_case(enum_name),
                                                     to_snek_case(variant_name))

        def to_enter_method_name(name):
            return "enter_{}".format(to_snek_case(name))

        def to_enter_enum_method_name(enum_name, variant_name):
            return "enter_enum_{}_variant_{}".format(to_snek_case(enum_name),
                                                     to_snek_case(variant_name))

        def to_leave_method_name(name):
            return "leave_{}".format(to_snek_case(name))

        def to_leave_enum_method_name(enum_name, variant_name):
            return "leave_enum_{}_variant_{}".format(to_snek_case(enum_name),
                                                     to_snek_case(variant_name))

        # --- Pass ---

        def emit_call(indent, ty, var):
            if ty.name == 'Vec':
                write(indent, "for item in {} {{", var)
                emit_call(indent + 1, ty.params[0], "item")
                write(indent, "}")
            elif ty.name == 'Option':
                write(indent, "if let Some(item) = {} {{", var)
                emit_call(indent + 1, ty.params[0], "item")
                write(indent, "}")
            elif ty.name in RUST_BUILTIN_TYPES:
                pass
            elif ty.name == 'Box':
                write(indent, "self.{}({});", to_method_name(ty.params[0].name), var)
            else:
                write(indent, "self.{}({});", to_method_name(ty.name), var)

        def emit_variant_dict_call(indent, enum_name, variant_name,
                                   variant_type):
            write(indent, "self.{}(",
                  to_enum_method_name(enum_name, variant_name))
            for field_name, field_ty in variant_type.items():
                write(indent + 1, "{},", field_name)
            write(indent, ")")

        def emit_variant_tuple_call(indent, enum_name, variant_name, param):
            write(indent, "self.{}({})",
                  to_enum_method_name(enum_name, variant_name),
                  param)

        def emit_variant_none_call(indent, enum_name, variant_name):
            write(indent, "self.{}()",
                  to_enum_method_name(enum_name, variant_name))

        write(0, "// WARNING: This file is auto-generated.")
        write(0, "")
        write(0, "#![allow(unused_mut)]")
        write(0, "#![allow(unused_parens)]")
        write(0, "#![allow(unused_variables)]")
        write(0, "#![allow(dead_code)]")
        write(0, "")
        write(0, "use crate::arena;")
        write(0, "use crate::types::*;")
        write(0, "")
        write(0, "pub trait Pass<'alloc> {")
        for name, type_decl in ast.type_decls.items():
            if name == "Void":
                # Hack in a quick fix
                continue

            write(1, "fn {}(&mut self, ast: &mut {}) {{",
                  to_method_name(type_decl.name), Type(name).to_rust_type(ast))

            write(2, "self.{}(ast);",
                  to_enter_method_name(type_decl.name))

            type_decl.write_rust_pass_method_body(write, emit_call,
                                                  emit_variant_dict_call,
                                                  emit_variant_tuple_call,
                                                  emit_variant_none_call)

            write(2, "self.{}(ast);",
                  to_leave_method_name(type_decl.name))

            write(1, "}")
            write(0, "")

            write(1, "fn {}(&mut self, ast: &mut {}) {{",
                  to_enter_method_name(type_decl.name),
                  Type(name).to_rust_type(ast))
            write(1, "}")
            write(0, "")

            write(1, "fn {}(&mut self, ast: &mut {}) {{",
                  to_leave_method_name(type_decl.name),
                  Type(name).to_rust_type(ast))
            write(1, "}")
            write(0, "")

            if isinstance(type_decl, Enum):
                for variant_name, variant_type in type_decl.variants.items():
                    if variant_type is None:
                        write(1, "fn {}(&mut self) {{",
                              to_enum_method_name(type_decl.name,
                                                  variant_name)),
                        write(1, "}")
                        write(0, "")
                    elif isinstance(variant_type, dict):
                        def write_field_params(indent, write, variant_type,
                                               ast):
                            for field_name, field_ty in variant_type.items():
                                write(2, "{}: &mut {},",
                                      field_name, field_ty.to_rust_type(ast))

                        write(1, "fn {}(",
                              to_enum_method_name(type_decl.name,
                                                  variant_name)),
                        write(2, "&mut self,")
                        write_field_params(2, write, variant_type, ast)
                        write(1, ") {")

                        write(2, "self.{}(",
                              to_enter_enum_method_name(type_decl.name,
                                                        variant_name))
                        for field_name, field_ty in variant_type.items():
                            write(3 , "{},", field_name)
                        write(2, ");")

                        type_decl.write_rust_pass_variant_dict_method_body(
                            emit_call, variant_type)

                        write(2, "self.{}(",
                              to_leave_enum_method_name(type_decl.name,
                                                        variant_name))
                        for field_name, field_ty in variant_type.items():
                            write(3, "{},", field_name)
                        write(2, ");")

                        write(1, "}")
                        write(0, "")

                        write(1, "fn {}(",
                              to_enter_enum_method_name(type_decl.name,
                                                        variant_name))
                        write(2, "&mut self,")
                        write_field_params(2, write, variant_type, ast)
                        write(1, ") {")
                        write(1, "}")
                        write(0, "")

                        write(1, "fn {}(",
                              to_leave_enum_method_name(type_decl.name,
                                                        variant_name))
                        write(2, "&mut self,")
                        write_field_params(2, write, variant_type, ast)
                        write(1, ") {")
                        write(1, "}")
                        write(0, "")
                    else:
                        def write_field_params(indent, write, variant_type,
                                               ast):
                            write(2, "ast: &mut {},",
                                  variant_type.to_rust_type(ast))

                        write(1, "fn {}(",
                              to_enum_method_name(type_decl.name,
                                                  variant_name)),
                        write(2, "&mut self,")
                        write_field_params(2, write, variant_type, ast)
                        write(1, ") {")

                        write(2, "self.{}(ast);",
                              to_enter_enum_method_name(type_decl.name,
                                                        variant_name))

                        type_decl.write_rust_pass_variant_tuple_method_body(
                            emit_call, variant_type, "ast")

                        write(2, "self.{}(ast);",
                              to_leave_enum_method_name(type_decl.name,
                                                        variant_name))

                        write(1, "}")
                        write(0, "")

                        write(1, "fn {}(",
                              to_enter_enum_method_name(type_decl.name,
                                                        variant_name))
                        write(2, "&mut self,")
                        write_field_params(2, write, variant_type, ast)
                        write(1, ") {")
                        write(1, "}")
                        write(0, "")

                        write(1, "fn {}(",
                              to_leave_enum_method_name(type_decl.name,
                                                        variant_name))
                        write(2, "&mut self,")
                        write_field_params(2, write, variant_type, ast)
                        write(1, ") {")
                        write(1, "}")
                        write(0, "")

        write(0, "}")
        write(0, "")


def ast_(ast):
    with open("src/types_generated.rs", "w+") as f:
        def write(*args):
            write_impl(f, *args)
        write(0, "// WARNING: This file is auto-generated.")
        write(0, "")
        write(0, "use crate::source_location::SourceLocation;")
        write(0, "use crate::arena;")
        write(0, "")
        for type_decl in ast.type_decls.values():
            type_decl.write_rust_type_decl(ast, write)


class AggregateTypeDecl:
    def __init__(self):
        self.has_lifetime = None

    def lifetime_params(self):
        if self.has_lifetime:
            return "<'alloc>"
        return ""


class Struct(AggregateTypeDecl):
    def __init__(self, name, struct_json):
        AggregateTypeDecl.__init__(self)
        self.name = name
        self.fields = {
            name: parse_type(ty)
            for name, ty in struct_json.items()
            if name != '_type'
        }

    def field_types(self):
        return iter(self.fields.values())

    def write_rust_type_decl(self, ast, write):
        if len(self.fields) == 0:
            write(0, "#[derive(Default, Debug, PartialEq)]")
        else:
            write(0, "#[derive(Debug, PartialEq)]")
        lifetime_params = self.lifetime_params()
        write(0, "pub struct {}{} {{", self.name, lifetime_params)
        for field, field_type in self.fields.items():
            write(1, "pub {}: {},", field, field_type.to_rust_type(ast))
        write(1, "pub loc: SourceLocation,")
        write(0, "}")
        write(0, "")

    def write_rust_pass_method_body(self, write, emit_call,
                                    emit_variant_dict_call,
                                    emit_variant_tuple_call,
                                    emit_variant_none_call):
        for name, ty in self.fields.items():
            emit_call(2, ty, "&mut ast.{}".format(name))


class Enum(AggregateTypeDecl):
    def __init__(self, name, enum_json):
        AggregateTypeDecl.__init__(self)

        def parse_maybe_type(ty):
            if ty is None:
                return None
            if isinstance(ty, dict):
                return {name: parse_type(field_ty) for name, field_ty in ty.items()}
            return parse_type(ty)

        self.name = name
        self.variants = {
            name: parse_maybe_type(ty)
            for name, ty in enum_json.items()
            if name != '_type'
        }
        self.has_lifetime = None

    def field_types(self):
        for var in self.variants.values():
            if isinstance(var, dict):
                yield from var.values()
            else:
                yield var

    def write_rust_type_decl(self, ast, write):
        write(0, "#[derive(Debug, PartialEq)]")
        lifetime_params = self.lifetime_params()
        write(0, "pub enum {}{} {{", self.name, lifetime_params)
        for variant_name, ty in self.variants.items():
            if ty is None:
                write(1, "{} {{", variant_name)
                write(2, "loc: SourceLocation,")
                write(1, "},")
            elif isinstance(ty, dict):
                write(1, "{} {{", variant_name)
                for field_name, field_ty in ty.items():
                    write(2, "{}: {},", field_name, field_ty.to_rust_type(ast))
                write(2, "loc: SourceLocation,")
                write(1, "},")
            else:
                write(1, "{}({}),", variant_name, ty.to_rust_type(ast))
        write(0, "}")
        write(0, "")

    def write_rust_pass_method_body(self, write, emit_call,
                                    emit_variant_dict_call,
                                    emit_variant_tuple_call,
                                    emit_variant_none_call):
        write(2, "match ast {")
        for variant_name, variant_type in self.variants.items():
            if variant_type is None:
                write(3, "{}::{} {{ .. }} => {{", self.name, variant_name)
                emit_variant_none_call(4, self.name, variant_name)
                write(3, "}")
            elif isinstance(variant_type, dict):
                write(3, "{}::{} {{ {}, .. }} => {{", self.name, variant_name, ', '.join(variant_type.keys()))
                emit_variant_dict_call(4, self.name, variant_name, variant_type);
                write(3, "}")
            else:
                write(3, "{}::{}(ast) => {{", self.name, variant_name)
                emit_variant_tuple_call(4, self.name, variant_name, "ast")
                write(3, "}")
        write(2, "}")

    def write_rust_pass_variant_dict_method_body(self, emit_call,
                                                 variant_type):
        for field_name, field_ty in variant_type.items():
            if field_ty.name == 'Vec':
                emit_call(2, field_ty, '{}.iter_mut()'.format(field_name))
            else:
                emit_call(2, field_ty, field_name)

    def write_rust_pass_variant_tuple_method_body(self, emit_call,
                                                  variant_type, param):
        emit_call(2, variant_type, param)

class Ast:
    def __init__(self, ast_json):
        self.type_decls = {}
        for name, contents in ast_json.items():
            _type = contents["_type"]
            if _type == "struct":
                t = Struct(name, contents)
            elif _type == "enum":
                t = Enum(name, contents)
            else:
                raise ValueError("unrecognized _type: " + repr(_type))
            self.type_decls[name] = t

        for name in self.type_decls:
            self._has_lifetime(name)

    def _has_lifetime(self, name):
        ty = self.type_decls[name]
        if ty.has_lifetime == "computing":
            raise ValueError("invalid AST structure: {} contains itself. Try adding a Box."
                             .format(name))
        if ty.has_lifetime is None:
            ty.has_lifetime = "computing"
            ty.has_lifetime = any(
                field_ty is not None and self.type_has_lifetime(field_ty)
                for field_ty in ty.field_types()
            )
        return ty.has_lifetime

    def type_has_lifetime(self, ty):
        return (
            ty is not None
            and (ty.name == 'Token'
                 or ty.name == 'String'
                 or ty.name == 'Box'
                 or ty.name == 'Vec'
                 or any(self.type_has_lifetime(u)
                        for u in ty.params)
                 or (ty.name in self.type_decls
                     and self._has_lifetime(ty.name))))


def main():
    with open("ast.json", "r") as json_file:
        ast_json = json.load(json_file)
    ast = Ast(ast_json)

    ast_(ast)
    stack_value(ast)
    loc_trait(ast)
    pass_(ast)


if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    main()
