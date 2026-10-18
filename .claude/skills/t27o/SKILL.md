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
| Baseline numbers, the field, ten weaknesses | `docs/nona-03-manifest/T27O-COMPETITORS-AND-DOGFOOD.md` |
| The overnight loop's state | epic #10522, its "Loop ledger" comment |
| The CLI glue (reads the file, runs rustc, t27b and t27c, writes) | `cmd_t27o` in `cli/tri/src/main.rs`; every rule it applies is gen-rust of the specs above |

## Make a refusal class convertible

1. Find it in the numbers, not by intuition: the first-refusal table in the doc above, or a fresh measurement (below). Pick the class with the most first refusals that needs no new type in the subset.
2. Reproduce on a real file: `./target/release/tri t27o <file.rs> | grep -n "refused fn <name>"`.
3. Write the test first, in `rust_convert.t27`: the tests near the end call `body(...)` / `convert(...)` on a Rust text and compare the output text. Add the rule; add a negative control (a near-miss form that must stay refused, or a mutated copy that must fail).
4. Inner loop: `./target/release/t27b test specs/tools/t27o/rust_convert.t27` (66 tests in about 14 s). The final gate once: `t27c test-report` on it, `vacuous passes` must be 0.
5. Regenerate, never hand-edit: `./target/release/t27c gen-rust specs/tools/t27o/rust_convert.t27 > gen/rust/tools/t27o/rust_convert.rs`, check it compiles with `rustc --edition 2021 --crate-type lib`, then `t27c seal ... --save` and `--verify`.
6. Measure the gain on the same files and commits, before and after, and put both numbers in the PR.

DRY: a std call goes through `rust_std.t27`, which names an existing t27 fn with the same I/O; search `t27c dupes --name <fn>` before writing a new one.

## Measure coverage

- One file: the last line of `tri t27o <file.rs>` is its count: `t27o: N fn seen, M converted (a of b tests), R refused (S kept as stubs); coverage P%`.
- Many files: the same command per file, with a time cap, each output kept, then the counts summed and the `// t27o: refused ... : <kind> \`<name>\`` lines tallied. Cap every file (`perl -e 'alarm 30; exec @ARGV' ./target/release/tri t27o <f>`): run time grows faster than file size, and `bootstrap/src/compiler.rs` (1.9 MB) does not finish in 5 min.
- Count first refusals as what they are: the cascade (`uses refused ...`) is 13% of them and moves when a root refusal is fixed, so rank work by root reasons.
- Quote a number only with its repository, commit and binary.

## `tri t27o --check`

`--check` builds the oracle harness with rustc, writes `specs/t27o_check/<stem>.t27` with the oracle's vectors as tests, and judges it. Since #10525 it runs `t27b test` first and `t27c test-report` only when t27b's exit code is not a verdict (`t27b_outcome` in `ledger.t27`); the fallback says which code caused it. Golden means equivalent on the recorded vectors, never a proof for all inputs.

## The loop tick (epic #10522)

1. Read the ledger comment on #10522; `CronList` (delete duplicates after a restart); list running subagents. A lane a row calls running is not started again. The analog loop keeps its own ledger on #10486 and owns #10509 and `specs/analog/`: never start work there.
2. Anomalies before new work: open PRs of the epic that are red, conflicting or stale; a lane that reported done with no PR (`git ls-remote origin | grep t27o`); a seal that no longer verifies.
3. Opus plans and reviews, Sonnet implements (`specs/claw/model_route.t27`). Two lanes at most from this loop while the analog loop runs on the same 8 GB Mac; one lane at a time edits `rust_convert.t27`.
4. Merge only green; enable auto-merge after reading the diff. To change a PR under auto-merge, disable it, push, enable it again.
5. Edit the ledger row last; add a trap here when it cost more than one try.

## Traps met on this Mac

- `.claude/skills/**` is not writable from the session's own worktree in the sandbox; a checkout that touches it half-applies (index changed, file unchanged). Work in a worktree under the scratchpad: `git worktree add --no-track -b <branch> $SCRATCH/wt-x origin/master`, then `git -C $SCRATCH/wt-x config --worktree core.hooksPath .githooks`.
- `--no-track` matters: the shared `.git/config` cannot be locked from the sandbox, so `-B` with tracking fails after creating the branch.
- `gh` fails on TLS in the sandbox: use the REST and GraphQL APIs with `curl -H "Authorization: Bearer $(gh auth token)"`; write bodies to a file and build JSON with `python3`, never with backticks inside double quotes.
- Auto-merge by API: the GraphQL mutation `enablePullRequestAutoMerge` with the PR's `node_id` and `mergeMethod: SQUASH`.
