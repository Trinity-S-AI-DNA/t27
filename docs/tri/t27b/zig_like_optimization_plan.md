# Zig-Like Optimizations Plan — Implementation Plan

This document records the plan for adding Zig-style optimizations to the T27B compiler
backend. It is **planning material**, not executable specification. Any executable rules
derived from this plan belong in `specs/tri/t27b/compile_time_rules.t27`.

## Overview

This plan addresses the four key optimization areas identified in
`specs/tri/t27b/compile_time_rules.t27` (formerly
`specs/tri/t27b/compile_time_optimizations.t27`):

1. **Low-level control** — direct register access, bit-packing
2. **Aggressive inlining** — no artificial recursion limits
3. **Compile-time evaluation** — comptime functions with large lookup tables
4. **CPU-specific optimizations** — SIMD, target-specific code generation

## Research Basis

- Hybrid C++ (Wong, Stroustrup 2025)
- Template Metaprogramming (Veldhuizen 1995)
- Metaprogramming in C++ (Crowl 2024)
- LTO Studies (Kostylev 2024)

## Implementation Phases

### Phase 1: Low-level control (3 weeks)
Direct register access and bit-packing capabilities:

- Inline assembly support to .t27 specs
- Bit-packing structures support
- Remove runtime bounds checking for known-safe operations
- Native support for atomic operations
- Dependencies: none

### Phase 2: Aggressive inlining (2 weeks)
Remove artificial recursion limits and improve inlining heuristics:

- Remove depth limits for function inlining
- Inline all small functions (<10 bytes) by default
- Cost-based inlining decisions
- Inline closures and lambdas where beneficial
- Dependencies: Phase 1 completed

### Phase 3: Compile-time evaluation (4 weeks)
Advanced comptime capabilities for optimization:

- Compile-time computation of lookup tables
- Const evaluation with recursion depth
- Template metaprogramming capabilities
- Compile-time function execution
- Dependencies: Phase 2 completed

### Phase 4: CPU-specific optimizations (4 weeks)
Target-specific code generation and SIMD support:

- ARM64 NEON instruction generation
- x86 AVX/SSE instruction generation
- Auto-vectorization of loops
- CPU feature detection and conditional compilation
- Dependencies: Phase 3 completed

**Total implementation time: 13 weeks**

## Compatibility Considerations

| Feature | Backward Compatibility | Migration Path |
|---------|----------------------|----------------|
| Inline assembly | New syntax, existing code unchanged | Add inline assembly gradually via compiler flags |
| Aggressive inlining | Performance change, semantics unchanged | Compiler flag to enable/disable aggressive inlining |
| Comptime evaluation | Potential compilation time changes | Compile-time depth limits can be configured |
| SIMD optimization | Same semantics, different performance | SIMD can be disabled via compiler flag |

## Risk Assessment

| Risk | Mitigation | Probability | Impact |
|------|-----------|-------------|--------|
| Complexity increase | Keep optimizations optional and configurable | medium | medium |
| Compilation time increase | Aggressive inlining and comptime can increase compile time | high | low |
| Code size increase | Add size optimization flags and post-link optimization | medium | low |
| Portability reductions | Keep architecture-specific features optional | low | medium |

## Implementation Roadmap

1. Phase 1: Direct register access and bit-packing (3 weeks)
2. Phase 2: Aggressive inlining heuristics (2 weeks)
3. Phase 3: Compile-time evaluation capabilities (4 weeks)
4. Phase 4: SIMD and CPU-specific optimizations (4 weeks)
5. **Total implementation time: 13 weeks**

## Success Metrics

### Performance Targets

| Workload | Target Speedup |
|----------|---------------|
| Vector operations | 1.8x (80% improvement) |
| Loop benchmarks | 1.5x (50% improvement) |
| Lookup table access | 2.0x (100% improvement) |

### Compilation Time Targets

| Size | Target Increase |
|------|----------------|
| Small programs | 10% |
| Large programs | 25% |

### Code Size Targets

| Size Category | Target Increase |
|---------------|----------------|
| Small functions | 5% |
| Large functions | 15% |

### Monitoring Metrics

| Metric | Unit | Target |
|--------|------|--------|
| Execution speed | Relative speedup | 1.5x minimum across workloads |
| Memory usage | Percent change | < 10% increase |
| Compilation time | Percent change | < 25% increase |
| Code size | Percent change | < 15% increase |

### Alert Thresholds

| Metric | Warning | Critical |
|--------|---------|----------|
| Execution speed | < 1.2x speedup | < 1.0x speedup |
| Memory usage | > 15% increase | > 20% increase |
| Compilation time | > 30% increase | > 50% increase |
| Code size | > 20% increase | > 25% increase |

## Verification

### Verification Steps

1. All existing T27B tests pass
2. New benchmarks show expected speedups
3. No regressions in compilation time
4. Code size changes within acceptable bounds
5. All optimizations are optional and configurable
6. Documentation and examples provided
7. Performance regression tests created

### Success Criteria

- Minimum 1.5x speedup across workloads
- No breaking changes to existing code
- All optimizations are backward compatible
- Performance metrics within acceptable bounds
- Comprehensive test coverage

## Status

This document is **planning material**. No implementation work has been done yet;
these are backlog items. Executable rules derived from this plan live in
`specs/tri/t27b/compile_time_rules.t27`.
