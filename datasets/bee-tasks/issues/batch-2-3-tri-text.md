# Implement the four stubbed functions in TriText (specs/tri/utils/text.t27)

## Context

`specs/tri/utils/text.t27` is the plain-text layout module of the `tri` standard library: wrap a
paragraph to a column width, indent a block, and count the words and lines in it. Its header says
exactly how far it reaches — `// Basic ASCII text processing` — so a byte is a character here and
no UTF-8 decoding belongs in this file; `specs/tri/utils/utf8.t27` is the module that owns that,
and `specs/tri/utils/terminal.t27` owns the escape sequences. `text.t27` declares one type,
`TextMetrics { width, height, lines }`, and four free functions over `[]const u8`. All four are
empty stubs today, so the module currently states signatures and nothing more.

**.t27 is hand-authored source.** It is compiled out to C/Rust/Verilog/Zig by
`/Users/playom/t27/target/release/t27c`. You are writing source, not compiler output. The
dialect is module-code: `module X;`, `use a::b;`, `fn name(arg : type) -> ret { ... }`, and
`test` blocks. Match the style already in the file.

## Exact current state (measured 2026-09-09, re-run these)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/utils/text.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/utils/text.t27 | head -1
text.t27: 7 node types

$ grep -c '// TODO: Implement from .tri spec' specs/tri/utils/text.t27
4
```

The whole file is 66 lines and parses clean today. Do not regress that.

## What to write

Replace each `// TODO: Implement from .tri spec` with a real body. **Keep every signature
exactly as it stands** — same name, same parameter names, same types, same return type. These
are the four, quoted verbatim:

```t27
    fn word_wrap(allocator: std.mem.Allocator, text: []const u8, width: usize) -> ![]u8 {
    fn count_words(text: []const u8) -> usize {
    fn count_lines(text: []const u8) -> usize {
    fn indent(allocator: std.mem.Allocator, text: []const u8, spaces: usize) -> ![]u8 {
```

What each must do — and the edge cases are the specification, so decide each one explicitly and
say so in a comment:

- **`count_words`** — the number of runs of non-whitespace bytes. Whitespace is space, tab,
  newline and carriage return. Empty input is `0`; leading, trailing and repeated separators must
  not inflate the count, which means counting *transitions* into a word, not separators.
- **`count_lines`** — the number of lines. State your rule and hold to it: the sensible one is
  "number of `\n` bytes, plus one if the text is non-empty and does not end in `\n`", so `""` is
  `0`, `"a"` is `1`, `"a\n"` is `1` and `"a\nb"` is `2`. A rule that makes `"a\n"` two lines is
  a different function; pick one and write it down.
- **`word_wrap`** — greedy wrap to `width` columns: accumulate words separated by single spaces,
  break to a new line before the word that would exceed `width`, never split a word in the
  middle. A word longer than `width` goes on its own line rather than being truncated. `width`
  of `0` returns a copy of the input rather than looping forever. Returns `![]u8` because it
  allocates the result — the caller frees it.
- **`indent`** — prefix every line with `spaces` space characters, including the first, and
  including lines after the final newline only if that trailing piece is non-empty. Do not indent
  a blank line into a line of trailing whitespace: leave empty lines empty. Allocates; caller
  frees.

The four existing test blocks are placeholders of the shape

```t27
    test count_words_basic_case
        given input = default_input()
        when result = count_words(input)
        then result != undefined
```

`default_input()` is defined nowhere in the module, so these assert nothing. Leave them and
**add** a real `test` block per function underneath, in the file's own
`test name / given / when / then / and` style (no braces, no quoted name — copy the layout
above). Pin the exact edge cases you chose above: `count_words("")` is `0`,
`count_words("  a   b  ")` is `2`, `count_lines("a\nb")` is `2`, `count_lines("a\n")` is `1`,
a wrap at width 5 of `"aaa bbb"` breaks between the words, `indent` of an empty line adds
nothing. There is **no t27 interpreter** — these blocks are parsed and counted, never executed.
Write them as the specification of the behaviour, and do not claim you ran them.

### Parser hazards (measured — these silently destroy the file)

The bootstrap parser swallows declarations on two constructs *without* raising
`recovery-events`, so the file can look clean and still have lost half its functions:

- `switch` arms with a **qualified** pattern (`Foo.bar => ...`) — measured on this very file:
  `FnDecl` fell from 4 to 2 with all three parse metrics still reporting 0. Use `if` / `else if`
  chains instead. Literal arms (`0 => ...`) are safe.
- The **anonymous struct literal** `.{ ... }` — measured on this file: `recovery-events: 1` and
  `FnDecl` fell from 4 to 2.

Safe and measured-clean in bodies: `if`/`else if`/`else`, `while`, `for (xs) |x|`, `return`,
`const`/`var` locals, `try`, `catch`, `orelse`, `defer`, builtins like `@as` / `@intCast`,
slicing `xs[0..n]`, struct literals `TextMetrics{ .width = w, .height = h, .lines = l }`, and
`and`/`or`. Run `t27c count ... | head -1` after every edit, not just `t27c parse` — that is
why criterion 5 exists.

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

- 1. all 4 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/utils/text.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/utils/text.t27 | grep -c 'kind: TestBlock'` prints at least 8
- 5. `t27c count specs/tri/utils/text.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `4` | `>= 8` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (word_wrap|count_words|count_lines|indent)\(' specs/tri/utils/text.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/utils/text.t27 <<'EOF'
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
$ ./target/release/t27c parse specs/tri/utils/text.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/utils/text.t27
```

Today it prints `4`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/utils/text.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/utils/text.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `4`. It must print at least `8` — the 4 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/utils/text.t27 | head -1
```

Today it prints `text.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 2. grep -c 'TODO: Implement' specs/tri/utils/text.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/utils/text.t27 | grep -c 'kind: TestBlock' prints at least 8: met
- 5. t27c count specs/tri/utils/text.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/utils/text.t27
