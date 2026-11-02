# P4 SHAKEDOWN RECEIPT — local pilot (hpc-t27-lab := local Mac)

**Date:** 2026-11-02  
**Authorization:** owner instruction "запускай локально, внедри hpc-t27-lab" — the local Mac is embedded as the lab for the shakedown track (`gpu_reservation.t27` CONFIRMED, reservation `T27-LOCAL-2026-11-02-001`, gpu_memory_gb = 0).  
**Honest scope:** pipeline shakedown ONLY. Not a GPU result. No model-quality claim. The bpb numbers use the trainer's internal definition on a 256-byte vocabulary fixture and are NOT comparable to the project's 0.4722 BPB ladder.

## Run facts (from `data_shakedown/out/run.json` / `summary.json`)

| Field | Value |
|-------|-------|
| Trainer | `igla-coder-gpu/train.py` (real runner, not a stub) |
| Model | 13m config, ternary arm, **4,492,544 params** (vocab-256 fixture) |
| Device | cpu (no CUDA on this Mac) |
| Steps / tokens | 20 / 81,920 |
| Throughput | 6,320 tok/s |
| Wall time | 0.0038 h (~14 s) |
| Train loss (last logged) | 3.5555, grad norm 2.28 (finite, healthy) |
| Val loss mix | 3.7038 (code 3.593 / docs 3.962) |
| Val bpb_mix | 0.0681 (step 10) → **0.0668** (step 20) — decreasing ✓ |
| Artifacts | `ckpt.pt`, `ckpt_predecay.pt`, `final.pt`, `log.jsonl`, `run.json`, `summary.json` |
| Corpus | repo-sourced byte streams: code 0.7 / docs 0.3, 900K train tokens, zero wraps |
| Seed | 13 |

## What this proves end-to-end

1. Reservation gate → data contract (manifest/mix/token_bytes) → trainer → STE ternary forward/backward → optimizer → eval → checkpointing → summary: the **entire one-command path works**, verified on the real `train.py`, not a stub.
2. The int8 commit (`6c8433f`) sits in the live path with the gate OFF on CPU — default run unaffected (measured: gate silent, latch untouched).
3. First real CUDA run can now be a pure A/B: same command + `IGLA_INT8_MM=1` vs off, compared at eval step 1.

## NOT claimed

- No GPU throughput/memory numbers (none exist here).
- No BPB comparison against any project ladder or competitor.
- No "Trained" status: that requires the measured P4 pilot at plan scale on a real GPU target with the evaluation verdict vs baseline.
