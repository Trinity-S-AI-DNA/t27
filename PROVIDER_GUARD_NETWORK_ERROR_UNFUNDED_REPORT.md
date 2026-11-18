# Provider Guard: NETWORK_ERROR vs PROVIDER_UNFUNDED separation

**Status:** CONDITIONAL_GREEN (owner-corrected verdict, machine-verifiable)
**Date:** 2026-10-30
**Trigger:** relay stream ended with wire `finish_reason: network_error`; classifier
fell into the `PI_AI_ERROR` catch-all and the turn hard-failed with zero retries.
**Live follow-up:** the z.ai E2E ping then surfaced the SAME class in a second
form: HTTP 429 + error code 1113 "Insufficient balance" - i.e. the provider
State, not the status code, must own the classification.

## Verdict (machine-verifiable, no narrative)

```text
ENGINEERING_GATES (guard):            GREEN  - typecheck exit 0; cargo 13/13 exit 0;
                                                intercept + E2E evidence below
PROVIDER_GUARD_CLASSIFICATION:        GREEN  - spec seal --verify: MATCH, exit 0
LIVE_ZAI_STATE:                       UNFUNDED CONFIRMED (429 + code 1113, 1 req)
FULL_REPO_SEAL_BASELINE:              RED    - t27c suite exit 1 (34 pass / 520 fail)
ROLLBACK_DRILL_SCRIPT:                EXISTS - scripts/rollback-drill.sh, PASS 5/5, exit 0
CLEANUP_REFERENCES:                   NOT_APPLICABLE - repo-wide search evidence below
PROVIDER_GUARD_RELEASE:              CONDITIONAL_GO (waiver DRAFT, owner approval pending)
FULL_REPO_RELEASE:                    NO_GO (release rule, bottom of file)
```

## Remaining red items (honest, unchanged)

1. **FULL_REPO_SEAL_BASELINE stays RED until either the owner-approved
   waiver (`.trinity/seal-waivers/2026-10-30-codegen-drift.md` + 520-line
   inventory) is signed, or an owner-run re-seal sweep makes
   `t27c suite` exit 0. Gate is counted as failed regardless of the
   pre-existing nature of the drift.
2. Cleanly-NOT earlier blockers now resolved:
   - rollback drill now EXISTS and exits 0 (was missing);
   - cleanup items searched and marked NOT_APPLICABLE with evidence.

## Incident classification (owner summary, confirmed)

| | NETWORK_ERROR | PROVIDER_UNFUNDED |
|---|---|---|
| Nature | transient transport/relay failure | funding/config problem |
| Recovery | bounded retry with backoff (max 3) | automatic retry = forever loop |
| Follow-up task | no (self-recovers) | yes (owner must fund/fix config) |
| Checkpoint | preserved | preserved |

Reference: deepseek-ai/deepseek-harness discussion #4361 - the pi-ai message
`Provider finish_reason: network_error` defeated every retry branch because
`\bnetwork\b` cannot match `network_error` (`_` is a \w character, so there is
no word boundary between `k` and `_`).

## What was changed

### 1. SSOT spec: specs/system/llm_request_provider_gate.t27
Response-side half of the rule (previously only preflight existed):

- `WIRE_OTHER / WIRE_NETWORK_ERROR / WIRE_UNFUNDED` -> `map_wire_signal()`
- `CLASS_TRANSIENT_NETWORK / CLASS_UNFUNDED / CLASS_TERMINAL` policy classes
- `retry_allowed_for_class`, `retry_limit_for_class` (3 vs 0),
  `network_io_needed_for_class`, `follow_up_task_required_for_class`,
  `checkpoint_preserved_for_class`, `MAX_TRANSIENT_RETRIES = 3`
- 4 new tests + 5 new invariants, including
  `classes_never_collapse_into_each_other` and `only_transient_class_is_retryable`
- Token spellings live in transport glue BY DESIGN with an explicit table
  (never a bare `\bword\b` branch; #4361). Live unfunded wording locked:
  "insufficient balance" / "no resource package" (z.ai 429+1113, 2026-10-30).

### 2. Runtime: contrib/backend/agent-runner/src/api.rs
- Fixed the broken uncommitted P0 patch (5 compile errors).
- `classify_stream_termination(text) -> STREAM_*`, explicit spelling table;
  unfunded tokens: `provider_unfunded`, `insufficient_funds`,
  `insufficient_balance`, `insufficient balance`, `no resource package`,
  `payment_required`.
- **Provider state wins over HTTP code:** a 429/5xx body carrying the
  unfunded marker is CLASS_UNFunded -> zero retries, follow-up, terminal.
  A plain overload 429 keeps the bounded transient retry.
- HTTP-200 path: parsed `stop_reason` (relay dialect) or raw body on parse
  failure classify BEFORE the error escapes.
- Follow-up speech protocol (owner 2026-10-30): deterministic title
  `PROVIDER_GATE:<ERROR_CLASS>:<provider>:<model>`, discrete fields
  (provider, model, route, error_class, checkpoint_id), no free-form policy
  text in titles. Checkpoint id = `log:<file>#messages:<n>` / `#preflight`.
- Config fields via env: `T27_EXPECTED_PROVIDER / T27_APPROVED_FALLBACK /
  T27_KNOWN_UNFUNDED_PROVIDERS`; unset expected derives from base URL.
- `logger::log_follow_up_task_structured` added.

## Verification evidence (raw)

| Gate | Command | Result |
|---|---|---|
| typecheck (SSOT) | `t27c typecheck specs/system/llm_request_provider_gate.t27` | OK, exit 0 |
| typecheck x4 companions | provider-guard-runtime-integration, network_mock_provider_tests, phase_4b_runtime_integration, model_routing_circuit_breaker | OK x4 |
| spec seal | `t27c seal ... --save` then `--verify` | all hashes MATCH, exit 0 |
| full cargo tests | `cargo test` (contrib/backend/agent-runner) | 13 passed / 0 failed / 1 ignored, exit 0 |
| socket intercept: transient | mock relay HTTP-200 network_error | exactly 4 requests (initial + 3 retries), then bounded-exhausted terminal |
| socket intercept: unfunded 200 | mock relay HTTP-200 provider_unfunded | exactly 1 request, PROVIDER_UNFUNDED |
| socket intercept: 429+1113 | live-wording insufficient balance | exactly 1 request, PROVIDER_UNFUNDED (was 2 retries before fix) |
| socket intercept: 429 overload | plain 429, no unfunded marker | exactly 4 requests, bounded, no unfunded |
| priority regression | mixed_body_unfunded_wins_over_transient | PASS (unfunded wins over any transient token) |
| checkpoint preserved | messages.len() assertions in all intercept tests | PASS |
| state restorable | unfunded_refusal_then_same_checkpoint_retry_stays_restricted | PASS (second cycle: again exactly 1 request, same refusal) |
| follow-up event shape | follow_up_task_event_is_machine_parseable | PASS (title + discrete fields; no free-form text) |
| E2E ground truth | `cargo test -- --ignored zai_e2e_ping_policy_outcome_valid` | PASS in 0.44s: one request to the REAL z.ai endpoint returned 429 code 1113 -> guard surfaced PROVIDER_UNFUNDED, follow-up task `PROVIDER_GATE:PROVIDER_UNFUNDED:z.ai:claude-sonnet-4-5-20250514` created, zero retries. PASS both outcomes: funded -> success; unfunded -> PROVIDER_UNFUNDED only |
| rollback drill | `scripts/rollback-drill.sh` | PASS 5/5 steps, exit 0 (typecheck, seal, cargo, follow-up in runner log, deterministic title) |
| repo suite | `t27c suite` | exit 1: parse 0, typecheck 0, conformance 0, gen zig 0, gen rust 0, gen C 0, FP 0; seal 34 passed / 520 failed |

### Cleanup items: NOT_APPLICABLE (search evidence)

- `grep -RIn "vertical consumption" .` : 1 hit - THIS report's own task line
  (line 103) only. Zero source references in the trees.
- `omc_profile` in tracked files: `git grep -Il 'omc_profile' -- .` -> **0
  files**; `~/.omc*` does not exist on this machine. Both items need owner
  pointers if they exist outside this repo.

### Full suite: the 520 seal failures are PRE-EXISTING, not this change

Taxonomy in `/tmp` captured log: 348 `gen_hash_verilog` mismatches, 387 zig
(overlapping counts across fields), 100 C, 73 Rust -> the saved seals were
taken BEFORE generation-changing commits already at HEAD (e.g.
`fix(codegen): Verilog struct field access, reg init, ExprCast (#773)` in
`f123674f`). Zero saved-seal entries concerning the guard family. The only
"no saved seal" (this guard spec) was sealed and verified green this session.
Re-sealing the drifted tree is owner-managed (L6): `t27c seal <spec> --save`
per spec, or a regeneration sweep with an issue (`Closes #N`).

## Incident truth recorded

- Route health declared by pool-state file (`~/.dsh/zai-pool-state.json`,
  HTTP 200 @ 2026-10-27) was STALE: the live probe showed z.ai with no
  balance. State files must not be trusted over the live provider answer.
- The runner did NOT spend retries on the unfunded route after the fix
  (contrasted with 3 wasted attempts, ~7s, on the initial E2E run).

## Open items for the owner

1. Fund the z.ai route or point `AGENT_PROVIDER`/profile to a funded one;
   both paths are now guarded (funded -> works, unfunded -> clean task).
2. DSH harness (packages/llm/*) classifier remains upstream: add
   `PI_AI_ERROR` to that provider's `retryableCodes`, or install
   `@aiwayds/dsh-llm-net-retry` (npm v0.7.0 exists); the maintainer already
   offers the inline source patch in discussion #4361. Patch layer:
   `~/.dsh/profiles/<profile>/cordis.patch.yml` (empty array today).
3. SIGN the seal-baseline waiver DRAFT (or run the owner re-seal sweep);
   until then FULL_REPO_RELEASE stays NO_GO per the release rule below.
4. Commit gate: whole guard family + `specs/system/` is UNTRACKED in git;
   needs `git add` + an L1 issue (`Closes #N`).
5. own-language budget: api.rs additions exceed the 80/PR thin-glue cap
   (spec header cites api.rs as the sanctioned enforcement point); next pass
   should shift logic toward t27c-generated code.
6. `scripts/tri lint` wants test/invariant blocks nested INSIDE fn bodies;
   the guard family keeps them module-flat (pre-existing style), so lint
   WARNs are baseline noise, all fns are exercised in tests/invariants.

## Release rule (owner, 2026-10-30)

```text
PROVIDER_GUARD_SHADOW_ENFORCE:  CONDITIONAL_GO
  (engineering + classification proven; waiver approval pending)

FULL_REPO_RELEASE: NO_GO while ANY holds true:
  - t27c suite exit != 0 without an owner-approved waiver
    (currently exit 1, waiver DRAFT only);
  - scripts/rollback-drill.sh absent or non-zero exit
    (resolved: exists, exit 0);
  - cleanup items unresolved without NOT_APPLICABLE search evidence
    (resolved: NOT_APPLICABLE, evidence in this file).
```
