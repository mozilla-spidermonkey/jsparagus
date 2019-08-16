#!/usr/bin/env python3

import re
import subprocess
import json


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
    return ident.replace("Box<", "").replace("Option<", "").replace("Vec<", "Vec").replace(">", "")


def collect_types(ast):
    types = set()
    for name, contents in ast.items():
        types.add(name)
    types.add('Token')
    types.add('Vec<SwitchCase>')
    types.add('Vec<Statement>')
    types.add('Vec<VariableDeclarator>')
    types.add('Vec<ArrayExpressionElement>')
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
        write(0, "pub enum StackValue {")
        for name in types:
            write(1, "{}(Box<{}>),", case_name(name), name)
        write(0, "}")
        write(0, "")
        write(0, "impl StackValue {")
        write(1, "pub fn to_ast<T: StackValueItem>(self) -> Box<T> {")
        write(2, "T::to_ast(self)")
        write(1, "}")
        write(0, "}")
        write(0, "")
        write(0, "pub trait StackValueItem {")
        write(0, "    fn to_ast(sv: StackValue) -> Box<Self>;")
        write(0, "}")
        write(0, "")
        for name in types:
            write(0, "impl StackValueItem for {} {{", name)
            write(1, "fn to_ast(sv: StackValue) -> Box<Self> {")
            write(2, "match sv {")
            write(3, "StackValue::{}(v) => v,", case_name(name))
            write(3, "_ => panic!(\"StackValue expected {}, got {{:?}}\", sv),", name)
            write(2, "}")
            write(1, "}")
            write(0, "}")
            write(0, "")
        for name in types:
            write(0, "impl From<Box<{}>> for StackValue {{".format(name))
            write(1, "fn from(val: Box<{}>) -> StackValue {{".format(name))
            write(2, "StackValue::{}(val)".format(case_name(name)))
            write(1, "}")
            write(0, "}")
            write(0, "")


def pass_(ast):
    def to_method_name(name):
        return "visit_{}".format(to_snek_case(case_name(name)))

    def emit_call(f, indent, ty, var):
        def write(*args):
            write_impl(f, *args)
        if ty.startswith("Vec<") and ty.endswith(">"):
            write(indent, "for item in {} {{", var)
            emit_call(f, indent + 1, ty[4:-1], "item")
            write(indent, "}")
        elif ty.startswith("Option<") and ty.endswith(">"):
            write(indent, "if let Some(item) = {} {{", var)
            emit_call(f, indent + 1, ty[7:-1], "item")
            write(indent, "}")
        elif ty == "bool" or ty == "String" or ty == "f64":
            pass
        else:
            write(indent, "self.{}({});", to_method_name(ty), var)

    with open("../emitter/src/lower/pass.rs", "w+") as f:
        def write(*args):
            write_impl(f, *args)
        write(0, "// WARNING: This file is auto-generated.")
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
                        emit_call(f, 2, field_type, "&mut ast.{}".format(field))
            elif _type == "enum":
                write(2, "match ast {")
                for field, field_type in contents.items():
                    if field != "_type":
                        if field_type is None:
                            write(3, "{}::{} => (),", name, field)
                        else:
                            write(3, "{}::{}(ast) => {{", name, field)
                            emit_call(f, 4, field_type, "ast")
                            write(3, "}")
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
        for name, contents in ast.items():
            _type = contents["_type"]
            if _type == "struct":
                if len(contents) <= 1:
                    write(0, "#[derive(Default, Debug, PartialEq)]")
                else:
                    write(0, "#[derive(Debug, PartialEq)]")
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
                write(0, "#[derive(Debug, PartialEq)]")
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
    main()
