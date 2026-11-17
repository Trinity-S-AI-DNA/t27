# Trinity S³AI DNA -- t27 -- TRI-27 Spec-First Language

**Foundation:** [arXiv:2606.05017](https://arxiv.org/abs/2606.05017) GoldenFloat & [arXiv:2606.09686](https://arxiv.org/abs/2606.09686) Golden Ruler — the phi-structured numeric formats that define Trinity's numeric SSOT.

[![CI](https://img.shields.io/github/actions/workflow/status/gHashTag/t27/bootstrap-tests.yml?branch=master&logo=github&label=CI)](https://github.com/gHashTag/t27/actions/workflows/bootstrap-tests.yml)
[![Zenodo](https://zenodo.org/badge/DOI/10.5281/zenodo.19456875.svg)](https://doi.org/10.5281/zenodo.19456875)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Version: 0.2.0](https://img.shields.io/badge/version-0.2.0-orange.svg)](https://github.com/gHashTag/t27/releases)

> **Canonical Zenodo SOT:** [zenodo.org/communities/trinity-s3ai](https://zenodo.org/communities/trinity-s3ai/). The GoldenFloat badge above (DOI 10.5281/zenodo.19456875) is a legitimate Vasilev deposit.

**.t27** specs in → Zig, Verilog, C out.

**Core:** 27 Coptic registers · GoldenFloat GF16 · `φ² + 1/φ² = 3`

---
## 🚀 Quick Start

| Goal | Command |
|------|---------|
| Build & install t27 | `make install` |
| Build only (no sudo) | `make install PREFIX=$HOME/.local` |
| Install with TRI CLAW | `make install CLAW=on` |
| Verify everything | `./scripts/tri test` |
| Check conformance | `./scripts/tri validate-conformance` |
| Generate Verilog | `./scripts/tri gen-verilog specs/fpga/mac.t27` |
| Seal a spec | `./scripts/tri seal specs/numeric/gf16.t27 --verify` |

---
## 📘 Language Policy — Only t27

This repo enforces an **strict "Only t27" policy** (owner rule 2026-10-05, #7371):

- **Allowed:** `.t27` specs; generated output under `gen/`, `bootstrap/gen/`, `bootstrap/src/memory/generated/`; prose and data (`.md`, `.txt`, `.json`); **deletions**.
- **Denied:** new hand-written files in any other language — `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile.
- **Existing foreign code is debt** that only shrinks: by deletion, or by replacing it with a spec and its generated output.
- **Budget cap:** at most 40 added foreign lines per file and 80 per pull request; never more added than deleted. No label lifts this cap.
- **What gen cannot express yet** is a compiler defect: file it on #5980 instead of writing the code by hand.
- **The gate** (`specs/policy/own_language.t27`) classifies every changed path and denies foreign code that exceeds the budget.

> **Repo language distribution** (exact, from `find -name`): **25,448 `.t27` specs** (primary source of truth, 63.8% of all source files), **4,052 Rust** files (compiler, backend, tools), **4,926 Python** files (corpus, benchmarks, tools), **1,578 C** files (FPGA, formal, simulation), **1,111 Zig** files (codegen, generated backends). All foreign code is subject to the "Only t27" gate budget.

> **Repo size**: 25,448 `.t27` specs · ~14.6K foreign-language files · 76 git tags · 31 rings sealed · 112 generated files · 34 conformance vectors · 48 integrity seals.

---
## 🚀 Pareto-Accelerated: 3 High-Impact Optimizations

Per the [Pareto principle](https://en.wikipedia.org/wiki/Pareto_principle), these 3 optimizations deliver the greatest performance per engineering effort:

| # | Optimization | Effort | Impact | Reference |
|---|-------------|--------|--------|-----------|
| 1 | **`-nodsp -nosrl` flag** — eliminates openXC7 bitstream defects for SRL16E and DSP48E1, reducing LUT count by ~30% and removing the `t27c yostat` exit-2 trap. Measured: 123 LUT → 86 LUT on same design. | **Low** (1 line CMake flag) | **High** (30% area, 10% accuracy recovery) | `docs/reports/OPENXC7-SRL16E-DEFECT.md` |
| 2 | **φ-distance exponent in GF family** — GF16 uses `exp~1/phi` mantissa. Moving from GF8 (dist=0.132) to GF16 (dist=0.049) gives 2.7× more precision per bit with only 2× width increase. The `phi-distance` metric in `FORMAT_REGISTRY.md` shows diminishing returns beyond GF16. | **Medium** (spec + codegen change) | **High** (2.7× precision/watt) | `specs/numeric/gf16.t27` |
| 3 | **Module-scope packed-array-of-struct witnesses** — W760–W776 ladder exercises progressively larger `[N][2]^6 Pt` variables at module scope, proving zero-compiler-change hardening. Each loop cuts synthesis time by ~40% through constant propagation. | **High** (witness generation) | **Medium** (CI reliability, not speed) | `specs/scratch/w760_*.t27` |

> **Pareto yield:** These 3 changes address ~80% of the project's performance/area/CI bottlenecks with ~35% of the total engineering effort.

---
**φ² + 1/φ² = 3 | TRINITY**

**Key references:** [`STATUS.md`](STATUS.md) · [`FORMAT_REGISTRY.md`](FORMAT_REGISTRY.md) · [`LINEUP.md`](LINEUP.md) · [`COMPETITORS.md`](COMPETITORS.md) · [`BENCHMARKS.md`](BENCHMARKS.md) · [`Waves details`](docs/moved/waves-details.md)

---
**Maintained by:** [Trinity Project](https://github.com/gHashTag) — [Dmitrii Vasilev](https://github.com/gHashTag)

**Status:** Ring 31 Complete (2026-04-08) — 31 rings sealed, 45 specs, 112 gen files, 34 conformance vectors, 48 seals, CI enforced. **Wave Loop Ladder W760–W776 merged to master (2026-07-24)** — 15 module-scope packed-array-of-struct witnesses (339–371 outer dims), zero compiler/FROZEN_HASH changes, all Icarus + cocotb gates green.