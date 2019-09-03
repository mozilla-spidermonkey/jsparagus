#!/usr/bin/env python3

import json
import os
import re
import subprocess


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


def case_name(ident):
    return ident\
        .replace("<'a>", "")\
        .replace("Box<", "")\
        .replace("Option<", "")\
        .replace("Vec<", "Vec")\
        .replace(">", "")


def collect_types(ast):
    types = set()
    for name, contents in ast.items():
        types.add(name)
    types.add('Token<\'a>')
    types.add('Vec<SwitchCase>')
    types.add('Vec<Statement>')
    types.add('Vec<VariableDeclarator>')
    types.add('Vec<ArrayExpressionElement>')
    types.add('Vec<ClassElement>')
    types.add('Vec<BindingProperty>')
    types.add('Vec<Option<Parameter>>')
    return sorted(types)


def stack_value(ast):
    types = collect_types(ast)
    with open("../generated_parser/src/stack_value.rs", "w+") as f:
        def write(*args):
            write_impl(f, *args)
        write(0, "// WARNING: This file is auto-generated.")
        write(0, "")
        write(0, "use crate::Token;")
        write(0, "use ast::*;")
        write(0, "")
        write(0, "#[derive(Debug, PartialEq)]")
        write(0, "pub enum StackValue<'a> {")
        for name in types:
            write(1, "{}(Box<{}>),", case_name(name), name)
        write(0, "}")
        write(0, "")
        write(0, "impl<'a> StackValue<'a> {")
        write(1, "pub fn to_ast<T: StackValueItem<'a>>(self) -> Box<T> {")
        write(2, "T::to_ast(self)")
        write(1, "}")
        write(0, "}")
        write(0, "")
        write(0, "pub trait StackValueItem<'a> {")
        write(1, "fn to_ast(sv: StackValue<'a>) -> Box<Self>;")
        write(0, "}")
        write(0, "")
        for name in types:
            write(0, "impl<'a> StackValueItem<'a> for {} {{", name)
            write(1, "fn to_ast(sv: StackValue<'a>) -> Box<Self> {")
            write(2, "match sv {")
            write(3, "StackValue::{}(v) => v,", case_name(name))
            write(3, "_ => panic!(\"StackValue expected {}, got {{:?}}\", sv),", name)
            write(2, "}")
            write(1, "}")
            write(0, "}")
            write(0, "")
        for name in types:
            if name == "Token<'a>":
                # more special hacks
                lifetime = "a"
                impl = "<'a>"
            else:
                lifetime = "static"
                impl = ""
            write(0, "impl{} From<Box<{}>> for StackValue<'{}> {{", impl, name, lifetime)
            write(1, "fn from(val: Box<{}>) -> StackValue<'{}> {{", name, lifetime)
            write(2, "StackValue::{}(val)".format(case_name(name)))
            write(1, "}")
            write(0, "}")
            write(0, "")


def pass_(ast):
    with open("../emitter/src/lower/pass.rs", "w+") as f:
        def write(*args):
            write_impl(f, *args)

        def to_method_name(name):
            return "visit_{}".format(to_snek_case(case_name(name)))

        # --- Pass ---

        def emit_call(indent, ty, var):
            if ty.startswith("Vec<") and ty.endswith(">"):
                write(indent, "for item in {} {{", var)
                emit_call(indent + 1, ty[4:-1], "item")
                write(indent, "}")
            elif ty.startswith("Option<") and ty.endswith(">"):
                write(indent, "if let Some(item) = {} {{", var)
                emit_call(indent + 1, ty[7:-1], "item")
                write(indent, "}")
            elif ty == "bool" or ty == "String" or ty == "f64":
                pass
            else:
                write(indent, "self.{}({});", to_method_name(ty), var)

        write(0, "// WARNING: This file is auto-generated.")
        write(0, "")
        write(0, "#![allow(unused_mut)]")
        write(0, "#![allow(unused_parens)]")
        write(0, "#![allow(unused_variables)]")
        write(0, "")
        write(0, "use ast::*;")
        write(0, "")
        write(0, "pub trait Pass {")
        for name, contents in ast.items():
            if name == "Void":
                # Hack in a quick fix
                continue
            _type = contents["_type"]
            write(1, "fn {}(&mut self, ast: &mut {}) {{", to_method_name(name), name)
            if _type == "struct":
                for field, field_type in contents.items():
                    if field != "_type":
                        emit_call(2, field_type, "&mut ast.{}".format(field))
            elif _type == "enum":
                write(2, "match ast {")
                for field, field_type in contents.items():
                    if field != "_type":
                        if field_type is None:
                            write(3, "{}::{} => (),", name, field)
                        else:
                            write(3, "{}::{}(ast) => {{", name, field)
                            emit_call(4, field_type, "ast")
                            write(3, "}")
                write(2, "}")
            else:
                raise Exception("Invalid type: " + _type)
            write(1, "}")
            write(0, "")
        write(0, "}")
        write(0, "")

        # --- PostfixPass ---

        def to_postfix_type(ty):
            if ty.startswith("Vec<") and ty.endswith(">"):
                res = to_postfix_type(ty[4:-1])
                return None if res is None else "Vec<{}>".format(res)
            elif ty.startswith("Option<") and ty.endswith(">"):
                res = to_postfix_type(ty[7:-1])
                return None if res is None else "Option<{}>".format(res)
            elif ty == "bool" or ty == "String" or ty == "f64":
                return None
            else:
                return "Self::Value"

        def append_postfix_pass(indent, ty, var):
            if ty.startswith("Vec<") and ty.endswith(">"):
                write(indent, "for item in {} {{", var)
                append_postfix_pass(indent + 1, ty[4:-1], "item")
                write(indent, "}")
            elif ty.startswith("Option<") and ty.endswith(">"):
                write(indent, "if let Some(item) = {} {{", var)
                append_postfix_pass(indent + 1, ty[7:-1], "item")
                write(indent, "}")
            elif ty == "bool" or ty == "String" or ty == "f64":
                pass
            else:
                write(indent, "result.append({});", var)

        write(0, "pub trait PostfixPassMonoid: Default {")
        write(1, "fn append(&mut self, other: Self);")
        write(0, "}")
        write(0, "")
        write(0, "pub trait PostfixPass {")
        write(1, "type Value: PostfixPassMonoid;")
        for name, contents in ast.items():
            if name == "Void":
                continue
            _type = contents["_type"]
            if _type == "struct":
                write(1, "fn {}(&self,", to_method_name(name))
                for field, field_type in contents.items():
                    if field != "_type":
                        postfix_type = to_postfix_type(field_type)
                        if postfix_type is None:
                            postfix_type = "&mut {}".format(field_type)
                        write(2, "{}: {},", field, postfix_type)
                write(1, ") -> Self::Value {")
                write(2, "let mut result = Self::Value::default();")
                for field, field_type in contents.items():
                    if field != "_type":
                        append_postfix_pass(2, field_type, field)
                write(2, "result")
                write(1, "}")
                write(0, "")
        write(0, "}")
        write(0, "")

        # --- PostfixPassVisitor ---
        def call_postfix_visitor(ty, var):
            if ty.startswith("Vec<") and ty.endswith(">"):
                res = call_postfix_visitor(ty[4:-1], "item")
                return None if res is None else "{}.iter_mut().map(|item| {}).collect::<Vec<_>>()".format(var, res)
            elif ty.startswith("Option<") and ty.endswith(">"):
                res = call_postfix_visitor(ty[7:-1], "item")
                return None if res is None else "{}.as_mut().map(|item| {})".format(var, res)
            elif ty == "bool" or ty == "String" or ty == "f64":
                return None
            else:
                return "self.{}({})".format(to_method_name(ty), var)

        write(0, "pub struct PostfixPassVisitor<T: PostfixPass> {")
        write(1, "pass: T,")
        write(0, "}")
        write(0, "")
        write(0, "impl<T: PostfixPass> PostfixPassVisitor<T> {")
        write(1, "pub fn new(pass: T) -> Self {")
        write(2, "Self { pass }")
        write(1, "}")
        for name, contents in ast.items():
            if name == "Void":
                # Hack in a quick fix
                continue
            _type = contents["_type"]
            write(1, "pub fn {}(&mut self, ast: &mut {}) -> T::Value {{", to_method_name(name), name)
            if _type == "struct":
                index = 0
                for field, field_type in contents.items():
                    if field != "_type":
                        res = call_postfix_visitor(field_type, "(&mut ast.{})".format(field))
                        if res is None:
                            res = "&mut ast.{}".format(field)
                        write(2, "let a{} = {};", index, res)
                        index += 1
                write(2, "self.pass.{}({})", to_method_name(name),
                      ", ".join("a{}".format(i) for i in range(index)))
            elif _type == "enum":
                write(2, "match ast {")
                for field, field_type in contents.items():
                    if field != "_type":
                        if field_type is None:
                            write(3, "{}::{} => T::Value::default(),", name, field)
                        else:
                            write(3, "{}::{}(ast) => self.{}(ast),",
                                  name, field, to_method_name(field_type))
                write(2, "}")
            else:
                raise Exception("Invalid type: " + _type)
            write(1, "}")
            write(0, "")
        write(0, "}")
        write(0, "")


def ast_(ast):
    with open("src/lib.rs", "w+") as f:
        def write(*args):
            write_impl(f, *args)
        write(0, "// WARNING: This file is auto-generated.")
        write(0, "")
        write(0, "pub mod json;")
        write(0, "")
        write(0, "use serde::{Deserialize, Serialize};")
        write(0, "")
        for name, contents in ast.items():
            _type = contents["_type"]
            if _type == "struct":
                if len(contents) <= 1:
                    write(0, "#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]")
                else:
                    write(0, "#[derive(Debug, PartialEq, Serialize, Deserialize)]")
                write(0, "pub struct {} {{", name)
                for field, field_type in contents.items():
                    if field != "_type":
                        write(1, "pub {}: {},", field, field_type)
                write(0, "}")
                write(0, "")
                write(0, "impl {} {{", name)
                write(1, "pub fn new(")
                for field, field_type in contents.items():
                    if field != "_type":
                        write(1, "{}: {},", field, field_type)
                write(1, ") -> Self {")
                write(2, "Self {")
                for field, field_type in contents.items():
                    if field != "_type":
                        write(3, "{},", field)
                write(2, "}")
                write(1, "}")
                write(0, "}")
                write(0, "")
            elif _type == "enum":
                write(0, "#[derive(Debug, PartialEq, Serialize, Deserialize)]")
                write(0, "pub enum {} {{", name)
                for field, field_type in contents.items():
                    if field != "_type":
                        if field_type is None:
                            write(1, "{},", field)
                        else:
                            write(1, "{}({}),", field, field_type)
                write(0, "}")
                write(0, "")
            else:
                raise Exception("Invalid type: " + _type)


def main():
    with open("ast.json", "r") as json_file:
        ast = json.load(json_file)
    ast_(ast)
    stack_value(ast)
    pass_(ast)
    subprocess.run(['cargo', 'fmt'], cwd="..")


if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    main()
