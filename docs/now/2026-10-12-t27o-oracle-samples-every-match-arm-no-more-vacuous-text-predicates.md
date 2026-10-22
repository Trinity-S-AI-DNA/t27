# NOW -- t27o: the oracle samples every match arm, so text predicates can no longer pass vacuously (2026-10-12)

## specs/tools/t27o/rust_convert.t27, gen/rust/tools/t27o/rust_convert.rs (Closes #9102)

- The oracle passed every text parameter the same eight fixed samples (`""`, `"a"`, `"abc"`, `"0"`,
  `"42"`, `"0x1f"`, `"a b"`, `"-7"`). A body whose arms compare against other texts -- every
  `compound_binop` arm, for instance -- answered `None` for all eight, so the emitted
  `test oracle_*` asserted `null` eight times and passed with nothing checked. A converted body with
  the arms deleted passed the same way.
- Now every text parameter also samples the body's own string literals (each alone and wrapped in one
  space), its `b"..."` byte strings the same way, and its `b'x'` / `'x'` one-byte literals; every
  integer parameter samples the body's integer literals one below, at it and one above, kept only when
  the parameter's type holds them. The walk is `derived_at` over the fn's own `{ .. }` body, capped at
  64 samples per parameter and fully deterministic (#9102 FR-001..FR-003).
- `num_value` refuses a non-integer token as `NUM_REFUSED` (2^62, a value no literal parses to)
  instead of `src.len`: a small source is shorter than a real literal (`255` inside a 33-byte fn is a
  sample, not a refusal).
- Negative control, before (the old binary, clean `compound_binop` copied verbatim from
  `bootstrap/src/compiler.rs`): `tri t27o --check` exits 0 and emits eight `== null` asserts; with the
  `"/="` arm deleted from the harness's Rust copy the check still exits 0 -- the vacuous pass of the
  issue.
- Negative control, after (the regenerated binary): the same `--check` exits 0 with 48 asserts, ten of
  them non-null (`plant::same_text(compound_binop("/=").?, "/")` among them); with the `"/="` arm
  deleted from the harness's Rust copy it exits 1, and with the `"/="` branch deleted from the
  converted spec it exits 1. The disagreement input `"/="` exists in the vectors only after this
  change.
- Positive controls, after: `tri t27o --check` exits 0 on `has_empty_operand` (byte-literal
  `matches!`), on `is_icarus_builtin` and on the four `int_type_names` helpers
  (`zig_uint_bits`, `is_integer_type_suffix`, `int_value_bits`, `int_rank`) taken from their Rust home.
- The spec's own suite: 32 tests, 32 pass, 0 FAIL, 0 vacuous (six new tests cover the sampler: strings
  alone and wrapped, integer edges the type holds, the `clean_text` / `num_value` gates, byte and char
  literals, the 64 cap, and the emitted harness). Seal re-saved: gen_hash_rust
  sha256:a1e95a40c30f70f9596150aca65afbe5528f62a3b759f96a951f3fc54ae377a2.
