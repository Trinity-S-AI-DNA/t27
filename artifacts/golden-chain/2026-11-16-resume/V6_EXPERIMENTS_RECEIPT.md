V6 - PILOT-2 AND SAMPLING EXPERIMENTS - 2026-11-17 morning
==========================================================

PILOT-2 (function-only corpus, comment-stripped, train 15,724,414 / valid 2,389,934 tokens,
  500 steps from ckpt_8000, same recipe as pilot-1, VPS nice 10):
  code valid loss 8.09 -> 2.51 (ppl 3256 -> 12.27) -- better than pilot-1 (3.03/20.7).
  Frozen bank v1 rescoring: generated tail is again pure whitespace (0/30 visible chars
  >10), parse artifact 30/30, compile 0/30, pass@1 0/30. Delta vs baseline: 0.
  => whitespace collapse is ROBUST to corpus composition at greedy decoding.

IN-WINDOW CODE PROBES (code prompt within the 27-token window, checkpoint 8500):
  "pub fn add8(a: i8, b: i8) i8 {return " -> whitespace only. Total collapse even when
  code is fully conditioned. This is the capability ceiling of the current checkpoint.

SAMPLING MODE IMPLEMENTED (specs/ml/generate_hslm.t27, tests 7/7, 0 vacuous):
  cfg keys seed / temp_milli; temperature-scaled full-vocab sampling by the trainer's
  xorshift64; same seed => identical output (spec test + VPS double-run verified).
  Probe (ckpt 8500, temp 0.9, seed 1729, code prompt): output escapes the space attractor
  but degenerates into TinyStories-like prose with mangled bytes -- code structure absent.
  pass@1 remains 0/30.

CONCLUSION (honest, no threshold change):
  Two adaptation recipes + greedy and sampling decoding: 0/30 pass@1 in all conditions.
  The 2.1M-parameter byte-BPE TinyStories checkpoint does not produce .t27 code after
  500-step adaptation; distributional loss improves (ppl 12-20 on code) but structure
  generation does not emerge. GOAL criteria B and C are NOT met. The binding constraints
  are architectural (27-token context, 729 byte-BPE vocab, tiny capacity) -- the strategy
  choice recorded in EXACT_OWNER_DECISIONS governs what is runnable next.

RUNNABLE-WITHOUT-OWNER (if the overnight window continues): more steps/data (scaling
  curve), corpus with heavy code-prompt/text pairing; both are plausible but low-expectation
  without the tokenizer/context decision.
