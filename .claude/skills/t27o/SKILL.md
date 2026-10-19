---
id: t27o
name: t27o
description: Change, test and measure t27o (Ouroboros), the converter of foreign code into .t27 (Rust front end today, SPICE beside it for #10486). Use when a refusal class should become convertible, when quoting t27o coverage, when running `tri t27o --check`, or when a loop tick works epic #10522.
argument-hint: "[refusal class | measure | check <file.rs>]"
---

# /t27o -- foreign code into .t27, measured

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## Where the truth lives

| What | Where |
|---|---|
| The Rust front end: what converts, what is refused and why | `specs/tools/t27o/rust_convert.t27` (header comment, lines 1-110) |
| Rust std calls mapped to existing t27 fns | `specs/tools/t27o/rust_std.t27` |
| What a run's evidence is worth; which runner's exit code is a verdict | `specs/tools/t27o/ledger.t27` |
| Golden criteria, the service, the installer | `golden.t27`, `service.t27`, `install.t27` in the same directory |
| The SPICE front end (analog M2a) | `specs/tools/t27o/spice_convert.t27`, #10509 -- owned by the analog loop |
| The Zig front end (`tri t27o <file>.zig`; the file's own `test` blocks, translated, are the oracle) | `specs/tools/t27o/zig_convert.t27`, #10597, step 1 #10765 |
| Coverage over a tree, reused by seal keys (`tri t27o-sweep`) | the rules in `specs/tools/t27o/ledger.t27` (#10543, `--repo` #10638) |
| Baseline numbers, the field, ten weaknesses | `docs/nona-03-manifest/T27O-COMPETITORS-AND-DOGFOOD.md` |
| The overnight loop's state | epic #10522, its "Loop ledger" comment |
| The CLI glue (reads the file, runs rustc, t27b and t27c, writes) | `cmd_t27o` in `cli/tri/src/main.rs`; every rule it applies is gen-rust of the specs above |

## Make a refusal class convertible

1. Find it in the numbers, not by intuition: the first-refusal table in the doc above, or a fresh measurement (below). Pick the class with the most first refusals that needs no new type in the subset.
2. Reproduce on a real file: `./target/release/tri t27o <file.rs> | grep -n "refused fn <name>"`.
3. Write the test first, in `rust_convert.t27`: the tests near the end call `body(...)` / `convert(...)` on a Rust text and compare the output text. Add the rule; add a negative control (a near-miss form that must stay refused, or a mutated copy that must fail).
4. Inner loop: `./target/release/t27b test specs/tools/t27o/rust_convert.t27` (about 130 tests in a few seconds since #10631). The final gate once: `t27c test-report` on it, `vacuous passes` must be 0.
5. Regenerate, never hand-edit: `./target/release/t27c gen-rust specs/tools/t27o/rust_convert.t27 > gen/rust/tools/t27o/rust_convert.rs`, check it compiles with `rustc --edition 2021 --crate-type lib`, then `t27c seal ... --save` and `--verify`.
6. Measure the gain on the same files and commits, before and after, and put both numbers in the PR.

DRY: a std call goes through `rust_std.t27`, which names an existing t27 fn with the same I/O; search `t27c dupes --name <fn>` before writing a new one.

## Measure coverage

- One file: the last line of `tri t27o <file.rs>` is its count: `t27o: N fn seen, M converted (a of b tests), R refused (S kept as stubs); coverage P%`.
- Many files: `tri t27o-sweep [PATHS...] [--repo <dir>] [--max-bytes N] [--cache FILE] [--top K]` converts every tracked `.rs` file in-process, prints one row per file, the totals with the coverage share, the root refusal kinds and the cascade apart. A row is reused while the file's SHA-256 and the `spec_hash` of the converter's seals hold, so a re-run after a converter change re-converts everything and a re-run without one takes well under a second. `--repo` sweeps another checkout (one cache per repository); run it from a t27 checkout, which supplies the seals.
- Since #10631 the converter is close to linear in the file: all 1,388 Rust files of the five roadmap repositories in 26 s, `bootstrap/src/compiler.rs` in 1.4 s. A file that takes seconds again is a regression: profile it with `cmd & pid=$!; sample $pid 3 -file out.txt` (macOS; `ps` and `pgrep` are blocked in the sandbox).
- Count first refusals as what they are: the cascade (`uses refused ...`) is 13% of them and moves when a root refusal is fixed, so rank work by root reasons.
- Quote a number only with its repository, commit and binary.

## `tri t27o --check`

`--check` builds the oracle harness with rustc, writes `specs/t27o_check/<stem>.t27` with the oracle's vectors as tests, and judges it. Since #10525 it runs `t27b test` first and `t27c test-report` only when t27b's exit code is not a verdict (`t27b_outcome` in `ledger.t27`); the fallback says which code caused it. Golden means equivalent on the recorded vectors, never a proof for all inputs.

For a `.zig` file there is no rustc oracle: the converted spec carries the file's own tests, so judge it with `t27b test` on the spec (`--check` for Zig is #10823).

## The loop tick (epic #10522)

1. Read the ledger comment on #10522; `CronList` (delete duplicates after a restart); list running subagents. A lane a row calls running is not started again. The analog loop keeps its own ledger on #10486 and owns #10509 and `specs/analog/`: never start work there.
2. Anomalies before new work: open PRs of the epic that are red, conflicting or stale; a lane that reported done with no PR (`git ls-remote origin | grep t27o`); a seal that no longer verifies.
3. Sonnet 5.5 plans, implements and reviews (`specs/claw/model_route.t27`, #10929, owner 2026-10-19); Opus only after two failed attempts or on the owner's word. Two lanes at most from this loop while the analog loop runs on the same 8 GB Mac; one lane at a time edits `rust_convert.t27`.
4. Review before auto-merge, on the PR head: `t27b test` and once `t27c test-report` (vacuous 0) on every touched spec; `t27c seal --verify`; regenerate each gen file and compare byte for byte; `python3 -I tools/dupe_scan.py` and `--like <spec>` (bodies -- `t27c dupes --name` only finds names; #10801 passed the name check with 13 copied bodies); the own-language budget compiled from the merge base (below); `python3 -I tools/check_devhome_paths.py` (#10646 brought nine `/Users/...` test literals); `tri types ratchet` when the diff declares a struct or enum (a fixture type named `Gate` clashed with crm-sellers' `Gate`: the types ratchet counts a second definition of a name as a conflict, so name fixture types uniquely); `cargo check -p tri`. A red is inherited only if the PR's failures are a subset of master's: run `dupe_scan` on the PR head AND on its merge base and `comm -23` the findings (against current master the base drift shows up as false extras). A spec that `use`s `rust_convert.t27` (spice, zig) carries it in its seal closure: re-seal it when rust_convert changes.
5. Merge only green, then auto-merge (GraphQL `enablePullRequestAutoMerge`). It refuses with "in unstable status" while a NON-required check is red; when the four required checks (validate, check-linked-issue, parse-ratchet, own-language) are green and every red is master's, merge with `PUT /repos/gHashTag/t27/pulls/N/merge` and `{"merge_method":"squash","sha":"<the head you reviewed>"}` (the sha makes a late push fail the merge instead of slipping in). Turning auto-merge off does not hold a PR -- any actor with the token can re-arm it (#10801 merged five minutes after it was switched off). To hold a PR, convert it to a draft (`convertPullRequestToDraft`); `markPullRequestReadyForReview` when it may go.
6. Stacked PRs and squash merges: after the base merges, `git rebase --onto origin/master <old base tip>`, check `git diff <reviewed head> HEAD -- <converter files>` is empty, push with `--force-with-lease=<branch>:<old sha>`; auto-merge survives a force push.
7. A t27b change that makes ledger-blocked specs pass, or moves a blocker, edits `docs/reports/t27b_expectations.json` in the same PR (the rows and `max_not_pass` recounted; precedents #10862, #10999). The steward applies UNEXPECTED PASS and UNLISTED from a master run itself but refuses MOVED; a new spec's UNLISTED row is the steward's.
8. Edit the epic ledger row last; add a trap here when it cost more than one try.

## Hand-written glue must net to zero

The required check `own-language` (`check_budget` in `specs/policy/own_language.t27`) fails a pull request when the counted hand-written foreign files gain more lines than they lose over the whole diff -- on top of the 40-per-file / 80-per-PR cap. Specs and `gen/` do not count. A `#[path]` mount plus a few lines in `cmd_t27o` therefore needs an equal deletion in the same diff: dead code in `cli/tri/src` (`cargo check -p tri` lists it), or hand logic moved into a spec and replaced by a call. Check before the first push:

```bash
BASE=$(git merge-base origin/master HEAD)
git show "${BASE}:gen/c/policy/own_language.c" > $TMPDIR/gate/gate.c   # zsh: write ${BASE}:, never $BASE: (the :g modifier)
printf '#include "gate.c"\n#include <stdio.h>\nint main(void){static char b[1<<22],o[1<<22];unsigned n=check_budget((uint8_t*)b,fread(b,1,sizeof b,stdin),(uint8_t*)o,sizeof o-1);fputs(o,stdout);return n!=0;}\n' | cc -w -I $TMPDIR/gate -x c -o $TMPDIR/gate/budget -
git diff --numstat --no-renames "${BASE}...HEAD" | $TMPDIR/gate/budget; echo rc=$?   # must print rc=0
```

This is the gate CI runs, compiled from the base revision. Most "never used" items in `cargo check -p tri` have test callers; truly dead code is rare, so fold glue (one dispatch branch for several front ends) before hunting deletions, and agree with peer sessions on which dead lines each PR deletes.

PR #10538 failed it with +9 / -0 at 1e20cfb34.

## Traps met on this Mac

- `.claude/skills/**` is not writable from the session's own worktree in the sandbox; a checkout that touches it half-applies (index changed, file unchanged). Work in a separate worktree: `git worktree add --no-track -b <branch> $TMPDIR/t27o-loop/wt-x origin/master`, then `git -C $TMPDIR/t27o-loop/wt-x config --worktree core.hooksPath .githooks`.
- The session scratchpad is wiped when the session resumes (2026-10-18 23:53: lane worktrees, the cargo target and raw measurements gone; pushed branches survived). `$TMPDIR` survived: keep worktrees and data under `$TMPDIR/t27o-loop/`, and commit + push after every working step.
- `--no-track` matters: the shared `.git/config` cannot be locked from the sandbox, so `-B` with tracking fails after creating the branch.
- `gh` fails on TLS in the sandbox: use the REST and GraphQL APIs with `curl -H "Authorization: Bearer $(gh auth token)"`; write bodies to a file and build JSON with `python3`, never with backticks inside double quotes.
- Auto-merge by API: the GraphQL mutation `enablePullRequestAutoMerge` with the PR's `node_id` and `mergeMethod: SQUASH`. The timeline API shows no AutoMergeEnabledEvent for an API-enabled auto-merge, so a timeline cannot prove who re-armed a PR.
- Building `tri` on master: when `Cargo.lock` pins a crate the sandbox cannot write into `~/.cargo`, clone the registry (`cp -c -R ~/.cargo/registry $TMPDIR/cargo-home/registry`, an APFS clone that costs no space), `CARGO_HOME=$TMPDIR/cargo-home cargo fetch`, and build with that `CARGO_HOME`. Give each lane its own `ZIG_LOCAL_CACHE_DIR`: two lanes sharing one get `BLOCKED ... FileNotFound`.
- zsh: `echo "$json"` expands `\n` inside JSON strings, so write API responses to a file and parse the file; `echo =====` runs `=` expansion; text with backticks goes through a quoted heredoc, never into a double-quoted argument (it runs the command).
- gen-rust's `use` splice matches words inside string literals and can pull a whole module into the generated file (#10824): after regenerating, look at the gen file's size.
- A trailing `&` inside a `run_in_background` Bash call dies at once (`nice(5) failed`): run the command itself in the background. `rc=$?` after `| tail` is tail's exit code, not the tool's.
- The sandbox cannot delete `.git/worktrees/<name>` (`git worktree remove` and `prune` fail on the admin entry, the directory itself goes): remove only your own entry through the Terminal tab, `rm -rf "$(git rev-parse --git-common-dir)/worktrees/<name>"`. It cannot write `~/.claude/projects` from Bash either (use the Edit tool for memory files).
- The issue-shape gate (`.github/workflows/needs-boundary.yml`) closed 185 issues on 2026-10-18/19, 128 of them wrongly (it judged a queued run's event payload; fixed by #11008). An issue "closed as not planned" with the label `shape-refused` proves nothing: compile `gen/c/policy/issue_shape.c` as the workflow does and run the issue's current body through it before accepting the close (#11007).
- Each lane worktree costs about 320 MB; remove finished ones (`git worktree remove --force`) and keep 8 GB free.
