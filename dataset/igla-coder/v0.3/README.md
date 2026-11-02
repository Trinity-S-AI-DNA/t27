# IGLA CODER Dataset v0.3-real — real spec-to-code pairs

**phi^2 + 1/phi^2 = 3 | TRINITY**

- **116 REAL pairs** (train 98 / val 11 / test 7): input = ASCII `.t27` spec,
  target = its built artifact from `gen/` (rust 84, c 31, json 1... see
  MANIFEST target_lang counts).
- Provenance: `Trinity-S-AI-DNA/t27@HEAD`, every pair carries both sha256 and
  the source rel path. Manifest sha256 (first 16): `1f5d66ddce1a735f`.
- Honest scale: real but small — a seed for pipeline validation and few-step
  real-data runs, NOT plan-scale training volume. Scale-up: emit more backends
  over remaining specs and sibling repos.
- Decontam: self-generated corpus, held-out = disjoint file sets; external
  decontam required before any public benchmark claim.
- v0.2 (4000 mock pairs) and v0.1 (8 seed pairs) stay on disk, clearly labelled.
