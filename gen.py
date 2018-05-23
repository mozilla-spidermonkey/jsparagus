#!/usr/bin/env python3

"""gen.py - First stab at a parser generator.

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
We verify that there is no left-recursion in the grammar, direct or indirect.
We verify that no rules are empty, i.e. no nonterminal matches the empty string.
We assume that every nonterminal matches at least one string of finite length.
We verify that no two rules for the same nonterminal have overlapping start sets.

These restrictions are severe; I don't think you can implement arithmetic
under them.
"""

def check(grammar):
    """ Check that there is no left-recursion in the grammar, direct or indirect.
    Check that no rule matches the empty string.

    If either check fails, throw.
    """

    # Maps names of nonterminals to one of:
    #     (missing) - we haven't seen this nt at all
    #     False - we are currently examining this nt for left-recursion
    #     True - we checked and this nt is not left-recursive
    status = {}

    def check_nt(nt):
        s = status.get(nt)
        if s == True:
            return
        elif s == False:
            raise ValueError("nonterminal {!r} is left-recursive".format(nt))
        else:
            assert s is None
            status[nt] = False
            for rule in grammar[nt]:
                if len(rule) == 0:
                    raise ValueError("nonterminal {!r} can match the empty string".format(nt))
                if is_nt(rule[0]):
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
        return set.union(*(rule_start(grammar, rule) for rule in grammar[element]))

def rule_start(grammar, rule):
    return start(grammar, rule[0])

def generate_parser(out, grammar, goal):
    check(grammar)

    write = out.write
    for nt, rules in grammar.items():
        write("def parse_{}(src):\n".format(nt))
        write("    token = src.peek()\n")
        seen = set()
        for i, rule in enumerate(rules):
            start_set = rule_start(grammar, rule)
            if start_set & seen:
                raise ValueError("invalid grammar: ambiguous token(s) {}".format(start_set & seen))
            seen |= start_set

            write("    {} token in {}:\n".format("if" if i == 0 else "elif", repr(tuple(start_set))))
            write("        return ({!r}, {!r}, [\n".format(nt, i))
            for element in rule:
                if is_terminal(element):
                    write("            src.take({}),\n".format(repr(element)))
                else:
                    write("            parse_{}(src),\n".format(element))
            write("        ])\n")
        write("    else:\n")
        write("        raise ValueError({!r}.format(token))\n".format("expected " + nt + ", got {!r}"))
        write("\n")

    # Write entry point.
    write("def parse(src):\n")
    write("    ast = parse_{}(src)\n".format(goal))
    write("    src.take(None)\n")
    write("    return ast\n")

if __name__ == '__main__':
    grammar = {
        'sexpr': [
            ['Symbol'],
            ["(", 'tail'],
        ],
        'tail': [
            [")"],
            ['sexpr', 'tail']
        ],
    }

    class Tokens:
        def __init__(self, space_separated):
            self.tokens = space_separated.split()

        def peek(self):
            if len(self.tokens) == 0:
                return None
            elif self.tokens[0] in '()':
                return self.tokens[0]
            else:
                return 'Symbol'

        def take(self, k):
            if k is None:
                if self.tokens:
                    raise ValueError("expected end of input")
            else:
                assert self.peek() == k
                return self.tokens.pop(0)

    import io
    out = io.StringIO()
    generate_parser(out, grammar, 'sexpr')
    code = out.getvalue()
    print(code)
    print("----")

    exec(code)


    tokens = Tokens('( lambda ( x ) ( * x x ) )')
    result = parse(tokens)
    assert result == ('sexpr', 1, [
        '(',
        ('tail', 1, [
            ('sexpr', 0, ['lambda']),
            ('tail', 1, [
                ('sexpr', 1, [
                    '(',
                    ('tail', 1, [
                        ('sexpr', 0, ['x']),
                        ('tail', 0, [')'])
                    ])
                ]),
                ('tail', 1, [
                    ('sexpr', 1, [
                        '(',
                        ('tail', 1, [
                            ('sexpr', 0, ['*']),
                            ('tail', 1, [
                                ('sexpr', 0, ['x']),
                                ('tail', 1, [
                                    ('sexpr', 0, ['x']),
                                    ('tail', 0, [')'])
                                ])
                            ])
                        ])
                    ]),
                    ('tail', 0, [')'])
                ])
            ])
        ])
    ])

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

