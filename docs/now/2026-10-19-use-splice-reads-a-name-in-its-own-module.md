# NOW -- the `use` splice reads a name inside a pulled declaration in that declaration's module (2026-10-19)

## specs/compiler/use_splice.t27, bootstrap/src/use_resolve.rs (Closes #10556, blocks epic #10879)

- Before: two imported modules that declare one helper name made the splice refuse both, even when the name
  sat inside a pulled function and could only mean the helper next to it. `t27c gen` exited 0 and the
  generated file called a function it never declared (`tri t27o --check` could not pass `compiler.rs` or
  `main.rs`: `tri::crypto::sha256` and `tri::t27b::type_text` both declare `same`).
- Now: a name the whole closure cannot answer is read in the scope of the module whose text names it (the
  module's own declaration, else the one its direct imports make). A qualified reference the importer writes
  (`beta::name_ok`) is read in beta. Both copies are spliced under `<module path>__<name>` and the uses inside
  the module's texts are renamed. A bare name the importer writes stays UNRESOLVED.
- Also: a path a pulled declaration writes (`type_text::same` in `compound_binop`, pulled from operator_text) loses
  its module when the declaration's module imports it. The importer's own paths always did; pulled ones kept the
  qualifier and the file declared no `type_text`. This, not only the clash, is what stopped `tri t27o --check`
  on `bootstrap/src/compiler.rs`.
- The rule is `specs/compiler/use_splice.t27`; `t27c gen-rust` writes `bootstrap/gen/rust/compiler/use_splice.rs`,
  which use_resolve.rs mounts by `#[path]`. `all_agree` and `alias_target` moved there too, so the hand Rust only
  shrinks.
- Measured: `gen`, `gen-rust`, `gen-c` and `gen-verilog` over the 756 specs that carry a `use` line (the new spec
  aside), base against this branch: 20 differ (11 had an UNRESOLVED line, 9 named an undeclared path), the other 736
  are byte-identical; none of the 20 compiled before and none stopped compiling. `seal --verify` prints the same on
  both for all 756, since a seal hashes the unspliced source, so only the new spec gets a seal.
- Trap, not fixed here: `identifiers` reads `;` prose lines, so a comment that names the ambiguous name in the
  importer (the header of the issue's `importer.t27` does) still prints UNRESOLVED, though the output is right.
