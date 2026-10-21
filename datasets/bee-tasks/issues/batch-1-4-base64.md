# Implement the six stubbed functions in specs/tri/crypto/base64.t27 (TriBase64)

## Context

`specs/tri/crypto/base64.t27` declares module `TriBase64`, the RFC 4648
base64 codec of the `specs/tri/crypto/` family (`base32.t27` is its direct
sibling and follows the same shape). Its header comment says "RFC 4648
compliant". The codec is a value, not a global:

```
pub const Base64 = struct {
    alphabet : []const u8,
    padding : bool,
};
```

so `standard()` and `url_safe()` are the two constructors and `encode` /
`decode` take the codec as their first argument. The six existing test blocks
pin the two alphabets literally, pin that they agree on their first 62 symbols
and differ only at indices 62 and 63, pin that the URL-safe alphabet contains
neither `+` nor `/`, and pin the length arithmetic in both directions.

`.t27` is a hand-authored source language, compiled out to C/Rust/Verilog/Zig
by `t27c`. You are writing source, not compiler output.

## Exact current state

Run from the repo root `/Users/playom/t27`.

```
$ ./target/release/t27c count specs/tri/crypto/base64.t27 | head -1
base64.t27: 7 node types

$ ./target/release/t27c parse specs/tri/crypto/base64.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ grep -c 'TODO: Implement' specs/tri/crypto/base64.t27
6
```

97 lines, parse-clean today, all six functions empty.

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. Keep every
signature **verbatim** and keep the `module TriBase64;` header form
(semicolon, indented declarations, no closing brace).

1. `fn standard() -> Base64`
   Return `Base64{ .alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", .padding = true }`.
   The test quotes that exact literal and asserts `codec.alphabet.len == 64 and codec.padding`.
2. `fn url_safe() -> Base64`
   Same but ending `-_` and `.padding = false` — test
   `url_safe_alphabet_holds_sixty_four_symbols` asserts
   `codec.alphabet.len == 64 and codec.padding == false`.
3. `fn encode(codec: Base64, input: []const u8, allocator: std.mem.Allocator) -> ![]const u8`
   Take three input bytes at a time, split into four 6-bit groups, map each
   through `codec.alphabet`. A trailing group of 1 or 2 bytes emits 2 or 3
   characters, followed by `=` padding to a multiple of four **only when**
   `codec.padding` is true. Allocate the result from `allocator`.
4. `fn decode(codec: Base64, input: []const u8, allocator: std.mem.Allocator) -> ![]const u8`
   The inverse. A symbol not in `codec.alphabet` (and not `=`) is an error —
   this returns `![]const u8`, so return the error rather than silently
   skipping.
5. `fn encoded_length(codec: Base64, input_len: usize) -> usize`
   Padded: `4 * ceil(input_len / 3)`. Test
   `encoded_length_is_four_characters_per_three_bytes` fixes 3 -> 4 and
   4 -> 8. Unpadded (`codec.padding == false`): `ceil(input_len * 4 / 3)`.
6. `fn decoded_length(input: []const u8) -> !usize`
   From the character count and the trailing `=` run. Test
   `decoded_length_undoes_encoded_length_on_whole_groups` fixes 12 characters
   -> 9 bytes. A length that cannot be a valid stream is an error.

The length tests, verbatim, are the arithmetic you must match:

```
    test encoded_length_is_four_characters_per_three_bytes
        given three = 4 * ((3 + 2) / 3)
        and four = 4 * ((4 + 2) / 3)
        then three == 4 and four == 8

    test decoded_length_undoes_encoded_length_on_whole_groups
        given encoded = 4 * ((9 + 2) / 3)
        when decoded = encoded / 4 * 3
        then encoded == 12 and decoded == 9
```

Do not delete or rewrite the six existing test blocks. Add one new test block
per function you implement, in the file's unbraced
`test name / given / when / then` style, so at least six new ones. There is
**no t27 interpreter** — test blocks are parsed and counted, never executed —
so state intent (e.g. that `encode(standard(), "Man")` is `"TWFu"`, the RFC's
own example) rather than writing something tuned to a runner.

Style reference for real bodies in a parse-clean file (read-only):
`specs/numeric/gf16.t27`, 53 implemented functions, all parse metrics 0.

Do **not** gate yourself on `t27c typecheck`: it returns
`{"ok":true,"errors":0}` on two lines of English prose.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 6 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/crypto/base64.t27` prints 0
- 3. the real-code node count is at least 12
- 4. `t27c parse specs/tri/crypto/base64.t27 | grep -c 'kind: TestBlock'` prints at least 12
- 5. `t27c count specs/tri/crypto/base64.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `6 names, fns=6 nonempty=0, clean` | `6 names, fns=n nonempty=n (n>=6), clean` |
| 2 | stub marker | `6` | `0` |
| 3 | real-code nodes | `0` | `>= 12` |
| 4 | test blocks | `6` | `>= 12` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 6 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (standard|url_safe|encode|decode|encoded_length|decoded_length)\(' specs/tri/crypto/base64.t27
```

Must print `6`. It prints `6` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/crypto/base64.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/crypto/base64.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/crypto/base64.t27
```

Today it prints `6`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/crypto/base64.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `12`.

The bar is two nodes per stubbed function (6 x 2 = 12 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/crypto/base64.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `6`. It must print at least `12` — the 6 blocks already in the file
plus one new block for each of the 6 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/crypto/base64.t27 | head -1
```

Today it prints `base64.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/crypto/base64.t27 prints 0: met
- 3. the real-code node count is at least 12: met
- 4. t27c parse specs/tri/crypto/base64.t27 | grep -c 'kind: TestBlock' prints at least 12: met
- 5. t27c count specs/tri/crypto/base64.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Named above only as a neighbour you must not drift into. No issue owns it
and nobody is working on it: `specs/tri/crypto/base32.t27`.

## Boundary

specs/tri/crypto/base64.t27
