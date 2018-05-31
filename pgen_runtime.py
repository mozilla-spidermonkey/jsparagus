import collections


# A Reduction is a step in a production that produces an AST node from the most recently parsed symbols.
Reduction = collections.namedtuple("Reduction", "tag_name tag_index arg_count")


# A symbol in a production is one of these three things:

def is_nt(element):
    return isinstance(element, str) and element[:1].islower()

def is_terminal(element):
    return isinstance(element, str) and not is_nt(element)

def is_reduction(element):
    return isinstance(element, Reduction)


def parse(M, goal, tokens):
    """ Table-driven predictive parser with explicit reduction actions. """
    t = tokens.peek()
    stack = [goal]
    operands = []
    while stack:
        plan = stack.pop()
        if t == plan:
            operands.append(tokens.take(t))
            t = tokens.peek()
        elif is_nt(plan):
            k = M[plan, t]
            if k is None:
                raise ValueError("expected {!r}, got {!r}"
                                 .format(plan, t))
            stack += k
        elif is_reduction(plan):
            args = operands[-plan.arg_count:]
            del operands[-plan.arg_count:]
            operands.append((plan.tag_name, plan.tag_index, args))
        else:
            raise ValueError("expected {!r}, got {!r}"
                             .format(plan, t))
    tokens.take(None)
    assert len(operands) == 1
    return operands[0]


def make_parse_fn(M, goal):
    return lambda tokens: parse(M, goal, tokens)
