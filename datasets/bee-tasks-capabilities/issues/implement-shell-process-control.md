# Implement the 13 empty process-control functions in specs/shell/process.t27

`.t27` is the hand-authored source language of this repo. `t27c` compiles it out
to Zig — you are writing **source**, not compiler output. Do not edit anything
under `gen/`, and do not "fix" this file by making it look like generated Zig.

## 1. Context: what this module is for

`specs/shell/process.t27` declares module `ShellProcess`. It is the process-control
capability of the corpus: spawn, wait, poll, kill, signal, and a bounded pool.
Nothing else in the corpus can start a subprocess. A `.t27` program that drives
`git`, or any publishing tool written in `.t27`, reaches the outside world through
exactly these 13 functions. Today every one of them is an empty body, so nothing
can.

The module declares one local type — a branded string:

```
    // ProcessID is a branded string representing a process identifier
    struct ProcessID(str);
```

and one local pool struct (note the colons — this is a **declaration**, colons are
correct here):

```
    // Pool represents a pool of processes
    struct Pool {
        processes: [ProcessID],
        maxSize: u32,
    }
```

and the signal enum:

```
    enum Signal {
        SIGTERM = 0,
        SIGKILL = 1,
        SIGINT = 2,
        SIGHUP = 3,
        SIGSTOP = 4,
        SIGCONT = 5,
    }
```

Everything else comes from the sibling `specs/shell/schema.t27` via `use shell::schema;`.
These are the contract the 13 functions operate on — quoted verbatim from that file:

```
    // ProcessStatus represents the status of a process
    enum ProcessStatus {
        Running = 0,
        Exited = 1,
        Signaled = 2,
        Stopped = 3,
    }

    // ProcessInfo represents information about a process
    struct ProcessInfo {
        pid: u32,               // Process ID
        command: str,           // Command that started the process
        status: ProcessStatus,   // Current status
        exitCode: i32?,         // Exit code if exited
        signal: u32?,           // Signal that terminated process
        startTime: u64,         // When the process started
        endTime: u64?,          // When the process ended
    }

    // ProcessOptions represents options for spawning a process
    struct ProcessOptions {
        cwd: str?,              // Working directory
        env: [str: str]?,       // Environment variables
        timeout: u64?,          // Timeout in milliseconds
        stdin: str?,            // Input to send to stdin
        captureOutput: bool,    // Whether to capture stdout/stderr
        abort: bool?,           // Whether abort signal should terminate
    }

    // ProcessResult represents the result of a process
    struct ProcessResult {
        exitCode: i32,          // Exit code
        stdout: str,            // Standard output
        stderr: str,            // Standard error
        success: bool,          // Whether the process succeeded
        timedOut: bool,         // Whether the process timed out
        duration: u64,          // Duration in milliseconds
    }

    // ShellError represents errors in shell operations
    enum ShellError {
        NotFound = 0,
        PermissionDenied = 1,
        Timeout = 2,
        Signal = 3,
        InvalidCommand = 4,
        ProcessError = 5,
        EnvironmentError = 6,
    }

    const SIGKILL_TIMEOUT_MS: u32 = 200;
```

`schema.t27` also already provides a helper you should reuse rather than reinvent:

```
    // is_success checks if a process result indicates success
    fn is_success(result: ProcessResult) -> bool {
        return result.exitCode == 0 && !result.timedOut;
    }
```

Style anchor for bodies in this same brace-module dialect, from the sibling
`specs/git/status.t27` lines 52-55 — a one-line delegating body, `return` plus an
expression, no Zig, no `std.`:

```
    // filter_added returns only added files
    fn filter_added(items: [Item]) -> [Item] {
        return filter_by_status(items, Kind::Added);
    }
```

The corpus idiom for a `Result` is `return Ok(x);` / `return Err(ShellError::Variant);`
(precedent: `specs/pins/parser.t27:295` `return Ok(id);` and
`specs/pins/parser.t27:411` `return Err(LexError::UnterminatedString);`).

## 2. Current state, with the commands that prove it

Every function in the file is an empty body. Note that this file does **not** use
the `// TODO: Implement from .tri spec` marker, so a marker grep is worthless here
and reads as 100% done before any work:

```
$ grep -c 'TODO: Implement' specs/shell/process.t27
0
```

The real count, by brace matching:

```
$ python3 - <<'EOF'   # the full script is in the acceptance criteria below
fn-with-braces total: 13 | empty bodies: 13
```

Codegen agrees — 13 panic stubs:

```
$ ./target/release/t27c gen specs/shell/process.t27 2>&1 | grep -c 'not yet implemented'
13
```

```
$ ./target/release/t27c gen specs/shell/process.t27 2>&1 | sed -n '38,43p'
fn spawn(command: []const u8, args: []const []const u8, opts: ProcessOptions) ShellError!ProcessID {
    _ = &command;
    _ = &args;
    _ = &opts;
    @panic("not yet implemented");
}
```

The file is also one of the 49 dirty files in `specs/` — it raises one parser
recovery event:

```
$ ./target/release/t27c parse specs/shell/process.t27 2>&1 | grep -E '^(recovery-events|recovery-at|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 1
recovery-at: Unexpected top-level token: Ident ('assert') at line 239:9
declarations-swallowed: 0
lexer-discarded-chars: 0
```

The cause is six struct-**literal** fields inside test blocks that use `:` where
every other literal in the file uses `=`. Lines 233, 234, 245, 246, 257 and 258:

```
$ sed -n '230,240p' specs/shell/process.t27
    test "process_result_zero_duration" {
        var result = ProcessResult {
            exitCode = 0,
            stdout: "quick",
            stderr: "",
            success = true,
            timedOut = false,
            duration = 0,
        };
        assert(result.duration == 0);
    }
```

I confirmed this is the sole cause by rewriting only `field: value,` to
`field = value,` on lines at or after the first `test "` line (line 113) in a
scratch copy and re-parsing:

```
$ ./target/release/t27c parse p3.t27 2>&1 | grep -E '^(recovery-events|recovery-at|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

**WARNING — do not apply that rewrite blindly.** Lines 61-62 (`processes: [ProcessID],`
and `maxSize: u32,`) are a struct **declaration**; colons are correct there, and so
are all the colons in `schema.t27`. Fix exactly the six literal fields inside the
three `process_result_*` tests.

## 3. What to write

Thirteen functions. Signatures quoted verbatim below — **keep every signature
exactly as it is**: same name, same parameter names, same types, same return type.
Where the file states its own intent in a comment, that comment is the module's
decision, already made; implement it, do not relitigate it.

Core process operations:

```
    fn spawn(command: str, args: [str], opts: ProcessOptions) -> Result<ProcessID, ShellError> {
        // Implementation: Spawn process and return ID
```
Start `command` with `args` under `opts`, return the new `ProcessID`. This is the
non-blocking one: it returns an identity, not a result. An unrunnable command is
`Err(ShellError::NotFound)`; an empty or malformed command is
`Err(ShellError::InvalidCommand)`.

```
    fn spawn_shell(command: str, opts: ProcessOptions) -> Result<ProcessResult, ShellError> {
        // Implementation: Execute command in shell and return result
```
The blocking convenience form: run `command` through a shell and return a complete
`ProcessResult`. Set `success` consistently with `is_success` from `schema.t27`;
set `timedOut` when `opts.timeout` was exceeded and return
`Err(ShellError::Timeout)` only where the spec's own tests say so — otherwise
report the timeout in the `ProcessResult` and leave the `Result` in `Ok`.
Respect `opts.captureOutput`: when false, `stdout` and `stderr` are empty strings.

```
    fn kill(pid: ProcessID, signal: str?) -> Result<void, ShellError> {
        // Implementation: Send signal to process
```
Signal by name. `signal` is optional — a null `signal` means the default
termination signal (`SIGTERM`). Unknown pid is `Err(ShellError::NotFound)`.

```
    fn kill_tree(pid: ProcessID) -> Result<void, ShellError> {
        // Implementation: Terminate process and all children
```
Terminate `pid` **and all its descendants**. `SIGKILL_TIMEOUT_MS` (200) from
`schema.t27` is the grace period between the polite signal and the hard one —
use that constant, do not hardcode 200.

```
    fn wait(pid: ProcessID) -> Result<ProcessResult, ShellError> {
        // Implementation: Wait for process and return result
```
Block until `pid` finishes; return its `ProcessResult`. This is the companion of
`spawn`: `spawn` then `wait` must be equivalent to `spawn_shell` for the same
command.

```
    fn poll(pid: ProcessID) -> Result<ProcessInfo?, ShellError> {
        // Implementation: Get current process info
```
Non-blocking. Returns `Ok(null)` when there is no such process (the `?` in the
return type is what that absence is for), `Ok(info)` otherwise, with `status`
drawn from `ProcessStatus` and `exitCode`/`signal`/`endTime` null while Running.

```
    fn is_alive(pid: ProcessID) -> Result<bool, ShellError> {
        // Implementation: Check if process is alive
```
Express this in terms of `poll` rather than duplicating its logic: alive means
`poll` yielded an info whose `status` is `ProcessStatus::Running`.

Pool operations — the pool is `maxSize`-bounded, which is the whole point of it:

```
    fn pool_create(maxSize: u32) -> Result<Pool, ShellError> {
        // Implementation: Create process pool
```
Return a `Pool` with an empty `processes` list and the given `maxSize`.

```
    fn pool_spawn(pool: Pool, command: str, args: [str], opts: ProcessOptions) -> Result<ProcessID, ShellError> {
        // Implementation: Spawn in pool with size check
```
The comment says **with size check**: refuse when `pool.processes.len` has already
reached `pool.maxSize`, and return `Err(ShellError::ProcessError)` in that case
rather than exceeding the bound. Otherwise delegate to `spawn` and record the
returned id in the pool.

```
    fn pool_wait_all(pool: Pool) -> Result<[ProcessResult], ShellError> {
        // Implementation: Wait for all pool processes
```
`wait` on every id in `pool.processes`, collecting the results in pool order.

```
    fn pool_cleanup(pool: Pool) -> Result<void, ShellError> {
        // Implementation: Clean up completed processes
```
Drop from `pool.processes` every id that is no longer running, leaving the live
ones — this is what makes room for later `pool_spawn` calls.

Signal operations:

```
    fn send_signal(pid: ProcessID, signal: Signal) -> Result<void, ShellError> {
        // Implementation: Send signal to process
```
The typed counterpart of `kill`. Takes the `Signal` enum rather than a string.

```
    fn signal_name(signal: Signal) -> str {
        // Implementation: Return signal name
```
Total mapping over the six enumerators, returning exactly `"SIGTERM"`, `"SIGKILL"`,
`"SIGINT"`, `"SIGHUP"`, `"SIGSTOP"`, `"SIGCONT"`. Note the return type is plain
`str`, not a `Result` — every input has an answer. This is the natural bridge
between `send_signal` and `kill`: `kill`'s string signal and `send_signal`'s enum
should agree through it.

Also: fix the six colon-style literal fields described in section 2, and add tests
for the behaviour you implement. The file has 17 `test` blocks today and all of
them only exercise struct literals and enum values — none calls a function. At
minimum add tests that call `signal_name` on each enumerator, and a test that
`pool_spawn` refuses a full pool.

### Do not do these

- Do **not** touch `struct ProcessID(str);`. The new `t27c` on PATH cannot parse
  that newtype form, but the failure is corpus-wide, not a defect of this file —
  `specs/tools/schema.t27`, `specs/account/schema.t27` and `specs/file/watcher.t27`
  all report `NOPARSE` on the PATH binary for the same newtype reason. Measured
  2026-09-15: `./target/release/t27c parse` accepts the newtype in all four, and
  reports `recovery-events: 2`, `0`, `1` and `1` respectively -- so those remaining
  events have other causes and only this file's is yours to clear (criterion 3).
  The newtype gap is a compiler gap and it is outside this boundary.
- For the same reason, do **not** gate your work on `t27c spec-status` or
  `t27c test-report` for this file; they are blocked by that gap. Use
  `./target/release/t27c` for every command below — it is a different, older binary
  and is the one that prints parse metrics.
- Do not use `t27c typecheck` as evidence of anything: it returns
  `{"ok":true,"errors":0}` on plain English prose.
- Never pipe the parse output through `tail -3`. On a dirty file a `recovery-at:`
  line sits between `recovery-events` and `declarations-swallowed`, and extra
  `discarded:` diagnostics print after the metrics, so `tail -3` hides the very
  number you are checking. Use the `grep -E` form given below.

## Acceptance criteria

Run every command from the repo root `/Users/playom/t27`. Each one fails today --
checked by running all six against the unmodified file on 2026-09-15.

Criterion 6 is the signature guard. A bare "signatures unchanged" check passes
before any work is done, so a bee could answer it `met` without touching the file;
it is therefore paired with criterion 1's detector and can only be answered `met`
once both hold. The checksum, not a count, is what catches a *renamed* function --
a rename keeps `fn-with-braces total: 13` intact.

- 1. The brace-matching detector below prints the whole line `fn-with-braces total: 13 | empty bodies: 0` — it prints `fn-with-braces total: 13 | empty bodies: 13` today.
- 2. `./target/release/t27c gen specs/shell/process.t27 2>&1 | grep -c 'not yet implemented'` prints `0` — it prints `13` today.
- 3. `./target/release/t27c parse specs/shell/process.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` prints exactly three lines, each ending in `: 0` — `recovery-events: 1` today.
- 4. `./target/release/t27c gen specs/shell/process.t27 2>&1 | grep -c '"SIGTERM"'` is at least `1`, proving `signal_name`'s body survived codegen — it prints `0` today.
- 5. `grep -c '^    test "' specs/shell/process.t27` prints at least `20` — it prints `17` today, and none of those 17 tests calls a function.
- 6. `grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/shell/process.t27 | shasum | cut -c1-12` still prints `1d9caee4517a` AND criterion 1's detector prints `empty bodies: 0` — the checksum matches today but the detector prints `13`, so this is unmet until you finish.

The detector for criterion 1, to run verbatim:

```
python3 - <<'EOF'
import re
t=open("specs/shell/process.t27",encoding="utf-8").read()
n=0; tot=0
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
    tot+=1
    if not b.strip(): n+=1
print("fn-with-braces total:", tot, "| empty bodies:", n)
EOF
```

Scoring note: the gold baseline for `specs/` is 457 of 506 files clean on all three
parse signals (90.3%), recomputed today. Criterion 3 moves this file from the dirty
49 into the clean 457. "100% of the corpus" is not the bar; this file is.

## Boundary

specs/shell/process.t27
