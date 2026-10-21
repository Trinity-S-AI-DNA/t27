# Implement the nine comment-only functions in specs/ml/transformer/mha_block.t27 (MHABlock)

## Context

`specs/ml/transformer/mha_block.t27` declares module `MHABlock`: one
Transformer encoder block — LayerNorm, multi-head attention, LayerNorm,
feed-forward, with residual connections — built on the `gf16` 16-bit float
format from `specs/numeric/gf16.t27`. It composes its neighbours
(`use ml::transformer::norm;`, `multi_head_attn`, `feed_forward`) rather than
reimplementing them. Its distinguishing feature is a φ-optimised residual: the
module declares `PHI_RESIDUAL_SCALE : gf16::GF16 = 1.0 / PHI`, and the block
can scale residual branches by it instead of adding them at unit weight.

Unlike most stubbed specs, this one is fully documented: **every empty body
already contains the algorithm as numbered comments**, and the 12 test blocks
plus 8 invariants pin the shapes and two of the arithmetic formulas exactly.
Your job is to turn those comments into t27, not to invent the design.

`.t27` is a hand-authored source language, compiled out to C/Rust/Verilog/Zig
by `t27c`. You are writing source, not compiler output.

## Exact current state

Run from the repo root `/Users/playom/t27`.

```
$ ./target/release/t27c count specs/ml/transformer/mha_block.t27 | head -1
mha_block.t27: 11 node types

$ ./target/release/t27c parse specs/ml/transformer/mha_block.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ grep -c 'TODO: Implement' specs/ml/transformer/mha_block.t27
0
```

383 lines, parse-clean today. Note there is **no** `TODO` marker in this file —
the nine stubs are bodies that contain nothing but comments, so the empty-body
checker in the criteria below is the only honest measure of progress here.

## What to write

Give each of the nine functions a real body. Keep every signature **verbatim**
and keep the `module MHABlock;` header form (semicolon, indented declarations,
no closing brace). Keep the existing numbered comments as documentation above
or inside each body; add code, do not merely delete the comments.

1. `fn init(config: BlockConfig) -> BlockState` — build `norm1` and `norm2` at
   `config.hidden_size`, `mha_state` at `(hidden_size, num_heads)`,
   `ffn_state` at `(hidden_size, expansion_factor)`, `is_training = true`.
2. `fn forward(state: BlockState, input: []gf16::GF16) -> BlockForwardResult` —
   the Pre-LN path already spelled out in the body: LN, MHA, dropout, residual,
   LN, FFN, dropout, residual. When `use_phi_residual`, scale residual branches
   by `PHI_RESIDUAL_SCALE`.
3. `fn forward_with_post_ln(state: BlockState, input: []gf16::GF16) -> BlockForwardResult` —
   the Post-LN ordering given in its comment.
4. `fn apply_residual_connection(x: []gf16::GF16, processed: []gf16::GF16, scale: gf16::GF16) -> []gf16::GF16` —
   elementwise `x + processed * scale`. Test `apply_residual_adds_inputs` fixes
   `x = [1.0,2.0,3.0]`, `processed = [0.1,0.2,0.3]`, `scale = 1.0` giving
   `[1.1, 2.2, 3.3]`.
5. `fn phi_scaled_residual(x: []gf16::GF16, processed: []gf16::GF16) -> []gf16::GF16` —
   `x + processed * PHI_RESIDUAL_SCALE`.
6. `fn backward(state: BlockState, result: BlockForwardResult, grad_output: []gf16::GF16) -> BlockGradients` —
   the nine numbered steps in its comment, in that order.
7. `fn get_parameter_count(config: BlockConfig) -> u64` — the test states the
   answer exactly:

```
    test get_parameter_count_matches_components
        given config = BlockConfig{.hidden_size = 128, .num_heads = 8, .expansion_factor = 4.0}
        when total = get_parameter_count(config)
        and   mha_params = 4 * 128 * 128 + 4 * 128 // Q,K,V,O + biases
        and   ffn_params = 2 * 128 * 512 + 2 * 512  // W1,W2 + biases
        and   norm_params = 2 * 2 * 128           // 2 Norms, gamma+beta each
        then total == mha_params + ffn_params + norm_params
```

   so: `4*h*h + 4*h` for MHA, `2*h*d_ff + h + d_ff` for FFN (with
   `d_ff = h * expansion_factor`), `2 * 2 * h` for the two norms.
8. `fn get_flops(config: BlockConfig, seq_len: u32) -> u64` — MHA
   `4 * seq_len^2 * hidden_size`, FFN
   `2 * seq_len * hidden_size * intermediate_size`, Norm
   `2 * seq_len * hidden_size * 2`. The invariant
   `flops_quadratic_in_sequence_length` and the test
   `get_flops_scales_quadratically_with_seq_len` both require the seq_len^2
   term to be present — a linear estimate fails them.
9. `fn create_residual_connection(scale: gf16::GF16) -> ResidualConnection` —
   `ResidualConnection{.scale = scale, .use_projection = false}`; the test
   asserts both fields.

Do **not** delete or rewrite any of the 12 test blocks, 8 invariant blocks or
8 bench blocks — they are the specification and the criteria count them. The
bench blocks assert wall-clock budgets (`elapsed_time_ms < 20` etc.); there is
**no t27 interpreter**, so nothing here is ever executed. Do not try to make a
benchmark pass and do not weaken one.

Style reference for real bodies in a parse-clean file (read-only):
`specs/numeric/gf16.t27` — same `gf16` domain, 53 implemented functions, all
parse metrics 0.

Do **not** gate yourself on `t27c typecheck`: it returns
`{"ok":true,"errors":0}` on two lines of English prose and on a Python file.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 9 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. the real-code node count is at least 18
- 3. `t27c parse specs/ml/transformer/mha_block.t27 | grep -c 'kind: TestBlock'` prints at least 21
- 4. `t27c count specs/ml/transformer/mha_block.t27 | head -1` reports at least 15 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `9 names, fns=9 nonempty=0, clean` | `9 names, fns=n nonempty=n (n>=9), clean` |
| 2 | real-code nodes | `0` | `>= 18` |
| 3 | test blocks | `12` | `>= 21` |
| 4 | node types | `11` | `>= 15` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 9 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|forward|forward_with_post_ln|apply_residual_connection|phi_scaled_residual|backward|get_parameter_count|get_flops|create_residual_connection)\(' specs/ml/transformer/mha_block.t27
```

Must print `9`. It prints `9` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/ml/transformer/mha_block.t27 <<'EOF'
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

Today it prints `fns=9 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 9**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 9 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/ml/transformer/mha_block.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/ml/transformer/mha_block.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `18`.

The bar is two nodes per stubbed function (9 x 2 = 18 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 3 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/ml/transformer/mha_block.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `12`. It must print at least `21` — the 12 blocks already in the file
plus one new block for each of the 9 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 4 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/ml/transformer/mha_block.t27 | head -1
```

Today it prints `mha_block.t27: 11 node types`. It must report at least `15`. For scale, 65.8% of
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
- 1. all 9 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. the real-code node count is at least 18: met
- 3. t27c parse specs/ml/transformer/mha_block.t27 | grep -c 'kind: TestBlock' prints at least 21: met
- 4. t27c count specs/ml/transformer/mha_block.t27 | head -1 reports at least 15 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Named above only as a neighbour you must not drift into. No issue owns these
and nobody is working on them: `specs/ml/transformer/norm.t27`, `specs/numeric/gf16.t27`.

## Boundary

specs/ml/transformer/mha_block.t27
