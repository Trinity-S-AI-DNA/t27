# NOW -- compiler.rs Verilog name predicates move to .t27, converted by t27o (2026-10-11)

## specs/compiler/verilog_names.t27, bootstrap/src/compiler.rs (Closes #9115; t27o epic #8998)

- Second port of the reference compiler's hand Rust through our own converter: `tri t27o --check` turned
  `is_icarus_builtin` and `is_simple_tuple_type` into t27. The oracle's 8 generic vectors sample only the
  false arm (the #9102 gap), so the spec's tests run per-arm vectors printed by rustc on the hand fns.
- The char patterns of `starts_with` / `ends_with` / `contains` are t27o's open miscompile (#9103 case 1):
  the body splices the byte forms -- `type_text::starts` for the prefix, `ty.len > 0 and ty[ty.len - 1] == ')'`
  for the suffix, `rust_std::bytes_contain` (the bytes row of t27o's own std table) for the colon. Text
  equality goes through `type_text::same`, as in #9093.
- `t27c gen-rust` writes `bootstrap/gen/rust/compiler/verilog_names.rs`, mounted by `#[path]`; the two hand
  fns are one-line calls into it, and their call sites are untouched.
- Generated output of gen, gen-verilog, gen-rust and gen-c over 2275 tracked specs: byte-identical with t27c
  built from master and from this branch; a planted fault in the generated module moves rows, so the
  comparison can see this code.
- FROZEN_HASH re-computed under the owner's standing rule for ports (2026-10-11).
