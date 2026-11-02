# IGLA CODER — Rewrite Plan onto the t27 Toolchain

**Date:** 2026-11-02 · Owner directive: study the .t27 improvements/accelerations and plan the rewrite of the Python pilot line onto our tool.
**Companion spec:** `specs/igla/coder/rewrite_plan.t27` (phases + gates, t27b-tested).

## 0. Why the pilot ran on Python (honest)

`t27c` lowers neither heap/file-I/O nor CUDA for the ML path (cells 24-25 of
`specs/igla/own_model.t27` record exactly what IS lowered: `--backend ml`
file fds + threads; the arena is plain t27). The int8 tensor-core path exists
only via `torch._int_mm`. Per the t27t law the Python runner is the external
backend and the .t27 layer holds the RULES (preflight/kill/budget/reservation).
The shakedown validated that external host. This plan retires it layer by layer.

## 1. Executable paths TODAY (measured)

| Path | Proven fact | Source |
|---|---|---|
| `t27c compile --backend ml` (Zig ReleaseFast) | Whole 2.13M-param HSLM **trained in-spec**: 8000 steps, 14.26M tokens, **306.7 tok/s**, PPL 25.41; real file fd (host_io_model 3/3); threads (thread_model, bit-stable N-thread sums) | own_model.t27:90-100, :76-81 |
| `t27c gen-c` + cc | 522.5M ops/s stream bench; 70-100x faster compile than clang | benchmarks/t27b-ops/RESULT.md |
| `t27b test/build` | AArch64/Mach-O/ELF for scalar-ABI pure-arithmetic fns (ternary_gemm_gpu checks math on it bit-exactly); NO structs/aggregates (exit 2) | cli/t27b/README.md; ternary_gemm_gpu.t27:10-11 |
| `t27c --backend ptx` | Accepts exactly one spec: ternary_gemm_ptx.t27 (kernel-specific emitter) | ternary_gemm_ptx.t27:8-12 |
| `t27c gen-python` | Unpushed commit #5983 — unusable | wave-audit |
| f32 trainer study | 1.1507x faster BUT REJECTed: val gap 0.0127 nats > 0.01 tolerance | ml/hslm_f32_study.t27:36-40 |

## 2. Measured accelerations ready to reuse

| Lever | Measured | Where |
|---|---|---|
| int8 tensor-core forward | **2.9-3.5x** vs bf16 (RTX 4090, 5 shapes) | specs/igla/coder/ternary_gemm_ptx.t27 (mma.sync m16n8k32, 40 tests) + t27t law: saving **16.4-23.8%** (`F_MEASURED=false` — never quote without saying so) |
| Backward dW kernel + split-k | **0.075 ms @ k=1782 = 11x CUTLASS s8**; split-k floor 48-64 slices | specs/igla/coder/hslm_bwd_ptx.t27 (28 tests) |
| Trainer speed modes 1-4 | speed1 branch-free bit-identical; speed2 skip-unscored; speed3 layer-wise windows; speed4 int8 activations | specs/ml/train_hslm.t27:54-68 |
| Threads | <= **27.46x** @48 cores (backend exists: thread_model.t27) | t27t.t27:576-604 |
| 27-position GEMM | **10.75x** — biggest lever without backend work | t27t.t27:576-604 |
| f32 | 1.15x — REJECT (quality gate) | hslm_f32_study.t27 |
| dp4a | 0.91-0.94x — never picked | t27t.t27:61-65 |
| GEN_CUDA_BAR_E2 | generated kernel must beat CUTLASS s8 by **10%** (bar 110) | t27t.t27:184-185 |
| CPU step profile | 94.2% matmul / 66.8% bwd-matmul / 42.4% outer_add | t27t.t27:555-567 |

## 3. Reuse anchors (strongest, tested)

`nn/hslm.t27` 61/61 (full ternary transformer fwd+bwd template) ·
`nn/ternary_ste.t27` 26/26 (absmean STE + true grad clip) ·
`ml/optimizer/adamw.t27` 12/12 (exact AdamW, arena-allocated moments) ·
`ml/recurrent/attention_mechanism.t27` (exact causal SDPA fwd+bwd, FD-checked) ·
`ml/train_hslm.t27` (whole-model trainer precedent, LAMB, speed modes) ·
`igla/coder/ternary_gemm_gpu.t27` (bit-exact i8xternary MMA math, dual-compiler) ·
`numeric/ocp_mx.t27` (e4m3/e5m2 round) · `ml/file_io_contract.t27` 12/12 +
`ml/host_io_model.t27` 3/3 (checkpoint/token IO) · `nn/hslm_tokenizer.t27`
(byte-level BPE, 729 vocab — swap tables for 256) · `ml/arena_heap.t27` 10/10.

**Do NOT reuse (placeholders):** arch.t27 `swiglu_scalar/vec` (relu-gate fake),
`lm_head_from_hidden`, `embedding_lookup`, `grouped_query_attention`;
`training.t27 clip_gradients` (per-VALUE clamp, wrong semantics);
`ml/transformer/norm.t27` (LayerNorm, not RMS).

## 4. Gaps

**Small (pure arithmetic):** tied LM head fwd+bwd · true SwiGLU fwd+bwd ·
GQA repeat_interleave · standard-RoPE batched cache (theta=10000) ·
`lr_at` warmup-stable-linear-decay · absmax-int8 activation quantizer
(reciprocal-scale form) · per-token ternary activation rounding (for
_TernaryWgrad) · tnf8e2/tnf8e3 magnitude tables · Fake8 per-tensor prescale ·
default_lr formula · ladder consts (13m/25m/52m/tc) · byte-level eval loop
with mask · top-k/temperature sampling on PCG32.

**Structural:** hand-composed whole-model backward (no autograd tape —
precedent: train_hslm.t27, 4433 lines, done for 3 blocks) · bf16 arithmetic ·
CUDA executor for the emitted PTX · batched KVCache at (B,kvH,T,hd) ·
memmap-style host IO + PCG64 state · atomic fs writes · bench process harness ·
OOM/SIGTERM/time-limit runtime loop.

## 5. Phases (each lands through normal gates: Closes #N, tests, seals)

### R0 — Reference inference in .t27 (target: 2-4 days of spec work)
Export `final.pt` weights -> `gen-json` -> .t27 const banks -> forward in .t27
(reuse nn/hslm layer math, swapped to GQA/RoPE-10000/SwiGLU/byte-vocab 256).
Runs under `--backend ml` with host_read for the weight file (or literal banks
for the 4.49M/vocab-256 fixture: wte 65,536 + blocks 4.43M f64... too big for
literals — file path it is; no argv: path is a string literal, documented trap).
**Gate R0 (strengthened by owner directive 2026-11-02):** logits match the
torch reference on **20 fixed prompts** (was 3), fixed seed recorded, and the
receipt carries the **sha256 of the logits from BOTH sides** (t27 reference and
torch reference); tok/s recorded. Evidence: receipt file + hashes.

### R1 — Training step in .t27
Hand-composed backward through the pilot net (per-op backwards exist: dense,
embedding, attention_mechanism, CE logits_grad; new: SwiGLU bwd, GQA bwd, tied
head bwd, per-token ternary rounding). AdamW (exists) replaces train_hslm's
LAMB; `lr_at` new; clip exists. **Gate R1:** same tokens + seed as the Python
pilot -> final loss within tolerance (target: |delta| <= 0.05 at 3M tokens on
the task-format pack); receipts from both sides.

### R2 — Reload/generate/grade loop inside t27
Checkpoint write/read via file_io_contract + host_io_model; greedy + top-k/temperature
on PCG32; the benchmark's grade step stays a host harness (spawn t27b), but the
MODEL side runs in-spec. **Gate R2:** fresh-process t27 binary reloads the
checkpoint and reproduces R0 generations bit-for-bit.

### R3 — CPU acceleration ladder (reuse, measured each step)
Order by Amdahl on the measured profile (94.2% matmul): (1) 27-position GEMM
10.75x, (2) threads to core count <=27.46x, (3) speed=4 int8 activations 1.31x,
(4) NEW: int8 x packed-trits kernel (t27t NOT_MEASURED #457 — first measurement
belongs here). **Gate R3:** tok/s multiples recorded with the command, bit-stability
tests (thread sums) stay green, quality unchanged on the fixed benchmark.

### R4 — GPU: emitted kernels only, bar 110
ternary_gemm_ptx forward (2.9-3.5x) + hslm_bwd_ptx dW/dX/splitk (11x dW);
gradients stay bf16 per pick_path; every kernel must beat CUTLASS s8 by 10%
(GEN_CUDA_BAR_E2) or it does not ship. **Gate R4:** the pilot benchmark's
tests_pass moves vs baselines, or the negative result is recorded again —
with cost/tok measured.

### R5 — T27OS integration
Device heartbeat for the local lab (devices.t27 refs), board widgets for the
ruler (benchmark rows), GOD MODE evolves specs not runners.

## 6. Sequencing rule

R0/R1 proceed NOW on CPU (no GPU, no new foreign files in t27 — new specs +
`--backend ml` binaries under gen/). The Python runner stays the experiment
backend until R2 supersedes it; every phase's evidence lands as receipts, not
prose. INT8 stays a separate experiment per owner E-phase rule.

## 7. Directive compliance (owner, 2026-11-02 evening)

- **R0 gate strengthened:** 20 fixed prompts + fixed seed + logits sha256 from
  both sides (spec consts `R0_PROMPT_COUNT/R0_SEED_FIXED/R0_LOGITS_SHA256_BOTH`,
  tests 8/8 t27b).
- **NOT_MEASURED #457:** `INT8_PACKED_TRITS_MEASURED = false` in spec law; no R3
  forecast may cite int8 x packed-trits until measured.
- **Foreign actor:** a parallel lane (synth-spec-recovery / corpus-ratchet
  repair, PR #13121 lineage) committed on the same branch while this lane
  worked (c8de1bf5, a782877e, 64d2fd97 - specs/scratch + docs/reviews paths,
  zero overlap with igla-coder files, verified `git diff` empty on our paths).
  Our commits are pathspec-only; their staged foreign files (cli/tri/src/fpga.rs
  etc.) were left staged untouched; no force-push; conflicts escalate to owner.
- **Evidence pack 2026-11-02:** t27 branch
  `golden-training/prepared-artifacts` -> 0c6c901e35 (plan+spec+inventory,
  301 insertions) and 2e8065e8c1 (datasets+specs+receipts, 167+ files);
  igla-coder-gpu branch `feat/int8-mm-optin` -> 6c8433f (int8, 40 lines),
  7a9ae61 (probe, 70), e6f5077 (bench+results, 824), 1d062f0 (pilot v2),
  5ea9a92 (pilot v3). Raw t27b tails: rewrite_plan 8 passed 0 failed
  (strengthened), bench_t27_tasks 5/0, rollback_plan 6/0, gpu_reservation
  5/0, p4_budget 4/0.
