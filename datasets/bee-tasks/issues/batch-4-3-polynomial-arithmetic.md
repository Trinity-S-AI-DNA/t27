# Implement the six stubbed functions in `TriPolynomial` (specs/tri/math/polynomial.t27)

## Context

`specs/tri/math/polynomial.t27` is the dense univariate polynomial module of the `tri` math family
(it sits beside `math.t27`, `probability.t27`, `measurement.t27`). It declares one type,
`Polynomial { coeffs : []f64, allocator : std.mem.Allocator }` — a dense coefficient vector that
owns its storage, which is why the allocator travels inside the struct and why there is a `deinit`.
The header comment is *"Free polynomial"*: ownership is the point of the module. The coefficient
convention is not left open — the second test in the file pins `coeffs[i]` to the coefficient of
`x^i` (ascending degree) and shows the Horner evaluation that follows from it. Six functions are
declared — `init`, `eval`, `add`, `multiply`, `derivative`, `deinit` — and all six are stubs.

`.t27` is a hand-authored source language, not compiler output. You are writing source.

## Exact current state

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/math/polynomial.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

The file parses clean today. **Do not regress that.**

```
$ ./target/release/t27c count specs/tri/math/polynomial.t27 | head -1
polynomial.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/math/polynomial.t27
6
```

```
$ grep -cE '^    fn ' specs/tri/math/polynomial.t27
6
```

Six declared functions, six stubs: every function in this module is a signature with an empty body.

## What to write

Fill each body. **Keep every signature byte-for-byte as it is** — same name, same parameter names,
same arrow-return type. Only the body changes. The signatures, verbatim from the file:

```
    fn init(allocator: std.mem.Allocator, coeffs: []f64) -> Polynomial {
    fn eval(p: *Polynomial, x: f64) -> f64 {
    fn add(a: *Polynomial, b: *Polynomial, allocator: std.mem.Allocator) -> Polynomial {
    fn multiply(a: *Polynomial, b: *Polynomial, allocator: std.mem.Allocator) -> Polynomial {
    fn derivative(p: *Polynomial, allocator: std.mem.Allocator) -> Polynomial {
    fn deinit(p: *Polynomial) -> void {
```

What each must do:

- `init` — build a `Polynomial` holding `coeffs` and `allocator`. An empty `coeffs` slice is legal
  and is the zero polynomial (first test below).
- `eval` — evaluate at `x` by **Horner's method**, walking `coeffs` from the highest index down:
  `acc = 0; for i from len-1 down to 0: acc = acc * x + coeffs[i]`. The zero polynomial evaluates
  to `0.0`. Horner is not a suggestion — the second test computes the expected value that way.
- `add` — allocate `max(a.len, b.len)` coefficients from `allocator` and sum term by term, treating
  a missing term in the shorter operand as `0.0`. The result owns its own storage; neither operand
  is mutated.
- `multiply` — allocate `a.len + b.len - 1` coefficients (0 when either operand is empty) and
  accumulate `result[i + j] += a[i] * b[j]`. Initialise the buffer to `0.0` before accumulating.
- `derivative` — allocate `p.coeffs.len - 1` coefficients with `result[i] = coeffs[i + 1] * (i + 1)`.
  A constant or the zero polynomial has an empty derivative, not a length-1 one; that boundary is
  pinned by the third test.
- `deinit` — free `p.coeffs` through `p.allocator` and leave `p.coeffs` an empty slice so a second
  `deinit` is harmless. This is the function the header comment ("Free polynomial") is named for.

The three existing tests are the specification and must not be edited. Verbatim, the two that
constrain the arithmetic:

```
    test coefficients_are_stored_in_ascending_degree
        // Verify: coeffs[i] is the coefficient of x^i. For 1 + 2x + 3x^2 the
        // Horner evaluation at x = 2 is 1 + 2*(2 + 2*3) = 17; under the
        // opposite (descending) convention the same slice would give 11
        given coeffs = [_]f64{ 1.0, 2.0, 3.0 }
        and p = Polynomial{ .coeffs = @constCast(&coeffs), .allocator = std.testing.allocator }
        when value = p.coeffs[0] + 2.0 * (p.coeffs[1] + 2.0 * p.coeffs[2])
        and error_ = @abs(value - 17.0)
        then error_ < 1.0e-12

    test degree_is_one_below_the_coefficient_count
        // Verify: a cubic needs four slots, so derivative() must return three
        given coeffs = [_]f64{ 4.0, 3.0, 2.0, 1.0 }
        and p = Polynomial{ .coeffs = @constCast(&coeffs), .allocator = std.testing.allocator }
        when degree = p.coeffs.len - 1
        then degree == 3
```

Ascending degree, and `derivative` of a length-4 slice has length 3. Do not invert either.

Add one `test` block per function you implement, appended to the existing test section. Match the
house style already in this file and in `specs/fpga/uart.t27` — bare name, then `given` / `and` /
`when` / `then` lines, no braces. Because these are floats, compare with a tolerance the way the
existing test does (`@abs(value - expected) < 1.0e-12`), never with `==`:

```
    test eval_of_the_zero_polynomial_is_zero
        given coeffs = [_]f64{}
        and p = Polynomial{ .coeffs = @constCast(&coeffs), .allocator = std.testing.allocator }
        when value = eval(&p, 3.0)
        then @abs(value) < 1.0e-12
```

Body style: `specs/fpga/uart.t27` lines 60-109 is the house form of a filled `-> type { }` body.
Constructs measured to parse clean here: `var`/`const` locals, `while (cond) { }`, `if (cond) { }`,
`return expr;`, indexing, field access, struct literals, `&x`.

**Do not** rewrite the `Polynomial` declaration, the header comment, or the three existing tests.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 6 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/math/polynomial.t27` prints 0
- 3. the real-code node count is at least 12
- 4. `t27c parse specs/tri/math/polynomial.t27 | grep -c 'kind: TestBlock'` prints at least 9
- 5. `t27c count specs/tri/math/polynomial.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `6 names, fns=6 nonempty=0, clean` | `6 names, fns=n nonempty=n (n>=6), clean` |
| 2 | stub marker | `6` | `0` |
| 3 | real-code nodes | `0` | `>= 12` |
| 4 | test blocks | `3` | `>= 9` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 6 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|eval|add|multiply|derivative|deinit)\(' specs/tri/math/polynomial.t27
```

Must print `6`. It prints `6` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/math/polynomial.t27 <<'EOF'
import re,sys
s=open(sys.argv[1],encoding='utf-8').read(); n=e=0
for m in re.finditer(r'^[ \t]*(?:pub[ \t]+)?fn[ \t]+\w+',s,re.M):
    i=s.find('{',m.end())
    if i<0: continue
    d=0
    for j in range(i,len(s)):
        if s[j]=='{': d+=1
        elif s[j]=='}':
            d-=1
            if d==0: break
    n+=1
    if re.sub(r'//[^\n]*','',s[i+1:j]).strip(): e+=1
print('fns=%d nonempty=%d'%(n,e))
EOF
```

Today it prints `fns=6 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 6**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 6 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/math/polynomial.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/math/polynomial.t27
```

Today it prints `6`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/math/polynomial.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `12`.

The bar is two nodes per stubbed function (6 x 2 = 12 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/math/polynomial.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `3`. It must print at least `9` — the 3 blocks already in the file
plus one new block for each of the 6 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/math/polynomial.t27 | head -1
```

Today it prints `polynomial.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
the stub-free, parse-clean files in `specs/` are at or above that number, and the
median across them is 17.5.

`| head -1` is not optional. The rest of the `t27c count` table is **not reproducible**:
rows with equal counts print in a different order on every run (measured: 3 distinct
orderings in 8 consecutive runs over the same unchanged file), and the table stops
after 15 rows with `... and N more`. 14 files in `specs/` contain test blocks that
`t27c count | grep TestBlock` cannot see for exactly that reason — and they are the
richly-implemented ones, which is the state this task is trying to reach. Never read
a number off that table; the first line is the only stable part of the output.

### What is not evidence

- **`t27c typecheck` is not a gate.** It returns `{"ok":true,"errors":0}` on 500 words
  of English prose and on a Python file. Do not cite it for anything.
- **There is no t27 interpreter.** Your test blocks will be parsed and counted, never
  executed. Write them — a `test` block beside a function is how this corpus states
  what a function must do — but never claim you ran one or that one passed.

### Your VERDICT

The first thing in your final message must be the block below, with each criterion in
the issue's own words on one line, ending in exactly `met`, `unmet` or
`could-not-check`:

```
## VERDICT
- 1. all 6 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/math/polynomial.t27 prints 0: met
- 3. the real-code node count is at least 12: met
- 4. t27c parse specs/tri/math/polynomial.t27 | grep -c 'kind: TestBlock' prints at least 9: met
- 5. t27c count specs/tri/math/polynomial.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/math/polynomial.t27
