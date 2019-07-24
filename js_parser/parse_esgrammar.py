"""Parse a grammar written in ECMArkup."""

import os
import re
from jsparagus import parse_pgen, gen, grammar
from jsparagus.lexer import LexicalGrammar
from jsparagus.ordered import OrderedFrozenSet


tokenize_esgrammar = LexicalGrammar(
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
    WPROSE=r'\[>[^]]*\]',
)


parse_esgrammar_generic = gen.compile(
    parse_pgen.load_grammar(
        os.path.join(os.path.dirname(__file__), "esgrammar.pgen")))


SIGIL_FALSE = '~'
SIGIL_TRUE = '+'

# Productions like
#
#     Expression : AssignmentExpression
#     PrimaryExpression : ArrayLiteral
#     Statement : IfStatement
#
# should not cause an extra method call; the action for each of these
# productions should be `$0`, i.e. just return the right-hand side unchanged.
# Then type inference will make sure that the two nonterminals (Statement and
# IfStatement, for example) are given the same type.
#
# ESGrammarBuilder uses the regular expressions below to determine when to do
# this. A production gets the special `$0` action if any of the regular
# expressions matches both sides.
PRODUCTION_GROUPS = [
    r'(Expression|^(Array|Object)?Literal)$',
    r'(Statement|Declaration|^StatementListItem|^ModuleItem)$',
    r'Method(Definition)?$'
]


class ESGrammarBuilder:
    def single(self, x):
        return [x]

    def append(self, x, y):
        return x + [y]

    def append_ignoring_separator(self, x, sep, y):
        return x + [y]

    def concat(self, x, y):
        return x + y

    def blank_line(self, nl):
        return []

    def nt_def_to_list(self, nt_def):
        return [nt_def]

    def is_matched_pair(self, lhs_name, rhs_element):
        if isinstance(rhs_element, grammar.Apply):
            rhs_element = rhs_element.nt
        for group in PRODUCTION_GROUPS:
            if (re.search(group, lhs_name) is not None
                    and re.search(group, rhs_element) is not None):
                return True
        return False

    def to_production(self, lhs, i, rhs, is_sole_production):
        """Wrap a list of grammar symbols `rhs` in a Production object."""
        if isinstance(rhs, grammar.ConditionalRhs):
            body = rhs.rhs
            return rhs._replace(
                rhs=self.to_production(lhs, i, body, is_sole_production))

        if isinstance(lhs, tuple):
            nt_name = lhs[0]
        else:
            nt_name = lhs

        nargs = sum(1 for e in rhs if grammar.is_concrete_element(e))
        if (len(rhs) == 1
                and nargs == 1
                and self.is_matched_pair(nt_name, rhs[0])):
            action = 0
        else:
            if is_sole_production:
                method_name = nt_name
            else:
                method_name = '{} {}'.format(nt_name, i)
            action = grammar.CallMethod(method_name, tuple(range(nargs)))
        return grammar.Production(rhs, action)

    def needs_asi(self, p):
        """True if p is a production in which ASI can happen."""
        # Two productions have body == [";"] -- one for EmptyStatement and one
        # for ClassMember. Neither should trigger ASI. The only other
        # semicolons that should not trigger ASI are the ones in `for`
        # statements, which happen to be exactly those semicolons that are not
        # at the end of a production.
        if isinstance(p, grammar.ConditionalRhs):
            return self.needs_asi(p.rhs)
        else:
            return len(p.body) > 1 and p.body[-1] == ';'

    def apply_asi(self, p):
        """Return two rules based on p, so that ASI can be applied."""
        assert self.needs_asi(p)

        if isinstance(p, grammar.ConditionalRhs):
            return p._replace(rhs=self.apply_asi(p.rhs))

        assert isinstance(p.action, grammar.CallMethod)

        # Don't pass the semicolon to the method.
        action = grammar.CallMethod(p.action.method,
                                    p.action.args[:-1])

        # Except for do-while loops, check at runtime that ASI occurs only at
        # the end of a line.
        if (len(p.body) == 7
                and p.body[0] == 'do'
                and p.body[2] == 'while'
                and p.body[3] == '('
                and p.body[5] == ')'
                and p.body[6] == ';'):
            asi_action = action
        else:
            asi_action = grammar.CallMethod('check_asi', (action,))

        return [
            # The preferred production, with the semicolon in.
            p.copy_with(body=p.body[:],
                        action=action),
            # The fallback production, performing ASI.
            p.copy_with(body=p.body[:-1] + [grammar.ErrorToken],
                        action=asi_action),
        ]

    def make_nt_def(self, lhs, eq, rhs_list):
        has_sole_production = (len(rhs_list) == 1)
        production_list = []
        for i, rhs in enumerate(rhs_list):
            p = self.to_production(lhs, i, rhs, has_sole_production)
            if self.needs_asi(p):
                production_list += self.apply_asi(p)
            else:
                production_list.append(p)
        if isinstance(lhs, tuple):
            name, args = lhs
            return (name, eq, grammar.Parameterized(args, production_list))
        else:
            return (lhs, eq, production_list)

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

    def t_list_line(self, terminals, nl):
        return terminals

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
        assert ((ob, no, line_terminator, here, cb)
                == ('[', 'no', 'LineTerminator', 'here', ']'))
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
        return grammar.LookaheadRule(OrderedFrozenSet([t]), True)

    def la_ne(self, ne, t):
        assert ne == "!="
        return grammar.LookaheadRule(OrderedFrozenSet([t]), False)

    def la_not_in_nonterminal(self, notin, nt):
        assert notin == '<!'
        return ('?!', nt)

    def la_not_in_set(self, notin, ob, lookahead_exclusions, cb):
        assert (notin, ob, cb) == ("<!", '{', '}')
        if all(len(excl) == 1 for excl in lookahead_exclusions):
            return grammar.LookaheadRule(
                OrderedFrozenSet(excl[0] for excl in lookahead_exclusions),
                False)
        raise ValueError("unsupported: lookahead > 1 token, {!r}"
                         .format(lookahead_exclusions))


def finish_grammar(nt_defs, goals):
    terminal_set = set()

    def hack_production(p):
        for i, e in enumerate(p.body):
            if isinstance(e, str) and e[:1] == "`":
                if len(e) < 3 or e[-1:] != "`":
                    raise ValueError(
                        "Unrecognized grammar symbol: {!r} (in {!r})"
                        .format(e, p))
                p[i] = token = e[1:-1]
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
            for p in rhs_list:
                if not isinstance(p, grammar.Production):
                    raise ValueError(
                        "invalid grammar: ifdef in non-function-call context")
                hack_production(p)
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


def parse_esgrammar(text, filename=None, goals=None):
    return finish_grammar(
        parse_esgrammar_generic(tokenize_esgrammar, text, ESGrammarBuilder()),
        goals=goals)
