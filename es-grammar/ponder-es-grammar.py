#!/usr/bin/env python

""" ponder-es-grammar.py - Try to parse the ES grammar. """

import sys; sys.path.append("..")

from lexer import LexicalGrammar
import gen
from gen import LookaheadRule
import parse_pgen
import argparse

tokenize_emug = LexicalGrammar(
    #   the operators and keywords:
    "[ ] { } , ~ + ? <! == != but empty here lookahead no not of one or through",
    NL="\n",
    EQ=r':+',                           # any number of colons together
    T=r'`[^` \n]+`|```',                # terminals of the ES grammar, quoted with backticks
    CHR=r'<[A-Z]+>|U\+[0-9A-f]{4}',     # also terminals, denoting control characters
    NTCALL=r'(?:uri|[A-Z])\w*(?=\[)',   # nonterminals that will be followed by boolean parameters
    NT=r'(?:uri|[A-Z])\w*',             # nonterminals (also, boolean parameters)
    NTALT=r'\|[A-Z]\w+\|',              # nonterminals wrapped in vertical bars for no apparent reason
    PRODID=r'#[A-Za-z]\w*',             # the spec also gives a few productions names
    PROSE=r'>.*',                       # prose to the end of the line
    WPROSE=r'\[>[^]]*\]'                # prose wrapped in square brackets
    )

parse_emug = gen.compile(parse_pgen.load_grammar("emug.pgen"), "grammar")

def unroll(ast, list_nt, element_nt):
    x = []
    if ast is not None:
        a, i, e = ast
        assert a == list_nt
        while i == 1:
            if len(e) == 2:
                [rtail, rhead] = e
            else:
                [rtail, comma, rhead] = e
                assert comma == ","
            assert rhead[0] == element_nt
            x.append(rhead)
            a, i, e = rtail
            assert a == list_nt
        assert i == 0
        [rhead] = e
        assert rhead[0] == element_nt
        x.append(rhead)
        x.reverse()
    return x


def postparse_param(ast):
    nt, i, args = ast
    assert nt == 'param'
    assert i == 0
    return args[0]

def postparse_nt_lhs(ast):
    nt, i, args = ast
    assert nt == "nt_lhs"
    if i == 0:  # NT
        [name] = args
        return name
    else:  # NTCALL [ params ]
        [name, lb, params, rb] = args
        assert lb == '['
        assert rb == ']'
        return (name, [postparse_param(ast) for ast in unroll(params, "params", "param")])

SIGIL_FALSE = ('definite_sigil', 0, ['~'])
SIGIL_TRUE = ('definite_sigil', 1, ['+'])

def postparse_arg(ast):
    nt, i, args = ast
    assert nt == "arg"
    assert i == 0
    [sigil, argname] = args
    if sigil == ('sigil', 1, ['?']):
        return '?' + argname
    elif sigil == ('sigil', 0, [SIGIL_FALSE]):
        return '~' + argname
    elif sigil == ('sigil', 0, [SIGIL_TRUE]):
        return '+' + argname
    else:
        assert False


def postparse_nonterminal(ast):
    nt, i, args = ast
    assert nt == "nonterminal"
    if i == 0:
        [name] = args
        return name
    elif i == 1:
        [name, lb, args_ast, rb] = args
        return ('apply', name, [postparse_arg(ast) for ast in unroll(args_ast, "args", "arg")])


def postparse_terminal(ast):
    nt, i, [t] = ast
    assert nt == "terminal"
    assert i in (0, 1)
    assert len(t) >= 3
    assert t[0] == "`"
    assert t[-1] == "`"
    return t[1:-1]


def postparse_lookahead_exclusion(ast):
    return [postparse_lookahead_exclusion_element(e_ast)
            for e_ast in unroll(ast, "lookahead_exclusion", "lookahead_exclusion_element")]


def postparse_lookahead_exclusion_element(ast):
    nt, i, args = ast
    assert nt == "lookahead_exclusion_element"
    if i == 0:
        [t_ast] = args
        return postparse_terminal(t_ast)
    else:
        assert i == 1
        [n_ast] = args
        return postparse_no_line_terminator_here(n_ast)


def postparse_lookahead_assertion(ast):
    nt, i, args = ast
    assert nt == "lookahead_assertion"
    if i in (0, 1):
        [relop, symbol] = args
        t = postparse_terminal(symbol)
        assert relop in ('==', '!=')
        return LookaheadRule(frozenset([t]), relop == "==")
    elif i == 2:
        [relop, symbol] = args
        assert relop == '<!'
        return ('?!', symbol)
    else:
        assert i == 3
        [relop, lc, xs_ast, rc] = args
        assert (relop, lc, rc) == ('<!', '{', '}')
        excls = [postparse_lookahead_exclusion(x_ast)
                 for x_ast in unroll(xs_ast, 'lookahead_exclusions', 'lookahead_exclusion')]
        if all(len(excl) == 1 for excl in excls):
            return LookaheadRule(frozenset(excl[0] for excl in excls), False)
        raise ValueError("unsupported: lookahead > 1 token, " + repr(excls))


def postparse_no_line_terminator_here(ast):
    nt2, i2, args2 = ast
    assert nt2 == "no_line_terminator_here"
    assert i2 == 0
    [lb, no_t, line_terminator, here_t, rb] = args2
    assert (lb, no_t, here_t, rb) == ('[', 'no', 'here', ']')
    nt3, i3, args3 = line_terminator
    assert nt3 == 'line_terminator'
    assert (i3, args3) in ((0, ["LineTerminator"]), (1, ["|LineTerminator|"]))
    return ("no-LineTerminator-here",)


def postparse_symbol(ast):
    nt, i, args = ast
    assert nt == "symbol"
    if i == 0:  # terminal
        [t_ast] = args
        return postparse_terminal(t_ast)
    elif i == 1:  #nonterminal
        [nt_ast] = args
        return postparse_nonterminal(nt_ast)
    elif i == 2:  #nonterminal "?"
        [nt_ast, q] = args
        assert q == "?"
        return gen.Optional(postparse_nonterminal(nt_ast))
    elif i == 3:  #nonterminal "but not" exclusion
        [nt_ast, but_t, not_t, exclusion_ast] = args
        assert but_t == "but"
        assert not_t == "not"
        return ('-', postparse_nonterminal(nt_ast), exclusion_ast)
    elif i == 4:  #nonterminal "but not one of" exclusion_list
        [nt_ast, but_t, not_t, one_t, of_t, exclusion_list_ast] = args
        assert (but_t, not_t, one_t, of_t) == ("but", "not", "one", "of")
        return ('-', postparse_nonterminal(nt_ast), exclusion_list_ast)
    elif i == 5:  # [lookahead ...]
        [lb, lookahead_t, look_assert_ast, rb] = args
        assert (lb, lookahead_t, rb) == ('[', 'lookahead', ']')
        return postparse_lookahead_assertion(look_assert_ast)
    elif i == 6:  # [no LineTerminator here]
        [n_ast] = args
        return postparse_no_line_terminator_here(n_ast)
    else:
        return ast


def postparse_symbols(ast):
    return [postparse_symbol(sym) for sym in unroll(ast, "symbols", "symbol")]


def postparse_rhs(ast):
    nt, i, args = ast
    assert nt == "rhs"
    if i == 0:
        [symbols] = args
        return postparse_symbols(symbols)
    else:
        assert i == 1
        assert args == ["[", "empty", "]"]
        return []


def postparse_rhs_line(ast):
    nt, i, args = ast
    assert nt == 'rhs_line'
    if i == 0:
        [ifdef, rhs, _prodid, nl] = args
        result = postparse_rhs(rhs)
        assert nl == "\n"
        if ifdef is not None:
            nt2, i2, args2 = ifdef
            assert nt2 == 'ifdef'
            assert i2 == 0
            lb, sigil, name, rb = args2
            assert lb == '['
            assert rb == ']'
            if sigil == SIGIL_FALSE:
                sigil = False
            elif sigil == SIGIL_TRUE:
                sigil = True
            result = ('if', name, sigil, result)
        return result
    else:
        [prose, nl] = args
        assert nl == "\n"
        return prose


def postparse_t_list_line(ast):
    nt, i, args = ast
    assert nt == 't_list_line'
    assert i == 0
    [ts_ast, nl] = args
    assert nl == "\n"
    return [postparse_terminal(t_ast) for t_ast in unroll(ts_ast, "terminal_seq", "terminal")]


def postparse_nt_def(ast):
    nt, i, args = ast
    assert nt == 'nt_def'
    if i == 0:  # nt_lhs EQ NL rhs_lines
        [lhs_ast, eq, _nl, rhs_lines_ast, _nl2] = args
        lhs = postparse_nt_lhs(lhs_ast)
        rhs_list = [postparse_rhs_line(line_ast)
                    for line_ast in unroll(rhs_lines_ast, "rhs_lines", "rhs_line")]

    else:
        assert i == 1  # nt_lhs EQ "one" "of" NL t_list_lines
        [lhs_ast, eq, one_t, of_t, nl, t_list_lines_ast, nl2] = args
        lhs = postparse_nt_lhs(lhs_ast)
        assert one_t == "one"
        assert of_t == "of"
        assert nl == "\n"
        assert nl2 == "\n"
        rhs_list = [[t]
                    for tll_ast in unroll(t_list_lines_ast, "t_list_lines", "t_list_line")
                    for t in postparse_t_list_line(tll_ast)]

    if isinstance(lhs, tuple):
        name, args = lhs
        return (name, eq, ("lambda", args, rhs_list))
    else:
        return (lhs, eq, rhs_list)


def expand_apply(e, env):
    if gen.is_optional(e):
        return gen.Optional(expand_apply(e.inner, env))
    if isinstance(e, tuple) and e[0] == 'apply':
        _tag, nt, args = e
        arg_values = []
        for arg in args:
            if arg[0] == '+':
                v = True
            elif arg[0] == '~':
                v = False
            else:
                assert arg[0] == '?'
                if arg[1:] not in env:
                    raise ValueError("invalid grammar: {} is not defined here"
                                     .format(arg[1:]))
                v = env[arg[1:]]
            arg_values.append(v)
        return gen.Apply(nt, tuple(arg_values))
    return e


def make_rhs_lambda(func_name, param_names, body):
    def build_rhs_list(*args):
        if len(param_names) != len(args):
            raise ValueError("nonterminal {} called with wrong number of arguments:"
                             "expected {!r}, got {!r}"
                             .format(nt_name, params, args))
        assert all(isinstance(a, bool) for a in args)
        env = dict(zip(param_names, args))

        result = []
        for rhs in body:
            if isinstance(rhs, tuple) and rhs[0] == 'if':
                _tag, name, enabled, seq = rhs
                if env[name] != enabled:
                    continue  # this production is inhibited, skip it
                rhs = seq
            expanded_rhs = [expand_apply(e, env) for e in rhs]
            result.append(expanded_rhs)
        return result

    build_rhs_list.__name__ = func_name
    return build_rhs_list


def postparse_grammar(ast):
    terminal_set = set()

    def hack_rhs(rhs):
        for i, e in enumerate(rhs):
            if isinstance(e, str) and e[:1] == "`":
                if len(e) < 3 or e[-1:] != "`":
                    raise ValueError("I don't know what this is: " + repr(e) + "(in " + repr(rhs) + ")")
                rhs[i] = token = e[1:-1]
                terminal_set.add(token)
            else:
                rhs[i] = expand_apply(rhs[i], {})

    grammar = {}
    for nt, i, [nt_def_ast] in unroll(ast, "grammar", "nt_def_or_blank_line"):
        if i == 1:
            nt_name, eq, rhs_list_or_lambda = postparse_nt_def(nt_def_ast)
            if isinstance(rhs_list_or_lambda, tuple):
                tag, params, rhs_list = rhs_list_or_lambda
                assert tag == "lambda"
                grammar[nt_name] = make_rhs_lambda(nt_name, params, rhs_list)
            else:
                rhs_list = rhs_list_or_lambda
                for rhs in rhs_list:
                    if not isinstance(rhs, list):
                        raise ValueError("invalid grammar: ifdef in non-function-call context")
                    hack_rhs(rhs)
                if eq == ':':
                    if nt_name in grammar:
                        raise ValueError("unsupported: multiple definitions for nt " + nt_name)
                    grammar[nt_name] = rhs_list

    for t in terminal_set:
        if t in grammar:
            raise ValueError("grammar contains both a terminal `{}` and nonterminal {}".format(t, t))

    return grammar


def main():
    parser = argparse.ArgumentParser(description='Ponder the ECMAScript grammar.')
    parser.add_argument('filename', metavar='FILE', nargs='?', default='./es-simplified.emug',
                        help="emug file containing the grammar")
    args = parser.parse_args()

    with open(args.filename) as f:
        text = f.read()

    tokens = tokenize_emug(text, filename=args.filename)
    ast = parse_emug(tokens)
    grammar = postparse_grammar(ast)
    #gen.dump_grammar(grammar)
    gen.generate_parser(sys.stdout, grammar, 'Script')

if __name__ == '__main__':
    main()
