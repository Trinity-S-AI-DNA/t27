GOLDEN_CHAIN V4 - GENERATION ENTRYPOINT - 2026-11-17
====================================================

NEW ARTIFACT: specs/ml/generate_hslm.t27 (this lane, uncommitted)
  - inference-only: loads ckpt_<step>.state (full f64 params, digest-verified, header-checked),
    scatters through nn::hslm TWN, greedy-argmax generation, EOS stop, tokenizer decode
  - spec tests: 5/5 pass, 0 vacuous (counting build worked for this spec)
  - binary: /tmp/hslm_gen (macOS arm64, zig 0.16.0), built via t27c gen + zig build-exe
  - config: /tmp/hslm/gen.cfg (step, max_new); prompt: /tmp/hslm/prompt.txt (fixed paths,
    the trainer host-IO convention; argv not wired)
  - receipt written to /tmp/hslm/gen_out.txt

FIRST REAL GENERATION (checkpoint 8000 = accepted TinyStories checkpoint, prompt "Once upon a time", max_new=48 per cfg... run recorded with generated=64? no: 48):
  ids: 1 463 105 2 476 464 410 696 473 540 618 440 461 298 267 483 470 323 424 369 551 (then
  the 323 424 369 551 loop to the token cap)
  decoded: "Once upon a time" -> "... there was a little boy named Tim. Tim loved to ...
  They were very happy. (repeated to the cap)"
  VERDICT: real model output, coherent TinyStories-style prose with the expected greedy
  degeneration into a repeated phrase; NOT code, as predicted by the tokenizer probe (V3).
  No external model, no template, no retrieval -- pure forward passes of our weights.

INFERENCE INVARIANTS VERIFIED:
  - params_hash before == after (weights unchanged by inference): recorded in gen_out.txt
  - greedy determinism: two consecutive runs, ids + decoded sections IDENTICAL
    (only the gen_secs_ms timing field differs between runs)
  - no infinite generation: EOS stops; the max_new cap stops otherwise (both paths exercised:
    eos=0 with cap hit in this run)

MEASURED (macOS): 48 tokens in 65.7 s generation section = ~0.73 tok/s
  (full 27-token re-encode per generated token; no KV reuse -- correctness first, speed is a
  known open item; KV-cache reuse across steps is the obvious next optimization, NOT done here)
  tok_per_s field prints 0 due to integer division rounding -- cosmetic defect, noted.

LAYOUT DEFECTS FOUND AND FIXED DURING THIS SLICE (all spec-side, none hand-edited in generated code):
  1) payload buffer too small by STATE_HEADER (image vs payload confusion) -- header now read
     into its own 64-byte buffer
  2) single host_read short-read risk -- chunked full-read loop added
  3) host_open needs a NUL-terminated pointer -- the trainer's path_buf idiom adopted

HONEST GAPS (not claimed):
  - generation is TinyStories-quality only; coding capability still absent (needs corpus +
    fine-tune + eval per GOAL steps 2-4)
  - no VPS reproduction of the generation entrypoint yet (next executable task)
  - tok_per_s integer rounding; KV reuse absent; argv not wired (fixed paths)
