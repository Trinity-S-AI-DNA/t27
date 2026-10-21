# Implement the four stubbed functions in specs/tri/utils/utf8.t27

## Context

`specs/tri/utils/utf8.t27` is the `TriUtf8` module — UTF-8 encoding and decoding for
the `specs/tri/utils/` toolbox (neighbours: `string.t27`, `bytes.t27`, `text.t27`,
`version.t27`). Its header claims "Full Unicode support", and the two declared types
say exactly what that means here:

```
    pub const Codepoint = struct {
        underlying : U21,
    };

    pub const Rune = struct {
        bytes : [4]u8,
        len : u8,
    };
```

A `Codepoint` is a scalar value up to U+10FFFF; a `Rune` is its encoded form — a
fixed 4-byte buffer plus the number of bytes actually used (1 to 4). The existing test
blocks pin both boundaries: U+0000 and U+10FFFF, and the 1-byte and 4-byte rune cases.

**.t27 is a hand-authored source language.** You are writing source that `t27c`
compiles out to C/Rust/Verilog/Zig. Write code, not prose about code.

## Current state (measured — re-run these yourself)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/utils/utf8.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/utils/utf8.t27 | head -1
utf8.t27: 7 node types

$ ./target/release/t27c parse specs/tri/utils/utf8.t27 2>&1 | grep -c 'kind: FnDecl'
4

$ ./target/release/t27c parse specs/tri/utils/utf8.t27 2>&1 | grep -c 'kind: TestBlock'
5

$ grep -c 'TODO: Implement' specs/tri/utils/utf8.t27
4
```

The file parses clean today. All four declared functions are empty bodies holding a
single `// TODO: Implement from .tri spec` comment.

## What to write

Fill in these four functions. **Keep each signature exactly as it is** — criterion 1
greps for all four names by hand. Quoted verbatim from the file:

1. `fn decode(str: []const u8, index: usize) -> Rune` — read the UTF-8 sequence that
   starts at `str[index]`. The lead byte gives the length: `0xxxxxxx` → 1,
   `110xxxxx` → 2, `1110xxxx` → 3, `11110xxx` → 4. Copy those bytes into
   `Rune.bytes` and set `Rune.len`. A malformed or truncated sequence must return a
   rune of length 1 holding the offending byte, never read past `str.len`, and never
   loop.
2. `fn encode(codepoint: Codepoint, allocator: std.mem.Allocator) -> ![]u8` —
   allocate and return the UTF-8 bytes for `codepoint.underlying`: 1 byte below
   0x80, 2 below 0x800, 3 below 0x10000, 4 up to 0x10FFFF, with continuation bytes
   `10xxxxxx`. U+10FFFF must come out as `F4 8F BF BF` — the existing test names those
   four bytes.
3. `fn count_codepoints(str: []const u8) -> usize` — count scalars, not bytes: step
   over the string skipping continuation bytes (`0b10xxxxxx`, i.e. `b & 0xC0 == 0x80`).
4. `fn validate(str: []const u8) -> bool` — true only when the whole slice is
   well-formed UTF-8: every lead byte followed by exactly its continuation bytes, no
   truncation at the end, no codepoint above 0x10FFFF.

### The tests already in the file are the specification

Do not edit or delete the five existing test blocks. Two of them fix the encoded forms
you must produce:

```
    test rune_buffer_is_four_bytes_the_utf8_maximum
        // Verify: the declared fixed buffer matches the longest UTF-8 sequence
        given r = Rune{.bytes=[_]u8{0,0,0,0},.len=0}
        then r.bytes.len == 4

    test four_byte_rune_fills_the_whole_buffer
        // Verify: U+10FFFF encodes as F4 8F BF BF -- four bytes, exactly the
        // declared buffer size
        given r = Rune{.bytes=[_]u8{0xF4,0x8F,0xBF,0xBF},.len=4}
        then r.len == 4 and r.bytes[3] == 0xBF
```

One comment in the test section is now stale — it opens with "decode/encode/
count_codepoints/validate are unimplemented stubs returning void, so no behavioural
assertion is possible yet." Once you have written the bodies that sentence is false.
Rewrite that comment to describe what the tests below it now check.

### Add one test block per function you implement

House style: the proof of a function is a `test` block beside it. This file uses the
`test <name> / given / and / when / then` form (no quotes, no braces) — match it, and
add at least one new block per implemented function, calling the function you wrote.
Shape that parses clean here:

```
    test ascii_decodes_as_a_one_byte_rune
        given text = "A"
        when r = decode(text, 0)
        then r.len == 1 and r.bytes[0] == 65
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
- 2. `grep -c 'TODO: Implement' specs/tri/utils/utf8.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/utils/utf8.t27 | grep -c 'kind: TestBlock'` prints at least 9
- 5. `t27c count specs/tri/utils/utf8.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `5` | `>= 9` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (decode|encode|count_codepoints|validate)\(' specs/tri/utils/utf8.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/utils/utf8.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/utils/utf8.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/utils/utf8.t27
```

Today it prints `4`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/utils/utf8.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/utils/utf8.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `5`. It must print at least `9` — the 5 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/utils/utf8.t27 | head -1
```

Today it prints `utf8.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 4 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/utils/utf8.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/utils/utf8.t27 | grep -c 'kind: TestBlock' prints at least 9: met
- 5. t27c count specs/tri/utils/utf8.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/utils/utf8.t27
