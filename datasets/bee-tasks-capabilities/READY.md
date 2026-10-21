# READY.md — 8 capability issues, audited and repaired

Audited 2026-09-15 against `/Users/playom/t27`, on a working tree in which **not one
of the six target `.t27` files has been modified**. Two binaries were used and they
are not interchangeable:

| binary | built | what only it can do |
|---|---|---|
| `./target/release/t27c` | 2026-09-08 14:59 | prints the three parse metrics; `gen` emits `@panic("not yet implemented")` |
| `t27c` (on PATH, `~/.local/bin/t27c`) | 2026-09-09 00:11 | `spec-status`, `test-report`, `check-calls`; `gen` emits `@compileError("not yet implemented")` |

**Every one of the 47 acceptance criteria in these 8 issues was executed against the
corpus in its current state, and every one of them failed.** That is the bar. A
criterion that already passes before any work is done is decoration: a bee can
honestly answer it `met` without touching the file.

## Audit result

| | count |
|---|---|
| issues drafted | 8 |
| issues surviving | **8** |
| issues deleted | 0 |
| criteria before the audit | 47 |
| criteria that already passed in the current state | **7 (15%)** |
| criteria after the audit | **47** (7 rewritten, 0 deleted) |
| criteria that fail in the current state | **47 (100%)** |
| boundary sections parsing to exactly the intended path | **8 of 8** |
| boundary collisions | 1 path shared by 3 issues — declared and ordered in all 3 |
| quoted `$` commands in the 8 issues | 84 (77 runnable) |
| quoted outputs re-run mechanically | **77 of 77** |
| quoted outputs that did not reproduce | **9** (all repaired) |
| empty function bodies these 8 issues cover | 96 |

The previous batch of 36 issues had 89 of 199 criteria (45%) passing before any work.
This batch is at 15%, and the seven were all the same species: **a no-regression guard
presented as a criterion**. None of the 8 issues had to be deleted — every one of the
six target files contains real, bounded, measured, unimplemented work.

## The seven decoration criteria, and what replaced them

A guard is by nature a no-change check, so it can never fail today on its own. Each
was therefore **fused** with the one signal in its own issue that does fail today,
producing a criterion that is still one line, still answerable as exactly
`met | unmet | could-not-check`, and now unanswerable-as-met before the work.

| issue | was | why it was free | now |
|---|---|---|---|
| `file-operations-path-and-content.md` | 6. signature checksum still `005522d8d251` | prints `005522d8d251` today | `…AND DETECTOR-A prints group empty bodies: 0` |
| `file-operations-read-and-write.md` | 3. PARSE-GATE three zeros | path-and-content clears `recovery-events: 1` **before this issue starts** | `…AND DETECTOR-B prints group empty bodies: 0` |
| `file-operations-read-and-write.md` | 6. signature checksum | prints `005522d8d251` today | `…AND DETECTOR-B prints group empty bodies: 0` |
| `file-operations-directory-and-ignore.md` | 4. PARSE-GATE three zeros | same — already clean two issues earlier | `…AND DETECTOR-C prints group empty bodies: 0` |
| `file-operations-directory-and-ignore.md` | 6. signature checksum | prints `005522d8d251` today | `…AND DETECTOR-C prints group empty bodies: 0` |
| `implement-shell-environment.md` | 6. "parse-metrics grep prints three zeros and DETECTOR A still prints `fns: 21`. Paste both." | both halves already true; also not answerable as one verdict | `…AND DETECTOR A prints fns: 21 empty bodies: 0` |
| `implement-shell-process-control.md` | 6. `grep -c '^    fn '` still prints `13` (the draft admitted it "passes today") | prints `13` today | checksum `1d9caee4517a` `…AND criterion 1's detector prints empty bodies: 0` |

Two of those deserve naming, because they are traps a later author will step in again:

- **The chained PARSE-GATE.** `specs/file/operations.t27` raises exactly one recovery
  event, and it is `whitelists: [],` at line 297 — a colon in a struct *literal*,
  inside a test block, nothing to do with any function body. Verified by copying the
  file, changing that one token to `whitelists = [],` and re-parsing: `0 0 0`.
  `file-operations-path-and-content.md` is the issue that fixes it, so for the two
  issues that run *after* it the parse gate is already green on arrival. The issue
  texts said `recovery-events: 1` "on the untouched file", which is true and
  irrelevant: the bee never sees the untouched file.
- **Deleting a function satisfies a scoped detector.** DETECTOR-A/B/C count empty
  bodies among a *fixed set of names*. Delete `fn truncate` and the detector prints
  `group empty bodies: 0` exactly as writing the body would. The signature checksum is
  the only thing in the issue that catches it, which is why it was fused rather than
  demoted to prose.

## The nine quoted outputs that did not reproduce

Every `$ `-prefixed command in all 8 issues was extracted and executed, and its output
compared against the block quoted beneath it — 84 commands, 77 of them runnable (the
other 7 are heredocs and named scratch files). A fabricated output teaches a bee to
fabricate, so all nine misses were repaired at the source.

1. **All three `file-operations-*` issues** quoted
   `$ ./target/release/t27c gen specs/file/operations.t27 2>/dev/null | sed -n '263p'`
   returning the mangled `WatcherHandle` line. It returns **nothing**: that build emits
   246 lines for this file. The 461-line output with the defect at line 263 is the PATH
   build — the same codegen `test-report` runs, which is why its error says
   `spec.zig:263:86`. Corrected to `t27c gen` in all three, each with a note on why the
   binary matters here. (Three of the nine.)
2. **`implement-shell-environment.md`** glossed `grep -rn "extern" specs/` as
   "the only hits are the word 'externally' … ternary_graph.t27:99". It returns **16**
   hits across five files (`external`, `is_external`, prose in a Markdown report).
   The substantive claim survives and is now proven by an anchored grep:
   `grep -rnE '^[[:space:]]*(pub[[:space:]]+)?extern\b' specs/ | wc -l` → `0`.
3. **`implement-shell-environment.md`** quoted `sed -n '26,35p'` of the generated Zig
   preamble; the block it shows is eleven lines starting one line earlier. Corrected to
   `sed -n '25,35p'`.
4. **`implement-shell-environment.md`** quoted `t27c test-report … | tail -4` showing the
   `--- test report: … ---` header. `tail -4` does not reach the header. Corrected to
   `tail -6`.
5. **`implement-shell-process-control.md`** quoted `sed -n '37,42p'` of the generated
   Zig; the real range is `38,43` — at 37 the window opens on a stray `}` and loses the
   closing brace. Corrected.
6. **`implement-shell-process-control.md`** quoted `$ python3 detect.py …`. There is no
   `detect.py` in this repository and creating one would be a Python tool in a corpus
   whose whole point is to stop writing Python tools. The *output* is real — it is what
   the heredoc detector in the acceptance criteria prints — so the invocation now points
   at that heredoc.

7. **`implement-shell-process-control.md`** asserted that `specs/tools/schema.t27`,
   `specs/account/schema.t27` and `specs/file/watcher.t27` "parse cleanly on
   `./target/release/t27c`". False for two of the three: `recovery-events: 2` and `1`.
   Replaced with the four measured numbers and a narrower claim — that build accepts the
   `struct ProcessID(str);` newtype, which is the point being made.

The six commands that still "mismatch" a mechanical re-run are all correct as written:
four are the *post-prerequisite* start states in issues 2 and 3 of the file-operations
chain, each labelled "after path-and-content has run"; two run against `r1.t27`, a
seven-line scratch module printed verbatim beside them. `r1.t27` was recreated from that
listing and both quoted outputs reproduced exactly — `22:fn f(x: []const u8) Result {`
from the PATH build and `12:fn f(x: []const u8) E![]const u8 {` from the release build.

## Everything that was re-measured and did reproduce

Spot-checked 41 quoted commands; 37 matched byte for byte, including every number that
gates a criterion. The load-bearing ones:

```
$ find specs -name '*.t27' | wc -l                       506
gold baseline, recomputed file by file                   457 clean / 49 dirty  (90.3%)
empty bodies, brace-matched:  file/operations 33 of 33   shell/environment 21 of 21
                              git/operations  19 of 19   shell/process     13 of 13
                              tri/net/http     7 of 7    tri/encoding/json  3 of 3
$ grep -o '^    fn [a-z_]*([^)]*)[^{]*' specs/file/operations.t27 | shasum | cut -c1-12
005522d8d251
$ grep -c 'TODO: Implement' specs/file/operations.t27    0      (on a file where 33 of 33 are empty)
$ grep -c 'TODO: Implement' specs/shell/environment.t27  0      (21 of 21 empty)
$ grep -c 'TODO: Implement' specs/git/operations.t27     0      (19 of 19 empty)
$ grep -c 'TODO: Implement' specs/shell/process.t27      0      (13 of 13 empty)
```

That last block is the reason every implementation criterion in this batch counts empty
bodies by brace matching. Four of the six files would read as 100% finished to a marker
grep, and any criterion built on one would pass before a bee did anything at all.

## Surviving issues

All 8 ship. Criteria counts are after the audit.

| # | file | path | criteria | bodies | order |
|---|---|---|---|---|---|
| 1 | `file-operations-path-and-content.md` | `specs/file/operations.t27` | 6 | 10 | **first** of 3 |
| 2 | `file-operations-read-and-write.md` | `specs/file/operations.t27` | 6 | 12 | **second** — needs #1 |
| 3 | `file-operations-directory-and-ignore.md` | `specs/file/operations.t27` | 6 | 11 | **third** — needs #1 and #2 |
| 4 | `implement-shell-environment.md` | `specs/shell/environment.t27` | 6 | 21 | independent |
| 5 | `implement-shell-process-control.md` | `specs/shell/process.t27` | 6 | 13 | independent |
| 6 | `git-operations-implement-19-command-bodies.md` | `specs/git/operations.t27` | 6 | 19 | independent |
| 7 | `implement-tri-http-url-and-status-helpers.md` | `specs/tri/net/http.t27` | 5 | 7 | independent |
| 8 | `tri-json-parse-stringify-get.md` | `specs/tri/encoding/json.t27` | 6 | 3 | independent |

Titles, as `gh` will read them from line 1:

1. Implement the 10 pure path and content bodies in specs/file/operations.t27
2. Implement the 12 read and write bodies in specs/file/operations.t27
3. Implement the 11 directory, ignore and absolute-path bodies in specs/file/operations.t27
4. Implement the 21 environment-access functions in specs/shell/environment.t27
5. Implement the 13 empty process-control functions in specs/shell/process.t27
6. Implement the 19 empty git command bodies in specs/git/operations.t27
7. Write the seven empty bodies in specs/tri/net/http.t27 — URL parsing and HTTP status classification
8. Write the RFC 8259 parser, serializer and object lookup in specs/tri/encoding/json.t27

### Boundary parse

Ported `QueenIssueBoundary` (section headed `## Boundary`, read to the next heading,
first path-shaped token per line) and ran it on all 8 files. Every one yields exactly
one entry and it is the intended path. Each `## Boundary` section is last in its file
and holds one blank line, one path, EOF — verified with `od -c`, so there is no prose
for the parser to mistake for a second boundary.

Ported `QueenSpecQuality.bullets` (heading text *contains* "acceptance criteria",
items begin `- ` or `* `) and ran it on all 8: it collects 6/6/6/6/6/6/5/6 = 47 items,
exactly the intended criteria, no stragglers. Every criterion is one physical line and
is written as `- 1.` … `- 6.`, never `1.`, which the bullet parser would drop.

### Boundary collisions

One collision, by design: issues 1, 2 and 3 all edit `specs/file/operations.t27`.
**They must not run concurrently.** All three say so in their own opening paragraph,
name the other two by filename, state the fixed order, and open with a start-state
check that stops the bee if the file is not where it should be
(`gen | grep -c 'not yet implemented'` must read `33`, then `23`, then `11`).
Issues 4–8 have five distinct paths and no collision with anything.

## Shipping them

Nothing below was run by this audit — the issues are drafted, not shipped.

```
$ gh issue create \
    --repo gHashTag/t27 \
    --title "$(head -1 datasets/bee-tasks-capabilities/issues/implement-shell-environment.md | sed 's/^# //')" \
    --body-file datasets/bee-tasks-capabilities/issues/implement-shell-environment.md \
    --label bee,t27-spec
```

All five independent issues, after `gh label list` confirms the labels exist:

```
$ for f in implement-shell-environment implement-shell-process-control \
           git-operations-implement-19-command-bodies \
           implement-tri-http-url-and-status-helpers \
           tri-json-parse-stringify-get; do
    p=datasets/bee-tasks-capabilities/issues/$f.md
    gh issue create --repo gHashTag/t27 \
      --title "$(head -1 "$p" | sed 's/^# //')" --body-file "$p" --label bee,t27-spec
  done
```

The three `file-operations-*` issues go **one at a time**, each opened only after the
previous one has merged. Opening all three at once is the one way to lose this work:
three bees editing the same 400-line file will clobber each other, and every count in
issues 2 and 3 assumes the earlier ones have landed.

`--body-file` carries the whole file including its `# ` title line; strip it with
`tail -n +2` first if that duplication matters.

The swarm reads two things by machine: the path under `## Boundary`, and the bullets
under `## Acceptance criteria`. The bee answers each as
`- N. <criterion>: met | unmet | could-not-check` in a `## VERDICT` block.

## What these tasks do NOT give us

Filling these 96 bodies is **necessary and nowhere near sufficient** for a `.t27`-native
toolchain. Stated plainly, because the gap is easy to lose behind a green dashboard.

**1. Nothing will actually touch the operating system.** There is no host-call
mechanism in the language. Measured: zero `extern` declarations anywhere under
`specs/` (`grep -rnE '^[[:space:]]*(pub[[:space:]]+)?extern\b' specs/ | wc -l` → `0`);
the only intrinsic names appearing anywhere in `bootstrap/src/` are `@as` and
`@truncate` (`grep -rhoE '"@[a-z_]+"' bootstrap/src/ | sort -u`); `bootstrap/src/host/`
is `csr_map.rs driver.rs irq.rs mmio.rs mod.rs` — MMIO/CSR/IRQ hardware support, not
process environment. So `read`, `write`, `spawn`, `kill`, `getenv` are written in terms of
invented seam names (`fs_read`, `fs_list`, `fs_cwd`, …) that are **declared nowhere and
resolve to nothing**. The issues adopt that convention deliberately, because
`specs/file/schema.t27` is already marked `IMPLEMENTED` while calling a `split` that
exists nowhere — `spec-status` reports on body presence, not on resolution. The
consequence is exact: after all 8 issues land, `spec-status` will say `IMPLEMENTED`
and not one of these functions will be able to open a file.

**2. The seam specs those names point at do not exist yet.**
`specs/tri/io/fs.t27` and `specs/tri/io/filesystem.t27` are `UNWRITTEN`;
`specs/tri/io/reader.t27` and `specs/tri/io/writer.t27` are `NOPARSE`. A
`specs/file/io.t27` issue, and the compiler work to make a host call mean anything,
are both still unfiled.

**3. Four of the six files cannot reach a compiler even when finished.** These are
codegen defects outside every one of these boundaries:

| file | today | cause |
|---|---|---|
| `specs/file/operations.t27` | `test-report` BLOCKED | codegen strips the quotes off `"/home/project"` → `spec.zig:263:86` |
| `specs/git/operations.t27` | `test-report` BLOCKED | the map type `env: [str:str]` passes through unchanged → `spec.zig:170:19` |
| `specs/tri/encoding/json.t27` | `test-report` BLOCKED | `JsonValue.data : *JsonValueData` points at a type nothing declares (issue 8 closes this one) |
| `specs/shell/process.t27` | `NOPARSE` on the PATH binary | `struct ProcessID(str);` — the newtype form the newer parser rejects |

Only `specs/tri/net/http.t27` compiles and runs its tests today.

**4. The two binaries disagree, and no single one of them can judge a spec.**
`./target/release/t27c` is the only build that prints parse metrics; the PATH build is
the only one with `spec-status`, `test-report` and `check-calls`. They disagree on
real files — `gen` on `specs/shell/process.t27` yields 13 stubs from one and a hard
parse error from the other. Every issue in this batch has to name which binary each
command needs. A toolchain cannot be built on that; one build has to win.

**5. Nothing consumes these modules.**
`grep -rnE 'shell::process|shell::environment|git::operations|file::operations|tri::net::http|tri::encoding::json' specs/ --include='*.t27'`
matches **nothing at all** — not one line in 506 files, exit status 1. The only
`use` lines in the whole corpus that reach these families are each file importing its
own sibling `schema`. No orchestrator, no driver, no `main` — so filling the bodies
produces a library with zero callers, and correctness of the composition is untested
by construction.

**6. A tool needs argv, stdout and an exit code, and the corpus has no notion of any.**
`grep -rln '\bargv\b' specs/ --include='*.t27'` → `0` files. Until a `.t27` program can
be *invoked*, a `.t27` "tool" is a set of functions nobody can run.

**7. No criterion here proves a function is correct.** `t27c test-report` does compile
and run (Zig 0.16.0), so the http and json issues can demand real passing tests — but
for the four BLOCKED/NOPARSE files every criterion is structural: a body exists, a
string survived codegen, a name is referenced. A `normalize` that mishandles `..` and
a `normalize` that handles it correctly are indistinguishable to all six criteria in
issue 1. Structural gates are what is available; they are not proof of behaviour, and
a green VERDICT block should never be read as one.

**8. Scale.** These 8 issues close 6 files. The corpus is 506 `.t27` files:
178 `IMPLEMENTED`, 158 `UNWRITTEN`, 114 `NOPARSE`, 51 `NOFN`, 5 `PARTIAL`. Six files is
1.2% of the corpus and about 4% of what is unwritten. The capability layer is the right
6 to do first — a tool that walks a repo needs files, processes, env and git before it
needs anything else — but "the capabilities are filled" is not "the swarm can write its
own tools in `.t27`."
