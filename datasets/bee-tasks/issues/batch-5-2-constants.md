# Implement the twelve stubbed accessors in specs/tri/math/constants.t27

## Context

`specs/tri/math/constants.t27` is the module every other spec in this corpus
imports: `use math::constants;` appears at the top of hundreds of files in
`specs/`. It is meant to be the single place where the framework's two families
of fixed values live — `SystemLimits` (four `usize` platform ceilings) and
`SacredConstants` (six `f64` mathematical constants: phi, pi, e, sqrt2, sqrt3,
golden_ratio). The header comment states the contract: "All constants are
comptime-known". Today all twelve accessors are empty bodies, so the module
that everything depends on cannot hand out a single value, and its own test
comment admits "no value can be read out of this module yet". Your job is to
write those twelve bodies in `.t27`.

`.t27` is hand-authored source, compiled out to C/Rust/Verilog/Zig by `t27c`.
You are writing the input language. Do not write Zig, C or Rust files.

## Exact current state

All commands run from `/Users/playom/t27`.

```
$ ./target/release/t27c parse specs/tri/math/constants.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/math/constants.t27 | head -1
constants.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/math/constants.t27
12
```

```
$ grep -cE '^[[:space:]]*test ' specs/tri/math/constants.t27
4
```

The file parses clean today. Keep it clean, and keep all four existing test
blocks.

## What to write

Twelve functions. Signatures quoted verbatim from the file — change only the
bodies.

The four `SystemLimits` accessors:

- `fn max_path_len() -> usize` — POSIX `PATH_MAX`: return `4096`.
- `fn max_line_len() -> usize` — the line buffer ceiling: return `65536`.
- `fn max_args() -> usize` — argv ceiling: return `4096`.
- `fn max_env_vars() -> usize` — environment ceiling: return `4096`.

Any consistent, documented set of powers-of-two is acceptable here; what is not
acceptable is a magic number with no comment. Put a one-line comment above each
saying what the number is.

The six `SacredConstants` accessors, all `f64`. These values are **fixed** — the
existing test blocks below pin them to 1e-12, so write the full-precision
literals:

- `fn get_p_h_i() -> f64` — phi, `1.61803398874989484820458683436563811772`
- `fn get_p_i() -> f64` — pi, `3.14159265358979323846264338327950288`
- `fn get_e() -> f64` — e, `2.7182818284590452353602874713526625`
- `fn get_s_q_r_t2() -> f64` — sqrt(2), `1.41421356237309504880168872420969808`
- `fn get_s_q_r_t3() -> f64` — sqrt(3), `1.73205080756887729352744634150587237`
- `fn get_golden_ratio() -> f64` — the same value as `get_p_h_i()`; return
  `get_p_h_i()` rather than repeating the literal, so the two can never drift.

The two aggregate accessors:

- `fn get_system_limits() -> SystemLimits` — return a `SystemLimits` struct
  literal whose four fields call the four accessors above, not fresh literals.
- `fn get_sacred_constants() -> SacredConstants` — return a `SacredConstants`
  struct literal whose six fields call the six accessors above.

Struct-literal syntax that is verified to parse clean in this dialect:

```
    fn get_system_limits() -> SystemLimits {
        return SystemLimits{
            .max_path_len = max_path_len(),
            .max_line_len = max_line_len(),
            .max_args = max_args(),
            .max_env_vars = max_env_vars(),
        };
    }
```

### The tests are the specification

Four test blocks already exist. They construct a `SacredConstants` literal by
hand and assert the identities the values must satisfy — this is exactly what
your accessors must return. Do not modify or delete them. The first one:

```
    test declared_f64_width_carries_the_sacred_identity
        // phi^2 + 1/phi^2 = 3
        given c = SacredConstants{ .phi = 1.61803398874989484820458683436563811772, ... }
        and phi_sq = c.phi * c.phi
        and sum = phi_sq + 1.0 / phi_sq
        then @abs(sum - 3.0) < 1e-12
```

The others pin `phi * phi == phi + 1`, `sqrt2 * sqrt2 == 2` and
`sqrt3 * sqrt3 == 3` to 1e-12, and pin the `SystemLimits` fields as `usize`.

Add new test blocks that state the same identities **through the accessors**
now that they have bodies — e.g. that `get_sacred_constants().phi` equals
`get_p_h_i()`, that `get_p_h_i() * get_p_h_i() - (get_p_h_i() + 1.0)` is under
1e-12, and that `get_system_limits().max_path_len == max_path_len()`. Also
replace the stale comment above the test section (the one beginning "Every one
of the twelve accessors above has an empty body").

**There is no t27 interpreter.** Test blocks are parsed and counted, never run.
Do not claim a test passed.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 12 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/math/constants.t27` prints 0
- 3. the real-code node count is at least 12
- 4. `t27c parse specs/tri/math/constants.t27 | grep -c 'kind: TestBlock'` prints at least 16
- 5. `t27c count specs/tri/math/constants.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `12 names, fns=12 nonempty=0, clean` | `12 names, fns=n nonempty=n (n>=12), clean` |
| 2 | stub marker | `12` | `0` |
| 3 | real-code nodes | `0` | `>= 12` |
| 4 | test blocks | `4` | `>= 16` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 12 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (max_path_len|max_line_len|max_args|max_env_vars|get_p_h_i|get_p_i|get_e|get_s_q_r_t2|get_s_q_r_t3|get_golden_ratio|get_system_limits|get_sacred_constants)\(' specs/tri/math/constants.t27
```

Must print `12`. It prints `12` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/math/constants.t27 <<'EOF'
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

Today it prints `fns=12 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 12**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 12 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/math/constants.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/math/constants.t27
```

Today it prints `12`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/math/constants.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `12`.

The bar is one node per stubbed function and no more, because these are constant
accessors: a body of `return PHI;` contributes exactly one `ExprReturn`. Measured on
a hand-written reference file in this dialect, three accessor-style functions
produced 5 real-code nodes — 1.7 per function — so 12 across 12 functions is a
floor a faithful implementation clears, and prose does not.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/math/constants.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `16` — the 4 blocks already in the file
plus one new block for each of the 12 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/math/constants.t27 | head -1
```

Today it prints `constants.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 12 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/math/constants.t27 prints 0: met
- 3. the real-code node count is at least 12: met
- 4. t27c parse specs/tri/math/constants.t27 | grep -c 'kind: TestBlock' prints at least 16: met
- 5. t27c count specs/tri/math/constants.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/math/constants.t27
