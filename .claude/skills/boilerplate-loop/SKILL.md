---
id: boilerplate-loop
name: boilerplate-loop
description: Drive epic gHashTag/t27#12444, boilerplate and spaghetti in .t27 -- a std layer (std::text, std::mem, std::signmag), migration of copied helpers onto it, a ratchet that sees renamed clones, and a claim protocol so several agents never do one row. Use when a tick of the boilerplate loop starts, when you are about to write a helper such as is_digit or a byte-equality loop, when a duplicate-bodies ratchet is red, or when a row of the epic Dashboard is yours.
argument-hint: "[row letter A-J] | census | dashboard"
---

# /boilerplate-loop -- one bounded tick against copied helpers

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## Where the truth lives

- The map and the Dashboard: the epic gHashTag/t27#12444. Rows A-J are its sub-issues; the Dashboard table is rewritten by each tick.
- The competitor survey and the measured numbers: `docs/reports/boilerplate_competitors_2026-10-25.md`.
- The cron card (the contract of the tick): `specs/crons/t27os-boilerplate-loop.t27`.
- The law for imports: `specs/compiler/use_splice.t27`; the code that splices: `bootstrap/src/use_resolve.rs`.

## The tick (observe, claim, one batch, verify, record)

1. **Observe.** Read the epic Dashboard and the last `DONE:` / `CLAIM:` comments. Run `tri dupes --name <fn>` before writing any helper: the copy you are about to write may exist.
2. **Claim.** `tri loop claim boiler-<sub-issue>`. Exit 0: yours. Exit 1: someone holds it, take another row. Exit 2: the claim could not be attempted (no network, no ssh), say so; do not assume you hold it. Post ONE comment on the sub-issue starting `CLAIM: in progress` with Who, Branch, Started (UTC), Done so far, Next, Blocked on, Files held, and edit it at every checkpoint.
3. **One batch.** One row, one bounded batch (at most 10 specs migrated). Two sub-issues whose Boundary names the same file run one after the other.
4. **Verify.** For every touched spec the `t27c test-report` counts (pass, fail, vacuous) equal the counts before the change. Take the baseline from `git show <base>:<path>`, never from HEAD after a commit. Run it through the Terminal tab, not the Mac Bash tool.
5. **Seal and ratchet.** `t27c seal <spec> --save` from a master-built t27c, then `grep -l /Users .trinity/seals/<seal>`. `python3 tools/dupe_scan.py --bless` only when the ledger diff is deletions only.
6. **Record.** Edit the claim comment to `DONE:` with the PR link, release the claim (`tri loop claim boiler-<n> --release`), rewrite the epic Dashboard row, end the tick with a `Requests out:` line (who, what, sent when, default, due).

A claim with no edit for 6 hours and no push on its branch is stale. Another agent posts `TAKEOVER:` quoting the stale claim and takes the row; the old comment is never deleted.

## Self-corrections (each one cost something; do not repeat it)

- **A test-count drop is a revert, not a finding.** A migration that loses a test (a `use` that resolves to nothing, a local test deleted with its helper) is undone and the row is rewritten.
- **Never migrate a Verilog-bound spec before #8769 closes.** `gen-verilog` does not splice `use`d functions (`run_gen_verilog` in `bootstrap/src/main.rs` skips `use_resolve::resolve`): the function is dropped and the command exits 0. A corpus ratchet row can then flip to a false "unexpected pass" (PR #9636). Specs with `on_comb` / `on_clock`, and everything under `specs/ternary/` GF-T, wait.
- **The duplicate-bodies key includes parameter names.** `is_digit` is defined 36 times in 12 text variants and the ratchet reports no group. A clean `--report` does not mean no copy: search by name with `grep -rn -A3 -E '^(pub )?fn <name>' specs`.
- **Never `--bless` a NEW group.** Bless only a deletions-only diff (a duplicate body removed makes the ledger go red as "gone", which is legitimate). A growth you cannot explain is a defect, not a baseline.
- **The long tail, not a head.** The top 10 groups hold 9 percent of the duplicate bodies, the top 100 hold 25 percent. A library that stops new copies beats chasing the biggest group; do not spend a tick on one big group unless it unblocks something (the GF-T family is blocked on #8769).
- **A migrated variant may differ in behaviour.** Compare the bodies first (`is_digit` that also accepts `_`, `is_space` that skips `\r`). A different variant stays, and goes in the PR body.
- **`Closes` only for your own sub-issue.** Every commit carries `Refs #12444` or `Closes #<sub-issue>` (the L1 check greps each commit; "see #N" and "Part of #N" fail). The epic never closes by keyword.
- **A master red may already have a fix.** Search open PRs and issues before filing, and compare the run's creation time with the commit's merge time (#12376 was mis-filed).
- **Mac rules.** No `t27c suite`, `gen-*` sweeps, `seal` sweeps or builds on the Mac through Bash (the hook denies them; never use `MAC_HEAVY_OK`). One spec at a time through the Terminal tab, load below 40 and more than 5.4 GiB free (`df -g .`). Scratch lives in the gitignored `target/boiler/` of the worktree, never in `/tmp`.
- **A same-named helper with another body anywhere in the import closure BLOCKS the spec.** `t27os/share.t27` imports `t27os::session_store`, which declares a lower-case-only `is_hex`; adding `use std::text::is_hex` gave `use of undeclared identifier 'is_hex'` (rule R4). Before a `use`, grep the importer's whole closure for the same name; the importer keeps its local copy and a comment says why.
- **Shape tests keep their copies by design.** `compiler/codegen_conformance.t27` carries `is_digit` as a gen-rust char-literal shape test (#8971); a migration there removes the test's subject.
- **Tracked-gen callers wait for a verified regen.** A spec that feeds `gen/` (six hand crates mount `gen/rust/...` through `#[path]`; `spec-guards.yml` regenerates `gen/c/claw/*` and runs `git diff --exit-code`) is migrated only after the local t27c reproduces the committed generated file byte for byte, and the file is re-added with `git add -f`.
- **Parameter type separates variants.** `u8`, `u32` and `i64` `is_digit` are different functions (isa/t27a, compiler/core/t27core); migrate only the `u8` ones.
- **A new library body must not clone an old one.** `std::text::is_hex` first written as `is_digit(b) or ...` equalled `tri/loop/lesson.t27`'s body and the ratchet went red (`is_hex was copied [2] and is now copied [2, 2]`). Write it through the sibling (`hex_value(b) >= 0`) until the old copy is migrated.
- **A red you did not make is not yours to bless.** `put_lit [2, 2]` (tri/handoff/handoff.t27 vs tri/trigit/verbs.t27 `put_s`) came in with master; list it in the PR body, leave the ledger alone.
- **Terminal tabs.** Each `run_in_terminal` call opens a new tab and the hook refuses a 7th: close finished ones with `stop_terminal_tab {close: true}` and set every variable in the same command (`T=...; $T ...`).
- **zsh and gh.** Quote globs (`--include='*.t27'`), brace `${b}` before a colon, use `--body-file` for `gh` (a backtick in a double-quoted body runs), retry `gh` on `TLS handshake timeout`, and use `gh api repos/gHashTag/t27/issues?...` when `gh issue list --search` returns `[]`.

## Commands

| need | command |
|---|---|
| does a helper of this name exist | `tri dupes --name <fn>` and `grep -rn -A3 -E '^(pub )?fn <fn>' specs` |
| duplicate census | `python3 tools/dupe_scan.py --report` |
| counts before and after | `t27c test-report <spec>` (Terminal tab) |
| seal one spec | `t27c seal <spec> --save` (master-built t27c) |
| claim, who, release | `tri loop claim boiler-<n>` / `--who` / `--release` |

## Rows

A std::text and the is_digit migration. B std::mem. C #8769 (the unlock). D std::signmag (blocked on C). E alpha-rename-proof body key. F `tri boiler` and `tri dupes --like`. G loop-shape census. H import shortcut design. I this loop (card, skill, protocol). J the remaining `gen_w` `build_tree` copies. The sub-issue numbers are in the epic Dashboard.
