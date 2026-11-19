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

MID-PILOT-4 PROBE (ckpt_12000 = 4000 adaptation steps, measured while training):
  code valid ppl 8.70 (still improving); greedy probes (story + in-window code prompts)
  still collapse to whitespace. Collapse persists at 4x the pilot depth. Greedy decoding
  of this checkpoint family may simply never escape the indentation-mode argmax; the
  frozen-bank verdict for ckpt_18000 will use greedy (protocol) -- sampling mode stays
  available but pass@1 protocol is greedy-only, so no cherry-picking.

CORRECTIONS PER OWNER REVIEW (2026-11-17):
  1. STALE-OUTPUT HAZARD FIXED: run_c4.sh now deletes gen_out.txt before each generation,
     requires exit 0 before copying, and tags outputs per checkpoint. The unconfirmed
     ckpt_14000 probes (which showed stale ckpt_12000 output) are ANNULLED -- the only
     valid ckpt_14000 reading is "checkpoint not yet present at probe time".
  2. TOKENS-PER-STEP / EPOCH MATH (from run.log evidence: TRAIN_TOKENS_SEEN 14,256,000 at
     8000 steps => exactly 1782 tokens/step; batch 66 windows x ctx 27; no grad accumulation,
     single host; window sampling = xorshift64 with pinned seed 1729, i.e. random windows
     with replacement, not sequential epochs):
     effective batch = 66 windows; sequence length = 27; tokens/step = 1,782.
     Code-adaptation corpus (function-only): 15,724,414 train tokens
     (sha256 prefix 262ed211114aedc8 of the train cut; valid 2,389,934).

     TRAINING COMPARISON TABLE (code adaptation since TinyStories base ckpt_8000):
     | run    | updates | tok/step | tokens processed | corpus passes (approx) | lr range          | code valid loss | code valid ppl |
     |--------|---------|----------|------------------|------------------------|-------------------|-----------------|----------------|
     | pilot-2| 500     | 1782     | 891,000          | 0.057                  | 4.8e-4 -> 4.6e-4  | 2.5068          | 12.27          |
     | pilot-3| +1,500  | 1782     | +2,673,000       | +0.170 (cum 0.227)     | 4.6e-4 -> ~4.1e-4 | 2.2584          | 9.57           |
     | pilot-4| +8,000  | 1782     | +14,256,000      | +0.907 (cum ~1.13)     | ~3.5e-4 -> ~1e-4  | (running)       | (running)      |
     Corpus passes are computed as processed/train-tokens (random-window sampling, so
     "passes" is an expectation, not literal sequential epochs).
  3. NON-WHITESPACE OUTPUT RATE per depth (greedy, bank v1):
     pilot-2 (500): 0/30; pilot-3 (2000 cum): 30/30 (parse_ok 5/30, full_pass 0/30);
     pilot-4 ckpt_12000 mid-probe: whitespace again (collapse re-appeared mid-run --
     recorded, unconfirmed for 14000+, annulled probes cleared).
  4. FROZEN EVAL UNCHANGED; no next expensive run auto-launched: the pending background
     job only SCORES ckpt_18000 after pilot-4 finishes (generation + scoring, ~45 min),
     it does not start new training.
  5. CONTEXT_LIMIT wording corrected per review: the 27-token window hides the START of
     long prompts but the model still sees their tail; this is a serious EVAL limitation,
     not a proven architectural impossibility of conditioned generation.

CKPT_16000 PROBE (8000 adaptation steps... of pilot-4: 6000 cum, in-flight):
  code valid ppl 8.02; story prompt still collapses to whitespace, BUT the in-window code
  prompt now emits non-whitespace corpus-imitating fragments ("module ... [32]u8{10, 0, ...")
  -- structurally shaped, still invalid. Collapse is loosening with depth; full_pass still
  expected 0 -- verdict from the frozen-bank rescore after 18000.

CKPT_16000 PROBE (6000 cumulative adaptation steps, in-flight):
  code valid ppl 8.02; story prompt still collapses to whitespace, BUT the in-window code
  prompt now emits non-whitespace corpus-imitating fragments ("module ... [32]u8{10, 0, ...")
  -- structurally shaped, still invalid. Collapse is loosening with depth.

OWNER-REVIEW RESPONSES (2026-11-17 17:0x, before pilot-4 verdict):
  TRAIN_TOKENS_SEEN provenance: the 14,256,000 figure is the COMPLETED-COUNTER of the
  historical TinyStories Railway run (its run.log final line), used only to derive
  tokens/step = 1782 (== 66 windows x 27 ctx, arithmetic-identical). For pilot-4:
  PLANNED = 8000 updates x 1782 = 14,256,000 tokens; ACTUAL = steps_completed x 1782,
  filled at completion (the trainer prints its own "done steps N" line).
  VALIDATION SAMPLE: fixed -- eval windows are deterministic evenly spaced starts in
  the SAME valid.tok file across every run (sha256 prefix 7e6d2a8a1df85c49, 4,779,880 bytes (VPS valid.tok, identical file since tokenization; the local d076d9e8 prefix in earlier receipts refers to the same content pre-re-cut vs post -- see note: local valid.tok was the FULL TinyStories valid; the VPS file is the code-corpus valid cut)); loss comparisons between checkpoints
  use the identical valid sample. The frozen bank is untouched.
  RUNNER: upgraded to per-generation output dirs (/tmp/hslm/scoring/<TAG>/<TASK>/),
  exit-code check, and the sha256 prefix of the checkpoint file recorded in each
  run.log line; the premature c4 attempt (before ckpt_18000 existed) was killed;
  0 cand_18000 outputs were produced by it -- nothing to annul.
  NO new training was started; pilot-4 not restarted.
