GOLDEN_CHAIN_CPU_RESUME_V2 - reproducibility slice - 2026-11-16
================================================================

PILOT_HOST (v1 pilot): macOS arm64, zig 0.16.0 (/tmp/zig-aarch64-macos-0.16.0),
t27c release built from worktree dsh-8087977d-5535 @ 22c6eb5ac7 (stale vs origin/master:
the trainer spec file was taken from origin/master commit 0bd1965960 UNCOMMITTED).
Source tree is NOT clean master; the exact overlay is:
  specs/ml/train_hslm.t27 replaced by origin/master 0bd1965960 version (sha256 4be3ffd7...)
  specs/ml/state_digest_migration.t27 added by this lane (uncommitted, 5/5 tests green)

RESUME EQUIVALENCE (measured, macOS):
  Arm A: continuous 8000 -> 8010 (10 steps), ckpt_8010 saved.
  Arm B: 8000 -> 8005, save, reload in the SAME binary invocation pattern, 8005 -> 8010.
  IMPORTANT: Arm B is equivalent ONLY with identical lr-schedule config in both arms:
  the first B attempt with total=8005 diverged (cosine target changed) -- weights differed
  from byte 64 of the payload; with total=8010 in both arms:
    ckpt_8010.state: IDENTICAL (cmp, 51,123,376 bytes)
    ckpt_8010.bin:   IDENTICAL (cmp, 4,971,796 bytes)
  => resume==continuous to the bit, under the same data/config/seed.
  (Confirms the #13168 claim independently, in this environment.)

CONFIG REGRESSION (recorded):
  resume=8000, steps=8000 (target <= restored step): runs 0 updates, exits rc 0,
  final line says "done steps 0" -- NO explicit "no updates planned" message.
  Finding: steps is an ABSOLUTE target; a clear notice + a regression test are still
  open items (would be a spec change; not done in this slice).

METRICS ROOT CAUSE (recorded, not patched):
  tokens/sec and s/step print 0.00 because tr_now() reads /proc/uptime, which does not
  exist on macOS (host_open fails -> returns 0.0). On Linux (VPS) the timers are real.
  The portable prof_now shim of #13168 covers only profile=1/2 mode, not the plain lines.

MIGRATION PROCEDURE (now a spec, not an ad-hoc script):
  specs/ml/state_digest_migration.t27 -- tests 5/5 pass, 0 vacuous (counting build works
  for this spec). Accepts ONLY the legacy marker (stored digest == 0, valid magic/version/
  nparam); refuses corrupt (nonzero digest mismatch), truncated, wrong header.
  Payload is proven byte-unchanged by a test. Real-file migration receipts:
    input  ckpt_8000.state  sha256 9034e049c61bb237... (digest field = 0)
    output ckpt_8000.state (migrated) sha256 e5bcdac0a2ff4080... (identical on macOS and VPS)
    original preserved as ckpt_8000.state.orig (both hosts)
  Trusted-source check is a receipt precondition (HF revision 39df9a77), enforced by the
  sha comparison above, NOT by the spec itself -- stated, not hidden.

VPS REPRODUCTION (givel-vps, 130.17.9.230, x86_64, 4 CPU / 7 GB RAM):
  zig 0.16.0 x86_64; corpus rebuilt in place from TinyStoriesV2-GPT4:
    train.tok sha256 9631d31544f1733e... == pinned model-card hash (EXACT MATCH)
  binaries built from the same generated Zig (hslm_train, hslm_corpus).
  Full pilot 8000 -> 8010 (eval_windows=19683) started at nice 15, no production impact;
  result appended when complete (pilot_vps.log).

SPEC TESTS STATUS (honest):
  train_hslm.t27: 41/41 pass; VACUITY VERIFICATION UNAVAILABLE on macOS this run
  (the vacuous-counting build fails: compiler defect spec.zig:988 'no field named fields
  in lang.Type.Struct'). The per-test runtime-assert counts printed by test-report come
  from the same broken counting build, so they are NOT independent evidence of non-vacuity.
  state_digest_migration.t27: 5/5 pass; vacuous 0 (counting build worked here).

QUALITY_VERDICT: INCONCLUSIVE (unchanged from V1; delta within noise, no improvement claim).

VPS RESULT (completed 2026-11-17 00:0x +07, log pilot_vps.log):
  header identical (8557893/7689354 tokens, from_step 8000)
  step 8000 eval:  3.2292 / 25.26 / 3.2352 / 25.41  == macOS pilot == frozen HF eval
  step 8010 eval:  3.2287 / 25.25 / 3.2354 / 25.42  == macOS pilot (all printed digits)
  last27 3.3169, avg 3.2422, gnorm 0.3031 -- identical
  PERFORMANCE (real timers, /proc/uptime works on Linux):
    tok_per_s 528.3, s_per_step 3.3730, train_s 33.73, wall_s 2926.38 (two 19683-window evals dominate), rc 0
  ckpt_8010 written and read back byte-equal.
CROSS-MACHINE CHECKPOINT COMPARISON:
  ckpt_8010.bin  sha256 63025a51be31ab85... IDENTICAL on macOS and VPS (ternary f32 shadow image)
  ckpt_8010.state payload: 2,938,969 of 6,390,414 f64 slots differ, MAX ABS DIFF 1.36e-16
  (one f64 ulp; arm64 vs x86_64 FMA/ordering). Header rng, step, loss identical.
  => weights bit-identical where the format rounds to f32; f64 shadows and LAMB moments
     differ by <=1 ulp across architectures. Same-architecture resume remains bit-identical.
