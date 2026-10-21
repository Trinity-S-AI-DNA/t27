# Implement the four stubbed functions in `TriSha256` (specs/tri/crypto/sha256.t27)

## Context

`specs/tri/crypto/sha256.t27` is the SHA-256 module of the `tri` crypto family (beside
`crypto.t27`). Its header comment names the construction it implements: *"Merkle-Damgard
construction with 64-byte blocks"*. It declares one type,
`SHA256 { state : [8]u32, buffer : [64]u8, count : u64 }`, and the three existing tests exist
precisely to pin those widths — 256 bits of chaining value, one 512-bit block of buffer, and a
64-bit length counter because the final padded block encodes the message length in 64 bits. Four
functions are declared — `init`, `update`, `final`, `hash` — and all four are stubs, which is why
the test-section comment says *"no digest can be computed here yet -- not even the empty-string
vector."* This module is the most tightly specified task in the batch: SHA-256 is defined by
FIPS 180-4 and there is exactly one correct answer for every byte.

`.t27` is a hand-authored source language, not compiler output. You are writing source.

## Exact current state

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/crypto/sha256.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

The file parses clean today. **Do not regress that.**

```
$ ./target/release/t27c count specs/tri/crypto/sha256.t27 | head -1
sha256.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/crypto/sha256.t27
4
```

```
$ grep -cE '^    fn ' specs/tri/crypto/sha256.t27
4
```

Four declared functions, four stubs: every function in this module is a signature with an empty
body.

## What to write

Fill each body. **Keep every signature byte-for-byte as it is** — same name, same parameter names,
same arrow-return type. Only the body changes. The signatures, verbatim from the file:

```
    fn init() -> SHA256 {
    fn update(sha: *SHA256, data: []const u8) -> void {
    fn final(sha: *SHA256) -> [32]u8 {
    fn hash(data: []const u8) -> [32]u8 {
```

You will need two tables that the module does not yet declare. Add them as module-level `pub const`
declarations above the functions, in the same style as the existing `pub const SHA256 = struct`:

- `H0 : [8]u32` — the FIPS 180-4 initial hash value: `0x6a09e667, 0xbb67ae85, 0x3c6ef372,
  0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19`.
- `K : [64]u32` — the 64 round constants (first 32 bits of the fractional parts of the cube roots
  of the first 64 primes), beginning `0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5` and ending
  `0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2`. Write all 64 out; do not abbreviate with a
  comment.

`specs/numeric/gf16.t27` shows the house form for a module-level table (`pub const pow2_table :
[32]u16 = [32]u16{ ... };`) — follow it.

What each function must do:

- `init` — return `SHA256{ .state = H0, .buffer = all zero, .count = 0 }`.
- `update` — absorb `data` into the buffer, running the compression function on every complete
  64-byte block and keeping the remainder in `buffer`; `count` is the **total message length in
  bytes** seen so far, and it must be incremented by `data.len` on every call. Calling `update`
  twice with `"ab"` then `"c"` must leave the same state as one call with `"abc"` — say so in a
  test.
- `final` — pad per Merkle-Damgard: append the byte `0x80`, then zero bytes until 56 bytes mod 64,
  then the message length **in bits** (`count * 8`) as a big-endian `u64`; compress the last
  block(s); serialise `state[0..8]` big-endian into the returned `[32]u8`.
- `hash` — the one-shot convenience: `init`, `update(data)`, `final`. It must be a composition of
  the other three, not a second copy of the algorithm.

The compression function itself (message schedule `W[0..64]`, the `Ch`/`Maj`/`Sigma0`/`Sigma1`/
`sigma0`/`sigma1` mixers, 64 rounds, `a..h` fed back into `state`) goes inside `update`, or in one
extra private helper if that reads better. If you add a helper, it gets a `test` block too.

The three existing tests fix the state widths and must not be edited. Verbatim:

```
    test state_is_the_256_bit_digest
        // eight 32-bit working variables a..h = 256 bits of chaining value
        given state_bits = @bitSizeOf(@FieldType(SHA256, "state"))
        then state_bits == 256

    test buffer_is_exactly_one_512_bit_block
        given buffer_bytes = @sizeOf(@FieldType(SHA256, "buffer"))
        then buffer_bytes == 64 and buffer_bytes * 8 == 512
```

Add one `test` block per function you implement, appended to the existing test section, and pin at
least one **published FIPS vector** as a literal so the file states the right answer even though
nothing can execute it: the digest of the empty string is
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, and the digest of `"abc"` is
`ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad`. Match the house style in this
file and in `specs/fpga/uart.t27` — bare name, then `given` / `and` / `when` / `then` lines, no
braces:

```
    test update_advances_the_length_counter_by_the_byte_count
        given sha = init()
        and   update(&sha, "abc")
        then sha.count == 3
```

Body style: `specs/fpga/uart.t27` lines 60-109 is the house form of a filled `-> type { }` body.
The following were measured to parse clean in this exact file (a probe copy of `sha256.t27` with two
bodies filled and four tests appended reported `recovery-events: 0`, `lexer-discarded-chars: 0`):
`var`/`const` locals, `while (cond) { }`, `if (cond) { }`, `return expr;`, `%`, indexing, field
access, struct literals, `[_]u8{0} ** 64`, `&x`, string literals.

**Do not** rewrite the `SHA256` declaration, the header comment, or the three existing tests. You may
update the test-section commentary at lines 47-50, which describes the stubs you are removing.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 4 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/crypto/sha256.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/crypto/sha256.t27 | grep -c 'kind: TestBlock'` prints at least 7
- 5. `t27c count specs/tri/crypto/sha256.t27 | head -1` reports at least 11 node types
- 6. `grep -c '0x428a2f98' specs/tri/crypto/sha256.t27` prints 1

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `3` | `>= 7` |
| 5 | node types | `7` | `>= 11` |
| 6 | SHA-256 round constant | `0` | `1` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|update|final|hash)\(' specs/tri/crypto/sha256.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/crypto/sha256.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/crypto/sha256.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/crypto/sha256.t27
```

Today it prints `4`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/crypto/sha256.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/crypto/sha256.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `3`. It must print at least `7` — the 3 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/crypto/sha256.t27 | head -1
```

Today it prints `sha256.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
the stub-free, parse-clean files in `specs/` are at or above that number, and the
median across them is 17.5.

`| head -1` is not optional. The rest of the `t27c count` table is **not reproducible**:
rows with equal counts print in a different order on every run (measured: 3 distinct
orderings in 8 consecutive runs over the same unchanged file), and the table stops
after 15 rows with `... and N more`. 14 files in `specs/` contain test blocks that
`t27c count | grep TestBlock` cannot see for exactly that reason — and they are the
richly-implemented ones, which is the state this task is trying to reach. Never read
a number off that table; the first line is the only stable part of the output.

**Criterion 6 — the real round constants are present.**

```
$ grep -c '0x428a2f98' specs/tri/crypto/sha256.t27
```

Today it prints `0`. `0x428a2f98` is the first of the 64 SHA-256 round constants (the
fractional part of the cube root of 2). It must print `1`. Nothing else in this issue can
tell a real SHA-256 compression function from a plausible-looking one, and no test here
can be executed to catch the difference — see the note on the missing interpreter above.

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
- 2. grep -c 'TODO: Implement' specs/tri/crypto/sha256.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/crypto/sha256.t27 | grep -c 'kind: TestBlock' prints at least 7: met
- 5. t27c count specs/tri/crypto/sha256.t27 | head -1 reports at least 11 node types: met
- 6. grep -c '0x428a2f98' specs/tri/crypto/sha256.t27 prints 1: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/crypto/sha256.t27
