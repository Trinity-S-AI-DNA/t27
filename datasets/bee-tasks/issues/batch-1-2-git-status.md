# Implement the ten comment-only functions in specs/git/status.t27 (GitStatus)

## Context

`specs/git/status.t27` declares module `GitStatus`, the working-tree status
surface of the `specs/git/` family (`diff.t27`, `operations.t27`,
`schema.t27` are its neighbours; this file does `use git::schema;` for the
`Item`, `Kind` and `GitError` types it manipulates). It has two halves: five
functions that shell out to `git status` in various porcelain formats, and
eight pure functions that filter and aggregate the resulting `[Item]` list.
Three of the pure functions are already implemented — `filter_added`,
`filter_deleted` and `filter_modified` all delegate to `filter_by_status`,
which is itself still a comment. Nine test blocks already exercise the
aggregation half, so **the tests here are a real specification**, not
placeholders.

`.t27` is a hand-authored source language, compiled out to C/Rust/Verilog/Zig
by `t27c`. You are writing source, not compiler output.

## Exact current state

Run from the repo root `/Users/playom/t27`.

```
$ ./target/release/t27c count specs/git/status.t27 | head -1
status.t27: 16 node types

$ ./target/release/t27c parse specs/git/status.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ grep -c '// Implementation' specs/git/status.t27
10
```

Note the marker in this file is `// Implementation:`, **not**
`TODO: Implement` — `grep -c 'TODO: Implement' specs/git/status.t27` already
prints 0 and proves nothing here. 189 lines, parse-clean today; keep it that
way (457/506 of `specs/` is clean on all three parse metrics and this file is
in that set).

## What to write

Replace each `// Implementation: ...` comment with a real body. Keep every
signature **verbatim**, and keep the braced `module GitStatus { ... }` header
form this file uses (unlike the `specs/tri/` files, which use
`module Name;` with no braces).

Shell-out half — each returns `Result<..., GitError>`; run the command the
existing comment names, map a non-zero exit to a `GitError`, parse stdout:

1. `fn status(cwd: str) -> Result<[Item], GitError>` — run
   `git status --porcelain=v1 --untracked-files=all`, parse each line into an
   `Item { file, code, status }`. The two-character code maps to `Kind` by the
   rules the last test pins down (see below).
2. `fn is_clean(cwd: str) -> Result<bool, GitError>` — `git diff --quiet` and
   `git diff --cached --quiet` both succeeding.
3. `fn status_short(cwd: str) -> Result<str, GitError>` — `git status --short`.
4. `fn status_long(cwd: str) -> Result<str, GitError>` — `git status --long`.
5. `fn status_ignored(cwd: str) -> Result<[str], GitError>` — `git status --ignored`,
   returning the ignored paths.

Pure half — these are what the tests constrain:

6. `fn filter_by_status(items: [Item], kind: Kind) -> [Item]` — every item whose
   `.status` equals `kind`, order preserved. `filter_added`, `filter_deleted`
   and `filter_modified` already call it, and three existing tests assert both
   the count and the surviving order, e.g.

```
    test "filter_added_filters_correctly" {
        var items = [
            Item { file = "a.t27", code = "A", status = Kind::Added },
            Item { file = "b.t27", code = "M", status = Kind::Modified },
            Item { file = "c.t27", code = "D", status = Kind::Deleted },
            Item { file = "d.t27", code = "??", status = Kind::Added },
        ];
        var added = filter_added(items);
        assert(added.len == 2);
        assert(added[0].file == "a.t27");
        assert(added[1].file == "d.t27");
    }
```

7. `fn filter_by_pattern(items: [Item], pattern: str) -> [Item]` — items whose
   `.file` matches `pattern` (glob-style, `*` and `?`), order preserved.
8. `fn count_by_kind(items: [Item]) -> (added: u32, deleted: u32, modified: u32)` —
   test `count_by_kind_counts_correctly` fixes the answer for its four-item
   input at `added == 2, deleted == 1, modified == 1`.
9. `fn get_changed_files(items: [Item]) -> [str]` — the `.file` of every item,
   in order; test `get_changed_files_extracts_paths` asserts `files.len == 2`
   and `files.contains(...)`.
10. `fn has_changes(items: [Item]) -> bool` — `items` is not empty. Two tests
    pin both directions.

One existing test calls a function this file does not declare:

```
    test "status_code_parsing" {
        var added = parse_kind("A ");
        ...
        var untracked = parse_kind("??");
        assert(untracked == Kind::Added);
        var unmerged = parse_kind("UU");
        assert(unmerged == Kind::Modified);
    }
```

`parse_kind` belongs to `git::schema` and is **out of scope** — do not add it
here and do not edit `specs/git/schema.t27`. Use it from `status` as the test
implies it exists.

Do not delete or rewrite any existing test block. Add at least three new ones,
in the file's braced `test "name" { ... }` style, for the three pure functions
that have no test today: `filter_by_status`, `filter_by_pattern` and
`is_clean`. There is **no t27 interpreter** — a test block is parsed and
counted, never executed — so write tests that state intent, not tests tuned to
a runner.

Do **not** gate yourself on `t27c typecheck`: it returns
`{"ok":true,"errors":0}` on two lines of English prose.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 13 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c '// Implementation' specs/git/status.t27` prints 0
- 3. the real-code node count is at least 118
- 4. `t27c parse specs/git/status.t27 | grep -c 'kind: TestBlock'` prints at least 19
- 5. `t27c count specs/git/status.t27 | head -1` reports at least 18 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `13 names, fns=13 nonempty=3, clean` | `13 names, fns=n nonempty=n (n>=13), clean` |
| 2 | stub marker | `10` | `0` |
| 3 | real-code nodes | `98` | `>= 118` |
| 4 | test blocks | `9` | `>= 19` |
| 5 | node types | `16` | `>= 18` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 13 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (status|is_clean|status_short|status_long|status_ignored|filter_by_status|filter_by_pattern|filter_added|filter_deleted|filter_modified|count_by_kind|get_changed_files|has_changes)\(' specs/git/status.t27
```

Must print `13`. It prints `13` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/git/status.t27 <<'EOF'
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

Today it prints `fns=13 nonempty=3`. The two numbers must be **equal**, and the first
must be **at least 13**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 13 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/git/status.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c '// Implementation' specs/git/status.t27
```

Today it prints `10`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/git/status.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `98`. It must print at least `118`.

The bar is two nodes per stubbed function (10 x 2 = 20 on top of the 98 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/git/status.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `9`. It must print at least `19` — the 9 blocks already in the file
plus one new block for each of the 10 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/git/status.t27 | head -1
```

Today it prints `status.t27: 16 node types`. It must report at least `18`. For scale, 50.0% of
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
- 1. all 13 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c '// Implementation' specs/git/status.t27 prints 0: met
- 3. the real-code node count is at least 118: met
- 4. t27c parse specs/git/status.t27 | grep -c 'kind: TestBlock' prints at least 19: met
- 5. t27c count specs/git/status.t27 | head -1 reports at least 18 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Named above only as a neighbour you must not drift into. No issue owns these
and nobody is working on them: `specs/git/schema.t27`, `specs/git/diff.t27`, `specs/git/operations.t27`.

## Boundary

specs/git/status.t27
