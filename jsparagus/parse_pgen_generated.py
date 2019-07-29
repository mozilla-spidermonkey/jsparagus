from jsparagus import runtime
from jsparagus.runtime import Nt, ErrorToken

actions = [
    # 0. <empty>
    {'nt': 1, 'goal': 2, 'token': 3, 'var': 4},

    # 1. "nt"
    {'IDENT': 10},

    # 2. "goal"
    {'nt': 11},

    # 3. "token"
    {'IDENT': 12},

    # 4. "var"
    {'token': 13},

    # 5. grammar
    {None: -9223372036854775807},

    # 6. nt_defs
    {'nt': 1, 'goal': 2, None: -1},

    # 7. token_defs
    {'token': 3, 'var': 4, 'nt': 1, 'goal': 2},

    # 8. nt_def
    {None: -5, 'nt': -5, 'goal': -5},

    # 9. token_def
    {'nt': -3, 'goal': -3, 'token': -3, 'var': -3},

    # 10. "nt" IDENT
    {'{': 17},

    # 11. "goal" "nt"
    {'IDENT': 18},

    # 12. "token" IDENT
    {'=': 19},

    # 13. "var" "token"
    {'IDENT': 20},

    # 14. nt_defs nt_def
    {None: -6, 'nt': -6, 'goal': -6},

    # 15. token_defs nt_defs
    {'nt': 1, 'goal': 2, None: -2},

    # 16. token_defs token_def
    {'nt': -4, 'goal': -4, 'token': -4, 'var': -4},

    # 17. "nt" IDENT "{"
    {'}': 21, 'IDENT': 22, 'STR': 23},

    # 18. "goal" "nt" IDENT
    {'{': 29},

    # 19. "token" IDENT "="
    {'STR': 30},

    # 20. "var" "token" IDENT
    {';': 31},

    # 21. "nt" IDENT "{" "}"
    {None: -9, 'nt': -9, 'goal': -9},

    # 22. "nt" IDENT "{" IDENT
    {';': -22, '?': -22, '=>': -22, 'IDENT': -22, 'STR': -22},

    # 23. "nt" IDENT "{" STR
    {';': -23, '?': -23, '=>': -23, 'IDENT': -23, 'STR': -23},

    # 24. "nt" IDENT "{" prods
    {'}': 32, 'IDENT': 22, 'STR': 23},

    # 25. "nt" IDENT "{" prod
    {'}': -13, 'IDENT': -13, 'STR': -13},

    # 26. "nt" IDENT "{" terms
    {';': 34, '=>': 35, 'IDENT': 22, 'STR': 23},

    # 27. "nt" IDENT "{" term
    {';': -17, '=>': -17, 'IDENT': -17, 'STR': -17},

    # 28. "nt" IDENT "{" symbol
    {'?': 38, ';': -20, '=>': -20, 'IDENT': -20, 'STR': -20},

    # 29. "goal" "nt" IDENT "{"
    {'}': 39, 'IDENT': 22, 'STR': 23},

    # 30. "token" IDENT "=" STR
    {';': 41},

    # 31. "var" "token" IDENT ";"
    {'nt': -8, 'goal': -8, 'token': -8, 'var': -8},

    # 32. "nt" IDENT "{" prods "}"
    {None: -11, 'nt': -11, 'goal': -11},

    # 33. "nt" IDENT "{" prods prod
    {'}': -14, 'IDENT': -14, 'STR': -14},

    # 34. "nt" IDENT "{" terms ";"
    {'}': -15, 'IDENT': -15, 'STR': -15},

    # 35. "nt" IDENT "{" terms "=>"
    {'IDENT': 42},

    # 36. "nt" IDENT "{" terms action
    {';': 43},

    # 37. "nt" IDENT "{" terms term
    {';': -18, '=>': -18, 'IDENT': -18, 'STR': -18},

    # 38. "nt" IDENT "{" symbol "?"
    {';': -21, '=>': -21, 'IDENT': -21, 'STR': -21},

    # 39. "goal" "nt" IDENT "{" "}"
    {None: -10, 'nt': -10, 'goal': -10},

    # 40. "goal" "nt" IDENT "{" prods
    {'}': 44, 'IDENT': 22, 'STR': 23},

    # 41. "token" IDENT "=" STR ";"
    {'nt': -7, 'goal': -7, 'token': -7, 'var': -7},

    # 42. "nt" IDENT "{" terms "=>" IDENT
    {';': -19},

    # 43. "nt" IDENT "{" terms action ";"
    {'}': -16, 'IDENT': -16, 'STR': -16},

    # 44. "goal" "nt" IDENT "{" prods "}"
    {None: -12, 'nt': -12, 'goal': -12},

]

ctns = [
    {'grammar': 5, 'nt_defs': 6, 'token_defs': 7, 'nt_def': 8, 'token_def': 9},
    {},
    {},
    {},
    {},
    {},
    {'nt_def': 14},
    {'nt_defs': 15, 'token_def': 16, 'nt_def': 8},
    {},
    {},
    {},
    {},
    {},
    {},
    {},
    {'nt_def': 14},
    {},
    {'prods': 24, 'prod': 25, 'terms': 26, 'term': 27, 'symbol': 28},
    {},
    {},
    {},
    {},
    {},
    {},
    {'prod': 33, 'terms': 26, 'term': 27, 'symbol': 28},
    {},
    {'action': 36, 'term': 37, 'symbol': 28},
    {},
    {},
    {'prods': 40, 'prod': 25, 'terms': 26, 'term': 27, 'symbol': 28},
    {},
    {},
    {},
    {},
    {},
    {},
    {},
    {},
    {},
    {},
    {'prod': 33, 'terms': 26, 'term': 27, 'symbol': 28},
    {},
    {},
    {},
    {},
]

error_codes = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None,
]

reductions = [
    # 0. grammar ::= nt_defs => grammar(None, $0)
    ('grammar', 1, lambda builder, x0: builder.grammar(None, x0)),
    # 1. grammar ::= token_defs nt_defs => grammar(Some($0), $1)
    ('grammar', 2, lambda builder, x0, x1: builder.grammar(x0, x1)),
    # 2. token_defs ::= token_def => single($0)
    ('token_defs', 1, lambda builder, x0: builder.single(x0)),
    # 3. token_defs ::= token_defs token_def => append($0, $1)
    ('token_defs', 2, lambda builder, x0, x1: builder.append(x0, x1)),
    # 4. nt_defs ::= nt_def => nt_defs_single($0)
    ('nt_defs', 1, lambda builder, x0: builder.nt_defs_single(x0)),
    # 5. nt_defs ::= nt_defs nt_def => nt_defs_append($0, $1)
    ('nt_defs', 2, lambda builder, x0, x1: builder.nt_defs_append(x0, x1)),
    # 6. token_def ::= "token" IDENT "=" STR ";" => const_token($0, $1, $2, $3, $4)
    ('token_def', 5, lambda builder, x0, x1, x2, x3, x4: builder.const_token(x0, x1, x2, x3, x4)),
    # 7. token_def ::= "var" "token" IDENT ";" => var_token($0, $1, $2, $3)
    ('token_def', 4, lambda builder, x0, x1, x2, x3: builder.var_token(x0, x1, x2, x3)),
    # 8. nt_def ::= "nt" IDENT "{" "}" => nt_def(None, $0, $1, $2, None, $3)
    ('nt_def', 4, lambda builder, x0, x1, x2, x3: builder.nt_def(None, x0, x1, x2, None, x3)),
    # 9. nt_def ::= "goal" "nt" IDENT "{" "}" => nt_def(Some($0), $1, $2, $3, None, $4)
    ('nt_def', 5, lambda builder, x0, x1, x2, x3, x4: builder.nt_def(x0, x1, x2, x3, None, x4)),
    # 10. nt_def ::= "nt" IDENT "{" prods "}" => nt_def(None, $0, $1, $2, Some($3), $4)
    ('nt_def', 5, lambda builder, x0, x1, x2, x3, x4: builder.nt_def(None, x0, x1, x2, x3, x4)),
    # 11. nt_def ::= "goal" "nt" IDENT "{" prods "}" => nt_def(Some($0), $1, $2, $3, Some($4), $5)
    ('nt_def', 6, lambda builder, x0, x1, x2, x3, x4, x5: builder.nt_def(x0, x1, x2, x3, x4, x5)),
    # 12. prods ::= prod => single($0)
    ('prods', 1, lambda builder, x0: builder.single(x0)),
    # 13. prods ::= prods prod => append($0, $1)
    ('prods', 2, lambda builder, x0, x1: builder.append(x0, x1)),
    # 14. prod ::= terms ";" => prod($0, None, $1)
    ('prod', 2, lambda builder, x0, x1: builder.prod(x0, None, x1)),
    # 15. prod ::= terms action ";" => prod($0, Some($1), $2)
    ('prod', 3, lambda builder, x0, x1, x2: builder.prod(x0, x1, x2)),
    # 16. terms ::= term => single($0)
    ('terms', 1, lambda builder, x0: builder.single(x0)),
    # 17. terms ::= terms term => append($0, $1)
    ('terms', 2, lambda builder, x0, x1: builder.append(x0, x1)),
    # 18. action ::= "=>" IDENT => action($0, $1)
    ('action', 2, lambda builder, x0, x1: builder.action(x0, x1)),
    # 19. term ::= symbol => $0
    ('term', 1, lambda builder, x0: x0),
    # 20. term ::= symbol "?" => optional($0, $1)
    ('term', 2, lambda builder, x0, x1: builder.optional(x0, x1)),
    # 21. symbol ::= IDENT => ident($0)
    ('symbol', 1, lambda builder, x0: builder.ident(x0)),
    # 22. symbol ::= STR => str($0)
    ('symbol', 1, lambda builder, x0: builder.str(x0)),
]


class DefaultBuilder:
    def grammar(self, x0, x1): return ('grammar', x0, x1)
    def single(self, x0): return ('single', x0)
    def append(self, x0, x1): return ('append', x0, x1)
    def const_token(self, x0, x1, x2, x3, x4): return ('const_token', x0, x1, x2, x3, x4)
    def var_token(self, x0, x1, x2, x3): return ('var_token', x0, x1, x2, x3)
    def nt_defs_single(self, x0): return ('nt_defs_single', x0)
    def nt_defs_append(self, x0, x1): return ('nt_defs_append', x0, x1)
    def nt_def(self, x0, x1, x2, x3, x4, x5): return ('nt_def', x0, x1, x2, x3, x4, x5)
    def prod(self, x0, x1, x2): return ('prod', x0, x1, x2)
    def optional(self, x0, x1): return ('optional', x0, x1)
    def ident(self, x0): return ('ident', x0)
    def str(self, x0): return ('str', x0)
    def action(self, x0, x1): return ('action', x0, x1)


parse_grammar = runtime.make_parse_fn(
actions, ctns, reductions, error_codes, 0, DefaultBuilder)
