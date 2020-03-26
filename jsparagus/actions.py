import itertools
from .ordered import OrderedFrozenSet
from .grammar import InitNt, Nt, Some

class Action:
    __slots__ = [
        "read",    # Set of trait names which are consumed by this action.
        "write",   # Set of trait names which are mutated by this action.
        "_hash",   # Cached hash.
    ]

    def __init__(self, read, write):
        assert isinstance(read, list)
        assert isinstance(write, list)
        self.read = read
        self.write = write
        self._hash = None

    def is_inconsistent(self):
        """Returns True whether this action is inconsistent. An action can be
        inconsistent if the parameters it is given cannot be evaluated given
        its current location in the parse table. Such as CheckNotOnNewLine.
        """
        return False

    def is_condition(self):
        "Unordered condition, which accept or not to reach the next state."
        return False

    def condition(self):
        "Return the conditional action."
        raise TypeError("Action::condition_flag not implemented")

    def update_stack(self):
        """Change the parser stack, and resume at a different location. If this function
        is defined, then the function reduce_with should be implemented."""
        return False

    def reduce_with(self):
        "Returns the non-terminal with which this action is reducing with."
        assert self.update_stack()
        raise TypeError("Action::reduce_to not implemented.")

    def shifted_action(self, shifted_term):
        "Returns the same action shifted by a given amount."
        return self

    def contains_accept(self):
        "Returns whether the current action stops the parser."
        return False

    def maybe_add(self, other):
        """Implement the fact of concatenating actions into a new action which can have
        a single state instead of multiple states which are following each others."""
        actions = []
        if isinstance(self, Seq):
            actions.extend(list(self.actions))
        else:
            actions.append(self)
        if isinstance(other, Seq):
            actions.extend(list(other.actions))
        else:
            actions.append(other)
        if any([a.is_condition() for a in actions]):
            return None
        if any([a.update_stack() for a in actions[:-1]]):
            return None
        return Seq(actions)

    def __eq__(self, other):
        if self.__class__ != other.__class__:
            return False
        if sorted(self.read) != sorted(other.read):
            return False
        if sorted(self.write) != sorted(other.write):
            return False
        for s in self.__slots__:
            if getattr(self, s) != getattr(other, s):
                return False
        return True

    def __hash__(self):
        if self._hash is not None:
            return self._hash
        def hashed_content():
            yield self.__class__
            yield "rd"
            for alias in self.read:
                yield alias
            yield "wd"
            for alias in self.write:
                yield alias
            for s in self.__slots__:
                yield repr(getattr(self, s))
        self._hash = hash(tuple(hashed_content()))
        return self._hash

    def __lt__(self, other):
        return hash(self) < hash(other)

    def __repr__(self):
        return str(self)

class Reduce(Action):
    """Define a reduce operation which pops N elements of he stack and pushes one
    non-terminal. The replay attribute of a reduce action corresponds to the
    number of stack elements which would have to be popped and pushed again
    using the parser table after reducing this operation. """
    __slots__ = 'nt', 'replay', 'pop'

    def __init__(self, nt, pop, replay = 0):
        name = nt.name
        if isinstance(name, InitNt):
            name = "Start_" + name.goal.name
        super().__init__([], ["nt_" + name])
        self.nt = nt    # Non-terminal which is reduced
        self.pop = pop  # Number of stack elements which should be replayed.
        self.replay = replay # List of terms to shift back

    def __str__(self):
        return "Reduce({}, {}, {})".format(self.nt, self.pop, self.replay)

    def update_stack(self):
        return True

    def reduce_with(self):
        return self

    def shifted_action(self, shifted_term):
        return Reduce(self.nt, self.pop, replay = self.replay + 1)

class Lookahead(Action):
    """Define a Lookahead assertion which is meant to either accept or reject
    sequences of terminal/non-terminals sequences."""
    __slots__ = 'terms', 'accept'

    def __init__(self, terms, accept):
        assert isinstance(terms, (OrderedFrozenSet, frozenset))
        assert all(not isinstance(t, Nt) for t in terms)
        assert isinstance(accept, bool)
        super().__init__([], [])
        self.terms = terms
        self.accept = accept

    def is_inconsistent(self):
        # A lookahead restriction cannot be encoded in code, it has to be
        # solved using fix_with_lookahead.
        return True

    def is_condition(self):
        return True

    def condition(self):
        return self

    def __str__(self):
        return "Lookahead({}, {})".format(self.terms, self.accept)

    def shifted_action(self, shifted_term):
        if isinstance(shifted_term, Nt):
            return True
        if shifted_term in self.terms:
            return self.accept
        return not self.accept

class CheckNotOnNewLine(Action):
    """Check whether the terminal at the given stack offset is on a new line or
    not. If not this would produce an Error, otherwise this rule would be
    shifted."""
    __slots__ = 'offset',

    def __init__(self, offset = 0):
        # assert offset >= -1 and "Smaller offsets are not supported on all backends."
        super().__init__([], [])
        self.offset = offset

    def is_inconsistent(self):
        # We can only look at stacked terminals. Having an offset of 0 implies
        # that we are looking for the next terminal, which is not yet shifted.
        # Therefore this action is inconsistent as long as the terminal is not
        # on the stack.
        return self.offset >= 0

    def is_condition(self):
        return True

    def condition(self):
        return self

    def shifted_action(self, shifted_term):
        if isinstance(shifted_term, Nt):
            return True
        return CheckNotOnNewLine(self.offset - 1)

    def __str__(self):
        return "CheckNotOnNewLine({})".format(self.offset)

class FilterFlag(Action):
    """Define a filter which check for one value of the flag, and continue to the
    next state if the top of the flag stack matches the expected value."""
    __slots__ = 'flag', 'value'

    def __init__(self, flag, value):
        super().__init__(["flag_" + flag], [])
        self.flag = flag
        self.value = value

    def is_condition(self):
        return True

    def condition(self):
        return self

    def __str__(self):
        return "FilterFlag({}, {})".format(self.flag, self.value)

class PushFlag(Action):
    """Define an action which pushes a value on a stack dedicated to the flag. This
    other stack correspond to another parse stack which live next to the
    default state machine and is popped by PopFlag, as-if this was another
    reduce action. This is particularly useful to raise the parse table from a
    LR(0) to an LR(k) without needing as much state duplications."""
    __slots__ = 'flag', 'value'

    def __init__(self, flag, value):
        super().__init__([], ["flag_" + flag])
        self.flag = flag
        self.value = value

    def __str__(self):
        return "PushFlag({}, {})".format(self.flag, self.value)

class PopFlag(Action):
    """Define an action which pops a flag from the flag bit stack."""
    __slots__ = 'flag',

    def __init__(self, flag):
        super().__init__(["flag_" + flag], ["flag_" + flag])
        self.flag = flag

    def __str__(self):
        return "PopFlag({})".format(self.flag)

class FunCall(Action):
    """Define a call method operation which reads N elements of he stack and
    pushpathne non-terminal. The replay attribute of a reduce action correspond
    to the number of stack elements which would have to be popped and pushed
    again using the parser table after reducing this operation. """
    __slots__ = 'trait', 'method', 'offset', 'args', 'fallible', 'set_to'

    def __init__(self, method, args,
                 trait = "AstBuilder",
                 fallible = False,
                 set_to = "val",
                 offset = 0,
                 alias_read = [],
                 alias_write = []):
        super().__init__(alias_read, alias_write)
        self.trait = trait       # Trait on which this method is implemented.
        self.method = method     # Method and argument to be read for calling it.
        self.fallible = fallible # Whether the function call can fail.
        self.offset = offset     # Offset to add to each argument offset.
        self.args = args         # Tuple of arguments offsets.
        self.set_to = set_to     # Temporary variable name to set with the result.

    def __str__(self):
        return "{} = {}::{}({}){} [off: {}]".format(
            self.set_to, self.trait, self.method,
            ", ".join(map(str, self.args)),
            self.fallible and '?' or '',
            self.offset)

    def __repr__(self):
        return "FunCall({})".format(', '.join(map(repr, [
            self.trait, self.method, self.fallible, self.read, self.write,
            self.args, self.set_to, self.offset
        ])))

    def contains_accept(self):
        return self.method == "accept"

    def map_args(self, f):
        return FunCall(self.method, tuple(f(self.offset, a) for a in self.args),
                       trait = self.trait,
                       fallible = self.fallible,
                       set_to = self.set_to,
                       offset = 0,
                       alias_read = self.read,
                       alias_write = self.write)

    def replace_set_to(self, name):
        return FunCall(self.method, self.args,
                       trait = self.trait,
                       fallible = self.fallible,
                       set_to = name,
                       offset = self.offset,
                       alias_read = self.read,
                       alias_write = self.write)

    def shifted_action(self, shifted_term):
        return FunCall(self.method, self.args,
                       trait = self.trait,
                       fallible = self.fallible,
                       set_to = self.set_to,
                       offset = self.offset + 1,
                       alias_read = self.read,
                       alias_write = self.write)

class Seq(Action):
    """Aggregate multiple actions in one sequence. Note, that the aggregated
    actions should not contain any condition or action which are mutating the
    state. Only the last action aggregated can update the parser stack"""
    __slots__ = 'actions',

    def __init__(self, actions):
        assert isinstance(actions, list)
        read = [ rd for a in actions for rd in a.read ]
        write = [ wr for a in actions for wr in a.write ]
        super().__init__(read, write)
        self.actions = tuple(actions)   # Ordered list of actions to execute.
        assert all([not a.is_condition() for a in actions[1:]])
        assert all([not a.update_stack() for a in actions[:-1]])

    def __str__(self):
        return "{{ {} }}".format("; ".join(map(str, self.actions)))

    def __repr__(self):
        return "Seq({})".format(repr(self.actions))

    def is_condition(self):
        return self.actions[0].is_condition()

    def condition(self):
        return self.actions[0]

    def update_stack(self):
        return self.actions[-1].update_stack()

    def reduce_with(self):
        return self.actions[-1].reduce_with()

    def shifted_action(self, shift):
        actions = list(map(lambda a: a.shifted_action(shift), self.actions))
        return Seq(actions)

    def contains_accept(self):
        return any(a.contains_accept() for a in self.actions)

class SeqBuilder:
    """Aggregate multiple actions in one sequence. Reduce actions added to this
    sequence are implicitly removed except for the last one but are still
    considered for rewriting arguments of FunCall. """

    def __init__(self):
        # offset_stack is used for rewritting offsets which are matching the
        # value of non-terminals.
        self.offset_stack = [1] # list(reversed(range(1, 16)))
        self.popped = 0
        self.replay = []
        # Until a Reduce action is seen, actions are stashed in the
        # stashed_actions list.
        self.actions = []
        self.stashed_actions = []
        self.last_reduce = None

    def ensure_stack(self, n):
        # Always add margin such that the first element is always an integer.
        n = n + 2 - len(self.offset_stack)
        if n < 0:
            return
        base = self.offset_stack[0] + 1
        self.offset_stack = list(itertools.chain(reversed(range(base, base + n)), self.offset_stack))

    def args_rewrite(self, offset, a):
        if isinstance(a, int):
            a = a + offset
            self.ensure_stack(a)
            # Return the variable name associated with the value pushed by
            # the shifted token.
            return self.offset_stack[-a]
        if isinstance(a, str):
            return a
        if isinstance(a, Some):
            return Some(self.args_rewrite(offset, a.inner))
        return a

    def can_shift(self, term):
        if not isinstance(term, Action):
            return len(self.replay) > 0
        assert not term.contains_accept()
        return not term.is_condition() or self.actions == []

    def shift(self, term):
        assert self.can_shift(term)
        if not isinstance(term, Action):
            # Push the token value of the lookahead, as a stack offset.
            self.offset_stack.append(self.replay.pop())
            self.popped -= 1
        elif isinstance(term, FunCall):
            term = term.map_args(self.args_rewrite)
            self.stashed_actions.append(term)
        elif isinstance(term, Reduce):
            self.ensure_stack(term.replay + term.pop)
            # Note, if we consume a reduce action before consuming all the
            # lookahead of previous reduce actions, the final reduce should
            # still have the remaining number of lookahead terms to replay.
            replay = term.replay
            while replay > 0:
                self.replay.append(self.offset_stack.pop())
                self.popped += 1
                replay -= 1

            # Remove all the terms pop-ed by all reduce actions seen until now.
            pop = term.pop
            while pop > 0:
                self.offset_stack.pop()
                self.popped += 1
                pop -= 1

            # New reduce action to be added if this is the last one.
            self.last_reduce = Reduce(term.nt, self.popped - len(self.replay), len(self.replay))

            # Move stashed actions to the list of actions to appear in this
            # sequence. The reason being that we end with a reduce action.
            # Therefore as we do not want code to be executed twice, we do not
            # want to append FunCall coming from the path where we reduced
            # into.
            self.actions.extend(self.stashed_actions)
            self.stashed_actions = []

            # The value of the non-terminal pushed on the stack after a
            # replayed action is going to be stored in the "value" variable.
            name = "action" + str(len(self.actions))
            self.popped -= 1
            if len(self.actions) >= 1 and isinstance(self.actions[-1], FunCall):
                self.offset_stack.append(name)
                self.actions[-1] = self.actions[-1].replace_set_to(name)
            else:
                self.offset_stack.append(len(self.replay) + 1)

        elif isinstance(term, Seq):
            for act in term.actions:
                self.shift(act)
        else:
            raise ValueError("Unsupported Action: {}".format(str(term)))

    def finish(self):
        if self.last_reduce is not None:
            if isinstance(self.actions[-1], FunCall):
                self.actions[-1] = self.actions[-1].replace_set_to("value")
            self.actions.append(self.last_reduce)
        if len(self.actions) == 1:
            return self.actions[0]
        return Seq(self.actions)
