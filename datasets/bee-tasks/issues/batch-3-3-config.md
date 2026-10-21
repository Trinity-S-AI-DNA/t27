# Implement `parse` and the three getters in specs/tri/utils/config.t27

## Context

`specs/tri/utils/config.t27` is the configuration-file reader of the `tri` standard
library (`module TriConfig;`), on the same shelf as `args.t27`, `template.t27`,
`logger.t27` and `error.t27` in `specs/tri/utils/`. Its header states the design rule the
whole module exists to enforce: *"Values are parsed as their natural type"* — a config
value is not a string that callers later coerce, it is a `ConfigValue` with one optional
slot per natural type (`string : ?[]const u8`, `number : ?f64`, `boolean : ?bool`) plus an
`is_null : bool` flag, so "absent" and "explicitly null" stay distinguishable. A `Config`
is a slice of `ConfigEntry { key, value }` plus an in-band `error : ?[]const u8`. The
types are fully declared; all four functions are signature-only stubs. `.t27` is
hand-authored source; `t27c` compiles it out to C/Rust/Verilog/Zig.

## Current state (commands actually run, 2026-09-09)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/utils/config.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/utils/config.t27 | head -1
config.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/utils/config.t27
4
$ grep -cE '^    fn ' specs/tri/utils/config.t27
4
$ grep -cE '^    test ' specs/tri/utils/config.t27
5
$ grep -cE '^ +(return|let|var|if|for|while|match) ' specs/tri/utils/config.t27
0
```

Three structs and four functions declared; zero statements in any body.

## What to write

Keep all four signatures byte-for-byte. Replace only each
`// TODO: Implement from .tri spec` line.

### 1. `fn parse(allocator: std.mem.Allocator, content: []const u8) -> !Config`

```
    // parse(allocator: std.mem.Allocator, content: []const u8) → !Config
    fn parse(allocator: std.mem.Allocator, content: []const u8) -> !Config {
        // TODO: Implement from .tri spec
    }
```

Read `content` line by line and build the `entries` slice. For each non-empty,
non-comment line, split on the first `=`, trim whitespace from both halves, and classify
the right-hand side **into its natural type**, not into a string:

- `true` / `false` → `.boolean` set, the other slots left null
- a numeric literal → `.number` set (parsed as `f64` — the field is `?f64`, double
  precision, and one of the existing tests exists to say so)
- the bare word `null` → `.is_null = true` with all three payload slots left null
- anything else → `.string` set

On a malformed line, do **not** return a Zig error: set `Config.error` to a message and
return the partially built `Config`. The field is `?[]const u8` precisely so a caller gets
both the entries that did parse and the reason the rest did not; the existing test
`parse_failure_is_reported_in_band` states this. On success leave `.error = null`.

### 2. `fn get_string(config: Config, key: []const u8, default: []const u8) -> []const u8`

```
    // get_string(config: Config, key: []const u8, default: []const u8) → []const u8
    fn get_string(config: Config, key: []const u8, default: []const u8) -> []const u8 {
        // TODO: Implement from .tri spec
    }
```

Scan `config.entries` for a matching `key`. Return its `value.string` if that slot is
set; return `default` when the key is absent, when the entry is null, or when the value
landed in a different slot. Never panic, never coerce a number into a string.

### 3. `fn get_number(config: Config, key: []const u8, default: f64) -> f64`

```
    // get_number(config: Config, key: []const u8, default: f64) → f64
    fn get_number(config: Config, key: []const u8, default: f64) -> f64 {
        // TODO: Implement from .tri spec
    }
```

Same lookup, reading `value.number`, returning `default` on absence or wrong slot.

### 4. `fn get_bool(config: Config, key: []const u8, default: bool) -> bool`

```
    // get_bool(config: Config, key: []const u8, default: bool) → bool
    fn get_bool(config: Config, key: []const u8, default: bool) -> bool {
        // TODO: Implement from .tri spec
    }
```

Same lookup, reading `value.boolean`, returning `default` on absence or wrong slot.

All three getters share one lookup shape; a small private helper that finds an entry by
key is welcome, but it counts as a fifth `fn` — see criterion 1 below, which allows the
function count to grow as long as the four named functions survive and every body is
non-empty.

### The existing tests are the specification

Already in the file. Do not delete or weaken them.

```
    test each_natural_type_is_its_own_optional_slot
        then @FieldType(ConfigValue, "string") == ?[]const u8
        and @FieldType(ConfigValue, "number") == ?f64
        and @FieldType(ConfigValue, "boolean") == ?bool

    test null_is_a_flag_not_a_missing_payload
        then @FieldType(ConfigValue, "is_null") == bool

    test parse_failure_is_reported_in_band
        then @FieldType(Config, "error") == ?[]const u8
        and @FieldType(Config, "entries") == []ConfigEntry
```

Add **at least four new `test` blocks**, one per function, in the file's own style
(`    test some_name` on its own line, then indented `given` / `when` / `then` lines).
Good subjects: a `ConfigValue` with `.number` set has `.string == null`; a getter falls
back to its default when the entry is absent; `is_null` is true with all three slots null;
a `Config` carrying an error message still has entries.

### Style notes, measured

Ordinary statement syntax parses clean inside a body — verified on copies of files in this
corpus: `let n = config.entries.len;`, `if (x == 0) { return default; }`,
`for (0..n) |i| { ... }`, `while (i > 0) { ... }`, `return Config{ .entries = e, .@"error" = null };`
all give recovery-events 0. Note that `error` is a keyword: the existing tests reach the
field as `.@"error"`, so use that form when you construct or read it. Four-space indent for
`fn` and `test` lines, eight for statements.

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

- 1. all 4 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/utils/config.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/utils/config.t27 | grep -c 'kind: TestBlock'` prints at least 9
- 5. `t27c count specs/tri/utils/config.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `5` | `>= 9` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (parse|get_string|get_number|get_bool)\(' specs/tri/utils/config.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/utils/config.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/utils/config.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/utils/config.t27
```

Today it prints `4`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/utils/config.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/utils/config.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `5`. It must print at least `9` — the 5 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/utils/config.t27 | head -1
```

Today it prints `config.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/utils/config.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/utils/config.t27 | grep -c 'kind: TestBlock' prints at least 9: met
- 5. t27c count specs/tri/utils/config.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/utils/config.t27
