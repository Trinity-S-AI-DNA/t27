# NOW -- golden functions: the hand lines in converted fns of partly converting files, and which of them move without changing a caller (2026-10-20)

## docs/now/2026-10-20-golden-functions.md (Closes #11159, step 4 of #9000; analysis only)

- Scope: no Rust is deleted and no spec is added by this change. It measures the hand-written Rust lines that sit in functions t27o
  already converts, inside files that only PARTLY convert (the whole-file version is `docs/now/2026-10-20-golden-candidates.md`, #11149,
  which this table excludes: 83 fns of 77 whole-converting trios files are left to it), and which of those fns, or groups of fns that call
  only each other, could be replaced by a generated fn mounted into the same file with `#[path]` so that the lines are deleted and no caller changes.
- Converter and driver: `tri t27o <file>` (rules: `specs/tools/t27o/rust_convert.t27`), one process per tracked `.rs` file, then `t27c gen-rust` on
  every spec that converts at least one fn. `tri` is the build of master 16c7dcf13; no commit between 16c7dcf13 and 0e00974da touches
  `specs/tools/t27o`, `gen/rust/tools/t27o` or `cli/tri/src`, so it is the converter of master. `tri t27o-sweep` prints counts only and writes no
  spec, so the per-fn data come from `tri t27o` on the same converter; lane A4's sweep summary (1,388 files) was the cross-check: for the t27
  files the converted non-test fn counts of this run equal the summary's in 87 of 96 files, the 9 others being test helper fns inside
  `#[cfg(test)]` modules (not counted here) and `bootstrap/src/compiler.rs`, which changed on master since the sweep copy.
- Sources: t27 = master 0e00974da (494 tracked `.rs`); trios = GitHub main 78671ba794 (939 `.rs`): the local trios checkout is the feature branch
  `golden-bridge-p1-bibliography` at 483dd9ee with 205 modified and 153 missing `.rs`, so the 436 files (366 `.rs`) that differ from main were fetched
  by blob sha and the rest copied; trinity = main 8e535bc9bb (1 file fetched), trinity-fpga = main c9b180bf78 (3 fetched), 999-multibots-telegraf = main
  fcdbb2f7dc (0 fetched). BrowserOS, ghashtag.github.io and trios-railway were not read.
- A converted fn is a fn with a body in the spec that no `// t27o: refused` line names (matched by its line in the Rust file, nested fns and
  fns of `#[cfg(test)]` modules are not rows). Its hand lines are the physical lines from the first contiguous `///` doc line or attribute to the closing
  brace, which is what `git diff --numstat` counts when the fn is deleted (the own-language budget counts added and deleted lines of hand-written files outside the
  generated roots `gen/`, `bootstrap/gen/`, `bootstrap/src/memory/generated/`, at most 40 added per file and 80 per PR, and a PR may not add more than it deletes).
  Blank lines between fns and `// ====` banner comments are not counted, so a real diff deletes a few more than the span. `#[cfg(test)]` lines are counted
  separately (column "cfg(test) lines" of the scale table) and are not in the fn lines.
- Exclusions (reason): generated (t27: `gen/`, `bootstrap/gen/`, `bootstrap/src/memory/generated/`, 123 files; trios: header "Generated from .t27 spec", 68), vendored
  (`vendor/`, `zig-pkg/`, `node_modules/`: trios 5, trinity 1, trinity-fpga 2), test-only (a `tests/` or `benches/` directory, `tests.rs`, `*_test.rs`: t27 158 files,
  trios 42, 999-multibots-telegraf 13; their fns are tests and test helpers that move into spec `test` blocks, not logic to port).
- Verdict rules. MOVABLE: a free fn (not in an `impl`), no cfg, proc-macro or ABI attribute, not unsafe or extern, gen-rust output that compiles, a signature gen-rust writes unchanged
  (or changed only in ways no caller feels: `const fn -> fn` with no const-context caller, `&[T]` -> `impl AsRef<[T]>`, a `&str` result -> `&'static str`), no hand-file type in it, only consts whose gen-rust
  output compiles, and every converted fn it calls MOVABLE too (a group is MOVABLE if every fn in it is). MOVABLE-WITH-CALLER-CHANGE: the same, but the call syntax or a type changes: a method or associated
  fn becomes the free fn `Type_name`, `&str` becomes `&[u8]`, `Result<(), E>` becomes `bool`, a by-reference argument becomes by-value. NOT-NOW: everything else, with the first reason. A caller that is itself a refused
  fn blocks a move only when the signature changes (it keeps calling the moved fn by name through a `use` of the mounted module, a `pub use` where the fn is public).

## Answer

- Scale: 223 partly converting files hold 817 converted fns with 6,555 hand lines (outside `#[cfg(test)]`; 44,864 `#[cfg(test)]` lines sit in the same files).
  That is 1.51% of the 435,491 physical lines lane A4 swept (the denominator of #11149), where the whole files of #11149 were 0.69%.
- MOVABLE (same name, same types, every caller unchanged): 146 fns, 1,375 hand lines (0.32% of the swept lines). MOVABLE-WITH-CALLER-CHANGE: 54 fns, 406 lines.
  NOT-NOW: 617 fns, 4,774 lines (73% of the hand lines in converted fns).
- Where the MOVABLE lines are: t27 workspace crates (bootstrap, cli/*) 217 lines in 27 fns; t27 `rings/` crates 510 in 38; t27 other crates 108 in 9; trios workspace crates 421 in 49;
  trios crates not built 110 in 14; the three other repositories 9 in 9. Methods never move without a caller change, and a fn that names a hand-file type cannot move before the type does.
- Net, if every MOVABLE bundle above 5 lines were ported with a 5-line mount each: 1036 lines out of hand files in 52 files (31 of them under 20 lines); the 40-line-per-file and 80-per-PR budget is never the limit, the fixed cost of one spec, one generated file, one seal and one ledger row per bundle is.
- Biggest call-connected group: `exp_f64` + `apply_softmax` of `rings/ring-092-rust/src/lib.rs`, 93 lines. No call-connected group reaches 200.
- First MOVABLE bundle that deletes at least 200 hand lines with every caller unchanged: **the 7 MOVABLE fns of `rings/ring-092-rust/src/lib.rs`, 201 hand lines**
  (`abs_f64` 9, `exp_f64` 45, `apply_softmax` 48, `cache_kv` 20, `compute_scores` 38, `weighted_values` 30, `identity_witness` 11), one file, one mount, five call-connected groups that do not call each other.
  Plainly: 201 lines deleted against a 5-line mount added, net -196; no other bundle exceeds 88 lines (ring-093 88, ffi 78, ring-099 46, cli/tri/src/fpga.rs 45). Section "First bundle" has the evidence.
- In the crates the t27 workspace builds the prize is small: the largest MOVABLE bundle is `cli/tri/src/fpga.rs` with 45 lines; all 27 MOVABLE fns of bootstrap and cli/* hold 217 lines.
- Valid output: 178 of the 223 specs were run through `t27c test-report` (one spec per file, a fresh ZIG_LOCAL_CACHE_DIR deleted after each run): 100 specs hold tests (377 tests, 377 pass, 0 FAIL, 57 vacuous passes);
  78 specs hold no test (their file has none), so the report has nothing to run for them and those rows lean on the rustc compile of the gen-rust output. 45 specs were NOT checked
  (stopped when the shared Mac ran low on disk under load): 45 files, the 44 rows marked `valid-output: unchecked` below (of 187 rows).
  Every row's gen-rust output was also compiled with `rustc --crate-type lib` (stub fns removed; the ring crates also under `#![no_std]`).

## Scale (partly converting files only; fns and lines are converted non-test fns)

| area | what it is | files | converted fns | hand lines | cfg(test) lines in those files | MOVABLE fns / lines | CALLER-CHANGE fns / lines | NOT-NOW fns / lines |
|---|---|---|---|---|---|---|---|---|
| W27 | t27 workspace crates (bootstrap, cli/*) | 63 | 295 | 2195 | 28194 | 27 / 217 | 29 / 229 | 239 / 1749 |
| R27 | t27 rings/ (standalone crates built by rings-rust.yml) | 17 | 142 | 1403 | 3907 | 38 / 510 | 1 / 10 | 103 / 883 |
| O27 | t27 other crates (ffi, bindings, contrib, backend; not built by the workspace) | 6 | 23 | 148 | 141 | 9 / 108 | 1 / 9 | 13 / 31 |
| WT | trios crates built by its workspace (88 members + path dependencies) | 82 | 242 | 1864 | 8743 | 49 / 421 | 9 / 106 | 184 / 1337 |
| NT | trios crates the workspace does not build | 40 | 79 | 712 | 3450 | 14 / 110 | 9 / 43 | 56 / 559 |
| OT | trinity, trinity-fpga, 999-multibots-telegraf (no workspace build checked) | 15 | 36 | 233 | 429 | 9 / 9 | 5 / 9 | 22 / 215 |
| all | | 223 | 817 | 6555 | 44864 | 146 / 1375 | 54 / 406 | 617 / 4774 |

## Why the rest cannot move now (NOT-NOW, by first reason)

| reason | fns | hand lines |
|---|---|---|
| ty-: method of, or signature names, a hand-file type with derives gen-rust drops (PartialEq, Eq, Hash, Default, serde ...): the type cannot move without breaking users | 340 | 2663 |
| ty+: method of, or signature names, a hand-file type whose derives gen-rust keeps (Debug, Clone, Copy): the type must move first | 137 | 952 |
| bad: gen-rust output does not compile (or calls a fn / needs a const that does not) | 68 | 736 |
| trait: method of a trait impl (callers dispatch through the trait) | 38 | 189 |
| attr: cfg, proc-macro or ABI attribute, or unsafe/extern | 20 | 146 |
| constctx: const fn called in a const context (gen-rust writes a plain fn) | 13 | 76 |
| dep: calls a NOT-NOW fn | 1 | 12 |

Per area (fns / lines): W27: ty+ 56/367, ty- 119/836, bad 43/400, constctx 9/59, trait 11/75, dep 1/12; R27: ty+ 14/73, ty- 71/694, bad 11/92, constctx 3/12, trait 4/12; O27: ty- 10/18, attr 2/10, trait 1/3; WT: ty+ 54/416, ty- 93/632, bad 5/88, constctx 1/5, attr 16/128, trait 15/68; NT: ty+ 11/92, ty- 34/320, bad 7/128, trait 4/19; OT: ty+ 2/4, ty- 13/163, bad 2/28, attr 2/8, trait 3/12

- 465 of the 617 NOT-NOW fns (3441 lines) are methods: gen-rust turns `Type::method(&self, ..)` into the free fn `Type_method(self_, ..)` and writes the type again in the generated module, so
  every method and every fn that takes or returns a hand-file type needs the type moved first; types that derive `PartialEq`, `Eq`, `Hash`, `Default` or serde
  lose those derives (gen-rust writes `Debug, Clone, Copy`), which is the second row. A type-move PR is the next step for the `ty+` fns, not a function move.
- `&str` parameters become `&[u8]` when the spec slices (56 free fns) or `&'static str` (9): the call sites need `.as_bytes()` or a literal, which is why those fns are CALLER-CHANGE.
- gen-rust output that does not compile: array consts (`pub const X: Vec<u64> = [..]`, `Vec<const u8>`), const-evaluated calls (E0015), `Option<T>` indexing, `&str` iteration,
  text indexing; these are gen-rust and converter defects, listed by rustc message in the appendix.

## First bundle: rings/ring-092-rust/src/lib.rs (7 fns, 201 hand lines)

- Crate `ring-092-rust` (package name), `rings/ring-092-rust`: in the root `Cargo.toml` `exclude` list, built standalone by `.github/workflows/rings-rust.yml` (Rust 1.83, `continue-on-error`).
  The names `apply_softmax`, `weighted_values`, `compute_scores` and `cache_kv` occur in no other `.rs` file of the five repositories; `identity_witness` (17 rings), `exp_f64` (ring-093) and `abs_f64` (ring-093, `compiler.rs`) are defined again in independent crates that do not depend on ring-092; `#![no_std]`, `#![forbid(unsafe_code)]`,
  `#![deny(missing_docs)]`.
- Callers: all of them are inside the file: the 28 `#[test]` fns of its `#[cfg(test)]` module (21 of them converted into the spec as tests) and, between the fns, `apply_softmax` -> `exp_f64`
  and `identity_witness` -> `abs_f64`. No refused fn calls any of the 7. Call-connected groups: {exp_f64, apply_softmax}, {abs_f64, identity_witness}, {cache_kv}, {compute_scores}, {weighted_values}.
- gen-rust signatures: identical names and types except the three fns that take `&[f64]`, which gen-rust writes `impl AsRef<[f64]>` (every call form of the file compiles; `&[]` literals would need an annotation).
  The consts the fns use (`NUM_HEADS`, `HEAD_DIM`, `EMBED_DIM`, `CONTEXT_LEN`, `SACRED_SCALE`) also convert and compile, but this count does not delete them from the hand file.
- What was built to check it (scratch only, not committed): a group-only spec of the 7 fns, its 16 consts and the 21 tests that call nothing else; `t27c gen-rust` of it (198 lines);
  a copy of the crate with the 7 fn spans deleted and a 5-line mount (`#[path]`, `#[allow(missing_docs, ..)]`, `mod golden;`, `pub use golden::{5 pub fns};`, `use golden::{abs_f64, exp_f64};`).
  `rustc --test` of the original: 28 passed, 0 failed; of the mounted copy: 28 passed, 0 failed; 201 lines out, 5 in (761 -> 565 lines). `abs_f64` and `exp_f64` are used only by tests and by
  generated code in the hand file, so the `use` line wants `#[cfg(test)]` in crates with `deny(warnings)` (rings 094-099); ring-092 only warns "unused imports".
- Differential run of the hand fns against the generated ones, same inputs, results compared by bit pattern (NaN equal to NaN), or both must panic: `exp_f64` and `abs_f64` over
  4,000,022 inputs each (specials, subnormals, +-inf, NaN, the 700 cut-off, random bit patterns, uniform and wide ranges); `apply_softmax` 300,000 arrays and `seq_len` values including out of range;
  `cache_kv` 200,000; `compute_scores` and `weighted_values` 30,000 each, with short buffers and out-of-range positions; `identity_witness`. 8,560,045 comparisons, 0 disagreements, 180,769 cases where
  both panicked (out-of-contract buffers), at `-C opt-level=0` (overflow checks on), `-C opt-level=3 -C overflow-checks=off` and `-C opt-level=3 -C overflow-checks=on`.
- `python3 -I tools/dupe_scan.py --like` on the group spec: "nothing in this file is written elsewhere" (no byte-identical body). But `specs/nn/attention.t27` already declares `cache_kv`, `compute_scores`, `apply_softmax`,
  `weighted_values` and `ternary_matmul`, with a different argument order for `compute_scores` and `weighted_values`; the crate says it mirrors that spec. A port must reconcile with it first (as the
  six `specs/port/` files do in #11149), or it adds a second spec of the same kernel.
- Shape of the pull request (not done): delete the 7 spans (-201), add the mount (+5), add the spec and the file `t27c gen-rust` writes under a generated root (for example `gen/rust/rings/`, mounted as
  `#[path = "../../../gen/rust/rings/ring092_attention.rs"]`), a seal and a t27b ledger row. The own-language budget sees +5 / -201 in one hand file.

## Bundles: the MOVABLE fns of one file, one mount per file (top 25 of 74 files)

Not all of a bundle's fns call each other: "groups" is the number of call-connected groups in it. "net" is lines minus a 5-line mount (3 if the mount attributes share one line, as in #11157).

| # | area | path | MOVABLE fns | groups | lines | net | consts the fns use (lines) | CALLER-CHANGE lines in the file | cfg(test) lines | valid output |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | R27 | rings/ring-092-rust/src/lib.rs | 7 | 5 | 201 | 196 | CONTEXT_LEN, EMBED_DIM, HEAD_DIM, NUM_HEADS +1 (18) | 0 | 355 | tr 21/21 vac 5 |
| 2 | R27 | rings/ring-093-rust/src/lib.rs | 5 | 4 | 88 | 83 | none | 0 | 498 | tr 14/14 vac 4 |
| 3 | O27 | ffi/src/lib.rs | 2 | 2 | 78 | 73 | GF16_EXP_BIAS, GF16_EXP_BITS, GF16_EXP_MASK, GF16_MANT_BITS +1 (5) | 0 | 141 | tr no tests in spec |
| 4 | R27 | rings/ring-099-rust/src/lib.rs | 5 | 5 | 46 | 41 | STAGE_COMMIT, STAGE_DONE, STAGE_GEN, STAGE_INIT +5 (10) | 0 | 328 | tr 23/23 vac 5 |
| 5 | W27 | cli/tri/src/fpga.rs | 5 | 4 | 45 | 40 | PVT_TEMP_MIN_C, PVT_VCCINT_MAX_MV (3) | 8 | 3918 | tr 12/12 vac 0 |
| 6 | R27 | rings/ring-097-rust/src/lib.rs | 3 | 1 | 45 | 40 | none | 0 | 337 | tr 4/4 vac 1 |
| 7 | WT | crates/trios-tri/src/core_compat.rs | 3 | 3 | 45 | 40 | none | 0 | 95 | valid-output: unchecked |
| 8 | R27 | rings/ring-095-rust/src/lib.rs | 6 | 5 | 40 | 35 | PHI, PHI_INV (4) | 0 | 387 | tr 7/7 vac 0 |
| 9 | W27 | bootstrap/src/math_compare.rs | 3 | 3 | 38 | 33 | none | 0 | 116 | tr no tests in spec |
| 10 | NT | crates/jepa_t_ingest/src/lib.rs | 1 | 1 | 36 | 31 | none | 0 | 62 | tr 4/4 vac 0 |
| 11 | W27 | cli/tri/src/prcheck.rs | 2 | 2 | 33 | 28 | none | 0 | 845 | tr 9/9 vac 1 |
| 12 | NT | apps/trios-macos/rings/RUST-13/trios-mesh/src/wire.rs | 5 | 3 | 32 | 27 | KIND_DATA, VERSION (2) | 0 | 34 | tr 2/2 vac 1 |
| 13 | WT | crates/trios-trainer/src/backward.rs | 1 | 1 | 29 | 24 | none | 0 | 153 | valid-output: unchecked |
| 14 | WT | tools/acm_ae_check/src/lib.rs | 3 | 3 | 29 | 24 | TRINITY_ANCHOR, ZENODO_DOI (8) | 0 | 190 | valid-output: unchecked |
| 15 | WT | crates/trios-chat/rings/CR-CHAT-07/src/egress_fingerprint.rs | 2 | 2 | 28 | 23 | CANONICAL_BURST_GAPS_MS, CANONICAL_LENGTH_CLASSES (5) | 0 | 94 | tr 2/2 vac 0 |
| 16 | R27 | rings/ring-088-rust/src/lib.rs | 2 | 1 | 27 | 22 | none | 0 | 127 | tr 1/1 vac 1 |
| 17 | O27 | bindings/javascript/src/lib.rs | 6 | 6 | 25 | 20 | none | 0 | 0 | tr no tests in spec |
| 18 | WT | crates/trios-trainer/src/forward.rs | 2 | 2 | 25 | 20 | none | 0 | 107 | valid-output: unchecked |
| 19 | WT | crates/trios-trainer/src/validation.rs | 1 | 1 | 24 | 19 | CHAMPION_MAX_BPB, CHAMPION_MIN_BPB (2) | 0 | 41 | valid-output: unchecked |
| 20 | WT | crates/trios-trainer/src/optimizer.rs | 1 | 1 | 21 | 16 | none | 0 | 272 | valid-output: unchecked |
| 21 | WT | crates/trios-tri/src/lib.rs | 1 | 1 | 21 | 16 | none | 0 | 64 | valid-output: unchecked |
| 22 | R27 | rings/ring-098-rust/src/lib.rs | 2 | 2 | 19 | 14 | none | 0 | 324 | tr 11/11 vac 2 |
| 23 | NT | apps/trios-macos/rings/RUST-05/clade-monitor/src/main.rs | 2 | 1 | 19 | 14 | none | 0 | 325 | tr no tests (binary crate spec) |
| 24 | WT | crates/trios-fpga/src/xdc.rs | 1 | 1 | 17 | 12 | none | 0 | 200 | tr no tests in spec |
| 25 | WT | crates/trios-chat/rings/CR-CHAT-07/src/lib.rs | 1 | 1 | 16 | 11 | CANONICAL_GAPS_MS (4) | 0 | 59 | tr 2/2 vac 0 |

MOVABLE lines by bundle size: under 10 lines: 35 files, 10-19: 18 files, 20-49: 18 files, 50-99: 2 files, 100-199: 0 files, 200 or more: 1 files

## Ranked table: every MOVABLE or MOVABLE-WITH-CALLER-CHANGE group (187 rows), ranked by hand lines

Legend. area: W27 t27 workspace crates, R27 t27 rings/, O27 other t27 crates, WT trios workspace crates, NT trios crates not built, OT trinity / trinity-fpga / 999-multibots-telegraf (repo named). A group is the
call-connected set of converted fns of one file that call only each other (a lone fn is a group of one); a group is MOVABLE only if every fn in it is. "callers in file": conv = another converted fn,
refused = a fn the converter refuses (it keeps calling the moved fn by name through a `use`, so it blocks only when a signature changes), test = a `#[test]` or helper in the `#[cfg(test)]` module,
other = a const/static initializer. "callers elsewhere": files outside the row that name the fn through its module path, an import or the crate name (checked by hand for every MOVABLE row that had a hit).
"gen-rust vs original": differences in the signature gen-rust writes; `const fn -> fn` matters only to a const context (checked: none for a MOVABLE row); `impl AsRef` widens. "valid output": `tr P/T vac V` is
`t27c test-report` of the file's converted spec (P passes of T tests, V vacuous passes); "gen-rust compiles" is `rustc --crate-type lib` of the generated file with the refused stubs removed.

| # | area | repo | path | fns (hand lines) | lines | callers in file | callers elsewhere | gen-rust vs original | valid output | verdict |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | R27 | t27 | rings/ring-092-rust/src/lib.rs | exp_f64 (45), apply_softmax (48) | 93 | 8 test, 1 inside group | none | same | tr 21/21 vac 5; gen-rust compiles | MOVABLE |
| 2 | O27 | t27 | ffi/src/lib.rs | encode_gf16_from_u32 (52) | 52 | 5 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 3 | R27 | t27 | rings/ring-097-rust/src/lib.rs | write_byte (8), write_usize (19), write_u64 (18) | 45 | 5 refused, 4 inside group | none | same | tr 4/4 vac 1; gen-rust compiles | MOVABLE |
| 4 | R27 | t27 | rings/ring-092-rust/src/lib.rs | compute_scores (38) | 38 | 2 test | none | &[f64] -> impl AsRef (widens) x2 | tr 21/21 vac 5; gen-rust compiles | MOVABLE |
| 5 | NT | trios | crates/jepa_t_ingest/src/lib.rs | quantize_phi_prior (36) | 36 | 1 refused, 7 test | crates/jepa_t_ingest/tests/quantize.rs (use jepa_t_ingest::quantize_phi_prior) | same | tr 4/4 vac 0; gen-rust compiles | MOVABLE |
| 6 | R27 | t27 | rings/ring-093-rust/src/lib.rs | exp_f64 (33) | 33 | 1 refused, 2 test | none | same | tr 14/14 vac 4; gen-rust compiles | MOVABLE |
| 7 | R27 | t27 | rings/ring-092-rust/src/lib.rs | weighted_values (30) | 30 | 1 test | none | &[f64] -> impl AsRef (widens) x2 | tr 21/21 vac 5; gen-rust compiles | MOVABLE |
| 8 | WT | trios | crates/trios-trainer/src/backward.rs | softmax_cross_entropy_backward (29) | 29 | 1 test | none | &[f32] -> impl AsRef (widens); &[usize] -> impl AsRef (widens) | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 9 | R27 | t27 | rings/ring-088-rust/src/lib.rs | round_half_to_even (17), libm_floor (10) | 27 | 1 refused, 1 inside group | none | same | tr 1/1 vac 1; gen-rust compiles | MOVABLE |
| 10 | R27 | t27 | rings/ring-093-rust/src/lib.rs | load_balance_loss (27) | 27 | 6 test | none | &[u32] -> impl AsRef (widens) | tr 14/14 vac 4; gen-rust compiles | MOVABLE |
| 11 | WT | trios | crates/trios-chat/rings/CR-CHAT-01/src/key_package_expiry_guard.rs | validate_key_package_expiry (27) | 27 | 10 test | crates/trios-chat/rings/CR-CHAT-01/src/lib.rs (1, import) | ret Result<(),KeyPackageExpiryError> -> bool | tr 9/9 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: kpx_01_expired_rejected, kpx_02_not_yet_valid_rejected, kpx_03_inverted_rejected, kpx_04_too_short_rejected +6] |
| 12 | O27 | t27 | ffi/src/lib.rs | decode_gf16_to_u32 (26) | 26 | 11 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 13 | W27 | t27 | cli/tri/src/fpga.rs | cclk_period_ns (9), cclk_nominal_hz (15) | 24 | 5 refused, 12 test, 1 inside group | none | same | tr 12/12 vac 0; gen-rust compiles | MOVABLE |
| 14 | WT | trios | crates/trios-trainer/src/validation.rs | is_within_champion_tolerance (24) | 24 | 2 test, 5 other | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 15 | W27 | t27 | cli/dlc10/src/lib.rs | Dlc10::build_read_cfg_packets (23) | 23 | 2 refused, 1 test | none | Type::fn -> free fn Dlc10_build_read_cfg_packets | tr no tests (binary crate spec); gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: Dlc10::read_cfg_reg_diag, Dlc10::read_cfg_reg_raw_n, build_read_cfg_packets_idcode_matches_openfpgaloader] |
| 16 | WT | trios | crates/trios-trainer/src/optimizer.rs | lr_schedule_54_f64 (21) | 21 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 17 | WT | trios | crates/trios-tri/src/lib.rs | ternary_size_bytes (21) | 21 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 18 | W27 | t27 | cli/tri/src/gates.rs | leaves_function (20) | 20 | 4 refused | none | &str -> &[u8] (callers .as_bytes()) | tr 4/4 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: assert_sites, boundary_sites, invert_sites, sites_in_direction] |
| 19 | R27 | t27 | rings/ring-092-rust/src/lib.rs | abs_f64 (9), identity_witness (11) | 20 | 14 test, 1 inside group | none | same | tr 21/21 vac 5; gen-rust compiles | MOVABLE |
| 20 | R27 | t27 | rings/ring-092-rust/src/lib.rs | cache_kv (20) | 20 | 1 test | none | &[f64] -> impl AsRef (widens) x2 | tr 21/21 vac 5; gen-rust compiles | MOVABLE |
| 21 | NT | trios | apps/trios-macos/rings/RUST-13/trios-mesh/src/wire.rs | be_byte (10), header_byte (10) | 20 | 1 refused, 2 inside group | none | same | tr 2/2 vac 1; gen-rust compiles | MOVABLE |
| 22 | R27 | t27 | rings/ring-095-rust/src/lib.rs | pow_u64 (15), compute_bias_correction (4) | 19 | 2 refused, 9 test, 1 inside group | none | same | tr 7/7 vac 0; gen-rust compiles | MOVABLE |
| 23 | NT | trios | apps/trios-macos/rings/RUST-05/clade-monitor/src/main.rs | calculate_backoff (9), calculate_backoff_with_jitter (10) | 19 | 5 refused, 15 test, 1 inside group | none | same | tr no tests (binary crate spec); gen-rust compiles | MOVABLE |
| 24 | WT | trios | crates/trios-chat/rings/CR-CHAT-02/src/ratchet_replay_window_guard.rs | validate_counter_advance (19) | 19 | 7 test | crates/trios-chat/rings/CR-CHAT-02/src/lib.rs (1, import) | ret Result<(),RatchetReplayError> -> bool | tr 7/7 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: rpla_01_zero_counter_rejected, rpla_02_not_advanced_rejected, rpla_03_rollback_rejected, rpla_04_gap_too_large_rejected +3] |
| 25 | W27 | t27 | bootstrap/src/formula_eval.rs | map_formula_id (18) | 18 | 1 refused | none | &str -> &'static str (narrows); ret &str -> &'static str (benign) | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: evaluate_formula] |
| 26 | W27 | t27 | cli/tri/src/prcheck.rs | verdict_code (18) | 18 | 1 refused, 1 test | none | same | tr 9/9 vac 1; gen-rust compiles | MOVABLE |
| 27 | R27 | t27 | rings/ring-093-rust/src/lib.rs | abs_f64 (8), identity_witness (10) | 18 | 20 test, 1 inside group | none | same | tr 14/14 vac 4; gen-rust compiles | MOVABLE |
| 28 | WT | trios | crates/trios-tri/src/core_compat.rs | ternary_compression_ratio (18) | 18 | 1 test | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 29 | WT | trios | crates/trios-tri/src/core_compat.rs | ternary_compression_vs_gf16 (18) | 18 | 1 test | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 30 | W27 | t27 | bootstrap/src/math_compare.rs | pell_f64 (17) | 17 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 31 | W27 | t27 | bootstrap/src/math_compare.rs | pell_u64 (17) | 17 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 32 | R27 | t27 | rings/ring-099-rust/src/lib.rs | advance (17) | 17 | 4 refused | none | const fn -> fn | tr 23/23 vac 5; gen-rust compiles | MOVABLE |
| 33 | WT | trios | crates/trios-fpga/src/xdc.rs | minimal_qmtech_xdc (17) | 17 | 3 test | crates/trios-fpga/src/build.rs:428 as crate::xdc::minimal_qmtech_xdc() | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 34 | WT | trios | crates/trios-chat/rings/CR-CHAT-07/src/lib.rs | uniform_gap_ms (16) | 16 | 9 test | crates/trios-chat/rings/CR-CHAT-07/src/jitter_side_channel.rs (2, import) | same | tr 2/2 vac 0; gen-rust compiles | MOVABLE |
| 35 | W27 | t27 | cli/tri/src/prcheck.rs | merge_outcome (15) | 15 | 2 refused, 3 test, 1 other | none | same | tr 9/9 vac 1; gen-rust compiles | MOVABLE |
| 36 | WT | trios | tools/defense_gate/src/lib.rs | count_substr (15) | 15 | 8 refused, 2 test | none | &str -> &[u8] (callers .as_bytes()) x2 | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: count_frames, count_qa_pairs, count_sections, count_substr_does_not_overlap] |
| 37 | WT | trios | crates/trios-chat/rings/CR-CHAT-07/src/egress_fingerprint.rs | uniform_burst_ms (14) | 14 | 1 refused, 6 test | crates/trios-chat/rings/CR-CHAT-07/src/lib.rs (1, import) | same | tr 2/2 vac 0; gen-rust compiles | MOVABLE |
| 38 | WT | trios | crates/trios-chat/rings/CR-CHAT-07/src/egress_fingerprint.rs | uniform_length_class (14) | 14 | 1 refused, 6 test | crates/trios-chat/rings/CR-CHAT-07/src/lib.rs (1, import) | same | tr 2/2 vac 0; gen-rust compiles | MOVABLE |
| 39 | W27 | t27 | bootstrap/src/main.rs | t27a_status_text (13) | 13 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 40 | W27 | t27 | cli/tri/src/fpga.rs | raw_ns_satisfies_flash_spec (13) | 13 | 4 refused | none | same | tr 12/12 vac 0; gen-rust compiles | MOVABLE |
| 41 | W27 | t27 | cli/tri/src/gates.rs | silences_stderr (9), counts_zero_when_absent (4) | 13 | 1 conv, 3 refused, 3 test, 2 other, 1 inside group | none | &str -> &[u8] (callers .as_bytes()) x2 | tr 4/4 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: a_count_that_reads_zero_is_the_same_defect_in_a_different_costume, counts_zero_when_absent, if_falls_through, is_quiet_candidate +2] |
| 42 | R27 | t27 | rings/ring-098-rust/src/lib.rs | pow_u64 (13) | 13 | 5 test | none | const fn -> fn | tr 11/11 vac 2; gen-rust compiles | MOVABLE |
| 43 | R27 | t27 | rings/ring-099-rust/src/lib.rs | pow_u64 (13) | 13 | 5 test | none | const fn -> fn | tr 23/23 vac 5; gen-rust compiles | MOVABLE |
| 44 | WT | trios | crates/trios-hdc/src/phi_quantization.rs | compression_ratio (13) | 13 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 45 | WT | trios | crates/trios-trainer/src/forward.rs | vec_scale (13) | 13 | 1 test | none | &[f32] -> impl AsRef (widens) | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 46 | WT | trios | tools/acm_ae_check/src/lib.rs | reusable_required_needles (13) | 13 | 1 refused | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 47 | W27 | t27 | bootstrap/src/compiler.rs | HirMemory::calc_addr_width (12) | 12 | 2 refused | none | Type::fn -> free fn HirMemory_calc_addr_width | tr 35/35 vac 4; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: HirMemory::new_bram, HirMemory::new_rom] |
| 48 | W27 | t27 | cli/tri/src/night.rs | action_for (12) | 12 | 2 refused, 4 test, 1 other | none | &str -> &[u8] (callers .as_bytes()) | tr 2/2 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: clean_merges_and_behind_updates, pr_line, run, unknown_states_refuse_to_guess] |
| 49 | W27 | t27 | cli/tri/src/red.rs | is_lower_bound (4), is_bounded_read (8) | 12 | 1 refused, 8 test, 1 inside group | none | same | tr 2/2 vac 0; gen-rust compiles | MOVABLE |
| 50 | WT | trios | crates/trios-igla-race/src/race.rs | deterministic_unit (12) | 12 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 51 | WT | trios | crates/trios-trainer/src/forward.rs | vec_add (12) | 12 | 1 test | none | &[f32] -> impl AsRef (widens) | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 52 | WT | trios | tools/acm_ae_check/src/lib.rs | functional_required_paths (12) | 12 | 1 refused | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 53 | WT | trios | crates/trios-ternary/src/lib.rs | ternary_size_bytes (11) | 11 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 54 | W27 | t27 | bootstrap/src/bitnet_axi.rs | resolve_param (10) | 10 | 2 refused, 5 test | none | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 55 | W27 | t27 | cli/tri/src/cibase.rs | count (10) | 10 | 3 refused, 5 test | none | &str -> &[u8] (callers .as_bytes()) | tr 1/1 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: an_unreadable_count_is_not_a_zero_and_does_not_claim_it_never_ran, baseline] |
| 56 | R27 | t27 | rings/ring-093-rust/src/lib.rs | relu_inplace (10) | 10 | 1 refused, 2 test | none | same | tr 14/14 vac 4; gen-rust compiles | MOVABLE |
| 57 | R27 | t27 | rings/ring-100-rust/src/lib.rs | Mesh::identity_witness (10) | 10 | 1 test | none | Type::fn -> free fn Mesh_identity_witness | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: identity_witness_holds] |
| 58 | WT | trios | crates/trios-chat/rings/CR-CHAT-04/src/padding_timing_uniformity_guard.rs | approx_sqrt (10) | 10 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 59 | WT | trios | crates/trios-chat/rings/CR-CHAT-07/src/cover_traffic_burst_interval_regularity_guard.rs | approx_sqrt (10) | 10 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 60 | WT | trios | crates/trios-cli/src/cmd/roster.rs | agent_name (10) | 10 | 1 refused | none | &str -> &[u8] (callers .as_bytes()) | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: roster_show] |
| 61 | WT | trios | crates/trios-phd/src/main.rs | regex_lite_global (10) | 10 | 1 refused | none | arity 2->3 | tr 3/3 vac 3; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: materialize_stubs] |
| 62 | WT | trios | crates/trios-trainer/src/train_loop.rs | evaluate_step (10) | 10 | 1 refused | none | ret Result<f32> -> Option<f32> | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: run] |
| 63 | W27 | t27 | bootstrap/src/compiler.rs | Codegen::zig_is_primitive (9) | 9 | 2 refused | none | &str -> &[u8] (callers .as_bytes()); Type::fn -> free fn Codegen_zig_is_primitive | tr 35/35 vac 4; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: Codegen::gen_fn_decl, Codegen::zig_binding_ident] |
| 64 | W27 | t27 | bootstrap/src/main.rs | backend_extension (9) | 9 | 3 refused | none | &str -> &[u8] (callers .as_bytes()); ret &str -> &'static str (benign) | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: run_compile, run_compile_all, run_compile_project] |
| 65 | W27 | t27 | bootstrap/src/phi_f64_literals.rs | expected_phi (9) | 9 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 66 | W27 | t27 | cli/tri/src/depin/phi_challenge.rs | pack_gf16_matrix (9) | 9 | 1 refused, 1 test | cli/tri/src/depin/prove.rs (2, import) | &[[u8;16];16] -> Vec<[u8;16]> | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: compute_phi_response_v2, test_v2_wrong_matrix_fails] |
| 67 | W27 | t27 | cli/tri/src/gates.rs | says_lower_bound (9) | 9 | 1 refused | none | &str -> &[u8] (callers .as_bytes()) | tr 4/4 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: bounded_reads_outside_the_crate] |
| 68 | W27 | t27 | cli/tri/src/issues.rs | read_is_complete (9) | 9 | 3 refused, 4 test, 1 other | cli/tri/src/gates.rs (1), cli/tri/src/prcheck.rs as issues::read_is_complete() | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 69 | O27 | t27 | contrib/solana/programs/tri-mining/src/lib.rs | pack_gf16_matrix (9) | 9 | 1 refused | none | &[[u8;16];16] -> Vec<[u8;16]> | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: verify_phi_response_v2] |
| 70 | R27 | t27 | rings/ring-095-rust/src/lib.rs | fabs_no_std (9) | 9 | 2 refused, 20 test | none | same | tr 7/7 vac 0; gen-rust compiles | MOVABLE |
| 71 | NT | trios | apps/trios-macos/rings/RUST-08/clade-promote/src/main.rs | severity_cost (9) | 9 | 1 refused, 11 test | none | &str -> &[u8] (callers .as_bytes()) | tr 1/1 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: severity_cost_critical, severity_cost_high, severity_cost_low, severity_cost_medium +3] |
| 72 | WT | trios | crates/trios-chat/rings/CR-CHAT-03/src/treekem_parent_hash_binding.rs | sibling_index (9) | 9 | 3 refused, 3 test | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 73 | WT | trios | crates/trios-igla-race/src/victory_new.rs | t_cdf_lower_tail (9) | 9 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 74 | NT | trios | crates/trios-mcp/rings/SR-01/src/audits_new.rs | category_to_label (9) | 9 | none | none | &str -> &'static str (narrows); ret &str -> &'static str (benign) | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: other files only] |
| 75 | WT | trios | crates/trios-tri/src/core_compat.rs | format_description (9) | 9 | 1 test | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 76 | W27 | t27 | bootstrap/src/compiler.rs | VerilogCodegen::is_primitive_scalar_type (8) | 8 | 18 refused | none | &str -> &[u8] (callers .as_bytes()); Type::fn -> free fn VerilogCodegen_is_primitive_scalar_type | tr 35/35 vac 4; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: Compiler::is_icarus_lowerable_type, VerilogCodegen::call_returning_cse_value_info, VerilogCodegen::expr_width_signed, VerilogCodegen::gen_verilog +7] |
| 77 | W27 | t27 | cli/tri/src/fpga.rs | quad_refused (8) | 8 | 1 refused, 5 test, 1 other | none | &str -> &[u8] (callers .as_bytes()) | tr 12/12 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: another_board_is_not_this_bench, both_quad_flags_are_refused_on_the_ssot_board, program_flash, without_a_quad_flag_nothing_is_refused] |
| 78 | R27 | t27 | rings/ring-091-rust/src/lib.rs | abs_f32 (8) | 8 | 6 test | none | same | tr 8/8 vac 1; gen-rust compiles | MOVABLE |
| 79 | R27 | t27 | rings/ring-096-rust/src/lib.rs | fabs_no_std (8) | 8 | 1 refused | none | same | tr 10/10 vac 5; gen-rust compiles | MOVABLE |
| 80 | NT | trios | apps/trios-macos/rings/RUST-13/trios-mesh/src/wire.rs | frame_kind_valid (4), parse_accepts (4) | 8 | 2 refused, 5 test, 1 inside group | none | same | tr 2/2 vac 1; gen-rust compiles | MOVABLE |
| 81 | WT | trios | crates/trios-server/src/rest_browseros.rs | is_safe_id (8) | 8 | 2 refused | none | &str -> &[u8] (callers .as_bytes()) | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: create_skill, resolve_skill] |
| 82 | WT | trios | crates/trios-train-cpu/src/attention.rs | relu_squared_backward (8) | 8 | 1 refused | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 83 | W27 | t27 | bootstrap/src/compiler.rs | CCodegen::is_primitive (7) | 7 | 4 refused | none | &str -> &[u8] (callers .as_bytes()); Type::fn -> free fn CCodegen_is_primitive | tr 35/35 vac 4; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: CCodegen::c_array_info, CCodegen::gen_c_const, CCodegen::is_type_name, Compiler::detect_unsupported_verilog_locals] |
| 84 | W27 | t27 | cli/tri/src/gates.rs | too_few_runs_to_judge (7) | 7 | 1 conv, 2 test | none | same | tr 4/4 vac 0; gen-rust compiles | MOVABLE |
| 85 | W27 | t27 | cli/tri/src/scratch.rs | yn (7) | 7 | 5 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 86 | W27 | t27 | cli/tri/src/unparsed.rs | is_if_capture (7) | 7 | 3 test, 1 other | none | &str -> &[u8] (callers .as_bytes()) | tr 5/5 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: a_capture_in_a_for_loop_is_not_the_if_form, bitwise_or_is_not_a_capture] |
| 87 | R27 | t27 | rings/ring-099-rust/src/lib.rs | pipeline_progress (7) | 7 | 7 test | none | same | tr 23/23 vac 5; gen-rust compiles | MOVABLE |
| 88 | WT | trios | crates/trios-golden-float/src/bin/gptq_calibration_ablation.rs | t_cdf_df2 (7) | 7 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 89 | W27 | t27 | bootstrap/src/compiler.rs | RustCodegen::is_int_type (6) | 6 | 3 refused | none | &str -> &[u8] (callers .as_bytes()); Type::fn -> free fn RustCodegen_is_int_type | tr 35/35 vac 4; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: RustCodegen::expr_to_rust_as, RustCodegen::infer_int_type] |
| 90 | W27 | t27 | bootstrap/src/service.rs | close (6) | 6 | 4 refused | none | same | tr 2/2 vac 0; gen-rust compiles | MOVABLE |
| 91 | W27 | t27 | bootstrap/src/service.rs | silicon_receipt_word (6) | 6 | 1 refused, 2 test | none | same | tr 2/2 vac 0; gen-rust compiles | MOVABLE |
| 92 | W27 | t27 | cli/tri/src/cibase.rs | is_not_found (6) | 6 | 1 refused, 4 test | none | &str -> &[u8] (callers .as_bytes()) | tr 1/1 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: a_404_is_an_answer_and_a_rate_limit_is_not, fetch] |
| 93 | R27 | t27 | rings/ring-098-rust/src/lib.rs | identity_witness (6) | 6 | 3 test | none | const fn -> fn | tr 11/11 vac 2; gen-rust compiles | MOVABLE |
| 94 | WT | trios | crates/trios-chat/rings/CR-CHAT-03/src/treekem_parent_hash_binding.rs | _parent_index (6) | 6 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 95 | WT | trios | crates/trios-phi-schedule/src/lib.rs | flat_lr (6) | 6 | 1 refused, 3 test | none | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 96 | NT | trios | crates/trios-ui/rings/UR-04/src/lib.rs | chrono_now_iso (6) | 6 | 1 refused | none | arity 0->1 | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: send_message] |
| 97 | O27 | t27 | bindings/javascript/src/lib.rs | gf16_is_inf (5) | 5 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 98 | O27 | t27 | bindings/javascript/src/lib.rs | gf16_is_nan (5) | 5 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 99 | O27 | t27 | bindings/javascript/src/lib.rs | gf32_is_nan (5) | 5 | 1 conv | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 100 | W27 | t27 | bootstrap/src/suite.rs | header_ok (5) | 5 | 1 refused | none | &str -> &[u8] (callers .as_bytes()) | tr 1/1 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: validate_gen_headers] |
| 101 | W27 | t27 | bootstrap/src/use_resolve.rs | indent_of (5) | 5 | 2 refused | none | &str -> &[u8] (callers .as_bytes()) | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: split_decls, top_level_indent] |
| 102 | W27 | t27 | cli/tri/src/unparsed.rs | is_fn_literal (5) | 5 | 2 test, 1 other | none | &str -> &[u8] (callers .as_bytes()) | tr 5/5 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: the_three_new_rows_have_the_boundary_right] |
| 103 | O27 | t27 | contrib/backend/agent-runner/src/api.rs | backoff_secs (5) | 5 | 2 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 104 | R27 | t27 | rings/ring-099-rust/src/lib.rs | identity_witness (5) | 5 | 3 test | none | const fn -> fn | tr 23/23 vac 5; gen-rust compiles | MOVABLE |
| 105 | R27 | t27 | rings/ring-101-rust/src/lib.rs | identity_witness (5) | 5 | 1 test | none | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 106 | R27 | t27 | rings/ring-102-rust/src/lib.rs | identity_witness (5) | 5 | 1 test | none | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 107 | R27 | t27 | rings/ring-103-rust/src/lib.rs | identity_witness (5) | 5 | 1 test | none | same | tr 2/2 vac 1; gen-rust compiles | MOVABLE |
| 108 | R27 | t27 | rings/ring-104-rust/src/lib.rs | identity_witness (5) | 5 | 1 test | none | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 109 | WT | trios | crates/trios-train-cpu/src/attention.rs | relu_squared (5) | 5 | 2 refused | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 110 | NT | trios | validate_bpb.rs | simulate_loss_step_0 (5) | 5 | 1 refused | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 111 | O27 | t27 | bindings/javascript/src/lib.rs | gf32_is_inf (4) | 4 | 1 conv | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 112 | W27 | t27 | bootstrap/src/compiler.rs | return_unsized (4) | 4 | 1 refused | none | arity 1->2 | tr 35/35 vac 4; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: RustCodegen::t27_type_to_rust] |
| 113 | W27 | t27 | bootstrap/src/entry_points.rs | is_void (4) | 4 | 3 refused | none | &str -> &[u8] (callers .as_bytes()) | tr no tests (binary crate spec); gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: classify_at, forced_signature, signature_of_forced_any] |
| 114 | W27 | t27 | bootstrap/src/math_compare.rs | phi_f64 (4) | 4 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 115 | W27 | t27 | cli/tri/src/fpga.rs | n25q128_pvt_temp_derating_ns (4) | 4 | 1 conv | none | same | tr 12/12 vac 0; gen-rust compiles | MOVABLE |
| 116 | W27 | t27 | cli/tri/src/fpga.rs | n25q128_pvt_voltage_derating_ns (4) | 4 | 1 conv | none | same | tr 12/12 vac 0; gen-rust compiles | MOVABLE |
| 117 | W27 | t27 | cli/tri/src/trees.rs | gib (4) | 4 | 4 refused, 2 test | none | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 118 | W27 | t27 | cli/tri/src/unparsed.rs | is_for_range (4) | 4 | 2 test, 1 other | none | &str -> &[u8] (callers .as_bytes()) | tr 5/5 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: a_capture_in_a_for_loop_is_not_the_if_form] |
| 119 | W27 | t27 | cli/tri/src/unparsed.rs | is_stray_close (4) | 4 | 3 test, 1 other | none | &str -> &[u8] (callers .as_bytes()) | tr 5/5 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: the_three_new_rows_have_the_boundary_right] |
| 120 | R27 | t27 | rings/ring-094-rust/src/lib.rs | identity_witness (4) | 4 | 1 test | none | same | tr 6/6 vac 1; gen-rust compiles | MOVABLE |
| 121 | R27 | t27 | rings/ring-095-rust/src/lib.rs | identity_witness (4) | 4 | 1 test | none | same | tr 7/7 vac 0; gen-rust compiles | MOVABLE |
| 122 | R27 | t27 | rings/ring-095-rust/src/lib.rs | update_first_moment (4) | 4 | 1 refused, 2 test | none | same | tr 7/7 vac 0; gen-rust compiles | MOVABLE |
| 123 | R27 | t27 | rings/ring-095-rust/src/lib.rs | update_second_moment (4) | 4 | 1 refused, 2 test | none | same | tr 7/7 vac 0; gen-rust compiles | MOVABLE |
| 124 | R27 | t27 | rings/ring-096-rust/src/lib.rs | is_nan (4) | 4 | 1 refused | none | same | tr 10/10 vac 5; gen-rust compiles | MOVABLE |
| 125 | R27 | t27 | rings/ring-099-rust/src/lib.rs | stage_name (4) | 4 | 3 test | none | const fn -> fn | tr 23/23 vac 5; gen-rust compiles | MOVABLE |
| 126 | NT | trios | apps/trios-macos/rings/RUST-13/trios-mesh/src/wire.rs | u32_be (4) | 4 | 2 refused | none | same | tr 2/2 vac 1; gen-rust compiles | MOVABLE |
| 127 | NT | trios | apps/trios-macos/rings/RUST-14/clade-tablecloth/src/main.rs | backoff_base_ms (4) | 4 | 1 refused, 3 test | none | same | tr 5/5 vac 3; gen-rust compiles | MOVABLE |
| 128 | WT | trios | crates/trios-chat/rings/CR-CHAT-04/src/external_init_secret_pinning.rs | exporter_len (4) | 4 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 129 | WT | trios | crates/trios-chat/rings/CR-CHAT-04/src/external_init_secret_pinning.rs | kem_ephemeral_len (4) | 4 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 130 | WT | trios | crates/trios-chat/rings/CR-CHAT-06/src/tag_stripping.rs | contains_any_tag (4) | 4 | 2 refused | none | &str -> &[u8] (callers .as_bytes()) | tr 1/1 vac 1; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: parse_structured_output, serialise_structured_output] |
| 131 | NT | trios | crates/trios-golden-float/rings/BR-OUTPUT/src/lib.rs | GoldenFloat::anchor (4) | 4 | 1 test | none | const fn -> fn; Type::fn -> free fn GoldenFloat_anchor | tr 1/1 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: anchor_const] |
| 132 | WT | trios | crates/trios-server/src/tools/mod.rs | count (4) | 4 | none | crates/trios-server/src/main.rs (2) as tools::count() | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 133 | WT | trios | tools/acm_ae_check/src/lib.rs | available_required_anchors (4) | 4 | 1 refused | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 134 | NT | trios | validate_bpb.rs | simulate_data_leak (4) | 4 | 1 refused | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 135 | NT | trios | validate_bpb.rs | simulate_weight_updates (4) | 4 | 1 refused | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 136 | OT | 999-multibots-telegraf | rust/rings/SILVER-RING-AI00/src/providers/kie.rs | KieProvider::is_sora_model (3) | 3 | 1 refused | none | &str -> &[u8] (callers .as_bytes()); Type::fn -> free fn KieProvider_is_sora_model | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: KieProvider::generate] |
| 137 | OT | 999-multibots-telegraf | rust/rings/SILVER-RING-AI00/src/providers/kie.rs | KieProvider::is_wan_model (3) | 3 | none | none | &str -> &[u8] (callers .as_bytes()); Type::fn -> free fn KieProvider_is_wan_model | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: other files only] |
| 138 | O27 | t27 | bindings/javascript/src/lib.rs | gf16_is_zero (3) | 3 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 139 | O27 | t27 | bindings/javascript/src/lib.rs | gf32_is_zero (3) | 3 | 1 conv | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 140 | W27 | t27 | bootstrap/src/compiler.rs | VerilogCodegen::type_is_float (3) | 3 | 6 refused | none | &str -> &[u8] (callers .as_bytes()); Type::fn -> free fn VerilogCodegen_type_is_float | tr 35/35 vac 4; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: VerilogCodegen::gen_verilog_const, VerilogCodegen::gen_verilog_fn, VerilogCodegen::verilog_expr_is_real] |
| 141 | W27 | t27 | bootstrap/src/compiler.rs | is_ident_byte (3) | 3 | 2 conv | none | same | tr 35/35 vac 4; gen-rust compiles | MOVABLE |
| 142 | W27 | t27 | bootstrap/src/parse_conform.rs | total (3) | 3 | none | bootstrap/src/main.rs (1), bootstrap/src/suite.rs (2) as parse_conform::total() | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 143 | W27 | t27 | bootstrap/src/test_report.rs | starts_token (3) | 3 | 2 refused | none | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 144 | W27 | t27 | cli/tri/src/census.rs | ident (3) | 3 | 2 refused | none | same | tr 1/1 vac 0; gen-rust compiles | MOVABLE |
| 145 | W27 | t27 | cli/tri/src/elab.rs | is_summary (3) | 3 | 2 refused, 2 test | none | &str -> &[u8] (callers .as_bytes()) | tr 1/1 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: classify, secondary, summary_line_is_recognised_as_a_total] |
| 146 | W27 | t27 | cli/tri/src/gates.rs | is_control_fn (3) | 3 | 4 refused | none | &str -> &[u8] (callers .as_bytes()) | tr 4/4 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: assert_sites, boundary_sites, invert_sites, sites_in_direction] |
| 147 | W27 | t27 | cli/tri/src/gates.rs | is_ident_byte (3) | 3 | 2 conv | none | same | tr 4/4 vac 0; gen-rust compiles | MOVABLE |
| 148 | W27 | t27 | cli/tri/src/leanreach.rs | is_word (3) | 3 | 2 conv | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 149 | W27 | t27 | cli/tri/src/unparsed.rs | is_fixture (3) | 3 | 2 refused, 3 test | none | &str -> &[u8] (callers .as_bytes()) | tr 5/5 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: a_fixture_is_not_debt, agree, parse_failures] |
| 150 | WT | trios | crates/trios-agents/src/types.rs | default_concurrency (3) | 3 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 151 | WT | trios | crates/trios-agents/src/types.rs | default_memory (3) | 3 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 152 | WT | trios | crates/trios-agents/src/types.rs | default_timeout (3) | 3 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 153 | WT | trios | crates/trios-chat/rings/CR-CHAT-04/src/padding_alignment_boundary_guard.rs | is_class (3) | 3 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 154 | WT | trios | crates/trios-chat/rings/CR-CHAT-04/src/padding_alignment_boundary_guard.rs | largest_class (3) | 3 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 155 | WT | trios | crates/trios-chat/rings/CR-CHAT-04/src/padding_alignment_boundary_guard.rs | smallest_class (3) | 3 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 156 | WT | trios | crates/trios-chat/rings/CR-CHAT-04/src/padding_block_size_alignment_guard.rs | is_power_of_two (3) | 3 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 157 | WT | trios | crates/trios-chat/rings/CR-CHAT-04/src/padding_class_transition_audit.rs | is_canonical (3) | 3 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 158 | WT | trios | crates/trios-chat/rings/CR-CHAT-06/src/agent_tool_input_sanitization_guard.rs | is_control (3) | 3 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 159 | NT | trios | crates/trios-crypto/rings/BR-OUTPUT/src/lib.rs | Crypto::anchor (3) | 3 | none | none | const fn -> fn; Type::fn -> free fn Crypto_anchor | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: other files only] |
| 160 | WT | trios | crates/trios-ext/rings/SILVER-RING-EXT-01/src/lib.rs | get_style (3) | 3 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 161 | WT | trios | crates/trios-golden-float/src/lib.rs | compression_ratio (3) | 3 | 1 test | none | same | tr 3/3 vac 1; gen-rust compiles | MOVABLE |
| 162 | WT | trios | crates/trios-golden-float/src/lib.rs | gf16_size_bytes (3) | 3 | 1 test | none | same | tr 3/3 vac 1; gen-rust compiles | MOVABLE |
| 163 | NT | trios | crates/trios-hdc/rings/BR-OUTPUT/src/lib.rs | Hdc::anchor (3) | 3 | none | none | const fn -> fn; Type::fn -> free fn Hdc_anchor | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: other files only] |
| 164 | NT | trios | crates/trios-kg/src/types.rs | default_limit (3) | 3 | none | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 165 | WT | trios | crates/trios-openclaw/rings/OC-01/src/lib.rs | is_supported (3) | 3 | 3 test | crates/trios-openclaw/src/lib.rs (1, import) | &str -> &[u8] (callers .as_bytes()) | tr 1/1 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: supported_set] |
| 166 | NT | trios | crates/trios-physics/rings/BR-OUTPUT/src/lib.rs | Physics::anchor (3) | 3 | 1 test | none | const fn -> fn; Type::fn -> free fn Physics_anchor | tr no tests in spec; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: rings_link] |
| 167 | NT | trios | crates/trios-queen-loop/src/main.rs | default_max_level (3) | 3 | 1 refused | none | same | tr no tests in spec; gen-rust compiles | MOVABLE |
| 168 | NT | trios | crates/trios-sacred/rings/BR-OUTPUT/src/lib.rs | Sacred::anchor (3) | 3 | 1 test | none | const fn -> fn; Type::fn -> free fn Sacred_anchor | tr 1/1 vac 0; gen-rust compiles | MOVABLE-WITH-CALLER-CHANGE [change callers in: anchor_const] |
| 169 | WT | trios | crates/trios-trinity-brain/src/phi.rs | phi_add (3) | 3 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 170 | WT | trios | crates/trios-trinity-brain/src/phi.rs | phi_mul (3) | 3 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 171 | WT | trios | crates/trios-trinity-brain/src/phi.rs | phi_subtract (3) | 3 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 172 | NT | trios | crates/trios-vsa/rings/BR-OUTPUT/src/lib.rs | Vsa::anchor (3) | 3 | none | none | const fn -> fn; Type::fn -> free fn Vsa_anchor | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: other files only] |
| 173 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/fal.rs | default_one (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 174 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/fal.rs | default_scale (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 175 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_batch_size (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 176 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_format (1) | 1 | none | none | arity 0->1 | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: other files only] |
| 177 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_guidance (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 178 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_learning_rate (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 179 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_lora_rank (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 180 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_optimizer (1) | 1 | none | none | arity 0->1 | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: other files only] |
| 181 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_outputs (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 182 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_quality (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 183 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_resolution (1) | 1 | none | none | arity 0->1 | valid-output: unchecked (gen-rust compiles) | MOVABLE-WITH-CALLER-CHANGE [change callers in: other files only] |
| 184 | OT | 999-multibots-telegraf | rust/rings/GOLD-RING-PR00/src/replicate.rs | default_steps (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 185 | WT | trios | crates/trios-trainer/src/config.rs | default_checkpoint_interval (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 186 | WT | trios | crates/trios-trainer/src/config.rs | default_eval_interval (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |
| 187 | WT | trios | crates/trios-trainer/src/config.rs | default_lr (1) | 1 | none | none | same | valid-output: unchecked (gen-rust compiles) | MOVABLE |

## Appendix A: what gen-rust changes in a signature (free fns, counted over all rows)

| difference | fns |
|---|---|
| &str -> &[u8] (callers .as_bytes()) | 57 |
| &str -> &'static str (narrows) | 9 |
| ret -> &'static str (benign) | 6 |
| &[u8] -> &'staticstr | 6 |
| &[f64] -> impl AsRef (widens) | 6 |
| arity 0->1 | 6 |
| ret Result<(),anyhow::Error> -> bool | 4 |
| ret Result<(),FfiNotAvailable> -> bool | 4 |
| &PvtContext -> PvtContext | 3 |
| Option<&str> -> Option<&'staticstr> | 3 |
| &ProofTrace -> ProofTrace | 3 |
| &[f32] -> impl AsRef (widens) | 3 |
| ret -> Option<&'staticstr> | 2 |
| &[[u8;16];16] -> Vec<[u8;16]> | 2 |

gen-rust output that does not compile, by rustc message (fns / lines): error[E0308]: mismatched types: expected X, found X 39/427; error[E0425]: cannot find function X in this scope: not found in this scope 10/57; error[E0277]: X is not an iterator: X is not an iterator; try calling X or X 8/85; error[E0277]: the type X cannot be indexed by X: string indices are ranges of X 4/65; error[E0425]: cannot find function, tuple struct or tuple variant X in this scop 3/24; error[E0425]: cannot find value X in this scope: not found in this scope 2/15

## Appendix B: every converted fn, by file

Entry: `name:lines:code`. M = MOVABLE, C = MOVABLE-WITH-CALLER-CHANGE, N-x = NOT-NOW with first reason x (ty+, ty-, bad, constctx, attr, trait, foreign, dep as in the reasons table). Methods are written `Type::name`.
Header per file: area, test-report of its spec, cfg(test) lines of the file.

- NT trios:.trinity/bin/trinity-bootstrap/src/main.rs [tr no tests in spec; cfg(test) 0]: Codename::label:31:N-ty- Codename::domain:31:N-ty- Codename::allowed_crate_globs:32:N-bad
- NT trios:apps/trios-macos/rings/RUST-02/clade-e2e/src/main.rs [tr 1/1 vac 0; cfg(test) 43]: resolve_variant:17:N-ty+
- NT trios:apps/trios-macos/rings/RUST-03/clade-rollback/src/main.rs [tr 4/4 vac 0; cfg(test) 107]: is_valid_snapshot:3:N-bad
- NT trios:apps/trios-macos/rings/RUST-04/clade-improve/src/sandbox.rs [tr 1/1 vac 0; cfg(test) 168]: verdict_tag:9:N-ty-
- NT trios:apps/trios-macos/rings/RUST-04/clade-improve/src/variant.rs [tr no tests in spec; cfg(test) 0]: Variant::mcp_port:7:N-ty- Variant::is_production:3:N-ty- Variant::can_self_modify:3:N-ty-
- NT trios:apps/trios-macos/rings/RUST-05/clade-monitor/src/main.rs [tr no tests (binary crate spec); cfg(test) 325]: calculate_backoff:9:M calculate_backoff_with_jitter:10:M default_budget:9:N-ty-
- NT trios:apps/trios-macos/rings/RUST-06/clade-dashboard/src/main.rs [tr 1/1 vac 1; cfg(test) 132]: default_budget:9:N-ty-
- NT trios:apps/trios-macos/rings/RUST-08/clade-promote/src/main.rs [tr 1/1 vac 0; cfg(test) 91]: severity_cost:9:C default_budget:3:N-ty-
- NT trios:apps/trios-macos/rings/RUST-13/trios-mesh/src/crypto.rs [tr no tests in spec; cfg(test) 375]: ReplayWindow::new:7:N-ty+
- NT trios:apps/trios-macos/rings/RUST-13/trios-mesh/src/daemon.rs [tr 7/7 vac 0; cfg(test) 148]: ConvergenceMetrics::new:6:N-ty+ ConvergenceMetrics::record_link_loss:4:N-bad ConvergenceMetrics::record_node_off:4:N-bad ConvergenceMetrics::check_ci_gates:22:N-ty+
- NT trios:apps/trios-macos/rings/RUST-13/trios-mesh/src/gf16.rs [tr no tests in spec; cfg(test) 48]: Gf16::from_bits:5:N-ty- Gf16::bits:5:N-ty- Gf16::is_nan:5:N-ty- CGf16::new:5:N-ty-
- NT trios:apps/trios-macos/rings/RUST-13/trios-mesh/src/routing.rs [tr no tests in spec; cfg(test) 261]: DeliveryRatio::kill:4:N-ty+ DeliveryRatio::ratio:3:N-ty+ LinkStats::kill:4:N-ty+
- NT trios:apps/trios-macos/rings/RUST-13/trios-mesh/src/wire.rs [tr 2/2 vac 1; cfg(test) 34]: frame_kind_valid:4:M be_byte:10:M u32_be:4:M header_byte:10:M parse_accepts:4:M
- NT trios:apps/trios-macos/rings/RUST-14/clade-tablecloth/src/main.rs [tr 5/5 vac 3; cfg(test) 379]: backoff_base_ms:4:M default_budget:9:N-ty-
- NT trios:crates/jepa_t_ingest/src/lib.rs [tr 4/4 vac 0; cfg(test) 62]: quantize_phi_prior:36:M IngestConfig::default:6:N-trait Triplet::to_bytes:14:N-ty- token_to_q15:12:N-bad
- NT trios:crates/phd-dashboard/src/main.rs [tr no tests in spec; cfg(test) 0]: coq_stats:28:N-bad
- NT trios:crates/trios-algorithm-arena/rings/SR-ALG-03/src/lib.rs [tr 4/4 vac 2; cfg(test) 219]: E2eTttEnv::default:7:N-trait
- NT trios:crates/trios-crypto/rings/BR-OUTPUT/src/lib.rs [tr no tests in spec; cfg(test) 12]: Crypto::anchor:3:C
- NT trios:crates/trios-doctor/rings/SILVER-RING-DR-04/src/lib.rs [tr 1/1 vac 0; cfg(test) 226]: RuleId::slug:13:N-ty- RuleId::all:13:N-ty- RuleEngine::new:4:N-ty-
- NT trios:crates/trios-golden-float/rings/BR-OUTPUT/src/lib.rs [tr 1/1 vac 0; cfg(test) 17]: GoldenFloat::anchor:4:C
- NT trios:crates/trios-hdc/rings/BR-OUTPUT/src/lib.rs [tr no tests in spec; cfg(test) 12]: Hdc::anchor:3:C
- NT trios:crates/trios-igla-race-pipeline/rings/BR-OUTPUT/src/lib.rs [tr no tests (binary crate spec); cfg(test) 82]: betacf:45:N-bad MockedTrainer::winning:12:N-ty+ MockedTrainer::losing:9:N-ty+
- NT trios:crates/trios-igla-race-pipeline/rings/SR-00/src/lib.rs [tr no tests in spec; cfg(test) 214]: Seed::value:4:N-ty-
- NT trios:crates/trios-igla-race-pipeline/rings/SR-03/src/lib.rs [tr no tests in spec; cfg(test) 217]: EmaPhiBand::alpha:4:N-ty+ EmaPhiBand::state:4:N-ty+
- NT trios:crates/trios-kg/src/types.rs [tr no tests in spec; cfg(test) 0]: default_limit:3:M
- NT trios:crates/trios-mcp/rings/SR-01/src/audits_new.rs [tr no tests in spec; cfg(test) 18]: category_to_label:9:C
- NT trios:crates/trios-physics/rings/BR-OUTPUT/src/lib.rs [tr no tests in spec; cfg(test) 11]: Physics::anchor:3:C
- NT trios:crates/trios-queen-loop/src/main.rs [tr no tests in spec; cfg(test) 0]: default_max_level:3:M
- NT trios:crates/trios-sacred/rings/BR-OUTPUT/src/lib.rs [tr 1/1 vac 0; cfg(test) 15]: Sacred::anchor:3:C
- NT trios:crates/trios-scarab-types/rings/SR-00/src/lib.rs [tr 1/1 vac 1; cfg(test) 37]: Term::slug:21:N-ty- Term::all:21:N-ty- TermScarab::new:4:N-ty-
- NT trios:crates/trios-scarab-types/rings/SR-01/src/lib.rs [tr 1/1 vac 1; cfg(test) 42]: LaneScarabType::slug:8:N-ty- LaneScarabType::all:8:N-ty-
- NT trios:crates/trios-scarab-types/rings/SR-02/src/lib.rs [tr 1/1 vac 1; cfg(test) 42]: SoulScarabType::slug:8:N-ty- SoulScarabType::all:8:N-ty-
- NT trios:crates/trios-scarab-types/rings/SR-03/src/lib.rs [valid-output: unchecked; cfg(test) 42]: GateScarabFourType::slug:8:N-ty- GateScarabFourType::all:8:N-ty-
- NT trios:crates/trios-scarab-types/rings/SR-04/src/lib.rs [valid-output: unchecked; cfg(test) 42]: GateScarabFiveType::slug:8:N-ty- GateScarabFiveType::all:8:N-ty-
- NT trios:crates/trios-ternary/rings/TR-00/src/lib.rs [valid-output: unchecked; cfg(test) 17]: Trit::to_i8:7:N-ty- Trit::from_i8:8:N-ty- Tryte::zero:3:N-ty-
- NT trios:crates/trios-ui/rings/UR-02/src/lib.rs [valid-output: unchecked; cfg(test) 0]: ButtonVariant::default:3:N-trait BadgeVariant::default:3:N-trait
- NT trios:crates/trios-ui/rings/UR-04/src/lib.rs [valid-output: unchecked; cfg(test) 0]: chrono_now_iso:6:C
- NT trios:crates/trios-ui/rings/UR-08/src/lib.rs [valid-output: unchecked; cfg(test) 0]: Route::label:10:N-ty-
- NT trios:crates/trios-vsa/rings/BR-OUTPUT/src/lib.rs [valid-output: unchecked; cfg(test) 12]: Vsa::anchor:3:C
- NT trios:validate_bpb.rs [valid-output: unchecked; cfg(test) 0]: simulate_data_leak:4:M simulate_loss_step_0:5:M simulate_weight_updates:4:M
- O27 backend/trinity-core/src/models.rs [tr no tests in spec; cfg(test) 0]: SessionState::default:3:N-trait
- O27 bindings/javascript/src/lib.rs [tr no tests in spec; cfg(test) 0]: gf16_is_zero:3:M gf16_is_inf:5:M gf16_is_nan:5:M gf32_is_zero:3:M gf32_is_inf:4:M gf32_is_nan:5:M GF32::bits:3:N-ty- GF32::is_zero:3:N-ty- GF32::is_inf:3:N-ty- GF32::is_nan:3:N-ty- is_positive_zero:5:N-attr is_negative_zero:5:N-attr
- O27 bindings/python/src/lib.rs [tr no tests in spec; cfg(test) 0]: PyGF16::bits:1:N-ty- PyGF16::__int__:1:N-ty- PyGF16::__hash__:1:N-ty- PyGF32::bits:1:N-ty- PyGF32::__int__:1:N-ty- PyGF32::__hash__:1:N-ty-
- O27 contrib/backend/agent-runner/src/api.rs [tr no tests in spec; cfg(test) 0]: backoff_secs:5:M
- O27 contrib/solana/programs/tri-mining/src/lib.rs [tr no tests in spec; cfg(test) 0]: pack_gf16_matrix:9:C
- O27 ffi/src/lib.rs [tr no tests in spec; cfg(test) 141]: encode_gf16_from_u32:52:M decode_gf16_to_u32:26:M
- OT 999-multibots-telegraf:rust/rings/GOLD-RING-PR00/src/fal.rs [valid-output: unchecked; cfg(test) 0]: default_one:1:M default_scale:1:M
- OT 999-multibots-telegraf:rust/rings/GOLD-RING-PR00/src/replicate.rs [valid-output: unchecked; cfg(test) 0]: default_lora_rank:1:M default_optimizer:1:C default_batch_size:1:M default_resolution:1:C default_learning_rate:1:M default_steps:1:M default_format:1:C default_guidance:1:M default_quality:1:M default_outputs:1:M
- OT 999-multibots-telegraf:rust/rings/GOLD-RING-TY00/src/bot.rs [valid-output: unchecked; cfg(test) 0]: BotName::display_name:19:N-ty-
- OT 999-multibots-telegraf:rust/rings/GOLD-RING-TY00/src/money.rs [valid-output: unchecked; cfg(test) 45]: Money::to_f64:5:N-ty- Money::minor_units:4:N-ty-
- OT 999-multibots-telegraf:rust/rings/GOLD-RING-TY00/src/payment.rs [valid-output: unchecked; cfg(test) 0]: PaymentMethod::currency:10:N-ty-
- OT 999-multibots-telegraf:rust/rings/GOLD-RING-TY00/src/scene.rs [valid-output: unchecked; cfg(test) 76]: SceneId::scene_name:64:N-ty-
- OT 999-multibots-telegraf:rust/rings/GOLD-RING-TY00/src/user.rs [valid-output: unchecked; cfg(test) 69]: Language::code:6:N-ty- Language::is_russian:3:N-ty-
- OT 999-multibots-telegraf:rust/rings/SILVER-RING-AI00/src/providers/kie.rs [valid-output: unchecked; cfg(test) 0]: KieProvider::is_sora_model:3:C KieProvider::is_wan_model:3:C
- OT 999-multibots-telegraf:rust/rings/SILVER-RING-JB00/src/worker.rs [valid-output: unchecked; cfg(test) 0]: JobType::as_str:13:N-ty- JobType::all_types:13:N-bad JobType::concurrency_limit:9:N-ty- JobType::timeout_secs:8:N-ty-
- OT 999-multibots-telegraf:rust/rings/SILVER-RING-PY00/src/telegram_stars.rs [valid-output: unchecked; cfg(test) 0]: TelegramStarsGateway::default:3:N-trait TelegramStarsGateway::new:1:N-ty+
- OT trinity:archive/vibee-browser/src-tauri/src/main.rs [valid-output: unchecked; cfg(test) 0]: get_version:4:N-attr
- OT trinity-fpga:archive/vibee-browser/src-tauri/src/main.rs [valid-output: unchecked; cfg(test) 0]: get_version:4:N-attr
- OT trinity-fpga:rings/FP-00/src/lib.rs [valid-output: unchecked; cfg(test) 63]: JtagConfig::default:6:N-trait TritValue::to_bits:7:N-ty- TritValue::from_bits:8:N-ty- VsaOp::code:7:N-ty-
- OT trinity-fpga:rings/FP-01/src/lib.rs [valid-output: unchecked; cfg(test) 128]: TernaryWeightConverter::default:3:N-trait TernaryWeightConverter::new:3:N-ty+
- OT trinity-fpga:rings/FP-02/src/lib.rs [valid-output: unchecked; cfg(test) 48]: XvcFlasher::extract_bitstream_data:15:N-bad
- R27 rings/ring-088-rust/src/lib.rs [tr 1/1 vac 1; cfg(test) 127]: Gf16::from_bits:4:N-ty- Gf16::to_bits:4:N-ty- Gf16::is_zero:5:N-ty- Gf16::is_inf:6:N-ty- Gf16::is_nan:6:N-ty- round_half_to_even:17:M libm_floor:10:M
- R27 rings/ring-089-rust/src/lib.rs [tr 3/3 vac 0; cfg(test) 187]: Trit::new:8:N-ty- Trit::from_i8_unchecked:4:N-ty- Trit::value:4:N-ty- Word27::zero:4:N-ty- Word27::from_trits:13:N-ty- Word27::to_i64:10:N-ty- Word27::trit_at:8:N-ty- Word27::set_trit:9:N-ty- Word27::negate:10:N-ty- Instr::nop:4:N-ty- Instr::halt:4:N-ty- Cpu::new:11:N-ty+ Cpu::load_program:16:N-ty- Cpu::read_reg:8:N-ty- Cpu::write_reg:12:N-ty- Cpu::default:3:N-trait
- R27 rings/ring-090-rust/src/lib.rs [tr 17/17 vac 0; cfg(test) 156]: SimState::tag:10:N-ty- SimState::from_tag:11:N-ty- sim_config:15:N-ty- sim_config_with_trace:16:N-ty- sim_ok:10:N-ty- sim_error:10:N-ty- probe:9:N-ty- trace_entry:8:N-ty- is_idle:4:N-ty- is_done:4:N-ty- is_error:4:N-ty- has_errors:4:N-ty- passed:4:N-ty- sim_time_ns:18:N-ty- sim_time_us:4:N-ty- sim_time_ms:4:N-ty- cycles_for_time_ns:16:N-ty- validate_sim_config:19:N-ty-
- R27 rings/ring-091-rust/src/lib.rs [tr 8/8 vac 1; cfg(test) 198]: SplitMix64::new:5:N-ty+ SplitMix64::next_u64:9:N-ty+ SplitMix64::next_f32_unit:9:N-ty+ floor_f32:20:N-bad frac_f32:5:N-bad is_finite_f32:6:N-bad abs_f32:8:M
- R27 rings/ring-092-rust/src/lib.rs [tr 21/21 vac 5; cfg(test) 355]: abs_f64:9:M exp_f64:45:M apply_softmax:48:M cache_kv:20:M compute_scores:38:M weighted_values:30:M identity_witness:11:M
- R27 rings/ring-093-rust/src/lib.rs [tr 14/14 vac 4; cfg(test) 498]: MoEConfig::trinity_defaults:11:N-ty- MoEConfig::is_valid:9:N-ty- abs_f64:8:M exp_f64:33:M relu_inplace:10:M load_balance_loss:27:M identity_witness:10:M
- R27 rings/ring-094-rust/src/lib.rs [tr 6/6 vac 1; cfg(test) 385]: Trit::to_i8:8:N-ty- Trit::from_i8:10:N-ty- TaskState::is_terminal:4:N-ty- priority_to_credit:17:N-ty- identity_witness:4:M
- R27 rings/ring-095-rust/src/lib.rs [tr 7/7 vac 0; cfg(test) 387]: AdamWConfig::defaults:12:N-ty- AdamWConfig::phi_preset:12:N-ty- AdamWConfig::effective_beta1:8:N-ty- AdamWConfig::effective_beta2:8:N-ty- AdamWConfig::default:3:N-trait fabs_no_std:9:M pow_u64:15:M compute_bias_correction:4:M update_first_moment:4:M update_second_moment:4:M identity_witness:4:M
- R27 rings/ring-096-rust/src/lib.rs [tr 10/10 vac 5; cfg(test) 353]: format_bytes:10:N-ty- fabs_no_std:8:M is_nan:4:M
- R27 rings/ring-097-rust/src/lib.rs [tr 4/4 vac 1; cfg(test) 337]: Trit::to_char:10:N-ty- k3_not:9:N-ty- ProofStep::new:31:N-ty- ProofStep::input_count:3:N-ty+ ProofStep::input:7:N-ty- new_proof_trace:19:N-ty- trace_length:4:N-ty+ is_at_capacity:4:N-ty+ step_at:8:N-bad trit_to_string:4:N-ty- write_byte:8:M write_str:12:N-bad write_bytes:3:N-bad write_usize:19:M write_u64:18:M
- R27 rings/ring-098-rust/src/lib.rs [tr 11/11 vac 2; cfg(test) 324]: Phase::next:11:N-ty- BrainState::init:22:N-ty- BrainState::phi_coherence:4:N-constctx Transition::empty:10:N-ty- WorldModel::new:12:N-ty- WorldModel::current_state:4:N-ty- WorldModel::current_phase:4:N-ty- WorldModel::state_count:4:N-ty+ WorldModel::transition_count:4:N-ty+ WorldModel::is_state_buffer_full:4:N-ty+ WorldModel::is_transition_buffer_full:4:N-ty+ WorldModel::record_transition:12:N-ty- WorldModel::state_at:8:N-bad WorldModel::transition_at:8:N-bad WorldModel::reset:4:N-ty+ WorldModel::default:3:N-trait pow_u64:13:M is_finite_f64:6:N-bad identity_witness:6:M
- R27 rings/ring-099-rust/src/lib.rs [tr 23/23 vac 5; cfg(test) 328]: Stage::from_code:18:N-ty- Stage::code:15:N-ty- Stage::next:17:N-ty- Stage::is_terminal:4:N-ty- Stage::name:15:N-ty- Pipeline::new:9:N-ty- Pipeline::current:4:N-constctx Pipeline::count:4:N-constctx Pipeline::stage_at:8:N-bad Pipeline::result_at:8:N-bad Pipeline::reset:4:N-ty+ Pipeline::verify:25:N-ty- Pipeline::reset_buffers:6:N-ty- Pipeline::default:3:N-trait pipeline_progress:7:M stage_name:4:M advance:17:M pow_u64:13:M identity_witness:5:M
- R27 rings/ring-100-rust/src/lib.rs [tr no tests in spec; cfg(test) 47]: TileRole::mnemonic:8:N-ty- Mesh::identity_witness:10:C
- R27 rings/ring-101-rust/src/lib.rs [tr 1/1 vac 0; cfg(test) 51]: identity_witness:5:M
- R27 rings/ring-102-rust/src/lib.rs [tr 1/1 vac 0; cfg(test) 53]: Lane::ideal:4:N-ty- identity_witness:5:M
- R27 rings/ring-103-rust/src/lib.rs [tr 2/2 vac 1; cfg(test) 55]: PhiSgd::default_smoke:4:N-ty+ identity_witness:5:M
- R27 rings/ring-104-rust/src/lib.rs [tr 1/1 vac 0; cfg(test) 66]: identity_witness:5:M
- W27 bootstrap/src/bitnet_axi.rs [tr 1/1 vac 0; cfg(test) 345]: resolve_param:10:M
- W27 bootstrap/src/check_calls.rs [tr no tests in spec; cfg(test) 0]: is_scalar_type:9:N-bad
- W27 bootstrap/src/codegen_js.rs [tr no tests in spec; cfg(test) 532]: Target::as_const:9:N-bad Target::is_py:3:N-bad Target::comment:9:N-bad
- W27 bootstrap/src/compiler.rs [tr 35/35 vac 4; cfg(test) 9704]: Parser::is_binary_op:28:N-ty- Parser::token_continues_expr:29:N-ty- Parser::opens_declaration:13:N-ty- Parser::is_block_boundary:15:N-ty- Codegen::escape_for_fmt:15:N-bad Codegen::zig_is_primitive:9:C LoopGuard::needs_block:14:N-ty+ VerilogCodegen::type_is_float:3:C VerilogCodegen::verilog_keywords:43:N-bad VerilogCodegen::is_string_field_type:28:N-bad VerilogCodegen::is_primitive_scalar_type:8:C CCodegen::is_primitive:7:C OptConfig::default:8:N-trait return_unsized:4:C mentions_type:15:N-bad is_ident_byte:3:M RustCodegen::is_int_type:6:C HirMemory::calc_addr_width:12:C HirResourceEstimate::zero:9:N-ty+ HirResourceEstimate::new:9:N-ty+ AsmSectionKind::from_i8:9:N-ty- AsmSectionKind::to_i8:8:N-ty- AsmRelocKind::from_i8:8:N-ty- AsmRelocKind::to_i8:7:N-ty- TbClockCfg::new:7:N-ty+ TbClockCfg::half_period:3:N-ty+ TbResetCfg::new:7:N-ty+ VcdVarKind::vcd_str:8:N-ty- LinkSegment::text:9:N-ty+ LinkSegment::data:9:N-ty+ LinkResult::ok:11:N-ty+ LinkResult::fail:11:N-ty+ LinkResult::passed:3:N-ty+ TimingReport::ok:11:N-ty+ TimingReport::fail:11:N-ty+ TimingReport::passed:3:N-ty+ BitstreamMeta::compute_checksum:8:N-bad InterFpgaLink::lvds:9:N-ty+ InterFpgaLink::serdes:9:N-ty+ InterFpgaLink::bandwidth_mbps:3:N-ty+ PartitionResult::ok:10:N-ty+ PartitionResult::fail:10:N-ty+ PartitionResult::passed:3:N-ty+ TestCoverage::new:8:N-ty+ TestCoverage::is_acceptable:3:N-ty+ CtsReport::passed:3:N-ty+ CrossOptReport::new:9:N-ty+ CrossOptReport::total_optimizations:3:N-ty+ CrossOptReport::is_effective:3:N-ty+ HirOptimizer::new:9:N-ty+ HirSimResult::ok:9:N-ty- HirSimResult::is_done:3:N-ty- HirSimResult::is_error:3:N-ty- HirSimResult::has_errors:3:N-ty+ HirSimResult::passed:3:N-ty- PipeResult::ok:9:N-ty+ PipeResult::ipc:6:N-ty+ PipeResult::cpi:6:N-ty+ PipeResult::gf16_throughput:6:N-ty+ PipeResult::passed:3:N-ty+ BuildMetrics::new:12:N-ty+
- W27 bootstrap/src/entry_points.rs [tr no tests (binary crate spec); cfg(test) 131]: is_void:4:C
- W27 bootstrap/src/formula_eval.rs [tr no tests in spec; cfg(test) 0]: map_formula_id:18:C
- W27 bootstrap/src/host/addrmap.rs [tr 5/5 vac 0; cfg(test) 113]: MemRegion::new:3:N-constctx MemRegion::end:3:N-ty- MemRegion::contains:3:N-ty- MemRegion::offset_of:7:N-ty- MemRegion::overlaps:3:N-ty- MemRegion::align_up:3:N-ty-
- W27 bootstrap/src/host/capflags.rs [tr 1/1 vac 0; cfg(test) 99]: CapFlags::none:3:N-constctx CapFlags::all:3:N-constctx CapFlags::from_bits:3:N-ty- CapFlags::bits:3:N-ty- CapFlags::has:3:N-ty- CapFlags::is_empty:3:N-ty- CapFlags::missing:3:N-ty- CapFlags::satisfies:3:N-ty- check_capabilities:8:N-ty-
- W27 bootstrap/src/host/checksum.rs [tr 2/2 vac 0; cfg(test) 85]: ChecksumKind::name:8:N-ty- ChecksumKind::digest_size:8:N-ty- compute_sum8:7:N-bad compute_xor8:7:N-bad compute_fletcher16:9:N-bad
- W27 bootstrap/src/host/cmdq.rs [tr 1/1 vac 0; cfg(test) 160]: Priority::default:3:N-trait QueueStats::utilization:7:N-ty-
- W27 bootstrap/src/host/config.rs [tr no tests in spec; cfg(test) 102]: HostConfig::default:13:N-trait HostConfig::builder:3:N-ty- HostConfig::total_weights:3:N-ty- HostConfigBuilder::new:5:N-ty+ HostConfigBuilder::default:3:N-trait
- W27 bootstrap/src/host/descring.rs [tr no tests in spec; cfg(test) 123]: Descriptor::new:8:N-ty-
- W27 bootstrap/src/host/dma.rs [tr no tests in spec; cfg(test) 198]: make_table:19:N-constctx DmaConfig::new:8:N-ty+
- W27 bootstrap/src/host/driver.rs [tr no tests in spec; cfg(test) 90]: CsrSnapshot::weight_addr_64:4:N-ty-
- W27 bootstrap/src/host/errors.rs [tr 1/1 vac 1; cfg(test) 99]: ErrorCode::raw:3:N-ty- ErrorCode::domain:14:N-ty- ErrorCode::code:3:N-constctx CatalogEntry::new:3:N-constctx
- W27 bootstrap/src/host/firmware.rs [tr no tests in spec; cfg(test) 127]: SectionHeader::end:3:N-ty-
- W27 bootstrap/src/host/histogram.rs [tr 9/9 vac 1; cfg(test) 124]: Histogram::new:9:N-ty- Histogram::record:12:N-bad Histogram::find_bin:8:N-bad Histogram::bin:3:N-ty- Histogram::count:3:N-ty- Histogram::sum_us:3:N-ty- Histogram::min_us:3:N-ty- Histogram::max_us:3:N-ty- Histogram::mean_us:7:N-ty- Histogram::merge:15:N-ty- Histogram::reset:7:N-ty- Histogram::is_empty:3:N-ty- Histogram::default:3:N-trait
- W27 bootstrap/src/host/irq.rs [tr 1/1 vac 0; cfg(test) 149]: IrqSource::all:8:N-ty-
- W27 bootstrap/src/host/mempool.rs [tr 1/1 vac 0; cfg(test) 149]: MemBlock::new:6:N-ty+ MemBlock::as_slice:3:N-bad MemBlock::as_mut_slice:3:N-ty+ MemBlock::capacity:3:N-ty+ MemBlock::remaining:3:N-ty+ MemBlock::clear:3:N-ty+ PoolStats::utilization:7:N-ty-
- W27 bootstrap/src/host/pipeline.rs [tr 1/1 vac 0; cfg(test) 135]: PipelineConfig::default:11:N-trait
- W27 bootstrap/src/host/protocol.rs [tr 4/4 vac 0; cfg(test) 152]: Cmd::from_u8:13:N-ty- Cmd::has_payload:3:N-ty- Cmd::expects_response:3:N-ty- RespCode::from_u8:11:N-ty- RespCode::is_ok:3:N-ty- CmdPacket::new:8:N-ty- RespPacket::new:7:N-ty-
- W27 bootstrap/src/host/ratelimit.rs [tr 3/3 vac 0; cfg(test) 111]: RateLimitConfig::default:6:N-trait RateLimitConfig::new:6:N-ty+ RateLimitConfig::burst:6:N-ty+ RateLimiter::new:9:N-ty+ RateLimiter::refill:8:N-ty+ RateLimiter::tokens:3:N-ty+ RateLimiter::total_allowed:3:N-ty+ RateLimiter::total_rejected:3:N-ty+ RateLimiter::reset:6:N-ty+ RateLimiter::stats:8:N-ty-
- W27 bootstrap/src/host/regmap.rs [tr 3/3 vac 0; cfg(test) 145]: CtrlReg::new:3:N-ty- CtrlReg::from_raw:3:N-ty- CtrlReg::raw:3:N-ty- StatusReg::from_raw:3:N-ty- StatusReg::raw:3:N-ty- IrqEnReg::new:3:N-ty- IrqEnReg::from_raw:3:N-ty- IrqEnReg::raw:3:N-ty- IrqStatReg::from_raw:3:N-ty- IrqStatReg::raw:3:N-ty- WeightAddr::new:6:N-ty- WeightAddr::addr:3:N-ty-
- W27 bootstrap/src/host/scatter_gather.rs [tr 4/4 vac 0; cfg(test) 140]: SgSegment::new:3:N-ty- SgSegment::src_end:3:N-ty- SgSegment::dst_end:3:N-ty- SgSegment::overlaps_src:3:N-ty- SgSegment::overlaps_dst:3:N-ty-
- W27 bootstrap/src/host/session.rs [tr no tests in spec; cfg(test) 150]: SessionConfig::default:6:N-trait
- W27 bootstrap/src/host/version.rs [tr 2/2 vac 0; cfg(test) 114]: SemVer::new:3:N-constctx SemVer::as_u32:3:N-ty- SemVer::from_u32:7:N-ty- SemVer::is_compatible:3:N-ty- VersionInfo::default:7:N-trait VersionInfo::new:3:N-bad BuildInfo::new:3:N-constctx
- W27 bootstrap/src/host/watchdog.rs [tr 1/1 vac 0; cfg(test) 160]: WatchdogConfig::default:7:N-trait WatchdogTimer::new:11:N-ty- WatchdogTimer::state:3:N-ty- WatchdogTimer::missed_heartbeats:3:N-ty+ WatchdogTimer::elapsed_ms:3:N-ty+ WatchdogTimer::reset:6:N-ty- WatchdogTimer::stats:10:N-ty-
- W27 bootstrap/src/host/weight_header.rs [tr 4/4 vac 0; cfg(test) 120]: WeightHeader::new:10:N-ty- WeightHeader::total_weights:3:N-ty- WeightHeader::total_weight_bytes:4:N-ty- WeightHeader::has_crc:3:N-ty- WeightHeader::is_ternary:3:N-ty-
- W27 bootstrap/src/host/weight_loader.rs [tr no tests in spec; cfg(test) 167]: LoadConfig::default:8:N-trait LoadConfig::raw:3:N-ty+ make_table:19:N-constctx
- W27 bootstrap/src/main.rs [tr no tests in spec; cfg(test) 4]: backend_extension:9:C is_vacuous_stmt:5:N-bad t27a_status_text:13:M
- W27 bootstrap/src/math_compare.rs [tr no tests in spec; cfg(test) 116]: phi_f64:4:M pell_u64:17:M pell_f64:17:M
- W27 bootstrap/src/parse_conform.rs [tr no tests in spec; cfg(test) 19]: total:3:M
- W27 bootstrap/src/phi_f64_literals.rs [tr no tests in spec; cfg(test) 0]: expected_phi:9:M
- W27 bootstrap/src/service.rs [tr 2/2 vac 0; cfg(test) 720]: Unresolved::label:8:N-ty- Unresolved::token:9:N-ty- Unresolved::machine_wide:11:N-ty- silicon_receipt_word:6:M close:6:M
- W27 bootstrap/src/source_kind.rs [tr no tests in spec; cfg(test) 79]: SourceKind::slug:12:N-ty- SourceKind::label:11:N-ty- SourceKind::is_source:5:N-ty-
- W27 bootstrap/src/suite.rs [tr 1/1 vac 0; cfg(test) 1003]: is_scratch:9:N-bad header_ok:5:C
- W27 bootstrap/src/test_report.rs [tr 1/1 vac 0; cfg(test) 294]: starts_token:3:M
- W27 bootstrap/src/tt_debug.rs [tr 3/3 vac 1; cfg(test) 236]: TtDebugErrorClass::slug:9:N-ty- TtDebugErrorClass::all:9:N-ty- TtDebugVersion::pack_u32:6:N-ty-
- W27 bootstrap/src/tt_manifest.rs [tr 2/2 vac 0; cfg(test) 141]: TtChip::slug:8:N-ty- AxiWidths::canonical:8:N-ty-
- W27 bootstrap/src/tt_profile.rs [tr 1/1 vac 0; cfg(test) 224]: TtPlatform::slug:8:N-ty-
- W27 bootstrap/src/use_resolve.rs [tr no tests in spec; cfg(test) 455]: brace_module:5:N-bad indent_of:5:C
- W27 cli/dlc10/src/lib.rs [tr no tests (binary crate spec); cfg(test) 310]: Dlc10::build_read_cfg_packets:23:C StatBits::from_raw:24:N-ty+
- W27 cli/t27b/src/ir.rs [tr no tests in spec; cfg(test) 0]: FOp::symbol:8:N-ty- FOp::apply:9:N-ty- FOp::apply_f32:9:N-ty- TrapKind::describe:17:N-ty-
- W27 cli/tri/src/census.rs [tr 1/1 vac 0; cfg(test) 216]: ident:3:M
- W27 cli/tri/src/cibase.rs [tr 1/1 vac 0; cfg(test) 173]: is_not_found:6:C count:10:C
- W27 cli/tri/src/depin/phi_challenge.rs [tr no tests in spec; cfg(test) 220]: pack_gf16_matrix:9:C
- W27 cli/tri/src/depin/types.rs [tr no tests in spec; cfg(test) 0]: MiningEpoch::genesis:7:N-ty+ AppState::new:5:N-ty+
- W27 cli/tri/src/elab.rs [tr 1/1 vac 0; cfg(test) 45]: is_summary:3:C
- W27 cli/tri/src/fpga.rs [tr 12/12 vac 0; cfg(test) 3918]: synthetic_pvt_context:12:N-ty- pvt_context_inside_envelope:9:N-ty- cclk_period_ns:9:M raw_ns_satisfies_flash_spec:13:M n25q128_pvt_temp_derating_ns:4:M n25q128_pvt_voltage_derating_ns:4:M n25q128_pvt_process_derating_ns:8:N-ty- n25q128_min_sck_half_ns_pvt:6:N-ty- pvt_worst_case_context:10:N-ty- cclk_nominal_hz:15:M pvt_envelope_margin_ns:12:N-dep raw_ns_satisfies_flash_spec_pvt:17:N-ty- CsvTimeUnit::to_seconds_multiplier:10:N-ty- CsvVoltageUnit::to_volts_multiplier:7:N-ty- quad_refused:8:C
- W27 cli/tri/src/gates.rs [tr 4/4 vac 0; cfg(test) 3016]: leaves_function:20:C is_control_fn:3:C Docstring::new:3:N-ty+ label:9:N-ty- is_test_attr:16:N-bad token_occurrences:23:N-bad is_ident_byte:3:M classify_warning:12:N-bad too_few_runs_to_judge:7:M Dispatch::label:7:N-ty- classify:14:N-ty- Reading::tag:8:N-ty- Reading::is_pass:5:N-ty- says_lower_bound:9:C silences_stderr:9:C counts_zero_when_absent:4:C existence_gated:11:N-bad is_quiet_candidate:23:N-bad
- W27 cli/tri/src/issues.rs [tr 1/1 vac 0; cfg(test) 646]: read_is_complete:9:M
- W27 cli/tri/src/leanreach.rs [tr no tests in spec; cfg(test) 52]: line_has_sorry:15:N-bad is_word:3:M
- W27 cli/tri/src/misread.rs [tr 1/1 vac 0; cfg(test) 73]: Shape::label:9:N-ty- Shape::backend:7:N-ty- Shape::all:9:N-ty-
- W27 cli/tri/src/night.rs [tr 2/2 vac 0; cfg(test) 16]: action_for:12:C
- W27 cli/tri/src/oneaway.rs [tr 1/1 vac 0; cfg(test) 45]: is_parse_error:15:N-bad
- W27 cli/tri/src/prcheck.rs [tr 9/9 vac 1; cfg(test) 845]: branch_matches:10:N-bad merge_outcome:15:M verdict_code:18:M
- W27 cli/tri/src/red.rs [tr 2/2 vac 0; cfg(test) 442]: is_lower_bound:4:M is_bounded_read:8:M
- W27 cli/tri/src/renum.rs [tr no tests in spec; cfg(test) 445]: appended_tail:11:N-bad
- W27 cli/tri/src/scratch.rs [tr no tests in spec; cfg(test) 88]: yn:7:M
- W27 cli/tri/src/trees.rs [tr 1/1 vac 0; cfg(test) 148]: gib:4:M
- W27 cli/tri/src/unparsed.rs [tr 5/5 vac 0; cfg(test) 364]: head:4:N-bad is_if_capture:7:C is_for_range:4:C is_prototype:3:N-bad is_tuple_struct:3:N-bad is_pub_module:3:N-bad is_path_module:3:N-bad is_import:3:N-bad is_use:5:N-bad is_trait:3:N-bad is_impl:3:N-bad is_algorithm:3:N-bad is_type_alias:3:N-bad is_zig_block:3:N-bad is_pub_use:4:N-bad is_fn_literal:5:C is_double_semicolon:4:N-bad is_stray_close:4:C is_fixture:3:C stage_of:15:N-ty-
- W27 cli/tri/src/vsim.rs [tr 2/2 vac 0; cfg(test) 82]: Stage::label:12:N-ty- attribute_failure:19:N-ty-
- W27 cli/tri/src/window.rs [tr 1/1 vac 0; cfg(test) 35]: short:10:N-bad
- WT trios:contrib/oracle/src/controller.rs [tr no tests in spec; cfg(test) 76]: OracleController::new:7:N-ty+
- WT trios:crates/trios-agent-harness/rings/AH-00/src/lib.rs [tr 1/1 vac 0; cfg(test) 42]: AgentAdapter::as_str:8:N-ty- AgentAdapter::parse:9:N-ty- PermissionMode::default:3:N-trait
- WT trios:crates/trios-agent-harness/rings/AH-03/src/lib.rs [tr no tests in spec; cfg(test) 57]: TurnStatus::is_terminal:3:N-ty-
- WT trios:crates/trios-agents/src/types.rs [tr no tests in spec; cfg(test) 0]: default_memory:3:M default_concurrency:3:M default_timeout:3:M
- WT trios:crates/trios-chat/rings/CR-CHAT-00/src/lib.rs [tr 2/2 vac 1; cfg(test) 71]: SessionId::new:4:N-ty- Counter::get:4:N-ty- DestHash::new:4:N-ty- chat_laws:30:N-bad
- WT trios:crates/trios-chat/rings/CR-CHAT-01/src/epoch_advancement_guard.rs [tr 7/7 vac 0; cfg(test) 89]: validate_epoch_transition:23:N-ty+
- WT trios:crates/trios-chat/rings/CR-CHAT-01/src/key_package_expiry_guard.rs [tr 9/9 vac 0; cfg(test) 98]: validate_key_package_expiry:27:C
- WT trios:crates/trios-chat/rings/CR-CHAT-01/src/prekey_bundle_freshness_guard.rs [tr 6/6 vac 0; cfg(test) 109]: validate_bundle_freshness:19:N-ty+
- WT trios:crates/trios-chat/rings/CR-CHAT-02/src/clock_skew.rs [tr 1/1 vac 1; cfg(test) 111]: ClockSkewBound::default:3:N-trait ClockSkewBound::new:5:N-ty- ClockSkewBound::accepts:6:N-ty-
- WT trios:crates/trios-chat/rings/CR-CHAT-02/src/ratchet_replay_window_guard.rs [tr 7/7 vac 0; cfg(test) 82]: validate_counter_advance:19:C
- WT trios:crates/trios-chat/rings/CR-CHAT-02/src/skipped_message_key_exhaustion.rs [tr 1/1 vac 0; cfg(test) 89]: validate_total_skipped:15:N-bad
- WT trios:crates/trios-chat/rings/CR-CHAT-03/src/treekem_parent_hash_binding.rs [tr no tests in spec; cfg(test) 174]: _parent_index:6:M sibling_index:9:M
- WT trios:crates/trios-chat/rings/CR-CHAT-04/src/external_init_secret_pinning.rs [tr no tests in spec; cfg(test) 151]: exporter_len:4:M kem_ephemeral_len:4:M
- WT trios:crates/trios-chat/rings/CR-CHAT-04/src/mls_plaintext_framing_integrity.rs [tr no tests in spec; cfg(test) 133]: MlsContentType::from_byte:8:N-ty-
- WT trios:crates/trios-chat/rings/CR-CHAT-04/src/padding_alignment_boundary_guard.rs [tr no tests in spec; cfg(test) 96]: smallest_class:3:M largest_class:3:M is_class:3:M
- WT trios:crates/trios-chat/rings/CR-CHAT-04/src/padding_block_size_alignment_guard.rs [tr no tests in spec; cfg(test) 129]: is_power_of_two:3:M
- WT trios:crates/trios-chat/rings/CR-CHAT-04/src/padding_class_transition_audit.rs [tr no tests in spec; cfg(test) 97]: is_canonical:3:M
- WT trios:crates/trios-chat/rings/CR-CHAT-04/src/padding_oracle_chosen_ct.rs [tr no tests in spec; cfg(test) 152]: VerdictLedger::new:7:N-ty+ VerdictLedger::consecutive_rejects:4:N-ty+
- WT trios:crates/trios-chat/rings/CR-CHAT-04/src/padding_timing_uniformity_guard.rs [tr no tests in spec; cfg(test) 134]: approx_sqrt:10:M
- WT trios:crates/trios-chat/rings/CR-CHAT-05/src/session_store_key_expiry_guard.rs [tr no tests in spec; cfg(test) 104]: SessionKeyEntry::is_expired:4:N-ty+
- WT trios:crates/trios-chat/rings/CR-CHAT-05/src/store_tombstone_purging_consistency_guard.rs [tr no tests in spec; cfg(test) 103]: TombstoneEntry::is_past_retention:4:N-ty+
- WT trios:crates/trios-chat/rings/CR-CHAT-05/src/welcome_keypackage_pinning.rs [tr 1/1 vac 1; cfg(test) 155]: KeyPackagePin::pin:4:N-ty+ KeyPackagePin::pinned:5:N-ty+
- WT trios:crates/trios-chat/rings/CR-CHAT-06/src/agent_output_content_type_validation_guard.rs [tr no tests in spec; cfg(test) 112]: ContentType::name:8:N-ty-
- WT trios:crates/trios-chat/rings/CR-CHAT-06/src/agent_tool_input_sanitization_guard.rs [tr no tests in spec; cfg(test) 105]: is_control:3:M
- WT trios:crates/trios-chat/rings/CR-CHAT-06/src/tag_stripping.rs [tr 1/1 vac 1; cfg(test) 150]: contains_any_tag:4:C
- WT trios:crates/trios-chat/rings/CR-CHAT-07/src/cover_traffic_burst_interval_regularity_guard.rs [tr no tests in spec; cfg(test) 137]: approx_sqrt:10:M
- WT trios:crates/trios-chat/rings/CR-CHAT-07/src/cover_traffic_emission_jitter_guard.rs [tr no tests in spec; cfg(test) 108]: CoverEmission::jitter_ms:8:N-ty+
- WT trios:crates/trios-chat/rings/CR-CHAT-07/src/cover_traffic_emission_pattern_randomness_guard.rs [tr no tests in spec; cfg(test) 132]: count_runs:16:N-bad
- WT trios:crates/trios-chat/rings/CR-CHAT-07/src/cover_traffic_entropy_fingerprint.rs [tr no tests in spec; cfg(test) 106]: max_zero_run:15:N-bad
- WT trios:crates/trios-chat/rings/CR-CHAT-07/src/egress_fingerprint.rs [tr 2/2 vac 0; cfg(test) 94]: EgressFingerprint::canonical_tls:4:N-ty- uniform_length_class:14:M uniform_burst_ms:14:M
- WT trios:crates/trios-chat/rings/CR-CHAT-07/src/jitter_side_channel.rs [tr 1/1 vac 1; cfg(test) 153]: JitterPolicy::default:6:N-trait
- WT trios:crates/trios-chat/rings/CR-CHAT-07/src/lib.rs [tr 2/2 vac 0; cfg(test) 59]: CoverScheduler::default:3:N-trait CoverScheduler::new:7:N-ty+ CoverScheduler::enqueue_real:4:N-ty+ CoverScheduler::ticks:4:N-ty+ CoverScheduler::queue_depth:5:N-ty+ CoverScheduler::tick:13:N-ty- uniform_gap_ms:16:M
- WT trios:crates/trios-cli/src/cmd/roster.rs [tr no tests in spec; cfg(test) 0]: agent_name:10:C
- WT trios:crates/trios-cli/src/gh.rs [tr no tests in spec; cfg(test) 0]: GhClient::new:4:N-ty-
- WT trios:crates/trios-core/src/types.rs [tr 6/6 vac 0; cfg(test) 82]: PrecisionFormat::bit_width:8:N-ty- PrecisionFormat::is_floating_point:4:N-ty- PrecisionFormat::compression_ratio:4:N-ty- PrecisionFormat::memory_bytes:4:N-ty- LayerType::base_sensitivity:12:N-ty- LayerType::default_precision:12:N-ty- HardwareCost::gf16:9:N-ty- HardwareCost::ternary:9:N-ty- HardwareCost::for_precision:13:N-ty- MemoryBudget::mb_16:7:N-ty- MemoryBudget::new:7:N-ty- MemoryBudget::fits:4:N-ty- MemoryBudget::remaining:4:N-ty- MemoryBudget::default:3:N-trait DspBudget::xc7a100t:8:N-ty- DspBudget::new:8:N-ty- DspBudget::available_dsp:4:N-ty- DspBudget::utilization:4:N-ty- DspBudget::fits:4:N-ty- DspBudget::default:3:N-trait LutBudget::xc7a100t:7:N-ty- LutBudget::new:7:N-ty- LutBudget::available_lut:4:N-ty- LutBudget::utilization:4:N-ty- LutBudget::fits:4:N-ty- LutBudget::default:3:N-trait FpgaTarget::xc7a100t:11:N-ty- FpgaTarget::default:3:N-trait
- WT trios:crates/trios-data/src/lib.rs [tr 1/1 vac 0; cfg(test) 75]: FineWebBatch::new:13:N-ty+ FineWebBatch::offset:4:N-ty+ FineWebBatch::batch_size:4:N-ty+ FineWebBatch::reset:4:N-ty+
- WT trios:crates/trios-ext/rings/SILVER-RING-EXT-01/src/lib.rs [tr no tests in spec; cfg(test) 0]: get_style:3:M
- WT trios:crates/trios-fpga/src/board.rs [tr 1/1 vac 1; cfg(test) 208]: KnownBoard::fpga_part:7:N-ty- KnownBoard::openfpgaloader_board:7:N-ty- KnownBoard::clock_mhz:7:N-ty- KnownBoard::default_xdc:6:N-ty- KnownBoard::chipdb_name:7:N-ty-
- WT trios:crates/trios-fpga/src/xdc.rs [tr no tests in spec; cfg(test) 200]: minimal_qmtech_xdc:17:M
- WT trios:crates/trios-golden-float/src/bin/gptq_calibration_ablation.rs [tr no tests in spec; cfg(test) 0]: Lcg::new:3:N-ty+ Lcg::next_u64:6:N-ty+ t_cdf_df2:7:M
- WT trios:crates/trios-golden-float/src/lib.rs [tr 3/3 vac 1; cfg(test) 37]: gf16_size_bytes:3:M compression_ratio:3:M
- WT trios:crates/trios-golden-float/src/router.rs [tr 8/8 vac 0; cfg(test) 84]: Precision::bit_width:8:N-ty- Precision::is_floating_point:4:N-ty- PrecisionRouter::new:4:N-ty+ PrecisionRouter::get_precision:19:N-ty- PrecisionRouter::needs_conversion:4:N-ty- PrecisionRouter::mac_lut_cost:8:N-ty- PrecisionRouter::mac_dsp_cost:8:N-ty- PrecisionRouter::default:3:N-trait
- WT trios:crates/trios-hdc/src/lib.rs [tr no tests in spec; cfg(test) 17]: HdcSpace::dimensions:3:N-ty- HdcSpace::dimensions:3:N-ty-
- WT trios:crates/trios-hdc/src/phi_quantization.rs [tr no tests in spec; cfg(test) 35]: compression_ratio:13:M
- WT trios:crates/trios-igla-race/src/asha.rs [tr no tests in spec; cfg(test) 0]: ArchKind::min_rung:9:N-ty- ArchKind::as_str:9:N-ty- AshaRung::next:9:N-ty-
- WT trios:crates/trios-igla-race/src/attn.rs [tr 1/1 vac 1; cfg(test) 151]: QkHead::d_model:4:N-ty- QkHead::num_heads:4:N-ty- QkHead::head_dim:4:N-ty- QkHead::scale:4:N-ty-
- WT trios:crates/trios-igla-race/src/ema.rs [tr 1/1 vac 1; cfg(test) 163]: EmaTracker::with_alpha:15:N-ty+ EmaTracker::alpha:5:N-ty+ EmaTracker::n_updates:5:N-ty+ EmaTracker::update:16:N-ty+ EmaTracker::raw:5:N-ty+ EmaTracker::reset:5:N-ty+
- WT trios:crates/trios-igla-race/src/hive_automaton.rs [tr 2/2 vac 0; cfg(test) 324]: State::is_absorbing:5:N-ty-
- WT trios:crates/trios-igla-race/src/neon.rs [tr 1/1 vac 0; cfg(test) 58]: NeonDb::client:3:N-ty+
- WT trios:crates/trios-igla-race/src/proxies/mod.rs [tr 2/2 vac 0; cfg(test) 80]: SynFlowScore::new:3:N-ty- SynFlowScore::is_valid:3:N-ty- GradNormScore::new:3:N-ty- GradNormScore::from_norm:5:N-ty- GradNormScore::is_valid:3:N-ty- EnsembleScore::default:8:N-trait EnsembleScore::new:3:N-ty- EnsembleScore::with_weights:8:N-ty- EnsembleScore::score:3:N-ty- EnsembleScore::is_valid:3:N-ty- ProxyMetrics::is_valid:3:N-ty-
- WT trios:crates/trios-igla-race/src/race.rs [tr no tests in spec; cfg(test) 270]: TrialStatus::as_str:9:N-ty- deterministic_unit:12:M
- WT trios:crates/trios-igla-race/src/real_forward.rs [tr no tests in spec; cfg(test) 80]: TrainerFormat::as_env_value:11:N-ty-
- WT trios:crates/trios-igla-race/src/rungs.rs [tr 2/2 vac 0; cfg(test) 128]: Rung::exp:5:N-constctx Rung::step:13:N-ty- Rung::next:8:N-ty- Rung::first:4:N-ty- Rung::last:4:N-ty-
- WT trios:crates/trios-igla-race/src/victory_new.rs [tr no tests in spec; cfg(test) 339]: t_cdf_lower_tail:9:M
- WT trios:crates/trios-openclaw/rings/OC-01/src/lib.rs [tr 1/1 vac 0; cfg(test) 40]: is_supported:3:C
- WT trios:crates/trios-phd/src/main.rs [tr 3/3 vac 3; cfg(test) 237]: regex_lite_global:10:C
- WT trios:crates/trios-phi-schedule/src/lib.rs [tr 1/1 vac 0; cfg(test) 159]: flat_lr:6:M
- WT trios:crates/trios-physics/src/lib.rs [tr no tests in spec; cfg(test) 18]: quantum_step:20:N-attr quantum_step:9:N-attr qcd_coupling:4:N-attr qcd_coupling:4:N-attr fibonacci_lattice_spacing:4:N-attr fibonacci_lattice_spacing:4:N-attr
- WT trios:crates/trios-precision-router/src/lib.rs [tr 3/3 vac 0; cfg(test) 45]: LayerType::cost_multiplier:7:N-ty- LayerType::dsp_per_param:7:N-ty- LayerType::lut_per_param:7:N-ty- MemoryBudget::mb_16:7:N-ty- MemoryBudget::fits:4:N-ty- DspBudget::xc7a100t:7:N-ty- DspBudget::available_dsp:4:N-ty- PrecisionFormat::bits_per_param:7:N-ty- PrecisionFormat::compression_ratio:4:N-ty- PrecisionRouter::new:6:N-ty+
- WT trios:crates/trios-sacred/src/lib.rs [tr 1/1 vac 1; cfg(test) 28]: phi_bottleneck:4:N-attr phi_bottleneck:4:N-attr
- WT trios:crates/trios-server/src/mcp.rs [valid-output: unchecked; cfg(test) 0]: McpService::new:3:N-ty+
- WT trios:crates/trios-server/src/rest_browseros.rs [valid-output: unchecked; cfg(test) 286]: is_safe_id:8:C
- WT trios:crates/trios-server/src/tools/mod.rs [valid-output: unchecked; cfg(test) 0]: count:4:M
- WT trios:crates/trios-store/rings/ST-00/src/lib.rs [valid-output: unchecked; cfg(test) 39]: Adapter::as_str:8:N-ty- Adapter::parse:9:N-ty- DetectedBy::default:3:N-trait DetectedBy::as_str:6:N-ty- DetectedBy::parse:7:N-ty-
- WT trios:crates/trios-ternary/src/lib.rs [valid-output: unchecked; cfg(test) 58]: Ternary::from_f32:10:N-ty- ternary_size_bytes:11:M
- WT trios:crates/trios-train-cpu/src/attention.rs [valid-output: unchecked; cfg(test) 238]: relu_squared:5:M relu_squared_backward:8:M AttentionConfig::head_dim:3:N-ty+ AttentionConfig::d_ffn:3:N-ty+
- WT trios:crates/trios-train-cpu/src/mup.rs [valid-output: unchecked; cfg(test) 100]: MupConfig::default:10:N-trait MupConfig::lr_multiplier:10:N-ty- MupConfig::validate:13:N-ty+ scale_dimensions:12:N-ty+ ModelDims::head_dim:4:N-ty+ ModelDims::validate:13:N-ty+
- WT trios:crates/trios-train-cpu/src/schedule.rs [valid-output: unchecked; cfg(test) 67]: WsdSchedule::new:4:N-ty+
- WT trios:crates/trios-trainer/src/backward.rs [valid-output: unchecked; cfg(test) 153]: softmax_cross_entropy_backward:29:M
- WT trios:crates/trios-trainer/src/config.rs [valid-output: unchecked; cfg(test) 13]: default_lr:1:M default_checkpoint_interval:1:M default_eval_interval:1:M
- WT trios:crates/trios-trainer/src/forward.rs [valid-output: unchecked; cfg(test) 107]: LayerDims::default:8:N-trait vec_add:12:M vec_scale:13:M
- WT trios:crates/trios-trainer/src/optimizer.rs [valid-output: unchecked; cfg(test) 272]: lr_schedule_54_f64:21:M
- WT trios:crates/trios-trainer/src/train_loop.rs [valid-output: unchecked; cfg(test) 10]: evaluate_step:10:C
- WT trios:crates/trios-trainer/src/validation.rs [valid-output: unchecked; cfg(test) 41]: is_within_champion_tolerance:24:M
- WT trios:crates/trios-tri/src/core_compat.rs [valid-output: unchecked; cfg(test) 95]: ternary_compression_ratio:18:M ternary_compression_vs_gf16:18:M format_description:9:M
- WT trios:crates/trios-tri/src/lib.rs [valid-output: unchecked; cfg(test) 64]: Ternary::from_f32:25:N-ty- ternary_size_bytes:21:M
- WT trios:crates/trios-tri/src/qat.rs [valid-output: unchecked; cfg(test) 139]: TernarySTE::new:7:N-ty+ TernarySTE::with_default_threshold:3:N-ty+ TernarySTE::backward:14:N-ty+ TernarySTE::threshold:4:N-ty+ TernarySTE::set_threshold:4:N-ty+ TernarySTE::default:3:N-trait LearnableScale::new:14:N-ty+ LearnableScale::with_min_scale:13:N-ty+ LearnableScale::value:4:N-ty+ LearnableScale::update:22:N-ty+ LearnableScale::apply:11:N-ty+ LearnableScale::invert:11:N-ty+ LearnableScale::reset:5:N-ty+ QatConfig::new:11:N-ty+ QatConfig::with_defaults:6:N-ty+ QatConfig::backward:11:N-ty+ QatConfig::update_scale:8:N-ty+ QatConfig::update_threshold:10:N-ty+ QatConfig::default:6:N-trait
- WT trios:crates/trios-trinity-brain/src/phi.rs [valid-output: unchecked; cfg(test) 0]: phi_add:3:M phi_mul:3:M phi_subtract:3:M
- WT trios:crates/trios-zig-agents/src/lib.rs [valid-output: unchecked; cfg(test) 12]: FlyRegion::to_region_code:14:N-ty- FlyRegion::to_location_name:14:N-ty- send_collaboration_message:16:N-attr deploy_to_fly:23:N-attr restart_instance:10:N-attr stop_instance:10:N-attr send_collaboration_message:4:N-attr deploy_to_fly:4:N-attr restart_instance:4:N-attr stop_instance:4:N-attr
- WT trios:tools/acm_ae_check/src/lib.rs [valid-output: unchecked; cfg(test) 190]: functional_required_paths:12:M reusable_required_needles:13:M available_required_anchors:4:M
- WT trios:tools/citetheorem_audit/src/lib.rs [valid-output: unchecked; cfg(test) 305]: strip_latex_line_comment:12:N-bad
- WT trios:tools/defense_gate/src/lib.rs [valid-output: unchecked; cfg(test) 288]: ReportFloors::from_registry:8:N-ty- count_substr:15:C

## Not run

- `t27c test-report` of 45 specs (30 trios files, 15 files of trinity, trinity-fpga and 999-multibots-telegraf): stopped when the shared Mac ran low on disk under load. Their rows say `valid-output: unchecked`.
  Their gen-rust output was still compiled with `rustc`.
- `tri t27o --check` (the oracle replay) on any file: not run. The only behavioural comparison is the differential run of the ring-092 bundle above; no other group was compared against its hand fns.
- No `cargo build`, `cargo check` or `cargo test` of any crate. Callers come from a token index of every `.rs` file of the five repositories (comments and strings blanked), with module-path, import and crate-name matching,
  then `git grep` by hand for every MOVABLE row that had a hit elsewhere; a caller that reaches a fn through a glob import and another name, or through a macro, would be missed. Only the ring-092 bundle was mounted and
  compiled (with `rustc`, the crate has no dependency).
- "gen-rust compiles" is a library compile of the generated file with the refused stubs removed, repeated after removing every item that failed; it does not prove the converted body equals the hand body.
- No seal, ledger row, `t27b` ledger edit, own-language gate run or `dupe_scan` over any spec but the ring-092 group spec; no spec was added to the repository.
- Method rows were not checked for the type-move variant (moving a type with its fns): the `ty+` rows are the candidates for it.
