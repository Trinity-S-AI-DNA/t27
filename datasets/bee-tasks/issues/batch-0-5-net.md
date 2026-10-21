# Implement parse_ip, is_localhost and is_valid_port in specs/tri/net/net.t27

## Context

`specs/tri/net/net.t27` is the `TriNet` module — the address layer under the rest of
`specs/tri/net/` (`http.t27`, `url.t27`, `channel.t27`, `async.t27` sit on top of it).
Its header claims "Supports both IPv4 and IPv6", and the two declared types are how
that claim is met: one address type, 16 bytes wide, with a flag saying how to read
them.

```
    pub const IpAddress = struct {
        is_v6 : bool,
        bytes : [16]u8,
    };

    pub const SocketAddr = struct {
        ip : IpAddress,
        port : u16,
    };
```

The existing tests pin the convention: an IPv4 address puts its four octets in
`bytes[0..4]` with `is_v6 = false` (the loopback test reads `bytes[0] == 127` and
`bytes[3] == 1`), an IPv6 address fills all sixteen with `is_v6 = true` (`::1` has
`bytes[15] == 1`). Follow that layout — do not invent an IPv4-mapped-IPv6 encoding.

**.t27 is a hand-authored source language.** You are writing source that `t27c`
compiles out to C/Rust/Verilog/Zig. Write code, not prose about code.

## Current state (measured — re-run these yourself)

```
$ cd /Users/playom/t27
$ ./target/release/t27c parse specs/tri/net/net.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0

$ ./target/release/t27c count specs/tri/net/net.t27 | head -1
net.t27: 7 node types

$ ./target/release/t27c parse specs/tri/net/net.t27 2>&1 | grep -c 'kind: FnDecl'
3

$ ./target/release/t27c parse specs/tri/net/net.t27 2>&1 | grep -c 'kind: TestBlock'
3

$ grep -c 'TODO: Implement' specs/tri/net/net.t27
3
```

The file parses clean today. All three declared functions are empty bodies holding a
single `// TODO: Implement from .tri spec` comment.

## What to write

Fill in these three functions. **Keep each signature exactly as it is** — criterion 1
greps for all three names by hand. Quoted verbatim from the file:

1. `fn parse_ip(addr: []const u8) -> ?IpAddress` — return null for anything that is
   not an address. If the text contains `:` treat it as IPv6, otherwise as IPv4. For
   IPv4: exactly four dot-separated decimal groups, each 0–255, written into
   `bytes[0..4]` with the remaining twelve bytes zero and `is_v6 = false`. For IPv6:
   colon-separated hex groups, each written big-endian as two bytes, with `::`
   expanding to the zeros needed to reach sixteen bytes, and `is_v6 = true`. Reject
   more than 4 groups (v4), more than 8 groups (v6), out-of-range values, and empty
   input — return null, do not panic.
2. `fn is_localhost(addr: IpAddress) -> bool` — true for the IPv4 loopback block
   (`bytes[0] == 127` when `!is_v6`) and for the IPv6 loopback `::1` (when `is_v6`,
   bytes 0..15 zero and `bytes[15] == 1`). False otherwise.
3. `fn is_valid_port(port: u16) -> bool` — false for 0, the wildcard, true for
   1–65535. `port` is already a `u16`, so the upper bound needs no check; say so in a
   comment rather than writing dead code.

### The tests already in the file are the specification

Do not edit or delete the three existing test blocks. These two fix the byte layout
`parse_ip` must produce:

```
    test ip_address_holds_an_ipv4_loopback
        given loopback = IpAddress{.is_v6=false,.bytes=[_]u8{127,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0}}
        then !loopback.is_v6
        and loopback.bytes[0] == 127
        and loopback.bytes[3] == 1

    test ip_address_reserves_sixteen_bytes_so_an_ipv6_address_fits
        // ::1 -- the widest address the type must carry
        given v6_loopback = IpAddress{.is_v6=true,.bytes=[_]u8{0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1}}
        then v6_loopback.is_v6
        and v6_loopback.bytes.len == 16
        and v6_loopback.bytes[15] == 1
```

One comment in the test section is now stale — it says "parse_ip(), is_localhost() and
is_valid_port() have no body yet, so they panic when called." Once you have written
the bodies that is false. Rewrite it to describe what the tests below it now check.

### Add test blocks for what you implement

House style: the proof of a function is a `test` block beside it. This file uses the
`test <name> / given / and / when / then` form (no quotes, no braces) — match it, and
add at least two new blocks per implemented function (one ordinary case, one boundary:
a rejected address, the wildcard port, the v6 loopback). This exact shape was checked
against `t27c parse` on a copy of this file and stayed at recovery-events 0:

```
    test is_valid_port_rejects_the_wildcard
        given wildcard = 0
        and highest = 65535
        when low_ok = is_valid_port(wildcard)
        and high_ok = is_valid_port(highest)
        then low_ok == false and high_ok == true
```

### Style reference

`specs/fpga/mac.t27` lines 89–111 is a parse-clean file in the same `module Name;`
dialect with real function bodies (`const x = ...;`, `if (...) { ... } else { ... }`,
`return x;`); `specs/vsa/vsa_core.t27` lines 65–79 shows `var` and `while (...) { }`
parsing clean in the same dialect. There is **no t27 interpreter**: nothing here
executes. Do not claim you ran a test.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 3 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/net/net.t27` prints 0
- 3. the real-code node count is at least 6
- 4. `t27c parse specs/tri/net/net.t27 | grep -c 'kind: TestBlock'` prints at least 6
- 5. `t27c count specs/tri/net/net.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `3 names, fns=3 nonempty=0, clean` | `3 names, fns=n nonempty=n (n>=3), clean` |
| 2 | stub marker | `3` | `0` |
| 3 | real-code nodes | `0` | `>= 6` |
| 4 | test blocks | `3` | `>= 6` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 3 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (parse_ip|is_localhost|is_valid_port)\(' specs/tri/net/net.t27
```

Must print `3`. It prints `3` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/net/net.t27 <<'EOF'
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

Today it prints `fns=3 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 3**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 3 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/net/net.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/net/net.t27
```

Today it prints `3`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/net/net.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `6`.

The bar is two nodes per stubbed function (3 x 2 = 6 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/net/net.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `3`. It must print at least `6` — the 3 blocks already in the file
plus one new block for each of the 3 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/net/net.t27 | head -1
```

Today it prints `net.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
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
- 1. all 3 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/net/net.t27 prints 0: met
- 3. the real-code node count is at least 6: met
- 4. t27c parse specs/tri/net/net.t27 | grep -c 'kind: TestBlock' prints at least 6: met
- 5. t27c count specs/tri/net/net.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/net/net.t27
