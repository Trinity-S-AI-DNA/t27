# Implement the five stubbed functions in TriLogging (specs/tri/utils/logging.t27)

## Context

`specs/tri/utils/logging.t27` is the log-record module of the `tri` standard library. It declares
what a log line *is* — `LogEntry { level, message, timestamp, tag }` — and a four-value severity
enum `LogLevel { debug, info, warn, error }`, then five functions that name a level, parse a
level back from text, colour it, render an entry to a string, and decide whether a message at one
level clears a configured threshold. Its header states the one external dependency:
`// Uses ANSI colors for terminal output`, so `level_color` returns escape sequences and the raw
escape alphabet lives next door in `specs/tri/utils/terminal.t27` and `specs/tri/utils/color.t27`.
Note what `LogEntry` deliberately does *not* carry: no allocator and no formatted buffer — a
record is data, and `format_entry` is the only thing that allocates. All five functions are
empty stubs today.

**.t27 is hand-authored source.** It is compiled out to C/Rust/Verilog/Zig by
`/Users/playom/t27/target/release/t27c`. You are writing source, not compiler output. The
dialect is module-code: `module X;`, `use a::b;`, `fn name(arg : type) -> ret { ... }`, and
`test` blocks. Match the style already in the file.

## Exact current state (measured 2026-09-09, re-run these)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/utils/logging.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/utils/logging.t27 | head -1
logging.t27: 9 node types

$ grep -c '// TODO: Implement from .tri spec' specs/tri/utils/logging.t27
5
```

The two types, quoted verbatim from the file:

```t27
    pub const LogLevel = struct {
        enum : [debug, info, warn, error],
    };

    pub const LogEntry = struct {
        level : LogLevel,
        message : []const u8,
        timestamp : u64,
        tag : ?[]const u8,
    };
```

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. **Keep every signature
exactly as it stands**, and **do not touch the two type declarations** — the existing tests
assert their exact shape and they are correct. These are the five, quoted verbatim:

```t27
    fn level_to_string(level: LogLevel) -> []const u8 {
    fn level_from_string(s: []const u8) -> ?LogLevel {
    fn level_color(level: LogLevel) -> []const u8 {
    fn format_entry(allocator: std.mem.Allocator, entry: LogEntry) -> ![]u8 {
    fn should_log(msg_level: LogLevel, min_level: LogLevel) -> bool {
```

What each must do:

- **`level_to_string`** — one uppercase name per variant: `"DEBUG"`, `"INFO"`, `"WARN"`,
  `"ERROR"`. Total over all four; no default case that hides a missing variant.
- **`level_from_string`** — the inverse. Return the matching level, or `null` when the text names
  no level — that is what the `?LogLevel` return is for. State in a comment whether the match is
  case-sensitive; accepting both `"warn"` and `"WARN"` is the friendlier choice for config files
  and is worth writing down.
- **`level_color`** — the ANSI SGR sequence for each severity, and it must be the *set* sequence
  only, never a reset: the caller composes. Use ordinary escape-containing string literals, one
  per variant, and say in a comment which colour each is (the conventional set is grey/cyan for
  debug, default or green for info, yellow for warn, red for error).
- **`format_entry`** — render `entry` into a freshly allocated `[]u8` and return it; `![]u8`
  because allocation can fail. Fix the layout in a comment and follow it, e.g.
  `<timestamp> [<LEVEL>] <tag>: <message>`. `tag` is optional, so decide what an absent tag
  prints (omit the `tag: ` segment entirely, or substitute `"-"`) and hold to it. The caller
  frees the result.
- **`should_log`** — true when `msg_level` is at least as severe as `min_level`. This is an
  ordering question: `debug < info < warn < error`, so compare the variants' ordinals. A message
  at exactly the threshold *is* logged.

The four existing test blocks are real assertions about the declared types (field count, the
`u64` timestamp width, a constructible untagged entry, the four enum variants) and they are
true — **do not delete or rewrite them**. Add a `test` block per implemented function underneath,
in the file's own `test name / given / when / then / and` style (no braces, no quoted name).
Worth pinning: `level_to_string(LogLevel.error)` is `"ERROR"`, `level_from_string("nope")` is
`null`, round-tripping every level through both functions returns the same level,
`should_log(LogLevel.warn, LogLevel.warn)` is true and `should_log(LogLevel.debug,
LogLevel.error)` is false. There is **no t27 interpreter** — these blocks are parsed and counted,
never executed. Write them as the specification of the behaviour, and do not claim you ran them.

### Parser hazards — this file is the one that triggered them

Measured on this exact file: writing `level_to_string` as a `switch` with qualified arms

```t27
        return switch (level) {
            LogLevel.debug => "DEBUG",
            ...
        };
```

left all three parse metrics at 0 **and dropped `FnDecl` from 5 to 1** — four functions and every
test block vanished from the AST with no diagnostic at all. The bootstrap parser swallows
declarations on two constructs without raising `recovery-events`:

- `switch` arms with a **qualified** pattern (`LogLevel.debug => ...`). Use `if` / `else if`
  chains on `level == LogLevel.debug` instead. Literal arms (`0 => ...`) are safe.
- The **anonymous struct literal** `.{ ... }` — measured: `recovery-events: 1` plus swallowed
  declarations. This one matters here because the obvious `format_entry` body reaches for
  `std.fmt.allocPrint(allocator, "...", .{ ... })`. Write the formatting by hand into an
  allocated buffer, or call a formatting helper without the `.{ }` argument tuple.

Safe and measured-clean in bodies: `if`/`else if`/`else`, `while`, `for (xs) |x|`, `return`,
`const`/`var` locals, `try`, `catch`, `orelse`, `defer`, builtins like `@as` / `@intCast`,
slicing `xs[0..n]`, struct literals `LogEntry{ .level = l, ... }`, and `and`/`or`. Run the real-code
node command after every edit, not just `t27c parse` — that is why criterion 3 exists.

Do **not** gate yourself on `t27c typecheck` — it returns `{"ok":true,"errors":0}` on English
prose and on Python files. It proves nothing. Ignore `t27c lint`'s `"warnings"` array too — it
reports "fn X has no test or invariant" for every function in the gold corpus. Its `"functions"`
and `"tests"` numbers are the useful part.

### House law

Repo law L3 (PURITY) requires source to be ASCII-only with English identifiers. The file's
existing banner comments already contain non-ASCII decoration - leave those lines exactly as they
are, and keep every line **you** add pure ASCII. Law L4 (TESTABILITY) is why the added `test`
blocks are not optional.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 5 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/utils/logging.t27` prints 0
- 3. the real-code node count is at least 10
- 4. `t27c parse specs/tri/utils/logging.t27 | grep -c 'kind: TestBlock'` prints at least 9
- 5. `t27c count specs/tri/utils/logging.t27 | head -1` reports at least 13 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `5 names, fns=5 nonempty=0, clean` | `5 names, fns=n nonempty=n (n>=5), clean` |
| 2 | stub marker | `5` | `0` |
| 3 | real-code nodes | `0` | `>= 10` |
| 4 | test blocks | `4` | `>= 9` |
| 5 | node types | `9` | `>= 13` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 5 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (level_to_string|level_from_string|level_color|format_entry|should_log)\(' specs/tri/utils/logging.t27
```

Must print `5`. It prints `5` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/utils/logging.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/utils/logging.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/utils/logging.t27
```

Today it prints `5`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/utils/logging.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `10`.

The bar is two nodes per stubbed function (5 x 2 = 10 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/utils/logging.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `9` — the 4 blocks already in the file
plus one new block for each of the 5 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/utils/logging.t27 | head -1
```

Today it prints `logging.t27: 9 node types`. It must report at least `13`. For scale, 70.2% of
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
- 2. grep -c 'TODO: Implement' specs/tri/utils/logging.t27 prints 0: met
- 3. the real-code node count is at least 10: met
- 4. t27c parse specs/tri/utils/logging.t27 | grep -c 'kind: TestBlock' prints at least 9: met
- 5. t27c count specs/tri/utils/logging.t27 | head -1 reports at least 13 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

Named above only as a neighbour you must not drift into. No issue owns it
and nobody is working on it: `specs/tri/utils/logger.t27`.

## Boundary

specs/tri/utils/logging.t27
