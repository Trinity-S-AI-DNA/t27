V7 - SCALING POINT AND SCORER CORRECTION - 2026-11-17
=====================================================

SCORER CORRECTION (caught before reporting):
  The quick scorer accepted "tests 0 / FAIL 0" as a pass. TASK_04 (candidate-3) scored
  a false full_pass this way: the composed spec parsed with ZERO tests (degenerate model
  text swallowed the checker block). CORRECTED PROTOCOL: full_pass requires
  tests >= 1 AND FAIL 0 AND compile. Recounted: candidate-3 full_pass = 0/30.
  This criterion (non-vacuous test count) is now part of the frozen protocol notes.

PILOT-3 (scaling point, 2000 total adaptation steps, function-only corpus, VPS):
  1500 additional steps from ckpt_8500; 3.36 s/step, 529 tok/s, wall 5103 s; rc 0.
  code valid loss: 2.51 -> 2.26 (ppl 12.27 -> 9.57).
  bank v1 scoring (tail-only, tests>=1 criterion): parse_ok 5/30, full_pass 0/30.
  TREND vs depth (same frozen bank, greedy):
    0 steps:      outputs TinyStories prose;                 parse 0/30, full_pass 0/30
    500 steps:    whitespace collapse;                       parse 30/30 (artifact), full_pass 0/30
    2000 steps:   structure-like text emerges, 25/30 files emit non-parsing code fragments;
                  whitespace collapse GONE (30/30 visible tails)
  => adaptation is moving in the right direction (loss down, structure appearing), but
  0 full passes: the 27-token context and byte-BPE ceiling dominate. GOAL criteria B/C
  still NOT met; pass@1 = 0 in every condition measured (baseline, 500, 2000 steps,
  greedy and sampled).

RUNNING TOTALS: three adaptation runs (500, 500-different-corpus, +1500), four scorings,
  all receipts under artifacts/golden-chain/2026-11-16-resume/. VPS production untouched.
