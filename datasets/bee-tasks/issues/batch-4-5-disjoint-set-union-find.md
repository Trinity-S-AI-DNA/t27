# Implement the four stubbed union-find functions in `TriDisjointSet` (specs/tri/graph/disjoint_set.t27)

## Context

`specs/tri/graph/disjoint_set.t27` is the union-find structure of the `tri` graph family — the
component-tracking primitive that Kruskal-style algorithms in that directory need. It declares
`DisjointSet { parent : []usize, rank : []usize, count : usize }`: two parallel arrays indexed by
element id plus a live component count. There is deliberately **no** allocator field — the file's
own test says so — so `init(size)` borrows its backing memory rather than owning it. The header
comment names the union strategy: *"Attach shorter tree under taller tree root"*, i.e. union by
rank. Four functions are declared — `init`, `find`, `union`, `connected` — and all four are stubs.
The file's test-section comment is blunt about the consequence: *"init, find, union and connected
are all unimplemented stubs ... so calling any of them aborts the test binary rather than failing a
test."* The four existing tests build the forest by hand instead. Your job is to make them callable.

`.t27` is a hand-authored source language, not compiler output. You are writing source.

## Exact current state

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/graph/disjoint_set.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

The file parses clean today — including `fn union(...)`, whose name shadows a keyword in some
dialects. **Do not regress that, and do not rename the function to dodge it.**

```
$ ./target/release/t27c count specs/tri/graph/disjoint_set.t27 | head -1
disjoint_set.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/graph/disjoint_set.t27
4
```

```
$ grep -cE '^    fn ' specs/tri/graph/disjoint_set.t27
4
```

Four declared functions, four stubs: every function in this module is a signature with an empty
body.

## What to write

Fill each body. **Keep every signature byte-for-byte as it is** — same name, same parameter names,
same arrow-return type. Only the body changes. The signatures, verbatim from the file:

```
    fn init(size: usize) -> DisjointSet {
    fn find(ds: *DisjointSet, x: usize) -> usize {
    fn union(ds: *DisjointSet, x: usize, y: usize) -> void {
    fn connected(ds: *const DisjointSet, x: usize, y: usize) -> bool {
```

What each must do:

- `init` — return the singleton forest over `size` elements: `parent[i] == i` for every `i`, every
  `rank[i] == 0`, `count == size`. The struct has no allocator, so the body must state where the two
  slices come from — take them from a module-level backing buffer or a comptime-sized array and say
  so in a comment. Do not add an `allocator` field: a test asserts it is absent.
- `find` — return the root of `x`: follow `parent` until `parent[r] == r`. Apply **path compression**
  on the way back so every node on the path points straight at the root; `ds` is a mutable pointer
  precisely so you can. `find(root)` is the root itself.
- `union` — `find` both roots; if they are already equal, return without touching anything (in
  particular without decrementing `count`). Otherwise attach the **shorter** tree under the taller,
  per the header comment: compare ranks, make the lower-rank root's `parent` the higher-rank root,
  and only when the two ranks are equal increment the surviving root's rank by 1. Decrement `count`
  by exactly 1 on a real merge.
- `connected` — `true` when `x` and `y` share a root. `ds` is `*const` here, so this one must not
  compress the path or mutate anything; walk the parents read-only.

The four existing tests are the specification and must not be edited. Verbatim, the two that fix
the representation:

```
    test parent_and_rank_are_parallel_arrays_over_the_same_elements
        // One slot per element in each: index i is element i in both. No
        // allocator is stored, so init(size) borrows its backing memory.
        then @typeInfo(DisjointSet).@"struct".fields.len == 3
        and @FieldType(DisjointSet, "parent") == []usize
        and @FieldType(DisjointSet, "rank") == @FieldType(DisjointSet, "parent")
        and @hasField(DisjointSet, "allocator") == false

    test one_union_drops_the_component_count_by_exactly_one
        // Attaching element 1 under root 0 leaves three components, not four,
        // and the taller root's rank goes up while the absorbed one keeps its.
        given parent = [_]usize{ 0, 0, 2, 3 }
        given rank = [_]usize{ 1, 0, 0, 0 }
        given ds = DisjointSet{ .parent = @constCast(&parent), .rank = @constCast(&rank), .count = 3 }
        then ds.parent[1] == 0
        and ds.rank[0] == 1
        and ds.count == ds.parent.len - 1
        and ds.count >= 1
```

Three fields, no allocator, and one union costs exactly one component.

Add one `test` block per function you implement, appended to the existing test section. Match the
house style already in this file and in `specs/fpga/uart.t27` — bare name, then `given` / `and` /
`when` / `then` lines, no braces:

```
    test union_of_two_already_joined_elements_is_a_no_op
        given ds = init(4)
        and   union(&ds, 0, 1)
        when before = ds.count
        and   union(&ds, 0, 1)
        then ds.count == before
```

Body style: `specs/fpga/uart.t27` lines 60-109 is the house form of a filled `-> type { }` body.
Constructs measured to parse clean here: `var`/`const` locals, `while (cond) { }`, `if (cond) { }`,
`return expr;`, indexing, field access, struct literals, `&x`.

**Do not** rewrite the `DisjointSet` declaration, the header comment, the test-section commentary,
or the four existing test blocks. (The commentary at lines 47-52 describes the stubs you are
removing; you may update that paragraph to match reality, but nothing below it.)

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 4 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/graph/disjoint_set.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/graph/disjoint_set.t27 | grep -c 'kind: TestBlock'` prints at least 8
- 5. `t27c count specs/tri/graph/disjoint_set.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `4` | `>= 8` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|find|union|connected)\(' specs/tri/graph/disjoint_set.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/graph/disjoint_set.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/graph/disjoint_set.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/graph/disjoint_set.t27
```

Today it prints `4`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/graph/disjoint_set.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/graph/disjoint_set.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `8` — the 4 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/graph/disjoint_set.t27 | head -1
```

Today it prints `disjoint_set.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/graph/disjoint_set.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/graph/disjoint_set.t27 | grep -c 'kind: TestBlock' prints at least 8: met
- 5. t27c count specs/tri/graph/disjoint_set.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/graph/disjoint_set.t27
