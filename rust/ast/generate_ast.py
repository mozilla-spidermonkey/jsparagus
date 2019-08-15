#!/usr/bin/env python3

import re
import subprocess
import json


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
        f.write("// WARNING: This file is auto-generated.\n")
        f.write("\n")
        f.write("use crate::Token;\n")
        f.write("use ast::*;\n")
        f.write("\n")
        f.write("#[derive(Debug, PartialEq)]\n")
        f.write("pub enum StackValue {\n")
        for name in types:
            f.write("    {}(Box<{}>),\n".format(case_name(name), name))
        f.write("}\n")
        f.write("\n")
        f.write("impl StackValue {\n")
        f.write("    pub fn to_ast<T: StackValueItem>(self) -> Box<T> {\n")
        f.write("        T::to_ast(self)\n")
        f.write("    }\n")
        f.write("}\n")
        f.write("\n")
        f.write("pub trait StackValueItem {\n")
        f.write("    fn to_ast(sv: StackValue) -> Box<Self>;\n")
        f.write("}\n")
        f.write("\n")
        for name in types:
            f.write("impl StackValueItem for {} {{\n".format(name))
            f.write("    fn to_ast(sv: StackValue) -> Box<Self> {\n")
            f.write("        match sv {\n")
            f.write("            StackValue::{}(v) => v,\n".format(case_name(name)))
            f.write("            _ => panic!(\"StackValue expected {}, got {{:?}}\", sv),\n".format(name))
            f.write("        }\n")
            f.write("    }\n")
            f.write("}\n")
            f.write("\n")
        for name in types:
            f.write("impl From<Box<{}>> for StackValue {{\n".format(name))
            f.write("    fn from(val: Box<{}>) -> StackValue {{\n".format(name))
            f.write("        StackValue::{}(val)\n".format(case_name(name)))
            f.write("    }\n")
            f.write("}\n")
            f.write("\n")


def pass_(ast):
    def to_method_name(name):
        return "visit_{}".format(to_snek_case(case_name(name)))

    def emit_call(f, indent, ty, var):
        if ty.startswith("Vec<") and ty.endswith(">"):
            f.write("    " * indent + "for item in {} {{\n".format(var))
            emit_call(f, indent + 1, ty[4:-1], "item")
            f.write("    " * indent + "}\n")
        elif ty.startswith("Option<") and ty.endswith(">"):
            f.write("    " * indent + "if let Some(item) = {} {{\n".format(var))
            emit_call(f, indent + 1, ty[7:-1], "item")
            f.write("    " * indent + "}\n")
        elif ty == "bool" or ty == "String" or ty == "f64":
            pass
        else:
            f.write("    " * indent + "self.{}({});\n".format(to_method_name(ty), var))

    with open("../emitter/src/lower/pass.rs", "w+") as f:
        f.write("// WARNING: This file is auto-generated.\n")
        f.write("\n")
        f.write("use ast::*;\n")
        f.write("\n")
        f.write("pub trait Pass {\n")
        for name, contents in ast.items():
            if name == "Void":
                # Hack in a quick fix
                continue
            _type = contents["_type"]
            f.write("    fn {}(&mut self, ast: &mut {}) {{\n".format(to_method_name(name), name))
            if _type == "struct":
                for field, field_type in contents.items():
                    if field != "_type":
                        emit_call(f, 2, field_type, "&mut ast.{}".format(field))
            elif _type == "enum":
                f.write("        match ast {\n")
                for field, field_type in contents.items():
                    if field != "_type":
                        if field_type is None:
                            f.write("            {}::{} => (),\n".format(name, field))
                        else:
                            f.write("            {}::{}(ast) => {{\n".format(name, field))
                            emit_call(f, 4, field_type, "ast")
                            f.write("            }\n")
                f.write("        }\n")
            else:
                raise Exception("Invalid type: " + _type)
            f.write("    }\n")
            f.write("\n")
        f.write("}\n")
        f.write("\n")


def main():
    with open("ast.json", "r") as json_file:
        ast = json.load(json_file)
    with open("src/lib.rs", "w+") as f:
        f.write("// WARNING: This file is auto-generated.\n")
        f.write("\n")
        for name, contents in ast.items():
            _type = contents["_type"]
            if _type == "struct":
                if len(contents) <= 1:
                    f.write("#[derive(Default, Debug, PartialEq)]\n")
                else:
                    f.write("#[derive(Debug, PartialEq)]\n")
                f.write("pub struct {} {{\n".format(name))
                for field, field_type in contents.items():
                    if field != "_type":
                        f.write("    pub {}: {},\n".format(field, field_type))
                f.write("}\n")
                f.write("\n")
                f.write("impl {} {{\n".format(name))
                f.write("    pub fn new(\n")
                for field, field_type in contents.items():
                    if field != "_type":
                        f.write("    {}: {},\n".format(field, field_type))
                f.write("    ) -> Self {\n")
                f.write("        Self {\n")
                for field, field_type in contents.items():
                    if field != "_type":
                        f.write("            {},\n".format(field))
                f.write("        }\n")
                f.write("    }\n")
                f.write("}\n")
                f.write("\n")
            elif _type == "enum":
                f.write("#[derive(Debug, PartialEq)]\n")
                f.write("pub enum {} {{\n".format(name))
                for field, field_type in contents.items():
                    if field != "_type":
                        if field_type is None:
                            f.write("    {},\n".format(field))
                        else:
                            f.write("    {}({}),\n".format(field, field_type))
                f.write("}\n")
                f.write("\n")
            else:
                raise Exception("Invalid type: " + _type)
    stack_value(ast)
    pass_(ast)
    subprocess.run(['cargo', 'fmt'])


if __name__ == "__main__":
    main()
