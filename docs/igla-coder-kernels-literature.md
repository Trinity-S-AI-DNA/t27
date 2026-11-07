# IGLA-coder acceleration shelf — literature grounding

Owner order 2026-11-02: "изучи научные работы по теме" for the ternary/int8
kernel acceleration shelf. This document separates three tiers so no number is
quoted above its proof:

- **T1 measured here** — numbers from this repo's own bench and audit, with the
  command/issue that produced them.
- **T2 mathematical** — facts that hold by definition of ternary {-1,0,+1}
  weights, needing no external paper.
- **T3 literature** — external papers, with arXiv IDs from general knowledge;
  these IDs were NOT re-verified in this session (the web survey subagent hung
  and web_search returned no durable result), so they are marked UNVERIFIED
  until the paper-claims spec or a fresh fetch confirms them.

---

## T1 — measured in this repository

Source of truth: `specs/t27t/t27t.t27`, bench `gHashTag/t27#10360` (RTX 4090,
CUTLASS 3.5.1, sm_89, CUDA 12.4, 2026-10-18). All ratios stored in hundredths
(2.93x is 293).

| Claim | Value | Status |
|---|---|---|
| Forward GEMMs on int8 tensor cores (`mma.sync s8`) vs bf16 | **2.9x–3.5x** | measured |
| CUDA-core int8 `dp4a` vs bf16 | **0.91x–0.94x** (slower) | measured |
| `pick_path` picks dp4a | never | measured |
| Saving law | `f × (1 − 1/s)` = **16–24%** ($16–24 on $100) | `F_MEASURED = false` — f is an estimate |
| Runner wired to `torch._int_mm` | no | `RUNNER_USES_INT_MM = false` |
| Generated kernel ships only above CUTLASS s8 by | **10%** (`GEN_CUDA_BAR_E2`) | law |

Audit discipline: `specs/numeric/paper_claims.t27` — of 45
numbers printed in the three format papers, **34 recompute exactly, 8 misfit,
3 cannot be checked**. A paper number that misfits is reported, never smoothed.

## T2 — the packed-trits equivalence (no external paper needed)

A ternary weight is `w ∈ {-1, 0, +1}`. The forward "multiply" `w·x` is a
sign-select, not a multiply:

- `w = +1` → `+x`
- `w = -1` → `-x`
- `w =  0` → `0` (skip the MAC entirely)

So an int8 × packed-trits "GEMM" is **exactly** a conditional add/subtract with
zero-skipping. There is no rounding and no quantization error introduced by the
weight side: the only numeric error in a ternary layer comes from the activation
side (the absmax-int8 activation quantizer), never from the weight. This is why
the shelf marks `int8 × packed-trits` as `NOT_MEASURED` (its SPEED is unmeasured)
but the arithmetic itself is exact by construction. The repo already encodes the
packing: `specs/ternary/bitnet_majority.t27` packs trits as `{N=0, Z=1, P=2}`.

## T3 — external literature (IDs UNVERIFIED in this session)

Two papers are already pinned in the repo's own science notes
(`docs/igla-coder-science-notes.md`, commit 9df6279c58) with IDs verified via
the arXiv export API:

| Paper | ID (repo-verified) | One-line takeaway |
|---|---|---|
| TinyStories (Eldan & Li, 2023) | arXiv:2305.07759 | sub-10M-param models on synthetic short-story data produce coherent text — the wall is data composition/curriculum, not scale |
| Deduplicating training data (Lee et al., ACL 2022) | arXiv:2107.06499 | near-duplicate removal → ~10x less memorized output, fewer steps to equal accuracy |

The following are the standard ternary/low-bit LLM kernel references named
across the project's own docs (BitNet b1.58, llm.int8, SmoothQuant, AWQ, GPTQ,
TWN, TTQ, DeepSeek-V3 FP8). Their IDs below are **from general knowledge and
were not re-verified in this session** — do not cite them as confirmed until
the paper-claims spec or a fresh web fetch closes the loop:

| Paper | ID (UNVERIFIED) | Relevance to the shelf |
|---|---|---|
| BitNet b1.58 — "The Era of 1-bit LLMs" (Ma et al.) | 2402.17764 | ternary {-1,0,+1} LLM weights; the shelf's BitLinear precedent |
| BitNet b1.58 2B4T (Ma et al.) | 2411.04965 | native ternary training at 2B scale |
| llm.int8() (Dettmers et al., 2022) | 2208.07339 | per-channel/outlier mixed-precision; why per-token outliers break naive int8 |
| SmoothQuant (Xiao et al., 2023) | 2211.10438 | per-token activation scaling to balance outliers (maps to our absmax per-token quantizer) |
| AWQ (Lin et al., 2023) | 2306.00978 | activation-aware weight quantization |
| GPTQ (Frantar et al., 2023) | 2210.17323 | layer-wise weight quantization with Hessian |
| Ternary Weight Networks (Li et al., 2016) | 1605.04711 | the {-1,0,+1} weight formulation the shelf generalizes |
| Trained Ternary Quantization (Zhu et al., 2017) | 1612.01064 | learned ternary scales |
| DeepSeek-V3 FP8 training (2024) | 2412.19437 | FP8 training at scale; the shelf's FP8/absmax precedent |

## The honest gaps (F_MEASURED / NOT_MEASURED)

Every shelf entry that is an estimate, not a measurement, is already flagged in
the spec:

- `F_MEASURED = false` — the 16–24% saving depends on the unmeasured GEMM share f.
- `int8 × packed-trits` speed — `NOT_MEASURED` (#457); assigned to R3 before any
  forecast cites it.
- `RUNNER_USES_INT_MM = false` — the measured 2.9–3.5x tensor-core win is NOT
  yet wired into the runner; it is a bench, not a production path.

No shelf claim here exceeds what the repo measured; the T3 IDs are flagged so
the literature survey can be completed honestly the moment a working fetch is
available.
