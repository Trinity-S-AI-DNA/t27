# Implement sort and sort_range in specs/tri/sort/quick_sort.t27

## Context

`specs/tri/sort/quick_sort.t27` is the quicksort of the Trinity sorting family
in `specs/tri/sort/` — ten modules covering insertion, selection, shell, heap,
merge, counting, radix, tim, the generic `sort.t27` facade, and this one. It has
made two decisions that its header and signatures record: the pivot rule ("Use
last element as pivot", the Lomuto partition) and that sorting is **in place** —
`sort` takes a mutable `[]i64` and returns `void`, so the caller's array is
reordered rather than copied into a freshly allocated one. Both functions are
empty bodies today, so as the file's own comment says, "neither the ordering nor
the last-element pivot rule in the header can be exercised". Your job is to
write those two bodies.

`.t27` is hand-authored source, compiled out to C/Rust/Verilog/Zig by `t27c`.
You are writing the input language, not compiler output.

## Exact current state

All commands run from `/Users/playom/t27`.

```
$ ./target/release/t27c parse specs/tri/sort/quick_sort.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/sort/quick_sort.t27 | head -1
quick_sort.t27: 5 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/sort/quick_sort.t27
2
```

```
$ grep -cE '^[[:space:]]*test ' specs/tri/sort/quick_sort.t27
2
```

The file parses clean today. Keep it clean.

## What to write

Two functions. Signatures quoted verbatim — change only the bodies.

1. `fn sort(values: []i64) -> void`
   The entry point. On a slice of fewer than two elements, return immediately.
   Otherwise delegate: `sort_range(values, 0, values.len - 1)`. Note the
   `- 1`: `high` is an inclusive index, and `values.len` is `usize`, so the
   empty-slice guard has to come first or the subtraction underflows. Say that
   in a comment — it is the one real trap in this file.

2. `fn sort_range(values: []i64, low: usize, high: usize) -> void`
   Lomuto quicksort over the inclusive range `[low, high]`, with the **last
   element as pivot** as the header demands:
   - return when `low >= high`;
   - `pivot = values[high]`; walk `j` from `low` to `high - 1`, keeping a
     boundary index `i` starting at `low`; whenever `values[j] <= pivot`, swap
     `values[i]` and `values[j]` and advance `i`;
   - swap `values[i]` with `values[high]` to seat the pivot;
   - recurse on `[low, i - 1]` and `[i + 1, high]`, guarding the `i - 1` case
     against `i == 0` underflow — again `usize`.

   A small `swap` helper is welcome; extra functions only add to the declaration
   count.

### The tests are the specification

Two test blocks exist. They pin the in-place shape:

```
    test sorting_is_in_place
        // a mutable slice in, nothing out -- the caller's array is reordered
        // rather than copied into a freshly allocated one
        given sort_type = @TypeOf(sort)
        then sort_type == fn ([]i64) void

    test sort_range_shares_the_in_place_signature
        given sort_range_type = @TypeOf(sort_range)
        then sort_range_type == fn ([]i64) void
```

Heads-up, and handle it honestly: the second block's assertion is **wrong today**
— `sort_range` takes three parameters, so its type is
`fn ([]i64, usize, usize) void`, not `fn ([]i64) void`. It was written against
the signature the module actually declares, and it contradicts it. Correct that
one assertion to the three-parameter type and note the fix in a comment. Leave
`test sorting_is_in_place` alone; it is right.

Add at least one new `test` block per implemented function stating what is now
expressible: that sorting a three-element array leaves it non-decreasing, that a
single-element and an empty slice are left untouched, that already-sorted and
reverse-sorted inputs both come out sorted, and that `sort_range` on a
sub-range leaves the elements outside `[low, high]` where they were. Also
replace the stale comment above the test section (the one beginning "Both
functions have empty bodies").

**There is no t27 interpreter.** Test blocks are parsed and counted, never
executed — nothing here checks that your quicksort actually sorts, so get it
right by reading it.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 2 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/sort/quick_sort.t27` prints 0
- 3. the real-code node count is at least 4
- 4. `t27c parse specs/tri/sort/quick_sort.t27 | grep -c 'kind: TestBlock'` prints at least 4
- 5. `t27c count specs/tri/sort/quick_sort.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `2 names, fns=2 nonempty=0, clean` | `2 names, fns=n nonempty=n (n>=2), clean` |
| 2 | stub marker | `2` | `0` |
| 3 | real-code nodes | `0` | `>= 4` |
| 4 | test blocks | `2` | `>= 4` |
| 5 | node types | `5` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 2 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (sort|sort_range)\(' specs/tri/sort/quick_sort.t27
```

Must print `2`. It prints `2` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/sort/quick_sort.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/sort/quick_sort.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/sort/quick_sort.t27
```

Today it prints `2`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/sort/quick_sort.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `4`.

The bar is two nodes per stubbed function (2 x 2 = 4 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/sort/quick_sort.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `2`. It must print at least `4` — the 2 blocks already in the file
plus one new block for each of the 2 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/sort/quick_sort.t27 | head -1
```

Today it prints `quick_sort.t27: 5 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/sort/quick_sort.t27 prints 0: met
- 3. the real-code node count is at least 4: met
- 4. t27c parse specs/tri/sort/quick_sort.t27 | grep -c 'kind: TestBlock' prints at least 4: met
- 5. t27c count specs/tri/sort/quick_sort.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/sort/quick_sort.t27
