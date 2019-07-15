"""Parse a grammar written in ECMArkup."""

from jsparagus.lexer import LexicalGrammar
from jsparagus import parse_pgen, gen, grammar
import os


tokenize_emug = LexicalGrammar(
    # the operators and keywords:
    "[ ] { } , ~ + ? <! == != "
    "but empty here lookahead no not of one or through",

    NL="\n",

    # any number of colons together
    EQ=r':+',

    # terminals of the ES grammar, quoted with backticks
    T=r'`[^` \n]+`|```',

    # also terminals, denoting control characters
    CHR=r'<[A-Z]+>|U\+[0-9A-f]{4}',

    # nonterminals that will be followed by boolean parameters
    NTCALL=r'(?:uri|[A-Z])\w*(?=\[)',

    # nonterminals (also, boolean parameters)
    NT=r'(?:uri|[A-Z])\w*',

    # nonterminals wrapped in vertical bars for no apparent reason
    NTALT=r'\|[A-Z]\w+\|',

    # the spec also gives a few productions names
    PRODID=r'#[A-Za-z]\w*',

    # prose to the end of the line
    PROSE=r'>.*',

    # prose wrapped in square brackets
    WPROSE=r'\[>[^]]*\]'
    )


parse_emug_generic = gen.compile(
    parse_pgen.load_grammar(
        os.path.join(os.path.dirname(__file__), "emug.pgen")))


SIGIL_FALSE = '~'
SIGIL_TRUE = '+'


class EmugBuilder:
    def single(self, x): return [x]
    def append(self, x, y): return x + [y]
    def append_ignoring_separator(self, x, sep, y): return x + [y]
    def concat(self, x, y): return x + y

    def blank_line(self, nl): return []
    def nt_def_to_list(self, nt_def): return [nt_def]

    def make_nt_def(self, lhs, eq, rhs_list):
        if isinstance(lhs, tuple):
            name, args = lhs
            return (name, eq, grammar.Parameterized(args, rhs_list))
        else:
            return (lhs, eq, rhs_list)

    def nt_def(self, nt_lhs, eq, nl, rhs_lines, nl2):
        # nt_lhs EQ NL rhs_lines NL
        assert nl == "\n"
        assert nl2 == "\n"
        return self.make_nt_def(nt_lhs, eq, rhs_lines)

    def nt_def_one_of(self, nt_lhs, eq, one, of, nl, terminals, nl2):
        # nt_lhs EQ "one" "of" NL t_list_lines
        assert one == "one"
        assert of == "of"
        assert nl == "\n"
        assert nl2 == "\n"
        return self.make_nt_def(nt_lhs, eq, [[t] for t in terminals])

    def nt_lhs_fn(self, name, ob, params, cb):
        # NTCALL [ params ]
        assert ob == '['
        assert cb == ']'
        return (name, params)

    def t_list_line(self, terminals, nl): return terminals

    def terminal(self, t):
        assert t[0] == "`"
        assert t[-1] == "`"
        return t[1:-1]

    def terminal_chr(self, chr):
        raise ValueError("FAILED: %r" % chr)

    def rhs_line(self, ifdef, rhs, prodid, nl):
        assert nl == "\n"
        result = rhs
        if ifdef is not None:
            name, value = ifdef
            result = grammar.ConditionalRhs(name, value, result)
        return result

    def rhs_line_prose(self, prose, nl):
        assert nl == "\n"
        return prose

    def empty_rhs(self, ob, empty, cb):
        assert (ob, empty, cb) == ("[", "empty", "]")
        return []

    def ifdef(self, ob, value, nt, cb):
        assert (ob, cb) == ("[", "]")
        return nt, value

    def optional(self, nt, q):
        # nonterminal `?`
        assert q == "?"
        return grammar.Optional(nt)

    def but_not(self, nt, but, not_, exclusion):
        # nonterminal "but not" exclusion
        assert but == "but"
        assert not_ == "not"
        return ('-', nt, exclusion)

    def but_not_one_of(self, nt, but, not_, one, of, exclusion_list):
        # nonterminal "but not one of" exclusion_list
        assert (but, not_, one, of) == ("but", "not", "one", "of")
        return ('-', nt, exclusion_list)

    def lookahead(self, ob, lookahead, look_assert, cb):
        # [lookahead ...]
        assert (ob, lookahead, cb) == ('[', 'lookahead', ']')
        return look_assert

    def no_line_terminator_here(self, ob, no, line_terminator, here, cb):
        assert ((ob, no, line_terminator, here, cb) ==
                ('[', 'no', 'LineTerminator', 'here', ']'))
        return ("no-LineTerminator-here",)

    def nonterminal(self, nt):
        return nt

    def nonterminal_apply(self, name, ob, args, cb):
        assert (ob, cb) == ('[', ']')
        if len(set(k for k, expr in args)) != len(args):
            raise ValueError("parameter passed multiple times")
        return grammar.Apply(name, tuple(args))

    def args_single(self, arg):
        return dict([arg])

    def arg_expr(self, sigil, argname):
        if sigil == '?':
            return (argname, grammar.Var(argname))
        else:
            return (argname, sigil)

    def sigil_false(self, sigil):
        assert sigil == SIGIL_FALSE
        return False

    def sigil_true(self, sigil):
        assert sigil == SIGIL_TRUE
        return True

    def exclusion_terminal(self, t):
        return ("t", t)

    def exclusion_nonterminal(self, nt):
        return ("nt", nt)

    def exclusion_chr_range(self, c1, through, c2):
        assert through == "through"
        return ("range", c1, c2)

    def la_eq(self, eq, t):
        assert eq == "=="
        return grammar.LookaheadRule(frozenset([t]), True)

    def la_ne(self, ne, t):
        assert ne == "!="
        return grammar.LookaheadRule(frozenset([t]), False)

    def la_not_in_nonterminal(self, notin, nt):
        assert notin == '<!'
        return ('?!', nt)

    def la_not_in_set(self, notin, ob, lookahead_exclusions, cb):
        assert (notin, ob, cb) == ("<!", '{', '}')
        if all(len(excl) == 1 for excl in lookahead_exclusions):
            return grammar.LookaheadRule(
                frozenset(excl[0] for excl in lookahead_exclusions),
                False)
        raise ValueError("unsupported: lookahead > 1 token, {!r}"
                         .format(lookahead_exclusions))


def finish_grammar(nt_defs, goals):
    terminal_set = set()

    def hack_rhs(rhs):
        for i, e in enumerate(rhs):
            if isinstance(e, str) and e[:1] == "`":
                if len(e) < 3 or e[-1:] != "`":
                    raise ValueError(
                        "I don't know what this is: {!r} (in {!r})"
                        .format(e, rhs))
                rhs[i] = token = e[1:-1]
                terminal_set.add(token)

    nonterminals = {}
    variable_terminals = set()
    for nt_name, eq, rhs_list_or_lambda in nt_defs:
        if eq == "::":
            variable_terminals.add(nt_name)

        if isinstance(rhs_list_or_lambda, grammar.Parameterized):
            nonterminals[nt_name] = rhs_list_or_lambda
        else:
            rhs_list = rhs_list_or_lambda
            for rhs in rhs_list:
                if not isinstance(rhs, list):
                    raise ValueError(
                        "invalid grammar: ifdef in non-function-call context")
                hack_rhs(rhs)
            if eq == ':':
                if nt_name in nonterminals:
                    raise ValueError(
                        "unsupported: multiple definitions for nt " + nt_name)
                nonterminals[nt_name] = rhs_list

    for t in terminal_set:
        if t in nonterminals:
            raise ValueError(
                "grammar contains both a terminal `{}` and nonterminal {}"
                .format(t, t))

    return grammar.Grammar(nonterminals, goals, variable_terminals)


def parse_emug(text, filename=None, goals=None):
    tokens = tokenize_emug(text, filename=filename)
    return finish_grammar(parse_emug_generic(tokens, EmugBuilder()),
                          goals=goals)
