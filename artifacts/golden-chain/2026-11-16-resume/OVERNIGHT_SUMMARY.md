OVERNIGHT SUMMARY - 2026-11-17 morning (GOAL: OVERNIGHT_T27_P0_AND_WORKING_IGLA_CODER)
Worktree: dsh-8087977d-5535 @ 22c6eb5ac7 + uncommitted overlay (train_hslm master-file fix
0bd1965960 + V3 fixes + new specs: state_digest_migration, generate_hslm, coding_eval/task_bank)

IMPLEMENTED (this overnight, all local-lane, no push/merge/deploy):
  1. Generation entrypoint specs/ml/generate_hslm.t27: state load (digest-checked), TWN
     scatter, greedy argmax, EOS/cap stop, decode, receipt with weights-hash invariance.
     Tests 5/5, 0 vacuous. macOS + VPS builds. cfg multi-line parser fixed (max_new honored,
     runtime-verified), portable clock fallback for macOS.
  2. Frozen coding eval: specs/ml/coding_eval/task_bank.t27, 30 tasks v1, structural test
     green; prompts/checks extracted to per-task files with FROZEN_MANIFEST; pass@1 protocol
     (greedy, one generation, no best-of-N) recorded.
  3. Cleaned .t27 corpus for adaptation: 3,279 spec files, eval bank excluded, cut to
     caps (train 15,074,712 tokens / valid 2,121,285), tokenized on VPS.

TESTED_ON_EXACT_SHA: specs at sha256 4be3ffd7… (train_hslm), bank 8aca3771… (v1),
  ckpt_8000.state e5bcdac0… (migrated, from HF 39df9a77 file 9034e049…).

P0_AUTHORIZATION_AND_ENFORCEMENT (line A, diagnosis only, their lane untouched):
  - VF_WRITE_ROLE env: NOT self-assignable in current branch revision (role is a literal
    stdin line, hook hard-codes it). Server side: workflow comes from master for
    pull_request_target -- sound.
  - REAL HOLE FOUND: effective_role() maps "landing" to min_role_for_diff -- the weakest
    role covering the diff -- so a self-declared "landing" PR passes the gate on OWNER-ONLY
    paths. The branch's own tests fail on this (vf_write_gate 12/13, vf_guard 16/17,
    measured locally). Minimal fix proposed: landing -> fixed ceiling GR_ENGINE.
  - PR #14083 CI blockers: (a) foreign-line budget counts the PR's own generated C
    (+565, gate-bootstrap catch-22, needs master-side/owner decision); (b) scan: home
    paths in runs/ref_complete.txt on the branch; (c) #14080 pull_request_target
    startup_failure since 16:01 UTC -- org Actions policy, admin action needed (exact
    request recorded in P0_VF_WRITE_GATE_AUDIT.md).

MODEL_GENERATION (line B):
  - macOS: real generation from ckpt_8000, coherent TinyStories prose, greedy-deterministic
    (ids/decoded identical across runs), weights hash unchanged.
  - VPS: reproduced, IDENTICAL ids/decoded to macOS (cross-arch), 3.6x faster.
    GOAL CRITERION A (reproducible inference) satisfied.

CODING_BASELINE (frozen eval v1, greedy, checkpoint 8000):
  parse_ok 0/30; pass@1 = 0/30. Outputs are prompt echo + TinyStories prose (expected from
  the tokenizer probe: 80.7% unmerged bytes on .t27).

CODING_CANDIDATE (ckpt_8500: 500-step code-adaptation pilot, lr~4.6e-4, VPS):
  Code-domain loss: valid ppl 2055 -> 20.7 (strong distribution adaptation).
  But greedy generation degenerates: 256 whitespace tokens after the prompt echo.
  parse_ok 30/30 -- COMPOSITION ARTIFACT (spaces + appended reference test block parse
  standalone), NOT code ability; compile_ok 0/30, test_pass 0/30, pass@1 = 0/30.

TASKS_SOLVED_BEFORE_AFTER: 0 -> 0 (delta 0; GOAL criterion C NOT met; no threshold lowering;
  goal continues -- pilot shows distribution adaptation but not yet usable code generation).

CPU_RAM_TIME (overnight totals, VPS 4 CPU / 7 GB): baseline 30 gens ~40 min; candidate run
  ~45 min; pilot 500 steps 29.7 min (3.44 s/step, 517 tok/s); scoring ~10 min local Mac.

ARTIFACTS_AND_RECEIPTS: artifacts/golden-chain/2026-11-16-resume/{V3,V4_RECEIPT,
  OVERNIGHT_SUMMARY,P0_VF_WRITE_GATE_AUDIT,FROZEN_MANIFEST}.md/.txt, pilot logs on VPS
  (/tmp/hslm/{pilot_ft.log,baseline.log,cand.log}), scored outputs under /tmp
  (baseline_scored, cand_scored/specs).

UNRESOLVED_DEFECTS:
  1. greedy whitespace collapse after code fine-tune (generation quality, not infra)
  2. vacuous-counting build broken on master (compiler spec.zig:988) -- reproducer with lane
  3. train_hslm "no updates planned" message + timer fallback are lane-local, unlanded
  4. tok_per_s integer rounding in gen receipt (cosmetic)
  5. eval-scoring harness is agent-driven, not a committed .t27 runner (documented; the
     .t27-only harness is future work)

EXACT_OWNER_DECISIONS (for morning):
  1. P0 gate: approve the landing->GR_ENGINE fix direction for PR #14083 (their lane).
  2. Foreign-line budget: master-side exemption/refactor for generated gate C (catch-22).
  3. #14080: org Actions policy check at admin level (request text in audit file).
  4. Coding MVP realism: with a 2.1M-param byte-BPE model, 5/30 pass@1 may be unreachable
     without tokenizer/merge changes (goal allows this only with explicit compat plan) --
     decision requested: continue fine-tune scaling vs tokenizer work vs scope adjustment.

NEXT_EXECUTABLE_TASK: anti-collapse generation experiments (temperature/seed mode already
  specced as optional; repeat-penalty; shorter prompts; more adaptation steps on function-only
  corpus slice), then rescore on the FROZEN v1 bank unchanged.

ROUND-2 ADDENDUM (2026-11-17 morning):
  pilot-2 (function-only corpus, 500 steps): code ppl 12.27; bank v1 pass@1 0/30
    (whitespace collapse persists at greedy).
  in-window probes: even with code fully inside the 27-token window, greedy emits spaces.
  sampling mode (seed/temp) implemented, 7/7 tests: escapes spaces, degenerates to prose.
  pilot-3 (+1500 steps, 2000 total): code ppl 9.57; whitespace collapse GONE (30/30
    visible tails); parse_ok 5/30; full_pass 0/30 (corrected scorer: tests>=1 required --
    a vacuous TASK_04 false-positive was caught and documented).
  v2-bank draft measured: signature prompts are 33-95 tokens; 0/30 fit the 27-token
    window -> option (a) dead as a full-bank design; feasibility analysis saved.
  P0 line A update: branch #14083 moved upstream (active lane); landing->GR_ENGINE fix
    validated green (13/13); second hole found (gate-bootstrap self-write of vf_guard.t27)
    with the branch's own two tests in direct contradiction -- owner policy decision.
