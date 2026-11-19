V10 - SECOND CROSS-MODEL REFERENCE: DeepSeek-Coder-1.3B-Instruct - 2026-11-18
==============================================================================
Same frozen bank v1, same protocol (greedy, max_new 256, fence extraction, checkers
appended, tests>=1 non-vacuous). Weights: deepseek-ai/deepseek-coder-1.3b-instruct,
2569 MiB, e063262d snapshot; local Mac CPU fp16 (resume-loop across 2 launches -- a
crash after TASK_09 in the fp32 first attempt is noted; fp16 rerun completed 30/30,
DS_EVAL_COMPLETE).

RESULT: visible code-shaped output 30/30, parse_ok 0/30, full_pass 0/30.
  Sample (TASK_01): prose answer + ```rust pub fn add8(a: i8, b: i8) -> i8 { a + b } ```
  -- DeepSeek answered in RUST idiom (arrow return type, no return keyword, prose
  around), which does not parse as .t27. Byte-fallback glyphs visible in prose decode.

CROSS-MODEL TABLE (frozen bank v1, greedy pass@1, full_pass):
  Qwen2.5-Coder-0.5B-Instruct : 23/30   (followed the literal .t27 grammar description)
  DeepSeek-Coder-1.3B-Instruct : 0/30   (answered in Rust idiom; language mismatch)
  IGLA CODER ckpt_18000        : 0/30   (all depths 0/30)
  IGLA CODER ckpt_8000 base    : 0/30

INTERPRETATION (measured, careful):
  1. The bank is valid and TRIVIALLY solvable when the model follows the grammar
     description in the prompt (Qwen-0.5B, 76.7%).
  2. Grammar adherence dominates task difficulty: a 2x-larger public coder scored 0
     by answering in its native idiom. This sharpens the architecture finding: IGLA
     CODER's failure is not only capacity -- its window/vocab prevent it from even
     reproducing the grammar the way Qwen did from the prompt text alone.
  3. For the GOAL MVP (>=5 pass@1 for IGLA CODER): the Qwen reference shows the bar
     is achievable IN PRINCIPLE by a transformer reading the same prompts; for THIS
     checkpoint it is blocked by ctx/vocab/capacity (owner strategy decision stands).
  Disclosure: single greedy run per task; fence extraction applied to both models;
  DeepSeek prose-inside-fence variants would still fail (Rust != .t27 grammar).
