# NOW -- the Rust files t27o converts whole, ranked by the hand lines a port deletes (2026-10-20)

## docs/now/2026-10-20-golden-candidates.md (Closes #11149, step 4 of #9000; analysis only)

- Scope: no Rust is deleted and no spec is added by this change. It measures which hand-written Rust files of our own
  repositories t27o converts whole, what a port would delete, and which of them a generated file replaces without
  changing a caller. The table below is the same one posted on #11149.
- Sweep: 1,388 `.rs` files (t27 382, trios 786, trinity 23, trinity-fpga 30, 999-multibots-telegraf 167; 435,491
  physical lines), one process per file, the `sweep_all.zsh` driver of lane A4 (every `.rs` of its copy of each checkout;
  the t27 copy has no `gen/` files), summary written 2026-10-20 00:48 local with that lane's converter build, which is earlier than ca90be110
  (02:10; #11130 changed refusal comments only and lost no converted fn). It is not a `tri t27o-sweep` run: there is
  no `tri` build on this Mac and the shared `tri` binary predates the converter. t27 had 376 non-generated `.rs`
  tracked at master plus 6 files of the swept checkout that master does not track (none of them converts whole).
- Whole: 85 files have `fns seen == converted > 0` (the 85 of #11149). 4 of them refuse an item that is not a fn
  and are not whole (list below), leaving 81. One of the 81 is a test-only integration test (excluded), leaving
  80 ranked rows with 3,006 physical lines (2,569 non-blank; 1,301 of the lines are `#[cfg(test)]`). That is
  0.69% of the 435,491 swept lines.
- Re-check at master a1780a1ba: `gen/rust/tools/t27o/rust_convert.rs` (sha256 prefix fdac1e1c4eeebc21) was last
  changed by ca90be110; no commit since touches `gen/rust/tools/t27o` or `specs/tools/t27o`. A driver built from it
  with rustc reproduces the fn, test and refusal counts of all 85 files exactly.

## Answer

- PORTABLE-NOW: 78 rows, 2,864 lines. NEEDS-CALLER-CHANGE: 1 row (SC-00, 40 lines). NOT-NOW: 1 row (rainbow-bridge
  `lib.rs`, 102 lines). Excluded: 1 test-only file, 8 generated files that the sweep did not see.
- Of the 78 PORTABLE-NOW rows: 42 keep every public item (855 lines), 7 keep every name and compile every caller
  unchanged (1,186 lines: five CR-CHAT guards, `trinity_loss`, `bitmask`), 29 rename items nobody uses (823 lines).
- 70 of the 80 ranked rows are scaffold stubs of 20-32 lines with no caller and no workspace build (1,643 lines).
  The other 10 rows (1,363 lines) are real code. Replacing a scaffold with a generated copy of itself is mechanical and
  gains nothing; deleting it is the trios owner's call, not a port.
- Seven rows delete at least 100 lines with every caller unchanged: the five CR-CHAT guards (837 lines),
  `crates/trinity_loss/src/lib.rs` (179) and `bootstrap/src/host/bitmask.rs` (170). First by rank is
  `wire_frame_integrity_guard.rs` (184 lines), but it narrows `Result<(), WireFrameError>` to `bool`, and trios has no
  generated root and no own-language gate to deliver it (below).
- First golden candidate: `bootstrap/src/host/bitmask.rs` of this repository, 170 lines (139 non-blank, 96 of them
  `#[cfg(test)]`), one caller (two lines of `bootstrap/src/host/mod.rs`), gate-counted hand lines +1 / -171.
  Section "First candidate" has the evidence. After it lands, no file of t27 converts whole (1 of 382 today).

## Ranked rows

Lines are physical lines (what `git diff --numstat` counts when the file is deleted) / non-blank lines / lines inside
`#[cfg(test)]`. fns excludes tests. "callers" is the number of files outside the row that name a public item or re-export it (then which), read
from an identifier index of every tracked text file of the five checkouts plus the 154 files of trios main that the
sweep missed; the callers of the ten non-stub rows were then read by hand, and for nine of them compiled against the
generated file (see "Not run"). "workspace" is
whether the trios Cargo workspace (main's 88 members and their path dependencies) builds the crate.
"t27c test-report" is `pass/tests, vac N` (vacuous passes). "oracle replay" is the `tri t27o --check` path replayed
by hand: oracle harness compiled with rustc, run, appended to the spec as tests, judged by `t27b test`
(`pass`/`fail` count the spec's tests; `asserts` is the number of recorded oracle vectors). All 85 files: gen-rust
output compiles with `rustc --crate-type lib`, `t27c test-report` exits 0, replay passes.

| # | repo | path | lines / non-blank / cfg(test) | fns / tests | public items | callers outside the file | crate (workspace) | t27c test-report | oracle replay | api after gen-rust | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | trios | crates/trios-chat/rings/CR-CHAT-04/src/wire_frame_integrity_guard.rs | 184 / 167 / 95 | 1 / 10 | WFGI_TAG_LEN, WFGI_MAX_HEADER, WFGI_MAX_FRAME (+2) | 1: the ring lib.rs (pub mod + pub use); no file calls the items | trios-chat-cr-chat-04 (member) | 10/10, vac 0 | PASS 11 pass 0 fail, 11 asserts | compatible, contract narrowed | PORTABLE-NOW |
| 2 | trios | crates/trios-chat/rings/CR-CHAT-06/src/tool_response_timeout_guard.rs | 180 / 162 / 100 | 2 / 10 | TRTO_MIN_TIMEOUT_MS, TRTO_MAX_TIMEOUT_MS, ToolTimeoutError (+1) | 1: the ring lib.rs (pub mod + pub use); no file calls the items | trios-chat-cr-chat-06 (member) | 10/10, vac 0 | PASS 12 pass 0 fail, 10 asserts | compatible, contract narrowed | PORTABLE-NOW |
| 3 | trios | crates/trinity_loss/src/lib.rs | 179 / 165 / 37 | 5 / 5 | PHI_INV_SQ, DEFAULT_MARGIN, DEFAULT_LAMBDA (+5) | 1: tests/loss.rs (16 tests) | trinity_loss (not built) | 5/5, vac 0 | PASS 10 pass 0 fail, 35 asserts | compatible | PORTABLE-NOW |
| 4 | trios | crates/trios-chat/rings/CR-CHAT-07/src/cover_decoy_ratio_governor.rs | 178 / 158 / 82 | 1 / 10 | CDRG_MIN_RATIO_NUM, CDRG_MIN_RATIO_DEN, CDRG_MAX_RATIO_NUM (+5) | 1: the ring lib.rs (pub mod + pub use); no file calls the items | trios-chat-cr-chat-07 (member) | 10/10, vac 0 | PASS 11 pass 0 fail, 10 asserts | compatible, contract narrowed | PORTABLE-NOW |
| 5 | t27 | bootstrap/src/host/bitmask.rs | 170 / 139 / 96 | 16 / 14 | mask, field_mask, extract (+14) | 1: host/mod.rs (pub mod + pub use of 10 names) | t27c (t27c bin) | 14/14, vac 0 | PASS 25 pass 0 fail, 95 asserts | compatible | PORTABLE-NOW |
| 6 | trios | crates/trios-chat/rings/CR-CHAT-04/src/padding_metadata_leak_guard.rs | 157 / 141 / 83 | 1 / 10 | PMLG_PREFIX, PMLG_CLASSES, PadMetaLeakError (+1) | 1: the ring lib.rs (pub mod + pub use); no file calls the items | trios-chat-cr-chat-04 (member) | 10/10, vac 0 | PASS 11 pass 0 fail, 9 asserts | compatible, contract narrowed | PORTABLE-NOW |
| 7 | trios | crates/trios-chat/rings/CR-CHAT-01/src/identity_key_usage_count_guard.rs | 138 / 121 / 70 | 2 / 10 | IKUC_MAX_SIGNATURES, IKUC_WARN_SIGNATURES, KeyUsageError (+2) | 1: the ring lib.rs (pub mod + pub use); no file calls the items | trios-chat-cr-chat-01 (member) | 10/10, vac 0 | PASS 12 pass 0 fail, 16 asserts | compatible, contract narrowed | PORTABLE-NOW |
| 8 | trios | crates/trios-rainbow-bridge/src/lib.rs | 102 / 89 / 21 | 0 / 2 | bridge, channel, event (+10) | 3: tests/falsify.rs, tests/integration.rs, trios-server/src/rainbow_routes.rs; the 8 submodules it declares | trios-rainbow-bridge (member) | 2/2, vac 2 | PASS 2 pass 0 fail, 0 asserts | items dropped | NOT-NOW |
| 9 | trios | crates/trios-sacred/rings/SC-00/src/lib.rs | 40 / 33 / 11 | 2 / 1 | Vec2, Vec2::new, Vec2::dot (+2) | 2: SC-01/src/lib.rs, BR-OUTPUT/src/lib.rs (Vec2::new x3) | trios-sacred-sc00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 10 asserts | changed | NEEDS-CALLER-CHANGE |
| 10 | trios | crates/trios-physics/rings/PH-00/src/lib.rs | 35 / 27 / 15 | 0 / 2 | C, H, HBAR (+2) | 2: PH-01/src/lib.rs, BR-OUTPUT/src/lib.rs | trios-physics-ph00 (not built) | 2/2, vac 2 | PASS 2 pass 0 fail, 0 asserts | same | PORTABLE-NOW |
| 11 | trios | crates/trios-bridge/rings/BR-OUTPUT/src/lib.rs | 32 / 26 / 14 | 1 / 2 | BridgeAssembly, BridgeAssembly::new, BridgeAssembly::TRINITY_ANCHOR | 0 | trios-bridge-broutput (not built) | 2/2, vac 1 | PASS 2 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 12 | trios | crates/trios-claude/rings/BR-OUTPUT/src/lib.rs | 32 / 26 / 14 | 1 / 2 | ClaudeAssembly, ClaudeAssembly::new, ClaudeAssembly::TRINITY_ANCHOR | 0 | trios-claude-broutput (not built) | 2/2, vac 1 | PASS 2 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 13 | trios | crates/trios-data/rings/BR-OUTPUT/src/lib.rs | 32 / 26 / 14 | 1 / 2 | DataAssembly, DataAssembly::new, DataAssembly::TRINITY_ANCHOR | 0 | trios-data-broutput (not built) | 2/2, vac 1 | PASS 2 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 14 | trios | crates/trios-hybrid/rings/BR-OUTPUT/src/lib.rs | 32 / 26 / 14 | 1 / 2 | HybridAssembly, HybridAssembly::new, HybridAssembly::TRINITY_ANCHOR | 0 | trios-hybrid-broutput (not built) | 2/2, vac 1 | PASS 2 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 15 | trios | crates/trios-kg/rings/BR-OUTPUT/src/lib.rs | 32 / 26 / 14 | 1 / 2 | KgAssembly, KgAssembly::new, KgAssembly::TRINITY_ANCHOR | 0 | trios-kg-broutput (not built) | 2/2, vac 1 | PASS 2 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 16 | trios | crates/trios-model/rings/BR-OUTPUT/src/lib.rs | 32 / 26 / 14 | 1 / 2 | ModelAssembly, ModelAssembly::new, ModelAssembly::TRINITY_ANCHOR | 0 | trios-model-broutput (not built) | 2/2, vac 1 | PASS 2 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 17 | trios | crates/trios-precision-router/rings/BR-OUTPUT/src/lib.rs | 32 / 26 / 14 | 1 / 2 | PrecrouterAssembly, PrecrouterAssembly::new, PrecrouterAssembly::TRINITY_ANCHOR | 0 | trios-precrouter-broutput (not built) | 2/2, vac 1 | PASS 2 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 18 | trios | crates/trios-sdk/rings/BR-OUTPUT/src/lib.rs | 32 / 26 / 14 | 1 / 2 | SdkAssembly, SdkAssembly::new, SdkAssembly::TRINITY_ANCHOR | 0 | trios-sdk-broutput (not built) | 2/2, vac 1 | PASS 2 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 19 | trios | crates/trios-bridge/rings/BR-00/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Br00Scope, Br00Scope::RING, Br00Scope::PURPOSE (+1) | 0 | trios-bridge-br00 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 20 | trios | crates/trios-bridge/rings/BR-01/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Br01Scope, Br01Scope::RING, Br01Scope::PURPOSE (+1) | 0 | trios-bridge-br01 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 21 | trios | crates/trios-bridge/rings/BR-02/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Br02Scope, Br02Scope::RING, Br02Scope::PURPOSE (+1) | 0 | trios-bridge-br02 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 22 | trios | crates/trios-claude/rings/CL-00/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Cl00Scope, Cl00Scope::RING, Cl00Scope::PURPOSE (+1) | 0 | trios-claude-cl00 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 23 | trios | crates/trios-claude/rings/CL-01/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Cl01Scope, Cl01Scope::RING, Cl01Scope::PURPOSE (+1) | 0 | trios-claude-cl01 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 24 | trios | crates/trios-claude/rings/CL-02/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Cl02Scope, Cl02Scope::RING, Cl02Scope::PURPOSE (+1) | 0 | trios-claude-cl02 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 25 | trios | crates/trios-data/rings/DT-00/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Dt00Scope, Dt00Scope::RING, Dt00Scope::PURPOSE (+1) | 0 | trios-data-dt00 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 26 | trios | crates/trios-data/rings/DT-01/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Dt01Scope, Dt01Scope::RING, Dt01Scope::PURPOSE (+1) | 0 | trios-data-dt01 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 27 | trios | crates/trios-data/rings/DT-02/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Dt02Scope, Dt02Scope::RING, Dt02Scope::PURPOSE (+1) | 0 | trios-data-dt02 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 28 | trios | crates/trios-hybrid/rings/HY-00/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Hy00Scope, Hy00Scope::RING, Hy00Scope::PURPOSE (+1) | 0 | trios-hybrid-hy00 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 29 | trios | crates/trios-hybrid/rings/HY-01/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Hy01Scope, Hy01Scope::RING, Hy01Scope::PURPOSE (+1) | 0 | trios-hybrid-hy01 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 30 | trios | crates/trios-kg/rings/KG-00/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Kg00Scope, Kg00Scope::RING, Kg00Scope::PURPOSE (+1) | 0 | trios-kg-kg00 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 31 | trios | crates/trios-kg/rings/KG-01/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Kg01Scope, Kg01Scope::RING, Kg01Scope::PURPOSE (+1) | 0 | trios-kg-kg01 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 32 | trios | crates/trios-kg/rings/KG-02/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Kg02Scope, Kg02Scope::RING, Kg02Scope::PURPOSE (+1) | 0 | trios-kg-kg02 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 33 | trios | crates/trios-model/rings/MD-00/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Md00Scope, Md00Scope::RING, Md00Scope::PURPOSE (+1) | 0 | trios-model-md00 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 34 | trios | crates/trios-model/rings/MD-01/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Md01Scope, Md01Scope::RING, Md01Scope::PURPOSE (+1) | 0 | trios-model-md01 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 35 | trios | crates/trios-precision-router/rings/PR-00/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Pr00Scope, Pr00Scope::RING, Pr00Scope::PURPOSE (+1) | 0 | trios-precrouter-pr00 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 36 | trios | crates/trios-precision-router/rings/PR-01/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Pr01Scope, Pr01Scope::RING, Pr01Scope::PURPOSE (+1) | 0 | trios-precrouter-pr01 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 37 | trios | crates/trios-sdk/rings/SD-00/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Sd00Scope, Sd00Scope::RING, Sd00Scope::PURPOSE (+1) | 0 | trios-sdk-sd00 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 38 | trios | crates/trios-sdk/rings/SD-01/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Sd01Scope, Sd01Scope::RING, Sd01Scope::PURPOSE (+1) | 0 | trios-sdk-sd01 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 39 | trios | crates/trios-sdk/rings/SD-02/src/lib.rs | 27 / 22 / 10 | 1 / 1 | Sd02Scope, Sd02Scope::RING, Sd02Scope::PURPOSE (+1) | 0 | trios-sdk-sd02 (not built) | 1/1, vac 0 | PASS 1 pass 0 fail, 0 asserts | renamed (no caller) | PORTABLE-NOW |
| 40 | trios | crates/trios-ca-mask/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-ca-mask-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 41 | trios | crates/trios-ca-mask/rings/CM-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-ca-mask-cm00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 42 | trios | crates/trios-ca-mask/rings/CM-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-ca-mask-cm01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 43 | trios | crates/trios-cli/rings/BR-BIN/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-cli-brbin (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 44 | trios | crates/trios-cli/rings/CI-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-cli-ci00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 45 | trios | crates/trios-cli/rings/CI-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-cli-ci01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 46 | trios | crates/trios-cli/rings/CI-02/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-cli-ci02 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 47 | trios | crates/trios-fpga/rings/BR-BITSTREAM/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-fpga-brbitstream (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 48 | trios | crates/trios-fpga/rings/FP-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-fpga-fp00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 49 | trios | crates/trios-fpga/rings/FP-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-fpga-fp01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 50 | trios | crates/trios-fpga/rings/FP-02/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-fpga-fp02 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 51 | trios | crates/trios-gb/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-gb-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 52 | trios | crates/trios-gb/rings/GB-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-gb-gb00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 53 | trios | crates/trios-gb/rings/GB-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-gb-gb01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 54 | trios | crates/trios-git/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-git-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 55 | trios | crates/trios-git/rings/GT-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-git-gt00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 56 | trios | crates/trios-git/rings/GT-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-git-gt01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 57 | trios | crates/trios-igla-race/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-igla-race-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 58 | trios | crates/trios-igla-race/rings/IR-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-igla-race-ir00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 59 | trios | crates/trios-igla-race/rings/IR-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-igla-race-ir01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 60 | trios | crates/trios-operator-smoke/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-operator-smoke-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 61 | trios | crates/trios-operator-smoke/rings/OS-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-operator-smoke-os00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 62 | trios | crates/trios-operator-smoke/rings/OS-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-operator-smoke-os01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 63 | trios | crates/trios-phi-schedule/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-phi-schedule-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 64 | trios | crates/trios-phi-schedule/rings/PS-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-phi-schedule-ps00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 65 | trios | crates/trios-phi-schedule/rings/PS-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-phi-schedule-ps01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 66 | trios | crates/trios-rainbow-bridge/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-rainbow-bridge-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 67 | trios | crates/trios-rainbow-bridge/rings/RB-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-rainbow-bridge-rb00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 68 | trios | crates/trios-rainbow-bridge/rings/RB-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-rainbow-bridge-rb01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 69 | trios | crates/trios-rainbow-bridge/rings/RB-02/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-rainbow-bridge-rb02 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 70 | trios | crates/trios-train-cpu/rings/BR-MODEL/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-train-cpu-brmodel (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 71 | trios | crates/trios-train-cpu/rings/TC-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-train-cpu-tc00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 72 | trios | crates/trios-train-cpu/rings/TC-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-train-cpu-tc01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 73 | trios | crates/trios-tri/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-tri-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 74 | trios | crates/trios-tri/rings/TI-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-tri-ti00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 75 | trios | crates/trios-tri/rings/TI-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-tri-ti01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 76 | trios | crates/trios-tri/rings/TI-02/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-tri-ti02 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 77 | trios | crates/trios-vm/rings/BR-OUTPUT/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-vm-broutput (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 78 | trios | crates/trios-vm/rings/VM-00/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-vm-vm00 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 79 | trios | crates/trios-vm/rings/VM-01/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-vm-vm01 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |
| 80 | trios | crates/trios-vm/rings/VM-02/src/lib.rs | 20 / 17 / 9 | 1 / 1 | ring_id | 0 | trios-vm-vm02 (not built) | 1/1, vac 0 | PASS 2 pass 0 fail, 1 asserts | same | PORTABLE-NOW |

Families of the 80 rows:

| family | files | lines | callers | api after gen-rust | verdict |
|---|---|---|---|---|---|
| `ring_id()` scaffold (`rings/*/src/lib.rs`) | 41 | 820 | none | same (`pub fn ring_id() -> &'static str`) | PORTABLE-NOW |
| ring-scope scaffold (`XScope`, RING, PURPOSE, new) | 21 | 567 | none | renamed: assoc consts and `new` become `XScope_RING`, `XScope_new`; unit struct becomes braced | PORTABLE-NOW (no caller) |
| BR-OUTPUT assembly scaffold (`XAssembly`) | 8 | 256 | none | renamed, same as above | PORTABLE-NOW (no caller) |
| CR-CHAT guards (trios-chat rings 01, 04 x2, 06, 07) | 5 | 837 | ring `lib.rs` re-export only | names kept, `Result<(), E>` becomes `bool`, `#[non_exhaustive]` lost | PORTABLE-NOW (contract narrowed) |
| `trinity_loss/src/lib.rs` | 1 | 179 | `tests/loss.rs` | `&[i8; 64]` becomes `impl AsRef<[i8]>`; the 16 tests pass unchanged against the generated lib | PORTABLE-NOW (DRY blocks, see below) |
| `bootstrap/src/host/bitmask.rs` | 1 | 170 | `host/mod.rs` | names kept; `const fn` becomes `fn`; `FieldDesc` methods become free fns; none used | PORTABLE-NOW |
| `PH-00/src/lib.rs` (5 `f64` consts) | 1 | 35 | PH-01, BR-OUTPUT | same | PORTABLE-NOW |
| `SC-00/src/lib.rs` | 1 | 40 | SC-01, BR-OUTPUT | `Vec2::new` and `.dot()` become free fns | NEEDS-CALLER-CHANGE |
| `trios-rainbow-bridge/src/lib.rs` | 1 | 102 | the crate | 8 `pub mod`, 8 `pub use`, 2 inner attributes dropped | NOT-NOW |

## Not ranked

Seen == converted but an item is refused (not whole):

| repo | path | lines | refused |
|---|---|---|---|
| trios | crates/trios-agents/src/types.rs | 130 | 7 structs (a `String` field) |
| trios | crates/trios-kg/src/types.rs | 109 | 7 items (`String`, `Vec` fields) |
| 999-multibots-telegraf | rust/rings/GOLD-RING-TY00/src/payment.rs | 49 | struct `Transaction` (a `uuid` field) |
| trios | crates/trios-trinity-brain/src/phi.rs | 17 | item `phi_attention` |

Excluded:

- `crates/trios-coq-witness/tests/witness_lepton_mass_chain_L01_L02_L03_numerical.rs` (25 lines): test-only path
  (`tests/`); its converted spec is a test already, 1 test, vacuous.
- The sweep read the local trios checkout (a feature branch), which lacks 153 `.rs` files that trios main has
  (111 under `apps/trios-macos/rings`), and trinity-fpga lacks a vendored `zig-pkg` file. Those 154 were fetched from
  GitHub main and converted with the same driver: 8 convert whole, all under
  `apps/trios-macos/rings/RUST-13/trios-mesh/gen/rust/` (483 lines, header "Generated from .t27 spec / DO NOT
  EDIT"). Generated, excluded.
- The 84 non-t27 whole files are byte-identical (git blob sha) to GitHub main at trios 78671ba794, trinity
  8e535bc9bb, trinity-fpga c9b180bf78, 999-multibots-telegraf fcdbb2f7dc. The sweep checkouts were at trios 483dd9ee
  (branch golden-bridge-p1-bibliography), trinity 0770abb9, trinity-fpga 20bf4c41, 999-multibots-telegraf 6e551289.
  The t27 row is `bootstrap/src/host/bitmask.rs` at master a1780a1ba.

## What a port changes (gen-rust mapping against the original)

Counted over the 80 ranked rows, from the converted spec's `t27c gen-rust` output:

- Kept exactly: integer types (`usize`, `u64`, `u32`, `u8`), `f64`/`f32` consts and `&'static str` (41 `ring_id`
  returns, the RING/PURPOSE consts). Numeric literals lose their `_` separators (same value).
- `Result<(), E>` becomes `bool` (5 fns, the CR-CHAT guards): the converter drops the `Err` payload ("Err payload
  dropped"), so a caller that matches the variant cannot be written. `#[non_exhaustive]` on the error enums is lost
  (5), and the enum is left without a producer.
- Methods and associated items become free items named `Type_name` with `self_` first: `impl` methods (31 rows),
  associated consts (29 rows). `Vec2::new(1.0, 0.0)` becomes `Vec2_new(1.0, 0.0)`. The only row with a caller that
  does this is SC-00; `rustc` on `SC-01/src/lib.rs` against the generated `SC-00` says E0599 "no associated function
  `new` found for struct `Vec2`".
- `const fn` becomes `fn` (8 fns in `bitmask.rs`): a `const` context using them stops compiling (none exists).
- `&[i8; 64]` becomes `impl AsRef<[i8]>` (5 fns of `trinity_loss`): every old call form still compiles, but the
  length is no longer part of the type.
- Unit and tuple structs become braced structs (30 rows); `Default`, `PartialEq`, `Eq` derives are dropped (31 rows)
  while the output always derives `Debug`, `Clone`, `Copy`.
- Doc comments, `#[inline]`, doctests and inner attributes (`#![forbid(unsafe_code)]` in the five CR-CHAT files,
  `#![deny(missing_docs, unsafe_code)]` in rainbow-bridge) are not carried. The `#[cfg(test)]` modules move into the
  spec as `test` blocks; `cargo test` loses them.
- The `use` splice adds public helper fns to the generated module (`pub fn same` in 70 files, `count_ones_u32` and
  friends in `bitmask.rs`): extra public names, harmless behind an explicit `pub use` list.
- "Whole" overclaims for a module hub: `trios-rainbow-bridge/src/lib.rs` has 8 `pub mod`, 8 `pub use` and 2 inner
  attributes that the converter drops without a refusal line; only its 5 consts and 2 tests convert. A port would
  break the crate. The refusal count should include dropped `mod`, `use` and inner-attribute items.
- Six of the 85 files already have a hand-written port spec under `specs/port/` (`trinity_loss`, three CR-CHAT files,
  SC-00, `phi.rs`). They disagree with the original too: `trinity_loss` takes `[i8; 64]` by value (the `&a` call
  sites of `tests/loss.rs` would not compile) and `validate_wire_frame` returns `i32` codes. `dupe_scan --like` flags
  `phi_prior_term` of the converted `trinity_loss` spec as already written there. A golden port of these files must
  replace or reconcile the old spec, or the DRY criterion of `specs/tools/t27o/golden.t27` refuses it.

## Delivery: where a generated file can replace a hand file

- t27: 117 `#[path = "...gen/rust/..."]` mounts in 22 hand files (`bootstrap/src/main.rs`, `compiler.rs`,
  `cli/t27b/src/lower.rs`, `cli/tri/src/main.rs`, ...). `bootstrap/gen/`, `gen/` and `bootstrap/src/memory/generated/`
  are generated roots, so the generated file is not counted; the `own-language` budget needs the diff to lose at
  least as many hand lines as it gains. Precedent: #7530 deleted `cli/t27b/src/lower/lencall.rs` (56 lines) and
  mounted `gen/rust/tri/t27b/lencall.rs`.
- trios (80 of 81 whole files; 79 of the 80 ranked rows): no generated root, no own-language gate or foreign-lines
  budget (21 workflows on main, none named for one), and no t27c, `tri gen` or gen-rust step in the 17 workflows of the sweep checkout. Its only generated Rust is the `trios-mesh`
  `gen/rust/` tree, which its own `lib.rs` documents as excluded from compilation. 73 of the 80 ranked rows are in
  crates that the workspace does not build (not a member, no path dependency from a member, absent from main's
  `Cargo.lock`). The only ranked trios crates in the lock are CR-CHAT rings 01, 04, 06, 07 (five files) and
  rainbow-bridge (one file).
- A golden PR in trios therefore needs, first, a generated-file convention in trios (header plus root) and the gate;
  until then the verdicts above say the replacement is safe for callers, not that it can be delivered.

## First candidate: bootstrap/src/host/bitmask.rs

- File: 170 physical lines, 139 non-blank, 96 `#[cfg(test)]` lines (14 `#[test]`); 11 free fns, `FieldDesc` and 5
  methods. Crate `t27c` (binary, no `[lib]`), module `host` (`mod host;` in `main.rs`).
- Callers: `bootstrap/src/host/mod.rs` line 26 `pub mod bitmask;` and line 69
  `pub use bitmask::{clear_bit, extract, field_mask, insert, is_set, mask, popcount, set_bit, toggle_bit, FieldDesc};`.
  Nothing else in the five repositories names `host::bitmask`, `FieldDesc`, or these fns through the module. All ten
  re-exported names exist in the generated file, and that `pub use` line compiles against it.
- Converted spec: 163 lines, 16 fns, 14 tests. `t27c test-report`: 14/14 pass, 0 vacuous. `t27b test` of the spec plus
  the oracle: 25 tests pass, 0 fail, 95 recorded asserts. `dupe_scan --like`: nothing in the file is written
  elsewhere. `t27c gen-rust` output (131 lines with the spliced std helpers) compiles with `rustc --crate-type lib`.
- Equivalence beyond the recorded vectors: a differential program compiled the hand file and the generated file side by
  side and compared every fn over all widths and (high, low) pairs and nine values: 68,268 comparisons, 0
  disagreements. A simulated tree with the generated file mounted by
  `#[path = "../../gen/rust/host/bitmask.rs"] pub mod bitmask;` built and ran.
- Shape of the pull request: delete `bootstrap/src/host/bitmask.rs` (-170); line 26 of `mod.rs` becomes the mount line
  with the `#[allow(...)]` attributes the other mounts carry (+1 / -1); add the spec (for example
  `specs/host/bitmask.t27`, directory to confirm against `specs/OWNERS.md`) and `bootstrap/gen/rust/host/bitmask.rs`.
  The `own-language` budget from the base revision, fed this numstat (1/1 mod.rs, 0/170 bitmask.rs, 150/0 spec,
  130/0 generated), returns rc 0; a control of +60/-0 in one hand file returns rc 1 with the 40-line message.
- Side effects to state in that PR: `cargo test -p t27c` loses 14 tests (they move into the spec); `const fn` and the
  `FieldDesc` methods are gone (no user); the new spec needs a seal, a t27b ledger row and a types-ratchet check
  (the diff declares `FieldDesc`; the name appears in `specs/` only inside `rust_convert.t27` comments).

## Not run

- `tri t27o --check` itself and `tri t27o-sweep`: no `tri` build (no cargo build on this Mac, disk). The `--check` path
  was replayed by hand with a driver built from master's converter, `t27b-tb2-c3` for the judge, and the shared
  `t27c` (13:04 on 2026-10-19) for `test-report` and `gen-rust`.
- No `cargo build` or `cargo test` of `t27c`, `trios` or any crate: caller claims rest on `rustc` simulations (the
  `pub use` lines of five rings, `PH-01` and `BR-OUTPUT` against the generated PH-00, `trinity_loss/tests/loss.rs`
  against the generated lib, the `#[path]` mount) and on the identifier index, not on a workspace build.
- Callers were searched in the five repositories only (BrowserOS, ghashtag.github.io and trios-railway were not
  read); the index is word-level, so a caller that reaches an item through a glob import and a different name would be
  missed (the CR-CHAT-04 glob `pub use trios_chat_cr_chat_04::*` in `BR-OUTPUT-CHAT` was checked: no file names the
  guard's items).
- No seal, ledger row or `t27b` ledger edit; no spec was added to the repository.
