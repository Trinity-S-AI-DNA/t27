V2 BANK DRAFT - CONTEXT FEASIBILITY (NOT frozen; owner-decision material) - 2026-11-17
=======================================================================================
DRAFT: 30 signature-completion prompts (harness prepends "module coding_eval_gen;"
as boilerplate scaffold, the same way the reference check block is appended; the
MODEL's generated tokens remain the scored output).
PROMPTS (draft, /tmp/v2_draft): signature + opening lines, e.g.
  "pub fn add8(a: i8, b: i8) i8 {\n    return "
MEASURED token counts (HSLM tokenizer, via generate_hslm prompt_tokens, greedy):
  min 33 (inc8/dec8), median ~40, max 95 (tri_sign).
  Tasks fitting the 27-token window: 0 of 30 (two at 33 tokens come closest).
DECISION IMPLICATION:
  Option (a) "re-freeze prompts to <= 27 tokens" is effectively DEAD as a full-bank
  design: the byte-BPE (1.25 bytes/token on .t27) cannot express even a minimal
  function signature within the window. What remains:
    (b) few-shot within the window -- same 27-token ceiling, only micro-tasks qualify;
    (c) architecture change: larger CONTEXT_LEN (window is 3^3 by design -- changing it
        touches kv caches, positional scheme, and invalidates checkpoint compatibility
        unless the position scheme is preserved), or vocab/merges (embeddings+output
        head incompatibility), or capacity (2.1M params);
    (d) accept the boilerplate-luck ceiling and report pass@1 as-is.
  This measured fact (0/30 conditionable) is the strongest input for the strategy pick.
Pilot-3 (scaling point): 1500 more steps on the function-only corpus from ckpt_8500
  (total 2000 adaptation steps) launched; bank v1 rescore to follow.
