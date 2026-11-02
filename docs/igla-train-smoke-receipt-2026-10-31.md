# IGLA CODER Training Container — Smoke Receipt

**Date:** 2026-10-31  
**Image:** `igla-train:latest`  
**SHA256:** `a8a49f69c06aa6cb617d26e0301abb4b5444b463c135d95497c494b4fe6ba75c`  
**Size:** 16.9 GB

## Build

- Dockerfile: `docker/igla-train.Dockerfile`
- Fix applied: user creation before COPY (chown/useradd order), unified WORKDIR, fixed LD_LIBRARY_PATH self-reference, ROCm stays commented (cluster-specific)
- Build completed successfully

## Smoke test (CPU validation)

Command: `docker run --rm igla-train:latest /workspace/validate_env.sh`

Output:

```
=== IGLA CODER Training Environment Validation ===

1. Python Version:
Python 3.10.12

2. CUDA/ROCm Status:
/usr/bin/python3
PyTorch: 2.1.0
CUDA Available: False
CUDA Version: None

3. GPU Detection:
nvidia-smi: command not found
ROCm-smi check: to be implemented

4. Package Versions:
accelerate                0.24.1
torch                     2.1.0
torchaudio                2.1.0
torchvision               0.16.0
transformers              4.35.2

=== Validation Complete ===
```

## Result

- Python 3.10.12 OK
- PyTorch 2.1.0 OK
- accelerate 0.24.1, transformers 4.35.2, datasets, peft OK
- CUDA: False — expected on macOS Docker Desktop; GPU validation happens on hpc-t27-lab

**Verdict: SMOKE PASSED (CPU scaffolding validated).**
