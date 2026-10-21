# Implement forward and backward in the Relu activation spec (specs/ml/activation/relu_activation.t27)

## Context

`specs/ml/activation/relu_activation.t27` is the rectified-linear activation of the `ml` spec
tree: the elementwise `max(0, x)` that every network in `specs/ml/layers/` and
`specs/ml/recurrent/` composes with. It declares a config struct `ReLUConfig { negative_slope,
inplace }`, which makes it the *leaky* rectifier — `negative_slope = 0.0` is textbook ReLU and a
small positive slope keeps a gradient alive below the hinge. The file header names the property
the whole thing exists for: `// ReLU produces sparse gradients`. Its one constant, `ZERO : f32 =
0.0`, is the hinge itself, and the file's existing test says exactly that: the cut is at zero,
not near it. Both functions are empty stubs today, so the module states the threshold and the
shape of the config and nothing about the rectifier.

**.t27 is hand-authored source.** It is compiled out to C/Rust/Verilog/Zig by
`/Users/playom/t27/target/release/t27c`. You are writing source, not compiler output. The
dialect is module-code: `module X;`, `use a::b;`, `fn name(arg : type) -> ret { ... }`, plus
`test` and `invariant` blocks. Match the style already in the file.

## Exact current state (measured 2026-09-09, re-run these)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/ml/activation/relu_activation.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/ml/activation/relu_activation.t27 | head -1
relu_activation.t27: 10 node types

$ grep -c '// TODO: Implement from .tri spec' specs/ml/activation/relu_activation.t27
2

$ ./target/release/t27c lint --json specs/ml/activation/relu_activation.t27 | python3 -c 'import json,sys; d=json.load(sys.stdin); print(d["functions"], d["tests"], d["invariants"])'
2 1 2
```

The declarations the bodies must respect, quoted verbatim from the file:

```t27
    const ZERO : f32 = 0.0;

    pub const ReLUConfig = struct {
        negative_slope : f32,
        inplace : bool,
    };
```

and the two invariants already in the file, which are the contract on the buffers:

```t27
    invariant relu_constraint_0
        given input = valid_input()
        then true // input.len == output.len

    invariant relu_constraint_1
        given input = valid_input()
        then true // All inputs must be finite
```

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. **Keep both signatures exactly
as they stand** — same names, same parameter names, same types, same return type. Quoted
verbatim:

```t27
    fn forward(input: []const f32, output: []f32, config: ReLUConfig) -> void {
    fn backward(grad_output: []const f32, input: []const f32, grad_input: []f32, config: ReLUConfig) -> void {
```

- **`forward`** — walk the input elementwise and write into `output`: a value strictly above
  `ZERO` passes through unchanged; a value at or below `ZERO` is multiplied by
  `config.negative_slope`. Use the constant `ZERO`, not a literal `0.0` — the constant is the
  declared hinge and the existing test pins it. Both buffers are the same length by
  `relu_constraint_0`, so drive the loop off `input.len`; do not silently truncate to the shorter
  of the two. Say in a comment which side of the hinge exactly-zero falls on (the convention is
  that `x == ZERO` takes the negative branch, which for the default `negative_slope = 0.0` gives
  the usual `max(0, x)`).
- **`backward`** — the gradient of that same hinge: for each element, `grad_input[i]` is
  `grad_output[i]` where `input[i] > ZERO`, and `grad_output[i] * config.negative_slope`
  otherwise. Note the gate is on the *forward input*, not on `grad_output` — that is what makes
  the gradient sparse, and it is the one thing that is easy to get wrong here. Add a comment
  saying so.
- `config.inplace` is declared but neither function is given a way to honour it (the buffers are
  separate parameters). Do **not** change the signatures to make it work. Write one comment
  noting that `inplace` is currently unused by these two functions, so the next reader does not
  assume the aliasing case is handled.

The single existing test, `zero_is_the_rectifier_threshold`, is correct — **do not delete or
rewrite it**, and do not touch the two invariant blocks. Add at least three new `test` blocks in
the file's own `test name / given / when / then / and` style (no braces, no quoted test name —
copy the layout of the existing one), placed with the other tests and above the invariants. Pin:
that the positive branch is the identity, that the negative branch scales by
`config.negative_slope`, that `negative_slope = 0.0` makes the negative branch exactly `ZERO`,
and that the backward gate reads `input` rather than `grad_output`. There is **no t27
interpreter** — these blocks are parsed and counted, never executed. Write them as the
specification of the behaviour, and do not claim you ran them.

### Parser hazards (measured — these silently destroy the file)

The bootstrap parser swallows declarations on two constructs *without* raising
`recovery-events`, so the file can look clean and still have lost declarations:

- `switch` arms with a **qualified** pattern (`Foo.bar => ...`) — measured: `FnDecl` fell from 4
  to 2 with all three parse metrics still reporting 0. Use `if` / `else if` chains instead.
  Literal arms (`0 => ...`) are safe.
- The **anonymous struct literal** `.{ ... }` — measured: `recovery-events: 1` plus swallowed
  declarations. The *named* form `ReLUConfig{ .negative_slope = 0.0, .inplace = false }` is safe
  and measured clean; only the leading-dot anonymous form breaks.

Measured clean in bodies for exactly this file: a `while` loop with `var i : usize = 0`,
`input[i]` indexing, `if`/`else`, `const` locals, field access `config.negative_slope`, and
assignment into `output[i]`. `for (xs) |x|`, `try`, `catch`, `orelse`, `defer` and `@as` are
clean too.

Note that `t27c count` prints only its top 15 node types and this file already reports 10 — once
bodies and tests are added it crosses the cut and starts printing `... and N more`, which can hide
`FnDecl`. That is why the criteria below use `t27c lint --json`, whose `"functions"` and
`"tests"` numbers are untruncated and which collapses the same way when declarations are
swallowed (measured: functions 5 -> 1, tests 4 -> 0 on a file broken by a qualified `switch`).

Do **not** gate yourself on `t27c typecheck` — it returns `{"ok":true,"errors":0}` on English
prose and on Python files. It proves nothing. Ignore `t27c lint`'s `"warnings"` array too — it
reports "fn X has no test or invariant" for every function in the gold corpus.

### House law

Repo law L3 (PURITY) requires source to be ASCII-only with English identifiers. The file's
existing banner comments already contain non-ASCII decoration — leave those lines exactly as they
are, and keep every line **you** add pure ASCII. Law L4 (TESTABILITY) is why the added `test`
blocks are not optional.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 2 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/ml/activation/relu_activation.t27` prints 0
- 3. the real-code node count is at least 4
- 4. `t27c parse specs/ml/activation/relu_activation.t27 | grep -c 'kind: TestBlock'` prints at least 3
- 5. `t27c count specs/ml/activation/relu_activation.t27 | head -1` reports at least 14 node types
- 6. `grep -c 'ZERO' specs/ml/activation/relu_activation.t27` prints at least 4

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `2 names, fns=2 nonempty=0, clean` | `2 names, fns=n nonempty=n (n>=2), clean` |
| 2 | stub marker | `2` | `0` |
| 3 | real-code nodes | `0` | `>= 4` |
| 4 | test blocks | `1` | `>= 3` |
| 5 | node types | `10` | `>= 14` |
| 6 | hinge named, not inlined | `2` | `>= 4` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 2 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (forward|backward)\(' specs/ml/activation/relu_activation.t27
```

Must print `2`. It prints `2` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/ml/activation/relu_activation.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/ml/activation/relu_activation.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/ml/activation/relu_activation.t27
```

Today it prints `2`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/ml/activation/relu_activation.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `4`.

The bar is two nodes per stubbed function (2 x 2 = 4 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/ml/activation/relu_activation.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `1`. It must print at least `3` — the 1 blocks already in the file
plus one new block for each of the 2 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/ml/activation/relu_activation.t27 | head -1
```

Today it prints `relu_activation.t27: 10 node types`. It must report at least `14`. For scale, 68.2% of
the stub-free, parse-clean files in `specs/` are at or above that number, and the
median across them is 17.5.

`| head -1` is not optional. The rest of the `t27c count` table is **not reproducible**:
rows with equal counts print in a different order on every run (measured: 3 distinct
orderings in 8 consecutive runs over the same unchanged file), and the table stops
after 15 rows with `... and N more`. 14 files in `specs/` contain test blocks that
`t27c count | grep TestBlock` cannot see for exactly that reason — and they are the
richly-implemented ones, which is the state this task is trying to reach. Never read
a number off that table; the first line is the only stable part of the output.

**Criterion 6 — the hinge is referenced by name, not by literal.**

```
$ grep -c 'ZERO' specs/ml/activation/relu_activation.t27
```

Today it prints `2`: the `const ZERO : f32 = 0.0;` declaration on line 13 and one use in a
test on line 47. It must print at least `4` — `forward` and `backward` both compare
against the named constant rather than writing a bare `0.0`. This is what stops a body
that happens to return the right shape from passing as a ReLU.

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
- 2. grep -c 'TODO: Implement' specs/ml/activation/relu_activation.t27 prints 0: met
- 3. the real-code node count is at least 4: met
- 4. t27c parse specs/ml/activation/relu_activation.t27 | grep -c 'kind: TestBlock' prints at least 3: met
- 5. t27c count specs/ml/activation/relu_activation.t27 | head -1 reports at least 14 node types: met
- 6. grep -c 'ZERO' specs/ml/activation/relu_activation.t27 prints at least 4: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/ml/activation/relu_activation.t27
