V8 - PILOT-4 FINAL VERDICT (frozen bank v1, non-vacuous protocol) - 2026-11-17
==============================================================================

RUN: pilot-4 = 8000 updates (cumulative 10,000 code-adaptation updates since TinyStories
  base), 14,256,000 tokens (planned == actual: "done steps 8000" rc 0), 1.13 corpus
  equivalents, lr 3.5e-4 -> 1e-6, wall 27,985 s (7.8 h), 3.489 s/step, 510.8 tok/s,
  peak ~69 MB class host (4 CPU / 7 GB, nice 10).
  checkpoint ckpt_18000: .state sha256 prefix 81cdeb5a95f80c0b (verified in every
  generation receipt line by the v2 runner).

SCORING (frozen v1, greedy pass@1, tail-only composition, isolation per procedure,
  tests>=1 non-vacuous criterion):
  parse_ok     0/30
  compile_ok   0/30
  full_pass    0/30
  non-whitespace visible tails 22/30 (collapse gone; model emits corpus-shaped fragments
  like "module ... key_hash: 1, }," -- structurally imitative, syntactically invalid)

FINAL COMPARISON TABLE (same frozen bank v1, same greedy protocol):
  | condition                      | updates | corpus equiv | code valid ppl | visible tails | parse_ok | full_pass |
  | TinyStories base (8000)        | 0       | 0            | n/a (prose)    | 30/30 prose   | 0/30     | 0/30      |
  | pilot-2 (500, full-spec corpus)| 500     | 0.057        | 12.27          | 0/30 spaces   | 30/30*   | 0/30      |
  | pilot-3 (2000, fn corpus)      | 2,000   | 0.227        | 9.57           | 30/30 broken  | 5/30     | 0/30      |
  | pilot-4 (10,000, fn corpus)    | 10,000  | 1.133        | 7.92           | 22/30 imitative| 0/30    | 0/30      |
  (* 30/30 parse_ok at 500 steps is the checker-only artifact: whitespace parses when the
  reference test block is appended. Not a code signal.)

VERDICT:
  EXECUTION: PASS -- the full train/save/reload/generate/measure loop runs to completion
  with receipts at every step (GOAL criterion A satisfied; D partially: loop demonstrated,
  but no verified-training-example weight-update cycle beyond the pilots themselves).
  QUALITY: criteria B (>=5 pass@1) and C (>= +3 tasks) NOT MET at 1.13 corpus equivalents.
  Measured ceiling factors, in order of evidential weight:
    1. CONTEXT_LEN 27 vs prompts 124-178 tokens (0/30 tasks conditionable);
    2. 729-token TinyStories byte-BPE (code = 80.7% unmerged bytes);
    3. 2.13M parameters.
  pass@1 = 0 in every measured condition (baseline, 0.057, 0.227, 1.133 corpus equiv;
  greedy and sampled). Thresholds NOT lowered; GOAL remains open. The next experiment
  class that could change the verdict requires an architecture/tokenizer decision
  (owner-gated): no further same-recipe scaling is planned -- 4x data already produced
  structure without validity.
  For external standing: same-bank cross-model eval (Qwen2.5-Coder-0.5B et al. on the
  identical frozen tasks) is prepared and runnable; awaiting owner go-ahead for
  third-party weights on givel-vps.
