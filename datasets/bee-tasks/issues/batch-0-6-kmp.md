# Implement build_prefix and search in specs/tri/search/knuth_morris_pratt.t27

## Context

`specs/tri/search/knuth_morris_pratt.t27` is the `TriKmp` module — Knuth–Morris–Pratt
substring search, one of the string-search family in `specs/tri/search/`
(`boyer_moore.t27`, `rabin_karp.t27`, `aho_corasick.t27`, `regex.t27` are its
neighbours; each owns one algorithm and this file owns KMP). The header states the
guarantee the implementation has to earn:

```
// O(n + m) time where n=text length, m=pattern length
```

The precomputed table is the whole trick, and the type declares its shape:

```
    pub const KMPPrefix = struct {
        table : []usize,
        pattern : []const u8,
    };
```

`table[i]` is the length of the longest *proper* border of `pattern[0..i+1]` — the
longest string that is both a proper prefix and a suffix of that slice. One entry per
pattern byte, so `table.len == pattern.len`.

**.t27 is a hand-authored source language.** You are writing source that `t27c`
compiles out to C/Rust/Verilog/Zig. Write code, not prose about code.

## Current state (measured — re-run these yourself)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/search/knuth_morris_pratt.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/search/knuth_morris_pratt.t27 | head -1
knuth_morris_pratt.t27: 7 node types

$ ./target/release/t27c parse specs/tri/search/knuth_morris_pratt.t27 2>&1 | grep -c 'kind: FnDecl'
2

$ ./target/release/t27c parse specs/tri/search/knuth_morris_pratt.t27 2>&1 | grep -c 'kind: TestBlock'
4

$ grep -c 'TODO: Implement' specs/tri/search/knuth_morris_pratt.t27
2
```

The file parses clean today. Both declared functions are empty bodies holding a single
`// TODO: Implement from .tri spec` comment.

## What to write

Fill in these two functions. **Keep each signature exactly as it is** — criterion 1
greps for both names by hand. Quoted verbatim from the file:

1. `fn build_prefix(pattern: []const u8, allocator: std.mem.Allocator) -> !KMPPrefix` —
   allocate a `table` of `pattern.len` entries and fill it with the border lengths.
   The standard construction: `table[0] = 0`; walk `i` from 1, carrying a border
   length `k`; while `k > 0` and `pattern[i] != pattern[k]`, set `k = table[k - 1]`;
   if `pattern[i] == pattern[k]` increment `k`; store `table[i] = k`. Return a
   `KMPPrefix` holding that table and the pattern. An empty pattern yields an empty
   table, not an error.
2. `fn search(text: []const u8, prefix: *const KMPPrefix) -> []usize` — return the
   start index of every occurrence of `prefix.pattern` in `text`, in increasing order.
   Scan `text` once with a match length `j`: on a mismatch fall back with
   `j = prefix.table[j - 1]` (never restart at the beginning of `text`), and on
   `j == prefix.pattern.len` record `i + 1 - j` and fall back the same way so
   overlapping matches are all found. The pattern arrives inside `prefix`, not as a
   separate argument — that is what `prefix.pattern` is for.

Together those two give the O(n + m) bound in the header: each fallback hop strictly
decreases `j`, so no character of `text` is examined more than a constant number of
times.

### The tests already in the file are the specification

Do not edit or delete the four existing test blocks. Two of them are the exact tables
your `build_prefix` must produce, written out by hand:

```
    test the_border_table_of_ababa
        // "ababa": no border at 'a' or 'ab', then "aba" borders on "a" (1),
        // "abab" on "ab" (2), "ababa" on "aba" (3). This is what build_prefix
        // must produce for this pattern.
        given table = [_]usize{ 0, 0, 1, 2, 3 }
        given kmp = KMPPrefix{ .table = @constCast(&table), .pattern = "ababa" }
        then kmp.table.len == kmp.pattern.len
        and kmp.table[0] == 0
        and kmp.table[2] == 1
        and kmp.table[4] == 3

    test a_fallback_chain_strictly_decreases_so_the_scan_terminates
        // On a mismatch the search follows j = table[j-1] repeatedly. Each hop
        // lands on a strictly smaller index, which is why the whole scan is
        // O(n + m) rather than quadratic: "ababa" falls 3 -> 1 -> 0.
        given table = [_]usize{ 0, 0, 1, 2, 3 }
        given first_hop = table[4]
        given second_hop = table[first_hop - 1]
        then first_hop == 3
        and second_hop == 1
        and second_hop < first_hop
        and table[second_hop - 1] == 0
```

The comment above them is now partly stale: it claims both functions are unimplemented
stubs that `@panic`, and that "search's signature is also truncated to
`search(text: []const u8)`, with nowhere to receive the pattern" — which the current
signature contradicts, since `prefix` carries the pattern. Rewrite that comment to
match the file you leave behind.

### Add test blocks for what you implement

House style: the proof of a function is a `test` block beside it. This file uses the
`test <name> / given / and / when / then` form (no quotes, no braces) — match it, and
add at least two new blocks per implemented function: for `build_prefix`, the "ababa"
table above and the all-same pattern "aaaa"; for `search`, one hit at index 0 and one
overlapping-matches case (pattern "aa" in text "aaaa" starts at 0, 1 and 2). Shape:

```
    test build_prefix_produces_the_ababa_table
        given kmp = build_prefix("ababa", std.testing.allocator)
        then kmp.table.len == 5
        and kmp.table[2] == 1
        and kmp.table[4] == 3
```

### Style reference

`specs/fpga/mac.t27` lines 89–111 is a parse-clean file in the same `module Name;`
dialect with real function bodies (`const x = ...;`, `if (...) { ... } else { ... }`,
`return x;`); `specs/vsa/vsa_core.t27` lines 65–79 shows `var` and `while (...) { }`
parsing clean in the same dialect — you will need the `while` form for both functions
here. There is **no t27 interpreter**: nothing here executes. Do not claim you ran a
test.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 2 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/search/knuth_morris_pratt.t27` prints 0
- 3. the real-code node count is at least 4
- 4. `t27c parse specs/tri/search/knuth_morris_pratt.t27 | grep -c 'kind: TestBlock'` prints at least 6
- 5. `t27c count specs/tri/search/knuth_morris_pratt.t27 | head -1` reports at least 11 node types

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
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (build_prefix|search)\(' specs/tri/search/knuth_morris_pratt.t27
```

Must print `2`. It prints `2` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/search/knuth_morris_pratt.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/search/knuth_morris_pratt.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/search/knuth_morris_pratt.t27
```

Today it prints `2`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/search/knuth_morris_pratt.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `4`.

The bar is two nodes per stubbed function (2 x 2 = 4 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/search/knuth_morris_pratt.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `6` — the 4 blocks already in the file
plus one new block for each of the 2 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/search/knuth_morris_pratt.t27 | head -1
```

Today it prints `knuth_morris_pratt.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/search/knuth_morris_pratt.t27 prints 0: met
- 3. the real-code node count is at least 4: met
- 4. t27c parse specs/tri/search/knuth_morris_pratt.t27 | grep -c 'kind: TestBlock' prints at least 6: met
- 5. t27c count specs/tri/search/knuth_morris_pratt.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Named above only as a neighbour you must not drift into. No issue owns it
and nobody is working on it: `specs/tri/search/boyer_moore.t27`.

## Boundary

specs/tri/search/knuth_morris_pratt.t27
