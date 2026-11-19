GOLDEN_CHAIN_RUNTIME_MVP_V3 - 2026-11-17
========================================

SMALL_FIXES (all in specs/ml/train_hslm.t27, this lane, uncommitted; 42/42 tests pass):
  F1 portable timer: tr_now() falls back to prof_now() (#13168 portable monotonic clock)
     when /proc/uptime is absent (macOS). Verified at runtime: tok_per_s 280.8,
     s_per_step 6.3472 on macOS (were 0.00).
  F2 explicit NO UPDATES PLANNED message when restored step >= target steps. Placed AFTER
     the header + restored-step eval line, so resume=X steps=X stays a working eval-only
     invocation (the first draft broke that; caught and fixed in this slice).
     Verified at runtime (message text recorded in pilot logs below).
  F3 regression test "steps_is_an_absolute_target_and_updates_are_the_difference":
     fresh 0->8000 = 8000; resume 8000->8010 = 10; 8000->8000 = 0; past-target = 0.
  Spec tests: 42/42 pass. Vacuity: counting build still broken on master (compiler defect
  spec.zig:988), NOT claimed fixed.

EVIDENCE_CORRECTIONS demanded by review, applied to receipts:
  - "bit-в-бит baseline match" claims downgraded to: printed values identical; the eval
    FILE bytes were not compared (only stdout lines). Where cmp byte-identity IS proven
    (ckpt_8010.state/.bin, same machine) it stays byte-level.
  - "max abs diff 1.36e-16" stays as the measured ABSOLUTE difference; the "1 ulp" and
    "FMA ordering" wording is a HYPOTHESIS for the cross-architecture pattern, not a
    measured fact (no per-value ULP-distance computed).
  - baseline line of V2 receipt is superseded by the table in FAST_EVAL below.
  - trusted-hash ordering: the migration spec does NOT verify trusted source itself;
    in every real migration performed, sha256 of the ORIGINAL file was recorded BEFORE
    migration (9034e049... macOS, 9034e049... VPS) and matched the HF-published file.
    This ordering is now written in the migration spec header as a named precondition.

FAST_EVAL (definition pinned BEFORE the runs below):
  data: /tmp/hslm/valid.tok = full TinyStoriesV2-GPT4-valid, 7,689,354 tokens,
        sha256 d076d9e8fd8a4fd6ab1ee67d0d57cf770fb0597bdf0ccee87c69a21e0c284d91
  windows: 512, evenly spaced starts (trainer tr_valid_loss), 28 tokens each
  command: resume=<step> steps=<step> eval_windows=512 (eval-only; prints one line)
  FULL eval remains eval_windows=19683 (the frozen HF comparison).
  FAST baseline at ACCEPTED checkpoint 8000 (macOS, this slice):
    last_loss 3.1147 last_ppl 22.53 full_loss 3.2225 full_ppl 25.09
  NOTE: FAST (512) and FULL (19683) numbers are NOT comparable to each other; each
  comparison is within its own window count. Train/valid split unchanged; eval windows
  are held-out valid.tok starts, never in training data.

TOKENIZER_CODE_COMPATIBILITY (measured, not assumed):
  HSLM vocab 729: ids 4..259 are raw bytes, 260..728 are 469 BPE merges trained on
  TinyStories prose. Probe: tokenize 270,346 bytes of .t27 specs (train_hslm + migration
  + nn/hslm) with specs/nn/hslm_tokenizer.t27 via the corpus binary:
    .t27:         216,294 tokens, 1.25 bytes/token, 80.7% single-byte (unmerged) tokens
    TinyStories:  2.93 bytes/token, 24.0% unmerged
  => every .t27 byte IS representable (no UNK wall), but 4 of 5 code tokens are raw
  bytes the model has never seen merged; the checkpoint has zero code-merge statistics
  and zero code in training data. Meaningful code GENERATION from this checkpoint is
  not expected and is NOT claimed. GAP + minimal plan (not started): (a) code-aware
  merge table or a code tokenizer, (b) corpus mixing spec, (c) fine-tune recipe,
  (d) a generation entrypoint -- none exists in the trainer today (train/eval only).

RUNTIME CYCLE, one demonstrated pass (existing components only):
  1 INPUT DEFECT     resume of legacy HF checkpoint refused rc=30 (V1 slice)
  2 CANDIDATE SPEC   specs/ml/state_digest_migration.t27 (new, this lane)
  3 BUILD/TEST       t27c test-report: 5/5 pass, 0 vacuous (counting build worked here)
  4 VERDICT          ACCEPT (migration semantics proven; corrupt-payload masking refused)
  5 RECEIPT          V2_RECEIPT.md + migration I/O hashes (in / in V2_RECEIPT)
  6 MODEL UPDATE     candidate ckpt_8010 produced by the resumed run (stored SEPARATELY
                     at /tmp/hslm/ckpt_8010.*; the ACCEPTED checkpoint remains ckpt_8000)
  7 EVAL             FAST(512) and FULL(19683) on candidate vs accepted (V2/V3 numbers)
  8 DECISION         INCONCLUSIVE (delta within noise: last -0.0005 / full +0.0002 on
                     FULL; FAST not yet run on candidate) => accepted checkpoint NOT
                     replaced; rollback trivially available (originals kept as .orig)
  9 NEXT CANDIDATE   specs/ml/train_hslm.t27 fixes F1-F3 of THIS slice, provenance:
                     the zero-timer and silent-zero-steps defects observed in V2 runs.
  Nothing in this cycle was called a weight update except the candidate checkpoint
  itself; cache/log changes are not claimed as model updates.

CHECKPOINT_DECISION: candidate ckpt_8010 KEPT AS CANDIDATE; accepted = ckpt_8000 (unchanged).
RECEIPTS: V3_RECEIPT.md (this file), V2_RECEIPT.md, RECEIPT.md, SHA256SUMS.txt,
  pilot_b.log, pilot_vps.log.
REMAINING_GAP: no inference/generation path; vacuity counting broken on master;
  equivalence beyond 10 steps not run (deferred by order, budget-gated);
  code-capability gap above.
