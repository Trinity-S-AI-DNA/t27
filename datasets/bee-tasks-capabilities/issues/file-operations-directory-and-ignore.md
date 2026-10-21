# Implement the 11 directory, ignore and absolute-path bodies in specs/file/operations.t27

`.t27` is the hand-authored source language of this repository. You are writing
source, not compiler output: `t27c gen` compiles `.t27` down to Zig under `gen/`,
and L2 GENERATION in CLAUDE.md says files under `gen/` are generated and must
not be edited. Everything you write
goes in the `.t27` file itself, in `.t27` syntax, with real control flow.

THIS IS THE LAST OF THREE ISSUES THAT SHARE ONE BOUNDARY FILE. The others are
`file-operations-path-and-content.md` (10 functions) and
`file-operations-read-and-write.md` (12 functions). All three edit
`specs/file/operations.t27`. **They must not run concurrently.** The order is
fixed: path-and-content, then read-and-write, then this one. Your eleven are the
remainder, so when you finish the file is fully written and `spec-status` flips --
that is criterion 3.

**THIS ISSUE HAS TWO HARD PREREQUISITES.** Both other issues must be finished and
merged first: `list_recursive` filters with your own `is_ignored`, `load_ignore_rules`
reads with `read` from read-and-write, and `absolute` resolves with `normalize` and
`resolve` from path-and-content. Check before you start:

```
$ ./target/release/t27c gen specs/file/operations.t27 2>&1 | grep -c 'not yet implemented'
11
```

If that prints anything other than `11`, a prerequisite has not run -- stop and
report which count you saw. On the untouched file it prints `33`.

## 1. Context -- what this module is for

`specs/file/operations.t27` is the file-system capability layer: 33 functions
covering reading, writing, directory traversal, ignore rules, path arithmetic and
content classification. Your eleven are the `// Directory Operations` and
`// Ignore Operations` banners, plus one straggler from `// Path Operations`.

```
module FileOperations {
    use base::types;
    use file::schema;
```

The file declares no types of its own. The contract lives in
`specs/file/schema.t27`, which is `IMPLEMENTED`. The declarations your eleven
functions operate on, quoted verbatim from that file:

```
    // FileNode represents a node in the file tree
    struct FileNode {
        name: str,             // File/directory name
        path: str,             // Relative path
        absolute: str,         // Absolute path
        fileType: FileType,    // Type of node
        ignored: bool,         // Whether node is ignored
        children: [FileNode]?, // Children (directories only)
    }

    // FileType represents the type of a file system entry
    enum FileType {
        File = 0,              // Regular file
        Directory = 1,         // Directory
        Symlink = 2,           // Symbolic link
    }

    // IgnorePattern represents a single ignore pattern
    struct IgnorePattern {
        pattern: str,          // Ignore pattern (gitignore style)
        isDir: bool,           // Whether pattern applies only to directories
    }

    // IgnoreRules represents a collection of ignore rules
    struct IgnoreRules {
        patterns: [IgnorePattern], // Ignore patterns
        whitelists: [str],    // Whitelisted paths (exceptions)
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

`FileNode` is recursive -- `children: [FileNode]?` -- and that shape is what makes
`list_recursive` a tree build rather than a flat walk. `IgnoreRules` carries
`whitelists` as well as `patterns`, so gitignore's `!pattern` negation has a
declared home and `is_ignored` must honour it. `FileError::IsDirectory` and
`FileError::NotDirectory` exist specifically for this group; nothing else in the
file can return them.

The file's own tests already pin two of these shapes. They are the module's
decision about field values and you must not change them:

```
    test "file_node_creation" {
        var node = FileNode {
            name = "dir",
            path = "dir",
            absolute = "/home/dir",
            fileType = FileType::Directory,
            ignored = false,
            children = null,
        };
        assert(node.fileType == FileType::Directory);
    }

    test "ignore_pattern_creation" {
        var pattern = IgnorePattern {
            pattern = "*.log",
            isDir = false,
        };
        assert(pattern.pattern == "*.log");
    }
```

Neighbours, for orientation only and out of boundary: `specs/file/schema.t27` is
`IMPLEMENTED`, `specs/file/watcher.t27` is `NOPARSE` (20 of 20 bodies empty).
Nothing in `specs/` imports this module -- `grep -rn 'FileOperations\|file::operations'
specs/ --include='*.t27'` matches only line 5 of the boundary file itself. So
nothing depends on you yet and nothing you write here can break another spec.

## 2. THE SEAM -- read this before writing a line

**There is no file-system primitive anywhere in `specs/`.** Measured, not assumed:

```
$ grep -rn 'std\.fs\|openFile\|readFileAlloc\|cwd()' specs/ --include='*.t27' | wc -l
0
```

`specs/tri/io/fs.t27`, `specs/tri/io/filesystem.t27`, `specs/tri/io/reader.t27` and
`specs/tri/io/writer.t27` all exist and are all `UNWRITTEN`. There is also no
notion of a working directory:

```
$ grep -rn 'cwd' specs/file/operations.t27 specs/file/schema.t27
(exit 1)
$ grep -n 'cwd\|current_dir' specs/shell/environment.t27
(no output)
```

The corpus already has a convention for this, in your own sibling file.
`specs/file/schema.t27` is marked `IMPLEMENTED` while calling a helper defined
nowhere and imported from nowhere:

```
    fn is_hidden(path: str) -> bool {
        var parts = split(path, "/");
```

Codegen emits the bare call unchanged and `spec-status` reports `IMPLEMENTED` on
body presence, not on resolution. Follow that convention exactly.

**SEAM NAMES, FIXED ACROSS ALL THREE ISSUES.** `file-operations-read-and-write.md`
fixed the first three; this issue adds five more. This is an issue-author decision
made for consistency, not a language feature. Use these names and no others:

```
    fs_read(path: str)                 -> Result<[u8], FileError>      (already in use)
    fs_write(path: str, data: [u8])    -> Result<void, FileError>      (already in use)
    fs_stat(path: str)                 -> Result<FileInfo, FileError>  (already in use)
    fs_list(path: str)                 -> Result<[FileInfo], FileError>
    fs_mkdir(path: str)                -> Result<void, FileError>
    fs_rmdir(path: str)                -> Result<void, FileError>
    fs_unlink(path: str)               -> Result<void, FileError>
    fs_rename(from: str, to: str)      -> Result<void, FileError>
    fs_cwd()                           -> Result<str, FileError>
```

Call them by bare name, exactly as `split` is called. **Do NOT add `fn` declarations
for them to this file** -- the file has 33 `fn` declarations and must still have 33
when you finish (criterion 6). Do NOT write an `extern`, an `@syscall`, an
`@import` or a `std.` call; nothing under `specs/` does. Name every seam function
you used in your report so a `specs/file/io.t27` issue can be filed for them.

Note that `copy` has no seam of its own. It is `read_binary` then `write_binary`,
both of which read-and-write already delivered. Do not add an `fs_copy`.

## 3. Current state, with the commands that prove it

Parse metrics, after the two prerequisites have run:

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
shows diagnostics instead of metrics. On the untouched file the first line reads
`recovery-events: 1`; path-and-content is the issue that clears it. Do not re-fix
it and do not let it come back.

```
$ t27c spec-status specs/file/operations.t27
UNWRITTEN
```

That stays `UNWRITTEN` through both prerequisites, because it reports on the whole
file. You are the issue that flips it.

THE TRAP THAT WILL OTHERWISE MAKE THIS ISSUE PASS BEFORE YOU START. Most of the
corpus marks an unwritten body with the literal line `// TODO: Implement from .tri
spec`. This file does not. It writes:

```
    // list lists contents of a directory
    fn list(path: str) -> Result<[FileNode], FileError> {
        // Implementation: List directory contents
    }
```

A body holding only a comment. So the usual marker grep reads as fully implemented:

```
$ grep -c 'TODO: Implement' specs/file/operations.t27
0
```

Never measure this file with a marker string. Measure empty bodies by brace
matching (DETECTOR-C below), and measure codegen with
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

Note the binary: that `sed -n '263p'` is the PATH build, whose output for this file
is 461 lines and is the same codegen `test-report` runs. `./target/release/t27c gen`
emits 246 lines for this file and prints nothing at line 263 -- do not substitute it
here. (Both builds agree on `grep -c 'not yet implemented'`, which is why the
criteria can pin the release build for that one.)

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
rather than leaving all eleven half-written. Tier 1 is what a swarm tool needs to
walk a repository; Tier 2 is mutation and cleanup.

### Tier 1 -- traversal and ignore rules (5 functions)

```
    // is_ignored checks if a path is ignored
    fn is_ignored(path: str, rules: IgnoreRules) -> bool {
        // Implementation: Check if path matches ignore rules
    }
```
Pure -- no seam. Gitignore-style glob matching over `rules.patterns`, honouring
`IgnorePattern.isDir` (a `isDir = true` pattern matches directories only) and then
`rules.whitelists`, which override a match. Note the return type is bare `bool`,
not a `Result`: an unmatchable pattern is not ignored, it is not an error.

```
    // add_ignore_pattern adds a pattern to ignore rules
    fn add_ignore_pattern(rules: IgnoreRules, pattern: str) -> IgnoreRules {
        // Implementation: Add ignore pattern
    }
```
Pure -- no seam. Takes `IgnoreRules` by value and returns a new one, so build and
return a new value rather than mutating the argument. A leading `!` makes it a
whitelist entry; a trailing `/` sets `isDir = true`. Both fields of `IgnorePattern`
must be set.

```
    // load_ignore_rules loads ignore rules from .gitignore/.ignore files
    fn load_ignore_rules(root: str) -> Result<IgnoreRules, FileError> {
        // Implementation: Load and parse ignore files
    }
```
The comment names both files: `.gitignore` and `.ignore`. Read each with `read`
(from read-and-write) via `join([root, ".gitignore"])`, skip blank lines and `#`
comments, and feed each surviving line to your own `add_ignore_pattern`. A missing
ignore file is empty rules, not an error -- use `exists` to tell "absent" from
"could not read".

```
    // list lists contents of a directory
    fn list(path: str) -> Result<[FileNode], FileError> {
        // Implementation: List directory contents
    }
```
`fs_list(path)`, then build one `FileNode` per entry with every field set: `name`,
`path` (relative, per the schema's comment), `absolute` (use your own `absolute`
below), `fileType`, `ignored`, and `children = null` -- this is the non-recursive
listing, so it does not descend. A path that is not a directory is
`Err(FileError::NotDirectory)`.

```
    // list_recursive lists directory contents recursively
    fn list_recursive(path: str, maxDepth: u32) -> Result<[FileNode], FileError> {
        // Implementation: List directory recursively
    }
```
Call `list`, then recurse into each `FileType::Directory` child, filling
`children`. Stop at `maxDepth`. Filter with `load_ignore_rules(path)` and
`is_ignored`, and set `FileNode.ignored` accordingly rather than dropping the node
silently -- the field exists to be reported.

### Tier 2 -- mutation, plus the straggler (6 functions)

```
    // create_dir creates a directory
    fn create_dir(path: str, recursive: bool) -> Result<void, FileError> {
        // Implementation: Create directory
    }
```
`fs_mkdir`. When `recursive` is true, create each missing parent in turn, walking
up with your own `dirname`. When false, a missing parent is
`Err(FileError::NotFound)`.

```
    // delete_dir deletes a directory
    fn delete_dir(path: str, recursive: bool) -> Result<void, FileError> {
        // Implementation: Delete directory
    }
```
`fs_rmdir`. When `recursive` is true, `list` the directory and delete children
first. When false, a non-empty directory is `Err(FileError::DeleteError)`.

```
    // move_path moves a file or directory
    fn move_path(from: str, to: str) -> Result<void, FileError> {
        // Implementation: Move file/directory
    }
```
`fs_rename(from, to)`. Do not fall back to copy-then-delete.

```
    // copy copies a file
    fn copy(from: str, to: str) -> Result<void, FileError> {
        // Implementation: Copy file
    }
```
The comment says file, not directory: `read_binary(from)` then
`write_binary(to, data)`. A directory argument is `Err(FileError::IsDirectory)`.

```
    // delete deletes a file
    fn delete(path: str) -> Result<void, FileError> {
        // Implementation: Delete file
    }
```
`fs_unlink`. A directory argument is `Err(FileError::IsDirectory)` -- that is what
separates it from `delete_dir`.

```
    // absolute returns absolute path
    fn absolute(path: str) -> Result<str, FileError> {
        // Implementation: Get absolute path
    }
```
This is the straggler from the `// Path Operations` banner, held back from
`file-operations-path-and-content.md` because it is the one path function that is
not pure. If `path` already starts with `"/"`, return `Ok(normalize(path))` with no
seam call at all. Otherwise `fs_cwd()` and return `Ok(resolve(cwd, path))`. Call
the `normalize` and `resolve` that path-and-content delivered; do not duplicate
them.

## 5. The detector, and how you are measured

Run from the repo root (`/Users/playom/t27`). Brace-matching, comment-stripping,
no marker string. This is DETECTOR-C and it is scoped to your eleven functions.

```
python3 - <<'EOF'
import re
GROUP={"list","list_recursive","create_dir","delete_dir","move_path","copy","delete",
       "is_ignored","load_ignore_rules","add_ignore_pattern","absolute"}
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
group empty bodies: 11
still empty: absolute add_ignore_pattern copy create_dir delete delete_dir is_ignored list list_recursive load_ignore_rules move_path
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
criterion 4 keeps it there.

## Acceptance criteria

Answer each as `- N. <criterion>: met | unmet | could-not-check` in a `## VERDICT`
block. Every command runs from `/Users/playom/t27`.

- 1. DETECTOR-C prints `group empty bodies: 0` -- today `11`.
- 2. `./target/release/t27c gen specs/file/operations.t27 2>&1 | grep -c 'not yet implemented'` prints `0` -- today `33`, or `11` after the prerequisites.
- 3. `t27c spec-status specs/file/operations.t27` prints `IMPLEMENTED` -- today `UNWRITTEN`.
- 4. PARSE-GATE prints three lines each ending in ` 0` AND DETECTOR-C prints `group empty bodies: 0` -- when you start, the parse gate ALREADY passes (path-and-content cleared it) while DETECTOR-C prints `11`, so this is unmet until you finish.
- 5. `awk '/^[[:space:]]*test "/{exit} /IgnorePattern/{n++} END{print n+0}' specs/file/operations.t27` prints 1 or more -- today `0`.
- 6. `grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/file/operations.t27 | shasum | cut -c1-12` still prints `005522d8d251` AND DETECTOR-C prints `group empty bodies: 0` -- the checksum already matches today while DETECTOR-C prints `11`, so this is unmet until you finish.

Criterion 2 is `11 - 11 = 0`: yours are the last empty bodies in the file, which is
also why criterion 3 can flip. Criterion 5 stops at the first `test "` line, so it
proves `add_ignore_pattern` constructs a real `IgnorePattern` in a *function body*
and not merely in the test block at the bottom of the file.

Criteria 4 and 6 are each a standing guard fused with DETECTOR-C, and the fusion is
the point. Measured on the unmodified file 2026-09-15: the parse gate reads
`recovery-events: 1` -- but path-and-content is what clears it, so by the time YOU
run, both guards already pass, and a bare "parse gate is clean" or "checksum is
unchanged" would be answerable `met` before you had written a line. Criterion 6 also
closes a hole DETECTOR-C cannot: DETECTOR-C counts empty bodies among eleven fixed
names, so DELETING `fn copy` would make it print `0` as surely as writing the body
would -- and adding an `fn fs_list(...)` seam declaration would change the checksum.

Criterion 3 needs the PATH binary `t27c` (the Sep 9 build); criteria 2 and 4 need
`./target/release/t27c` (the Sep 8 build), which is the only one that prints parse
metrics. The two binaries are different and their capabilities are disjoint -- use
each exactly as written above.

## Boundary

specs/file/operations.t27
