# Implement the six stubbed functions in specs/tri/collections/priority_queue.t27

## Context

`specs/tri/collections/priority_queue.t27` is the priority queue of the `tri` standard
library (`module TriPriorityQueue;`), one of ~29 container modules in
`specs/tri/collections/` alongside `queue.t27`, `deque.t27`, `stack.t27` and
`skip_list.t27`. It declares an owning, allocator-backed container —
`PriorityQueue { data : []i64, size : usize, allocator : std.mem.Allocator }` — with a
**signed** priority type, a `size` tracked separately from `data.len` (capacity), and a
stored allocator, which is why a `deinit` exists at all. All six functions are
signature-only stubs. `.t27` is hand-authored source; `t27c` compiles it out to
C/Rust/Verilog/Zig.

## Current state (commands actually run, 2026-09-09)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/collections/priority_queue.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/collections/priority_queue.t27 | head -1
priority_queue.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/collections/priority_queue.t27
6
$ grep -cE '^    fn ' specs/tri/collections/priority_queue.t27
6
$ grep -cE '^    test ' specs/tri/collections/priority_queue.t27
3
$ grep -cE '^ +(return|let|var|if|for|while|match) ' specs/tri/collections/priority_queue.t27
0
```

Six declared functions, not one statement between them.

## What to write

Implement a **binary min-heap** over `pq.data`, with `pq.size` as the live element count.
The comment block above the tests says the current state plainly: *"Every function above is
an unimplemented stub emitting `@panic("not yet implemented")`, so no heap ordering can be
exercised."* Your job is to make the ordering real.

Keep every signature byte-for-byte; replace only each `// TODO: Implement from .tri spec`.

### 1. `fn init(allocator: std.mem.Allocator) -> PriorityQueue`

```
    fn init(allocator: std.mem.Allocator) -> PriorityQueue {
```

Return an empty queue: `size = 0`, an empty `data` slice, and the allocator stored in the
struct. No allocation here — the queue owns its allocator, and growth happens on first
`enqueue`.

### 2. `fn enqueue(pq: *PriorityQueue, value: i64) -> !void`

```
    fn enqueue(pq: *PriorityQueue, value: i64) -> !void {
```

Grow `data` through `pq.allocator` when `pq.size == pq.data.len` (doubling, minimum
capacity 1 — this is the `!` in the return type: allocation is the only failure mode).
Write `value` at index `pq.size`, then **sift up**: while the element is smaller than its
parent at `(i - 1) / 2`, swap and continue. Increment `pq.size` last.

### 3. `fn dequeue(pq: *PriorityQueue) -> i64`

```
    fn dequeue(pq: *PriorityQueue) -> i64 {
```

Remove and return the minimum, which is `data[0]`. Move the last live element into slot 0,
decrement `size`, then **sift down**: repeatedly compare against the smaller of the two
children at `2*i + 1` and `2*i + 2` and swap while the child is smaller. The signature
returns a bare `i64` with no error and no optional, so state in a comment what an empty
queue does and keep it consistent with `is_empty` — callers are expected to check first.

### 4. `fn peek(pq: *PriorityQueue) -> i64`

```
    fn peek(pq: *PriorityQueue) -> i64 {
```

Return `data[0]` without mutating anything.

### 5. `fn is_empty(pq: *PriorityQueue) -> bool`

```
    fn is_empty(pq: *PriorityQueue) -> bool {
```

Return `pq.size == 0`. Note it is `size`, not `data.len`: those are two distinct numbers
and the existing test `size_is_tracked_separately_from_the_buffer` exists to say so.

### 6. `fn deinit(pq: *PriorityQueue) -> void`

```
    fn deinit(pq: *PriorityQueue) -> void {
```

Free `data` through the stored allocator and reset `size` to 0.

### The existing tests are the specification

Already in the file. Do not delete or weaken them.

```
    test priorities_are_signed
        then @FieldType(PriorityQueue, "data") == []i64

    test size_is_tracked_separately_from_the_buffer
        then @hasField(PriorityQueue, "size")
        and @FieldType(PriorityQueue, "size") == usize

    test the_queue_owns_its_allocator
        then @FieldType(PriorityQueue, "allocator") == std.mem.Allocator
        and @typeInfo(PriorityQueue).@"struct".fields.len == 3
```

`data : []i64` is signed on purpose — a negative priority is representable, so callers may
order by cost as well as by rank. Do not "fix" it to `usize`. The struct has exactly three
fields; adding a fourth breaks the third test.

Add **at least six new `test` blocks**, one per function, in the file's own style
(`    test some_name` on its own line, then indented `given` / `when` / `then` lines).
Good subjects: a freshly built queue has `size == 0` and `data.len == 0`; the root slot of
a heap-ordered array holds its minimum; a parent index is `(i - 1) / 2` for `i = 1, 2`;
children of index 0 are 1 and 2; a negative priority sorts ahead of a positive one.

### Style notes, measured

All six bodies were prototyped on a copy of this exact file and parsed with
recovery-events 0, declarations-swallowed 0 and lexer-discarded-chars 0. The statement
forms that were verified: `let i = pq.size;`, `pq.data[i] = value;`,
`while (i > 0) { ... }`, `if (pq.data[parent] <= pq.data[i]) { break; }`,
`pq.size = pq.size + 1;`, `return PriorityQueue{ .data = &[_]i64{}, .size = 0, .allocator = allocator };`,
`pq.allocator.free(pq.data);`. That prototype was rebuilt and re-measured on 2026-09-09: it takes the file from 7
node types to 21 with recovery-events 0, 42 real-code nodes and `fns=6 nonempty=6`,
so criteria 1, 3 and 5 below are comfortably reachable. Four-space indent for `fn` and `test` lines,
eight for statements.

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

- 1. all 6 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/collections/priority_queue.t27` prints 0
- 3. the real-code node count is at least 12
- 4. `t27c parse specs/tri/collections/priority_queue.t27 | grep -c 'kind: TestBlock'` prints at least 9
- 5. `t27c count specs/tri/collections/priority_queue.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `6 names, fns=6 nonempty=0, clean` | `6 names, fns=n nonempty=n (n>=6), clean` |
| 2 | stub marker | `6` | `0` |
| 3 | real-code nodes | `0` | `>= 12` |
| 4 | test blocks | `3` | `>= 9` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 6 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|enqueue|dequeue|peek|is_empty|deinit)\(' specs/tri/collections/priority_queue.t27
```

Must print `6`. It prints `6` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/collections/priority_queue.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/collections/priority_queue.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/collections/priority_queue.t27
```

Today it prints `6`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/collections/priority_queue.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `12`.

The bar is two nodes per stubbed function (6 x 2 = 12 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/collections/priority_queue.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `3`. It must print at least `9` — the 3 blocks already in the file
plus one new block for each of the 6 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/collections/priority_queue.t27 | head -1
```

Today it prints `priority_queue.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/collections/priority_queue.t27 prints 0: met
- 3. the real-code node count is at least 12: met
- 4. t27c parse specs/tri/collections/priority_queue.t27 | grep -c 'kind: TestBlock' prints at least 9: met
- 5. t27c count specs/tri/collections/priority_queue.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Owned by another issue in this set, so definitely not yours: `specs/tri/collections/circular_buffer.t27`.

## Boundary

specs/tri/collections/priority_queue.t27
