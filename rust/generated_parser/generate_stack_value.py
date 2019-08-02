#!/usr/bin/env python3

import re
import subprocess


def to_snek_case(ident):
    # https://stackoverflow.com/questions/1175208
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', ident)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


def case_name(ident):
    return ident.replace("<", "").replace(">", "")


def main():
    rg_process = subprocess.run(['rg', r'^pub (enum|struct) (\w+).*', '../ast/src/lib.rs',
                                 "--replace=$2"], stdout=subprocess.PIPE)
    names = rg_process.stdout.decode('utf-8').splitlines()
    names.append('Token')
    names.append('Vec<SwitchCase>')
    names.append('Vec<Statement>')
    names.append('Vec<VariableDeclarator>')
    names.append('Vec<ArrayExpressionElement>')
    with open("src/stack_value.rs", "w+") as f:
        f.write("use crate::Token;\n")
        f.write("use ast::*;\n")
        f.write("\n")
        f.write("#[derive(Debug, PartialEq)]\n")
        f.write("pub enum StackValue {\n")
        for name in names:
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
        for name in names:
            f.write("impl StackValueItem for {} {{\n".format(name))
            f.write("    fn to_ast(sv: StackValue) -> Box<Self> {\n")
            f.write("        match sv {\n")
            f.write("            StackValue::{}(v) => v,\n".format(case_name(name)))
            f.write("            _ => panic!(\"StackValue expected {}, got {{:?}}\", sv),\n".format(name))
            f.write("        }\n")
            f.write("    }\n")
            f.write("}\n")
            f.write("\n")
        for name in names:
            f.write("impl From<Box<{}>> for StackValue {{\n".format(name))
            f.write("    fn from(val: Box<{}>) -> StackValue {{\n".format(name))
            f.write("        StackValue::{}(val)\n".format(case_name(name)))
            f.write("    }\n")
            f.write("}\n")
            f.write("\n")


if __name__ == "__main__":
    main()
