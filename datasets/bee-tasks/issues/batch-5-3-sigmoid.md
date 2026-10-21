# Implement forward, forward_batch and derivative in specs/ml/activation/sigmoid_activation.t27

## Context

`specs/ml/activation/sigmoid_activation.t27` is the sigmoid activation of the ML
layer, one of ten activation modules in `specs/ml/activation/` alongside
`relu_activation.t27`, `tanh_activation.t27`, `gelu_activation.t27` and
`softmax.t27`. Its shape follows the family convention: a `SigmoidConfig` struct
that exists only to keep the interface uniform with activations that do have
parameters (LeakyReLU's slope, ELU's alpha) — sigmoid has none, so the config
carries a single `void` field and occupies no storage. The header comment names
the property that makes sigmoid cheap in backprop: "Derivative (compute from
output)". All three functions are empty bodies today, so as the file's own test
comment says, none of what makes sigmoid worth having can be stated: not
`forward(0) == 0.5`, not that the output is confined to (0, 1), and not
`sigma'(x) = sigma(x) * (1 - sigma(x))`. Your job is to write the three bodies
and then state those properties as test blocks.

`.t27` is hand-authored source, compiled out to C/Rust/Verilog/Zig by `t27c`.
You are writing the input language, not compiler output.

## Exact current state

All commands run from `/Users/playom/t27`.

```
$ ./target/release/t27c parse specs/ml/activation/sigmoid_activation.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/ml/activation/sigmoid_activation.t27 | head -1
sigmoid_activation.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/ml/activation/sigmoid_activation.t27
3
```

```
$ grep -cE '^[[:space:]]*test ' specs/ml/activation/sigmoid_activation.t27
2
```

The file parses clean today. Keep it clean and keep both existing test blocks.

## What to write

Three functions. Signatures quoted verbatim — change only the bodies.

1. `fn forward(x: f32) -> f32`
   The logistic function, `1 / (1 + exp(-x))`. Use the numerically stable split:
   for `x >= 0` compute `1.0 / (1.0 + @exp(-x))`; for `x < 0` compute
   `@exp(x) / (1.0 + @exp(x))`, which avoids `@exp` of a large positive argument
   overflowing. Both branches agree mathematically; write a comment saying why
   the branch exists.

2. `fn forward_batch(input: []f32) -> []f32`
   Elementwise `forward` over the slice. The signature returns a slice and takes
   no allocator, so write it as an in-place map over `input` returning `input` —
   do not invent an allocator parameter, and do not change the signature.
   Comment the aliasing decision.

3. `fn derivative(sigmoid_x: f32) -> f32`
   Note the parameter name: it takes the **output** of `forward`, not the input.
   Return `sigmoid_x * (1.0 - sigmoid_x)`. This is the identity in the header
   comment and the reason sigmoid is cheap to backpropagate.

### The tests are the specification

Two test blocks already exist, pinning the config shape. Do not modify or delete
them:

```
    test sigmoid_has_no_tunable_parameter
        given size = @sizeOf(SigmoidConfig)
        then size == 0

    test sigmoid_config_is_a_placeholder_not_an_empty_struct
        given fields = std.meta.fields(SigmoidConfig)
        then (fields.len == 1) and (fields[0].type == void)
```

The test-section comment above them names exactly the tests to add once the
bodies exist. Add at least one test block for each of the three, covering:

- `forward(0.0)` is `0.5` (to a small epsilon)
- the output is confined to `(0, 1)` for large-magnitude inputs both ways
- `derivative(forward(0.0))` is `0.25`, the maximum of the derivative
- `forward_batch` returns a slice of the same length it was given

Then replace the stale comment above the test section (the one beginning
"forward()/forward_batch()/derivative() have no body yet") with one describing
what the module now states.

Both of these test forms parse clean in this corpus — use either:

```
    test "forward_at_zero_is_one_half" {
        expect(@abs(forward(0.0) - 0.5) < 1e-6)
    }

    test forward_at_zero_is_one_half
        given y = forward(0.0)
        then @abs(y - 0.5) < 1e-6
```

**There is no t27 interpreter.** Test blocks are parsed and counted, never
executed. Do not claim a test passed.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 3 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/ml/activation/sigmoid_activation.t27` prints 0
- 3. the real-code node count is at least 6
- 4. `t27c parse specs/ml/activation/sigmoid_activation.t27 | grep -c 'kind: TestBlock'` prints at least 5
- 5. `t27c count specs/ml/activation/sigmoid_activation.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `3 names, fns=3 nonempty=0, clean` | `3 names, fns=n nonempty=n (n>=3), clean` |
| 2 | stub marker | `3` | `0` |
| 3 | real-code nodes | `0` | `>= 6` |
| 4 | test blocks | `2` | `>= 5` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 3 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (forward|forward_batch|derivative)\(' specs/ml/activation/sigmoid_activation.t27
```

Must print `3`. It prints `3` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/ml/activation/sigmoid_activation.t27 <<'EOF'
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

Today it prints `fns=3 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 3**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 3 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/ml/activation/sigmoid_activation.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/ml/activation/sigmoid_activation.t27
```

Today it prints `3`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/ml/activation/sigmoid_activation.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `6`.

The bar is two nodes per stubbed function (3 x 2 = 6 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/ml/activation/sigmoid_activation.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `2`. It must print at least `5` — the 2 blocks already in the file
plus one new block for each of the 3 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/ml/activation/sigmoid_activation.t27 | head -1
```

Today it prints `sigmoid_activation.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 3 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/ml/activation/sigmoid_activation.t27 prints 0: met
- 3. the real-code node count is at least 6: met
- 4. t27c parse specs/ml/activation/sigmoid_activation.t27 | grep -c 'kind: TestBlock' prints at least 5: met
- 5. t27c count specs/ml/activation/sigmoid_activation.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/ml/activation/sigmoid_activation.t27
