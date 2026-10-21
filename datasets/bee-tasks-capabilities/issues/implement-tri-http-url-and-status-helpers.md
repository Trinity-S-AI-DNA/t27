# Write the seven empty bodies in specs/tri/net/http.t27 — URL parsing and HTTP status classification

`.t27` is the hand-authored source language of this project. You are writing `.t27`,
and `t27c` compiles it out to Zig. You are writing source, not compiler output. Do not
edit anything under `gen/`; do not paste generated Zig into the spec. Write the bodies in
`.t27` and let `t27c gen` produce the Zig.

## 1. Context — what this module is for

`specs/tri/net/http.t27` declares `module TriHttp;` (line 5). It is the HTTP
**vocabulary** module: the method verbs, the status-code classification, and the
URL value type. Its three type declarations are the contract, and they are already
written — quoted verbatim from lines 13-29:

```
    pub const HttpMethod = struct {
        enum : [GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS],
    };

    pub const HttpStatus = struct {
        code : u16,
        reason : []const u8,
    };

    pub const Url = struct {
        scheme : ?[]const u8,
        host : ?[]const u8,
        port : ?u16,
        path : []const u8,
        query : ?[]const u8,
        fragment : ?[]const u8,
    };
```

Three decisions are already made by those declarations and by the module's own tests,
and they are not yours to relitigate:

- `reason` is `[]const u8`, not an owned buffer. The file's test at line 82 says so
  in words: "the reason is borrowed, not owned -- status_from_code can hand back a
  literal." Return string literals. Do not allocate reason phrases.
- `HttpMethod` has exactly seven verbs, in this order. The comment at lines 120-127
  records that an earlier version declared an empty variant list, that this read as
  green under a weaker assertion, and that the seven verbs were recovered from the
  upstream spec `trinity-fpga tri_http.tri:7`. `t27c gen` emits this as a real Zig
  `enum` with variants `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `OPTIONS`
  (uppercase — switch on `.GET`, not `.get`).
- Every `Url` field but `path` is optional. The test at line 101 is named
  `every_url_component_but_the_path_is_optional`, and its comment reads: "A bare
  "/index.html" is a legal Url: no scheme, host, port, query or fragment."

### Honest scope: this module is helpers, not a client

Filling these seven bodies does **not** give the toolchain the ability to fetch
anything. There is no transport here and none anywhere else in `specs/`. Verified:

```
$ grep -rnE '^\s*(pub )?fn (connect|send|recv|listen|accept|request|fetch|get_url|post)\s*\(' specs/
specs/tri/net/channel.t27:29:    fn send(channel: Channel(T), value: T) -> bool {
specs/tri/net/channel.t27:34:    fn recv(channel: Channel(T)) -> Maybe(T) {
specs/tri/io/writer.t27:33:    fn listen(writer: Writer(W, T)) -> Writer(W, struct { T, W }) {
specs/git/operations.t27:59:    fn fetch(cwd: str, remote: str?) -> Result<void, GitError> {
```

`channel.t27`'s `send`/`recv` are in-process channels. `writer.t27`'s `listen` is a
stream combinator. `git/operations.t27`'s `fetch` shells out to git. None is a socket.

After this work, an agent written in `.t27` still cannot make an HTTP request. Still
missing, and out of scope for this issue:

- a socket layer — no TCP connect/read/write exists in `specs/` at all;
- TLS, so no `https:` even once sockets exist;
- DNS resolution — `specs/tri/net/net.t27` declares `parse_ip`, `is_localhost`,
  `is_valid_port` and all three are empty (`fn-with-braces: 3 | empty bodies: 3`);
- request/response wire serialization — no header map type, no request-line writer,
  no chunked-body reader, no `HttpRequest`/`HttpResponse` in this module;
- URL *re*-construction and percent-coding — `specs/tri/net/url.t27` declares
  `parse`, `encode`, `decode`, `to_string` and all four are empty
  (`fn-with-braces: 4 | empty bodies: 4`).

What it *does* buy: every caller that already has bytes from somewhere can classify a
status and take a URL apart. That is the prerequisite for the rest, and it is worth
doing on its own.

### A working exemplar in this repo

`specs/server/http.t27` is a separate, server-side module that has already written
the same two ideas. Its bodies, quoted verbatim from lines 186-197 and 225-232, are
the idiom to follow:

```
pub fn status_is_success(status: HttpStatus) bool {
    return status.code >= 200 and status.code < 300;
}

pub fn method_to_string(method: HttpMethod) []u8 {
    return switch (method) {
        .get => METHOD_GET,
        .post => METHOD_POST,
        .put => METHOD_PUT,
        .delete => METHOD_DELETE,
    };
}
```

Note that module has four verbs and lowercase variants; `TriHttp` has seven and
uppercase. Follow the shape, not the names. `specs/base/debounce.t27` is another
fully-written spec (`empty bodies: 0`, `t27c spec-status` says `IMPLEMENTED`,
`t27c test-report` says `tests 3 / pass 3 / FAIL 0`) if you want a second reference.

## 2. Current state, with the command that proves it

Every one of the seven functions is a signature with a comment where a body belongs.
Lines 36-38, verbatim:

```
    // method_to_string(method: HttpMethod) → []const u8
    fn method_to_string(method: HttpMethod) -> []const u8 {
        // TODO: Implement from .tri spec
    }
```

Measured 2026-09-15, pasted verbatim:

```
$ python3 - <<'EOF'
import re
t=open("specs/tri/net/http.t27",encoding="utf-8").read()
n=0
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
    if not b.strip(): n+=1
print("empty bodies:", n)
EOF
empty bodies: 7

$ ./target/release/t27c gen specs/tri/net/http.t27 2>&1 | grep -c 'not yet implemented'
7

$ t27c spec-status specs/tri/net/http.t27
UNWRITTEN

$ ./target/release/t27c parse specs/tri/net/http.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ t27c test-report specs/tri/net/http.t27 2>&1 | tail -8
--- test report: specs/tri/net/http.t27 ---

  tests       5
  pass        5
  FAIL        0
  rate    100.0%
```

Read those last two blocks carefully, because they are the trap on this file:

- **The file already parses clean.** All three parse signals are 0 today. Keeping them
  at 0 is a no-regression duty, not evidence of work. Note also: never check these with
  `| tail -3` — on a dirty file a `recovery-at:` line is interleaved between
  `recovery-events` and `declarations-swallowed`, and `tail -3` hides the non-zero
  count. Always use the `grep -E` above.
- **The test suite already reports 100%.** It passes because all five tests are about
  the *declarations* — `@typeInfo`, `@FieldType`, and arithmetic on integer literals.
  The file says this itself at lines 74-77: "Every function above is an unimplemented
  stub: each one emits `@panic("not yet implemented")`, so calling any of them aborts
  the test binary rather than failing a test." Not one test calls a function under test.
  A green `test-report` on this file today means nothing about the implementation.
- **`t27c typecheck` is not evidence.** `t27c typecheck --json` returns
  `{"ok":true,"errors":0}` on plain English prose. Do not cite it.

One more trap, for the record: this file *does* use the literal `// TODO: Implement`
marker (`grep -c 'TODO: Implement' specs/tri/net/http.t27` → `7`), but four of its
sibling capability specs do not — they write `// Implementation: ...` instead, so a
marker grep reads them as finished. Every implementation check below counts **empty
bodies by brace matching**, never a marker string.

Note the two `t27c` binaries differ. `./target/release/t27c` (Sep 8) prints the parse
metrics; bare `t27c` on PATH (Sep 9) prints none but has `spec-status` and
`test-report`, which the release binary lacks. Use each exactly as written below.

## 3. What to write

Seven functions. Signatures quoted verbatim from the file — **do not change any
signature**; the tests at lines 79-131 assert against these types.

### `fn method_to_string(method: HttpMethod) -> []const u8` (line 36)

File's own header comment, line 35: `// method_to_string(method: HttpMethod) → []const u8`.
Switch over all seven variants and return the uppercase RFC 9110 token for each:
`.GET` → `"GET"`, `.POST` → `"POST"`, `.PUT` → `"PUT"`, `.DELETE` → `"DELETE"`,
`.PATCH` → `"PATCH"`, `.HEAD` → `"HEAD"`, `.OPTIONS` → `"OPTIONS"`. No `else` arm —
an exhaustive switch is what makes adding an eighth verb a compile error rather than
a silent wrong answer.

### `fn status_from_code(code: u16) -> HttpStatus` (line 41)

Return `HttpStatus{ .code = code, .reason = <literal> }`. The `reason` is borrowed
(line 82), so every arm returns a string literal — allocate nothing. Cover at minimum
the common RFC 9110 phrases: 200 OK, 201 Created, 204 No Content, 301 Moved
Permanently, 302 Found, 304 Not Modified, 400 Bad Request, 401 Unauthorized,
403 Forbidden, 404 Not Found, 405 Method Not Allowed, 409 Conflict, 429 Too Many
Requests, 500 Internal Server Error, 502 Bad Gateway, 503 Service Unavailable,
504 Gateway Timeout. Unknown codes must still return a value, not panic — fall back to
the class phrase (`"Informational"` / `"OK"` / `"Redirection"` / `"Client Error"` /
`"Server Error"`) and `"Unknown"` outside 100..599. The signature returns `HttpStatus`,
not `!HttpStatus`: there is no error path to take.

### The four class predicates (lines 46, 51, 56, 61)

```
    fn is_success(code: u16) -> bool {
    fn is_redirect(code: u16) -> bool {
    fn is_client_error(code: u16) -> bool {
    fn is_server_error(code: u16) -> bool {
```

Half-open hundreds-blocks: `>= 200 and < 300`, `>= 300 and < 400`, `>= 400 and < 500`,
`>= 500 and < 600`. The file's test at lines 88-91 fixes this: "four disjoint
hundreds-blocks, so a code belongs to at most one of them and 200/301/404/500 land
apart." 299 is success and 300 is not; 199 and 600 belong to none of the four. Mirror
`specs/server/http.t27:186` exactly.

### `fn parse_url(allocator: std.mem.Allocator, url: []const u8) -> !Url` (line 66)

The one function with an error union, and the one with an allocator. Split, in order:

1. `scheme` — text before `"://"`, if present; otherwise `null`.
2. authority — up to the first `/`, `?` or `#` after the scheme. Inside it, split
   `host` from `port` at the last `:`. Parse the port with `std.fmt.parseInt(u16, ...)`;
   a port that does not parse, or overflows `u16`, is the error case this `!` exists
   for. With no authority present, `host` and `port` are both `null`.
3. `path` — from the authority's end up to `?` or `#`. This field is **not** optional:
   when there is no path component, use `"/"`. A relative reference like
   `"/index.html"` has a path and nothing else, per the test at line 112.
4. `query` — between `?` and `#`, exclusive of both, or `null`.
5. `fragment` — everything after the first `#`, or `null`.

`std.mem.indexOf` / `std.mem.indexOfScalar` / `std.mem.lastIndexOfScalar` are the tools;
they are used elsewhere in the corpus (`specs/tri/crypto/base64.t27`,
`specs/config/paths.t27`, `specs/server/sse.t27`). Prefer returning **subslices of the
input** — the `Url` fields are all `[]const u8`, so borrowing costs nothing and leaves
the caller owning the one buffer it already had. If you do allocate, say so in a comment
on the function and be consistent across all fields. No percent-decoding here: that is
`specs/tri/net/url.t27`'s `decode`, which is a separate empty body and a separate task.

### Tests

The five existing tests assert on declarations only and must keep passing. Add tests
that actually **call** the seven functions, now that calling them no longer panics — at
minimum: `method_to_string(.GET) == "GET"` and one more verb; `status_from_code(404)`
gives code 404 and reason `"Not Found"`; the four predicates on 200/301/404/500 plus
the boundary pair 299/300; `parse_url` on a full
`"https://example.com:8443/a/b?x=1#frag"` and on the bare `"/index.html"` from line 115.
Use `std.mem.eql(u8, ...)` for string comparison — `==` on slices compares pointers.

## Acceptance criteria

- 1. The brace-matching detector in section 2, run verbatim on `specs/tri/net/http.t27`, prints `empty bodies: 0` (today: `empty bodies: 7`).
- 2. `./target/release/t27c parse specs/tri/net/http.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` still prints all three as `0`, and `./target/release/t27c gen specs/tri/net/http.t27 2>&1 | grep -c 'not yet implemented'` prints `0` (today: `7`).
- 3. `t27c spec-status specs/tri/net/http.t27` prints `IMPLEMENTED` (today: `UNWRITTEN`).
- 4. `./target/release/t27c gen specs/tri/net/http.t27 2>&1 | grep -c '"Not Found"'` prints at least `1` (today: `0`).
- 5. `t27c test-report specs/tri/net/http.t27 2>&1 | grep -E '^\s+(tests|pass|FAIL)'` shows `tests` at least `9`, `pass` equal to `tests`, and `FAIL` `0` (today: `tests 5`, `pass 5`, `FAIL 0`).

All five fail today. Signature preservation is not a criterion because
`grep -cE '^\s*fn (method_to_string|status_from_code|is_success|is_redirect|is_client_error|is_server_error|parse_url)\(' specs/tri/net/http.t27`
already prints `7` — it is a duty stated in section 3, and criterion 5 catches a
broken signature anyway, since the existing `@FieldType` tests would stop passing.

## Boundary

specs/tri/net/http.t27
