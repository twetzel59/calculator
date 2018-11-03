# calculator
A WIP/experimental command-line calculator

# Implementation
As of yet, the idea is that the user may enter an expression and the calculator will evaluate it.
At the minimum, addition and multiplication will be supported. Hopefully, most common Algebra operations will be supported.

There are at least two ways to implement this:
* Recursively or iteratively simplify the expression until only a scalar is left.
* Parse into a tree and recursively simplify.
So, both methods rely on iterative simplification of the expression. The question is whether to parse into a tree structure or linear structure. Most programming languages use the tree approach since their syntax is naturally nested. This would make parenthesis easier. On the other hand, simple expressions are best evaluated by simplifying in-line. I have yet to determine which approach is better all in all.

# Goals
The user should be able to enter expressions like ``3 + 7 * 4`` and the calculator should evaluate the expression following order of operations. In other words, it should evaluate ``7 * 4`` first, and then add that quantity to ``3``.

# Far Goals
* I *might* support parenthesis, e.g. ``(3 + 7) * 4`` or ``(9 + 3) * 7 * (3 + 2)``.
* I *might* support some constants, e.g. ``E`` or ``PI``.
* I *might* try to support common functions, e.g. ``sin(PI)``.
* I *might* support variables, e.g:
```
>> A = 2
2
>> B = 3
3
>> A + B
5
```

**I don't expect that I'll have time to finish these far goals, as I am taking some heavy classes currently.** I do hope to acheieve the basic goal of evaluating simple arithmetic.
