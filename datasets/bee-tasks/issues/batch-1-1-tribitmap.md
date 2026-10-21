# Implement the ten stubbed functions in specs/tri/collections/bitmap.t27 (TriBitmap)

## Context

`specs/tri/collections/bitmap.t27` declares module `TriBitmap`: a fixed-capacity
bit set backed by an array of machine words. Its own header comment says
"Uses usize words for efficiency", and the declared type is

```
pub const Bitmap = struct {
    bits : []usize,
    capacity : usize,
};
```

`capacity` is a **bit** count, not a word count — the module's own test
`capacity_is_a_bit_count_not_a_word_count` states that 1024 bits needs
`ceil(1024 / @bitSizeOf(usize))` words, i.e. 16 words on a 64-bit target and
emphatically not 1024. The struct has exactly two fields and stores no
allocator (test `bitmap_is_a_slice_plus_a_length`), so `init` takes the
allocator as a parameter and does not retain it. Its neighbours in
`specs/tri/collections/` are `bitvector.t27` (a growable variant) and
`bitset.t27`; this one is the fixed-capacity member of that family.

`.t27` is a hand-authored source language, compiled out to C/Rust/Verilog/Zig
by `t27c`. You are writing source, not compiler output.

## Exact current state

Run from the repo root `/Users/playom/t27`.

```
$ ./target/release/t27c count specs/tri/collections/bitmap.t27 | head -1
bitmap.t27: 7 node types

$ ./target/release/t27c parse specs/tri/collections/bitmap.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ grep -c 'TODO: Implement' specs/tri/collections/bitmap.t27
10
```

The file is 101 lines, parses clean today, and **every one of its ten
functions is an empty body**. Keep it parse-clean: 90.3% of `specs/` (457/506)
is clean on all three parse metrics, and this file is currently in that set.
Do not drop it out.

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. Keep every
signature **verbatim** — same name, same parameter names and types, same `->`
return type. Keep the `module TriBitmap;` header form (semicolon, indented
declarations, no closing brace) exactly as it is.

Let `W = @bitSizeOf(usize)`, `word = index / W`, `off = index % W`.

1. `fn init(capacity: usize, allocator: std.mem.Allocator) -> !Bitmap`
   Allocate `ceil(capacity / W)` words, zero them, return
   `Bitmap{ .bits = ..., .capacity = capacity }`. The allocator is used and
   then forgotten — the struct has no allocator field.
2. `fn get(bitmap: Bitmap, index: usize) -> bool`
   Return bit `index`. Out-of-range `index` (>= `capacity`) must not read past
   the slice.
3. `fn set(bitmap: *Bitmap, index: usize) -> void` — OR in `1 << off`.
4. `fn clear(bitmap: *Bitmap, index: usize) -> void` — AND with `~(1 << off)`.
5. `fn flip(bitmap: *Bitmap, index: usize) -> void` — XOR with `1 << off`.
6. `fn set_all(bitmap: *Bitmap) -> void` — every word becomes all-ones. Bits
   past `capacity` in the final word are padding; leaving them set is
   acceptable only if `count` and `find_last` do not count them, so prefer
   masking the tail word.
7. `fn clear_all(bitmap: *Bitmap) -> void` — every word becomes 0.
8. `fn count(bitmap: Bitmap) -> usize` — population count over all words.
9. `fn find_first(bitmap: Bitmap) -> ?usize` — index of the lowest set bit, or
   `null` when none is set.
10. `fn find_last(bitmap: Bitmap) -> ?usize` — index of the highest set bit, or
    `null` when none is set.

The three existing test blocks are the specification for the *shape* and must
stay exactly as they are:

```
    test bits_are_packed_into_usize_words
        then @FieldType(Bitmap, "bits") == []usize
        and @bitSizeOf(usize) >= 32
```

Add one new test block per function you implement. Both test forms parse
clean in this file — the unbraced `test name / given / when / then` form used
above, and the braced `test "name" { ... }` form. Prefer the unbraced form to
match the file. There is **no t27 interpreter**: a test block is parsed and
counted, never executed, so write tests that state the intended behaviour
rather than tests tuned to pass some runner.

For a style reference of real function bodies in a parse-clean file, read
(do not edit) `specs/numeric/gf16.t27` — 53 implemented functions, all three
parse metrics 0.

Do **not** gate yourself on `t27c typecheck`. It returns `{"ok":true,"errors":0}`
on two lines of English prose; it proves nothing.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 10 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/collections/bitmap.t27` prints 0
- 3. the real-code node count is at least 20
- 4. `t27c parse specs/tri/collections/bitmap.t27 | grep -c 'kind: TestBlock'` prints at least 13
- 5. `t27c count specs/tri/collections/bitmap.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `10 names, fns=10 nonempty=0, clean` | `10 names, fns=n nonempty=n (n>=10), clean` |
| 2 | stub marker | `10` | `0` |
| 3 | real-code nodes | `0` | `>= 20` |
| 4 | test blocks | `3` | `>= 13` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 10 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|get|set|clear|flip|set_all|clear_all|count|find_first|find_last)\(' specs/tri/collections/bitmap.t27
```

Must print `10`. It prints `10` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/collections/bitmap.t27 <<'EOF'
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

Today it prints `fns=10 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 10**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 10 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/collections/bitmap.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/collections/bitmap.t27
```

Today it prints `10`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/collections/bitmap.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `20`.

The bar is two nodes per stubbed function (10 x 2 = 20 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/collections/bitmap.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `3`. It must print at least `13` — the 3 blocks already in the file
plus one new block for each of the 10 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/collections/bitmap.t27 | head -1
```

Today it prints `bitmap.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 10 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/collections/bitmap.t27 prints 0: met
- 3. the real-code node count is at least 20: met
- 4. t27c parse specs/tri/collections/bitmap.t27 | grep -c 'kind: TestBlock' prints at least 13: met
- 5. t27c count specs/tri/collections/bitmap.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/collections/bitmap.t27
