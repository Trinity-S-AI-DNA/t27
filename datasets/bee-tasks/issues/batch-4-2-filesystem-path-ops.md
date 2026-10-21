# Implement the seven stubbed path functions in `TriFilesystem` (specs/tri/io/filesystem.t27)

## Context

`specs/tri/io/filesystem.t27` is the path-manipulation module of the `tri` I/O family. It is pure
string surgery over `[]const u8` paths — no syscalls, no directory reads. Its header comment states
the module's one global invariant: *"Results don't end with separator (except root)"*. It declares
two types: `PathError`, a five-variant enum (`invalid_path`, `not_found`, `not_a_directory`,
`not_a_file`, `permission_denied`) recovered from the upstream `.tri` spec, and `FileInfo`, the
`{ path, size, is_dir, is_file, modified }` record whose `is_dir`/`is_file` pair is mutually
exclusive. On top of those it declares seven path functions — `join`, `basename`, `dirname`, `ext`,
`has_ext`, `is_absolute`, `normalize` — and all seven are stubs. The file's own comment says why the
existing tests only exercise `FileInfo`: *"All seven path functions are unimplemented stubs
returning void, so no join/basename/dirname round trip can be asserted yet."* Your job is to make
that round trip assertable.

`.t27` is a hand-authored source language, not compiler output. You are writing source.

## Exact current state

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/io/filesystem.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

The file parses clean today. **Do not regress that.**

```
$ ./target/release/t27c count specs/tri/io/filesystem.t27 | head -1
filesystem.t27: 9 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/io/filesystem.t27
7
```

```
$ grep -cE '^    fn ' specs/tri/io/filesystem.t27
7
```

Seven declared functions, seven stubs: every function in this module is a signature with an empty
body.

## What to write

Fill each body. **Keep every signature byte-for-byte as it is** — same name, same parameter names,
same arrow-return type. Only the body changes. The signatures, verbatim from the file:

```
    fn join(allocator: std.mem.Allocator, parts: [][]const u8) -> ![]u8 {
    fn basename(path: []const u8) -> []const u8 {
    fn dirname(path: []const u8) -> []const u8 {
    fn ext(path: []const u8) -> []const u8 {
    fn has_ext(path: []const u8, ext: []const u8) -> bool {
    fn is_absolute(path: []const u8) -> bool {
    fn normalize(allocator: std.mem.Allocator, path: []const u8) -> ![]u8 {
```

The separator is `/`. What each must do:

- `is_absolute` — `true` when `path.len > 0` and `path[0] == '/'`. This is the only one with no
  allocation and no edge case beyond the empty path.
- `basename` — the segment after the last `/`. `"/usr/local/bin/t27c"` gives `"t27c"`;
  a path with no separator is returned whole; `"/"` gives `"/"` (the root exception in the header
  comment); a trailing separator must not produce an empty result.
- `dirname` — everything before the last `/`, **without** the trailing separator, per the header
  invariant. `"/usr/local/bin/t27c"` gives `"/usr/local/bin"`; `"/t27c"` gives `"/"` (root is the
  exception that keeps its separator); a path with no separator gives `"."`.
- `ext` — the extension of `basename(path)`, from the last `.` onward, `"."` included: `"a/b.tar.gz"`
  gives `".gz"`. A basename with no dot, or a dotfile whose only dot is at index 0 (`".bashrc"`),
  has no extension — return an empty slice.
- `has_ext` — whether `ext(path)` equals the `ext` argument. Decide and comment one thing: whether
  the argument is expected with or without the leading dot. Accepting both is fine as long as the
  body says so and a test pins it.
- `join` — allocate and concatenate `parts` with exactly one `/` between neighbours: no doubled
  separator when a part already ends with one, no separator before an empty part, and no trailing
  separator on the result. Returns `![]u8`, so allocation failure propagates.
- `normalize` — allocate and return the path with `.` segments dropped, `..` segments resolved
  against the preceding segment, and runs of `//` collapsed. An absolute path keeps its leading
  `/`; a `..` that would escape an absolute root is dropped, not kept. The result must satisfy the
  header invariant: no trailing separator unless the result *is* `/`.

`basename` and `dirname` are the pair that must agree: for any path with a separator,
`join(dirname(p), basename(p))` reconstructs `p`. Say so in a test.

The existing test blocks pin the types and must not be edited. The one that constrains you most,
verbatim:

```
    test path_error_lists_the_failures_a_path_operation_can_have
        // Recovered from trinity-fpga specs/tri/tri_filesystem.tri:7. The
        // not_a_directory / not_a_file pair is what makes is_dir and is_file on
        // FileInfo checkable rather than merely declared.
        then @typeInfo(PathError).@"enum".fields.len == 5
        and std.mem.eql(u8, @typeInfo(PathError).@"enum".fields[0].name, "invalid_path")
        and std.mem.eql(u8, @typeInfo(PathError).@"enum".fields[4].name, "permission_denied")
```

`PathError` has five variants and they stay five. If a path function reports a failure, it reports
one of those.

Add one `test` block per function you implement, appended to the existing test section. Match the
house style already in this file and in `specs/fpga/uart.t27` — bare name, then `given` / `and` /
`when` / `then` lines, no braces:

```
    test dirname_drops_the_trailing_separator
        given path = "/usr/local/bin/t27c"
        when parent = dirname(path)
        then std.mem.eql(u8, parent, "/usr/local/bin")
```

Body style: `specs/fpga/uart.t27` lines 60-109 is the house form of a filled `-> type { }` body.
Constructs measured to parse clean here: `var`/`const` locals, `while (cond) { }`, `if (cond) { }`,
`return expr;`, indexing, field access, string literals, `&x`.

**Do not** rewrite `PathError`, `FileInfo`, the header comment, or the six existing test blocks.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 7 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/io/filesystem.t27` prints 0
- 3. the real-code node count is at least 14
- 4. `t27c parse specs/tri/io/filesystem.t27 | grep -c 'kind: TestBlock'` prints at least 13
- 5. `t27c count specs/tri/io/filesystem.t27 | head -1` reports at least 13 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `7 names, fns=7 nonempty=0, clean` | `7 names, fns=n nonempty=n (n>=7), clean` |
| 2 | stub marker | `7` | `0` |
| 3 | real-code nodes | `0` | `>= 14` |
| 4 | test blocks | `6` | `>= 13` |
| 5 | node types | `9` | `>= 13` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 7 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (join|basename|dirname|ext|has_ext|is_absolute|normalize)\(' specs/tri/io/filesystem.t27
```

Must print `7`. It prints `7` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/io/filesystem.t27 <<'EOF'
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

Today it prints `fns=7 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 7**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 7 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/io/filesystem.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/io/filesystem.t27
```

Today it prints `7`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/io/filesystem.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `14`.

The bar is two nodes per stubbed function (7 x 2 = 14 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/io/filesystem.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `6`. It must print at least `13` — the 6 blocks already in the file
plus one new block for each of the 7 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/io/filesystem.t27 | head -1
```

Today it prints `filesystem.t27: 9 node types`. It must report at least `13`. For scale, 70.2% of
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
- 1. all 7 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/io/filesystem.t27 prints 0: met
- 3. the real-code node count is at least 14: met
- 4. t27c parse specs/tri/io/filesystem.t27 | grep -c 'kind: TestBlock' prints at least 13: met
- 5. t27c count specs/tri/io/filesystem.t27 | head -1 reports at least 13 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/io/filesystem.t27
