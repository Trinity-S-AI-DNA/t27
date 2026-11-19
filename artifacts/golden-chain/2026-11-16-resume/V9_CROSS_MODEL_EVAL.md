V9 - SAME-BANK CROSS-MODEL EVAL (external reference, run locally on Mac) - 2026-11-17
=====================================================================================
Rationale: owner-ordered external standing measurement. HumanEval/MBPP are structurally
inapplicable to IGLA CODER (27-token ctx, 729 byte-BPE, .t27 language) -- recorded as an
architecture boundary. The FAIR comparable measurement: run public code models on the
IDENTICAL frozen bank v1 (same prompts, greedy temp-0, 256-token cap, same checker tests,
same non-vacuous protocol, code-block fence extraction -- standard harness practice,
documented).

MODEL: Qwen2.5-Coder-0.5B-Instruct (public weights, 953 MiB, ea3f2471 snapshot),
  transformers 5.17 / torch 2.11 CPU, chat template, greedy, max_new 256.
  Run locally on Mac (dev/verification host per owner rule); givel-vps untouched.

RESULTS on frozen bank v1 (30 tasks, pass@1 = full_pass, tests>=1 criterion):
  Qwen2.5-Coder-0.5B-Instruct : parse 27/30, full_pass 23/30  (3.2-6.0 s/task on CPU)
  IGLA CODER ckpt_18000       : parse 0/30,  full_pass  0/30
  IGLA CODER ckpt_8000 (base) : parse 0/30,  full_pass  0/30
TASK VALIDITY PROVEN: 23/30 solvable on the frozen tasks by a public 0.5B model that
  has never seen .t27 -- the tasks are well-formed; the barrier is entirely on our
  model side (architecture: 27-token ctx, 729 byte-BPE, 2.13M params).

MEASURED ANSWER TO "WILL A PUBLIC MODEL SOLVE THESE TRIVIALLY?": yes -- 76.7% pass@1
  for Qwen-0.5B vs 0% for IGLA CODER at every training depth measured (0, 500, 2000,
  10,000 updates; 0 -> 1.13 corpus equivalents).

STANDING (honest): IGLA CODER is a working train/generation loop without code-solving
  capability; Qwen-0.5B defines the same-bank reference at 23/30. GOAL criteria B/C
  remain unmet; the gap to close is 23 tasks of architecture+tokenizer distance.

SCORER DISCLOSURES (each caught before reporting):
  - vacuous positive (tests=0 & FAIL 0 counted as pass) -- fixed with tests>=1;
  - composition without checker append (Qwen rescore #1) -- caught by tests=0 anomaly,
    recomposed with checkers, rescored;
  - 4-way parallel runner cross-contamination -- batch annulled, rerun clean with
    lock + per-task dirs + checkpoint hash per receipt line.
