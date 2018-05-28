#!/usr/bin/env python3

"""gen.py - Second stab at a parser generator.

**Nature of a grammar.**
A grammar is a dictionary {str: [[str]]} mapping names of nonterminals to rule lists.
A rule is a nonempty list of elements.
Each element denotes either a terminal (by kind) or a nonterminal (by name).

**Context of the generated parser.**
The user passes to each method an object representing the input sequence.
This object must support two methods:

*   `src.peek()` returns the kind of the next token, or `None` at the end of input.

*   `src.take(kind)` throws an exception if `src.peek() != kind`;
    otherwise, it removes the next token from the input stream and returns it.
    The special case `src.take(None)` checks that the input stream is empty:
    if so, it returns None; if not, it throws.

**Simplifying assumptions about the grammar.**
We verify that there is no indirect left-recursion (like `a ::= b; b ::= a X`).
We verify that no production matches the empty string.
We assume that every nonterminal matches at least one string of finite length.
We verify that the grammar is unambiguous by enforcing the following two rules:

*   Non-left-recursive productions of a nonterminal must have disjoint start sets.
    That is, peeking at the next token must be sufficient to rule out all but one.

*   The symbol immediately following any left-recursion must be a terminal
    that occurs nowhere else in the grammar.

These restrictions are severe; the last rules out unary `-`.
"""

import collections

def check(grammar):
    """ Check that there is no left-recursion in the grammar, direct or indirect.
    Check that no rule matches the empty string.

    If either check fails, throw.
    """

    # Maps names of nonterminals to one of:
    #     (missing) - we haven't seen this nt at all
    #     False - we are currently examining this nt for indirect left-recursion
    #     True - we checked and this nt is not indirectly left-recursive
    status = {}

    terminal_counts = collections.Counter(
        symbol
        for rules in grammar.values()
            for rule in rules
                for symbol in rule
                    if is_terminal(symbol))

    def check_nt(nt):
        s = status.get(nt)
        if s == True:
            return
        elif s == False:
            raise ValueError("invalid grammar: nonterminal {!r} is indirectly left-recursive".format(nt))
        else:
            assert s is None
            status[nt] = False
            rules = grammar[nt]
            for rule in rules:
                if len(rule) == 0:
                    raise ValueError("invalid grammar: nonterminal {!r} can match the empty string".format(nt))
                if rule[0] == nt:
                    if len(rule) == 1:
                        raise ValueError("invalid grammar: nonterminal {!r} has a silly production, matching itself".format(nt))
                    elif is_terminal(rule[1]):
                        if terminal_counts[rule[1]] > 1:
                            raise ValueError("invalid grammar: terminal `{}` after left-recursion on {} can't also be used anywhere else".format(rule[1], nt))
                    else:
                        raise ValueError("invalid grammar: expected terminal after left-recursion in production {} ::= {}".format(nt, ' '.join(rule)))
                elif is_nt(rule[0]):
                    check_nt(rule[0])
            status[nt] = True

    for nt in grammar:
        check_nt(nt)


def is_nt(element):
    return element[:1].islower()

def is_terminal(element):
    return not is_nt(element)

def start(grammar, element):
    if is_terminal(element):
        return {element}
    else:
        return set.union(*(rule_start(grammar, rule)
                           for rule in grammar[element]
                           if rule[0] != element))

def rule_start(grammar, rule):
    return start(grammar, rule[0])


def generate_parser(out, grammar, goal):
    check(grammar)

    write = out.write

    def emit_rules(nt, rules, indent, result_var, prepend_previous_result, on_fail):
        if_keyword = "if"

        # Set of terminals that can be the first token of a match for any rule
        # we've considered so far. We track this to rule out ambiguity (overzealously).
        #
        # We track this set even when we're emitting code for left-recursive productions;
        # it's not necessary, because check() imposes a much tougher rule, but the extra
        # checking doesn't hurt anything.
        seen = set()
        for i, rule in rules:
            start_set = rule_start(grammar, rule)
            if start_set & seen:
                raise ValueError("invalid grammar: ambiguous token(s) {}".format(start_set & seen))
            seen |= start_set

            write(indent + "{} token in {}:\n".format(if_keyword, repr(tuple(start_set))))
            if_keyword = "elif"
            if result_var is None:
                store = 'return'
            else:
                store = result_var + " ="
            write(indent + "    {} ({!r}, {!r}, [\n".format(store, nt, i))
            if prepend_previous_result:
                write(indent + "        {},\n".format(result_var))
            for element in rule:
                if is_terminal(element):
                    write(indent + "        src.take({}),\n".format(repr(element)))
                else:
                    write(indent + "        parse_{}(src),\n".format(element))
            write(indent + "    ])\n")
        write(indent + "else:\n")
        write(indent + "    {}\n".format(on_fail))

    for nt, rules in grammar.items():
        write("def parse_{}(src):\n".format(nt))
        write("    token = src.peek()\n")
        plain_rules = []
        left_recursive_rules = []
        for i, rule in enumerate(rules):
            if rule[:1] == [nt]:
                left_recursive_rules.append((i, rule[1:]))
            else:
                plain_rules.append((i, rule))

        if len(plain_rules) == 0:
            if left_recursive_rules:
                raise ValueError("invalid grammar: all productions for nonterminal {} are left-recursive".format(nt))
            else:
                raise ValueError("invalid grammar: nonterminal {} has no productions".format(nt))

        emit_rules(nt,
                   plain_rules,
                   indent=4 * " ",
                   result_var='result' if left_recursive_rules else None,
                   prepend_previous_result=False,
                   on_fail="raise ValueError({!r}.format(token))".format("expected " + nt + ", got {!r}"))
        if left_recursive_rules:
            write("    while True:\n")
            write("        token = src.peek()\n")
            emit_rules(nt,
                       left_recursive_rules,
                       indent=8 * " ",
                       result_var='result',
                       prepend_previous_result=True,
                       on_fail="break")
            write("    return result\n")

        write("\n")

    # Write entry point.
    write("def parse(src):\n")
    write("    ast = parse_{}(src)\n".format(goal))
    write("    src.take(None)\n")
    write("    return ast\n")

def main():
    grammar = {
        'expr': [
            ['term'],
            ['expr', '+', 'term'],
            ['expr', '-', 'term'],
        ],
        'term': [
            ['prim'],
            ['term', '*', 'prim'],
            ['term', '/', 'prim'],
        ],
        'prim': [
            ['NUM'],
            ['VAR'],
            ['(', 'expr', ')'],
        ],
    }

    class Tokens:
        def __init__(self, space_separated):
            self.tokens = space_separated.split()

        def peek(self):
            if len(self.tokens) == 0:
                return None
            else:
                next = self.tokens[0]
                if next.isdigit():
                    return "NUM"
                elif next.isalpha():
                    return "VAR"
                elif next in '+-*/()':
                    return next
                else:
                    raise ValueError("unexpected token {!r}".format(next))

        def take(self, k):
            if k is None:
                if self.tokens:
                    raise ValueError("expected end of input")
            else:
                assert self.peek() == k
                return self.tokens.pop(0)

    import io
    out = io.StringIO()
    generate_parser(out, grammar, 'expr')
    code = out.getvalue()
    print(code)
    print("----")

    sandbox = {}
    exec(code, sandbox)
    parse = sandbox['parse']

    while True:
        try:
            line = input('> ')
        except EOFError as _:
            break
        tokens = Tokens(line)
        try:
            result = parse(tokens)
        except Exception as exc:
            print(exc)
        else:
            print(result)

if __name__ == '__main__':
    main()
