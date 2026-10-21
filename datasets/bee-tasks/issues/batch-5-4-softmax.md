# Implement softmax forward and backward in specs/ml/activation/softmax.t27

## Context

`specs/ml/activation/softmax.t27` is the softmax layer of the ML activation
family in `specs/ml/activation/` (ten modules: relu, gelu, sigmoid, tanh, silu,
elu, leaky relu, and this one). Unlike its neighbours it is a *distribution*
rather than a pointwise function: it maps a vector of logits to probabilities
that sum to 1. The module has already made three design decisions that its
constants and config record — a temperature that divides the logits with a
positive floor `MIN_TEMP = 0.001` (so the constraint can never admit a division
by zero), an `axis : u32` index, and an opt-in `stable : bool` selecting the
max-subtraction trick, which the file's own test notes means "the unstable path
is reachable by construction". Both functions are empty bodies today, so the
sum-to-one property recorded in `invariant softmax_constraint_2` "cannot be
checked here". Your job is to write those two bodies.

`.t27` is hand-authored source, compiled out to C/Rust/Verilog/Zig by `t27c`.
You are writing the input language.

## Exact current state

All commands run from `/Users/playom/t27`.

```
$ ./target/release/t27c parse specs/ml/activation/softmax.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/ml/activation/softmax.t27 | head -1
softmax.t27: 10 node types
```

```
$ grep -c 'TODO: Implement' specs/ml/activation/softmax.t27
2
```

```
$ grep -cE '^[[:space:]]*test ' specs/ml/activation/softmax.t27
3
```

The file parses clean today. Keep it clean, and keep all three test blocks and
all three invariant blocks.

## What to write

Two functions. Signatures quoted verbatim — change only the bodies.

1. `fn forward(logits: []const f32, output: []f32, config: SoftmaxConfig) -> void`

   Write the result into `output`; the function returns nothing, so nothing is
   allocated. Steps:
   - clamp the divisor: use `config.temperature` but never below `MIN_TEMP` —
     that is what the positive floor is for.
   - if `config.stable`, find `max(logits)` and subtract it from every scaled
     logit before exponentiating; if not, skip that pass. Both paths must be
     present, because `stable` is a runtime bool.
   - `output[i] = @exp(scaled[i])`, accumulate the sum, then divide every
     element by the sum so the vector sums to 1.
   - `logits.len` and `output.len` are equal by `invariant softmax_constraint_0`;
     you may assume it, but say so in a comment.
   - `config.axis` is declared but this signature takes a flat 1-D slice. Do not
     invent a shape argument: comment that axis is unused at this rank and leave
     the signature alone.

2. `fn backward(grad_output: []const f32, probabilities: []const f32, grad_input: []f32, config: SoftmaxConfig) -> void`

   The softmax Jacobian-vector product. With `p = probabilities` and
   `g = grad_output`, compute `dot = sum(g[i] * p[i])` and then
   `grad_input[i] = p[i] * (g[i] - dot) / temperature`, using the same
   `MIN_TEMP`-clamped temperature as `forward`. Write into `grad_input`; return
   nothing.

### The tests are the specification

These three test blocks already exist and pin the constants and the config
shape. Do not modify or delete them:

```
    test min_temp_is_strictly_above_zero
        then MIN_TEMP > ZERO
        and @abs(MIN_TEMP - 0.001) < 1e-9
        and ZERO == 0.0

    test temperature_is_a_float_axis_is_an_index
        then @FieldType(SoftmaxConfig, "temperature") == f32
        and @FieldType(SoftmaxConfig, "axis") == u32

    test the_stable_max_subtraction_is_opt_in
        then @FieldType(SoftmaxConfig, "stable") == bool
        and @typeInfo(SoftmaxConfig).@"struct".fields.len == 3
```

So do the three invariant blocks, which name the three constraints your bodies
must honour — equal lengths, `temperature >= MIN_TEMP`, and `sum(output) ~= 1.0`:

```
    invariant softmax_constraint_2
        given input = valid_input()
        then true // sum(output) ≈ 1.0 (within floating precision)
```

Add at least one new `test` block per implemented function, stating the
properties that are now expressible: that `forward` on a uniform input produces
`1/n` in every slot, that the outputs sum to 1 within an epsilon, that the
stable and unstable paths agree on a small well-scaled input, and that
`backward` on `grad_output == probabilities` behaves as the formula predicts.
Also replace the stale comment above the test section (the one beginning
"forward and backward are unimplemented stubs").

**There is no t27 interpreter.** Test and invariant blocks are parsed and
counted, never executed. Do not claim a test passed.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 2 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/ml/activation/softmax.t27` prints 0
- 3. the real-code node count is at least 4
- 4. `t27c parse specs/ml/activation/softmax.t27 | grep -c 'kind: TestBlock'` prints at least 5
- 5. `t27c count specs/ml/activation/softmax.t27 | head -1` reports at least 14 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `2 names, fns=2 nonempty=0, clean` | `2 names, fns=n nonempty=n (n>=2), clean` |
| 2 | stub marker | `2` | `0` |
| 3 | real-code nodes | `0` | `>= 4` |
| 4 | test blocks | `3` | `>= 5` |
| 5 | node types | `10` | `>= 14` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 2 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (forward|backward)\(' specs/ml/activation/softmax.t27
```

Must print `2`. It prints `2` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/ml/activation/softmax.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/ml/activation/softmax.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/ml/activation/softmax.t27
```

Today it prints `2`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/ml/activation/softmax.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `4`.

The bar is two nodes per stubbed function (2 x 2 = 4 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/ml/activation/softmax.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `3`. It must print at least `5` — the 3 blocks already in the file
plus one new block for each of the 2 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/ml/activation/softmax.t27 | head -1
```

Today it prints `softmax.t27: 10 node types`. It must report at least `14`. For scale, 68.2% of
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
- 2. grep -c 'TODO: Implement' specs/ml/activation/softmax.t27 prints 0: met
- 3. the real-code node count is at least 4: met
- 4. t27c parse specs/ml/activation/softmax.t27 | grep -c 'kind: TestBlock' prints at least 5: met
- 5. t27c count specs/ml/activation/softmax.t27 | head -1 reports at least 14 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/ml/activation/softmax.t27
