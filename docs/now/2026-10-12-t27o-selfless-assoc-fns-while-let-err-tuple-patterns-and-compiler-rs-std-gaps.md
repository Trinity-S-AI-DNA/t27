# NOW -- t27o: self-less associated fns, while-let / if-let-Err / tuple patterns, and the compiler.rs std table gaps (2026-10-12)

Three acceptance gaps of the t27o Rust front end, one commit each, all measured on
`bootstrap/src/compiler.rs` (sha256 e769ef021713, 46621 lines, master 783b7bcdd).

## specs/tools/t27o/rust_convert.t27, gen/rust/tools/t27o/rust_convert.rs (Closes #9099)

- A `pub fn` with no `self` inside `impl Parser` was refused with "uses refused `Parser`"
  even when its body never touched a `Parser` value: `deps()` rewrote `Self::X` to the
  impl name and then refused the impl name itself, and a receiver bound by
  `let p: &Parser = ...` or produced by a path expression (`Parser::new(..)`,
  `Enum::A(..)`) had no owner, so the fn inherited the impl type's refusal.
- Now `Self::` inside a self-less fn resolves for naming but is not itself a refusal; a
  let-bound or path receiver picks its own impl (a `let` receiver takes its last
  binding, an enum path names its own impl); and a one-line `struct X;` impl body stubs
  out instead of refusing the whole impl, so the surviving members of a partially
  refused impl still convert.
- Before: `is_binary_op`, `is_block_boundary` and `token_continues_expr` were refused
  with "uses refused `Parser`" (grep of the issue's four names: 3; `has_empty_operand`
  left compiler.rs for `specs/compiler/hir_cost_models.t27` in #9114, so its body can
  no longer appear -- the grep criterion reads 0 either way). After: grep 0, and all
  three bodies are converted (`fn CCodegen_is_binary_op(`, `fn Parser_is_block_boundary(`,
  `fn Parser_token_continues_expr(`). Converted fns on this tree: 89 -> 98 (the four issue fns convert along with five more that shared the same owner bug).

## specs/tools/t27o/rust_convert.t27 (Closes #9101)

- `while let`, `if let Err(..) = ..` with no payload read, and if-let over a call were
  all refused as the generic "if let / while let" kind; a tuple pattern in an if-let was
  refused under the same wrong name. Now `while let` and `if let` read their call once
  into a temp (`const it : T = call();` then `if it[..]`), `if let Err(e) = e` where `e`
  is never read becomes the failure arm directly, a tuple pattern is refused naming the
  tuple (FR-003), and the whole construct is wired into the statement stream so a
  refused sub-expression no longer swallows the surrounding statements.
- Before: 8 "if let / while let" refusals. After: 1 -- `a_bang_that_is_not_a_macro_still_parses`,
  whose `e` IS read, which the issue explicitly leaves refused. Converted fns on this
  tree: 94 -> 99.

## specs/tools/t27o/rust_convert.t27, rust_std.t27, specs/tri/t27b/type_text.t27, gen (Closes #9100)

- The std table grew from 44 to 55 rows, each mapping to an existing t27 fn, never a
  duplicate: `eq` -> `plant::same_text`; `is_ascii_alphabetic`/`is_ascii_alphanumeric`/
  `is_ascii_digit` -> `type_text::alpha` / `rust_std::alnum` / `type_text::digit` with
  `use` counts (4/1/4); `chars` -> `"#"` identity; `find`/`trim_start_matches`/
  `trim_end_matches`/`strip_prefix`/`strip_suffix` in text and byte variants ->
  `rust_std::find_text`/`find_byte`, `trim_start_text`/`trim_start_byte` &c.;
  `starts_with(byte)` -> `rust_std::starts_with_byte`. `max`/`min` on integers now
  convert instead of being refused as literal-only.
- Receivers got classes: a let-bound receiver, a field-of-class receiver, a str-chain
  receiver and a call receiver each resolve their method against the owning chain, so
  `let s = f(); s.trim_start_matches(..)` and `self.name.find(..)` find their row.
  `matches!` over a call rewrites to the arm comparison instead of a macro refusal; a
  splice of depth 2 (`..${..}..`) inside `int_type_names` is now refused as R_SPLICE,
  naming the construct.
- One t27 defect surfaced and is worked around at the converter: the zig backend does
  not lower ANY open slice `x[n..]` (literal or variable low bound -- it emits the
  single-index form), while closed slices `x[n .. m]` lower correctly. The converter
  now closes every open slice over its receiver: `x[n..]` emits `x[n .. x.len]`.
  The backend fix is owner-gated Rust (only-t27 rule) and stays open.
- `type_text.t27` grows `alpha`'s use count (outside #9100's stated boundary, but the
  splice of depth 2 needed it and the alternative was a duplicate fn).
- Before: 17 `method .(is_ascii_*|chars|find|max|trim_start_matches|strip_prefix|starts_with). at`
  refusals and 4 `macro .matches. at` refusals. After: 0 and 0. Converted fns on the
  final tree: 98 -> 105 (21 kept as stubs; `tri t27o --check bootstrap/src/compiler.rs`
  exits 0, oracle test report 84 tests / 84 pass / FAIL 0).
- Suites: rust_convert 49 tests (49 pass, 0 vacuous), rust_std 9 (9 pass), type_text
  30 (30 pass). Seals re-saved: T27oRust gen_hash_rust sha256:f86e32ca..., T27oRustStd
  sha256:c72e9b40..., T27bTypeText sha256:27096adf....
