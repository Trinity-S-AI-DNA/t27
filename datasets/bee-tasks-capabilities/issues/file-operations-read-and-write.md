# Implement the 12 read and write bodies in specs/file/operations.t27

`.t27` is the hand-authored source language of this repository. You are writing
source, not compiler output: `t27c gen` compiles `.t27` down to Zig under `gen/`,
and L2 GENERATION in CLAUDE.md says files under `gen/` are generated and must
not be edited. Everything you write
goes in the `.t27` file itself, in `.t27` syntax, with real control flow.

THIS IS THE SECOND OF THREE ISSUES THAT SHARE ONE BOUNDARY FILE. The others are
`file-operations-path-and-content.md` (10 functions) and
`file-operations-directory-and-ignore.md` (11 functions). All three edit
`specs/file/operations.t27`. **They must not run concurrently.** The order is
fixed: path-and-content first, then this one, then directory-and-ignore.

**THIS ISSUE HAS A HARD PREREQUISITE.** `file-operations-path-and-content.md` must
be finished and merged first, because eight of your twelve functions are written in
terms of the path and content helpers it delivers. Check before you start:

```
$ ./target/release/t27c gen specs/file/operations.t27 2>&1 | grep -c 'not yet implemented'
23
```

If that prints `33`, path-and-content has not run -- stop and report it. If it
prints anything other than `23` or `33`, another issue has already touched the
file -- stop and report that.

## 1. Context -- what this module is for

`specs/file/operations.t27` is the file-system capability layer: 33 functions
covering reading, writing, directory traversal, ignore rules, path arithmetic and
content classification. Your twelve are the two banners it opens with, `// Read
Operations` and `// Write Operations`. They are what any `.t27`-authored swarm tool
calls to actually move bytes.

```
module FileOperations {
    use base::types;
    use file::schema;
```

The file declares no types of its own. The contract lives in
`specs/file/schema.t27`, which is `IMPLEMENTED`. The declarations your twelve
functions operate on, quoted verbatim from that file:

```
    // FileInfo represents metadata about a file
    struct FileInfo {
        path: str,             // File path
        fileType: FileType,    // Type of file
        size: u64,             // File size in bytes
        modified: u64,         // Last modified timestamp (Unix epoch)
        permissions: u32,      // File permissions (octal)
        isHidden: bool,        // Whether file is hidden
        isIgnored: bool,       // Whether file is ignored
    }

    // FileContent represents the content of a file
    struct FileContent {
        type: ContentType,     // Type of content
        content: str,          // File content (text or encoded)
        diff: str?,            // Git diff if available
        mimeType: str,         // MIME type
        encoding: str?,        // Encoding (e.g., "base64" for images)
        lineCount: u32?,       // Number of lines (text only)
    }

    // FileType represents the type of a file system entry
    enum FileType {
        File = 0,              // Regular file
        Directory = 1,         // Directory
        Symlink = 2,           // Symbolic link
    }

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

    const MAX_FILE_SIZE: u64 = 10485760;      // 10MB max file size
    const DEFAULT_READ_CHUNK: u32 = 8192;      // 8KB default read chunk
```

`FileError` has nine members and they are the module's decision about how failure
is reported. Use them. `MAX_FILE_SIZE` is not decoration -- it is the read budget
and criterion 5 checks that a body enforces it.

Neighbours, for orientation only and out of boundary: `specs/file/schema.t27` is
`IMPLEMENTED`, `specs/file/watcher.t27` is `NOPARSE` (20 of 20 bodies empty).
Nothing in `specs/` imports this module -- `grep -rn 'FileOperations\|file::operations'
specs/ --include='*.t27'` matches only line 5 of the boundary file itself. So
nothing depends on you yet and nothing you write here can break another spec.

## 2. THE ONE THING THAT MAKES THIS ISSUE DIFFERENT

**There is no file-system primitive anywhere in `specs/`.** This is measured, not
assumed:

```
$ grep -rn 'std\.fs\|openFile\|readFileAlloc\|cwd()' specs/ --include='*.t27' | wc -l
0
```

`specs/tri/io/fs.t27`, `specs/tri/io/filesystem.t27`, `specs/tri/io/reader.t27` and
`specs/tri/io/writer.t27` all exist and are all `UNWRITTEN` -- 4, 7, 4 and 3 empty
bodies respectively. So no sibling can be imported and no `std.` call is available.
Your twelve functions cannot reach the disk on their own.

The corpus already has a convention for exactly this, and it is in your own sibling
file. `specs/file/schema.t27` is marked `IMPLEMENTED` while calling a helper that is
defined nowhere and imported from nowhere:

```
    fn is_hidden(path: str) -> bool {
        var parts = split(path, "/");
```

```
$ ./target/release/t27c gen specs/file/schema.t27 2>/dev/null | grep -n 'split'
153:    const parts = split(path, "/");
162:    const parts = split(path, ".");
```

Codegen emits the bare call unchanged. `spec-status` reports `IMPLEMENTED` on body
presence, not on resolution. Follow that convention exactly.

**THREE SEAM NAMES, FIXED HERE SO ALL THREE ISSUES AGREE.** This is an
issue-author decision made for consistency, not a language feature. Use these
names and no others:

```
    fs_read(path: str)              -> Result<[u8], FileError>
    fs_write(path: str, data: [u8]) -> Result<void, FileError>
    fs_stat(path: str)              -> Result<FileInfo, FileError>
```

Call them by bare name, exactly as `split` is called. **Do NOT add `fn` declarations
for them to this file** -- the file has 33 `fn` declarations and must still have 33
when you finish (criterion 6). Do NOT write an `extern`, an `@syscall`, an
`@import` or a `std.` call; nothing under `specs/` does. Name all three in your
report so a `specs/file/io.t27` issue can be filed for them.

Only four of your twelve functions touch the seam. The other eight are pure
compositions of those four plus the path and content helpers that
`file-operations-path-and-content.md` already delivered. Write them that way. Do
not open a second route to the disk.

## 3. Current state, with the commands that prove it

Parse metrics, after path-and-content has run:

PARSE-GATE, referenced by the acceptance criteria below:

```
$ ./target/release/t27c parse specs/file/operations.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use exactly that `grep -E`, never `| tail -3`. On a dirty file the parser
interleaves a `recovery-at:` line between the metrics and prints `  discarded: ...`
diagnostics after them, so `tail -3` both hides a non-zero `recovery-events` and
shows diagnostics instead of metrics. On the untouched file those three lines read
`recovery-events: 1` / `declarations-swallowed: 0` / `lexer-discarded-chars: 0`;
path-and-content is the issue that clears it. Do not re-fix it and do not let it
come back.

THE TRAP THAT WILL OTHERWISE MAKE THIS ISSUE PASS BEFORE YOU START. Most of the
corpus marks an unwritten body with the literal line `// TODO: Implement from .tri
spec`. This file does not. It writes:

```
    // read reads a file's contents
    fn read(path: str, includeDiff: bool) -> Result<FileContent, FileError> {
        // Implementation: Read file and return content
    }
```

A body holding only a comment. So the usual marker grep reads as fully implemented:

```
$ grep -c 'TODO: Implement' specs/file/operations.t27
0
```

Never measure this file with a marker string. Measure empty bodies by brace
matching (DETECTOR-B below), and measure codegen with
`grep -c 'not yet implemented'`, which is binary-independent: the Sep 8 binary
emits `@panic("not yet implemented")` and the Sep 9 one `@compileError("not yet
implemented")`.

DO NOT gate on `t27c test-report`. It is BLOCKED for a reason you cannot fix from
inside the boundary:

```
$ t27c test-report specs/file/operations.t27 2>&1 | tail -4
  BLOCKED  does not compile: /var/folders/9d/5s6sn_xj6q119lz_ht66mkcm0000gn/T/t27c-test-report-operations/spec.zig:263:86: error: expected expression, found '/'
```

Codegen strips the quotes off a string literal. The `.t27` source is correct:

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

That is a codegen gap, not your bug. Leave line 357 alone. `specs/file/schema.t27`
is `BLOCKED` at compile too while being `IMPLEMENTED` -- compiling is not this
module's bar. Also do not gate on `t27c typecheck --json`: it returns
`{"ok":true,"errors":0}` on plain English prose.

## 4. What to write

Each function's existing `// Implementation:` comment is the module's decision
about what the body does. It is quoted below verbatim. Implement that. Do not
substitute different behaviour and do not relitigate the choice -- if you believe
one is wrong, implement it as written and say so in your report.

Do not add, remove or rename any function, and do not change any signature. Do
not delete or rewrite the `// Tests` block at the bottom of the file either --
L4 TESTABILITY in CLAUDE.md requires every `.t27` spec to contain a `test`, and
those tests pin field values you must match.

**If you run out of budget, finish Tier 1 completely and leave Tier 2 untouched**
rather than leaving all twelve half-written. Tier 2 is unwritable without Tier 1
anyway.

### Tier 1 -- the four that touch the seam. Write these first.

```
    // read_binary reads binary file content
    fn read_binary(path: str) -> Result<[u8], FileError> {
        // Implementation: Read binary file
    }
```
`fs_read(path)` and propagate the error unchanged. Reject a path whose length
would exceed `MAX_FILE_SIZE` with `Err(FileError::ReadError)`.

```
    // write_binary writes binary content to a file
    fn write_binary(path: str, data: [u8]) -> Result<void, FileError> {
        // Implementation: Write binary data to file
    }
```
`fs_write(path, data)` and propagate the error unchanged.

```
    // stat returns file information
    fn stat(path: str) -> Result<FileInfo, FileError> {
        // Implementation: Get file metadata
    }
```
`fs_stat(path)`. Fill `isHidden` from the schema's own `is_hidden(path)` rather
than recomputing it.

```
    // exists checks if a file exists
    fn exists(path: str) -> Result<bool, FileError> {
        // Implementation: Check file existence
    }
```
Note the return type: `Result<bool, ...>`, not `bool`. `Err(FileError::NotFound)`
from `stat` becomes `Ok(false)`, success becomes `Ok(true)`, and any other
`FileError` propagates as an error. That distinction -- "absent" versus "could not
tell" -- is why this returns a `Result` and it is the whole point of the function.

### Tier 2 -- the eight compositions. No seam calls here.

```
    // read reads a file's contents
    fn read(path: str, includeDiff: bool) -> Result<FileContent, FileError> {
        // Implementation: Read file and return content
    }
```
Call `read_binary`, then fill every field of `FileContent`: `type` from
`detect_type(path, content)`, `content`, `mimeType`, `encoding` (`"base64"` when
`detect_type` says `Image`, per the schema's own comment on that field), and
`lineCount` from `count_lines(content)` for text only. `includeDiff` selects
whether `diff` is populated; there is no git access in this module, so when
`includeDiff` is true and no diff is available, set `diff = null`. Say so in your
report.

```
    // read_range reads a range of lines from a file
    fn read_range(path: str, startLine: u32, endLine: u32) -> Result<str, FileError> {
        // Implementation: Read specific line range
    }
```
Call `read`, split on `'\n'`, return lines `startLine..endLine`. The schema's
`SearchMatch.line` carries the comment `// Line number (1-indexed)`, so treat line
numbers as 1-indexed here too. An out-of-range or inverted range is
`Err(FileError::InvalidPath)`.

```
    // write writes content to a file
    fn write(path: str, content: str) -> Result<void, FileError> {
        // Implementation: Write content to file
    }
```
Delegate to `write_binary`.

```
    // append appends content to a file
    fn append(path: str, content: str) -> Result<void, FileError> {
        // Implementation: Append content to file
    }
```
`read` then `write` the concatenation. A missing file appends to empty rather than
failing.

```
    // edit edits a file with replacements
    fn edit(path: str, oldText: str, newText: str) -> Result<void, FileError> {
        // Implementation: Replace text in file
    }
```
Replace the FIRST occurrence only -- that is what separates it from `edit_all`
below. `oldText` not found is `Err(FileError::InvalidPath)`; do not silently
succeed.

```
    // edit_all edits all occurrences in a file
    fn edit_all(path: str, oldText: str, newText: str) -> Result<u32, FileError> {
        // Implementation: Replace all occurrences, return count
    }
```
The comment mandates the count, and the return type is `Result<u32, ...>` for that
reason. Zero occurrences is `Ok(0)`, not an error -- the count is the answer.

```
    // insert inserts text at a line number
    fn insert(path: str, line: u32, content: str) -> Result<void, FileError> {
        // Implementation: Insert content at line
    }
```
1-indexed, consistent with `read_range`. Inserting at one past the last line
appends; beyond that is `Err(FileError::InvalidPath)`.

```
    // delete_lines removes a line range from a file
    fn delete_lines(path: str, startLine: u32, endLine: u32) -> Result<void, FileError> {
        // Implementation: Delete line range
    }
```
Inclusive range, 1-indexed, same validation as `read_range`.

## 5. The detector, and how you are measured

Run from the repo root (`/Users/playom/t27`). Brace-matching, comment-stripping,
no marker string. This is DETECTOR-B and it is scoped to your twelve functions.

```
python3 - <<'EOF'
import re
GROUP={"read","read_range","read_binary","exists","stat",
       "write","write_binary","append","edit","edit_all","insert","delete_lines"}
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
group empty bodies: 12
still empty: append delete_lines edit edit_all exists insert read read_binary read_range stat write write_binary
```

SIGNATURE GUARD. The count and checksum below prove you did not delete a function,
add a seam declaration, or edit a signature to make a number move. Both must be
unchanged when you finish.

```
$ grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/file/operations.t27 | wc -l | tr -d ' '
33
$ grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/file/operations.t27 | shasum | cut -c1-12
005522d8d251
```

GOLD BASELINE. 457 of 506 files under `specs/` (90.3%) are clean on all three parse
signals. `file-operations-path-and-content.md` moves this file into that set;
criterion 3 keeps it there.

## Acceptance criteria

Answer each as `- N. <criterion>: met | unmet | could-not-check` in a `## VERDICT`
block. Every command runs from `/Users/playom/t27`.

- 1. DETECTOR-B prints `group empty bodies: 0` -- today `12`.
- 2. `./target/release/t27c gen specs/file/operations.t27 2>&1 | grep -c 'not yet implemented'` prints `11` -- today `33`, or `23` after the prerequisite.
- 3. PARSE-GATE prints three lines each ending in ` 0` AND DETECTOR-B prints `group empty bodies: 0` -- when you start, the parse gate ALREADY passes (path-and-content cleared it) while DETECTOR-B prints `12`, so this is unmet until you finish.
- 4. `awk '/^[[:space:]]*test "/{exit} /fs_read|fs_write|fs_stat/{n++} END{print n+0}' specs/file/operations.t27` prints 3 or more -- today `0`.
- 5. `awk '/^[[:space:]]*test "/{exit} /MAX_FILE_SIZE/{n++} END{print n+0}' specs/file/operations.t27` prints 1 or more -- today `0`.
- 6. `grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/file/operations.t27 | shasum | cut -c1-12` still prints `005522d8d251` AND DETECTOR-B prints `group empty bodies: 0` -- the checksum already matches today while DETECTOR-B prints `12`, so this is unmet until you finish.

Criterion 2 is `23 - 12 = 11`: the eleven directory-and-ignore bodies are not yours
and must still be empty. Criteria 4 and 5 stop at the first `test "` line, so they
prove the text appears in a *function body* and not merely in the test block at the
bottom of the file; 4 proves all three seam names are actually called, 5 proves the
read budget is enforced.

Criteria 3 and 6 are each a standing guard fused with DETECTOR-B, and the fusion is
the point. Measured on the unmodified file 2026-09-15: the parse gate reads
`recovery-events: 1` -- but path-and-content is what clears it, so by the time YOU
run, both guards already pass, and a bare "parse gate is clean" or "checksum is
unchanged" would be answerable `met` before you had written a line. Criterion 6 also
closes a hole DETECTOR-B cannot: DETECTOR-B counts empty bodies among twelve fixed
names, so DELETING `fn insert` would make it print `0` as surely as writing the body
would -- and adding an `fn fs_read(...)` seam declaration would change the checksum.

Criteria 2 and 3 need `./target/release/t27c` (the Sep 8 build); it is the only one
that prints parse metrics. `t27c spec-status` is NOT a criterion here, because the
eleven directory-and-ignore bodies are still empty and it will correctly keep
printing `UNWRITTEN` until all three issues are done.

## Boundary

specs/file/operations.t27
