# Implement the eight stubbed functions in TriBytes (specs/tri/utils/bytes.t27)

## Context

`specs/tri/utils/bytes.t27` is the byte-buffer type of the `tri` standard library: a `Bytes`
value is a `[]u8` plus an `owned : bool` flag saying whether this value is responsible for
freeing that memory. The file header states the design rule the whole module turns on —
`// Views share underlying data` — so `from_slice` and `slice` produce borrowed views
(`owned = false`) over memory somebody else owns, while `clone`, `concat` and `split` allocate
and hand back owned buffers (`owned = true`). It lives beside `specs/tri/utils/text.t27` and
`specs/tri/utils/utf8.t27`; those handle characters and encodings, `bytes.t27` handles raw
octets and says nothing about encoding. All eight of its functions are empty stubs today.

**.t27 is hand-authored source.** It is compiled out to C/Rust/Verilog/Zig by
`/Users/playom/t27/target/release/t27c`. You are writing source, not compiler output. The
dialect is module-code: `module X;`, `use a::b;`, `fn name(arg : type) -> ret { ... }`, and
`test` blocks. Match the style already in the file.

## Exact current state (measured 2026-09-09, re-run these)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/utils/bytes.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/utils/bytes.t27 | head -1
bytes.t27: 7 node types

$ grep -c '// TODO: Implement from .tri spec' specs/tri/utils/bytes.t27
8
```

The type it is all about, quoted verbatim from the file:

```t27
    pub const Bytes = struct {
        data : []u8,
        owned : bool,
    };
```

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. **Keep every signature
exactly as it stands** — same name, same parameter names, same types, same return type. These
are the eight, quoted verbatim:

```t27
    fn empty() -> Bytes {
    fn from_slice(slice: []const u8) -> Bytes {
    fn clone(bytes: Bytes, allocator: std.mem.Allocator) -> !Bytes {
    fn equals(a: Bytes, b: Bytes) -> bool {
    fn slice(bytes: Bytes, start: usize, end: usize) -> Bytes {
    fn concat(a: Bytes, b: Bytes, allocator: std.mem.Allocator) -> !Bytes {
    fn index_of(bytes: Bytes, pattern: []const u8) -> ?usize {
    fn split(bytes: Bytes, delimiter: u8, allocator: std.mem.Allocator) -> ![]Bytes {
```

The `owned` flag is the specification. Get it right in every one:

- **`empty`** — a zero-length buffer, `owned = false`. Nothing was allocated, so nothing is owned.
- **`from_slice`** — wrap the caller's slice as a **view**: same backing memory, `owned = false`.
  Do not copy. This is the rule the header states.
- **`clone`** — allocate `bytes.data.len` bytes, copy, return `owned = true`. Returns `!Bytes`
  because allocation can fail.
- **`equals`** — true when the two lengths match and every byte matches. `owned` takes no part in
  equality: two buffers with the same content are equal whoever owns them. Say so in a comment.
- **`slice`** — a sub-view `bytes.data[start..end]`, `owned = false` no matter what `bytes.owned`
  was: a view of an owned buffer is still only a view, and must never free. Clamp `start` and
  `end` into range and return the empty view when `start >= end`.
- **`concat`** — allocate `a.data.len + b.data.len`, copy both in order, return `owned = true`.
- **`index_of`** — the index of the first occurrence of `pattern` in `bytes.data`, or `null` when
  absent. An empty pattern matches at `0`. The `?usize` return is what carries "not found" —
  never return a sentinel number.
- **`split`** — split on `delimiter` and return the pieces. Say in a comment whether the pieces
  are views into the original (cheap, `owned = false`) or copies, and be consistent: the returned
  slice-of-`Bytes` is itself allocated, so it must be freed by the caller either way.

The eight existing test blocks are placeholders of the shape

```t27
    test empty_basic_case
        given input = default_input()
        when result = empty(input)
        then result != undefined
```

`default_input()` is defined nowhere in the module and `empty` takes no arguments, so these
assert nothing. Leave them and **add** a real `test` block per function underneath, in the file's
own `test name / given / when / then / and` style (no braces, no quoted name — copy the layout
above). Pin the facts that matter: `empty().owned == false`, `from_slice(s).owned == false`,
`clone(b, a).owned == true`, `equals` on two equal-content buffers with different `owned` flags
is true, a slice of an owned buffer is not owned, `index_of` on an absent pattern is `null`.
There is **no t27 interpreter** — these blocks are parsed and counted, never executed. Write
them as the specification of the behaviour, and do not claim you ran them.

### Parser hazards (measured — these silently destroy the file)

The bootstrap parser swallows declarations on two constructs *without* raising
`recovery-events`, so the file can look clean and still have lost half its functions:

- `switch` arms with a **qualified** pattern (`Foo.bar => ...`) — measured: `FnDecl` fell from 4
  to 2 with all three parse metrics still reporting 0. Use `if` / `else if` chains instead.
  Literal arms (`0 => ...`) are safe.
- The **anonymous struct literal** `.{ ... }` — measured: `recovery-events: 1` and `FnDecl` fell
  from 4 to 2.

Safe and measured-clean in bodies: `if`/`else if`/`else`, `while`, `for (xs) |x|`, `return`,
`const`/`var` locals, `try`, `catch`, `orelse`, `defer`, builtins like `@as` / `@intCast`,
slicing `xs[0..n]`, struct literals `Bytes{ .data = d, .owned = false }`, and `and`/`or`. Run the real-code
node command after every edit, not just `t27c parse` — that is why criterion 3 exists.

Do **not** gate yourself on `t27c typecheck` — it returns `{"ok":true,"errors":0}` on English
prose and on Python files. It proves nothing.

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

- 1. all 8 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/utils/bytes.t27` prints 0
- 3. the real-code node count is at least 16
- 4. `t27c parse specs/tri/utils/bytes.t27 | grep -c 'kind: TestBlock'` prints at least 16
- 5. `t27c count specs/tri/utils/bytes.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `8 names, fns=8 nonempty=0, clean` | `8 names, fns=n nonempty=n (n>=8), clean` |
| 2 | stub marker | `8` | `0` |
| 3 | real-code nodes | `0` | `>= 16` |
| 4 | test blocks | `8` | `>= 16` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 8 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (empty|from_slice|clone|equals|slice|concat|index_of|split)\(' specs/tri/utils/bytes.t27
```

Must print `8`. It prints `8` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/utils/bytes.t27 <<'EOF'
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

Today it prints `fns=8 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 8**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 8 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/utils/bytes.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/utils/bytes.t27
```

Today it prints `8`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/utils/bytes.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `16`.

The bar is two nodes per stubbed function (8 x 2 = 16 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/utils/bytes.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `8`. It must print at least `16` — the 8 blocks already in the file
plus one new block for each of the 8 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/utils/bytes.t27 | head -1
```

Today it prints `bytes.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 8 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/utils/bytes.t27 prints 0: met
- 3. the real-code node count is at least 16: met
- 4. t27c parse specs/tri/utils/bytes.t27 | grep -c 'kind: TestBlock' prints at least 16: met
- 5. t27c count specs/tri/utils/bytes.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/utils/bytes.t27
