# NOW -- compiler.rs operator text moves to .t27, converted by t27o (2026-10-11)

## specs/compiler/operator_text.t27, bootstrap/src/compiler.rs (t27o epic #8998)

- Second port of the reference compiler's hand Rust through t27o (the first is #9093): `compound_binop` and
  `CCodegen::has_empty_operand`, converted by `tri t27o --check` with their oracle passing.
- Text equality reuses `tri::t27b::type_text`'s `same`. `t27c gen-rust` writes
  `bootstrap/gen/rust/compiler/operator_text.rs`, mounted by `#[path]`; both hand bodies are deleted and
  their three call sites call the generated module.
- Tests: 45 asserts from the hand Rust (every operator, the near misses, dropped right sides), 0 vacuous.
