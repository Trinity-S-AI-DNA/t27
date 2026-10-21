# Implement the four stubbed functions in specs/tri/utils/random.t27

## Context

`specs/tri/utils/random.t27` is the `TriRandom` module — the deterministic PRNG the
`specs/tri/utils/` toolbox exposes to everything else (its neighbours there are
`time.t27`, `utf8.t27`, `version.t27`, `string.t27`: small, self-contained utility
modules). The file's header line names the algorithm, and that decision is already
made for you:

```
// Xorshift64* PRNG algorithm | φ² + 1/φ² = 3 | TRINITY
```

The state is one 64-bit word and nothing else:

```
    pub const Rng = struct {
        state : u64,
    };
```

The five existing test blocks pin the algorithm down to its constants: shifts 12, 25,
27 (right, left, right), the multiplier 2685821657736338717, and the fact that state 0
is a fixed point the seeding must avoid. Implement *that* generator — not an LCG, not
`std.rand`.

**.t27 is a hand-authored source language.** You are writing source that `t27c`
compiles out to C/Rust/Verilog/Zig. Write code, not prose about code.

## Current state (measured — re-run these yourself)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/utils/random.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/utils/random.t27 | head -1
random.t27: 7 node types

$ ./target/release/t27c parse specs/tri/utils/random.t27 2>&1 | grep -c 'kind: FnDecl'
4

$ ./target/release/t27c parse specs/tri/utils/random.t27 2>&1 | grep -c 'kind: TestBlock'
6

$ grep -c 'TODO: Implement' specs/tri/utils/random.t27
4
```

The file parses clean today. All four declared functions are empty bodies holding a
single `// TODO: Implement from .tri spec` comment.

## What to write

Fill in these four functions. **Keep each signature exactly as it is** — criterion 1
greps for all four names by hand. Quoted verbatim from the file:

1. `fn init(seed: u64) -> Rng` — build the starting state. Seed 0 must not be stored
   as-is: the test `a_zero_state_is_a_fixed_point_of_the_xorshift` proves 0 maps to 0
   forever, so substitute a non-zero default (the reference seed 88172645463325252 is
   the one this file already names) when `seed == 0`.
2. `fn next(rng: *Rng) -> u64` — one xorshift64\* step, in this order:
   `x ^= x >> 12`, `x ^= x << 25`, `x ^= x >> 27`, store `x` back into `rng.state`,
   and return `x * 2685821657736338717`. The multiply is the output stage only — never
   fold it back into the state.
3. `fn range(rng: *Rng, max: u64) -> u64` — a value in the half-open interval
   `[0, max)`, so `max` itself is never returned; return 0 when `max == 0`.
4. `fn range_inclusive(rng: *Rng, min: i64, max: i64) -> i64` — a value in the closed
   interval `[min, max]`, which spans `max - min + 1` values. The last existing test
   states exactly this difference between the two entry points.

Add a named constant for the multiplier (e.g. `const XORSHIFT_MULTIPLIER : u64 = 2685821657736338717;`)
in the Types section rather than repeating the literal.

### The tests already in the file are the specification

Do not edit or delete the six existing test blocks. Two of them fix the constants you
must use:

```
    test the_multiplier_is_the_star_in_xorshift64_star
        // Verify: the output stage multiplies by 2685821657736338717, which is
        // odd and therefore invertible modulo 2^64 — it scrambles the bits
        // without ever mapping two states onto one output
        given multiplier = 2685821657736338717
        when parity = multiplier % 2
        then parity == 1

    test range_endpoints_bound_a_half_open_interval
        // Verify: range(lo, hi) spans hi - lo values and range_inclusive spans one
        // more, which is the only difference between the two entry points
        given lo = 3
        and hi = 7
        when half_open = hi - lo
        and inclusive = hi - lo + 1
        then half_open == 4 and inclusive == 5
```

### Add one test block per function you implement

House style: the proof of a function is a `test` block beside it. This file uses the
`test <name> / given / and / when / then` form (no quotes, no braces) — match it, and
add at least one new block per implemented function, calling the function you wrote.
Shape that parses clean here:

```
    test init_replaces_the_zero_seed
        given rng = init(0)
        then rng.state != 0
```

### Style reference

`specs/fpga/mac.t27` lines 89–111 is a parse-clean file in the same `module Name;`
dialect with real function bodies (`const x = ...;`, `if (...) { ... } else { ... }`,
`return x;`); `specs/vsa/vsa_core.t27` lines 65–79 shows `var` and `while (...) { }`
parsing clean in the same dialect. There is **no t27 interpreter**: nothing here
executes. Do not claim you ran a test.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 4 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/utils/random.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/utils/random.t27 | grep -c 'kind: TestBlock'` prints at least 10
- 5. `t27c count specs/tri/utils/random.t27 | head -1` reports at least 11 node types
- 6. `grep -c '2685821657736338717' specs/tri/utils/random.t27` prints at least 3

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `6` | `>= 10` |
| 5 | node types | `7` | `>= 11` |
| 6 | xorshift* multiplier | `2` | `>= 3` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|next|range|range_inclusive)\(' specs/tri/utils/random.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/utils/random.t27 <<'EOF'
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

Today it prints `fns=4 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 4**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 4 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/utils/random.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/utils/random.t27
```

Today it prints `4`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/utils/random.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/utils/random.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `6`. It must print at least `10` — the 6 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/utils/random.t27 | head -1
```

Today it prints `random.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
the stub-free, parse-clean files in `specs/` are at or above that number, and the
median across them is 17.5.

`| head -1` is not optional. The rest of the `t27c count` table is **not reproducible**:
rows with equal counts print in a different order on every run (measured: 3 distinct
orderings in 8 consecutive runs over the same unchanged file), and the table stops
after 15 rows with `... and N more`. 14 files in `specs/` contain test blocks that
`t27c count | grep TestBlock` cannot see for exactly that reason — and they are the
richly-implemented ones, which is the state this task is trying to reach. Never read
a number off that table; the first line is the only stable part of the output.

**Criterion 6 — the output stage actually uses the multiplier.**

```
$ grep -c '2685821657736338717' specs/tri/utils/random.t27
```

Today it prints `2`, and both occurrences are inside a `test` block (lines 81 and 84) —
the constant is asserted about but never used. It must print at least `3`, which happens
when `next` multiplies by it in a real body. This is the one criterion here that checks
*which* algorithm you wrote and not merely that you wrote one.

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
- 1. all 4 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/utils/random.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/utils/random.t27 | grep -c 'kind: TestBlock' prints at least 10: met
- 5. t27c count specs/tri/utils/random.t27 | head -1 reports at least 11 node types: met
- 6. grep -c '2685821657736338717' specs/tri/utils/random.t27 prints at least 3: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/utils/random.t27
