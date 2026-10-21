# Implement the four stubbed functions in TriOption (specs/tri/collections/option.t27)

## Context

`specs/tri/collections/option.t27` is the "maybe a value" type of the `tri` standard library, and
its header states the reason it exists: `// Eliminates null pointer issues`. `Option(T)` is
deliberately *not* a nullable pointer — it is a struct with two independent fields, a `bool`
discriminant and a payload slot of type `T`, so an absent value is a flag set to `false` next to
a perfectly ordinary `T`, never a null address. It sits beside `bitvector.t27` and the other
containers in `specs/tri/collections/`, and it is the module those containers are expected to
return from any lookup that can miss. All four of its functions are empty stubs today, so the
module states the shape of `Option(T)` and nothing about how to build or read one.

**.t27 is hand-authored source.** It is compiled out to C/Rust/Verilog/Zig by
`/Users/playom/t27/target/release/t27c`. You are writing source, not compiler output. The
dialect is module-code: `module X;`, `use a::b;`, `fn name(arg : type) -> ret { ... }`, and
`test` blocks. Match the style already in the file.

## Exact current state (measured 2026-09-09, re-run these)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/collections/option.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/collections/option.t27 | head -1
option.t27: 7 node types

$ grep -c '// TODO: Implement from .tri spec' specs/tri/collections/option.t27
4

$ ./target/release/t27c lint --json specs/tri/collections/option.t27 | python3 -c 'import json,sys; d=json.load(sys.stdin); print(d["functions"], d["tests"], d["invariants"])'
4 4 0
```

The type, quoted verbatim from the file:

```t27
    pub const Option(T) = struct {
        is_some : bool,
        value : T,
    };
```

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. **Keep every signature exactly
as it stands**, and **do not touch the type declaration** — the four existing tests assert its
shape and they are correct. These are the four, quoted verbatim:

```t27
    fn some(value: T) -> Option(T) {
    fn none() -> Option(void) {
    fn unwrap_or(opt: Option(T), default: T) -> T {
    fn is_some(opt: Option(T)) -> bool {
```

What each must do:

- **`some`** — build the present case: `Option(T){ .is_some = true, .value = value }`. One
  statement. The payload is stored by value, not by reference.
- **`none`** — build the absent case. Note the return type the file actually declares:
  `Option(void)`, not `Option(T)`. So this returns the absent case over the empty payload type,
  with `is_some = false` and the `void` payload slot. Do not "fix" the signature to `Option(T)`
  — implement what is declared, and if you think the declaration is wrong, say so in a comment
  above the function rather than changing it.
- **`unwrap_or`** — return `opt.value` when `opt.is_some` is true, and `default` otherwise. The
  point of this function is that it never inspects `value` in the absent case, so read the flag
  first and return `default` before touching the payload.
- **`is_some`** — return `opt.is_some`. The accessor exists so callers never reach into the
  representation directly; keep it a one-line read of the field.

The four existing test blocks are real assertions about the declared shape (the present case, the
absent case, that the flag alone distinguishes them, and that `Option` is generic over its
payload) and they are correct — **do not delete or rewrite them**. Add a `test` block per
implemented function underneath, in the file's own `test name / given / when / then / and` style
(no braces, no quoted test name — copy the layout of the existing blocks). Worth pinning:
`some(7).is_some` is true and `some(7).value` is `7`; `none().is_some` is false;
`unwrap_or(some(7), 0)` is `7` and `unwrap_or` on an absent option returns the default;
`is_some(some(7))` is true. There is **no t27 interpreter** — these blocks are parsed and
counted, never executed. Write them as the specification of the behaviour, and do not claim you
ran them.

### Parser hazards (measured — these silently destroy the file)

The bootstrap parser swallows declarations on two constructs *without* raising
`recovery-events`, so the file can look clean and still have lost half its declarations:

- `switch` arms with a **qualified** pattern (`Foo.bar => ...`) — measured: `FnDecl` fell from 4
  to 2 with all three parse metrics still reporting 0. Use `if` / `else if` chains instead.
  Literal arms (`0 => ...`) are safe.
- The **anonymous struct literal** `.{ ... }` — measured: `recovery-events: 1` plus swallowed
  declarations. The *named* struct literal `Option(T){ .is_some = true, .value = value }` is
  safe and measured clean; it is only the leading-dot anonymous form that breaks.

Also measured clean in bodies: `if`/`else if`/`else`, `while`, `for (xs) |x|`, `return`,
`const`/`var` locals, `try`, `catch`, `orelse`, `defer`, `@as` / `@intCast`, and `and`/`or`.
Run the real-code node command after every edit, not just `t27c parse` — that is why
criterion 3 exists.

Do **not** gate yourself on `t27c typecheck` — it returns `{"ok":true,"errors":0}` on English
prose and on Python files. It proves nothing. Ignore `t27c lint`'s `"warnings"` array too — it
reports "fn X has no test or invariant" for every function in the gold corpus. Its `"functions"`
and `"tests"` numbers are the useful part.

### House law

Repo law L3 (PURITY) requires source to be ASCII-only with English identifiers. The file's
existing banner comments already contain non-ASCII decoration — leave those lines exactly as they
are, and keep every line **you** add pure ASCII. Law L4 (TESTABILITY) is why the added `test`
blocks are not optional.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 4 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/collections/option.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/collections/option.t27 | grep -c 'kind: TestBlock'` prints at least 8
- 5. `t27c count specs/tri/collections/option.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `5` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `4` | `>= 8` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (some|none|unwrap_or|is_some)\(' specs/tri/collections/option.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/collections/option.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/collections/option.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/collections/option.t27
```

Today it prints `5`. It must print `0`.

That is `5`, not `4`. 4 of them are the stub bodies; the other 1 sits in a prose
comment further down the file which states that these functions are still
unimplemented. That sentence becomes false when you finish, so rewrite or delete
it as part of the task.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/collections/option.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/collections/option.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `8` — the 4 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/collections/option.t27 | head -1
```

Today it prints `option.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/collections/option.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/collections/option.t27 | grep -c 'kind: TestBlock' prints at least 8: met
- 5. t27c count specs/tri/collections/option.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/collections/option.t27
