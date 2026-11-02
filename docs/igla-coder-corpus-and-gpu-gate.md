# IGLA CODER — bpb decomposition, corpus plan, GPU gate

**Date:** 2026-11-02 · owner directive: prove reload + one useful skill before any GPU spend

## 1. bpb_mix decomposition (item 4)

Trainer formula (`train.py evaluate()`):

```
w   = token_bytes[target_token]     # BYTE LENGTH of token t in the real tokenizer table
mask = w > 0                        # zero-length (special separator) tokens drop out of BOTH sums
bpb = sum(per_token_loss_nats[mask]) / ln(2) / sum(w[mask])
```

**Fixture bug found and owned:** the shakedown run passed
`token_bytes = np.arange(256, uint8)` — an identity table whose VALUE is the
ASCII code, so byte_sum counted each token as ~80 "bytes" (mean ASCII value of
source text), deflating bpb by that factor. The reported 0.0668 is exactly
`loss 3.70 nats/tok / ln(2) / ~80`. With the honest 1-byte-per-token table
(byte-level vocab fixture), the same checkpoint scores **5.47–5.59 bpb**
(≈ uniform, ln(256) = 5.545 — expected after 20 steps / 82K tokens).
No comparison to the 0.4722 ladder is valid from this fixture.

## 2. Corpus plan (item 6)

**v0.3-real (built today):** 117 real pairs `specs/**/*.t27 -> gen/**` from
one HEAD; train/val/test = 99/11/7 by a deterministic hash split; every pair
carries both sha256 + source rel path; manifest `dataset/igla-coder/v0.3/`.
**Leak checks:** (a) splits are disjoint FILE sets by construction;
(b) benchmark tasks (bench_t27_tasks.t27) may originate only from val/test
files and are rejected if their provenance appears in train;
(c) external-corpus decontam is required before any public benchmark claim —
self-generated data cannot be externally decontaminated by definition.

**Scale-up path (no round-number goals):** emit more backends (verilog/zig)
over the remaining spec set, then sibling repos; grow by measuring learning
curves on the fixed benchmark, not by a token-count target.

## 3. GPU gate (item 7) — pre-registered criteria

The next GPU run is authorized only when ALL of:

| # | Criterion | Status |
|---|-----------|--------|
| A | `final.pt` reloads in a fresh process and generates deterministically | ✅ done (infer_probe.py, 3 fixed prompts) |
| B | Benchmark harness exists with baselines run and numbers recorded (even if the model loses to baselines) | spec drafted (5/5); harness + baselines pending |
| C | Real corpus ≥ the size at which val loss on the FIXED metric moves; leak checks green | v0.3-real built; movement test pending |
| D | Budget inputs confirmed: live $/h price, measured tok/s scaling from THIS runner, checkpoint frequency, stop conditions, kill thresholds armed | budget law ready; live price + GPU tok/s pending |
| E | Rollback plan sealed and drill executed once on CPU | spec 6/6; drill pending |

The $10–21 figure remains a scenario until D is measured.
DOCS-NOTE: INT8 stays a separate experiment (E-phase of the owner plan): never
switch arithmetic and scale in the same run.
