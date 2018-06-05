import collections
import sys


# A Reduction is a step in a production that produces an AST node from the most recently parsed symbols.
Reduction = collections.namedtuple("Reduction", "tag_name tag_index arg_count")


# A symbol in a production is one of these three things:

def is_nt(element):
    return isinstance(element, str) and element[:1].islower()

def is_terminal(element):
    return isinstance(element, str) and not is_nt(element)

def is_reduction(element):
    return isinstance(element, Reduction)


ACCEPT = -0x7fffffffffffffff
ERROR = ACCEPT - 1


def parse(actions, ctns, reductions, entry_state, tokens):
    """ Table-driven LR parser. """
    t = tokens.peek()
    stack = [entry_state]  # alternates state-ids and AST nodes
    while True:
        state = stack[-1]
        action = actions[state].get(t, ERROR)
        # possible actions are: shift, reduce, accept, error; all encoded as integers
        if action >= 0:  # shift
            stack.append(tokens.take(t))
            stack.append(action)
            t = tokens.peek()
        elif action > ACCEPT:  # reduce
            r = reductions[-action - 1]
            n = r.arg_count
            start = -2 * n
            node = (r.tag_name, r.tag_index, stack[start::2])
            stack[start:] = [node, ctns[stack[start - 1]][r.tag_name]]
        elif action == ACCEPT:
            assert len(stack) == 3
            return stack[1]
        else:
            assert action == ERROR
            expected = set(actions[state].keys())
            if len(expected) < 2:
                raise ValueError("expected {!r}, got {!r}".format(list(expected)[0], t))
            else:
                raise ValueError("expected one of {!r}, got {!r}"
                                 .format(sorted(expected), t))


def make_parse_fn(actions, ctns, reductions, entry_state):
    return lambda tokens: parse(actions, ctns, reductions, entry_state, tokens)
