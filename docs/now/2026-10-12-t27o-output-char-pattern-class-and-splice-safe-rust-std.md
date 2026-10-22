# NOW -- t27o output: owned Strings lower to the out-buffer form, and the std table survives the splice (2026-10-12)

## specs/tools/t27o/{rust_convert,rust_std}.t27, specs/tri/t27b/type_text.t27 (Closes #9098, Closes #9103; Refs #7215)

- `fn f(...) -> String` lowers to the out-buffer form when the String is only
  created, appended to and returned: the signature becomes
  `fn assign_op_text(extra_op: []const u8, out: []u8) -> usize`, the body writes
  `var t27o_w : usize = 0;` and every append is
  `t27o_w = rust_std::append_text(out, t27o_w, X);`, with `return t27o_w;` at
  each exit (#9098 FR-001). `format!` of `{}` ints/texts/chars/bools lowers to
  those append helpers (FR-002) -- `format!(" {}= ", op)` becomes three appends
  of `" "`, `op`, `"= "`. The helpers were deduped first with
  `t27c dupes --name append_text`: one form per primitive class, no
  hand-copied variants. The oracle records a converted String result
  byte-exactly (FR-003): it unwraps the returned `usize` length against the
  out buffer the harness owns, so `tri t27o --check` asserts on the exact
  bytes, not on a Debug rendering. Anything else -- a String read past
  append/return, a String field, a String born from another fn's return --
  still refuses, naming the read (FR-004, R_STRUSE): the honest whole-file
  count is 18 `an owned String` fn refusals (master's converter has no such
  read-classification -- those fns refuse with the raw type instead), plus
  the struct refusals whose *fields* hold String -- out of FR-001's scope,
  refused by design.
- The std table picks an entry by the pattern's class, so a `char` pattern no
  longer lands on the text form: `s.contains(':')` converts to
  `rust_std::str_contains(ty, ":")` (the byte pattern read as a one-byte
  text), while `s.starts_with('(')` stays on hosting::toolchain's text fns.
  `is_simple_tuple_type` converts clean, and its file checks green
  (exit 0, 11 oracle asserts) (#9103 FR-001/FR-002). A converted fn that
  calls rust_std compiles under `t27c test-report`: a `use`d module is
  spliced into the importer by word (#7176), so rust_std's own
  `type_text::trim_lo` left `type_text` undeclared in every importer --
  the trims live in tri::t27b::type_text, beside trim_lo / trim_hi. But
  not as fns: master e0f144b88 carries them as pub fns whose gen-rust
  form (`-> &'static str`) cannot return a slice of a parameter -- three
  E0308s that leave `t27b` uncompilable. This PR deletes the pub fns and
  has the std table emit the slice compositions themselves
  (`s[trim_lo(s, 0, s.len)..s.len]` and friends), with the oracle vectors
  rust_std proved against Rust's std pinned as a test. And `==` on texts
  moved off
  mutate::plant's `same_text` onto type_text's `same`, whose gen-rust
  signature is already `&[u8]` (FR-003). rust_std keeps only what it alone
  defines. `fn w(ty: &str) -> u32 { match ty.trim() { "u8" => 8, _ => 0 } }`
  converts and its file checks green (exit 0, 10 oracle asserts). An
  unreadable param type stays refused with the nesting named and emits
  nothing (FR-004): `count_bad(rows: &[&str])` prints
  `// t27o: refused fn count_bad (line 2): type `&str` at line 2` and the
  file still checks, exit 0.
- Whole file, three stacked poisons sat between the pinned snippets and
  `bootstrap/src/compiler.rs`, each found by the whole-file run refusing a fn
  the snippets converted. All three are fixed and pinned:
  - `decl_type` read the raw source to `src.len`, keeping the LAST `name:` of
    its range -- so `fn type_rank(t: &TypeInfo)` sixty fns down typed
    `assign_op_text`'s `t` (the `extra_op.trim()` slice two lines up) as
    `&TypeInfo`, a class out_format refuses. Queries now stop at `fn_end`,
    the same boundary the K_STRLOC windows already gave the name table.
  - a catch-all match arm binding the scrutinee's own name
    (`for c in s.chars() { match c { .. c => .. } }`) emitted `const c = c;`,
    which shadows the for capture -- t27c refuses the shadow, and the
    whole-file check was BLOCKED on exactly that (escape_for_fmt). The name
    already binds the value, so no const is written; a different binder
    still gets its const.
  - `gen_mod_name` required the gen-path `#[path]` attribute to sit
    immediately before the `mod`, but compiler.rs spells every gen mod
    `#[path = ".."] #[allow(dead_code, unused_parens)] mod m;` -- so
    `operator_text` never counted as a gen module and every
    `if let Some(op) = operator_text::compound_binop(..)` binder was
    untyped. The mod now matches the end of an unbroken attribute run that
    carries a gen path.
- Whole file, the honest endpoint, measured against master e0f144b88's own
  `bootstrap/src/compiler.rs` (a file this PR does not touch): `tri t27o
  --check` (22m23s) ends blocked by exactly one thing, the allowed #7215
  class: t27c's use-splice keeps a used module's own `type_text::` qualifier
  (`spec.zig:3360:9: error: use of undeclared identifier 'type_text'`) while
  the importer's own calls are flattened -- a backend defect, reported on the
  issue, not fixed here. Nothing else blocks: the c-shadow blocker is gone
  with the match fix. The plain convert (10m33s): 1766 fn seen, 109 converted
  (master's own converter, rebuilt from the same tree without this PR's
  specs, converts 105 of the same file -- `zig_escape` and `assign_op_text`
  are exactly its two holdout refusals, both gone here), 39 of 702 tests,
  1657 refused (21 stubs), 0 `refused fn zig_escape`, 0 `refused fn
  assign_op_text`. expr_to_rust_cond's refusal is FR-004 working, not a
  poison: its String is born from `self.expr_to_rust(node)`'s return, not
  built by appends. (A 146-converted count quoted in earlier status was a
  different input file -- #9109's operator-text-extracted compiler.rs, which
  never landed on master; on the same file both converters agree.)
- Suites: rust_convert 65 tests / 0 FAIL / 0 vacuous (three stale pins from
  the earlier port were re-pinned against the actual output -- if-let temps
  now read `op_t27o`, an unused `self` renames to `self_` with no discard
  line, and a trim slice inside `same()` is parenthesized); rust_std 10/10;
  type_text 31 / 0 vacuous, all three under master e0f144b88's own t27c.
  Seals re-saved for all three modules. On the lab, against base e0f144b88:
  `tools/l2_regen_check.py --base e0f144b88 --head HEAD` is clean over all
  10 checked gen files, and the whole workspace `cargo build --release` is
  green -- while master e0f144b88 itself does not build: `t27b` dies on
  three E0308s in the old gen type_text.rs, whose trim fns return `&[u8]`
  from a `-> &'static str` signature. Those three fns are exactly what this
  PR deletes (the callers write the slices over trim_lo / trim_hi), so this
  PR repairs master's build, not only its gen staleness.
