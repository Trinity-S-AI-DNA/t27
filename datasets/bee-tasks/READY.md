# READY.md — 36 bee issues, audited and repaired

Audited 2026-09-09 against `/Users/playom/t27` with `./target/release/t27c`
(the binary at `/Users/playom/t27/target/release/t27c`, built 2026-09-08).

**Every one of the 183 acceptance criteria below was executed against the corpus in its
current, unimplemented state, and every one of them failed.** That is the bar: a
criterion that already passes before any work is done is decoration, and a bee can
honestly report `met` for it without touching the file.

## Audit result

| | count |
|---|---|
| issues drafted | 36 |
| issues surviving | **36** |
| issues deleted | 0 |
| criteria before the audit | 199 |
| criteria that already passed in the unimplemented state (deleted or folded) | **89 (45%)** |
| criteria after the audit | **183** |
| criteria that fail in the current state | **183 (100%)** |
| file-boundary collisions found | 0 (36 issues, 36 distinct files) |
| fabricated / unreproducible quoted outputs found | 32 |
| fabricated / unreproducible quoted outputs remaining | **0** |
| stub function bodies these 36 issues cover | 182 |

Nearly half of the original criteria — 89 of 199 — passed before a bee did anything. That is
the headline finding: the drafts were mostly well-researched, but their *gates* were
not gates. No issue had to be deleted; every one of the 36 target files contains real,
bounded, unimplemented work.

## What was broken, and what it was replaced with

**1. The parse gate was free in all 36 issues.** Criterion 1 everywhere was
"`t27c parse F` reports recovery-events 0, declarations-swallowed 0,
lexer-discarded-chars 0". All 36 files already parse clean, so all 36 issues opened
with a criterion a bee could satisfy by doing nothing. It is still a necessary
regression guard, so it was not deleted — it was folded into a compound criterion 1
alongside a body scan that reads `nonempty=0` today, so the pair can only pass after
the work.

**2. Signature-preservation criteria were free.** "`grep -c 'kind: FnDecl'` prints
exactly 8", "`t27c count | grep FnDecl` still shows FnDecl 10", "all four named
functions still exist" — 14 issues carried one, and every one of them passed already.
Folded into criterion 1 as sub-check 1a, which now greps the function names literally
(`fn (empty|with_capacity|push|...)` ) rather than counting nodes, so renaming a
function fails it too.

**3. `t27c count` output is not reproducible, and 10 issues pasted it as if it were.**
Rows with equal counts print in a different order on every invocation — measured, 3
distinct orderings across 8 consecutive runs of `t27c count` on the same unchanged
file. Every pasted `count` table in the issues was therefore unreproducible, and 22 of
the 32 quoted-output mismatches found in the audit trace to this one cause. All pasted
tables were collapsed to `t27c count F | head -1`, which is the only deterministic part
of that output.

**4. `t27c count | grep TestBlock` is a trap that fires when the bee succeeds.** The
table stops after 15 rows with `... and N more`. 14 files in `specs/` contain test
blocks that `t27c count | grep TestBlock` cannot see — and they are the richly
implemented files (20 to 27 node types), which is exactly the state these tasks aim
for. Seven issues gated on that command. All test-block counting now goes through
`t27c parse F | grep -c 'kind: TestBlock'`, which never truncates and which returned
an identical number on 10 consecutive runs.

**5. Two issues quoted `t27c lint --json` output that the tool does not produce.**
`batch-2-5` and `batch-2-6` showed `{"functions": 4, "tests": 4, "invariants": 0, ...}`.
The tool prints multi-line pretty JSON. Replaced with a real, deterministic extraction
(`| python3 -c 'import json,sys; ...'`) and its real output.

**6. Prose criteria that no command could settle.** "prints the same 6 signature lines
quoted above, unchanged" (5 issues) asked for an eyeball diff with no expected value.
"all four named functions still exist", "both named functions still exist and the three
invariants survive" (6 issues) had no number attached. All rewritten as commands with
an expected output.

**7. Eleven false statements about who owns a neighbouring file.** Six issues told the
bee that a neighbour "another task owns" or "other bees own" when no issue in the set
owned it (`boyer_moore.t27`, `queue.t27`, `base32.t27`, `logger.t27`, `git/schema.t27`,
`git/diff.t27`, `git/operations.t27`, `norm.t27`, `gf16.t27`,
`gelu_approx_activation.t27`). The boundary was right; the reason given was invented.
Rewritten to say which files another issue really owns and which are simply out of
scope.

**8. One threshold was unreachable.** `batch-5-2-constants` would have required 3
real-code nodes per function from twelve pure constant accessors. Measured against a
hand-written reference file in this dialect: three accessor-style functions produce 5
real-code nodes, 1.7 per function. That issue's floor was lowered to 1 per function and
the measurement is quoted in the issue.

**9. Two issues had no `TODO: Implement` marker at all.** `specs/git/status.t27` and
`specs/ml/transformer/mha_block.t27` report `grep -c 'TODO: Implement'` = 0, so their
stub criterion already passed. Both are genuinely stubbed — 10 of 13 and 9 of 9
functions have comment-only bodies. `status.t27` now gates on its real marker
(`// Implementation`, 10 occurrences); `mha_block.t27` has no marker and carries 4
criteria instead of 5.

## What each issue is gated on now

Uniform across all 36, each command with an expected output, each failing today:

| # | check | why it cannot be faked |
|---|-------|------------------------|
| 1 | named signatures survive **and** every body is non-empty **and** the file parses clean | prose bodies leave `nonempty` at 0; deleting functions fails the name grep |
| 2 | the stub marker is gone (`TODO: Implement`, or `// Implementation` for `git/status`) | not present in `mha_block`, which has 4 criteria |
| 3 | `grep -cE 'kind: (StmtLocal\|ExprReturn\|ExprCall\|ExprBinary\|ExprIndex\|ExprFieldAccess)'` over `t27c parse` | comment-only bodies produce none of these; the floor is 2 per stubbed function, and 88.0% of stub-free parse-clean gold files carry at least that |
| 4 | `t27c parse \| grep -c 'kind: TestBlock'` >= existing + one per function implemented | only blocks the parser accepted are counted |
| 5 | `t27c count \| head -1` node types >= a per-file floor of 11-18 | 50-78% of stub-free parse-clean gold files are at or above each floor |
| 6 | *(4 issues)* an algorithm-specific literal: the xorshift* multiplier, the `ZERO` hinge name, `0x428a2f98`, or the removal of `default_input()` | the only checks here that constrain *which* algorithm was written |

Every threshold is gold-relative: measured against the 292 files in `specs/` that are
stub-free and parse with recovery-events 0, never against perfection. The gold corpus
itself is only 90.3% parse-clean (457 of 506 files), so "must be perfect" would be a
criterion the corpus's own authors fail.

## Surviving issues

| file | title | target spec | stubs | criteria |
|------|-------|-------------|-------|----------|
| `batch-0-1-bitvector.md` | Implement the eight stubbed functions in specs/tri/collections/bitvector.t27 | `specs/tri/collections/bitvector.t27` | 8 | 5 |
| `batch-0-2-random.md` | Implement the four stubbed functions in specs/tri/utils/random.t27 | `specs/tri/utils/random.t27` | 4 | 6 |
| `batch-0-3-utf8.md` | Implement the four stubbed functions in specs/tri/utils/utf8.t27 | `specs/tri/utils/utf8.t27` | 4 | 5 |
| `batch-0-4-version.md` | Implement parse, compare and next in specs/tri/utils/version.t27 | `specs/tri/utils/version.t27` | 3 | 5 |
| `batch-0-5-net.md` | Implement parse_ip, is_localhost and is_valid_port in specs/tri/net/net.t27 | `specs/tri/net/net.t27` | 3 | 5 |
| `batch-0-6-kmp.md` | Implement build_prefix and search in specs/tri/search/knuth_morris_pratt.t27 | `specs/tri/search/knuth_morris_pratt.t27` | 2 | 5 |
| `batch-1-1-tribitmap.md` | Implement the ten stubbed functions in specs/tri/collections/bitmap.t27 (TriBitmap) | `specs/tri/collections/bitmap.t27` | 10 | 5 |
| `batch-1-2-git-status.md` | Implement the ten comment-only functions in specs/git/status.t27 (GitStatus) | `specs/git/status.t27` | 10 | 5 |
| `batch-1-3-ring-buffer.md` | Implement the five stubbed functions in specs/tri/collections/ring_buffer.t27 (TriRing) | `specs/tri/collections/ring_buffer.t27` | 5 | 5 |
| `batch-1-4-base64.md` | Implement the six stubbed functions in specs/tri/crypto/base64.t27 (TriBase64) | `specs/tri/crypto/base64.t27` | 6 | 5 |
| `batch-1-5-deque.md` | Implement the six stubbed functions in specs/tri/collections/deque.t27 (TriDeque) | `specs/tri/collections/deque.t27` | 6 | 5 |
| `batch-1-6-mha-block.md` | Implement the nine comment-only functions in specs/ml/transformer/mha_block.t27 (MHABlock) | `specs/ml/transformer/mha_block.t27` | 9 | 4 |
| `batch-2-1-tri-statistics.md` | Implement the six stubbed functions in TriStatistics (specs/tri/math/statistics.t27) | `specs/tri/math/statistics.t27` | 6 | 5 |
| `batch-2-2-tri-bytes.md` | Implement the eight stubbed functions in TriBytes (specs/tri/utils/bytes.t27) | `specs/tri/utils/bytes.t27` | 8 | 5 |
| `batch-2-3-tri-text.md` | Implement the four stubbed functions in TriText (specs/tri/utils/text.t27) | `specs/tri/utils/text.t27` | 4 | 5 |
| `batch-2-4-tri-logging.md` | Implement the five stubbed functions in TriLogging (specs/tri/utils/logging.t27) | `specs/tri/utils/logging.t27` | 5 | 5 |
| `batch-2-5-tri-option.md` | Implement the four stubbed functions in TriOption (specs/tri/collections/option.t27) | `specs/tri/collections/option.t27` | 4 | 5 |
| `batch-2-6-relu-activation.md` | Implement forward and backward in the Relu activation spec (specs/ml/activation/relu_activation.t27) | `specs/ml/activation/relu_activation.t27` | 2 | 6 |
| `batch-3-1-bezier.md` | Implement `evaluate` and `derivative` in specs/tri/math/bezier.t27 | `specs/tri/math/bezier.t27` | 2 | 5 |
| `batch-3-2-msgpack.md` | Implement `encode` and `decode` in specs/tri/encoding/msgpack.t27 | `specs/tri/encoding/msgpack.t27` | 2 | 5 |
| `batch-3-3-config.md` | Implement `parse` and the three getters in specs/tri/utils/config.t27 | `specs/tri/utils/config.t27` | 4 | 5 |
| `batch-3-4-priority-queue.md` | Implement the six stubbed functions in specs/tri/collections/priority_queue.t27 | `specs/tri/collections/priority_queue.t27` | 6 | 5 |
| `batch-3-5-circular-buffer.md` | Implement the five stubbed functions in specs/tri/collections/circular_buffer.t27 | `specs/tri/collections/circular_buffer.t27` | 5 | 5 |
| `batch-3-6-gelu-activation.md` | Implement `forward` and `backward` in specs/ml/activation/gelu_activation.t27 | `specs/ml/activation/gelu_activation.t27` | 2 | 5 |
| `batch-4-1-array-slice-ops.md` | Implement the nine stubbed slice/search functions in `TriArray` (specs/tri/collections/array.t27) | `specs/tri/collections/array.t27` | 9 | 5 |
| `batch-4-2-filesystem-path-ops.md` | Implement the seven stubbed path functions in `TriFilesystem` (specs/tri/io/filesystem.t27) | `specs/tri/io/filesystem.t27` | 7 | 5 |
| `batch-4-3-polynomial-arithmetic.md` | Implement the six stubbed functions in `TriPolynomial` (specs/tri/math/polynomial.t27) | `specs/tri/math/polynomial.t27` | 6 | 5 |
| `batch-4-4-linked-list-ops.md` | Implement the five stubbed functions in `TriLinkedList` and replace its five placeholder tests (specs/tri/collections/linked_list.t27) | `specs/tri/collections/linked_list.t27` | 5 | 6 |
| `batch-4-5-disjoint-set-union-find.md` | Implement the four stubbed union-find functions in `TriDisjointSet` (specs/tri/graph/disjoint_set.t27) | `specs/tri/graph/disjoint_set.t27` | 4 | 5 |
| `batch-4-6-sha256-digest.md` | Implement the four stubbed functions in `TriSha256` (specs/tri/crypto/sha256.t27) | `specs/tri/crypto/sha256.t27` | 4 | 6 |
| `batch-5-1-hex.md` | Implement the four stubbed functions in specs/tri/crypto/hex.t27 | `specs/tri/crypto/hex.t27` | 4 | 5 |
| `batch-5-2-constants.md` | Implement the twelve stubbed accessors in specs/tri/math/constants.t27 | `specs/tri/math/constants.t27` | 12 | 5 |
| `batch-5-3-sigmoid.md` | Implement forward, forward_batch and derivative in specs/ml/activation/sigmoid_activation.t27 | `specs/ml/activation/sigmoid_activation.t27` | 3 | 5 |
| `batch-5-4-softmax.md` | Implement softmax forward and backward in specs/ml/activation/softmax.t27 | `specs/ml/activation/softmax.t27` | 2 | 5 |
| `batch-5-5-graph-bfs.md` | Implement init, add_edge, traverse and deinit in specs/tri/graph/graph_bfs.t27 | `specs/tri/graph/graph_bfs.t27` | 4 | 5 |
| `batch-5-6-quick-sort.md` | Implement sort and sort_range in specs/tri/sort/quick_sort.t27 | `specs/tri/sort/quick_sort.t27` | 2 | 5 |

## Shipping them

From `/Users/playom/t27`, one issue at a time (nothing below was run by the audit — the
issues are drafted, not shipped):

```
$ gh issue create \
    --repo <owner>/<repo> \
    --title "$(head -1 datasets/bee-tasks/issues/batch-0-1-bitvector.md | sed 's/^# //')" \
    --body-file datasets/bee-tasks/issues/batch-0-1-bitvector.md \
    --label bee,t27-spec
```

All 36, after checking the label exists (`gh label list`) and confirming the repo:

```
$ for f in datasets/bee-tasks/issues/batch-*.md; do
    gh issue create --repo <owner>/<repo> \
      --title "$(head -1 "$f" | sed 's/^# //')" \
      --body-file "$f" --label bee,t27-spec
  done
```

The first line of every issue file is its `# ` title and the rest is the body, so
`--body-file` carries the whole file including the title line; strip it first if that
duplication matters:

```
$ tail -n +2 datasets/bee-tasks/issues/batch-0-1-bitvector.md > /tmp/body.md
```

The swarm harness reads two things out of each issue: the body, and the numbered list
under `## What you will be judged by`. Those numbers are the criteria the bee must
answer in its `## VERDICT` block, one per line, each ending in exactly `met`, `unmet`
or `could-not-check`. Every issue ends with that block pre-written so the wording
matches.

## What these tasks cannot prove

**There is no t27 interpreter.** Nothing in this repository can execute a `.t27` test
block. `t27c` parses, counts, and generates C / Rust / Verilog / Zig; it does not run
anything. Every criterion in every one of these 36 issues is therefore a *structural*
check. Concretely:

- **No criterion proves a function is correct.** A `get` that returns the wrong bit, a
  `sort` that does not sort, a `sha256` that hashes to the wrong digest — all of them
  pass every criterion here, provided they are written as real code with tests beside
  them. The four algorithm-specific literal checks in criterion 6 are the only place
  correctness is touched at all, and they only prove that the right constant appears in
  the file, not that it is used correctly.
- **No criterion proves a test passes.** Test blocks are parsed and counted, never
  executed. A `test` whose `then` clause is false counts exactly as much as one that is
  true. Any bee that reports "the tests pass" is reporting something it cannot have
  checked; the issues say so explicitly and tell it not to.
- **`t27c typecheck` is not a gate and is banned from every issue.** Verified on
  2026-09-09: it returns `{"ok":true,"errors":0}` on English prose and on a Python
  file. No criterion here cites it.
- **`t27c gen` / `compile` were not used as gates either.** Generating C or Zig from a
  spec would be a stronger signal than parsing, but generation succeeding still says
  nothing about the semantics of what was generated, and it introduces a second failure
  surface (the backend) that a bee cannot distinguish from its own mistake.
- **What is actually proven** by a `met` verdict on all criteria: the named functions
  still exist; none of their bodies is empty or comment-only; the file still parses with
  zero recovery events, zero swallowed declarations and zero discarded characters; it
  contains real statement and expression nodes rather than prose; it carries one new
  parsed test block per function implemented; and it is structurally at least as rich as
  the median implemented file in `specs/`.

That is a floor on *effort and form*, not a proof of *behaviour*. Reviewing the
resulting `.t27` by hand, or building an interpreter, is the only way to close that gap
— and until one exists, no issue in this set should be written as if it could.
