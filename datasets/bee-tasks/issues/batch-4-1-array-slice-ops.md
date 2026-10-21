# Implement the nine stubbed slice/search functions in `TriArray` (specs/tri/collections/array.t27)

## Context

`specs/tri/collections/array.t27` is the zero-copy array-view module of the `tri` collections
family (it sits beside `bitvector.t27`, `bitmap.t27`, `linked_list.t27` in the same directory).
Its header comment states the module's purpose: *"ArrayView provides zero-copy access"*. It declares
two types — `ArrayView(T)`, a `{ ptr, len }` pair that borrows storage instead of duplicating it,
and `SliceRange`, a `{ start, end, step }` triple with a **signed** `step` so ranges can run
backwards. On top of those it declares nine free functions over `[]const T`: windowing (`slice`,
`slice_from`), endpoint access (`first`, `last`, `is_empty`), linear search (`contains`,
`index_of`) and two allocating builders (`reverse`, `concat`). Every one of the nine is a stub.
The six existing `test` blocks already pin the *types* — that a view is zero-copy, that `start` is
inclusive and `end` exclusive, that `step` is signed — so the semantics you must implement are not
open questions: they are written down.

`.t27` is a hand-authored source language, not compiler output. You are writing source.

## Exact current state

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/collections/array.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

The file parses clean today. **Do not regress that.**

```
$ ./target/release/t27c count specs/tri/collections/array.t27 | head -1
array.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/collections/array.t27
9
```

```
$ grep -cE '^    fn ' specs/tri/collections/array.t27
9
```

Nine declared functions, nine stubs: every function in this module is a signature with an empty
body.

## What to write

Fill each body. **Keep every signature byte-for-byte as it is** — same name, same parameter names,
same arrow-return type. Only the body changes. The signatures, verbatim from the file:

```
    fn slice(arr: []const T, start: usize, end: usize) -> []const T {
    fn slice_from(arr: []const T, start: usize) -> []const T {
    fn first(arr: []const T) -> T {
    fn last(arr: []const T) -> T {
    fn is_empty(arr: []const T) -> bool {
    fn contains(arr: []const T, item: T) -> bool {
    fn index_of(arr: []const T, item: T) -> ?usize {
    fn reverse(allocator: std.mem.Allocator, arr: []const T) -> []T {
    fn concat(allocator: std.mem.Allocator, a: []const T, b: []const T) -> []T {
```

What each must do:

- `slice` — return the window `arr[start..end]`. `start` is inclusive, `end` is exclusive; when
  `start == end` the result has length 0. This is fixed by the tests below, not your choice.
- `slice_from` — `arr[start..arr.len]`; `start == arr.len` yields an empty window.
- `first` / `last` — element `0` and element `arr.len - 1`. Both are partial: an empty input has no
  answer, so guard with `is_empty` and make the empty case explicit in the body rather than
  silently indexing out of range.
- `is_empty` — `arr.len == 0`.
- `contains` — linear scan, `true` on the first equal element, `false` after the scan ends.
- `index_of` — the same scan but returning `?usize`: the index of the first equal element, `null`
  when there is none. `contains` and `index_of` must agree — `contains` is `index_of(...) != null`.
- `reverse` — allocate `arr.len` elements from `allocator` and fill them in reverse order. The
  input is `[]const T` and the result `[]T`: a fresh buffer, not an in-place mutation.
- `concat` — allocate `a.len + b.len` elements, copy `a` then `b`.

The tests in the file are the specification for the range convention. Quoted verbatim:

```
    test slice_range_start_is_inclusive_and_end_is_exclusive
        // Verify: the field pair matches Zig's own slice syntax, so a range of
        // 1..4 over five elements selects three of them starting at index 1
        given data = [_]u8{ 0, 1, 2, 3, 4 }
        and range = SliceRange{ .start = 1, .end = 4, .step = 1 }
        when window = data[range.start..range.end]
        and window_matches = std.mem.eql(u8, window, &[_]u8{ 1, 2, 3 })
        then window.len == 3 and window_matches

    test an_empty_slice_range_has_equal_bounds
        // Verify: start == end selects nothing, which is how is_empty distinguishes
        // an empty view from a one-element one
        given data = [_]u8{ 0, 1, 2 }
        and range = SliceRange{ .start = 2, .end = 2, .step = 1 }
        when window = data[range.start..range.end]
        then window.len == 0
```

Add one `test` block per function you implement, appended to the existing test section at the
bottom of the file. Match the house style already in this file and in `specs/fpga/uart.t27` — bare
name, then `given` / `and` / `when` / `then` lines, no braces:

```
    test index_of_returns_null_when_the_item_is_absent
        given data = [_]u8{ 1, 2, 3 }
        when found = index_of(&data, 9)
        then found == null
```

Body style: look at `specs/fpga/uart.t27` lines 60-109 for the house form of a filled `-> type { }`
body (statements, `if`, `return`, `while`). Constructs measured to parse clean in this dialect:
`var`/`const` locals, `while (cond) { }`, `if (cond) { }`, `return expr;`, field access, indexing,
struct literals, `&x`.

**Do not** rewrite the type declarations, the header comment, or the six existing test blocks.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 9 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/collections/array.t27` prints 0
- 3. the real-code node count is at least 18
- 4. `t27c parse specs/tri/collections/array.t27 | grep -c 'kind: TestBlock'` prints at least 15
- 5. `t27c count specs/tri/collections/array.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `9 names, fns=9 nonempty=0, clean` | `9 names, fns=n nonempty=n (n>=9), clean` |
| 2 | stub marker | `9` | `0` |
| 3 | real-code nodes | `0` | `>= 18` |
| 4 | test blocks | `6` | `>= 15` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 9 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (slice|slice_from|first|last|is_empty|contains|index_of|reverse|concat)\(' specs/tri/collections/array.t27
```

Must print `9`. It prints `9` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/collections/array.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/collections/array.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/collections/array.t27
```

Today it prints `9`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/collections/array.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `18`.

The bar is two nodes per stubbed function (9 x 2 = 18 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/collections/array.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `6`. It must print at least `15` — the 6 blocks already in the file
plus one new block for each of the 9 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/collections/array.t27 | head -1
```

Today it prints `array.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/collections/array.t27 prints 0: met
- 3. the real-code node count is at least 18: met
- 4. t27c parse specs/tri/collections/array.t27 | grep -c 'kind: TestBlock' prints at least 15: met
- 5. t27c count specs/tri/collections/array.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/collections/array.t27
