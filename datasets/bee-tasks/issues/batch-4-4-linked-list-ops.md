# Implement the five stubbed functions in `TriLinkedList` and replace its five placeholder tests (specs/tri/collections/linked_list.t27)

## Context

`specs/tri/collections/linked_list.t27` is the doubly-linked list of the `tri` collections family
(beside `array.t27`, `bitvector.t27`, `bitmap.t27`). It declares `ListNode { value, prev, next }`
with **both** links optional, and `LinkedList { head, tail, length, allocator }` — a list that
tracks both ends and its own length, and that owns its nodes, which is why the allocator is stored
in the struct. The header comment names the responsibility that follows from that ownership:
*"Free all nodes"*. Five functions are declared — `init`, `append`, `prepend`, `remove`, `deinit` —
and all five are stubs.

This file has a second problem, and fixing it is part of the task. Its five `test` blocks are
generated placeholders: each one calls a non-existent `default_input()` and asserts
`result != undefined`, which pins nothing at all. They must be replaced with tests that actually
constrain the list.

`.t27` is a hand-authored source language, not compiler output. You are writing source.

## Exact current state

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/collections/linked_list.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

The file parses clean today. **Do not regress that.**

```
$ ./target/release/t27c count specs/tri/collections/linked_list.t27 | head -1
linked_list.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/collections/linked_list.t27
5
```

```
$ grep -cE '^    fn ' specs/tri/collections/linked_list.t27
5
```

```
$ grep -c 'default_input' specs/tri/collections/linked_list.t27
5
```

Five declared functions, five stubs, and five placeholder tests that assert nothing.

## What to write

### 1. Fill the five bodies

**Keep every signature byte-for-byte as it is** — same name, same parameter names, same
arrow-return type. Only the body changes. The signatures, verbatim from the file:

```
    fn init() -> LinkedList {
    fn append(list: *LinkedList, value: i64) -> !void {
    fn prepend(list: *LinkedList, value: i64) -> !void {
    fn remove(list: *LinkedList, value: i64) -> bool {
    fn deinit(list: *LinkedList) -> void {
```

What each must do:

- `init` — return an empty list: `head` null, `tail` null, `length` 0. Note the signature takes no
  allocator, so the `allocator` field is whatever the struct's default is; set the three other
  fields explicitly and leave a comment saying the caller fills `allocator` before the first
  `append`.
- `append` — allocate a node through `list.allocator`, link it after `list.tail` (`node.prev = tail`,
  `tail.next = node`, `node.next = null`), make it the new `tail`, and set `head` too when the list
  was empty. `length` increases by exactly 1. Returns `!void`, so allocation failure propagates.
- `prepend` — the mirror image at `head`: `node.next = head`, `head.prev = node`, `node.prev = null`,
  new `head`, and also the new `tail` when the list was empty. `length` increases by exactly 1.
- `remove` — walk from `head` to the **first** node whose `value` equals `value`; unlink it by
  patching its neighbours (`prev.next = node.next`, `next.prev = node.prev`), fix `head` when it was
  the head and `tail` when it was the tail, free the node through `list.allocator`, decrement
  `length`, return `true`. Return `false` and change nothing when no node matches. Removing the only
  node must leave `head` and `tail` both null and `length` 0.
- `deinit` — walk the chain from `head` freeing every node through `list.allocator` (capture
  `node.next` *before* freeing `node`), then leave `head` null, `tail` null, `length` 0. This is the
  function the header comment is named for.

The invariant every one of them must preserve, and which your tests should state: `length` equals
the number of reachable nodes; `head` is null exactly when `tail` is null; and `head.prev` and
`tail.next` are both null.

### 2. Replace the five placeholder tests

Delete these five blocks — they are generated stubs and `default_input()` does not exist:

```
    test init_basic_case
        given input = default_input()
        when result = init(input)
        then result != undefined
```

(and the identical `append_basic_case`, `prepend_basic_case`, `remove_basic_case`,
`deinit_basic_case`.)

Write real ones in their place — at least one per function, plus the empty-list and single-node
boundaries. Match the house style in `specs/fpga/uart.t27` — bare name, then `given` / `and` /
`when` / `then` lines, no braces:

```
    test append_to_an_empty_list_sets_both_ends
        given list = init()
        and   append(&list, 7)
        then list.length == 1
        and  list.head != null
        and  list.head == list.tail

    test remove_of_an_absent_value_changes_nothing
        given list = init()
        and   append(&list, 7)
        when removed = remove(&list, 9)
        then removed == false
        and  list.length == 1
```

Body style: `specs/fpga/uart.t27` lines 60-109 is the house form of a filled `-> type { }` body.
Constructs measured to parse clean here: `var`/`const` locals, `while (cond) { }`, `if (cond) { }`,
`return expr;`, field access, struct literals, `&x`, comparison against `null`.

**Do not** rewrite the `ListNode` / `LinkedList` declarations or the header comment.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 5 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/collections/linked_list.t27` prints 0
- 3. the real-code node count is at least 10
- 4. `t27c parse specs/tri/collections/linked_list.t27 | grep -c 'kind: TestBlock'` prints at least 10
- 5. `t27c count specs/tri/collections/linked_list.t27 | head -1` reports at least 11 node types
- 6. `grep -c 'default_input' specs/tri/collections/linked_list.t27` prints 0

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `5 names, fns=5 nonempty=0, clean` | `5 names, fns=n nonempty=n (n>=5), clean` |
| 2 | stub marker | `5` | `0` |
| 3 | real-code nodes | `0` | `>= 10` |
| 4 | test blocks | `5` | `>= 10` |
| 5 | node types | `7` | `>= 11` |
| 6 | placeholder tests replaced | `5` | `0` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 5 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|append|prepend|remove|deinit)\(' specs/tri/collections/linked_list.t27
```

Must print `5`. It prints `5` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/collections/linked_list.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/collections/linked_list.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/collections/linked_list.t27
```

Today it prints `5`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/collections/linked_list.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `10`.

The bar is two nodes per stubbed function (5 x 2 = 10 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/collections/linked_list.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `5`. It must print at least `10` — the 5 blocks already in the file
plus one new block for each of the 5 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/collections/linked_list.t27 | head -1
```

Today it prints `linked_list.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
the stub-free, parse-clean files in `specs/` are at or above that number, and the
median across them is 17.5.

`| head -1` is not optional. The rest of the `t27c count` table is **not reproducible**:
rows with equal counts print in a different order on every run (measured: 3 distinct
orderings in 8 consecutive runs over the same unchanged file), and the table stops
after 15 rows with `... and N more`. 14 files in `specs/` contain test blocks that
`t27c count | grep TestBlock` cannot see for exactly that reason — and they are the
richly-implemented ones, which is the state this task is trying to reach. Never read
a number off that table; the first line is the only stable part of the output.

**Criterion 6 — the five placeholder tests are gone.**

```
$ grep -c 'default_input' specs/tri/collections/linked_list.t27
```

Today it prints `5`. All five existing test blocks open with `given input = default_input()`
and `default_input` is not declared anywhere in the file — they are placeholders, not
tests. It must print `0`: replace each of them with a block that names a real value and a
real call. These five rewritten blocks count toward criterion 4 only if the parser accepts
them, which is why criterion 4 counts `kind: TestBlock` out of `t27c parse`.

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
- 2. grep -c 'TODO: Implement' specs/tri/collections/linked_list.t27 prints 0: met
- 3. the real-code node count is at least 10: met
- 4. t27c parse specs/tri/collections/linked_list.t27 | grep -c 'kind: TestBlock' prints at least 10: met
- 5. t27c count specs/tri/collections/linked_list.t27 | head -1 reports at least 11 node types: met
- 6. grep -c 'default_input' specs/tri/collections/linked_list.t27 prints 0: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/collections/linked_list.t27
