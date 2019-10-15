""" Functions for loading the ECMAScript lexical and syntactic grammars. """

from jsparagus.ordered import OrderedSet, OrderedFrozenSet
from .lexer import ECMASCRIPT_FULL_KEYWORDS, ECMASCRIPT_CONDITIONAL_KEYWORDS
from .parse_esgrammar import parse_esgrammar


ECMASCRIPT_LEXICAL_SYNTHETIC_TERMINALS = {
    # Theoretically, this should be the set of all Unicode characters, but that
    # would take a lot of memory, and in practice, the set is not used.
    'SourceCharacter': OrderedFrozenSet([]),
}

ECMASCRIPT_LEXICAL_GOAL_NTS = [
    'WhiteSpace',
    'InputElementDiv',
    'InputElementRegExp',
]


def load_lexical_grammar(filename):
    """Load the ECMAScript lexical grammar."""
    with open(filename) as f:
        grammar_text = f.read()
    g = parse_esgrammar(
        grammar_text,
        filename=filename,
        goals=ECMASCRIPT_LEXICAL_GOAL_NTS,
        synthetic_terminals=ECMASCRIPT_LEXICAL_SYNTHETIC_TERMINALS,
        terminal_names=ECMASCRIPT_LEXICAL_SYNTHETIC_TERMINALS.keys())
    return gen.expand_parameterized_nonterminals(g)


ECMASCRIPT_SYNTACTIC_GOAL_NTS = [
    'Script',
    'Module',
    # 'FormalParameters',
    # 'FunctionBody',
]

# Identifiers are complicated. A "synthetic terminal" is a shorthand symbol
# that stands for any one of a set of terminals. For example, *IdentifierName*
# stands for any token that looks like an identifier, including keywords.
ECMASCRIPT_SYNTHETIC_TERMINALS = {
    'IdentifierName': OrderedSet([
        'Name',
        *ECMASCRIPT_FULL_KEYWORDS,
        *ECMASCRIPT_CONDITIONAL_KEYWORDS
    ]),
    'Identifier': OrderedSet([
        'Name',
        *ECMASCRIPT_CONDITIONAL_KEYWORDS
    ]),
}

# Names of lexical nonterminals that correspond to whole tokens and thus serve
# as terminals in the syntactic grammar. The spec makes *Identifier* a
# nonterminal in the syntactic grammar, but we treat it as a synthetic
# terminal instead.
ECMASCRIPT_TOKEN_NAMES = OrderedFrozenSet([
    'BooleanLiteral',
    'Identifier',
    'IdentifierName',
    'NoSubstitutionTemplate',
    'NullLiteral',
    'NumericLiteral',
    'RegularExpressionLiteral',
    'StringLiteral',
    'TemplateHead',
    'TemplateMiddle',
    'TemplateTail',
])


def load_syntactic_grammar(filename):
    """Load the ECMAScript syntactic grammar."""
    with open(filename) as f:
        grammar_text = f.read()

    g = parse_esgrammar(
        grammar_text,
        filename=filename,
        goals=ECMASCRIPT_SYNTACTIC_GOAL_NTS,
        synthetic_terminals=ECMASCRIPT_SYNTHETIC_TERMINALS,
        terminal_names=ECMASCRIPT_TOKEN_NAMES)

    return g
