# Implement the four stubbed functions in specs/tri/crypto/hex.t27

## Context

`specs/tri/crypto/hex.t27` is the hex (base-16) codec of the Trinity standard
library, sitting beside `base32.t27`, `base64.t27` and `sha256.t27` in
`specs/tri/crypto/`. Its job is the one stated in its own header comment:
"Every byte becomes 2 chars". The module declares a codec value
`Hex { uppercase : bool }` — a single flag choosing between the `a-f` and `A-F`
alphabets — and four functions: two constructors for that codec and the
encode/decode pair. Every one of the four is a signature with an empty body
today, which is why the file's own test-section comment says the round trip
"cannot be stated yet". Your job is to write those four bodies in `.t27`, in the
module-code dialect the file already uses, and to add the test blocks the
comment says belong there "the moment the bodies exist".

`.t27` is hand-authored source. It is compiled out to C/Rust/Verilog/Zig by
`t27c`; you are writing the input, not compiler output. Do not write Zig, C or
Rust files.

## Exact current state

All commands run from `/Users/playom/t27`.

```
$ ./target/release/t27c parse specs/tri/crypto/hex.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/crypto/hex.t27 | head -1
hex.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/crypto/hex.t27
4
```

```
$ grep -cE '^[[:space:]]*test ' specs/tri/crypto/hex.t27
2
```

The file parses clean today. **Keep it clean** — a change that raises
`recovery-events` above 0 is a regression, and the two existing test blocks must
survive untouched.

## What to write

Four functions, signatures quoted verbatim from the file. Keep each signature
exactly as it is — only the body changes.

1. `fn lower_case() -> Hex`
   Return the lowercase codec: `Hex{ .uppercase = false }`.

2. `fn upper_case() -> Hex`
   Return the uppercase codec: `Hex{ .uppercase = true }`.

3. `fn encode(codec: Hex, input: []const u8, allocator: std.mem.Allocator) -> ![]const u8`
   Allocate `input.len * 2` bytes and write two characters per input byte, high
   nibble first. Digits 0-9 map to `'0'..'9'` (48..57). Digits 10-15 map to
   `'a'..'f'` (97..102) when `codec.uppercase` is false and to `'A'..'F'`
   (65..70) when it is true. This is the "every byte becomes 2 chars" claim in
   the header made real.

4. `fn decode(input: []const u8, allocator: std.mem.Allocator) -> ![]const u8`
   The inverse. Allocate `input.len / 2` bytes; each output byte is
   `(hi << 4) | lo` from the two characters. Accept both letter cases on input —
   `decode` takes no codec argument, so it cannot be told which case to expect.

You may add small private helpers (e.g. `fn nibble_to_char(codec: Hex, nibble: u8) -> u8`
and `fn char_to_nibble(c: u8) -> u8`) — extra functions are fine, they only add
to the declaration count.

### The tests are the specification

These two test blocks already exist and pin the codec value. Do not modify or
delete them:

```
    test hex_codec_records_its_letter_case
        given lower = Hex{.uppercase=false}
        and upper = Hex{.uppercase=true}
        then !lower.uppercase
        and upper.uppercase

    test hex_the_two_codecs_are_distinguishable
        given lower = Hex{.uppercase=false}
        and upper = Hex{.uppercase=true}
        then lower.uppercase != upper.uppercase
```

Add at least one new `test` block per function you implement, and replace the
stale prose comment above the test section (the one beginning "lower_case(),
upper_case(), encode() and decode() have no body yet") with a comment describing
what is now stated. The round trip `decode(encode(codec, x)) == x` and the
length identity `encode(codec, x).len == x.len * 2` are the two properties worth
writing down.

Both test forms in this corpus parse clean — use either:

```
    test "encode_doubles_the_length" {
        expect(encode_len(3) == 6)
    }

    test encode_doubles_the_length
        given n = 3
        then n * 2 == 6
```

**There is no t27 interpreter.** A test block can be parsed and counted but not
executed. Never claim a test passed; the criteria below are all static.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 4 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/crypto/hex.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/crypto/hex.t27 | grep -c 'kind: TestBlock'` prints at least 6
- 5. `t27c count specs/tri/crypto/hex.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `2` | `>= 6` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (lower_case|upper_case|encode|decode)\(' specs/tri/crypto/hex.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/crypto/hex.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/crypto/hex.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/crypto/hex.t27
```

Today it prints `4`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/crypto/hex.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/crypto/hex.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `2`. It must print at least `6` — the 2 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/crypto/hex.t27 | head -1
```

Today it prints `hex.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/crypto/hex.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/crypto/hex.t27 | grep -c 'kind: TestBlock' prints at least 6: met
- 5. t27c count specs/tri/crypto/hex.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/crypto/hex.t27
