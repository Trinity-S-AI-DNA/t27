# NOW -- compiler.rs HIR cost models move to .t27, converted by t27o (2026-10-12)

## specs/compiler/hir_cost_models.t27, bootstrap/src/compiler.rs (Closes #9114; t27o epic #8998)

- Second port of the reference compiler's hand Rust through our own converter, after #9093: `tri t27o
  --check` turned the 13 static estimates of `TimingModel`, `PowerModel`, `RouteModel` and `CtsModel`
  into t27 (13/13 converted, 0 refused; oracle passed, 0 vacuous).
- The 12 `Self::` consts the 13 fns read become module consts; the impl keeps the consts its other
  readers use (`analyze_module` still reads `Self::BRAM_DELAY_PS`; the unit tests read them all).
  `t27c gen-rust` writes `bootstrap/gen/rust/compiler/hir_cost_models.rs`, mounted by `#[path]` as
  #8991/#9093 mounted theirs. The hand bodies become one-line calls; signatures kept, callers untouched.
- Tests: one per function, 130 runtime asserts, expected values printed by the hand Rust itself on the
  lab (every arm, both sides of every threshold, truncation edges) -- not t27o's generic samples, which
  miss arms (#9102). A consts test was dropped: const equality folds at compile time and passes vacuous
  (T730); the constants are pinned by the fn tests that read them.
- The board-side twins live in specs/fpga/{timing,power,router,cts}.t27 (old-style, consts as fns).
  Bodies differ token for token, so dupe_scan reports no new pair; a number change must move both homes.
- Generated output of gen, gen-verilog, gen-rust and gen-c over 2284 tracked specs: byte-identical with
  t27c built from master and from this branch. A planted fault in the generated module (LUT_DELAY_PS
  100 -> 101) moved 0 corpus rows -- these 13 fns feed reports and unit tests, not codegen (#9093's
  int_type_names sits on the Zig codegen path, which is why its fault moved 18). The fault was caught by
  the wrappers' own unit tests in compiler.rs: test_est_comb_delay and test_est_reg_to_reg_delay flipped
  (603 != 600, 803 != 800), 662 passed / 2 failed under `cargo test --release -p t27c --bin t27c est_`.
  The spec's own tests stayed 13/13 -- they compile the spec, not the mounted module, which is exactly
  why the corpus sweep and the unit tests are both part of the evidence.
- Inherited from master, present before this branch: `cargo test` cannot build the `tri` test target
  (cli/tri/src/gates.rs still include_str!s the deleted brain-seal-refresh.yml), and the dupe ledger
  gate reports five groups from the verilog_names / t27o-service ports. Both identical with and without
  this branch; none of the five names a body from this spec. The gates.rs repair is owner-gated.
- FROZEN_HASH re-computed under the owner's standing rule for ports (2026-10-12).
