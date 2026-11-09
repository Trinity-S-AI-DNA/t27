# Gitflow merge campaign: three shared master blockers, one experience note

2026-11-05T00:58Z | t27 merge lane | A 22-PR merge campaign (2026-11-01 .. 11-03)
cleared the queue by separating **master-wide failures** from **per-PR work**,
and every fix that recurs became a rule. This note records the lessons with
their evidence, so the next campaign does not re-measure them.

## The three shared-blocker classes (master red, every PR inherits)

- **Seal-drift** -- a spec edit lands without a re-seal from a master-built
  `t27c`, so one stale seal fails `coverage` and `spec-guards` on every open PR.
  Seen: `specs/policy/own_language.t27` (#13061 -> #13096). Fix: `t27c seal
  <spec> --save` from a master-built compiler, never `--update-baseline`.
- **Gen-drift** -- an emitter fix changes generated output without re-sealing;
  the same two checks name the specs, and the committed `gen/` artifact can be
  behind too (release.rs missed `F_LIVE_DEV`, #13152). Fix: re-seal AND
  regenerate the committed artifact (`git add -f`, the #11324->#11328 lesson).
- **Ledger/ratchet drift** -- a type rename resolves a conflict but leaves the
  name in `docs/reports/type_conflicts.json` and its verdict in
  `type_conflicts_classified.json`, so the identity-keyed ratchet reads a
  resolved-name failure and `Corpus Ratchet` goes red master-wide (#13096 ->
  #13166). Fix: sweep both files in the same commit; the law is now
  `specs/claw/type_conflict_sweep.t27`.

## The unblock is a wave, not a nudge

After a shared blocker lands, mass `gh pr update-branch` re-bases the whole
queue onto the repaired master; the `CLEAN` wave then arrives and a single
sweep merges 12 PRs in minutes. The reverse also holds: a red that a
full re-run reproduces is the PR's own, not a transient. `specs/claw/red_origin.t27`
already encodes "whose red is it"; this campaign exercised it as the go-to first step.

## Gates that corrected the work

- **The full compiler refused what t27b's loose cross-module check passed**
  (`put_bytes` arity 4 vs 5) -- the seal gate caught a real regression that
  native tests missed (#13137). Trust the compiler over the native runner.
- **dupe ratchet is right about reuse**: reshaping a copied loop still trips it;
  the fix is to `use` the canonical function (#13159 -> `std::mem::bytes_eq`).
- **foreign-budget is net-negative**: a hand-written `.py` edit must delete
  more than it adds; move history narration to the issue, not the file (#13152).

- Outcome: 5 repair PRs merged (#13096, #13152, #13159, #13166, #13137), each 33/33 green;
  3 issues closed (#13094, #13151, #13144) + #13131; master Seal Coverage / Spec Guards /
  Corpus Ratchet green in base; ~22 PRs merged total; `type_conflict_sweep.t27` added to
  make the ledger-sweep rule a spec instead of a memory.
