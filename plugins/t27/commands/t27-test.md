---
name: t27-test
description: Run one spec's tests with t27b and t27c test-report. Use when behavior must be shown.
argument-hint: "<spec.t27>"
---

# Test one spec

From the t27 repository root. The spec path is the argument given with this
command. Prefer `target/release`, else `target/debug`.

1. When `t27b` exists, run `t27b test <spec.t27>`. Exit 0 is pass. Exit 1 is a
   failed test. Any other exit is not a verdict: report the code and stop.
2. Run `t27c test-report <spec.t27>` for the per-test table. Before it, set
   `ZIG_GLOBAL_CACHE_DIR` and `ZIG_LOCAL_CACHE_DIR` to directories under the
   temp directory.
3. A pass with zero runtime asserts is vacuous. Say so. Do not invent counts.

`t27c test` only lists tests. Do not quote it as a result. A missing binary is
a missing build.
