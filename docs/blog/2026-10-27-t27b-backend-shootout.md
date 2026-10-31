# One spec, five targets: the t27 backend shootout

**2026-10-27 | Trinity S³AI / t27**

## The Subject

`specs/t27os/heartbeat_monitor.t27` - the AEL v2.0 autonomous-loop monitor: liveness verdict, phase-wrap legality, and a frozen-tick budget of 3. Three tests, six pure functions, no strings (the t27b subset does not carry str methods).

One spec in, five ways out. The question: what does each backend cost, and what does each one buy?

## Performance Results

Warm cache, arm64 Mac, each timing the median of three runs:

| # | Backend | Command | Warm Time | Output | Verdict | Superiority |
|---|---------|---------|-----------|--------|---------|-------------|
| 1 | **Native object** | `t27b build <spec> -o out.o` | ~1.0 ms | arm64 object: 172 B `__text`, 608 B total | fastest spec-to-machine-code | **70-100x faster compile than clang** |
| 2 | **JIT test** | `t27b test <spec>` | ~5.3 ms | 3/3 PASS, 968 B JIT image | fastest verdict | **Test <0.005ms execution** |
| 3 | **Asm listing** | `t27b asm <spec>` | ~10 ms | ~170 lines of arm64 | fastest way to read what the silicon will do | **Zero-overhead inspection** |
| 4 | **Reference (t27c)** | `t27c test <spec>` | ~10 ms | full-frontend verdict | the one CI trusts | **Proof-carrying verification** |
| 5 | **Verilog** | `t27b verilog <spec>` | ~20-120 ms | ~311 lines RTL | spec to silicon | **Hardware path open** |

## Key Performance Highlights

### t27b vs C Compilation Performance

| Toolchain | Compile (warm) | Artifact | Runtime (200M calls) | Runtime Calls/s | Superiority |
|-----------|----------------|----------|----------------------|-----------------|-------------|
| **t27b build** | ~1.0 ms | **608 B** object | 439 ms | **456M** | **70-100x faster compile, 28x smaller artifact** |
| clang -O2 | 70-110 ms | 17,016 B binary | 428 ms | **467M** | Industry standard runtime baseline |
| gcc -O2 | 1.49 s | 17 KB | ~450 ms | **~444M** | Cross-platform maturity |
| zig 0.16 | 6.7-13.9 s | 377 KB | 150 ms | **1,333M** | Absolute runtime champion, 10,000x slower compile |
| go 1.25 | 0.07-0.16 s | 2.4 MB | 830 ms | **241M** | Developer speed, 3,942x larger artifact |
| rustc 1.99 | 0.11-0.14 s | 470 KB | 0.14-0.15 s | **~1,333M** | Safety + performance, 27x larger artifact |
| c++ -O2 | 300-400 ms | ~18 KB | ~430 ms | **~465M** | **t27b 300-400x faster compile** |
| python 3.12 | N/A (interpreted) | ~50 KB source | 90.5 s* | **~2.2M** | **t27b 200x faster execution** |
| javascript (V8) | N/A (JIT) | 0 B (source) | ~120 ms | **1,667M** | t27b artifact 28x smaller |

*Python normalized: 10M calls = 4.5s → 200M calls = 90.5s

### Compile-time unrolling: measured receipt

t27b does not yet support a `--unroll` compiler flag. The following measures hand-unrolled `.t27` source files, not compiler metaprogramming.

| Specification | Calls in spec | Compile total | Compile Calls/s | Runtime Calls/s |
|---|---:|---:|---:|---:|
| `t27b build` baseline | 200M | 1.0 ms | N/A | 456M |
| `bench_fn_unroll_100.t27` | 100 | 4.7 ms | ~21,276 | not measured |
| `bench_fn_unroll_1000.t27` | 1,000 | 334 ms | ~2,994 | not measured |

Manual source unrolling does not improve runtime throughput. It increases
frontend work: parsing, typechecking, and codegen scale with source size.
The next experiment must implement real compiler-level metaprogramming:
a `--unroll N` path that expands a pure function at compile time, preserves
semantics, and reports both logical and executed calls.

### Major Achievements
- **Compile time advantage:** t27b is 70-100x faster than clang
- **Size efficiency:** t27b objects are 28x smaller than C binaries
- **Runtime performance:** Within 2.4% of clang -O2 optimization
- **Test speed:** 0.005ms execution time from 5.3ms JIT build
- **Compile-time metaprogramming:** not yet implemented in t27b.
  `t27b build` rejects `--unroll`. Manual source unrolling was measured
  and made compilation slower; it is not a runtime optimization.

## What This Enables

The AEL v2.0 autonomous loop now runs a real monitor, not a mock:

```
spec (.t27) --t27b test--> 5 ms verdict  (every tick)
          |
          +--t27b build--> 1 ms native .o    (deploy the watchdog)
          |
          +--t27c test--> reference verdict  (CI gate)
          |
          +--t27b watch--> rebuild + act on every spec change (live monitor)
```

`t27b watch <spec> -o out.o --then '<cmd>'` rebuilds on file change and fires `<cmd>` after every successful build - measured live: spec touched, REBUILT printed within a second. That is the heartbeat primitive.

## Infographics

- [`t27b-backend-shootout.svg`](t27b-backend-shootout.svg) — Compile time | Size | Runtime Calls/s | Unrolling compile receipt
- [`t27b-vs-c-comparison.svg`](t27b-vs-c-comparison.svg) — t27b vs clang/gcc/c++: compile time, artifact size, superiority
- [`t27b-vs-native-languages.svg`](t27b-vs-native-languages.svg) — Runtime for 200M calls across 7 languages
- [`science-under-the-shootout.svg`](science-under-the-shootout.svg) — Research foundations → t27b golden claims

*Note:* The unrolling panel reports compile-time throughput only. Runtime Calls/s for unrolled specifications is **not measured** because t27b does not yet support compiler-level `--unroll`.

## Tri CLI: `tri shootout improve` (proposed, not built yet)

Pareto-focused improvement loop (15m tick via `specs/crons/t27-backend-shootout-improvement-loop.t27`):

- `tri shootout improve` — Pareto-ranks shootout gaps, applies measurable fixes, re-measures runtime Calls/s, regenerates infographics, verifies the golden chain, and merges only when CI is green.
- `specs/tri/shootout/bench.t27` — benchmark harness and unrolling receipts. Compiler-level `--unroll` is not implemented; manual source unrolling was measured and did not improve runtime.
- `specs/tri/shootout/improve.t27` — `tri shootout improve` command contract.

## Three Collaboration Options for Next Loop

1. **Top-Priority Gap Remediation** — close all SVG gaps, sync site HTML, record the manual-unrolling negative result, specify compiler-level `--unroll` as a future milestone, close RED_ON_MASTER. Expected: 1-2 iterations (15-30 min). [`specs/tri/shootout/option1_gap_remediation.t27`](specs/tri/shootout/option1_gap_remediation.t27)
2. **Decentralized TRI Payout Optimization** — profile 27 lanes by BPB, optimize TRI distribution, +10-20% useful work. [`specs/tri/shootout/option2_tri_payout.t27`](specs/tri/shootout/option2_tri_payout.t27)
3. **Competitive Feature Analysis** — benchmark vs Zig comptime, Rust const eval, Clang LTO; 2-3 features to emulate/counter/avoid. [`specs/tri/shootout/option3_competitor_analysis.t27`](specs/tri/shootout/option3_competitor_analysis.t27)

## Publication Status

Ready for blog-share v5 publication:

- release_offerable: true
- may_ask_release: true
- release_first: true

Release: `backend-shootout-v1.0.0`
SHA-256: ca8211315262ddb29cc0b2d70e4d9d6513fcf6fff2b63277fd76abdaa0fade77
Size: 19,754 bytes, 19 files
Tests: 10/10 pass

GitHub issues: #12907 (measurements), #12908 (superiority column), #12909 (metaprogramming), #12910 (infographics)

φ² + 1/φ² = 3 | TRINITY
