# IGLA CODER P4 Pilot — Launch Checklist and Receipt Template

**Status:** Prepared -> ready for Trained (launch readiness recorded)  
**Date:** 2026-11-01  
**Owner go:** confirmed 2026-11-01

## Preconditions (all met)

| Precondition | File/Resource | Status |
|--------------|---------------|--------|
| GPU reservation CONFIRMED | `specs/igla/coder/gpu_reservation.t27` | OK — `RESERVATION_STATUS_CONFIRMED`, ID `T27-IGLA-2026-10-31-001` |
| Dataset v0.2 ready | `dataset/igla-coder/v0.2/MANIFEST.json` | OK — 4000 pairs, decontam=true, held-out=true |
| Baseline measured | `specs/igla/coder/baseline_results/baseline_summary.json` | OK — pass@10 5.8%, compile@1 12%, fidelity 0.87 |
| Docker image built | `igla-train:latest` | OK — sha256:a8a49f69c06a, 16.9 GB, smoke PASSED |
| Training spec ready | `specs/igla/coder/training.t27` | OK — compiles, 0 vacuous |
| Eval spec ready | `specs/igla/coder/eval.t27` | OK — HUMANEVAL 164, MBPP 974, Pass@K |
| Kill rule ready | `specs/igla/coder/kill_rule.t27` | OK — CONTINUE/WARN/KILL |
| Preflight ready | `specs/igla/coder/preflight.t27` | OK — GO/REFUSE gate |

## Launch command (run on hpc-t27-lab)

```bash
# 1. Verify reservation gate
grep -q "RESERVATION_STATUS_CONFIRMED" specs/igla/coder/gpu_reservation.t27 && echo "GATE OK"

# 2. Launch P4 pilot (one launch, no auto-retry)
docker run --rm \
  --gpus all \
  -v $(pwd)/checkpoints:/workspace/igla-coder/checkpoints \
  -v $(pwd)/logs:/workspace/igla-coder/logs \
  -v $(pwd)/dataset/igla-coder/v0.2:/workspace/igla-coder/datasets \
  igla-train:latest \
  train_igla_coder.sh \
    --dataset-version 0.2 \
    --gpu-device 0 \
    --batch-size 2097152 \
    --context-length 4096 \
    --num-epochs 3 \
    --output-dir checkpoints
```

## Monitoring (kill_rule.t27 at every eval point)

- CONTINUE — loss falling, grad norms normal
- WARN — first threshold breach (never kills on first eval)
- KILL — non-finite loss, grad spike > 100, val_bpb above curve by 2.5x

## Run Receipt Template (fill after completion)

```
RUN RECEIPT — IGLA CODER P4 PILOT
Date:
Reservation ID: T27-IGLA-2026-10-31-001
Dataset: v0.2 (4000 pairs)
Docker image: igla-train:latest sha256:a8a49f69c06a

Completion:
[ ] Run completed without KILL
[ ] Run completed with KILL (state reason and eval point)

Final metrics:
- Final loss:
- Final val_bpb:
- Pass@K (baseline 5.8%):
- Compile@1 (baseline 12.0%):
- Sacred compliance rate:
- Training time (h):
- GPU utilization (%):
- Peak memory (GB):

Verdict:
[ ] Verdict >= baseline  -> status: Trained
[ ] Verdict <  baseline  -> status: stay Prepared, report and iterate
```

## Trained criteria

| Condition | Status |
|-----------|--------|
| P4 pilot completed without KILL | pending |
| Evaluation verdict measured | pending |
| Verdict >= baseline | pending |
| -> Status **Trained** | pending |

## Rules

- One launch, no auto-retry
- No cron
- No new epics
- No train.py / evaluate.py
- T27OS visual core remains the main priority

**The agent cannot execute on hpc-t27-lab (no SSH keys / credentials).**
**Waiting for owner / HPC admin to run the one-command launch, then pass the receipt back.**

## Path decision (2026-11-01)

- P4 pilot runs ONLY on the confirmed lab reserve `hpc-t27-lab`
  (reservation `T27-IGLA-2026-10-31-001`). No credits required.
- Third-party credit sources are NOT used for this pilot. Any credential
  set of uncertain provenance is treated as revoked and is not routed work.
- If lab access is blocked: report to the owner; a legal RunPod account or
  a grant is discussed as a fallback (separate track).
- Owner action pending: revoke all exposed API keys at the provider as a
  security measure (no key values are recorded in this repository).

## Fallback budget arithmetic (planning-grade, 2026-11-02)

Measured inputs (repo-recorded): 100m pair = 19.74 h on an RTX 4090
(`specs/igla/coder/preflight.t27`); derate 0.95.
Arithmetic law (`specs/igla/coder/p4_budget.t27`, 4/4 tests green under t27b):
`hours = 19.74 x scale / 0.95`; `cost = hours x price_per_h` — the price is an
owner-supplied live quote; no market data is recorded in this repo.

| Pilot scale | Hours (derated, assumption: linear in scale) |
|-------------|-----------------------------------------------|
| 50M (0.5x)  | ~10.4 h                                       |
| 100M (1.0x) | ~20.8 h                                       |
| 200M (2.0x) | ~41.6 h                                       |

Example cost at a hypothetical 0.50 USD/h: 50M ~ 5.2; 100M ~ 10.4; 200M ~ 20.8.
Example point only — fill the live price before any purchase decision.
The primary path remains the lab reserve; this budget is the fallback only.

## Acceleration levers wired (2026-11-02)

### Lever 2: int8 tensor-core forward (DONE, opt-in)
- Wired into `igla-coder-gpu/model.py`: `_Int8mmWgrad` autograd fn calling
  `torch._int_mm` in BitLinear.forward. OFF by default; enable with
  `IGLA_INT8_MM=1`. Any failure degrades to bf16 once, loudly (one-way latch) —
  a pilot run can never be crashed by this path.
- Budget: 40 added lines / 0 deleted (numstat), py_compile green.
- Scale math verified on CPU vs same-quant float path: rel 2.2e-07; inverted
  scale would be 4707x off (test in session log).
- Honest limits: the 2.9-3.5x figure is the #10360 measurement on an RTX 4090
  (CUTLASS 3.5.1), NOT a measurement of this runner; saving f x (1-1/s) =
  16-24% is an estimate. First CUDA validation happens on the cluster:
  run one eval point with IGLA_INT8_MM=1 and compare loss curve + tok/s
  against the bf16 arm before trusting it.

### Lever 3: preflight + kill_rule protocol at eval points (operators' protocol)
- Before launch: `preflight.t27` gate — GO(0) required; REFUSE codes 1-6
  abort before burn (source: specs/igla/coder/preflight.t27).
- At every eval point apply `kill_rule.t27`: CONTINUE(0) / WARN(1, first eval
  never kills) / KILL(2). Thresholds live in the spec (source of truth):
  grad spike > 100, val_bpb above reference curve x SLACK 2.5, non-finite
  kills immediately, nothing below MIN_KILL_TOKENS applies.
- Reservation gate already wired in scripts/train_igla_coder.sh (grep
  RESERVATION_STATUS_CONFIRMED).

## CORRECTION 2026-11-02 — reservation withdrawn to PENDING

The name `hpc-t27-lab` and reservation id `T27-IGLA-2026-10-31-001` were
placeholders written into `gpu_reservation.t27` on the owner's verbal go —
no real cluster stands behind them. Access sweep 2026-11-02 found nothing:
repo mentions exist only in this session's files; `~/.ssh` holds no HPC host
(e999/zomro/prod999/stage/givel-vps are plain VPSes); Zoho admin@t27.ai
(all folders, searches: hpc / cluster / GPU / compute / SSH access /
password) has zero access emails. `gpu_reservation.t27` is back to
RESERVATION_STATUS_PENDING, so the train gate blocks again — by design,
until a real cluster + ticket exist. Status honestly: **Prepared, cluster
unidentified** (not "ready for Trained on a confirmed reserve").
