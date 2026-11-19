GOLDEN_CHAIN_CPU_RESUME_V1 receipt - 2026-11-16
Host: macOS arm64 (local build), zig 0.16.0 (/tmp/zig-aarch64-macos-0.16.0), t27c built from master 22c6eb5ac7 (worktree dsh-8087977d-5535)
Spec source: specs/ml/train_hslm.t27 with PR #13959 f64 array-init fix (0bd1965960), file taken from origin/master, uncommitted in worktree

RC30 ROOT CAUSE (two independent causes, both fixed by config/artifact, no code change):
1. HF ckpt_8000.state stores digest=0 at header byte 52 (written by pre-#13168 writer);
   the #13168 loader recomputes the FNV-style digest (h0=1469598103, h=(h*31 + wb%4294967291)%4294967291
   over 3*NPARAM f64 payload in file order) and refuses a mismatch with RC_RESUME=30.
   Migration: recomputed digest (539461042) written into a COPY (ckpt_8000.state.orig preserved,
   sha256 9034e049...); migrated file = ckpt_8000.state e5bcdac0... (sha in SHA256SUMS of mig copy /tmp/hslm/ckpt_8000_mig.state).
2. Loop condition `while (step < cfg_steps)`: cfg_steps is the ABSOLUTE target step, not a count.
   resume=8000 with steps=10 runs zero steps. Correct invocation: steps=8010 total=8010.

BASELINE (current env, step 8000, eval_windows=19683, corpus train.tok sha256 9631d315... pinned match):
  valid_loss 3.2292 valid_ppl 25.26 valid_full_loss 3.2352 valid_full_ppl 25.41
  == frozen HF eval_8000_w19683.txt bit-for-bit in printed precision.

PILOT: resume 8000 -> 8010, 10 steps, LAMB lr floor 1e-6, seed 1729, real TinyStories corpus.
  step 8010: valid_loss 3.2287 (25.25) full 3.2354 (25.42)
  DELTA: last -0.0005 (improved), full +0.0002 (worsened) -> no convincing change (expected: 10 steps at lr floor).
RELOAD PROOF: resume=8010 loads new state, from_step 8010, step 8011 trained, ckpt_8011 written and read back byte-equal.

EXECUTION_VERDICT: PASS (resume executes end to end, receipts at each stage)
QUALITY_VERDICT: INCONCLUSIVE (delta within noise; no quality claim made)
NOT RESTORED / MEASURED: tokens/sec and s/step show 0.00 in resumed runs (profile timers reset on resume; separate defect to file)
Synthetic smoke (pilot A) was used only for diagnosis, never as quality evidence.
