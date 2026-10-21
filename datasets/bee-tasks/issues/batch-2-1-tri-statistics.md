# Implement the six stubbed functions in TriStatistics (specs/tri/math/statistics.t27)

## Context

`specs/tri/math/statistics.t27` is the descriptive-statistics module of the `tri` standard
library: the module every other spec is expected to call when it needs a mean, a spread, an
order statistic or a linear-correlation coefficient over a `[]f64` sample. Its header line is
`// Pearson correlation coefficient | φ² + 1/φ² = 3 | TRINITY`, which names the one non-obvious
function in the file. It sits beside `specs/tri/math/probability.t27` and
`specs/tri/math/measurement.t27` in the same `math` group, and unlike its neighbours it declares
no types at all — six free functions over plain float slices, nothing else. Every one of those
six is an empty stub today, so the module currently states signatures and nothing more.

**.t27 is hand-authored source.** It is compiled out to C/Rust/Verilog/Zig by
`/Users/playom/t27/target/release/t27c`. You are writing source, not compiler output. The
dialect is module-code: `module X;`, `use a::b;`, `fn name(arg : type) -> ret { ... }`, and
`test` blocks. Match the style already in the file.

## Exact current state (measured 2026-09-09, re-run these)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/math/statistics.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/math/statistics.t27 | head -1
statistics.t27: 5 node types

$ grep -c '// TODO: Implement from .tri spec' specs/tri/math/statistics.t27
6
```

The file is 76 lines and parses clean today. Do not regress that.

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. **Keep every signature
exactly as it stands** — same name, same parameter names, same parameter types, same return
type. These are the six, quoted verbatim from the file:

```t27
    fn mean(values: []f64) -> f64 {
    fn variance(values: []f64) -> f64 {
    fn std_dev(values: []f64) -> f64 {
    fn median(allocator: std.mem.Allocator, values: []f64) -> f64 {
    fn percentile(allocator: std.mem.Allocator, values: []f64, p: f64) -> f64 {
    fn correlation(x: []f64, y: []f64) -> f64 {
```

What each must do:

- **`mean`** — arithmetic mean, `sum / n`. Guard the empty slice (return `0.0`); dividing by zero
  is not a specification.
- **`variance`** — sample variance about the mean, i.e. divide the sum of squared deviations by
  `n - 1`, not `n`. A sample of fewer than two elements has no sample variance: return `0.0`.
  State the choice of divisor in a comment — it is the one thing a reader will want to check.
- **`std_dev`** — the square root of `variance(values)`. Call `variance`; do not re-derive it.
- **`median`** — the middle order statistic. It takes an allocator because it must not disturb
  the caller's slice: allocate a copy, sort the copy, read the middle (or the mean of the two
  middles for even `n`), free the copy. Empty slice returns `0.0`.
- **`percentile`** — the value at fraction `p` of the sorted sample, with `p` in `[0, 1]`. Same
  copy-and-sort discipline as `median`. Say in a comment which interpolation you use (nearest
  rank or linear between neighbours) and clamp `p` into range; an unstated convention here is
  the difference between two different functions.
- **`correlation`** — Pearson's r between `x` and `y`: covariance over the product of the two
  standard deviations. Return `0.0` when the lengths differ, when `n < 2`, or when either
  standard deviation is zero (r is undefined for a constant series — do not divide by zero).

The six existing test blocks are placeholders of the shape

```t27
    test mean_basic_case
        given input = default_input()
        when result = mean(input)
        then result != undefined
```

`default_input()` is not defined anywhere in the module, so these assert nothing. Leave them in
place and **add** a real `test` block per function underneath, in the file's own
`test name / given / when / then / and` style (no braces, no quoted test name — copy the layout
above). Each new test should pin a value a reader can check by hand: the mean of `{1.0, 3.0}` is
`2.0`, the variance of a constant sample is `0.0`, the median of a three-element sample is its
middle element, `correlation(x, x)` is `1.0`. There is **no t27 interpreter**, so these blocks
are read and counted, never executed — write them as the specification of the behaviour, and do
not claim you ran them.

### Parser hazards (measured — these silently destroy the file)

The bootstrap parser swallows declarations on two constructs *without* raising
`recovery-events`, so a file can look clean and still have lost half its functions:

- `switch` arms with a **qualified** pattern (`Foo.bar => ...`) — measured: `FnDecl` fell from 4
  to 2 with all three parse metrics still reporting 0. Use `if` / `else if` chains instead.
  Literal arms (`0 => ...`) are safe.
- The **anonymous struct literal** `.{ ... }` — measured: `recovery-events: 1` and `FnDecl` fell
  from 4 to 2.

Safe and measured-clean in bodies: `if`/`else if`/`else`, `while`, `for (xs) |x|`, `return`,
`const`/`var` locals, `try`, `catch`, `orelse`, `defer`, builtins like `@as` / `@intCast`,
slicing `xs[0..n]`, struct literals `T{ .f = v }`, and `and`/`or`. This is why criteria 3 and 5
below exist: run the real-code and node-type commands after every edit, not just
`t27c parse`.

Do **not** gate yourself on `t27c typecheck` — it returns `{"ok":true,"errors":0}` on English
prose and on Python files. It proves nothing.

### House law

Repo law L3 (PURITY) requires source to be ASCII-only with English identifiers. The file's
existing banner comments already contain non-ASCII decoration - leave those lines exactly as they
are, and keep every line **you** add pure ASCII. Law L4 (TESTABILITY) is why the added `test`
blocks are not optional.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 6 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/math/statistics.t27` prints 0
- 3. the real-code node count is at least 12
- 4. `t27c parse specs/tri/math/statistics.t27 | grep -c 'kind: TestBlock'` prints at least 12
- 5. `t27c count specs/tri/math/statistics.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `6 names, fns=6 nonempty=0, clean` | `6 names, fns=n nonempty=n (n>=6), clean` |
| 2 | stub marker | `6` | `0` |
| 3 | real-code nodes | `0` | `>= 12` |
| 4 | test blocks | `6` | `>= 12` |
| 5 | node types | `5` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 6 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (mean|variance|std_dev|median|percentile|correlation)\(' specs/tri/math/statistics.t27
```

Must print `6`. It prints `6` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/math/statistics.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/math/statistics.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/math/statistics.t27
```

Today it prints `6`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/math/statistics.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `12`.

The bar is two nodes per stubbed function (6 x 2 = 12 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/math/statistics.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `6`. It must print at least `12` — the 6 blocks already in the file
plus one new block for each of the 6 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/math/statistics.t27 | head -1
```

Today it prints `statistics.t27: 5 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/math/statistics.t27 prints 0: met
- 3. the real-code node count is at least 12: met
- 4. t27c parse specs/tri/math/statistics.t27 | grep -c 'kind: TestBlock' prints at least 12: met
- 5. t27c count specs/tri/math/statistics.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/math/statistics.t27
