# Implement the five stubbed functions in specs/tri/collections/ring_buffer.t27 (TriRing)

## Context

`specs/tri/collections/ring_buffer.t27` declares module `TriRing`: a
fixed-capacity circular queue, generic over its element type. Header comment:
"Wraps around when full". The type is

```
pub const Ring(T) = struct {
    buffer : []T,
    head : usize,
    tail : usize,
    capacity : usize,
};
```

Its six existing test blocks are unusually precise and effectively **are** the
specification: they fix that `head == tail` means empty, that indices advance
modulo `capacity`, that occupancy is `(tail + capacity - head) % capacity`, and
— crucially — that because this layout carries no separate element count, a
full ring and an empty ring are indistinguishable, so **a correct `push` must
stop at `capacity - 1` usable entries**. Its neighbour
`specs/tri/collections/circular_buffer.t27` is the non-generic sibling.

`.t27` is a hand-authored source language, compiled out to C/Rust/Verilog/Zig
by `t27c`. You are writing source, not compiler output.

## Exact current state

Run from the repo root `/Users/playom/t27`.

```
$ ./target/release/t27c count specs/tri/collections/ring_buffer.t27 | head -1
ring_buffer.t27: 7 node types

$ ./target/release/t27c parse specs/tri/collections/ring_buffer.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ grep -c 'TODO: Implement' specs/tri/collections/ring_buffer.t27
5
```

101 lines, parse-clean today, all five functions empty. Keep it in the clean
set (457/506 of `specs/`).

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. Keep every
signature **verbatim** and keep the `module TriRing;` header form (semicolon,
indented declarations, no closing brace).

1. `fn new(capacity: usize) -> Ring(T)`
   Return a ring with `head = 0`, `tail = 0`, `capacity = capacity` and a
   `buffer` of exactly `capacity` slots — test
   `a_fresh_ring_has_head_and_tail_at_the_same_slot` asserts
   `ring.buffer.len == ring.capacity`.
2. `fn push(ring: Ring(T), value: T) -> bool`
   Write `value` at `tail`, advance `tail` to `(tail + 1) % capacity`, return
   `true`. Return `false` without writing when the ring is full. Full means
   `(tail + 1) % capacity == head`, i.e. at most `capacity - 1` entries — this
   is what `a_full_ring_and_an_empty_ring_both_show_head_equal_to_tail`
   demands.
3. `fn pop(ring: Ring(T)) -> Option(T)`
   Return the element at `head` and advance `head` to `(head + 1) % capacity`;
   return the `Option` none case when `is_empty(ring)`.
4. `fn is_empty(ring: Ring(T)) -> bool` — `ring.head == ring.tail`.
5. `fn is_full(ring: Ring(T)) -> bool` — `(ring.tail + 1) % ring.capacity == ring.head`.

The two occupancy tests are the arithmetic you must match:

```
    test occupancy_is_tail_minus_head_modulo_capacity
        given storage = [_]u8{ 0, 0, 0, 0 }
        and ring = Ring(u8){ .buffer = @constCast(&storage), .head = 3, .tail = 1, .capacity = 4 }
        when used = (ring.tail + ring.capacity - ring.head) % ring.capacity
        then used == 2 and used < ring.capacity
```

Note that `push` and `pop` take `ring` by value in the declared signature.
Keep the signature as written — do not change it to a pointer. Express the
mutation in the body as the module intends; the parameter list is frozen.

Do not delete or rewrite the six existing test blocks. Add one new test block
per function you implement, in the file's unbraced
`test name / given / when / then` style, so at least five new ones. There is
**no t27 interpreter** — test blocks are parsed and counted, never executed.

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

- 1. all 5 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/collections/ring_buffer.t27` prints 0
- 3. the real-code node count is at least 10
- 4. `t27c parse specs/tri/collections/ring_buffer.t27 | grep -c 'kind: TestBlock'` prints at least 11
- 5. `t27c count specs/tri/collections/ring_buffer.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `5 names, fns=5 nonempty=0, clean` | `5 names, fns=n nonempty=n (n>=5), clean` |
| 2 | stub marker | `5` | `0` |
| 3 | real-code nodes | `0` | `>= 10` |
| 4 | test blocks | `6` | `>= 11` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 5 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (new|push|pop|is_empty|is_full)\(' specs/tri/collections/ring_buffer.t27
```

Must print `5`. It prints `5` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/collections/ring_buffer.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/collections/ring_buffer.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/collections/ring_buffer.t27
```

Today it prints `5`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/collections/ring_buffer.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `10`.

The bar is two nodes per stubbed function (5 x 2 = 10 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/collections/ring_buffer.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `6`. It must print at least `11` — the 6 blocks already in the file
plus one new block for each of the 5 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/collections/ring_buffer.t27 | head -1
```

Today it prints `ring_buffer.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/collections/ring_buffer.t27 prints 0: met
- 3. the real-code node count is at least 10: met
- 4. t27c parse specs/tri/collections/ring_buffer.t27 | grep -c 'kind: TestBlock' prints at least 11: met
- 5. t27c count specs/tri/collections/ring_buffer.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Owned by another issue in this set, so definitely not yours: `specs/tri/collections/circular_buffer.t27`.
Named above only as a neighbour you must not drift into. No issue owns it
and nobody is working on it: `specs/tri/collections/queue.t27`.

## Boundary

specs/tri/collections/ring_buffer.t27
