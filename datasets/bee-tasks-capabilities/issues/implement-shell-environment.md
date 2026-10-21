# Implement the 21 environment-access functions in specs/shell/environment.t27

`.t27` is the hand-authored source language of this repository. You are writing
source, not compiler output. `t27c` compiles `.t27` down to Zig under `gen/`;
never hand-edit the Zig, and never treat generated Zig as the thing to fix. The
deliverable of this issue is `.t27` text inside one file.

All commands below were run from `/Users/playom/t27` and their output is pasted
verbatim. Two `t27c` binaries exist on this machine and they are NOT
interchangeable:

    /Users/playom/t27/target/release/t27c   (Sep  8)  -- prints parse metrics, lowers Result<T,E>
    /Users/playom/.local/bin/t27c           (Sep  9)  -- has spec-status; does NOT lower Result<T,E>

Every command below names which one it means. `t27c` bare = the PATH binary.

---

## 1. Context: what this module is for

`specs/shell/environment.t27` is the environment-access capability: reading and
writing process environment variables, manipulating `PATH`, choosing a shell,
and detecting the platform. It is the module a tool calls when it must read a
value out of its own environment rather than hold that value itself.

It is a leaf today. Nothing imports it:

    $ grep -rn "ShellEnvironment" specs/ docs/ 2>/dev/null | head
    specs/shell/environment.t27:5:module ShellEnvironment {

    $ grep -rn "use shell::" specs/ 2>/dev/null | head
    specs/shell/environment.t27:7:    use shell::schema;
    specs/shell/process.t27:7:    use shell::schema;

Its whole contract comes from its sibling `specs/shell/schema.t27`, which it
imports at line 7 (`use shell::schema;`). These are the declarations the 21
functions operate on, quoted verbatim from that file:

    // specs/shell/schema.t27:13
    enum ShellType {
        Bash = 0,
        Zsh = 1,
        Fish = 2,
        Dash = 3,
        Sh = 4,
        Pwsh = 5,
        PowerShell = 6,
        Cmd = 7,
        Unknown = 8,
    }

    // specs/shell/schema.t27:26
    enum Platform {
        Darwin = 0,
        Linux = 1,
        Windows = 2,
    }

    // specs/shell/schema.t27:80
    struct ShellProperties {
        path: str,              // Full path to the shell
        name: str,              // Shell name
        shellType: ShellType,   // Type of shell
        isLogin: bool,          // Whether it's a login shell
        isPosix: bool,          // Whether it's POSIX compliant
        isBlacklisted: bool,    // Whether it's blacklisted
    }

    // specs/shell/schema.t27:94
    struct EnvVar {
        name: str,              // Variable name
        value: str,             // Variable value
    }

    // specs/shell/schema.t27:111
    enum ShellError {
        NotFound = 0,
        PermissionDenied = 1,
        Timeout = 2,
        Signal = 3,
        InvalidCommand = 4,
        ProcessError = 5,
        EnvironmentError = 6,
    }

    // specs/shell/schema.t27:104
    const SHELL_ENV_VAR: str = "SHELL";

`schema.t27` also already implements three classifier helpers that this module
imports and MUST reuse rather than reimplement -- `is_posix`, `is_login`,
`is_blacklisted`. Their bodies are written; here is one, verbatim from
`specs/shell/schema.t27:126`:

    fn is_posix(shellType: ShellType) -> bool {
        return shellType == ShellType::Bash
            || shellType == ShellType::Zsh
            || shellType == ShellType::Dash
            || shellType == ShellType::Sh;
    }

That the codegen already resolves those imports is confirmed -- the release
binary emits them as aliases:

    $ ./target/release/t27c gen specs/shell/environment.t27 2>/dev/null | sed -n '25,35p'
    const types = @import("../base/types.zig");
    const schema = @import("schema.zig");
    const EnvVar = schema.EnvVar;
    const Platform = schema.Platform;
    const ProcessStatus = schema.ProcessStatus;
    const ShellError = schema.ShellError;
    const ShellProperties = schema.ShellProperties;
    const ShellType = schema.ShellType;
    const is_blacklisted = schema.is_blacklisted;
    const is_login = schema.is_login;
    const is_posix = schema.is_posix;

The module's own `Result<T, ShellError>` return type is real and the release
binary lowers it correctly to a Zig error union:

    $ ./target/release/t27c gen specs/shell/environment.t27 2>/dev/null | grep -n 'fn get(\|fn list(\|fn get_path(\|fn platform('
    37:fn get(name: []const u8) ShellError!?[]const u8 {
    50:fn list() ShellError![]const EnvVar {
    58:fn get_path() ShellError![]const u8 {
    104:fn platform() Platform {

---

## 2. Current state: every function is declared and empty

The file parses perfectly clean today. All three parse signals are zero:

    $ ./target/release/t27c parse specs/shell/environment.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
    recovery-events: 0
    declarations-swallowed: 0
    lexer-discarded-chars: 0

Never pipe that to `tail -3`. On a dirty file the parser interleaves a
`recovery-at:` line between the metrics and prints `  discarded:` diagnostics
after them, so `tail -3` hides the very number you are checking. Use the
`grep -E` above, exactly as written.

Nothing is implemented. All 21 function bodies are empty:

    $ python3 - <<'EOF'
    import re
    t=open("specs/shell/environment.t27",encoding="utf-8").read()
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
        tot+=1
        b=re.sub(r'/\*.*?\*/','',t[i+1:j],flags=re.S); b=re.sub(r'(?m)//.*$','',b)
        if not b.strip(): n+=1
    print("fns:", tot, "empty bodies:", n)
    EOF
    fns: 21 empty bodies: 21

Call that DETECTOR A. It is quoted again in the acceptance criteria and it is
the primary gate.

**Read this before you write a single criterion check of your own.** This file
does NOT use the corpus's usual `// TODO: Implement from .tri spec` marker. It
writes a body that contains only a comment:

    fn get(name: str) -> Result<str?, ShellError> {
        // Implementation: Get environment variable value
    }

So the marker grep reports zero on a file where nothing at all is written:

    $ grep -c 'TODO: Implement' specs/shell/environment.t27
    0

Any "is it implemented" check built on a marker string PASSES BEFORE THE WORK IS
DONE on this file. Count empty bodies by brace matching, as DETECTOR A does.

Two further confirmations of the current state:

    $ t27c spec-status specs/shell/environment.t27
    UNWRITTEN

    $ ./target/release/t27c gen specs/shell/environment.t27 2>&1 | grep -c 'not yet implemented'
    21

    $ grep -cE '^[[:space:]]+return\b' specs/shell/environment.t27
    0

    $ ./target/release/t27c gen specs/shell/environment.t27 2>/dev/null | sed -n '37,40p'
    fn get(name: []const u8) ShellError!?[]const u8 {
        _ = &name;
        @panic("not yet implemented");
    }

The 18 existing test blocks test only the imported types and the constants.
Not one of them calls a function of this module:

    $ python3 - <<'EOF'
    import re
    t=open("specs/shell/environment.t27",encoding="utf-8").read()
    names=re.findall(r'(?m)^\s*fn\s+(\w+)\s*\(', t)
    blocks=re.findall(r'(?ms)^    test\s+"([^"]+)"\s*\{(.*?)^    \}', t)
    hit=[b[0] for b in blocks if any(re.search(r'\b'+n+r'\s*\(', b[1]) for n in names)]
    print("tests:", len(blocks), "tests calling a module fn:", len(hit))
    EOF
    tests: 18 tests calling a module fn: 0

Call that DETECTOR B.

---

## 3. A hard constraint found while measuring: there is no host-call mechanism

Five of these functions must read or write the real OS process environment.
`.t27` has no way to express that today. This was checked, not assumed:

    $ grep -rn "extern" specs/ | wc -l
    16
    $ grep -rnE '^[[:space:]]*(pub[[:space:]]+)?extern\b' specs/ | wc -l
    0

    All 16 hits are the substring inside ordinary words -- `externally` in a
    comment at specs/isa/ternary_graph.t27:99, `external`/`is_external` in
    specs/fpga/clock_domain.t27, `external` in prose elsewhere. Not one is an
    `extern` declaration; the anchored grep above is the one that proves it.

    $ grep -rhoE '"@[a-z_]+"' bootstrap/src/ | sort -u
    "@as"
    "@truncate"

    $ grep -rn "getenv\|std.process.getEnv\|posix.getenv" bootstrap/src/*.rs | head
    (no output)

`bootstrap/src/host/` exists but is MMIO/CSR/IRQ hardware support
(`csr_map.rs  driver.rs  irq.rs  mmio.rs  mod.rs`), not process environment.

So: `get`, `set`, `unset`, `list` and `platform` cannot reach the OS from pure
`.t27` as the language stands. The boundary of this issue is one file, so you
may not add a compiler intrinsic to fix that.

What this means for your work:

- Implement in full every function whose logic is pure or is expressible in
  terms of the other functions in this module. That is the large majority.
- For those five primitives, implement everything that IS expressible: argument
  validation, the documented error returns, the documented null/empty cases, the
  structure of the answer.
- If the final host read genuinely cannot be expressed, say so. Report it in
  your `## VERDICT` as `unmet` or `could-not-check` with the reason. Do NOT
  write a body that returns an invented value dressed up as a real one. An
  honest `unmet` on criterion 1 or 3 is the correct outcome in that case and
  this issue anticipates it. A faked body is not.

---

## 4. What to write

21 functions, in file order. Each signature below is quoted verbatim from
`specs/shell/environment.t27`, together with the intent comment the file already
carries. Those comments are the module's decision, not yours to re-litigate --
implement what they say.

### Environment operations

`specs/shell/environment.t27:14`

    fn get(name: str) -> Result<str?, ShellError> {
        // Implementation: Get environment variable value

Look up `name`. Absent variable is not an error -- the return is `str?`, so
absent is a null value inside `Ok`. Reserve `ShellError::EnvironmentError` for a
genuine failure. An empty or malformed `name` is `ShellError::InvalidCommand`.

`specs/shell/environment.t27:19`

    fn set(name: str, value: str) -> Result<void, ShellError> {
        // Implementation: Set environment variable

`specs/shell/environment.t27:24`

    fn unset(name: str) -> Result<void, ShellError> {
        // Implementation: Remove environment variable

Unsetting a variable that is not set succeeds; it is not `NotFound`.

`specs/shell/environment.t27:29`

    fn list() -> Result<[EnvVar], ShellError> {
        // Implementation: Get all environment variables

Returns `EnvVar` records -- the struct is quoted in section 1, fields `name` and
`value`, both `str`.

`specs/shell/environment.t27:34`

    fn expand(input: str) -> Result<str, ShellError> {
        // Implementation: Expand $VAR and ${VAR} patterns

Both forms named in that comment are required: bare `$VAR` and braced `${VAR}`.
This is pure string scanning over `input` plus a call to `get` per name found.
Decide and comment what an undefined name expands to, and cover it with a test.

### Path operations

`const PATH: str = "PATH";` is already declared at line 43. Use it; do not
repeat the literal `"PATH"` in your bodies.

`specs/shell/environment.t27:46`

    fn get_path() -> Result<str, ShellError> {
        // Implementation: Get PATH value

`specs/shell/environment.t27:51`

    fn set_path(value: str) -> Result<void, ShellError> {
        // Implementation: Set PATH value

`specs/shell/environment.t27:56`

    fn path_append(directory: str, position: str?) -> Result<void, ShellError> {
        // Implementation: Append to PATH (position: "front" or "back")

The comment fixes the vocabulary: `position` is `"front"` or `"back"`. It is
optional (`str?`) -- pick the default the comment implies by the function's name
and state it in a comment. A `position` that is neither string is
`ShellError::InvalidCommand`.

`specs/shell/environment.t27:61`

    fn path_remove(directory: str) -> Result<void, ShellError> {
        // Implementation: Remove from PATH

`specs/shell/environment.t27:66`

    fn path_list() -> Result<[str], ShellError> {
        // Implementation: Split PATH into directories

`specs/shell/environment.t27:71`

    fn path_which(executable: str) -> Result<str?, ShellError> {
        // Implementation: Find executable in PATH

Not found is a null value inside `Ok`, not `ShellError::NotFound` -- the return
type is `str?`. Build on `path_list`.

### Shell selection

`specs/shell/environment.t27:80`

    fn detect() -> Result<ShellProperties, ShellError> {
        // Implementation: Detect current shell from env

"from env" means the `SHELL` variable. `schema.t27:104` already declares
`const SHELL_ENV_VAR: str = "SHELL";` -- use that constant. Turning the path
into a populated `ShellProperties` is pure work you can write today: derive
`name` from the path, map `name` to `ShellType`, then fill `isPosix`, `isLogin`
and `isBlacklisted` by calling the imported `is_posix`, `is_login` and
`is_blacklisted`. An unrecognised shell is `ShellType::Unknown`, not an error.

`specs/shell/environment.t27:85`

    fn find(name: str) -> Result<ShellProperties?, ShellError> {
        // Implementation: Search for shell in system

`specs/shell/environment.t27:90`

    fn select(preferred: str?) -> Result<ShellProperties, ShellError> {
        // Implementation: Select shell, preferring preferred

The comment fixes the precedence: try `preferred` first, then fall back. Compose
`find`, `detect` and `fallback`; write no new search logic here.

`specs/shell/environment.t27:95`

    fn fallback(platform: Platform) -> Result<ShellProperties, ShellError> {
        // Implementation: Return platform-specific fallback

Fully pure and fully writable today: `Platform` in, `ShellProperties` out, one
arm per enum member (`Darwin`, `Linux`, `Windows` -- the enum is quoted in
section 1). Fill every field of the struct.

### Home and user

`const HOME: str = "HOME";` (line 104) and `const USER: str = "USER";`
(line 112) are already declared. Use them.

`specs/shell/environment.t27:107`

    fn get_home() -> Result<str, ShellError> {
        // Implementation: Get HOME directory

`specs/shell/environment.t27:115`

    fn get_user() -> Result<str, ShellError> {
        // Implementation: Get current user name

Both return a non-optional `str`, so unlike `get` an absent variable here IS an
error -- `ShellError::NotFound`.

### Platform detection

`specs/shell/environment.t27:124`

    fn platform() -> Platform {
        // Implementation: Detect current platform

Note the return type: bare `Platform`, no `Result`. This function cannot fail.
Unknown platform still has to answer with a `Platform` member.

`specs/shell/environment.t27:129`

    fn is_windows() -> bool {
        // Implementation: Check for Windows

`specs/shell/environment.t27:134`

    fn is_darwin() -> bool {
        // Implementation: Check for macOS

`specs/shell/environment.t27:139`

    fn is_linux() -> bool {
        // Implementation: Check for Linux

All three are one line each in terms of `platform()` -- compare against the
`Platform` member. Do not re-detect inside them.

### Tests

Law L4 requires this spec to keep its tests, and the 18 it has exercise only
types (DETECTOR B: `tests calling a module fn: 0`). Add behavioural test blocks
that actually call what you wrote. At minimum cover: `expand` on both `$VAR` and
`${VAR}`; `path_append` with `"front"` and with `"back"`; `path_remove` on a
directory that is absent; `path_which` returning the null case; `fallback` for
each of the three `Platform` members; `select` preferring `preferred`.

**Keep the existing `test "name" { ... }` brace form.** The PATH binary's parser
rejects the braceless `test name / given / when / then` form used elsewhere in
the corpus, and it rejects `{ "k": "v" }` map literals. Both were confirmed:

    $ t27c parse specs/tri/sort/quick_sort.t27 2>&1 | head -1
    Error: Parse error: Expected LBrace, got KwTest ('test') at line 37:5

    $ t27c parse specs/shell/schema.t27 2>&1 | head -1
    Error: Parse error: parse error in fn 'process_options_full' near line 235: Unexpected token in expression: LBrace ('{') at line 235:31

Introducing either construct into `environment.t27` flips `t27c spec-status` to
`NOPARSE` and criterion 3 fails through no fault of your bodies.

### Do not chase `t27c test-report` -- it is blocked for a reason outside this file

    $ t27c test-report specs/shell/environment.t27 2>&1 | tail -6
    --- test report: specs/shell/environment.t27 ---
      BLOCKED  does not compile: /var/folders/9d/5s6sn_xj6q119lz_ht66mkcm0000gn/T/t27c-test-report-environment/spec.zig:20:26: error: use of undeclared identifier 'Result'

That is the PATH binary failing to lower `Result<T, E>` at all. It has nothing
to do with function bodies. Proven on a two-line scratch file outside the
corpus:

    module R1 {
        enum E { A = 0, B = 1, }
        fn f(x: str) -> Result<str, E> {
            return Ok(x);
        }
        test "t" { assert(1 == 1); }
    }

    $ t27c gen r1.t27 2>&1 | grep -n 'fn f'
    22:fn f(x: []const u8) Result {
    $ ./target/release/t27c gen r1.t27 2>&1 | grep -n 'fn f'
    12:fn f(x: []const u8) E![]const u8 {

The body is written and `Result` is still emitted bare by the PATH binary.
`test-report` will stay `BLOCKED` after you have finished, and it is therefore
NOT an acceptance criterion. Do not change the module's return types to get
around it -- those signatures are the contract.

---

## Acceptance criteria

Run each command from `/Users/playom/t27`. Three traps, already proven above:
do not swap DETECTOR A for a marker grep (`grep -c 'TODO: Implement'` prints `0`
today on an entirely unwritten file); do not use the unanchored `grep -c 'return'`
in criterion 4 (it prints `7` today by matching the word "returns" in comments);
never pipe the parse metrics to `tail -3`. `t27c test-report` is BLOCKED for a
compiler reason outside this file and is deliberately not a criterion here.

All six were run against the unmodified file on 2026-09-15 and all six fail.
Criterion 6 carries two standing guards -- the file already parses clean and
already declares 21 functions -- so on its own it would be answerable `met`
before any work is done. It is paired with DETECTOR A's body count for exactly
that reason: `fns: 21` alone is not evidence of work.

- 1. DETECTOR A prints `fns: 21 empty bodies: 0`. Today: `fns: 21 empty bodies: 21`.
- 2. `./target/release/t27c gen specs/shell/environment.t27 2>&1 | grep -c 'not yet implemented'` prints `0`. Today: `21`.
- 3. `t27c spec-status specs/shell/environment.t27` prints `IMPLEMENTED`. Today: `UNWRITTEN`.
- 4. `grep -cE '^[[:space:]]+return\b' specs/shell/environment.t27` prints at least `21`. Today: `0`.
- 5. DETECTOR B reports at least `8` for `tests calling a module fn`. Today: `0`.
- 6. The section-2 parse-metrics grep still prints three lines each ending in ` 0` AND DETECTOR A prints `fns: 21 empty bodies: 0`. Today the parse metrics already print three zeros while DETECTOR A prints `empty bodies: 21`, so this is unmet until you finish.

Answer each in a `## VERDICT` block as `- N. <criterion>: met | unmet | could-not-check`. If the missing host-call mechanism described in section 3 blocks criteria 1 or 3, report them honestly rather than writing a body that invents a value.

## Boundary

specs/shell/environment.t27
