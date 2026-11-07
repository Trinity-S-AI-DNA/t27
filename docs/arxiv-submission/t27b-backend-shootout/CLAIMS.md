# Claims and evidence

Every claim in `main.tex` maps to one committed artifact:

- Report: `docs/reports/t27b-bench-2026-10.md` (sections A-E).
- Raw data: `docs/reports/t27b-bench-2026-10.json`.
- Measured revision: commit `9a0f4c394a1474bb75103f141cc0b405d9de51b9` (2026-10-10).

The measurement harness is lab scratch and is not committed. The report
lists every command it ran, and the JSON holds every sample with the
host load at the time.

## Claims made

| Id | Claim | Evidence | Status |
|---|---|---|---|
| C4 | t27b passes 1,296 of the 1,311 files the reference passes; 0 of 8,344 tests disagree | report section A, JSON | measured; self-consistency behind one front end |
| C5 | Paper tables are taken from the committed report and JSON | report sections A-E | every table cell cites a report section |
| C10 | Test-compile 4.75 ms geomean over 18 specs; 8.8x below GCC -O0; 640x below one Zig Debug invocation, 8.6x when 20 specs share one | report section B | measured on a shared host |
| C11 | 0.91x GCC -O0 and 3.4x GCC -O2 executed instructions; 4.1x GCC -O2 and 3.7x Clang -O2 (9 of 20 not folded) emulated time; 1.8x GCC -O2 size | report section C | measured; time is qemu-aarch64 emulation |
| C12 | 3.08 MB executable, one crate dependency, 5.26 MiB median peak RSS | report section D | measured |
| C13 | t27b objects and listings bit-identical within and across directories | report section E | measured |

## Withdrawn from this paper

| Id | Former claim | Why withdrawn |
|---|---|---|
| C1 | One specification passes five paths (smoke) | Its specification is not on master and no raw runs were kept |
| C2 | Heartbeat build latency on an arm64 Mac | No raw runs; the paper reports corpus latency (C10) instead |
| C3 | Native throughput close to Clang -O2 | Not supported. The corpus measurement (C11) shows 3.7x slower than Clang -O2 |

## Planned, not claimed

These rows are not claims. A `review` command named here does not exist.

| Id | What would be required | Current state |
|---|---|---|
| C6 | t27b compiles a defined external t27 IR subset to AArch64 | No external IR reader; t27b lowers only its in-memory IR |
| C7 | A defined C subset lowers to t27 IR with equivalent output | No C front end |
| C8 | Backend time reported separately from front-end time | Not measured |
| C9 | End-to-end emit fixtures with expected output hashes for gen-c, gen-rust, the Zig emitter, Verilog and Python | Not built; gen-c output has undefined behaviour across -O levels, gen-rust lowers no tests, gen-python emits declarations only |
| -- | `review smoke`, `review bench`, `review corpus`, `review tables` | Planned tooling, not implemented |
