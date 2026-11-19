FROZEN-EVAL SCORING PROCEDURE (isolation contract) - 2026-11-17
================================================================
Per task TASK_NN (01..30), one greedy generation, checkpoint fixed, bank v1:
  1. prompt := coding_eval/TASK_NN.prompt.txt (deployed copy, hash in FROZEN_MANIFEST)
  2. gen.cfg := step=<CKPT> max_new=256            (greedy when seed absent)
  3. run ./hslm_gen (t27c gen + zig build-exe binary; no network use by construction)
  4. compose: generated tail (prompt echo stripped by prompt-token count)
     + reference checker block (from the bank, never shown to the model)
  5. score, in order, with ISOLATION on every execution of model-derived content:
       timeout 120 (kill -9), ulimit -v 2000000 (2 GB address space), cwd /tmp,
       env -i (no credentials, no network tokens; t27c/zig need no network)
       a. t27c parse  -> parse_ok
       b. t27c test-report -> compile_ok (not BLOCKED) AND tests >= 1 (non-vacuous
          criterion; "FAIL 0" with tests=0 is NOT a pass) AND FAIL 0 -> full_pass
  6. record per-task: raw ids, decoded text, composed file, each verdict + reason.
  Levels: decode_ok / parse_ok / compile_ok / test_pass / full_pass. pass@1 = full_pass.
  The isolation wrapper applies to scoring only; training and generation binaries are
  ours (not model-derived) and run under nice on the idle host.
