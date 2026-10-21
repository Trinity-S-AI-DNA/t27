# Implement the 10 pure path and content bodies in specs/file/operations.t27

`.t27` is the hand-authored source language of this repository. You are writing
source, not compiler output: `t27c gen` compiles `.t27` down to Zig under `gen/`,
and L2 GENERATION in CLAUDE.md says files under `gen/` are generated and must
not be edited. Everything you write
goes in the `.t27` file itself, in `.t27` syntax, with real control flow.

THIS IS ONE OF THREE ISSUES THAT SHARE ONE BOUNDARY FILE. The other two are
`file-operations-read-and-write.md` (12 functions) and
`file-operations-directory-and-ignore.md` (11 functions). All three edit
`specs/file/operations.t27`. **They must not run concurrently.** The order is
fixed: this issue first, then read-and-write, then directory-and-ignore. The
expected counts in the acceptance criteria below assume you are running first on
an untouched file. If `./target/release/t27c gen specs/file/operations.t27 2>&1 |
grep -c 'not yet implemented'` does not print `33` when you start, stop and report
that another issue has already run.

## 1. Context -- what this module is for

`specs/file/operations.t27` is the file-system capability layer: 33 functions
covering reading, writing, directory traversal, ignore rules, path arithmetic and
content classification. It is the capability any `.t27`-authored swarm tool needs
before it can touch a file at all. Every one of the 33 bodies is empty today.

```
module FileOperations {
    use base::types;
    use file::schema;
```

The file declares no types of its own. The contract lives in
`specs/file/schema.t27`, which is `IMPLEMENTED`. The two type declarations your
ten functions operate on, quoted verbatim from that file:

```
    // ContentType represents the type of file content
    enum ContentType {
        Text = 0,              // Text content
        Binary = 1,            // Binary content
        Image = 2,             // Image content
    }

    // FileError represents errors in file operations
    enum FileError {
        NotFound = 0,
        PermissionDenied = 1,
        IsDirectory = 2,
        NotDirectory = 3,
        AccessDenied = 4,
        InvalidPath = 5,
        ReadError = 6,
        WriteError = 7,
        DeleteError = 8,
    }
```

`relative` is the only function in your group that returns a `Result`, and
`FileError::InvalidPath` is the member it exists to return.

The schema also already implements two path helpers in the same dialect you are
about to write in. **Read them first -- they are the style model and they are three
lines each:**

```
    // is_hidden checks if a path is hidden
    fn is_hidden(path: str) -> bool {
        var parts = split(path, "/");
        for part in parts {
            if part.len > 0 && part[0] == '.' && part != "." && part != ".." {
                return true;
            }
        }
        return false;
    }

    // get_extension returns the file extension
    fn get_extension(path: str) -> str {
        var parts = split(path, ".");
        if parts.len <= 1 {
            return "";
        }
        return parts[parts.len - 1];
    }
```

Note what those two establish, because it governs everything below:

- `"/"` is the separator this module uses. `split(s, sep)` is called by bare name
  with no import and no definition anywhere in `specs/`. That is the module's
  existing convention and `specs/file/schema.t27` is marked `IMPLEMENTED` while
  doing it. Follow it -- call the same helpers by the same bare names. Do not
  invent an `extern`, an `@syscall` or a `std.` call; nothing under `specs/` uses
  one (`grep -rn 'std\.fs\|openFile\|readFileAlloc\|cwd()' specs/ --include='*.t27'`
  matches 0 lines).
- `extname` in your group and `get_extension` in the schema overlap. Do not delete
  either and do not edit the schema. `extname` is inside your boundary and
  `get_extension` is not.

A longer style model, if you want loops and index arithmetic rather than `split`:
`specs/compiler/stdlib.t27` is 50 functions with 0 empty bodies, e.g.

```
    fn str_index_of(s: Str, ch: u8) -> i32 {
        var i: u32 = 0;
        while (i < s.len) {
            if (s.data[i] == ch) {
                return i as i32;
            }
            i = i + 1;
        }
        return -1;
    }
```

Neighbours, for orientation only and out of boundary: `specs/file/schema.t27` is
`IMPLEMENTED`, `specs/file/watcher.t27` is `NOPARSE` (20 of 20 bodies empty), and
`specs/tri/io/fs.t27` declares its own `join` / `basename` / `dirname` over a
different `Path` struct and is `UNWRITTEN` (4 of 4 empty). Nothing in `specs/`
imports this module -- `grep -rn 'FileOperations\|file::operations' specs/
--include='*.t27'` matches only line 5 of the boundary file itself. So nothing
depends on you yet and nothing you write here can break another spec.

## 2. Current state, with the commands that prove it

The file does NOT parse clean. PARSE-GATE is the exact command the acceptance
criteria below refer to -- three metric lines, all of which must read `0`:

```
./target/release/t27c parse specs/file/operations.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

Today it prints:

```
recovery-events: 1
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use exactly that `grep -E`, never `| tail -3`. Widening it to show where the
recovery happened makes the trap visible -- the parser interleaves a `recovery-at:`
line between `recovery-events` and `declarations-swallowed`, and prints
`  discarded: ...` diagnostics after the metrics, so `tail -3` hides the non-zero
`recovery-events` entirely:

```
$ ./target/release/t27c parse specs/file/operations.t27 2>&1 | grep -E '^(recovery-events|recovery-at|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 1
recovery-at: Unexpected top-level token: Ident ('assert') at line 299:9
declarations-swallowed: 0
lexer-discarded-chars: 0
```

**Fixing that one recovery event is part of this issue.** The cause is not at line
299; it is two lines above, at line 297, inside a test struct literal:

```
$ sed -n '295,299p' specs/file/operations.t27
        var rules = IgnoreRules {
            patterns = patterns,
            whitelists: [],
        };
        assert(rules.patterns.len == 1);
```

Struct *literals* in this corpus use `field = value`. Struct *declarations* use
`field: type`. Line 297 uses a colon in a literal. Changing that single token to
`whitelists = [],` clears the event -- verified on a scratch copy of the file:

```
$ ./target/release/t27c parse <scratch copy of the file, line 297 fixed> 2>&1 | grep -E '^(recovery-events|recovery-at|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Change ONLY line 297. It is the file's single colon-style literal field. Do not run
a blanket colon-to-equals rewrite: the struct declarations in `specs/file/schema.t27`
and the type annotations in this file's own signatures are correctly colons, and a
blanket rewrite destroys them.

Every function body is empty:

```
$ ./target/release/t27c gen specs/file/operations.t27 2>&1 | grep -c 'not yet implemented'
33
```

```
$ t27c spec-status specs/file/operations.t27
UNWRITTEN
```

THE TRAP THAT WILL OTHERWISE MAKE THIS ISSUE PASS BEFORE YOU START. Most of the
corpus marks an unwritten body with the literal line `// TODO: Implement from .tri
spec`. This file does not. It writes:

```
    // normalize normalizes a file path
    fn normalize(path: str) -> str {
        // Implementation: Normalize path separators and resolve . and ..
    }
```

A body holding only a comment. So the usual marker grep reads as fully implemented:

```
$ grep -c 'TODO: Implement' specs/file/operations.t27
0
```

Never measure this file with a marker string. Measure empty bodies by brace
matching (the DETECTOR below), and measure codegen with
`grep -c 'not yet implemented'`, which is binary-independent: the Sep 8 binary
emits `@panic("not yet implemented")` and the Sep 9 one `@compileError("not yet
implemented")`.

DO NOT gate on `t27c test-report`. It is BLOCKED for a reason you cannot fix from
inside the boundary:

```
$ t27c test-report specs/file/operations.t27 2>&1 | tail -4
  BLOCKED  does not compile: /var/folders/9d/5s6sn_xj6q119lz_ht66mkcm0000gn/T/t27c-test-report-operations/spec.zig:263:86: error: expected expression, found '/'
```

The cause is codegen stripping the quotes off a string literal. The `.t27` source
is correct:

```
$ grep -n 'home/project' specs/file/operations.t27
357:            paths = ["/home/project"],
$ t27c gen specs/file/operations.t27 2>/dev/null | sed -n '263p'
    var handle = WatcherHandle{ .id = "watch-1", .paths = @constCast(&[_][]const u8{ /home/project }), .active = true };
```

Note the binary: that `sed -n '263p'` is the PATH build, whose output for this file is
461 lines and is the same codegen `test-report` runs. `./target/release/t27c gen` emits
246 lines for this file and prints nothing at line 263 -- do not substitute it here.
(Both builds agree on `grep -c 'not yet implemented'`, which is why the criteria can
pin the release build for that one.)

That is a codegen gap, not your bug. Leave line 357 alone. Note it in your report
so it can be filed separately. `specs/file/schema.t27` is `BLOCKED` at compile too,
while being marked `IMPLEMENTED` -- compiling is not this module's bar.

Also do not gate on `t27c typecheck --json`. It returns `{"ok":true,"errors":0}` on
plain English prose, so it proves nothing.

## 3. What to write

Ten functions, all of them **pure**: they take strings and numbers and return
strings, numbers or an enum. None of them needs a file-system call, which is why
this group goes first -- it is the only one of the three that is fully writable
with what exists in `specs/` today.

Each function's existing `// Implementation:` comment is the module's decision
about what the body does. It is quoted below verbatim. Implement that. Do not
substitute different behaviour and do not relitigate the choice -- if you believe
one is wrong, implement it as written and say so in your report.

Do not add, remove or rename any function, and do not change any signature. Do
not delete or rewrite the `// Tests` block at the bottom of the file either --
L4 TESTABILITY in CLAUDE.md requires every `.t27` spec to contain a `test`, and
those tests pin field values you must match. There
are 33 `fn` declarations in the file and there must still be 33 when you finish.

### Path Operations (7 functions)

They live under this banner in the file:

```
    // ====================================================================
    // Path Operations
    // ====================================================================
```

```
    // normalize normalizes a file path
    fn normalize(path: str) -> str {
        // Implementation: Normalize path separators and resolve . and ..
    }
```
Collapse repeated `/`, drop `.` segments, and pop the previous segment for each
`..`. Preserve a leading `/` if the input had one. This is the one function whose
comment names `..` explicitly, and criterion 4 checks that a body actually mentions
it.

```
    // join joins path components
    fn join(parts: [str]) -> str {
        // Implementation: Join path parts
    }
```
Join with `"/"`, skipping empty parts so `join(["a", "", "b"])` is `"a/b"` and not
`"a//b"`.

```
    // dirname returns the directory name of a path
    fn dirname(path: str) -> str {
        // Implementation: Get directory name
    }
```
Everything before the last `"/"`. A path with no `"/"` has dirname `"."`.

```
    // basename returns the file name of a path
    fn basename(path: str) -> str {
        // Implementation: Get file name
    }
```
Everything after the last `"/"`; the whole string when there is no `"/"`.

```
    // extname returns the file extension
    fn extname(path: str) -> str {
        // Implementation: Get file extension
    }
```
Match `get_extension` in `specs/file/schema.t27`, which this file's own test pins:
`assert(get_extension("test.txt") == "txt")` and
`assert(get_extension("file.tar.gz") == "gz")`, with `""` for a name carrying no
dot. Take the extension of the `basename`, not of the whole path, so a dot in a
directory name does not leak into the result.

```
    // relative returns relative path from base to path
    fn relative(from: str, to: str) -> Result<str, FileError> {
        // Implementation: Get relative path
    }
```
Normalize both, strip the shared leading segments, emit one `..` per remaining
segment of `from`, then the remaining segments of `to`. Return
`Err(FileError::InvalidPath)` when the two cannot be related -- one absolute and
one relative. This is the only function in the group that returns a `Result` and
the only place `FileError` is reachable.

```
    // resolve resolves a path to absolute form
    fn resolve(base: str, path: str) -> str {
        // Implementation: Resolve path relative to base
    }
```
If `path` is already absolute, return `normalize(path)`; otherwise return
`normalize(join([base, path]))`. Call your own `normalize` and `join` -- do not
duplicate their logic.

NOT IN THIS GROUP: `fn absolute(path: str) -> Result<str, FileError>` is the eighth
function under the Path Operations banner and it is **excluded on purpose**. It
needs the process working directory, and no notion of one exists anywhere in
`specs/file/` or `specs/shell/environment.t27` (`grep -rn 'cwd' specs/file/*.t27`
exits 1). It is assigned to `file-operations-directory-and-ignore.md` along with
the other functions that need something that does not exist yet. Leave its body
empty.

### Content Operations (3 functions)

They live under this banner:

```
    // ====================================================================
    // Content Operations
    // ====================================================================
```

```
    // detect_type detects content type
    fn detect_type(path: str, content: str) -> ContentType {
        // Implementation: Detect if text, binary, or image
    }
```
Return one of `ContentType::Image`, `ContentType::Binary`, `ContentType::Text`, in
that order of checking. Decide `Image` from the extension of `path` (use your own
`extname`). Decide `Binary` from `content` -- the conventional test is a NUL byte
or a high proportion of non-printable bytes. Otherwise `Text`. Criterion 5 checks
that a body actually names a `ContentType::` variant.

```
    // count_lines counts lines in content
    fn count_lines(content: str) -> u32 {
        // Implementation: Count line breaks
    }
```
The comment says line breaks, so count `'\n'`. Empty content is `0`. This feeds
`FileContent.lineCount`, which `specs/file/schema.t27` declares as `lineCount: u32?`
with the comment `// Number of lines (text only)`.

```
    // truncate truncates content to max length
    fn truncate(content: str, maxLength: u32) -> str {
        // Implementation: Truncate and add ellipsis
    }
```
Return `content` unchanged when it is at or under `maxLength`. Otherwise cut and
append an ellipsis -- the comment mandates the ellipsis, so it is not optional.
The related budget already declared in the schema is
`const MAX_FILE_SIZE: u64 = 10485760;`.

## 4. The detector, and how you are measured

Run from the repo root (`/Users/playom/t27`). Brace-matching, comment-stripping,
no marker string. This is DETECTOR-A and it is scoped to your ten functions.

```
python3 - <<'EOF'
import re
GROUP={"normalize","join","dirname","basename","extname","relative","resolve",
       "detect_type","count_lines","truncate"}
t=open("specs/file/operations.t27",encoding="utf-8").read()
empty=[]
for m in re.finditer(r'(?m)^\s*(?:pub\s+)?fn\s+(\w+)\s*\(', t):
    if m.group(1) not in GROUP: continue
    i=t.find('{', m.end()); semi=t.find(';', m.end())
    if i<0 or (semi!=-1 and semi<i): continue
    depth=0; j=i
    while j<len(t):
        if t[j]=='{': depth+=1
        elif t[j]=='}':
            depth-=1
            if depth==0: break
        j+=1
    b=re.sub(r'/\*.*?\*/','',t[i+1:j],flags=re.S); b=re.sub(r'(?m)//.*$','',b)
    if not b.strip(): empty.append(m.group(1))
print("group empty bodies:", len(empty))
print("still empty:", " ".join(sorted(empty)) or "(none)")
EOF
```

Verbatim output today:

```
group empty bodies: 10
still empty: basename count_lines detect_type dirname extname join normalize relative resolve truncate
```

SIGNATURE GUARD. The count and checksum below prove you did not delete a function
or edit a signature to make a number move. Both must be unchanged when you finish.

```
$ grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/file/operations.t27 | wc -l | tr -d ' '
33
$ grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/file/operations.t27 | shasum | cut -c1-12
005522d8d251
```

GOLD BASELINE. 457 of 506 files under `specs/` (90.3%) are clean on all three parse
signals. `specs/file/operations.t27` is currently one of the 49 that are NOT.
Criterion 3 moves it into the 457.

## Acceptance criteria

Answer each as `- N. <criterion>: met | unmet | could-not-check` in a `## VERDICT`
block. Every command runs from `/Users/playom/t27`.

- 1. DETECTOR-A prints `group empty bodies: 0` -- today `10`.
- 2. `./target/release/t27c gen specs/file/operations.t27 2>&1 | grep -c 'not yet implemented'` prints `23` -- today `33`.
- 3. PARSE-GATE prints three lines each ending in ` 0` -- today `recovery-events: 1`.
- 4. `awk '/^[[:space:]]*test "/{exit} /"\.\."/{n++} END{print n+0}' specs/file/operations.t27` prints 1 or more -- today `0`.
- 5. `awk '/^[[:space:]]*test "/{exit} /ContentType::/{n++} END{print n+0}' specs/file/operations.t27` prints 1 or more -- today `0`.
- 6. `grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/file/operations.t27 | shasum | cut -c1-12` still prints `005522d8d251` AND DETECTOR-A prints `group empty bodies: 0` -- the checksum already matches today while DETECTOR-A prints `10`, so this is unmet until you finish.

Criterion 2 is `33 - 10 = 23`: the other 23 bodies are not yours and must still be
empty. Criteria 4 and 5 stop at the first `test "` line, so they prove the text
appears in a *function body* and not merely in the test block at the bottom of the
file; 4 proves `normalize` handles `..` as its own comment mandates, 5 proves
`detect_type` returns a real enum variant.

Criterion 6 is the signature guard, and it is deliberately fused with DETECTOR-A.
On its own -- "the checksum is unchanged" -- it passes before any work is done and
a bee could honestly answer it `met` without touching the file. It also closes a
hole DETECTOR-A cannot: DETECTOR-A only counts empty bodies among a fixed set of
ten names, so DELETING `fn truncate` would make it print `0` as surely as writing
the body would. The checksum catches that, and catches a renamed parameter too.

All six criteria were run against the unmodified file on 2026-09-15; all six fail
today.

Criteria 2 and 3 need `./target/release/t27c` (the Sep 8 build); it is the only one
that prints parse metrics. `t27c spec-status` needs the PATH binary (Sep 9 build)
but is NOT a criterion here, because the other 23 bodies are still empty and it
will correctly keep printing `UNWRITTEN` until all three issues are done.

## Boundary

specs/file/operations.t27
