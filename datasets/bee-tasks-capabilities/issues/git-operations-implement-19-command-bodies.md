# Implement the 19 empty git command bodies in specs/git/operations.t27

`.t27` is the hand-authored source language of this repository. You are writing
source, not compiler output: `t27c gen` compiles `.t27` down to Zig under `gen/`,
and L2 in CLAUDE.md says files under `gen/` are generated and must not be edited.
Everything you write goes in the `.t27` file itself, in `.t27` syntax, with real
control flow. This is proven to survive codegen: the Lomuto quicksort in
`specs/tri/sort/quick_sort.t27` comes out of `t27c gen` as correct idiomatic Zig
with its loop, conditionals and recursion intact.

## 1. Context -- what this module is for

`specs/git/operations.t27` is the shell-out layer for git. It is the primitive
that any `.t27`-authored swarm tooling needs in order to branch, stage, commit and
sync: CLAUDE.md drives ring work off branch names ("Check ring and phase state from
branch name", "This triggers automatic branch creation for the next phase"), and
none of that is reachable while every function here is a signature with a comment
where a body should be.

The module is 175 lines, declares no types of its own, and imports everything:

```
module GitOperations {
    use base::types;
    use git::schema;
```

The contract lives in `specs/git/schema.t27`. These are the types every one of the
19 functions operates on, quoted verbatim:

```
    // Options for git command execution
    struct Options {
        cwd: str,
        env: [str: str],
    }

    // Result represents the output of a git command
    struct Result {
        exit_code: u32,
        text: str,
        stdout: [u8],
        stderr: [u8],
    }

    // GitError represents a git operation error
    struct GitError {
        message: str,
        exit_code: u32,
        command: [str],
    }

    // Base represents a git branch or reference
    struct Base {
        name: str,
        ref: str,
    }
```

and the constants every git invocation is expected to carry:

```
    const DEFAULT_BRANCH_MAIN: str = "main";
    const DEFAULT_BRANCH_MASTER: str = "master";
    const GIT_CONFIG_DEFAULT_BRANCH: str = "init.defaultBranch";

    // Git configuration flags
    const GIT_FLAGS: [str] = [
        "--no-optional-locks",
        "-c", "core.autocrlf=false",
        "-c", "core.fsmonitor=false",
        "-c", "core.longpaths=true",
        "-c", "core.symlinks=true",
        "-c", "core.quotepath=false",
    ];
```

`GIT_FLAGS` is not decoration. Two of this file's own tests assert it reaches the
command line:

```
    test "git_flags_includes_autocrlf_false" {
        // Verify core.autocrlf=false is in flags
        assert(GIT_FLAGS.contains("core.autocrlf=false"));
    }
```

Note the name collision and how it already resolves: `fn run(...) -> Result<Result,
GitError>` uses `Result` both as the generic result type and as git's own struct.
Codegen handles it -- `./target/release/t27c gen` emits
`fn run(args: []const []const u8, opts: Options) GitError!Result` with
`const Result = schema.Result;` above it. Do not rename anything to work around it.

Neighbours, for orientation only (out of boundary): `specs/git/schema.t27` is
IMPLEMENTED, `specs/git/status.t27` is PARTIAL (10 of 13 bodies empty),
`specs/git/diff.t27` is UNWRITTEN (15 of 15 empty). Those two declare their own
shell-outs rather than calling `run` here -- a grep for `GitOperations` or
`git::operations` across `specs/` matches only `specs/git/operations.t27` itself.
So nothing depends on you yet, and nothing you write here can break them.

## 2. Current state, with the commands that prove it

The file parses perfectly clean today. That is not evidence of anything:

```
$ ./target/release/t27c parse specs/git/operations.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use exactly that `grep -E`, never `| tail -3`. On a dirty file the parser prints
`  discarded: ...` diagnostics after the metrics and interleaves a `recovery-at:`
line between them, so `tail -3` both hides a non-zero `recovery-events` and shows
diagnostics instead of metrics.

Every function body is empty:

```
$ ./target/release/t27c gen specs/git/operations.t27 2>&1 | grep -c 'not yet implemented'
19
```

```
$ t27c spec-status specs/git/operations.t27
UNWRITTEN
```

THE TRAP THAT WILL OTHERWISE MAKE THIS ISSUE PASS BEFORE YOU START. Most of the
corpus marks an unwritten body with the literal line `// TODO: Implement from .tri
spec`. This file does not. It writes:

```
    // branch returns the current branch name
    fn branch(cwd: str) -> Result<str?, GitError> {
        // Implementation: Run git symbolic-ref --short HEAD
    }
```

so the usual marker grep reads as fully implemented:

```
$ grep -c 'TODO: Implement' specs/git/operations.t27
0
```

Never measure this file with a marker string. Measure empty bodies by brace
matching (the DETECTOR below) and measure codegen with
`grep -c 'not yet implemented'`, which is binary-independent: the old binary emits
`@panic("not yet implemented")` and the new one `@compileError("not yet implemented")`.

DO NOT gate on `t27c test-report`. It is BLOCKED for reasons entirely outside this
file and you cannot fix them from inside the boundary:

```
$ t27c test-report specs/git/operations.t27 2>&1 | tail -8
--- test report: specs/git/operations.t27 ---
  BLOCKED  does not compile: /var/folders/9d/5s6sn_xj6q119lz_ht66mkcm0000gn/T/t27c-test-report-operations/spec.zig:170:19: error: expected type expression, found ','
```

The cause is the map type in the sibling schema, which codegen passes through
unchanged into Zig:

```
$ t27c gen specs/git/operations.t27 2>/dev/null | sed -n '168,171p'
const Options = struct {
    cwd: []const u8,
    env: [str:str],
};
```

`env: [str: str]` at `specs/git/schema.t27:54` is the only occurrence of that type
anywhere under `specs/git/`, and it blocks `specs/git/schema.t27` identically
(`spec.zig:40:19`, same error). It is a codegen gap, not your bug. Leave it. Note
it in your report so it can be filed separately.

Also do not gate on `t27c typecheck --json` -- it returns `{"ok":true,"errors":0}`
on plain English prose, so it proves nothing.

## 3. What to write

Two tiers. **If you run out of budget, finish Tier 1 completely and leave Tier 2
untouched rather than leaving all 19 half-written.** Tier 1 is what any
`.t27`-authored swarm tool calls on its first run; Tier 2 is reporting and
housekeeping that nothing blocks on.

Each function's existing `// Implementation:` comment is the module's decision
about which git plumbing command to use. It is quoted below verbatim. Implement
that command. Do not substitute a different one and do not relitigate the choice --
if you believe one is wrong, implement it as written and say so in your report.

FIRST, TWO THINGS THAT ARE NOT HERE. The toolchain-critical set is usually named
as clone / fetch / checkout / commit / push. `fetch`, `checkout` and `commit` are
declared in this file. `clone` and `push` are **not declared anywhere under
`specs/git/`**:

```
$ grep -rn 'fn clone\|fn push\|fn stash\|fn tag' specs/git/
(exit 1)
```

Do not add them in this issue. Your scope is the 19 signatures that exist; adding
new declarations changes the module's surface and belongs in its own spec issue.
Say in your report that `clone` and `push` are missing so that issue can be filed.

### Tier 1 -- load-bearing, do these first (9 functions)

`run` is the primitive; every other function in the file is a thin wrapper over it.
Write `run` first and express the other eight in terms of it rather than
duplicating process-spawning logic.

```
    // run executes a git command with the given arguments
    fn run(args: [str], opts: Options) -> Result<Result, GitError> {
        // Implementation: Spawn git process with args, return Result
    }
```
Build the argv as `git` + `GIT_FLAGS` + `args`, run it in `opts.cwd` with
`opts.env`, and fill every field of the `Result` struct: `exit_code`, `text`,
`stdout`, `stderr`. `GIT_FLAGS` MUST be prepended here -- that is the one place it
can be, and it is what makes the two `git_flags_*` tests meaningful. On a spawn
failure (not a non-zero exit) return `GitError` with `message`, `exit_code` and the
full `command` argv. A non-zero exit is a successful `Result` with a non-zero
`exit_code`, not an error -- the file's own test says so:

```
    test "run_with_invalid_args" {
        ...
        var result = run(["invalid-command"], opts);
        // Should fail with non-zero exit code
        assert(result.is_ok());
        assert(result.unwrap().exit_code != 0);
    }
```

```
    // branch returns the current branch name
    fn branch(cwd: str) -> Result<str?, GitError> {
        // Implementation: Run git symbolic-ref --short HEAD
    }
```
Return the trimmed stdout. The return type is `str?` -- a detached HEAD makes
`symbolic-ref` exit non-zero, and that is `null`, not an error.

```
    // has_head checks if the repository has a HEAD commit
    fn has_head(cwd: str) -> Result<bool, GitError> {
        // Implementation: Run git rev-parse --verify HEAD
    }
```
Exit code 0 is `true`; non-zero is `false` (a fresh repo with no commits), not an
error.

```
    // rev_parse returns the full commit hash for a ref
    fn rev_parse(cwd: str, ref: str) -> Result<str, GitError> {
        // Implementation: Run git rev-parse ref
    }
```
Trimmed stdout. Non-zero exit here IS a `GitError` -- the return type is `str`, not
`str?`, so there is no way to say "no such ref" except by erroring.

```
    // checkout switches to a branch or commit
    fn checkout(cwd: str, target: str) -> Result<void, GitError> {
        // Implementation: Run git checkout target
    }
```

```
    // fetch retrieves changes from a remote
    fn fetch(cwd: str, remote: str?) -> Result<void, GitError> {
        // Implementation: Run git fetch [remote]
    }
```
The brackets in the comment are the contract: omit the remote argument entirely
when `remote` is null, do not pass an empty string.

```
    // pull fetches and merges changes from a remote
    fn pull(cwd: str, remote: str?, branch: str?) -> Result<void, GitError> {
        // Implementation: Run git pull [remote [branch]]
    }
```
Nested optionality: `branch` is only appended when `remote` was supplied.

```
    // commit creates a new commit
    fn commit(cwd: str, message: str) -> Result<str, GitError> {
        // Implementation: Run git commit -m message
    }
```
Returns `str` -- the new commit hash. `git commit -m` does not print a bare hash, so
follow a successful commit with `rev_parse(cwd, "HEAD")` and return that.

```
    // add stages files for commit
    fn add(cwd: str, paths: [str]) -> Result<void, GitError> {
        // Implementation: Run git add paths...
    }
```
Append every element of `paths`. Guard the empty-list case rather than invoking a
bare `git add`, which means something else.

All five `Result<void, GitError>` functions above share one shape: run the command,
return `GitError` on a non-zero exit (with `exit_code` and the argv in `command`),
return void otherwise. Write that once and reuse it.

### Tier 2 -- secondary, nothing blocks on these (10 functions)

```
    // prefix returns the git directory prefix (relative to repo root)
    fn prefix(cwd: str) -> Result<str, GitError> {
        // Implementation: Run git rev-parse --show-prefix
    }

    // default_branch returns the default branch for the repository
    fn default_branch(cwd: str) -> Result<Base?, GitError> {
        // Implementation: Check remote HEAD, then configured, then main/master
    }

    // merge_base returns the merge base commit between two refs
    fn merge_base(cwd: str, base: str, head: str?) -> Result<str?, GitError> {
        // Implementation: Run git merge-base base head
    }

    // show returns the contents of a file at a specific ref
    fn show(cwd: str, ref: str, file: str, prefix: str?) -> Result<str, GitError> {
        // Implementation: Run git show ref:file
    }

    // reset resets the repository state
    fn reset(cwd: str, mode: str, ref: str) -> Result<void, GitError> {
        // Implementation: Run git reset --mode ref
    }

    // remote_list returns all configured remotes
    fn remote_list(cwd: str) -> Result<[str], GitError> {
        // Implementation: Run git remote
    }

    // remote_url returns the URL for a remote
    fn remote_url(cwd: str, name: str) -> Result<str, GitError> {
        // Implementation: Run git remote get-url name
    }

    // branch_list returns all local branches
    fn branch_list(cwd: str) -> Result<[str], GitError> {
        // Implementation: Run git branch --format=%(refname:short)
    }

    // branch_create creates a new branch
    fn branch_create(cwd: str, name: str, start_point: str?) -> Result<void, GitError> {
        // Implementation: Run git branch name [start_point]
    }

    // branch_delete deletes a branch
    fn branch_delete(cwd: str, name: str, force: bool) -> Result<void, GitError> {
        // Implementation: Run git branch [-D|-d] name
    }
```

Three of these have contract details worth stating. `default_branch` returns
`Base?` and its comment fixes a three-step order -- remote HEAD, then the configured
`GIT_CONFIG_DEFAULT_BRANCH` (`init.defaultBranch`), then `DEFAULT_BRANCH_MAIN`
falling back to `DEFAULT_BRANCH_MASTER`; use those constants by name, do not
re-spell "main" and "master" as literals. `show` takes a `prefix: str?` parameter
that the comment's `git show ref:file` does not mention -- when it is non-null, join
it onto `file` before building the `ref:path` argument. `branch_delete` must select
`-D` when `force` is true and `-d` otherwise.

Keep the existing `// Implementation:` comments in place above or inside the bodies
you write. They are the module's documented intent and part of what makes this file
useful as training data. The DETECTOR strips comments before deciding a body is
empty, so keeping them costs you nothing.

Do not touch the eight `test` blocks at lines 116-174. They are already written and
already correct.

### DETECTOR -- the gate for criteria 1 and 2

Run from the repo root (`/Users/playom/t27`). Brace-matching, comment-stripping, no
marker string. Today it prints `empty bodies: 19` and lists all nine Tier 1 names.

```
python3 - <<'EOF'
import re
TIER1={"run","branch","checkout","fetch","pull","commit","add","rev_parse","has_head"}
t=open("specs/git/operations.t27",encoding="utf-8").read()
empty=[]
for m in re.finditer(r'(?m)^\s*(?:pub\s+)?fn\s+(\w+)\s*\(', t):
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
print("empty bodies:", len(empty))
print("tier1 still empty:", " ".join(sorted(set(empty)&TIER1)) or "(none)")
EOF
```

Verbatim output today:

```
empty bodies: 19
tier1 still empty: add branch checkout commit fetch has_head pull rev_parse run
```

NO-REGRESSION GUARD, not a criterion because it passes today. After your edit, this
must still print three zeroes:

```
$ ./target/release/t27c parse specs/git/operations.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

The gold baseline for `specs/` is 457 of 506 files (90.3%) clean on all three
signals. This file is currently one of the 457. Keep it there.

## Acceptance criteria

Answer each as `- N. <criterion>: met | unmet | could-not-check` in a `## VERDICT`
block. Every command runs from `/Users/playom/t27`.

- 1. DETECTOR prints `tier1 still empty: (none)` -- today it prints 9 names.
- 2. DETECTOR prints `empty bodies: 0` -- today it prints `empty bodies: 19`.
- 3. `./target/release/t27c gen specs/git/operations.t27 2>&1 | grep -c 'not yet implemented'` prints `0` -- today `19`.
- 4. `awk '/^[[:space:]]*test "/{exit} /GIT_FLAGS/{n++} END{print n+0}' specs/git/operations.t27` prints 1 or more -- today `0`.
- 5. `./target/release/t27c gen specs/git/operations.t27 2>&1 | grep -c 'symbolic-ref'` prints 1 or more -- today `0`.
- 6. `t27c spec-status specs/git/operations.t27` prints `IMPLEMENTED` -- today `UNWRITTEN`.

Criterion 4 proves `GIT_FLAGS` is referenced in a function body and not only in the
tests (the `awk` stops at the first `test "` line). Criterion 5 proves at least one
body carries the git plumbing command its `// Implementation:` comment names.
Criterion 6 is the one command that needs the PATH binary `t27c` (Sep 9 build);
criteria 3 and 5 need `./target/release/t27c` (Sep 8 build), which is the only one
that prints parse metrics. The two binaries are different and their capabilities
are disjoint -- use each exactly as written above.

## Boundary

specs/git/operations.t27
