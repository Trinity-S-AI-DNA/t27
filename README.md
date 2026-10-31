# t27 — The Spec-First Contract Language for Bit-Exact Silicon

> **One `.t27` specification. Sealed software, tests, Zig, C, and Verilog.**  
> **No hand-editing generated code. Ever.**

**t27** is the source-of-truth toolchain for Trinity S³AI’s open,
high-assurance ternary AI silicon. Its primary path is:

```text
.t27 specification
  → conformance vectors
  → SHA-256 seals
  → Zig / C / Verilog artifacts
  → tests and differential verification
  → FPGA synthesis and board receipts
  → Tiny Tapeout reproducibility manifests
```

**φ² + 1/φ² = 3 | TRINITY**

## The Golden Series

t27 is Article 3 of **THE GOLDEN SERIES**:

| Layer | Article | What it establishes |
|---|---|---|
| **Numbers** | [GoldenFloat](https://arxiv.org/abs/2606.05017) | One phi-derived rule for the GF4–GF1024 floating-point family |
| **Verification** | [Golden Ruler](https://arxiv.org/abs/2606.09686) | Bit-exact conformance vectors across numeric formats |
| **Execution** | **GOLDEN CHAIN** | One `.t27` contract from source to silicon evidence |

The chain is directional:

```text
GoldenFloat → Golden Ruler → GOLDEN CHAIN
   numbers  →   verification  →   silicon evidence
```

## Why t27 Exists

Machine-learning formats diverge across CPU, GPU, FPGA, and ASIC.
Golden Ruler addresses numeric divergence with 109 formats, six bit-exact
conformance packs, and explicit disclosure of unresolved differences.

t27 extends that principle from numbers to systems:

- **Spec-first:** `.t27` is the only source of truth.
- **Generated artifacts:** Zig, C, and Verilog are outputs, never sources.
- **Sealed evidence:** specifications and artifacts carry SHA-256 seals.
- **Bit-exact verification:** conformance vectors are language-agnostic.
- **Hardware path:** `.t27` can become synthesizable Verilog and FPGA evidence.

## Core Principle

```text
No hand-editing generated code. Ever.
```

If a generated artifact is wrong, fix the `.t27` specification, the
compiler, or the conformance suite—not the generated file.

## Quick Start

Requires Rust/Cargo on macOS or Linux.

```bash
git clone --recursive https://github.com/Trinity-S-AI-DNA/t27.git
cd t27

make install                        # build tri, t27c and t27b, install them under /usr/local
make -n install                     # print the plan, change nothing
make install PREFIX=$HOME/.local    # install under a directory you own, no sudo
make install CLAW=on                # also install TRI CLAW, the merge and gate agent
make uninstall                      # remove exactly the files the install listed
make help
tri --version                       # with PREFIX/bin on your PATH
```

A second `make install` changes nothing. `CLAW=on` installs TRI CLAW's service spec
(`specs/claw/service.t27`) with its seals and verifies them with `t27c seal --verify`.
No lab node ships in this repository yet, so `make install EARN=on` stops before it
changes anything.

Verify the repository:

```bash
./scripts/tri test
./scripts/tri validate-conformance
./scripts/tri validate-gen-headers
```

Use the toolchain:

```bash
# Parse a specification
./scripts/tri parse specs/base/types.t27

# Generate Zig
./scripts/tri gen specs/numeric/gf16.t27

# Generate Verilog
./scripts/tri gen-verilog specs/fpga/mac.t27

# Generate C
./scripts/tri gen-c specs/base/ops.t27

# Verify a seal
./scripts/tri seal specs/numeric/gf16.t27 --verify
```

Full installation options, including `PREFIX`, `CLAW=on`, and `EARN=on`,
are in [`QUICK_START.md`](QUICK_START.md).

## The GOLDEN CHAIN

```text
.t27 specification
  ├── t27c parse          → canonical frontend
  ├── conformance/*.json  → bit-exact expected behavior
  ├── t27c seal           → SHA-256 integrity evidence
  ├── t27c gen            → Zig artifacts
  ├── t27c gen-c          → C artifacts
  ├── t27c gen-verilog    → synthesizable RTL
  ├── t27c test           → reference verification
  └── FPGA flow           → synthesis, board profile, receipts
```

The repository currently records:

- 170+ parsing specifications.
- 170+ seals under `.trinity/seals/`.
- 34 conformance-vector JSON files.
- 5/5 FPGA modules passing `gen-verilog`.
- CI gates for conformance vectors, generated headers, seal coverage,
  and issue traceability.

See [`CANON.md`](CANON.md), [`SOUL.md`](SOUL.md), and
[`docs/T27-CONSTITUTION.md`](docs/T27-CONSTITUTION.md).

## Primary Numeric Path

The default numeric path is **GoldenFloat GF16**. The GF4–GF32 family is
registered in [`conformance/FORMAT-SPEC-001.json`](conformance/FORMAT-SPEC-001.json).

| Format | Bits | Primary use |
|---|---:|---|
| GF4 | 4 | Binary masks |
| GF8 | 8 | Weights |
| GF12 | 12 | Attention |
| **GF16** | **16** | **Primary path** |
| GF20 | 20 | Training |
| GF24 | 24 | Precision |
| GF32 | 32 | Full precision |

FP8 compatibility and NF4 / INT4 / INT8 quantization bridges are **planned,
not shipped**. Details: [`FORMAT_REGISTRY.md`](FORMAT_REGISTRY.md).

## Honest Status

| Domain | Status | Evidence |
|---|---|---|
| `.t27` parsing | 🟢 GREEN | 170+ specs parse |
| Verilog generation | 🟢 GREEN | 5/5 FPGA modules synthesize |
| Sealing | 🟢 GREEN | 170+ seals exist |
| Conformance vectors | 🟢 GREEN | 34 JSON vector sets validate |
| FPGA synthesis CI | 🔴 RED | `fpga-synthesis` failing; last green 2026-04-14 |
| End-to-end bitstream CI | 🔴 RED | No `.bit` artifact uploaded since 2026-04-14 |
| Tiny Tapeout reproducibility | 🟡 YELLOW | 2/4 tracks complete; no die fabricated |

A red status is not hidden: it is part of the chain. The TTSKY26a and
TTSKY26b Tiny Tapeout submissions were withdrawn and refunded; no die has
been fabricated. See [`LINEUP.md`](LINEUP.md).

## What t27 Generates

```text
specs/*.t27              # source of truth
gen/zig/                 # generated Zig — DO NOT EDIT
gen/c/                   # generated C — DO NOT EDIT
gen/verilog/             # generated Verilog — DO NOT EDIT
conformance/*.json       # language-agnostic expected behavior
.trinity/seals/          # SHA-256 integrity evidence
```

## Repository Map

| Path | Purpose |
|---|---|
| [`specs/`](specs/) | `.t27` specifications: the only source of truth |
| [`compiler/`](compiler/) | Compiler specs: parser, codegen, CLI, runtime |
| [`gen/`](gen/) | Generated Zig, C, and Verilog artifacts |
| [`conformance/`](conformance/) | Bit-exact test vectors |
| [`bootstrap/`](bootstrap/) | Stage-0 Rust compiler |
| [`architecture/`](architecture/) | Dependency graph and ADRs |
| [`docs/`](docs/) | First-party documentation and live status |
| [`chips/`](chips/) | Tiny Tapeout chip submodules |
| [`fpga/`](fpga/) | FPGA flows, profiles, and board support |

Domain ownership begins at [`OWNERS.md`](OWNERS.md).

## Development Law

Every change follows the **PHI LOOP**:

```bash
tri skill begin <task> --issue <N>
tri spec edit <module>
tri seal <file.t27> --save
tri gen
tri test
tri verdict --toxic
tri experience save
tri skill end
git commit
```

Repository laws:

1. `.t27` specs are the only source of truth.
2. Generated Zig, C, and Verilog are never edited by hand.
3. Every mutation is bound to a GitHub issue and PR with `Closes #N`.
4. Every specification carries tests, invariants, or benchmarks.
5. GoldenFloat is defined in specs and conformance JSON, not backend code.
6. Evolution follows the architecture graph; circular dependencies are forbidden.

## Contributing

1. Open an issue first—no issue, no work.
2. Create a branch: `ring/<N>-<name>`, `fix/<name>`, or `task/<name>`.
3. Edit `.t27` specifications only.
4. Run `./scripts/tri test`, `validate-conformance`, and `validate-gen-headers`.
5. Open a PR to `master` with `Closes #N`.

See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Documentation

- [`QUICK_START.md`](QUICK_START.md) — installation and first commands
- [`CANON.md`](CANON.md) — canonical project definitions
- [`SOUL.md`](SOUL.md) — constitutional law
- [`STATUS.md`](STATUS.md) — SPEC / RTL / SIM / SYNTH / GDS / SILICON status
- [`BENCHMARKS.md`](BENCHMARKS.md) — measured results and methodology
- [`COMPETITORS.md`](COMPETITORS.md) — positioning and benchmark policy
- [`FORMAT_REGISTRY.md`](FORMAT_REGISTRY.md) — numeric formats and bridges
- [`docs/README.md`](docs/README.md) — full documentation map
- [`docs/golden-series/README.md`](docs/golden-series/README.md) — THE GOLDEN SERIES manifest (planned)

## Research

- **GoldenFloat:** [arXiv:2606.05017](https://arxiv.org/abs/2606.05017)
- **Golden Ruler:** [arXiv:2606.09686](https://arxiv.org/abs/2606.09686)
- **GOLDEN CHAIN:** this repository and its sealed `.t27` evidence chain

## License

Apache License 2.0. See [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE).

**φ² + 1/φ² = 3 | TRINITY**

## Moved Material (links to dedicated docs)

The following material was moved out of this README into dedicated documents:

| Topic | Location |
|---|---|
| Wave 11–17 plans, Track A–D | [`docs/history/WAVES.md`](docs/history/WAVES.md) |
| SEED-RINGS table | [`docs/nona-01-foundation/SEED-RINGS.md`](docs/nona-01-foundation/SEED-RINGS.md) |
| 27-agent alphabet | [`docs/agents/AGENTS_ALPHABET.md`](docs/agents/AGENTS_ALPHABET.md) |
| Automated Reasoning modules | [`docs/AUTOMATED_REASONING.md`](docs/AUTOMATED_REASONING.md) |
| BitNet HLS pipeline | [`docs/BITNET_HLS.md`](docs/BITNET_HLS.md) |
| Tiny Tapeout CLI and mĀnifests | [`docs/TINY_TAPEOUT.md`](docs/TINY_TAPEOUT.md) |
| Sacred Constants | [`docs/SACRED_CONSTANTS.md`](docs/SACRED_CONSTANTS.md) |
| GoldenFloat installation packages | [`docs/GOLDEN_FLOAT_INSTALL.md`](docs/GOLDEN_FLOAT_INSTALL.md) |
