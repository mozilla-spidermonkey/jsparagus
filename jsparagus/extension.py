"""Data structure extracted from parsing the EDSL which are added within the
Rust code."""

import collections
from .utils import keep_until

class ImplFor(collections.namedtuple("ImplFor", "param trait for_type")):
    def __new__(cls, param, trait, for_type):
        self = super(ImplFor, cls).__new__(cls, param, trait, for_type)
        return self
    def __eq__(self, other):
        return isinstance(other, ImplFor) and super(ImplFor, self).__eq__(other)

def eq_productions(grammar, prod1, prod2):
    s1 = tuple(e for e in prod1.body if grammar.is_shifted_element(e))
    s2 = tuple(e for e in prod2.body if grammar.is_shifted_element(e))
    return s1 == s2

def merge_productions(grammar, prod1, prod2):
    # Consider all shifted elements as non-moveable elements, and insert other
    # around these.
    assert eq_productions(grammar, prod1, prod2)
    l1 = list(prod1.body)
    l2 = list(prod2.body)
    body = []
    while l1 != [] and l2 != []:
        front1 = list(keep_until(l1, grammar.is_shifted_element))
        front2 = list(keep_until(l2, grammar.is_shifted_element))
        assert front1[-1] == front2[-1]
        l1 = l1[len(front1):]
        l2 = l2[len(front2):]
        if len(front1) == 1:
            body = body + front2
        elif len(front2) == 1:
            body = body + front1
        else:
            assert len(front1) == 1 or len(front2) == 1, """
            We do not know how to sort operations yet.
            """
    return prod1.copy_with(body=body)

class ExtPatch(collections.namedtuple("ExtPatch", "prod")):
    "Patch an existing grammar rule by adding Code"
    def __new__(cls, prod):
        self = super(ExtPatch, cls).__new__(cls, prod)
        return self
    def __eq__(self, other):
        return isinstance(other, ExtPatch) and super(ExtPatch, self).__eq__(other)

    def apply_patch(self, filename, grammar, nonterminals):
        (name, ns, nt_def) = self.prod
        gnt_def = nonterminals[name]
        # Find a matching production in the grammar.
        assert nt_def.params == gnt_def.params
        new_rhs_list = []
        assert len(nt_def.rhs_list) == 1
        pp = nt_def.rhs_list[0]
        applied = False
        for gp in gnt_def.rhs_list:
            if eq_productions(grammar, gp, pp):
                gp = merge_productions(grammar, gp, pp)
                applied = True
            new_rhs_list.append(gp)
        if not applied:
            raise ValueError("{}: Unable to find a matching production for {} in the grammar:\n {}"
                             .format(filename, name, grammar.production_to_str(name, pp)))
        result = gnt_def.with_rhs_list(new_rhs_list)
        nonterminals[name] = result

class GrammarExtension:
    """A collection of grammar extensions, with added code, added traits for the
    action functions.

    """

    def __init__(self, target, grammar, filename):
        self.target = target
        self.grammar = grammar
        self.filename = filename
        pass
    def __repr__(self):
        return "GrammarExtension({}, {})".format(repr(self.target), repr(self.grammar))

    def apply_patch(self, grammar, nonterminals):
        # A grammar extension is composed of multiple production patches.
        for ext in self.grammar:
            if isinstance(ext, ExtPatch):
                ext.apply_patch(self.filename, grammar, nonterminals)

