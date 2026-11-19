V5 - VPS_INFERENCE_AND_FROZEN_CODING_BASELINE - 2026-11-17
===========================================================

VPS_GENERATION (checkpoint 8000, prompt "Once upon a time", max_new=48, givel-vps x86_64):
  source: specs/ml/generate_hslm.t27 this worktree (uncommitted); built on VPS with
  zig 0.16.0 x86_64 from the same generated artifact as macOS (byte-identical zig source
  shipped, not rebuilt from a different spec).
  decoded output: real model text (TinyStories-style); weights hash unchanged
  (401aba20c7266fe2 before == after); load_secs_ms 144-155; gen_secs_ms ~13000.
  SPEED METRIC FIXED per review: tok_per_s_milli 3696 (= 3.696 tok/s fractional, over NEW
  tokens only); load time now reported separately from generation time (fields
  load_secs_ms / gen_secs_ms). Peak RSS on VPS: 71,040 KB (~69 MB, /usr/bin/time -v).
VPS_GREEDY_REPEATABLE: two consecutive runs, ids+decoded sections byte-identical.
EOS_AND_CAP:
  cap stop: exercised in every run (eos=0, cap hit) -- verified.
  EOS stop: mechanism pinned by new spec test the_stop_rule_fires_on_eos_and_cap_only
  (6/6 tests green). LIVE OBSERVATION: greedy never emitted EOS even at max_new=400
  (12-token story prompt) -- recorded as an observation, NOT claimed as a live EOS pass.
CONTEXT_LIMIT (architecture constraint, not a tokenizer defect):
  ALL 30 frozen prompts encode to 124-178 tokens; the model's causal window is
  CONTEXT_LEN = 27. The sliding window therefore shows the model only the LAST 27 tokens
  of each task statement -- it never sees the start of its own instruction. Consequence:
  in the current architecture, conditioning on the specific task is impossible; any
  pass@1 gains are limited to boilerplate-shape luck. Honest options (owner decision):
  (a) re-freeze prompts to <= 27 tokens each (conditionable), (b) few-shot with in-window
  exemplars, (c) architecture change (out of current scope). Version stays v1 until owner
  picks; nothing re-scored against a silently changed bank.

FROZEN_EVAL: task_bank v1 unchanged (8aca3771...); prompts/checks per task + manifest.

BASELINE_COMPLETED_TASKS: 30 / 30 (checkpoint 8000, greedy, max_new=256, one generation
  per task, no best-of-N, no templates or manual fixes; raw ids + decoded preserved).
PASS_AT_1: 0 / 30 (parse_ok 0/30 scored on generated tail + reference checker; earlier
  prompt-echo composition also 0/30 -- result unchanged, method corrected per review).
FAILURE_BREAKDOWN: 30/30 outputs are TinyStories-style prose continuation after prompt
  echo; none contains a module/function body (consistent with the tokenizer probe:
  .t27 is 80.7% unmerged bytes for this vocab and the model never saw code in training).

TRAINING_CORPUS (prepared): 3,279 spec files, eval bank excluded, 15,074,712 train /
2,121,285 valid tokens (under the loader caps), tokenized on VPS; sha256 of the source
concatenation 4b286186aa6368c4 (truncated), full receipt in SHA256SUMS.

NEXT_CODE_ADAPTATION_RUN (proposal, within existing budget): repeat the 500-step pilot
  with a function-only corpus slice (strip prose comments), then rescore the SAME bank.
  Prior pilot: code valid ppl 2055 -> 20.7 but greedy whitespace collapse -- the next
  experiment targets that collapse, not more of the same.

RESOURCE_ESTIMATE: candidate rescoring ~45 min VPS CPU per checkpoint; pilot 500 steps
  ~30 min; both fit idle-night capacity at nice 10-15, production untouched.

EVIDENCE: /tmp -> artifacts copies: gen_out.txt, rep1.txt (repeatability), eos-run
  (max_new=400), baseline/candidate genouts (tgz) copied into this directory; hashes in
  SHA256SUMS.txt. Corrections to earlier reports applied: CODING_BASELINE now MEASURED
  (0/30, table above); EOS claim replaced by mechanism-test + live observation note.

EVIDENCE FILES (this directory):
  once_run.txt / twice_run.txt -- VPS ckpt_8000 "Once upon a time" max_new=48:
    two consecutive runs; ids+decoded sections byte-identical (cmp verified).
  eos400_run.txt -- VPS max_new=400, story prompt: eos=0 (greedy did not emit EOS in
    400 tokens); stop mechanism covered by spec test the_stop_rule_fires_on_eos_and_cap_only.
  baseline_genouts/ -- 30 raw generations, checkpoint 8000 (frozen-eval baseline).
  candidate_genouts/ -- 30 raw generations, checkpoint 8500 (pilot).
  Overlaid spec fixes in this slice: fractional tok_per_s_milli, load/generation time split,
    extracted stop rule + regression test (generate_hslm.t27: 6/6 tests, 0 vacuous).
