# Implement `forward` and `backward` in specs/ml/activation/gelu_activation.t27

## Context

`specs/ml/activation/gelu_activation.t27` is the GELU activation of the ML layer
(`module Gelu;`), one of ten activation modules in `specs/ml/activation/` alongside
`relu_activation.t27`, `sigmoid_activation.t27`, `silu_swish_activation.t27` and
`softmax.t27`. Its header states the definition it must implement: *"Based on cumulative
distribution function"* — GELU(x) = x·Φ(x), where Φ is the standard normal CDF. The module
already declares the two constants that formula needs, `SQRT_2_OVER_PI = 0.7978845608` and
`SQRT_PI_OVER_2 = 1.2533141373`, and a `GELUConfig { approximate : bool }` that selects
between the exact `erf` form and the tanh approximation. (Its sibling
`gelu_approx_activation.t27` is the approximation-only variant with a `TANH_COEF`
constant; this file must support **both** branches.) Both functions are signature-only
stubs. `.t27` is hand-authored source; `t27c` compiles it out to C/Rust/Verilog/Zig.

## Current state (commands actually run, 2026-09-09)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/ml/activation/gelu_activation.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/ml/activation/gelu_activation.t27 | head -1
gelu_activation.t27: 9 node types
```

```
$ grep -c 'TODO: Implement' specs/ml/activation/gelu_activation.t27
2
$ grep -cE '^    fn ' specs/ml/activation/gelu_activation.t27
2
$ grep -cE '^    test ' specs/ml/activation/gelu_activation.t27
4
$ grep -cE '^ +(return|let|var|if|for|while|match) ' specs/ml/activation/gelu_activation.t27
0
```

Two constants and two functions declared; zero statements in either body.

## What to write

Keep both signatures byte-for-byte; replace only each `// TODO: Implement from .tri spec`.
Both are element-wise slice kernels: they write into a caller-provided output slice and
return `void`, so allocate nothing.

### 1. `fn forward(input: []const f32, output: []f32, config: GELUConfig) -> void`

```
    // forward(input: []const f32, output: []f32, config: GELUConfig) → void
    fn forward(input: []const f32, output: []f32, config: GELUConfig) -> void {
        // TODO: Implement from .tri spec
    }
```

For each index `i`, write `output[i] = gelu(input[i])`, branching on
`config.approximate`:

- **exact** (`approximate == false`): `0.5 * x * (1 + erf(x / sqrt(2)))`. `sqrt(2)` is
  `1.4142135624`; equivalently, `x * Φ(x)` with Φ the normal CDF, which is the form the
  file header names.
- **approximate** (`approximate == true`): the Hendrycks–Gimpel tanh form
  `0.5 * x * (1 + tanh(SQRT_2_OVER_PI * (x + 0.044715 * x^3)))`. Use the declared
  `SQRT_2_OVER_PI` constant rather than re-typing the literal — that constant exists for
  this expression and two of the tests below check its value.

Loop over `input.len`; do not read past the end of `output`.

### 2. `fn backward(grad_output: []const f32, input: []const f32, grad_input: []f32, config: GELUConfig) -> void`

```
    // backward(grad_output: []const f32, input: []const f32, grad_input: []f32, config: GELUConfig) → void
    fn backward(grad_output: []const f32, input: []const f32, grad_input: []f32, config: GELUConfig) -> void {
        // TODO: Implement from .tri spec
    }
```

Element-wise `grad_input[i] = grad_output[i] * dGELU/dx (input[i])`, on the **same branch**
as `forward` — the derivative of the exact form is
`Φ(x) + x * φ(x)`, where `φ(x) = exp(-x²/2) / sqrt(2π)` is the normal PDF, and
`1 / sqrt(2π)` is `1 / (SQRT_PI_OVER_2 * 2)`; the derivative of the tanh form differentiates
that expression directly. `SQRT_PI_OVER_2` is declared for the normalisation factor here —
use it. A `backward` that silently uses the exact derivative while `forward` used the
approximation is wrong even though it is close; branch on `config.approximate` in both.

### The existing tests are the specification

Already in the file. Do not delete or weaken them, and in particular do not "round" either
constant — the first test fails if their product moves off 1.

```
    test gelu_sqrt_constants_are_reciprocals
        given product = SQRT_2_OVER_PI * SQRT_PI_OVER_2
        then @abs(product - 1.0) < 0.0000005

    test gelu_sqrt_2_over_pi_squares_to_2_over_pi
        given squared = SQRT_2_OVER_PI * SQRT_2_OVER_PI
        then @abs(squared - 0.63661977) < 0.0000003

    test gelu_config_records_the_approximation_choice
        given exact = GELUConfig{.approximate=false}
        and approx = GELUConfig{.approximate=true}
        then !exact.approximate
        and approx.approximate
```

Add **at least two new `test` blocks**, one per function, in the file's own style
(`    test some_name` on its own line, then indented `given` / `when` / `then` lines).
Good subjects: GELU(0) = 0 on both branches; the tanh cubic coefficient is `0.044715`;
`0.5 * (1 + tanh(0))` is `0.5`, so the derivative at 0 is `0.5`; GELU is monotone in the
positive tail, so a larger input gives a larger output.

### Style notes, measured

`forward` was prototyped on a copy of this exact file and parsed with recovery-events 0,
declarations-swallowed 0 and lexer-discarded-chars 0, taking the node-type count from 9 to
17. The statement forms that were verified: `let n = input.len;`,
`for (0..n) |i| { ... }`, `let x = input[i];`,
`if (config.approximate) { ... } else { ... }`,
`output[i] = 0.5 * x * (1.0 + tanh(inner));`. Four-space indent for `fn` and `test` lines,
eight for statements.

**There is no `.t27` interpreter.** Test blocks are parsed and counted, never executed.
Write them — they are the house style — but do not claim you ran them.

**Do not lean on `t27c typecheck`.** It returns `{"ok":true,"errors":0}` on English prose
and on Python files. It is not a gate.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 2 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/ml/activation/gelu_activation.t27` prints 0
- 3. the real-code node count is at least 4
- 4. `t27c parse specs/ml/activation/gelu_activation.t27 | grep -c 'kind: TestBlock'` prints at least 6
- 5. `t27c count specs/ml/activation/gelu_activation.t27 | head -1` reports at least 13 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `2 names, fns=2 nonempty=0, clean` | `2 names, fns=n nonempty=n (n>=2), clean` |
| 2 | stub marker | `2` | `0` |
| 3 | real-code nodes | `0` | `>= 4` |
| 4 | test blocks | `4` | `>= 6` |
| 5 | node types | `9` | `>= 13` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 2 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (forward|backward)\(' specs/ml/activation/gelu_activation.t27
```

Must print `2`. It prints `2` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/ml/activation/gelu_activation.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/ml/activation/gelu_activation.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/ml/activation/gelu_activation.t27
```

Today it prints `2`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/ml/activation/gelu_activation.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `4`.

The bar is two nodes per stubbed function (2 x 2 = 4 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/ml/activation/gelu_activation.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `6` — the 4 blocks already in the file
plus one new block for each of the 2 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/ml/activation/gelu_activation.t27 | head -1
```

Today it prints `gelu_activation.t27: 9 node types`. It must report at least `13`. For scale, 70.2% of
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
- 2. grep -c 'TODO: Implement' specs/ml/activation/gelu_activation.t27 prints 0: met
- 3. the real-code node count is at least 4: met
- 4. t27c parse specs/ml/activation/gelu_activation.t27 | grep -c 'kind: TestBlock' prints at least 6: met
- 5. t27c count specs/ml/activation/gelu_activation.t27 | head -1 reports at least 13 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Named above only as a neighbour you must not drift into. No issue owns it
and nobody is working on it: `specs/ml/activation/gelu_approx_activation.t27`.

## Boundary

specs/ml/activation/gelu_activation.t27
