// SPDX-License-Identifier: Apache-2.0
# OWNERS — specs/memory/

## Primary

**P-Physics** — Trinity Memory Contracts are foundational to the stack; memory overlays connect to math/physics constants (φ² + φ⁻² = 3).

## Contents

Trinity Memory Contracts: `types`, `bridge`, `tensorpack`, `stream_compute`, `conformance`, `edge_demo`, `session` subdirectories, plus `formula_embed.t27`, `memory_primitives.t27`, `notebooklm.t27`, `semantic_search.t27` at this level.

## Dependencies

- `specs/base/types.t27` — language core types
- `specs/math/constants.t27` — math/physics constants

## Generates

- `gen/zig/*mem*`, `gen/c/*mem*`, `gen/verilog/*mem*` via `tri`/`t27c`

## Notes

This directory was previously unowned (an unowned path per `specs/t27os/evolve.t27`). `specs/memory/tmem/OWNERS.md` recorded its owner here as `specs/memory/OWNERS.md`; this file fixes that dangling reference.
