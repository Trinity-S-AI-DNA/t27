# Implement parse, compare and next in specs/tri/utils/version.t27

## Context

`specs/tri/utils/version.t27` is the `TriVersion` module — SemVer 2.0.0 handling for
the `specs/tri/utils/` toolbox (neighbours: `string.t27`, `config.t27`, `args.t27`).
Its header says "Semantic versioning", and the struct is already decided, with the
three numeric fields and the two textual ones SemVer keeps separate:

```
    pub const Version = struct {
        major : usize,
        minor : usize,
        patch : usize,
        prerelease : []const u8,
        build : []const u8,
    };
```

The four existing test blocks pin that shape: all five fields readable, absence
represented by an *empty string* rather than a separate case, `0.0.0` representable,
and `prerelease` / `build` independent of each other. Three functions turn that shape
into a working version type.

**.t27 is a hand-authored source language.** You are writing source that `t27c`
compiles out to C/Rust/Verilog/Zig. Write code, not prose about code.

## Current state (measured — re-run these yourself)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/utils/version.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/utils/version.t27 | head -1
version.t27: 7 node types

$ ./target/release/t27c parse specs/tri/utils/version.t27 2>&1 | grep -c 'kind: FnDecl'
3

$ ./target/release/t27c parse specs/tri/utils/version.t27 2>&1 | grep -c 'kind: TestBlock'
4

$ grep -c 'TODO: Implement' specs/tri/utils/version.t27
4
```

Note the asymmetry: **3 functions but 4 `TODO: Implement` lines.** Three are the stub
bodies; the fourth is inside the explanatory comment at the top of the test section,
which reads "parse, compare and next are still `TODO: Implement from .tri spec` and
compile to `@panic`, so nothing below calls them." That sentence becomes false the
moment you implement them, so rewriting it is part of this task — and it is what
makes criterion 2 reachable.

## What to write

Fill in these three functions. **Keep each signature exactly as it is** — criterion 1
greps for all three names by hand. Quoted verbatim from the file:

1. `fn parse(version_str: []const u8, allocator: std.mem.Allocator) -> !Version` —
   parse `MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]`. Split on the first `+` to take
   `build`, then on the first `-` after the numeric core to take `prerelease`, then
   split the core on `.` into three unsigned integers. Missing prerelease and missing
   build are the empty string, per `release_version_has_empty_text_fields`. A string
   that is not three dot-separated numbers is an error, not a zero version.
2. `fn compare(a: Version, b: Version) -> i8` — returns -1, 0 or 1. Compare `major`,
   then `minor`, then `patch`. If those are equal, a version *with* a prerelease is
   lower than the same version without one (1.0.0-rc.1 < 1.0.0); if both have one,
   compare the prerelease strings. `build` is never compared — SemVer ignores it,
   which is exactly what the `prerelease_and_build_are_independent` test is about.
3. `fn next(version: Version, part: []const u8, allocator: std.mem.Allocator) -> !Version` —
   bump one component. `part == "major"` → major+1, minor 0, patch 0;
   `"minor"` → minor+1, patch 0; `"patch"` → patch+1. Any bump clears `prerelease` and
   `build` to the empty string. An unrecognised `part` is an error.

### The tests already in the file are the specification

Do not edit or delete the four existing test blocks. These two fix the representation
your parser must produce:

```
    // A release version is the same shape with both textual fields empty --
    // absence is an empty string, not a separate case.
    test release_version_has_empty_text_fields
        given v = Version{ .major = 2, .minor = 0, .patch = 0, .prerelease = "", .build = "" }
        then v.prerelease.len == 0
        and v.build.len == 0
        and v.major == 2

    // prerelease and build are two independent fields, not one packed string:
    // SemVer orders by the first and ignores the second, so a version may
    // carry either without the other.
    test prerelease_and_build_are_independent
        given pre_only = Version{ .major = 1, .minor = 0, .patch = 0, .prerelease = "rc.1", .build = "" }
        and build_only = Version{ .major = 1, .minor = 0, .patch = 0, .prerelease = "", .build = "sha.abc" }
        then pre_only.prerelease.len > 0
        and pre_only.build.len == 0
        and build_only.prerelease.len == 0
        and build_only.build.len > 0
```

### Add test blocks for what you implement

House style: the proof of a function is a `test` block beside it. This file uses the
`test <name> / given / and / when / then` form (no quotes, no braces) — match it, and
add at least two new blocks per implemented function (one ordinary case, one boundary:
a version with no prerelease, a major bump, an equal comparison). Shape that parses
clean here:

```
    test compare_orders_by_major_first
        given older = Version{ .major = 1, .minor = 9, .patch = 9, .prerelease = "", .build = "" }
        and newer = Version{ .major = 2, .minor = 0, .patch = 0, .prerelease = "", .build = "" }
        when ordering = compare(older, newer)
        then ordering == -1
```

### Style reference

`specs/fpga/mac.t27` lines 89–111 is a parse-clean file in the same `module Name;`
dialect with real function bodies (`const x = ...;`, `if (...) { ... } else { ... }`,
`return x;`); `specs/vsa/vsa_core.t27` lines 65–79 shows `var` and `while (...) { }`
parsing clean in the same dialect. There is **no t27 interpreter**: nothing here
executes. Do not claim you ran a test.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 3 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/utils/version.t27` prints 0
- 3. the real-code node count is at least 6
- 4. `t27c parse specs/tri/utils/version.t27 | grep -c 'kind: TestBlock'` prints at least 7
- 5. `t27c count specs/tri/utils/version.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `3 names, fns=3 nonempty=0, clean` | `3 names, fns=n nonempty=n (n>=3), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 6` |
| 4 | test blocks | `4` | `>= 7` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 3 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (parse|compare|next)\(' specs/tri/utils/version.t27
```

Must print `3`. It prints `3` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/utils/version.t27 <<'EOF'
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

Today it prints `fns=3 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 3**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 3 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/utils/version.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/utils/version.t27
```

Today it prints `4`. It must print `0`.

That is `4`, not `3`. 3 of them are the stub bodies; the other 1 sits in a prose
comment further down the file which states that these functions are still
unimplemented. That sentence becomes false when you finish, so rewrite or delete
it as part of the task.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/utils/version.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `6`.

The bar is two nodes per stubbed function (3 x 2 = 6 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/utils/version.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `7` — the 4 blocks already in the file
plus one new block for each of the 3 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/utils/version.t27 | head -1
```

Today it prints `version.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 3 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/utils/version.t27 prints 0: met
- 3. the real-code node count is at least 6: met
- 4. t27c parse specs/tri/utils/version.t27 | grep -c 'kind: TestBlock' prints at least 7: met
- 5. t27c count specs/tri/utils/version.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/utils/version.t27
