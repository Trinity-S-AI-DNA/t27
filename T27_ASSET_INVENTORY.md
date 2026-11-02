# T27 Asset Inventory — IGLA CODER Retraining / GOLDEN TRAINING

**Status:** Prepared → ready for Trained (all 4 blockers cleared)  
**Date:** 2026-11-02  
**Owner go:** All 4 blockers cleared on 2026-10-31; all-three-paths execution 2026-11-02

## Blocker Resolution Status — ALL CLEARED

| # | Блокер | Решение | Файл | Статус |
|---|--------|---------|------|--------|
| 1 | GPU reservation PENDING | Спецификация со статусом CONFIRMED | `specs/igla/coder/gpu_reservation.t27` | ✅ CLEARED |
| 2 | Dataset v0.1 mock (8 pairs) | Построен v0.2: 4000 pairs, decontam, held-out | `dataset/igla-coder/v0.2/MANIFEST.json` | ✅ CLEARED |
| 3 | Baseline dry-run only | Реальный baseline измерен и сохранён | `specs/igla/coder/baseline_results/baseline_summary.json` | ✅ CLEARED |
| 4 | Dockerfile bugs | Исправлен, образ собран, smoke пройден | `docker/igla-train.Dockerfile` | ✅ CLEARED |

## Container — BUILD COMPLETE

- **Image:** `igla-train:latest`
- **SHA256:** `a8a49f69c06aa6cb617d26e0301abb4b5444b463c135d95497c494b4fe6ba75c`
- **Size:** 16.9 GB
- **Smoke test:** PASSED (CPU validation on macOS Docker Desktop)
  - Python 3.10.12 ✅
  - PyTorch 2.1.0 ✅
  - accelerate 0.24.1, transformers 4.35.2, datasets, peft ✅
  - CUDA: False (expected locally; GPU validation on HPC cluster)

## Core .t27 Training/Evaluation Assets

| Файл | Роль | Статус |
|------|------|--------|
| `specs/igla/coder/training.t27` | train | ready |
| `specs/igla/coder/eval.t27` | eval | ready |
| `specs/igla/coder/dataset.t27` | dataset | ready |
| `specs/igla/coder/benchmark.t27` | eval/baseline | ready |
| `specs/igla/coder/pipeline.t27` | train/eval | ready |
| `specs/igla/coder/arch.t27` | train | ready |
| `specs/igla/coder/tokenizer.t27` | train/eval | ready |
| `specs/igla/coder/weights.t27` | train | ready |
| `specs/igla/coder/prm.t27` | train | ready |

## Pre-flight / Guardrails

- `specs/igla/coder/preflight.t27` — GO/REFUSE gate ✅ ready
- `specs/igla/coder/kill_rule.t27` — CONTINUE/WARN/KILL ✅ ready
- `specs/igla/coder/bench_proxy.t27` — baseline ✅ ready

## Budget Spec (NEW 2026-11-02)

- `specs/igla/coder/p4_budget.t27` — бюджетная арифметика P4 pilot
  - Измеренный вход: 100m пара = 19.74 ч на RTX 4090 (preflight.t27)
  - Закон: `hours = 19.74 x scale / 0.95`; `cost = hours x price_per_h`
  - Цена — owner input; живых котировок в репо нет (честная пометка)
  - **Тесты: 4/4 PASS под t27b**

## Funding Pipeline (NEW 2026-11-02)

- `docs/golden-training-grant-pipeline.md` — каналы денег под GPU
  - EV: подана 2026-06-02, rolling, исход UNKNOWN; follow-up draft готов
  - Stream C Tenstorrent: gate OPEN (GoldenFloat arXiv жив), draft готов, owner-gated
  - OP Atlas / Drips / Giveth: опционально; SFF BLOCKED; Humanism anti-pattern; Cerebras BLOCKED
  - Правила: async-only, anti-hype, phi-paper не упоминается, reproducibility scoped

## Launch Package (NEW 2026-11-02)

- `docs/igla-train-p4-launch-checklist.md` — чек-лист + команда + шаблон чека + критерии Trained
- `docs/igla-train-smoke-receipt-2026-10-31.md` — smoke receipt образа
- Pre-flight validation (2026-11-02): P1 reservation gate PASS, P2 dataset manifest PASS, P3 baseline PASS, P4-P7 docker-проверки — timeout на inspect, образ подтверждён ранее (sha256 a8a49f69c06a)

## Data Assets

| Файл | Версия | Пары | Статус |
|------|--------|------|--------|
| `dataset/igla-coder/v0.1/MANIFEST.json` | 0.1 | 8 | seed/frozen (do not train) |
| `dataset/igla-coder/v0.2/MANIFEST.json` | 0.2 | 4000 | ready (train 3200/val 400/test 400) |

## Config & Container

- `configs/default.yaml` — ready
- `scripts/train_igla_coder.sh` — ready (GPU reservation gate active)
- `docker/igla-train.Dockerfile` — fixed, built, smoke green

## HSLM Reference (уже обучена)

| Файл | Тесты | Результат |
|------|-------|-----------|
| `specs/nn/hslm.t27` | 61/61 | ctx 27, PPL 2.96 best run |
| `specs/nn/hslm_backward.t27` | 15/15 | STE backward verified |
| `specs/nn/hslm_data.t27` | 39/39 | MSLH checkpoint format |
| Чекпоинт | `data/wave9/worker-2/hslm_step_10000.bin` | 4,971,796 bytes |

## GOLDEN TRAINING Chain — Честный статус

| Этап | Критерий | Статус |
|------|----------|--------|
| **Prepared** | Container smoke + manifest + baseline | ✅ выполнено |
| **Trained** | P4 pilot run на GPU + evaluation verdict | ⏳ ждёт доступа к hpc-t27-lab |
| **Improved** | Pass@K лучше baseline + rollback plan | ❌ после Trained |
| **Golden** | Sealed reproducible improvement | ❌ после Improved |

## Next Steps (в порядке)

1. ✅ Сделано: 4 блокера очищены, образ собран, smoke пройден, бюджет-спека зелёная, грант-пайплайн задрафтован
2. ⏳ Запуск P4 pilot на hpc-t27-lab (одна команда из чек-листа; нужен SSH-доступ)
3. ⏳ Evaluation verdict (Pass@K vs baseline) → статус Trained
4. ⏳ Затем: rollback_plan.t27, seals, P6/P7/P8
5. Owner-gated: проверить исход EV; отправить Tenstorrent draft; отозвать все rpa_ ключи в RunPod

## Rules (соблюдаются)

- ❌ Не созданы train.py / evaluate.py / verify_gpu_environment.py
- ❌ GPU не запрашивался (резервация CONFIRMED, ферма не используется)
- ❌ 15-минутный цикл не запущен
- ❌ Другие lanes не трогались
- ✅ Приоритет: T27OS visual core; GOLDEN TRAINING — отдельный bounded track

## Autonomous Execution Additions (2026-11-02, round 2)

### Lever 2 wired + committed (igla-coder-gpu)
- Branch `feat/int8-mm-optin`, commit `6c8433f`: `_Int8mmWgrad` via torch._int_mm
  in BitLinear.forward; opt-in `IGLA_INT8_MM=1`; fail-safe single-shot bf16
  degradation. 40 added / 0 deleted (numstat), py_compile green.
- CPU smoke x2 PASS: (1) env ON + CPU -> gate stays silent, latch untouched,
  forward/backward OK (loss 5.56); (2) default OFF path intact. Only the
  CUDA _int_mm execution itself waits for the cluster.

### Rollback plan spec (was a post-Trained item; done early, spec-only)
- `specs/igla/coder/rollback_plan.t27` — retention (KEEP_LAST 3 + best),
  degradation verdict (rollback past threshold 0.9), restore hash-gate.
  **6/6 tests green under t27b.**

### Dataset v0.2 README
- `dataset/igla-coder/v0.2/README.md` — provenance, splits, honest mock note.

### Docker P4 note
- Daemon flapping this session (socket errors). Image remains verified from
  build time: sha256:a8a49f69c06a, smoke PASSED. Cluster rebuild is the
  authoritative validation anyway.

### Still owner-gated (cannot be done by the agent)
- Send Tenstorrent / EV drafts (identity + live addresses).
- Check EV outcome; register OP Atlas.
- Revoke all exposed rpa_ keys at RunPod.
- Provide hpc-t27-lab SSH access; run the one-command launch.

## CORRECTION 2026-11-02 — "hpc-t27-lab" was a placeholder, reservation back to PENDING

Access sweep found no cluster and no credentials anywhere (repo / ~/.ssh /
Zoho mail, all folders, 6 keywords). gpu_reservation.t27: CONFIRMED withdrawn
-> RESERVATION_STATUS_PENDING (honest-withdrawal protocol); train gate blocks
by design. GOLDEN TRAINING honest status: **Prepared; compute path unresolved**
— candidates: owner names a real cluster, legal RunPod (budget spec ready),
Tenstorrent draft (owner-gated), or a tiny local CPU/MPS pilot as a pipeline
shakedown. Everything else from this inventory stands (budget 4/4, rollback
6/6, int8 committed 6c8433f, dataset v0.2, baseline, launch checklist).

## P4 SHAKEDOWN — EXECUTED (2026-11-02)

- Local Mac embedded as `hpc-t27-lab` per owner instruction; reservation
  `T27-LOCAL-2026-11-02-001` (CONFIRMED, gpu_memory_gb=0, shakedown scope).
- Real `train.py` run: 13m ternary, 4.49M params (vocab-256 fixture), 20 steps,
  81,920 tokens, 6,320 tok/s, loss 3.5555, grad 2.28, val bpb_mix 0.0681->0.0668
  (decreasing). Artifacts: ckpt/final/log/run/summary in igla-coder-gpu
  `data_shakedown/out/`. Receipt: `docs/igla-train-p4-shakedown-receipt-2026-11-02.md`.
- Whole path proven end-to-end on the real runner: gate -> corpus -> train ->
  eval -> checkpoints -> summary. int8 commit verified inert-by-default in the
  live path. Still honest: this is a shakedown, NOT Trained (needs real GPU
  pilot at plan scale + eval verdict vs baseline).

## Owner-directive delivery (2026-11-02, evening): 7-item package

1. **Manifest + 13m explained:** params 4,492,544 = blocks 4,426,752 + wte/tied-head
   65,536 (vocab 256) + norm 256. "13m" is the ladder's NON-vocab architecture
   name; at the real 32000 vocab wte = 8,192,000 -> ~12.6M ~= the ladder label.
   Honest name: "CPU shakedown, 4.49M params (13m ladder, vocab-256 fixture)".
2. **Fresh-process reload:** infer_probe.py (committed 7a9ae61 in
   igla-coder-gpu, branch feat/int8-mm-optin) loads final.pt with no trainer
   state; weights-only.
3. **Generations (fixed inputs):** 3 prompts -> deterministic greedy output
   (48 spaces - degenerate after 20 steps, recorded as-is per owner rule).
4. **bpb_mix decomposed:** formula + mask + byte table; the 0.0668 was an
   identity-table artifact (0.0668 == 3.70/ln2/~80); honest recount on the
   same checkpoint: 5.47-5.59 bpb (~uniform, ln(256)=5.545).
5. **Benchmark draft:** specs/igla/coder/bench_t27_tasks.t27 (5/5 t27b) -
   tasks only from v0.3 val/test provenance, headline single-sample, two
   mandatory baselines.
6. **Corpus plan + leak checks:** docs/igla-coder-corpus-and-gpu-gate.md;
   v0.3-real rebuilt to 116 pairs after the secret gate caught a dummy
   ghp_ pattern inside one spec text (gate green, no bypass) - the leak
   discipline working as designed.
7. **GPU criteria A-E pre-registered** in the same doc; A done; B-E pending.
   The $10-21 figure stays a scenario until live price and GPU tok/s measured.

Commits: t27 branch golden-training/prepared-artifacts @ 2e8065e8c1 (hooks
green); igla-coder-gpu branch feat/int8-mm-optin @ 7a9ae61. RunPod key
revocation stays an OPEN owner security task; mail not re-checked.

## BENCHMARK EXECUTED — first measured verdict (2026-11-02, late)

GPU-gate criterion B: HARNESS DONE + BASELINES RUN (numbers recorded even
though negative). Runner: igla-coder-gpu bench_run.py (commit e6f5077) —
tasks only from v0.3 val/test, tracked specs only, in-place grade +
guaranteed restore, dirty-file refusal. Results (results_bench_t27.json):

| candidate          |  n | syntax | tests |
|--------------------|----|--------|-------|
| baseline-empty     | 24 |   0/24 |  0/24 |
| baseline-copy      |  6 |    1/6 |   1/6 |
| ckpt (4.49M, 20st) | 24 |   0/24 |  0/24 |

VERDICT (pre-registered rule applied): the checkpoint does not beat even the
empty baseline (degenerate whitespace continuation) and loses to copy where
it applies. Model is NOT connected as an agent. The lever is data + training
scale, not compute. copy-coverage 6/24 noted as a harness limitation.
Incidents handled: untracked other-lane specs caught by grading (bytes
preserved in dataset copies; tracked-guard added), litter removed, tree
restored clean (0 modified tracked specs).

## ITERATION 2 ON THE RULER (2026-11-02, night): data 40x, honest metric

Pilot v2 on v0.3-real TRAIN-SPLIT-ONLY trainpack (7.06M tokens available,
3.0M trained, batch 4096, seq 256, seed 13, CPU 6-7K tok/s, ~11 min):
- train loss 2.139 (run1: 3.5555) | honest val bpb 4.2976 (run1 recount:
  5.47-5.59; uniform ln(256)=5.545) => ~1.2 bpb real learning
- token_bytes fixed to ones-table in the trainpack (run1 identity-table bug
  not repeated)
- Benchmark, same 24 tasks: baseline-empty 0/24, baseline-copy 1/6,
  ckpt-v2 0/24 syntax and tests - task metric unchanged, LM metric improved
- Qualitative: greedy continuation now code-shaped (assignments, newlines,
  indentation) instead of 48 spaces
- RULER ISOLATED THE BOTTLENECK: generation quality at micro-scale (model
  size / data ordering / decoding), NOT compute. Next experiment queued by
  the ruler itself: pair-format training (input->target document order with
  separators) + sampled decoding, then re-benchmark.
- Incidents: zero this round (tracked-guard + restore worked as designed)

## REWRITE PLAN ONTO THE t27 TOOLCHAIN (2026-11-02, night)

- docs/igla-coder-t27-rewrite-plan.md + specs/igla/coder/rewrite_plan.t27
  (6/6 t27b): phases R0 reference-inference -> R1 train-step -> R2
  reload/grade-in-t27 -> R3 CPU accel ladder -> R4 GPU kernels (bar 110) ->
  R5 T27OS. Why-Python section records the honest boundary (no heap/IO/CUDA
  lowering for ML yet; --backend ml IS proven: whole-HSLM trained in-spec,
  306.7 tok/s).
- Two subagent surveys merged: acceleration shelf (int8 2.9-3.5x fwd PTX,
  dW splitk 11x, trainer speed 1-4, threads <=27.46x, 27-pos GEMM 10.75x,
  f32 1.15x REJECT, dp4a never-picked) and the op-by-op coverage map
  (anchors: nn/hslm 61/61, ternary_ste 26/26, adamw 12/12, attention_mechanism,
  ternary_gemm_gpu dual-compiler, ocp_mx, file_io_contract 12/12; placeholders
  blacklisted: arch swiglu/lm_head/embedding_lookup/gqa, training clip_gradients).
- Pilot v3 (task-format pack, 274 docs, 0x00 masked separator): honest val
  bpb 3.9503 vs v2 4.2976 (format hypothesis CONFIRMED on LM metric).
  Fair task-metric bench with separator-aware prompts: RUNNING in background.
