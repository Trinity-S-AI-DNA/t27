---
name: t27-check
description: Parse and typecheck one .t27 spec with t27c. Use after a spec edit, before tests.
argument-hint: "<spec.t27>"
---

# Check one spec

From the t27 repository root, run the `t27c` binary that is already built.
Prefer `target/release/t27c`. Use `target/debug/t27c` when release is absent.

```
./target/release/t27c check <spec.t27>
```

The spec path is the argument given with this command. Quote the compiler's
own output. A missing binary is a missing build, not a passing check. Do not
edit `gen/`. Do not treat `t27c test` as a run: it only lists tests.
