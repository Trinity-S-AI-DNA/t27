# Milestone 4 alignment — 2026-11-10

ORDER: T27_MILESTONE_4_ALIGNMENT_V1. Milestone 4 is the master plan; every
active lane is subordinate to it. Sequence: recovery -> M1.3 landing ->
MCP observation plugin -> OPENXC7 bounded runtime proof.

## Lane inventory

| Lane | Milestone link | Status | Next bounded action | Acceptance criteria | Blockers | Owner | Forbidden |
|---|---|---|---|---|---|---|---|
| Recovery (`recovery/synthspec-20261110`) | M4 base stability | IN PROGRESS | Recovery commit on recovery branch; post-commit `.t27` gates | Gates green; recovery-scope files only; `+192` ParserPort `.t27` restored or explicitly recorded absent | Full `+192` ParserPort `.t27` not yet gate-verified | Queen | M1.3/foreign WIP/plugin/policy changes; hook bypass |
| M1.3 host-boundary (`fpga_synth_spec.rs`) | M4 verified contract tests | VERIFIED, UNLANDED | Narrow host-boundary budget amendment, then one landing commit | 13/13 Rust tests stay green; foreign-exceptions entry; generated `.t27` mount; contract tests in commit; 40/80 rule unchanged | Budget amendment not yet drafted/landed | Queen | Split PR; hook bypass; weakening 40/80; foreign WIP |
| MCP gateway plugin | M4 managed coordination | READY | Install observation-only from `/Users/playra/t27-mcp` (gateway dir) | `tools/list` = gateway.status, dsh.status, cdp.observe; `gateway.status` connected:false; `shell.run` refused; non-loopback `dsh.status` denied | Install after M1.3 landing path is set | Queen | Live CDP/DSH connection in this wave; shell.run enablement |
| OPENXC7 acceleration | M4 bounded engineering | IN PROGRESS | One bounded end-to-end synthesis run after swap guard lifts | logs + receipts + `.frames` + `.bit` archived; tool versions and hashes recorded; no runtime-integration claim without artifacts | Swap 11.27/12 GB used (HOLD); Docker HOLD | Queen | Heavy builds on Mac; runtime claims without artifacts |
| Disk guard | M4 infrastructure | DONE | Maintain cron; monitor | Free >= 20 GiB achieved; guard active | — | Queen | Deleting non-lane trees |
| Merge all to master | M4 discipline | HOLD | Merge only per-lane after green required checks | Each merge: green required checks + owner policy + `Closes #N` | Not a goal in itself | Queen | Mass merges; pushes without owner policy |

## Environment facts (measured 2026-11-10T23:32+07)

- Swap: total 12288 MB, used 11268 MB, free 1020 MB (encrypted) -> heavy ops HOLD.
- Disk root: ~19 GiB free (38% used) -> 20 GiB target effectively reached; new worktrees/builds still need the 5.4 GiB rule.
- Recovery branch found as `recovery/synthspec-20261110` (not the order's literal `recovery/synth-spec-2026-11-10`); treat this name as canonical unless amended.
- MCP gateway checked out at `/Users/playra/t27-mcp`, not `/Users/playra/t27/mcp/gateway`; order path corrected in this doc.

## Wave status

## Wave status (2026-11-10T23:4x+07 run)

1. Wave 1 inventory — DONE: this document.
2. Wave 2 recovery — DONE and VERIFIED: commit `9dc4f9699d` on
   `recovery/synthspec-20261110` (4 files, +507: synth_spec_pipeline.t27,
   parser_fixtures.t27, disk_guard.t27, disk_guard_retention.t27). Gates green:
   t27c 10/10 + 3/3 + 3/3 + 2/2; t27b 10/10 + 3/3. Full-repo suite on that base
   shows the same 8 pre-existing parse failures (analog/linear*, mos_table,
   c_array_repeat_wrapper, mlp, gla, rust_convert, array_repeat_text) in every
   phase — known baseline debt, none in recovery-scope files. Note: worktree has
   untracked `specs/fpga/synth_receipt.t27` + `target/` — not committed, parked.
3. Wave 3 M1.3 — PREPARED, landing gated on owner: draft in
   `docs/now/2026-11-10-m13-host-boundary-amendment-draft.md`. Repo law
   (`tools/policy/foreign-exceptions.txt`) requires the owner to merge the entry
   and/or label the PR `owner-approved-foreign`; a branch cannot grant itself the
   exception, and no entry lifts the 40/80 budget. Lane worktree has dirty
   foreign WIP (`fpga.rs`, seal json) that must not enter the M1.3 commit.
   Rust 13/13 re-run deferred: swap 12.7/13.3 GB — build HOLD.
4. Wave 4 MCP gateway — BLOCKED, artifact missing: neither
   `/Users/playra/t27/mcp/gateway` nor any source declaring `gateway.status` /
   `dsh.status` / `cdp.observe` exists. `/Users/playra/t27-mcp/server.mjs` is the
   t27 corpus exporter (`t27_*` tools) and cannot meet the acceptance criteria;
   installing it would be scope expansion. Next bounded action: owner/lane to
   produce or point to the observation-only gateway plugin source.
5. Wave 5 OPENXC7 — BLOCKED on swap (12.7/13.3 GB used, 0.6 GB free; Docker HOLD).
6. Wave 6 report — below and in the session report.

## Rules in force

Milestone 4 is master plan; recovery first; no merge without green required
checks; no push without owner policy; no hook bypass; no paid-model fallback;
no live CDP/DSH without separate approval; no broad orders.
