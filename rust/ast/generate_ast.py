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
    subprocess.run(['cargo', 'fmt'])


if __name__ == "__main__":
    main()
