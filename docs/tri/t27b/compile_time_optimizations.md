# Compile-Time Optimizations — Design Notes

This document records the research and rationale behind compile-time optimization
investigations for the T27B backend. It is **design material**, not executable
specification. Executable rules live in
`specs/tri/t27b/compile_time_rules.t27`.

## Four optimization areas

The four areas identified as making Zig potentially faster than T27B:

1. **Low-level control** — direct register access, bit-packing
2. **Aggressive inlining** — no artificial recursion limits
3. **Compile-time evaluation** — comptime functions with large lookup tables
4. **CPU-specific optimizations** — SIMD, target-specific code generation

## Research basis

- Hybrid C++ (Wong, Stroustrup 2025): Computation-at-write benefits
- Template Metaprogramming (Veldhuizen 2023): Compile-time computation
- Metaprogramming in C++ (Crowl 2024): Inlining heuristics
- LTO Studies (Kostylev 2024): Link-time optimization

## Benchmark configurations

| Config | Description | Optimizations | Direct Register Access | Aggressive Inlining | Comptime Eval | SIMD |
|--------|-------------|---------------|------------------------|---------------------|---------------|------|
| t27b_baseline | Current T27B implementation | no | no | no | no | no |
| t27b_zig_like | T27B with Zig-style optimizations | yes | yes | yes | yes | yes |
| zig_reference | Reference Zig implementation | — | — | — | — | compiler: zig-0.13.0 |

## Workloads

| Workload | Description | Iterations | Vector Size | Matrix Size | Table Size | Access Iterations | Input Size |
|----------|-------------|------------|-------------|-------------|------------|-------------------|------------|
| vector_addition | Vector addition with SIMD optimization | 1,000,000 | 256 | — | — | — | — |
| matrix_multiplication | Matrix multiplication (small to medium) | 50,000 | — | 64 | — | — | — |
| lookup_table | Large lookup table construction and access | — | — | — | 65,536 | 1,000,000 | — |
| recursive_fibonacci | Recursive function (tests inlining depth) | 100,000 | — | — | — | — | 30 |

## Test cases

1. **Low-level control and bit-packing** — tests direct register access and bit manipulation (Hybrid C++ principles)
2. **Aggressive inlining** — tests function inlining without artificial recursion limits (Template Metaprogramming principles)
3. **Compile-time evaluation** — tests compile-time computation of large lookup tables
4. **CPU-specific optimizations** — tests SIMD and target-specific optimizations (ARM64 NEON)

## Status

This document is **research material**. The executable contract for compile-time
optimization rules lives in `specs/tri/t27b/compile_time_rules.t27`. No implementation
work has been done on these optimizations yet; they are a backlog item.
