# Seal baseline waiver 2026-10-30 - pre-existing codegen drift

**Status:** WAIVER DRAFT - requires OWNER approval (AI may not approve)
** scope:** FULL repo suite `t27c suite`; blocker for FULL_REPO_RELEASE only.
**Provider guard verdict:** unaffected - ENGINEERING_GREEN stands without this
waiver (see PROVIDER_GUARD_NETWORK_ERROR_UNFUNDED_REPORT.md).

## Verdict impact (machine-verifiable)

```text
ENGINEERING_GATES (guard):        GREEN  - exit 0: typecheck/cargo/intercept/E2E
FULL_REPO_SEAL_BASELINE:          RED    - t27c suite exit 1
RELEASE for provider guard:       CONDITIONAL_GO (this waiver once approved)
FULL repo release:                NO_GO until waiver approval or re-seal sweep
```

## Inventory

- File: `.trinity/seal-waivers/2026-10-30-inventory.txt`
- Lines: **520 unique failing specs** (one line per spec from the final
  `t27c suite` run, `/private/var/folders/.../t27-suite-final.log`).
- Composition: **465** specs fail with `hashes differ from saved seal`
  (gen-hash field mismatches); **55** specs have `No saved seal found`.
- Field taxonomy (overlapping; a spec can fail several fields):
  `gen_hash_verilog 348`, `gen_hash_zig 387`, `gen_hash_c 100`,
  `gen_hash_rust 73`.
- The engineering phases during the SAME run were all zero:
  parse 0, typecheck 0, GF16 conformance 0, gen zig/rust/verilog/C 0,
  fixed-point 0. **The specs themselves are intact**; only the saved
  baseline is stale.

## Root cause (established)

The saved seals were generated BEFORE generation-changing commits already in
HEAD history, most recently `fix(codegen): Verilog struct field access, reg
init, ExprCast (#773)` (`f123674f` merges it). Current generator output is a
superset of compiler fixes the seals never saw: `spec_hash` still MATCHes
(specs unchanged); the mismatch is confined to generated-code hashes.
This is therefore **drift, not corruption**: no spec content changed, and no
guard work introduced it (the guard spec family is untracked/ sealed this
session; the 520 failing paths contain none of the guard specs).

## Remediation (owner-managed, L6 CEILING)

Option A (waiver route): owner approves THIS waiver with a validity limit
(until the next codegen-changing merge) - suite stays exit 1, the release
gate reads the waiver.
Option B (repair route): a re-seal sweep in one owner-approved commit, per
spec: `t27c seal <spec-path> --save`, then `t27c suite` must read exit 0.
Requires an L1 issue (`Closes #N`) naming the sweep.

**Remediation owner:** repo Owner (tri-cell W - seal authority per
docs/agents/AGENTS_ALPHABET.md §W; root default reviewer A-Architect per
OWNERS.md). AI-crafted change may NOT approve the waiver nor run Option B.

## Regression guard added this session

A single new-spec seal is now saved and verified green (exit-0 proof):
`.trinity/seals/llm_request_provider_gate.json` - `t27c seal ... --verify`
-> all hashes MATCH, exit 0. If a future change re-breaks the GUARD spec seal
itself, it will surface as one additional suite failure, distinguishable
from the 520-line baseline documented here.
