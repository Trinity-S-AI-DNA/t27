# t27b: a fast baseline AArch64 compiler (arXiv draft)

Status: draft. Not submitted. Tracking issue: #13202.

## Sources

- Measurements: `docs/reports/t27b-bench-2026-10.md` and its raw data
  `docs/reports/t27b-bench-2026-10.json`, taken at commit
  `9a0f4c394a1474bb75103f141cc0b405d9de51b9` (2026-10-10).
- State after the snapshot (prose only, not re-measured): issue #13202,
  and on master `cli/t27b/src/main.rs` for `build --elf`, `run` and `verilog`.

## Claim ledger

| Claim | Value | Report section |
|---|---|---|
| Coverage | 1,296 / 1,311 = 98.9% | A |
| Corpus agreement | 0 / 8,344 tests disagree | A |
| Test-compile latency | 4.75 ms geomean, 18 specs | B |
| vs GCC -O0 / Zig Debug | 8.8x / 640x; 8.6x with 20 specs in one Zig run | B |
| Executed instructions | 0.91x GCC -O0, 3.4x GCC -O2 | C |
| Emulated run time | 4.1x GCC -O2, 3.7x Clang -O2 (9 specs) | C |
| Code size | 1.8x GCC -O2 | C |
| Peak memory | 5.26 MiB median | D |
| Executable | 3.08 MB (x86_64), 2.79 MB (aarch64) | D |
| Determinism | bit-identical, path-independent | E |

`CLAIMS.md` maps each claim to its evidence and lists the withdrawn and
planned ones. `LIMITATIONS.md` lists the limits. The same ratios live as
cells of `specs/tri/shootout/acceptance_matrix.t27`, tagged with the report
commit.

Negative results kept in the paper:

- Generated code is at GCC -O0 level: 3.4x the instructions of GCC -O2.
- Agreement is self-consistency, not independence.
- A source-level fuzzer found 7 disagreements in 1,000 programs.

## Build

```sh
tectonic main.tex
```

The output is `main.pdf`. `abstract.txt` is the plain-text abstract for
the arXiv form (1,629 characters; the limit is 1,920).
