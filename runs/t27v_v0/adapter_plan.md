# T27V minimal adapter plan (order T27V_V0_EVIDENCE_PUSH_AND_ADAPTER_PLAN, 2026-11-24)

Status: PLAN ONLY. No runtime change, no dependency install, no paid model until the owner
approves the specific item. V0 evidence stays procedural (REEL_TYPE: PROCEDURAL_PILLOW_FFMPEG).

## 1. Inventory (measured 2026-11-24 20:04 +07)

- w3070 (WSL2 win11-3070): Python 3.14.4, pip via `python3 -m pip` (no pip3 shim), NO node,
  NO torch, no huggingface cache, `~/t27-work` exists, `/home` has 922 GiB free of 1007 GiB.
- GPUs: GPU0 14498/16384 MiB used (Windows desktop side) - DO NOT USE. GPU1 42/16384 MiB used,
  0% util - the only GPU this lane may touch.
- Mac (coordination only): Pillow 12.1.1 + ffmpeg 8.1, no diffusion anything. V0 runner
  `runs/t27v_v0/pipeline.py` is the local procedural adapter (untracked by repo law).

## 2. Adapter boundary (one interface, many backends)

```
T27V runner contract (this lane, local-only .py until a .t27 spec exists):
  brief.json  -> adapter.run(task, brief, assets) -> artifacts + ledger + hashes
Adapters (pluggable, each behind the same run() shape):
  procedural  (shipped in V0: Pillow+ffmpeg)
  image_gguf  (M1: Qwen-Image / FLUX GGUF via a GGUF runner on w3070 GPU1)
  image_edit  (M1: Qwen-Image-Edit / FLUX Kontext-class GGUF)
  video_i2v   (M2: Pika H3 Flex, image-to-video, short/low-res)
  quantizer   (M3: weights -> GGUF/NVFP4/INT8/ONNX, registry + hashes)
```

- The formal `.t27` spec for adapters belongs to the T27V format-contract lane
  (peer session-0681887b); this lane does not write `specs/**`. Handoff note, not a claim.
- One smoke per adapter before anything batch-shaped.

## 3. Artifact schema (extends V0, additive)

```
artifact = { id, kind: image|video|frame|ui|audio, path, format, width, height,
             fps?, duration_s?, codec, bytes, sha256, created_utc,
             generator: { adapter, model?, quant?, seed, params },
             brief_ref, order_ref, t27l_ref }
```

## 4. Model registry (local json, no service)

```
registry/models.json = [ { model_id, family, quant, file, sha256, bytes, vram_est_mib,
                           adapter, license_note, installed_utc, approved_by } ]
```
A model runs only if it has an `approved_by: owner` entry. Registry lives under
`runs/t27v_v0/registry/` on w3070 work dir; mirrored into evidence on each smoke.

## 5. Hash / provenance schema

Same as V0 (`hashes.json`), extended with `generator` fields: every artifact records
adapter + model + quant + seed + params + command log path. No artifact without a hash line.
Diffusion claims require a matching `model_id` with `installed_utc` - otherwise the ledger
must say PROCEDURAL.

## 6. GPU memory budget (measured caps)

- GPU1 only, hard ceiling 14 GiB of 16384 MiB (leave >= 2 GiB for desktop spikes).
- M1 text-to-image: GGUF Q4_Q_K target <= 10 GiB weights + activations; if a candidate
  exceeds budget at Q4, drop to Q3/Q2 or reject the model - never borrow GPU0.
- M2 video: 480p first (<= 8 GiB est), then 768p if headroom allows. No 4K. Ever.

## 7. Fallback ladder (if a model does not fit or a run fails)

1. Lower quant step (Q8 -> Q6 -> Q5 -> Q4 -> Q3).
2. Lower resolution step (768p -> 512p; video 768p -> 480p -> 360p).
3. Shorter duration (8 s -> 5 s -> 3 s).
4. Back to the V0 procedural adapter so the pipeline still yields evidence.
Every fallback is recorded in the ledger as a measured line, never silently.

## 8. Smoke plans (one each, then owner review, then batch)

- M1_SMOKE_1 text-to-image: Qwen-Image GGUF Q4 on GPU1, one prompt from `visual_brief.json`,
  768p single image, seed 27, artifact + sha256 + T27L + 60 s wall-clock budget.
- M1_SMOKE_2 image-edit: Qwen-Image-Edit GGUF Q4, input = `artifacts/v0_image.png`,
  instruction = existing `edit_instruction.json`, output + hash + T27L.
- M2_SMOKE_1 image-to-video: Pika H3 Flex (or approved local i2v GGUF), input = M1 image,
  480p, 3-5 s, 30 fps, artifact + hash + T27L + visual-regression smoke (codec/fps/size).
- M3_SMOKE_1 quantizer: convert ONE approved image model to GGUF Q4, registry entry,
  round-trip hash, one inference differs-only-in-quant check.

## 9. Approval gates (unchanged)

- No dependency install without owner approval, per item (torch is NOT installed today).
- No paid model/API without owner approval.
- No push of anything but evidence; no merge; no publication.
- Adapter `.t27` spec = format-contract lane; this lane stays in `runs/t27v_v0/**`.
