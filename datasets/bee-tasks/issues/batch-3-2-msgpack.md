# Implement `encode` and `decode` in specs/tri/encoding/msgpack.t27

## Context

`specs/tri/encoding/msgpack.t27` is the MessagePack codec of the `tri` standard library
(`module TriMsgpack;`). It lives beside `json.t27`, `bson.t27`, `csv.t27` and `xml.t27` in
`specs/tri/encoding/` — the same "one module per wire format" shelf — and its header calls
it the *"Compact binary format"*. The module already declares its value model in full: a
nine-variant `MsgPackType` enum (`Nil, Bool, Int, Uint, Float, Str, Bin, Array, Map`) and
a `MsgPackValue` struct holding one payload slot per family, including a recursive
`array_value : []MsgPackValue` and `map_value : std.StringHashMap(MsgPackValue)`. What is
missing is the codec itself: both `encode` and `decode` are signature-only stubs.
`.t27` is hand-authored source; `t27c` compiles it out to C/Rust/Verilog/Zig.

## Current state (commands actually run, 2026-09-09)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/encoding/msgpack.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/encoding/msgpack.t27 | head -1
msgpack.t27: 9 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/encoding/msgpack.t27
2
$ grep -cE '^    fn ' specs/tri/encoding/msgpack.t27
2
$ grep -cE '^    test ' specs/tri/encoding/msgpack.t27
6
$ grep -cE '^ +(return|let|var|if|for|while|match) ' specs/tri/encoding/msgpack.t27
0
```

Nine enum variants and eight payload fields are declared; zero statements exist in either
function.

## What to write

Keep both signatures byte-for-byte. Replace only the `// TODO: Implement from .tri spec`
line in each.

### 1. `fn encode(value: MsgPackValue, allocator: std.mem.Allocator) -> ![]u8`

```
    // encode(value: MsgPackValue, allocator: std.mem.Allocator) → ![]u8
    fn encode(value: MsgPackValue, allocator: std.mem.Allocator) -> ![]u8 {
        // TODO: Implement from .tri spec
    }
```

Serialize one `MsgPackValue` to a freshly allocated byte slice. Switch on `value.type` and
emit the MessagePack tag byte followed by the big-endian payload. The tag ranges the
existing tests already pin down:

- `Nil` → `0xc0`; `Bool` → `0xc2` (false) / `0xc3` (true)
- `Uint` in `0x00..0x7f` → that single byte (positive fixint)
- `Int` in `-32..-1` → a single byte in `0xe0..0xff` (negative fixint)
- wider ints → `0xcc/0xcd/0xce/0xcf` (u8/u16/u32/u64) and `0xd0/0xd1/0xd2/0xd3` (i8/i16/i32/i64)
- `Float` → `0xcb` plus eight big-endian bytes
- `Str` → fixstr `0xa0 | len` for `len < 32`, else `0xd9/0xda/0xdb`
- `Bin` → `0xc4/0xc5/0xc6` plus the length prefix
- `Array` / `Map` → fixarray `0x90 | n` / fixmap `0x80 | n` for `n < 16`, else
  `0xdc/0xdd` and `0xde/0xdf`, then each element (and each key/value pair for maps)
  encoded recursively.

Recurse through `array_value` and `map_value` — that recursion is the whole point of the
declared type, and one of the existing tests exists to say so.

### 2. `fn decode(data: []const u8, allocator: std.mem.Allocator) -> !MsgPackValue`

```
    // decode(data: []const u8, allocator: std.mem.Allocator) → !MsgPackValue
    fn decode(data: []const u8, allocator: std.mem.Allocator) -> !MsgPackValue {
        // TODO: Implement from .tri spec
    }
```

The exact inverse: read the leading tag byte, dispatch to the matching family, fill the
one payload slot for that family, and return the `MsgPackValue`. Decoding must be an
error — not a panic and not a silent default — when the tag is unknown or the buffer ends
early. `str` must land in `str_value` and `bin` in `bin_value`; the two are separate
families in the 2013 spec revision and one of the tests below says so.

### The existing tests are the specification

These are already in the file. Do not delete or weaken them; your implementation has to be
consistent with every one.

```
    test msgpack_type_names_the_nine_families
        given family_count = @typeInfo(MsgPackType).@"enum".fields.len
        and has_nil = @hasField(MsgPackType, "Nil")
        and has_map = @hasField(MsgPackType, "Map")
        then family_count == 9 and has_nil and has_map

    test signed_and_unsigned_integers_are_separate_families
        given signed_type = @FieldType(MsgPackValue, "int_value")
        and unsigned_type = @FieldType(MsgPackValue, "uint_value")
        then signed_type == i64 and unsigned_type == u64

    test positive_fixint_covers_zero_through_127
        given low = 0x00
        and high = 0x7f
        when span = high - low + 1
        then span == 128

    test negative_fixint_covers_minus_32_through_minus_1
        given low = 0xe0
        and high = 0xff
        when span = high - low + 1
        then span == 32
```

Add **at least two new `test` blocks**, one for `encode` and one for `decode`, in the
file's own style (`    test some_name`, then indented `given` / `when` / `then` lines).
Good subjects: nil encodes to the single byte `0xc0`; the fixstr tag for a 5-byte string is
`0xa5`; the fixarray tag for 3 elements is `0x93`; the round trip `decode(encode(v))`
returns the same family.

### Style notes, measured

Ordinary statement syntax parses clean inside a body — verified on copies of files in this
corpus: `let n = data.len;`, `if (tag == 0xc0) { ... }`, `for (0..n) |i| { ... }`,
`while (i > 0) { ... }`, `match x { A => 1, _ => 0, }`, `return Value{ .a = 1 };` all give
recovery-events 0. Four-space indent for `fn` and `test` lines, eight for statements.

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

- 1. all 2 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/encoding/msgpack.t27` prints 0
- 3. the real-code node count is at least 4
- 4. `t27c parse specs/tri/encoding/msgpack.t27 | grep -c 'kind: TestBlock'` prints at least 8
- 5. `t27c count specs/tri/encoding/msgpack.t27 | head -1` reports at least 13 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `2 names, fns=2 nonempty=0, clean` | `2 names, fns=n nonempty=n (n>=2), clean` |
| 2 | stub marker | `2` | `0` |
| 3 | real-code nodes | `0` | `>= 4` |
| 4 | test blocks | `6` | `>= 8` |
| 5 | node types | `9` | `>= 13` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 2 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (encode|decode)\(' specs/tri/encoding/msgpack.t27
```

Must print `2`. It prints `2` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/encoding/msgpack.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/encoding/msgpack.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/encoding/msgpack.t27
```

Today it prints `2`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/encoding/msgpack.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `4`.

The bar is two nodes per stubbed function (2 x 2 = 4 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/encoding/msgpack.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `6`. It must print at least `8` — the 6 blocks already in the file
plus one new block for each of the 2 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/encoding/msgpack.t27 | head -1
```

Today it prints `msgpack.t27: 9 node types`. It must report at least `13`. For scale, 70.2% of
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
- 2. grep -c 'TODO: Implement' specs/tri/encoding/msgpack.t27 prints 0: met
- 3. the real-code node count is at least 4: met
- 4. t27c parse specs/tri/encoding/msgpack.t27 | grep -c 'kind: TestBlock' prints at least 8: met
- 5. t27c count specs/tri/encoding/msgpack.t27 | head -1 reports at least 13 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/encoding/msgpack.t27
