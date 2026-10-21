# Implement the five stubbed functions in specs/tri/collections/circular_buffer.t27

## Context

`specs/tri/collections/circular_buffer.t27` is the fixed-capacity ring of the `tri`
standard library (`module TriCircularBuffer;`), one of ~29 container modules in
`specs/tri/collections/`. Its header states the behaviour that defines it: *"Wraps around
when full"* — head and tail are ring positions modulo `capacity`, not monotonic indices.
It is the concrete, `i64`-typed, allocator-owning sibling of `ring_buffer.t27` (`module
TriRing;`), which declares a generic `Ring(T)` with no allocator; this file is the one that
must actually manage memory. The struct is
`CircularBuffer { data : []i64, head : usize, tail : usize, capacity : usize, allocator : std.mem.Allocator }`
and all five functions are signature-only stubs. `.t27` is hand-authored source; `t27c`
compiles it out to C/Rust/Verilog/Zig.

## Current state (commands actually run, 2026-09-09)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/collections/circular_buffer.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/collections/circular_buffer.t27 | head -1
circular_buffer.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/collections/circular_buffer.t27
5
$ grep -cE '^    fn ' specs/tri/collections/circular_buffer.t27
5
$ grep -cE '^    test ' specs/tri/collections/circular_buffer.t27
3
$ grep -cE '^ +(return|let|var|if|for|while|match) ' specs/tri/collections/circular_buffer.t27
0
```

Five declared functions, zero statements.

## What to write

Keep every signature byte-for-byte; replace only each `// TODO: Implement from .tri spec`.

### 1. `fn init(allocator: std.mem.Allocator, capacity: usize) -> CircularBuffer`

```
    fn init(allocator: std.mem.Allocator, capacity: usize) -> CircularBuffer {
```

Allocate a `data` slice of `capacity` elements through `allocator`, set `head = tail = 0`,
store both `capacity` and `allocator`. The existing test
`circular_buffer_capacity_matches_its_backing_array` requires `capacity == data.len`, so
never store a capacity that does not match the slice you allocated.

### 2. `fn write(buf: *CircularBuffer, value: i64) -> !void`

```
    fn write(buf: *CircularBuffer, value: i64) -> !void {
```

Store `value` at `buf.data[buf.head]`, then advance `head` **modulo capacity** — that
wrap is the whole point of the module. The `!` return type is where a full-buffer
condition goes: state your full/overwrite policy in a comment above the body and be
consistent with `read` and `is_empty`. Zero capacity must not divide by zero.

### 3. `fn read(buf: *CircularBuffer) -> i64`

```
    fn read(buf: *CircularBuffer) -> i64 {
```

Return `buf.data[buf.tail]` and advance `tail` modulo `capacity`. The signature carries no
error and no optional, so callers are expected to check `is_empty` first; say so in a
comment rather than inventing a sentinel value.

### 4. `fn is_empty(buf: *CircularBuffer) -> bool`

```
    fn is_empty(buf: *CircularBuffer) -> bool {
```

Return `buf.head == buf.tail`. The first existing test,
`circular_buffer_empty_state_has_head_meeting_tail`, states exactly this condition, so do
not pick a different emptiness rule.

### 5. `fn deinit(buf: *CircularBuffer) -> void`

```
    fn deinit(buf: *CircularBuffer) -> void {
```

Free `data` through the stored allocator and reset `head`, `tail` and `capacity` to 0.

### The existing tests are the specification

Already in the file. Do not delete or weaken them.

```
    test circular_buffer_empty_state_has_head_meeting_tail
        given empty = CircularBuffer{.data=&[_]i64{},.head=0,.tail=0,.capacity=0,.allocator=std.testing.allocator}
        then empty.head == empty.tail
        and empty.capacity == 0
        and empty.data.len == 0

    test circular_buffer_capacity_matches_its_backing_array
        given buf = CircularBuffer{.data=@constCast(&[_]i64{7,8,9,10}),.head=3,.tail=1,.capacity=4,.allocator=std.testing.allocator}
        then buf.capacity == buf.data.len
        and buf.head != buf.tail
        and buf.data[buf.tail] == 8

    test circular_buffer_indices_stay_inside_the_capacity
        given wrapped = CircularBuffer{.data=@constCast(&[_]i64{1,2,3,4}),.head=3,.tail=1,.capacity=4,.allocator=std.testing.allocator}
        then wrapped.head > wrapped.tail
        and wrapped.head < wrapped.capacity
        and wrapped.tail < wrapped.capacity
```

Read the third one carefully: **`head > tail` is legal**. Both are ring positions strictly
below `capacity`, in either order. Any implementation that assumes `tail <= head`
contradicts a test already in the file.

Add **at least five new `test` blocks**, one per function, in the file's own style
(`    test some_name` on its own line, then indented `given` / `when` / `then` lines).
Good subjects: a freshly initialised buffer has `head == tail == 0` and
`capacity == data.len`; advancing `head` from `capacity - 1` wraps to 0; after `deinit`
the capacity is 0; `is_empty` is exactly `head == tail`.

### Style notes, measured

Ordinary statement syntax parses clean inside a body — verified on copies of files in this
corpus: `let i = buf.head;`, `buf.data[i] = value;`, `if (buf.capacity == 0) { ... }`,
`while (i > 0) { ... }`, `for (0..n) |i| { ... }`,
`return CircularBuffer{ .data = d, .head = 0, .tail = 0, .capacity = capacity, .allocator = allocator };`
all give recovery-events 0. A six-function fill on the sibling file
`priority_queue.t27` was rebuilt and re-measured on 2026-09-09: it took that file
from 7 node types to 21 with recovery-events 0, so criterion 5 below is reachable. Four-space indent for `fn` and `test` lines, eight for statements.

**There is no `.t27` interpreter.** Test blocks are parsed and counted, never executed.
Write them — they are the house style — but do not claim you ran them.

**Do not lean on `t27c typecheck`.** It returns `{"ok":true,"errors":0}` on English prose
and on Python files. It is not a gate.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 5 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/collections/circular_buffer.t27` prints 0
- 3. the real-code node count is at least 10
- 4. `t27c parse specs/tri/collections/circular_buffer.t27 | grep -c 'kind: TestBlock'` prints at least 8
- 5. `t27c count specs/tri/collections/circular_buffer.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `5 names, fns=5 nonempty=0, clean` | `5 names, fns=n nonempty=n (n>=5), clean` |
| 2 | stub marker | `5` | `0` |
| 3 | real-code nodes | `0` | `>= 10` |
| 4 | test blocks | `3` | `>= 8` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 5 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|write|read|is_empty|deinit)\(' specs/tri/collections/circular_buffer.t27
```

Must print `5`. It prints `5` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/collections/circular_buffer.t27 <<'EOF'
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

Today it prints `fns=5 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 5**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 5 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/collections/circular_buffer.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/collections/circular_buffer.t27
```

Today it prints `5`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/collections/circular_buffer.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `10`.

The bar is two nodes per stubbed function (5 x 2 = 10 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/collections/circular_buffer.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `3`. It must print at least `8` — the 3 blocks already in the file
plus one new block for each of the 5 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/collections/circular_buffer.t27 | head -1
```

Today it prints `circular_buffer.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 5 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/collections/circular_buffer.t27 prints 0: met
- 3. the real-code node count is at least 10: met
- 4. t27c parse specs/tri/collections/circular_buffer.t27 | grep -c 'kind: TestBlock' prints at least 8: met
- 5. t27c count specs/tri/collections/circular_buffer.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Owned by another issue in this set, so definitely not yours: `specs/tri/collections/ring_buffer.t27`, `specs/tri/collections/priority_queue.t27`.

## Boundary

specs/tri/collections/circular_buffer.t27
