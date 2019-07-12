import pgen_runtime
from pgen_runtime import Apply

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
    {None: -7, 'nt': -7, 'goal': -7},

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
    {None: -8, 'nt': -8, 'goal': -8},

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
    {';': -21, '?': -21, '=>': -21, 'IDENT': -21, 'STR': -21},

    # 23. "nt" IDENT "{" STR
    {';': -22, '?': -22, '=>': -22, 'IDENT': -22, 'STR': -22},

    # 24. "nt" IDENT "{" prods
    {'}': 32, 'IDENT': 22, 'STR': 23},

    # 25. "nt" IDENT "{" prod
    {'}': -13, 'IDENT': -13, 'STR': -13},

    # 26. "nt" IDENT "{" terms
    {';': 34, '=>': 35, 'IDENT': 22, 'STR': 23},

    # 27. "nt" IDENT "{" term
    {';': -17, '=>': -17, 'IDENT': -17, 'STR': -17},

    # 28. "nt" IDENT "{" symbol
    {'?': 38, ';': -19, '=>': -19, 'IDENT': -19, 'STR': -19},

    # 29. "goal" "nt" IDENT "{"
    {'}': 39, 'IDENT': 22, 'STR': 23},

    # 30. "token" IDENT "=" STR
    {';': 41},

    # 31. "var" "token" IDENT ";"
    {'nt': -6, 'goal': -6, 'token': -6, 'var': -6},

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
    {';': -20, '=>': -20, 'IDENT': -20, 'STR': -20},

    # 39. "goal" "nt" IDENT "{" "}"
    {None: -10, 'nt': -10, 'goal': -10},

    # 40. "goal" "nt" IDENT "{" prods
    {'}': 44, 'IDENT': 22, 'STR': 23},

    # 41. "token" IDENT "=" STR ";"
    {'nt': -5, 'goal': -5, 'token': -5, 'var': -5},

    # 42. "nt" IDENT "{" terms "=>" IDENT
    {';': -23},

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

reductions = [
    ('grammar', 1, lambda builder, x0: builder.grammar_P0(None, x0)),
    ('grammar', 2, lambda builder, x0, x1: builder.grammar_P0(x0, x1)),
    ('token_defs', 1, lambda builder, x0: builder.token_defs_P0(x0)),
    ('token_defs', 2, lambda builder, x0, x1: builder.token_defs_P1(x0, x1)),
    ('token_def', 5, lambda builder, x0, x1, x2, x3, x4: builder.token_def_P0(x0, x1, x2, x3, x4)),
    ('token_def', 4, lambda builder, x0, x1, x2, x3: builder.token_def_P1(x0, x1, x2, x3)),
    ('nt_defs', 1, lambda builder, x0: builder.nt_defs_P0(x0)),
    ('nt_defs', 2, lambda builder, x0, x1: builder.nt_defs_P1(x0, x1)),
    ('nt_def', 4, lambda builder, x0, x1, x2, x3: builder.nt_def_P0(None, x0, x1, x2, None, x3)),
    ('nt_def', 5, lambda builder, x0, x1, x2, x3, x4: builder.nt_def_P0(x0, x1, x2, x3, None, x4)),
    ('nt_def', 5, lambda builder, x0, x1, x2, x3, x4: builder.nt_def_P0(None, x0, x1, x2, x3, x4)),
    ('nt_def', 6, lambda builder, x0, x1, x2, x3, x4, x5: builder.nt_def_P0(x0, x1, x2, x3, x4, x5)),
    ('prods', 1, lambda builder, x0: builder.prods_P0(x0)),
    ('prods', 2, lambda builder, x0, x1: builder.prods_P1(x0, x1)),
    ('prod', 2, lambda builder, x0, x1: builder.prod_P0(x0, None, x1)),
    ('prod', 3, lambda builder, x0, x1, x2: builder.prod_P0(x0, x1, x2)),
    ('terms', 1, lambda builder, x0: builder.terms_P0(x0)),
    ('terms', 2, lambda builder, x0, x1: builder.terms_P1(x0, x1)),
    ('term', 1, lambda builder, x0: builder.term_P0(x0)),
    ('term', 2, lambda builder, x0, x1: builder.term_P1(x0, x1)),
    ('symbol', 1, lambda builder, x0: builder.symbol_P0(x0)),
    ('symbol', 1, lambda builder, x0: builder.symbol_P1(x0)),
    ('action', 2, lambda builder, x0, x1: builder.action_P0(x0, x1)),
]

class DefaultBuilder:
    def grammar_P0(self, *args): return ('grammar', 0, list(args))
    def grammar_P0(self, *args): return ('grammar', 0, list(args))
    def token_defs_P0(self, *args): return ('token_defs', 0, list(args))
    def token_defs_P1(self, *args): return ('token_defs', 1, list(args))
    def token_def_P0(self, *args): return ('token_def', 0, list(args))
    def token_def_P1(self, *args): return ('token_def', 1, list(args))
    def nt_defs_P0(self, *args): return ('nt_defs', 0, list(args))
    def nt_defs_P1(self, *args): return ('nt_defs', 1, list(args))
    def nt_def_P0(self, *args): return ('nt_def', 0, list(args))
    def nt_def_P0(self, *args): return ('nt_def', 0, list(args))
    def nt_def_P0(self, *args): return ('nt_def', 0, list(args))
    def nt_def_P0(self, *args): return ('nt_def', 0, list(args))
    def prods_P0(self, *args): return ('prods', 0, list(args))
    def prods_P1(self, *args): return ('prods', 1, list(args))
    def prod_P0(self, *args): return ('prod', 0, list(args))
    def prod_P0(self, *args): return ('prod', 0, list(args))
    def terms_P0(self, *args): return ('terms', 0, list(args))
    def terms_P1(self, *args): return ('terms', 1, list(args))
    def term_P0(self, *args): return ('term', 0, list(args))
    def term_P1(self, *args): return ('term', 1, list(args))
    def symbol_P0(self, *args): return ('symbol', 0, list(args))
    def symbol_P1(self, *args): return ('symbol', 1, list(args))
    def action_P0(self, *args): return ('action', 0, list(args))


parse_grammar = pgen_runtime.make_parse_fn(actions, ctns, reductions, 0, DefaultBuilder)
