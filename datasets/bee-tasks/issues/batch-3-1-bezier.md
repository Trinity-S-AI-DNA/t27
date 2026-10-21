# Implement `evaluate` and `derivative` in specs/tri/math/bezier.t27

## Context

`specs/tri/math/bezier.t27` is the Bézier-curve module of the `tri` standard library
(`module TriBezier;`), sitting beside `polynomial.t27`, `matrix.t27` and `statistics.t27`
in `specs/tri/math/`. Its file header names the method it must use: *"De Casteljau
algorithm for stable evaluation"*. It declares two types — `Point { x : f64, y : f64 }`
and `BezierCurve { control : []Point, degree : usize }` — and two functions that are
currently signature-only stubs. `.t27` is hand-authored source; `t27c` compiles it out
to C/Rust/Verilog/Zig. You are writing source, not compiler output.

## Current state (commands actually run, 2026-09-09)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/math/bezier.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/math/bezier.t27 | head -1
bezier.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/math/bezier.t27
2
$ grep -cE '^    fn ' specs/tri/math/bezier.t27
2
$ grep -cE '^    test ' specs/tri/math/bezier.t27
4
$ grep -cE '^ +(return|let|var|if|for|while|match) ' specs/tri/math/bezier.t27
0
```

That last `0` is the point of this task: the file has two declared functions and not one
statement inside either of them.

## What to write

Both functions are declared exactly as below. Keep the signatures byte-for-byte; replace
only the `// TODO: Implement from .tri spec` line with a real body.

### 1. `fn evaluate(curve: *const BezierCurve, t: f64) -> Point`

```
    // evaluate(curve: *const BezierCurve, t: f64) → Point
    fn evaluate(curve: *const BezierCurve, t: f64) -> Point {
        // TODO: Implement from .tri spec
    }
```

Evaluate the curve at parameter `t` using **de Casteljau's algorithm**, not the Bernstein
polynomial sum — the header says so and it is the numerically stable form. Repeatedly
linearly interpolate neighbouring control points: given points `P[0..n]`, compute
`P'[i] = (1-t)*P[i] + t*P[i+1]` for `i` in `0..n-1`, shrinking the array by one each
round, until one point remains. That point is the return value. A degree-0 curve returns
`curve.control[0]`.

### 2. `fn derivative(curve: *const BezierCurve) -> BezierCurve`

```
    // derivative(curve: *const BezierCurve) → BezierCurve
    fn derivative(curve: *const BezierCurve) -> BezierCurve {
        // TODO: Implement from .tri spec
    }
```

Return the hodograph: a curve of degree `n - 1` whose control points are
`n * (P[i+1] - P[i])` for `i` in `0..n-1`. Set `.degree = curve.degree - 1` and keep the
`control.len == degree + 1` relation that the existing tests assert. A degree-0 curve has
a zero derivative — return a degree-0 curve at the origin rather than underflowing
`degree`.

### The existing tests are the specification

These four `test` blocks are already in the file and state the relations your code must
respect. Do not delete or weaken them.

```
    test degree_one_curve_has_two_control_points
        given curve = BezierCurve{.control=@constCast(&[_]Point{Point{.x=0.0,.y=0.0},Point{.x=4.0,.y=2.0}}),.degree=1}
        then curve.control.len == 2 and curve.control.len == curve.degree + 1

    test curve_endpoints_are_its_first_and_last_control_points
        given curve = BezierCurve{.control=@constCast(&[_]Point{Point{.x=0.0,.y=0.0},Point{.x=1.0,.y=2.0},Point{.x=2.0,.y=0.0}}),.degree=2}
        and first = curve.control[0]
        and last = curve.control[curve.degree]
        then @abs(last.x - 2.0) < 1e-12 and first.x < last.x

    test linear_curve_midpoint_is_the_average_of_its_control_points
        given p0 = Point{.x=0.0,.y=0.0}
        and p1 = Point{.x=4.0,.y=2.0}
        and mid_x = (p0.x + p1.x) / 2.0
        and mid_y = (p0.y + p1.y) / 2.0
        and x_ok = @abs(mid_x - 2.0) < 1e-12
        and y_ok = @abs(mid_y - 1.0) < 1e-12
        then x_ok and y_ok
```

Add **at least two new `test` blocks**, one per function you implement, in the file's own
style (`    test some_name` on its own line, then indented `given` / `when` / `then`
lines). Reasonable additions: `evaluate` at `t = 0` returns `control[0]`; `evaluate` at
`t = 1` returns `control[degree]`; `derivative` of a degree-1 curve has one control point
equal to `P1 - P0`.

### Style notes, measured

The parser accepts ordinary statement syntax inside a body. This was verified on a copy of
this exact file: `let n = curve.degree;`, `let mut x = 0.0;`, `if (n == 0) { return
curve.control[0]; }`, `for (0..n) |i| { ... }`, `return Point{ .x = x, .y = 0.0 };` all
parse with recovery-events 0. Use four-space indentation for `fn` and `test` lines to
match the rest of the file, and eight spaces for statements inside a body.

**There is no `.t27` interpreter.** Your tests will be parsed and counted, never executed.
Write them anyway — they are the house style for stating what a function must do — but do
not claim you ran them.

**Do not lean on `t27c typecheck`.** It returns `{"ok":true,"errors":0}` on English prose
and on Python files. It is not a gate.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 2 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/math/bezier.t27` prints 0
- 3. the real-code node count is at least 4
- 4. `t27c parse specs/tri/math/bezier.t27 | grep -c 'kind: TestBlock'` prints at least 6
- 5. `t27c count specs/tri/math/bezier.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `2 names, fns=2 nonempty=0, clean` | `2 names, fns=n nonempty=n (n>=2), clean` |
| 2 | stub marker | `2` | `0` |
| 3 | real-code nodes | `0` | `>= 4` |
| 4 | test blocks | `4` | `>= 6` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 2 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (evaluate|derivative)\(' specs/tri/math/bezier.t27
```

Must print `2`. It prints `2` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/math/bezier.t27 <<'EOF'
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

Today it prints `fns=2 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 2**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 2 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/math/bezier.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/math/bezier.t27
```

Today it prints `2`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/math/bezier.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `4`.

The bar is two nodes per stubbed function (2 x 2 = 4 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/math/bezier.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `6` — the 4 blocks already in the file
plus one new block for each of the 2 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/math/bezier.t27 | head -1
```

Today it prints `bezier.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 2 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/math/bezier.t27 prints 0: met
- 3. the real-code node count is at least 4: met
- 4. t27c parse specs/tri/math/bezier.t27 | grep -c 'kind: TestBlock' prints at least 6: met
- 5. t27c count specs/tri/math/bezier.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/math/bezier.t27
