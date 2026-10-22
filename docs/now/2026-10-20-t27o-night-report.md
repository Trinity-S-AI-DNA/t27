gHashTag/t27 - epic #10522 - linked to analog emulation #10486 - t27b side: epic #10879

# t27o Night Report (2026-10-18 to 2026-10-20)

Saved from the private report page at its version 27 so that the record lives in the repository; one section (a partnership option that was not chosen) is left out of this public copy. The running ledger is the epic comment named below; the queue for the next worker is the handoff comment on epic #10522.

**Handed to the Queen, 2026-10-20 14:30 local.** At the owner's order the loop is stopped (cron deleted, no lane running) and its queue, rules and open decisions are in the handoff comment on epic #10522. The next tick of work is #11258, then one port per issue.

t27o turns foreign code into `.t27`. This report covers two days of the loop that runs it on every roadmap repository: 2026-10-18 23:04 to 2026-10-20 11:45 local (UTC+7). The first night measured the weak spots and fixed the Rust converter; the second day started the Zig front end, made the output valid, and moved the native runner t27b from 89.9% to 94.1% of what t27o writes.
 as of 2026-10-20 11:45 local Ledger: [epic comment](https://github.com/gHashTag/t27/issues/10522#issuecomment-6252258786) Every number comes from a command named in the epic or a PR, at a named commit. Draft work is marked as draft.

## Measured since the first report

- **89.9% -> 94.1%** t27o specs that run natively under t27b: 1,093 -> 1,144 of 1,216; the real gap (t27c passes, t27b refuses) was 58, 21 after #10999 (PRs #10999 and #11046, merged)
- **55 -> 0** specs t27o wrote that t27c could not compile; now valid or refused. Converted fns 1,890 -> 1,791 because 99 invalid conversions are refused (PR #10976)
- **1,839 -> 2,022** Zig fns converted, step 1 to step 2 of the Zig front end, judged by each file's own `test` blocks (PRs #10819, #10918)
- **9% -> 100%** functions of `rust_convert.t27` named by a test: 32 of 360 before, 395 of 395 now (PR #10883)
- **9.93% -> 9.88%** Rust fns converted on the same measure (without the five big files): 1,666 of 16,775 at 04:20, 1,657 of 16,765 tonight. Flat, because 99 invalid conversions are now refused (PR #10976) and the heap types below are not in the subset
- **128 of 185** issues the issue-shape gate had closed wrongly, found and repaired; the gate itself is fixed (issue #11007, PR #11008)
- **25 min -> 26 s** t27o over all 1,388 Rust files, and `compiler.rs` from over 600 s to 1.4 s (first report, PRs #10587, #10631)

## t27b for t27o: the native runner on what t27o writes

t27b compiles a spec to native arm64 and runs its tests in about a hundredth of the time of `t27c test-report`. The question of epic #10879 is whether it can run every spec t27o emits and agree with t27c. The corpus is the 1,216 specs t27o writes from the roadmap repositories (492 from Rust, 724 from Zig), run with a 20 s cap per spec.

| Front end | Specs | Run on master before | After #10999 | After #11046 |
|---|---|---|---|---|
| Rust | 492 | 387 | 420 | 434 |
| Zig | 724 | 706 | 710 | 710 |
| **Total** | 1,216 | 1,093 (89.9%) | 1,130 (92.9%) | 1,144 (94.1%) |

The last column was measured against a slightly different master build (Rust 421 before, 434 after, none lost). Agreement: all 37 newly passing specs of #10999 and all 13 of #11046 pass every test under t27c as well (0 disagreements). What t27b withdraws for a body or a constant that no test reaches is exactly what Zig never type-checks there; the refusals Zig's parser makes in every fn are kept. A hole in the first version, found by the lane that wrote #11046: a local of a type declared nowhere, in an unreached fn, passed under t27b while t27c refused it (issue #11053; fixed in #11046).

## First night, start against end

Same sources (t27 at 7496ed220, trios 483dd9ee, trinity 0770abb9, trinity-fpga 20bf4c418, 999-multibots-telegraf 6e5512881), same file set; only the converter differs. These are the figures of the first report. The sweep re-run tonight, on the checkouts as they stand now, is in the next section.

| Measure | Start | End |
|---|---|---|
| Rust fns converted (16,775 seen) | 1,573 (9.38%) | 1,666 (9.93%) |
| Rust tests converted (8,228 seen) | 504 | 592 |
| Files over the time cap | 5 (30 s cap) | 0 (slowest 1.36 s) |
| Wall time for all 1,388 files | about 25 min | 26 s |
| Silent miscompiles known | 1 (integer `!`) | 0 known |

## The Rust ceiling

Tonight's sweep covers all 1,388 tracked Rust files of the five repositories plus the five big files: 19,392 fns seen, 1,824 converted (9.41%), 16,057 fns refused at a first construct. The converter's subset is pure scalar, slice and struct code. The refusals show how much of the roadmap code is outside it.

| First refusal | Largest tokens | Count | Share of 16,057 |
|---|---|---|---|
| uses a refused fn (cascade) | `self` 563 | 4,468 | 27.8% |
| type | `Vec` 1,289, `Path` 435, `String` 232, `PathBuf` 230, `std` 212, `anyhow` 144 | 3,049 | 19.0% |
| method | `join` 421, `iter` 246, `to_string` 235, `into` 189, `push` 164 | 2,765 | 17.2% |
| macro | `vec` 552, `format` 89 | 960 | 6.0% |
| tuple | `(` 587 | 847 | 5.3% |
| name declared elsewhere | `new` 323 | 742 | 4.6% |
| path call |  | 709 | 4.4% |

Cascade, type and method are 64.0% of the first refusals, and most of them are heap, path and I/O types and the methods that go with them. The two days of converter work (std methods, valid output, `vec!` literals, the `Vec` out-buffer form) removed real blockers but moved the share by almost nothing: the `Vec` out-buffer form (#11109) converts 4 fns, because only 91 of 626 fns returning a bare `Vec` build it by `push` alone and only 19 of those have scalar elements. The figure that can move is the share inside the subset, and the files that convert whole; the next loop reports those two.

Root attribution (#11127) on the saved outputs: "uses refused" falls from 4,468 to 696 that cannot be followed, 3,824 are counted under the first refusal at their root (type 4,263 -> 6,180, method 2,765 -> 3,323 among all refused items), and 85 files convert whole. Of the 696, 656 were `self` or `Self`: the refusal line did not name the receiver type. #11130 names it, and the cascades left unattributed fall to 58 (47 type aliases, 9 extern fns, 2 `assert`).

Two measurements after that, each made before any rule was written, say the same thing. `Vec` parameters are 87 of the 1,289 `type Vec` refusals (6.7%), and the form could convert at most 1 fn (#11135). Iterator chains with a one-expression closure: of 676 fns where a chain can be lowered, 17 convert end to end, 32 counting callers, 0.17% of all fns against a 1% target (#11145). Both issues were closed as not planned, with the tables on them. The converted share is bounded by what t27 cannot say (heap collections, owned strings and paths, closures over iterators, tuples), not by missing converter rules. What can still move is the file: 85 files convert whole today.

Two analyses of what conversion can delete close the question for Rust. Whole files hold 3,006 hand lines (0.69% of the 435,491 swept), and the movable functions of partly converting files hold 1,375 (0.32%), 217 of them in the crates the t27 workspace builds. Together that is about 1% of the hand-written lines, and the first of them is gone: #11157. Each further port costs one spec, one generated file, one seal and one ledger row, so the largest remaining bundle (201 lines in `rings/ring-092-rust`, whose spec `specs/nn/attention.t27` already declares four of its fns with another argument order) is a demonstration and not a programme.

## Pull requests

| PR | What | State |
|---|---|---|
| [#11160](https://github.com/gHashTag/t27/pull/11160) | The per-function golden prize, measured: 223 partly converting files hold 817 converted fns with 6,555 hand lines (1.51% of the swept lines); 146 fns with 1,375 lines (0.32%) can move with no caller change, 217 of those lines in the crates the t27 workspace builds, and the largest bundle there is 45 lines. 44 of 187 ranked rows are marked as not checked for valid output (the run was stopped when the machine ran out of disk) | merged |
| [#11176](https://github.com/gHashTag/t27/pull/11176) | The Python debt census (#11168), parsed with Python's own `ast`: 812 of 331,183 distinct hand-written lines (0.25%) sit in functions that fit the typed subset today, 4,090 (1.23%) with inferred parameter types, and a further 5.0% (16,577 lines) is pure code blocked only by heap values. No whole-file candidate of 150 lines or more, so no Python front end is built. The scanner agreed with 30 of 30 hand-classified functions on its final sample | merged |
| [#11207](https://github.com/gHashTag/t27/pull/11207) | `tri pr preflight` exists. The glue (40 lines in `cli/tri/src/prcheck.rs`, at the per-file cap) is paid for by deleting 52 lines of provably dead code in `inflight.rs`: net -12 hand lines, no exception file. It runs 13 rules (check, tests with vacuity, seal, types, dupes against the base, ledger rows, census, the own-language budget, the home-path scan) and exits 0, 1, or 2 for unreadable evidence, which is never a pass. Run for real on seven cases: a clean branch passes; a copied function body, the +9/-0 shape of #10538 and a home path each fail their own rule with evidence; a stubbed output and a missing `t27c` read as UNREAD. Running it also found a spec defect (a bare `mktemp -d` that ignores `TMPDIR` on macOS), fixed test first | merged |
| [#11255](https://github.com/gHashTag/t27/pull/11255) | The second golden port (issue #11235): `bootstrap/src/host/csr_map.rs`, 214 lines, becomes `specs/host/csr_map.t27` (30 constants, 13 tests) and a generated Rust file mounted like the first port. Hand lines +1/-215 (net -214). Every one of the 30 constants was compared old against new in one rustc harness: 0 disagree, and the harness reports 5 disagreements when two differences are planted. The three refused tests had to be rewritten first (`windows(2)`, `is_power_of_two`), and the lane found that a hand test asserting on two constants is folded at compile time and counts as a vacuous pass, so the 13 tests read their constants through arrays. Reds read as subsets of master: `duplicate-bodies` (the same 4 groups), `Verilog widths`, and `t27b-native-ratchet`, whose only new row is the one the steward adds for a new spec. Two ports so far delete 385 hand lines | merged |
| [#11258](https://github.com/gHashTag/t27/issues/11258) | `tri t27o-sweep --near N`: lists the files at most N refused fns away from converting whole, each refusal tagged test or logic, so the next port is chosen in one command instead of an ad-hoc ranking script. Lane N1 is on it | in work |
| [#11240](https://github.com/gHashTag/t27/pull/11240) | `gen-c` defect D5 of #11183, fixed (issue #11239): a range slice `a[lo..hi]` is now the pointer `(a + lo)` (it was `a[(lo .. hi)]`, not C, or silently the element `a[lo]`), a slice local is `T* x`, and an untyped array-literal local or constant is an array typed from its first non-number element. Probe matrices run by the lane: range slice 44 of 594 cells compiled on master, 528 now; untyped list 131 of 296, now 196 (the matrices are not committed; the committed tests are, and the new fixture gives 20 `cc` errors on master, which I reproduced). `compiler.rs` +40/-20, a never-used 117-line stub in `bridge.rs` deleted to pay the budget, `FROZEN_HASH` checked by me against the digest, also on master after the merge. 316 of 2,882 specs change their C, none newly refused, none of the 26 committed `gen/c` files; 243 specs re-sealed and 4 seals edited by hand because their Zig tests cannot run here. Three reds, all inherited: `Verilog widths`, `duplicate-bodies` (the same 4 groups fail on master) and `t27b-native-ratchet` (the same 22 rows as master). The lane's commit said `Closes #11183` while two defects remain there, so I closed #11239 instead and merged with my own message | merged |
| [#11234](https://github.com/gHashTag/t27/pull/11234) | Skills: one bullet in the `t27o` skill, "find the spec a hand file mirrors before naming it a port" (issue #11233), after the ring-092 correction in the anomalies list | merged |
| [#11232](https://github.com/gHashTag/t27/issues/11232), [#11235](https://github.com/gHashTag/t27/issues/11235) | Open, not yet pull requests: #11232 is the `gen-rust` track that replaces the ring crates; #11235 ports `bootstrap/src/host/csr_map.rs` (214 lines, three refused test bodies, no mirrored spec), lane P2 is on it | in work |
| [#11210](https://github.com/gHashTag/t27/pull/11210) | Skills: the `t27o` skill now names `tri pr preflight --quick` as the local gate and carries the lessons of day two, one line each | docs, auto-merge armed |
| [#11213](https://github.com/gHashTag/t27/pull/11213) | Two `gen-rust` defects from the RFC experiment fixed (#11182): a slice next to a struct field named `len` now reads `.len()`, and a `[N]str` element converts to bytes where bytes are needed. On a 93-cell matrix, D1: 27 wrong on master, 9 now (7 of those are invalid cells or unrelated); D3: 16 wrong, 0 now; no cell that compiled on master is wrong now. `compiler.rs` +25/-44 (net -19), `FROZEN_HASH` checked by me against the file's digest. 85 generated outputs change, and 93 seals were re-saved after the lane read each change. The lane asked to split it from a third change that would have raised a ledger by 66 rows; it also retracted, in its own PR body, an earlier set of claims from a broken measuring loop | merged |
| [#11218](https://github.com/gHashTag/t27/issues/11218) | The third `gen-rust` defect, held as a draft pull request: a `[]T` struct field filled from borrowed storage is written `Vec<T>` and the caller's storage is moved. Refusing it loudly touches 73 specs that rustc already rejects on master, but adds 66 rows to a suite ledger (69 to 135). That is your call, see below | held |
| [#11192](https://github.com/gHashTag/t27/pull/11192) | The silent miscompile found by the RFC experiment (#11181) is fixed. `t27c gen-c` wrote a repeat array `[7] ** 4` as four zeros with exit 0, so the generated C computed other values than the spec, Zig and `t27b`. On a 408-cell probe matrix 137 cells were wrong on master and none are now. Cause: the parser gives a repeat's single element two shapes, and the C emitter only handled one. The diff in `compiler.rs` is +40/-42 (net -2 hand lines), `FROZEN_HASH` is updated, 66 of 2,857 generated outputs change (zeros became the real values, nothing else), and 3 specs that repeat a list, which C cannot write, are now refused loudly and ledgered with an expiry instead of miscompiled. The first CI run found 16 stale seals the lane had wrongly judged already stale; re-sealed, then all checks passed except two that master fails too | merged |
| [#11180](https://github.com/gHashTag/t27/pull/11180) | The RFC for heap-free collections, owned strings and inlined closures (#11177), 981 lines, written from the measured demand. Its recommendation is **none of the five constructs now**: at the end-to-end rates measured so far the whole package converts 30 to 64 more Rust fns (0.15% to 0.33%) and deletes 15 to 139 hand lines, and the bar to build anything is a pilot rate of at least 15 in 100. The experiment (four census functions written by hand in the proposed forms, 9 tests pass under both runners, 9 of 9 mutants killed) also found seven back-end defects, now issues #11181 to #11183; one of them, #11181, is a silent miscompile in `gen-c` that I reproduced on a fresh master build | merged |
| [#11157](https://github.com/gHashTag/t27/pull/11157) | The first golden port (step 4 of #9000): `bootstrap/src/host/bitmask.rs` becomes `specs/host/bitmask.t27` and the Rust that `t27c gen-rust` writes from it. Hand-written lines +1 / -171. The hand file and the generated one agree on 756,232,068 side-by-side comparisons (0 disagreements; 12.8 million out-of-contract calls panic on both), the real `t27c` crate builds and passes 577 tests, the spec has 26 tests and 100% function coverage, and all 46 CI checks are green except two that master has too. Public differences, stated in the spec header: `const fn` becomes `fn`, `FieldDesc` methods become free functions, and `FieldDesc` no longer derives `PartialEq` and `Eq`; nothing in the repository uses any of them | merged |
| [#11155](https://github.com/gHashTag/t27/pull/11155) | The ranked golden candidates: 85 whole files, 80 rankable rows, 3,006 hand lines (0.69% of the swept lines); 78 portable now, 1 needs a caller change, 1 not now | merged |
| [#11127](https://github.com/gHashTag/t27/pull/11127) | `tri t27o-sweep` attributes each "uses refused X" to X's own first refusal and prints the files that convert whole. On the saved outputs of tonight's sweep, unattributed cascades fall from 4,468 to 696, 3,824 are counted under named roots, 85 files convert whole. The real command has not been run end to end yet; CI compiled the glue | merged |
| [#11125](https://github.com/gHashTag/t27/pull/11125) | `tri pr preflight` spec gains two rules: the own-language budget compiled from the merge base, and the developer-home-path scan. On #10538's real head (+9/-0 hand lines) the budget rule reports a failure. Spec only: the command's glue is parked (#11126) | merged |
| [#11109](https://github.com/gHashTag/t27/pull/11109) | A fn returning `Vec<T>` built by `push` converts in the out-buffer form (#10546 step 2), with a guarded oracle thread (a Vec that never stops growing no longer eats the Mac's memory), a stray-comma fix for zero-parameter fns, and typed integer arrays (#11112). Whole-corpus sweep against the merge base: +4 converted fns, 0 dropped, 0 files from pass to not-pass | merged |
| [#11064](https://github.com/gHashTag/t27/pull/11064) | Three converter defects: a `static mut` named by its name (#10829), `str_chain` opening with a text field (#10834), 64-bit unsigned parameters sampled (#10864). Effect: no converted count changes on the files compared; 91 of 1,326 oracle files get wider sample values (28,706 -> 37,244 calls, no pass turned into a fail) | merged |
| [#11023](https://github.com/gHashTag/t27/pull/11023) | Skills: the evening lessons in the `t27o` skill | merged |
| [#11012](https://github.com/gHashTag/t27/pull/11012) | The use splice reads a name inside a pulled declaration in that declaration's module (rule in `specs/compiler/use_splice.t27`, hand lines -112); 20 of 756 importing specs generate differently, none stopped compiling | merged |
| [#11008](https://github.com/gHashTag/t27/pull/11008) | The issue-shape gate judges the issue as it is when the run starts, not its event payload | merged |
| [#10999](https://github.com/gHashTag/t27/pull/10999) | t27b: Sema-level refusals in a fn no test reaches are withdrawn, from one plan spec; 1,093 -> 1,130 specs | merged |
| [#10976](https://github.com/gHashTag/t27/pull/10976) | No invalid output: the 55 specs t27c could not compile are valid or refused | merged |
| [#10929](https://github.com/gHashTag/t27/pull/10929) | Model routing: Sonnet 5.5 for every agent, Opus only after two failures or on the owner's word | merged |
| [#10918](https://github.com/gHashTag/t27/pull/10918) | Zig front end step 2: `--check` on `.zig`, sweep lists `.zig`, imported consts, `std.math.pow` | merged |
| [#10883](https://github.com/gHashTag/t27/pull/10883) | Direct tests for every function of `rust_convert.t27`, coverage 9% -> 100% | merged |
| [#10819](https://github.com/gHashTag/t27/pull/10819) | Zig front end step 1: `zig_convert.t27` converts tier-1 fns and containers and translates the file's own tests | merged |
| [#10801](https://github.com/gHashTag/t27/pull/10801) - [#10817](https://github.com/gHashTag/t27/pull/10817) | Pure std methods convert (bit counts, `pow`, saturating ops, `sqrt`, `is_ascii`, `unwrap_or`); the second pays down the duplicate bodies the first left | merged |
| [#10648](https://github.com/gHashTag/t27/pull/10648) - [#10654](https://github.com/gHashTag/t27/pull/10654) - [#10646](https://github.com/gHashTag/t27/pull/10646) | Read-only `vec![..]` becomes a typed array; the home-path repair of #10646; `tri t27o-sweep --repo` | merged |
| [#11046](https://github.com/gHashTag/t27/pull/11046) | t27b: a module constant nothing analyzed reaches is lazy like an unreached body (issues #10899, #10903), plus the fix of the hole #11053. 13 more t27o specs pass, none lost, 0 disagreements with t27c; hand lines in cli/t27b net -1 | merged |
| [#11145](https://github.com/gHashTag/t27/issues/11145) - [#11135](https://github.com/gHashTag/t27/issues/11135) | Measured first, no rule written: iterator chains lowered to loops (17 to 32 fns would convert) and `Vec` parameters (ceiling 1 fn). The tables are on the issues, which were closed as not planned | measured, closed |
| [#11130](https://github.com/gHashTag/t27/pull/11130) | Three converter fixes: a caller of an out-buffer String fn is refused with a reason that names the form (#11111); the oracle of `regex_lite_global` samples a match (it recorded count 0 on all seven calls before; now count 2) (#11113); a refusal caused by a refused `self` receiver names the impl type (#11128). Sweep over all 1,388 files: unattributed cascades 696 -> 58, 0 converted fns lost, 0 files from pass to not-pass; the other 153 changed output specs differ in refusal comment lines only | merged |
| lane A5 | #11135: measure the `Vec` PARAMETERS among the 1,289 `type Vec` first refusals, then convert a read-only `&Vec<T>` to a slice and a push-only `&mut Vec<T>` to the out-buffer form. The lane reports the split first and opens no rule pull request if the form is not worth it | in progress |

The eleven pull requests of the first night (#10529 to #10631) are in the epic ledger. Merge rule: the four required checks green (validate, check-linked-issue, parse-ratchet, own-language); a red is inherited only when the pull request's failures are a subset of master's, compared against the merge base. fpga-conformance, fpga-lint, duplicate-bodies and the ledger's UNLISTED rows were master's reds today.

## Beyond Rust: what the roadmap is written in

Rust is a minority of the code behind app.t27.ai. The census counted every function on the same five repositories and checked it against a stated first subset per language. Zig is a real parse (std.zig.Ast); shell is coarse (regex); the subsets are ours, and "in subset" means the structure passes, not that a converter ran. The Zig front end now runs on its first subset.

| Language | Distinct fns | In a first subset | Main blocker | Roadmap stage |
|---|---|---|---|---|
| Zig real parse | 36,495 distinct (first census 28,855) | 2,022 converted | error unions are 41.8% of fns but the only blocker for 3.8%; pointers 30.1% | 5, the Zig core (#4547) |
| Rust | 16,775 | 9.9% converted (9.88% tonight) | heap values (Vec, vec!, String, Path) about 24% | 6, trios rings (#4548) |
| TypeScript | 9,746 | 2.5% | async/await 26.4% | 3, app.t27.ai backends (#4545) |
| Python | 6,468 | 3.7% | imports, tuples, methods outside the table | 1, the swarm's tools (#4543) |
| Shell coarse | 812 scripts | 1.4-2.1% | 74-82% orchestration | 1 |

## Anomalies found and repaired

First night, local time; second day, UTC.

- 23:53
The session scratchpad was wiped when the session resumed: lane worktrees, the cargo target, raw outputs. Pushed branches and agents survived. Lanes now push after every step and keep loop files in `$TMPDIR/t27o-loop`.
- 00:39
#10538 failed the required `own-language` check at +9/-0 hand lines: the policy refuses any diff whose hand-written foreign lines grow. Glue cut to 6 lines and paid with a 7-line dead function; the rule went into the skills.
- 03:58
A claim of mine, "341 generated Zig files naming `.vibee`", was refuted by the real-parse census (12 generator lines name it). Corrected in the issue and here.
- 03:22Z
#10801 merged with 13 copied bodies. My review had checked names, not bodies; its auto-merge was switched off and then re-armed by another actor with the account's token. #10817 paid the copies down. Rules now: review with `dupe_scan` on bodies against the merge base, and hold a pull request as a draft, since auto-merge can be re-armed.
- 09:40Z
The disk reached 100% (251 MB free); lane caches were the largest share. Rules now: one zig cache per run, an 8 GB floor for builds, a watcher that wakes the loop at 4 GB. A second dip to 133 MB about four hours later recovered on its own, and no owner was found.
- 06:34Z
The issue-shape gate closed 185 issues since 2026-10-18; 128 of them pass the gate's own checker with their current body. Cause: a run queued for 38 minutes judged a stale event payload. Repaired at 11:20Z: 88 closed as completed (a merged pull request names each), 40 reopened with a comment; gate fixed in #11008. I filed this as #11007 without seeing that the claw lane had documented the same defect a day earlier (#10662, with a replay spec); both are linked and closed.
- 11:42Z
#10999 merged one minute after auto-squash was armed under the owner's token, before its slower checks finished. They came back green. The same pull request had needed a rebase and a renamed fixture type (`Gate` collided with another spec's, which the types ratchet counts as a conflict).
- 11:46Z
My skills pull request #11023 turned `scan` red: a line named the maintainer's home directory, the trap of #10646, and the same rule sat in the skill I was editing. Fixed in six minutes by deriving the path from `git rev-parse`.
- 13:30Z
Lane TB2 found a hole in my merged #10999: a local of an undeclared type in an unreached fn passes under t27b where t27c refuses it. Filed as #11053 with the failing spec; the fix is in #11046.
- 13:30Z
The lane that wrote #11046 reported that the ledger commit I had told it to make did not exist. The ratchet still read one UNEXPECTED PASS (`specs/lsp/schema.t27`). I made the commit myself after the rebase, so the row and the cap (128 -> 127) are in the pull request.
- 18:05Z
I had written that `tri pr preflight` already exists. Only its spec does: the 38 lines of glue have been parked since 2026-10-18 because the own-language gate refuses net growth, and the note called it the owner's decision. The lane that added the two rules (#11125) changed no glue and said so. Corrected on #11070; the way through is the one used for #10538, glue paid by an equal deletion of dead code, filed as #11126.
- 16:16Z
Free disk fell to 759 MB with none of my jobs running. `du` found about 5 GB under the shared temp directory, 1.0 GB of it mine, so the missing space was elsewhere: the analog session measured swap at 3.0 GB in the same volume and a load average of 76 on 8 cores, with five lanes from three sessions running. Lane A4 was stopped, the two idle lane worktrees (700 MB) were removed, lanes now run one job at a time. Free space came back with the load.
- 04:55Z
The report named `rings/ring-092-rust` as the first bundle to port, 201 hand lines, with the note "reconcile with `specs/nn/attention.t27`". I had not done the reconcile. Doing it today: the hand crate is a `no_std` mirror of that spec (own `Trit`, `forbid(unsafe_code)`, a different `compute_scores` argument order), nothing in the workspace depends on it, a conversion would create a second spec, and `gen-rust` on the existing spec does not compile. Of the 9 ring crates whose header names an existing spec (13 crate-spec pairs measured), 4 produce output that compiles, and none of those 4 has the same public surface. Option A rewritten above; the lesson (check the spec that exists before naming a port) is in the skill.

## Three ways to work together in the next loop

Each is a direction for the next overnight loop and can be combined. Epic #10879 (t27b for t27o) keeps running under any of them. Where nothing is said, the default is what happens at the next tick.

A - Depth

### Delete what can be deleted, then stop at Rust

**Work** Measured on 2026-10-20: this tree has no whole file left to port (385 swept, 0 convert whole; `bitmask.rs`, the one that did, went in #11157). The ring crates I had named as the next bundle are hand mirrors of specs that already exist (`ring-090` of `specs/fpga/simulator.t27`, `ring-092` of `specs/nn/attention.t27`), so converting them would write a second spec, which the one-spec rule refuses. Replacing them with `gen-rust` output of the existing spec: `ring-090` compiles but shares 16 of 19 functions, 5 of 5 types and 0 of 8 constants with the hand crate; `ring-092`'s spec produces output that does not compile (53 errors, `static mut` buffers set to `undefined`, tests not lowered). So option A becomes a back-end track (#11232): teach `gen-rust` statics, constants and test lowering, then replace the ring crates one at a time, `ring-090` first, and port the whole files of the other four repos by the #11157 recipe.

**Measure** Hand-written Rust lines deleted per pull request, with the side-by-side comparison count and the build of the real crate as evidence, as in #11157; for the back-end track, the number of ring crates whose mirrored spec compiles and matches the hand crate's public surface (today 0 of 4 that compile).

**Default** Next tick, a read-only sweep of the other four repos for whole files; the first one found is ported. Lane L3 landed (#11240); #11235 is done (#11255, the second port); #11258 adds the ranking command, then the next near-miss files (histogram, ratelimit, night) are ported one per tick, and the ring track (#11232) follows.

B - Language

### Fix the back ends the RFC found, then pilot before any construct

**Work** The RFC (#11180) recommends building none of the new constructs now. It recommends three steps that change no language: fix the back-end defects its experiment found (a silent wrong value in `gen-c` #11181, `gen-rust` #11182, slice length in `gen-c` and unbound fields in `gen-verilog` #11183), keep the buffer forms as one shared library, and run a sized pilot (X2) of the whole package. The Python front end (#8995, #9006) waits behind it: no whole file of 150 lines or more, and `t27c gen-python` writes no function bodies.

**Measure** The pilot's end-to-end rate. The RFC's bar for building a construct is at least 15 convertible in 100; the best end-to-end rate measured so far is about 1.3 in 100.

**Default** The silent miscompile (#11181) is fixed (#11192), `gen-rust` D1 and D3 are merged (#11213) and `gen-c` D5 is merged (#11240). What is left of the RFC's list is `gen-c` D4 and `gen-verilog` D7 (#11183) and `gen-rust` D2 (#11218), both held for a ledger decision. Compiler constructs stay unstarted; "start language features" still means the pilot's winner.

C - Analog partnership

### Left out of this public copy

The report named a third option, a partnership direction built on the analog-emulation loop (#10486). It was not chosen and not started; it begins only on the owner's word. Its details are not kept in this public copy.

## A decision that is yours

Every measurement of the two days says the same thing about Rust and Python: the converted share is bounded by what t27 cannot say. The largest blocks are lists (2,813 Rust fns once cascades are followed to their root), owned strings (1,655), iterator chains with closures (588), tuples and paths. The RFC the loop wrote on this (#11180) reaches a plain answer: at the rates measured so far, none of the five constructs it costed would pay for itself, and it names the bar that would change that, a pilot rate of at least 15 in 100. So the default is not to build any construct. If you want one anyway, say "start language features" and the implementation starts from the RFC's option (a), the caller-sized buffer, because it reuses what the Zig back end and `t27b` already do. Meanwhile the loop fixes the back-end defects the RFC found and deletes what already converts.

A second decision, smaller and more concrete: **may the suite ledger rise by 66 rows** (69 to 135) so that `gen-rust` refuses 73 specs it now turns into Rust that rustc rejects (#11218)? The refusal is more honest, since today the command exits 0 and the file does not compile, but ledgers have only moved down. My default is no: the draft stays held. Say "accept ledger rise" and it merges as is, or the 73 specs can be fixed or ported instead.

Same kind, found by the lane that fixed #11240 (#11183, defect D7): `gen-verilog` writes the fields of a pointer or slice parameter as names nothing declares. Refusing such a spec would newly refuse 608 specs (about 600 ledger rows); skipping only the function rewrites the Verilog and seals of the same 608. The code and its test are held in a comment on #11183. My default is no, and nothing changes. The slice-length defect D4 in the same issue is measured: refusing slices in C would add 898 rows, a length for every slice changes 6,467 signatures, and the lane recommends a length only where the spec reads `.len` (at most 661 specs, at most 2 committed files); that is a design choice the next lane takes up unless you say otherwise.

## What this report does not claim

Converted means the converter wrote a body; golden means equal on recorded vectors, not a proof for all inputs. "Runs natively" means t27b passes the spec, and its agreement with t27c was checked on the 50 newly passing specs named above, not on every spec. The coverage target of 100% was reached for the functions of `rust_convert.t27`, not for the code on the roadmap: the Rust converted share is flat (9.93% at 04:20, 9.88% tonight on the same measure), and the converter fixes of the day (#11064 and #11109) moved it by 4 fns. The Python, TypeScript and Zig figures count functions inside a subset we defined, except where a front end ran.
