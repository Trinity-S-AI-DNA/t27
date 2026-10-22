# NOW -- compiler.rs integer type names move to .t27, converted by t27o (2026-10-11)

## specs/compiler/int_type_names.t27, bootstrap/src/compiler.rs (Closes #9090; t27o epic #8998)

- First port of the reference compiler's hand Rust through our own converter: `tri t27o --check` turned
  `zig_uint_bits`, `is_integer_type_suffix`, `int_value_bits` and `int_rank` into t27, and its oracle passed.
- The bodies reuse `tri::t27b::type_text` (`trim_lo`, `trim_hi`, `same`). `t27c gen-rust` writes
  `bootstrap/gen/rust/compiler/int_type_names.rs`, mounted by `#[path]` as #8991 mounted `cse_hoist`.
- The four hand bodies are deleted, and their seven call sites call the generated module: compiler.rs +14 -71.
- Generated output of gen, gen-verilog, gen-rust and gen-c over 2270 tracked specs: byte-identical with t27c
  built from master and from this branch. A planted fault in the generated module moves 18 of those outputs,
  so the comparison can see this code.
- FROZEN_HASH re-computed under the owner's standing rule for ports (2026-10-11).
