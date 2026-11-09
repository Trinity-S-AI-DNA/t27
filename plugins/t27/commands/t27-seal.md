---
name: t27-seal
description: Seal one spec with t27c seal --save and --verify. Use after that spec's tests pass.
argument-hint: "<spec.t27>"
---

# Seal one spec

After that spec's tests pass, from the t27 repository root. Prefer
`target/release/t27c`, else `target/debug/t27c`.

Set `ZIG_GLOBAL_CACHE_DIR` and `ZIG_LOCAL_CACHE_DIR` to directories under the
temp directory, then:

```
./target/release/t27c seal <spec.t27> --save
./target/release/t27c seal <spec.t27> --verify
```

The spec path is the argument given with this command. `--verify` must report
that the hashes match. A missing binary is a missing build. Do not hand-edit
the seal or `gen/`.
