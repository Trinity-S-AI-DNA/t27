# ORDER: T27V_UNIFIED_VISUAL_PIPELINE_V0

Unified visual stack T27V: `.t27 visual brief -> .o visual objects -> model routing -> image/video/UI generation -> edit/refine/upscale -> compose -> render -> screenshot/frame/clip -> visual regression -> T27L evidence -> T27OS/DSH UI`.

## Boundary

- V0 runtime lane: `runs/t27v_v0/**` in worktree `dsh-c954da08-9b88` (image, masked edit, UI render, reel, frames, regression, hashes, T27L).
- Format contract lane (`.t27` spec, container/codec/timeline, T27OS UI Kit) belongs to session-0681887b; no overlap, see `docs/coordination/claims/2026-11-24-t27v-v0-runtime-lane.md`.
- No publishing of visuals without owner approval. No push/PR/merge without the gate.
- No paid model without owner approval. No new dependency without approval.
- `t27b` is a `.t27`-to-native compiler, NOT a quantizer; model weight quantization (GGUF / FP4 / INT8 / ONNX) is a separate T27V quantizer lane.

## User Scenarios

- Owner gives one idea; pipeline returns one verified reel with full provenance (hashes, ffprobe facts, T27L record).
- Owner reviews one image, edits it via masked instruction, gets the edited PNG plus evidence that only the masked band changed.
- Owner renders one UI object and gets a screenshot plus visual regression verdict.

## Requirements

- Every claim carries measured evidence: ffprobe JSON, SHA-256 manifest, per-job logs, GPU probe.
- Provenance is honest: if no diffusion backend ran, artifacts say so (procedural generator).
- Target hardware: w3070, 2x RTX 3070 Laptop 16 GB. No full-res 4K video on 2x16 GB.

## Success Criteria (V0, measured 2026-11-24)

- one idea -> one image: `runs/t27v_v0/artifacts/v0_image.png` (1080x1920, seed 27)
- one image -> one edit: `v0_image_edited.png` (mask band y1340-1720, outside-band delta 0.0 luma)
- one UI -> one screenshot: `v0_ui_card.png` (1080x1350)
- one image -> one short video: `t27v_v0_reel_0001.mp4` (8.0s, 1080x1920@30, h264+aac, 1279 kbps, burned captions + procedural music)
- one video -> frames: `v0_frame_000/120/227.png`
- visual regression passes: `runs/t27v_v0/visual_regression.json` 11/11
- T27L log exists: `runs/t27v_v0/t27l_v0.json`
- full report: `runs/t27v_v0/REPORT.md`

V0 imagery is procedural (Pillow + ffmpeg, no diffusion backend on host). M1 image layer (Qwen-Image/FLUX via GGUF on w3070 GPU1) is the next milestone and requires adapter install; any paid model needs owner approval first.
