# std::mem callers: the equality and find copies in specs/ (2026-10-25)

Epic gHashTag/t27#12444, row B (gHashTag/t27#12446). This is the census that fixed the signature of
`specs/std/mem.t27`. No caller is migrated by the PR that adds the library; migration is batches of at most
10 specs, each with equal `t27c test-report` counts before and after.

## How it was measured

Every `fn` in `specs/` whose parameter list is exactly two text values (`[]const u8`, `string` or `str`)
and whose result is `bool`: 216 helpers. The full list, one `file:line: signature` per line, is the
appendix below. The ratchet (`python3 tools/dupe_scan.py --name same_text`) cannot join them, because the
group key is the body text including parameter names, and the authors named the parameters `a`/`b`,
`s`/`t`, `hay`/`needle`, `name`/`prefix`, `src`/`w` and so on.

## By name (top of the tail)

| name | copies | std::mem function |
|---|---|---|
| same_text | 12 | bytes_eq |
| contains | 11 | bytes_contains |
| str_eq | 10 | bytes_eq |
| starts_with | 9 | bytes_starts_with |
| ends_with | 8 | bytes_ends_with |
| same | 5 | bytes_eq |
| word_is, strings_equal, contains_substring, bytes_eq | 4 each | bytes_eq / bytes_contains |
| str_contains, eq_s | 3 each | bytes_contains / bytes_eq |
| starts_s, same_span, same_bytes, mem_eql | 2 each | bytes_starts_with / bytes_eq |

About 40 more names carry one copy each. By name pattern (not exclusive: `has_prefix` matches both the
prefix and the has pattern) 67 names say equality (`eq`, `same`, `equal`, `eql`), 17 say prefix, 13 say
suffix and 48 say contains / has / find. Some of those are domain tests (`has_word`, `listed`, `ends_ci`)
that look like find but compare case-insensitively or on token boundaries; they stay put, and each batch
reads the body before it moves a name.

## By signature (decides what can move without a cast)

| parameter types | copies |
|---|---|
| `[]const u8, []const u8` | 125 |
| `string, string` | 58 |
| `str, str` | 24 |
| `[]const u8, string` / `string` / `str` mixed | 9 |

Of the 125 slice pairs, 44 are written exactly `(a: []const u8, b: []const u8)`. Those 44 are one ratchet
group on equality and are the cheapest first batch: no cast, no rename.

The 82 `string` / `str` copies need one check before they move: whether `string` and `str` lower to the
same slice type as `[]const u8` in each backend the importing spec feeds. Do not assume it; run the
importer's `t27c test-report` before and after, and keep the copy where the counts differ.

## By directory

| directory | copies |
|---|---|
| specs/port | 51 |
| specs/widgets | 42 |
| specs/igla | 29 |
| specs/tri | 26 |
| specs/t27os | 22 |
| specs/verified, specs/hosting, specs/tools | 17 together |
| all other directories | 29 |

`specs/port` and `specs/widgets` hold 93 of 216 (43 percent): two batches of directory cover the head.

## Design decisions this census forced

1. The library takes `[]const u8` and uses `.len`. The t27os `(s, n)` pairs are the exception (22 copies) and
   a caller with an explicit length can pass its slice when `n == s.len`.
2. Parameter names are not `a` and `b`. `bytes_eq(x, y)` is a library body, and the 44 local copies are
   `(a, b)`; if the library used the same names its body would join their ratchet group and count as a
   clone instead of the home.
3. Every name has the `bytes_` prefix. `starts_with`, `ends_with` and `contains` exist in this tree with
   other signatures; the `use` splice refuses two different bodies under one bare name (R4), and a prefixed
   name cannot collide with them.
4. Lookups answer `i64`, `-1` when absent. Equality and prefix tests answer `bool`.
5. No dependency on std::text, no module constants, no slicing (a caller migrating must not need a cast).
6. Not for Verilog-bound specs until gHashTag/t27#8769 closes (`gen-verilog` does not resolve `use`).

## Migration order (batches of at most 10, equal counts before and after)

1. The 44 exact `(a: []const u8, b: []const u8)` copies, outside Verilog-bound and tracked-generated specs.
2. The remaining `[]const u8` slice pairs, directory by directory (`specs/port`, `specs/widgets`, then the
   rest).
3. The `string` / `str` pairs, after the type check above.
4. The t27os `(s, n)` pairs, passing the slice when the length is the slice length.

A migration that drops a test count is a revert, not a finding. A callee with the same bare name in the
importer's closure but a different body blocks the importer (R4): keep the local copy and say so in the
commit, as `specs/t27os/share.t27` does for `is_hex`.

## Appendix: the 216 candidates (file:line: signature)

```
specs/ar/asp_solver.t27:38:    fn str_eq(a: string, b: string)
specs/ar/coa_planning.t27:182:    fn str_eq(a: string, b: string)
specs/ar/datalog_engine.t27:38:    fn str_eq(a: string, b: string)
specs/ar/proof_trace.t27:126:    fn str_eq(a: string, b: string)
specs/automation/ball-board.t27:304:    fn may_link(kind: str, value: str)
specs/automation/ball-board.t27:316:    fn title_claims(title_head: str, value: str)
specs/automation/ball-grants.t27:82:    fn grant_covers(grant_client: str, card_client: str)
specs/automation/blog-share.t27:118:    fn left_the_form(before: str, after: str)
specs/automation/meta-connect.t27:135:    fn retry_after_relink(old_id: str, new_id: str)
specs/claw/recipes.t27:110:pub fn str_eq(a: str, b: str)
specs/claw/t27f_gate_pins.t27:19:fn same_as_lit(a: []const u8, b: []const u8)
specs/compiler/use_splice.t27:136:pub fn same_decl(a: []const u8, b: []const u8)
specs/crons/t27os-night-loop.t27:435:pub fn str_differs(a: str, b: str)
specs/file/operations.t27:334:    fn match_simple_pattern(path: str, pattern: str)
specs/hosting/panel.t27:103:pub fn same_word(a: string, b: string)
specs/hosting/panel.t27:248:pub fn word_is(word: string, name: string)
specs/hosting/panel.t27:269:pub fn names_key(first: string, second: string)
specs/hosting/panel.t27:276:pub fn names_host_id(first: string, second: string)
specs/hosting/toolchain.t27:172:pub fn ends_with(text: string, tail: string)
specs/hosting/toolchain.t27:183:pub fn starts_with(text: string, head: string)
specs/igla/coder/bench_proxy.t27:332:fn str_eq(a: string, b: string)
specs/igla/coder/bench_proxy.t27:98:fn str_contains(hay: string, needle: string)
specs/igla/coder/eval.t27:1348:pub fn run_icarus_sim(rtl: string, testbench: string)
specs/igla/coder/eval.t27:1440:pub fn run_icarus_cli(verilog_file: string, testbench_file: string)
specs/igla/coder/eval.t27:1453:pub fn verify_rtl_with_full_toolchain(rtl: string, template_name: string)
specs/igla/coder/eval.t27:1550:pub fn run_icarus_real(verilog_file: string, testbench: string)
specs/igla/coder/eval.t27:1584:pub fn synthesize_to_bitstream(verilog_file: string, board_name: string)
specs/igla/coder/eval.t27:1593:pub fn upload_to_fpga(bitstream_file: string, board_name: string)
specs/igla/coder/eval.t27:2133:fn starts_with(s: string, prefix: string)
specs/igla/coder/eval.t27:2141:pub fn cross_verify_verilog_python(verilog: string, python: string)
specs/igla/coder/eval.t27:2177:pub fn cross_verify_with_cocotb(verilog: string, python_tb: string)
specs/igla/coder/eval.t27:310:pub fn write_verilog_file(code: string, path: string)
specs/igla/coder/eval.t27:697:pub fn verify_rtl_with_testbench(rtl: string, template_name: string)
specs/igla/coder/pipeline.t27:234:fn pl_has(s: string, needle: string)
specs/igla/coder/pipeline.t27:253:fn pl_same(a: string, b: string)
specs/igla/coder/prm.t27:210:pub fn contains_substring(haystack: string, needle: string)
specs/igla/coder/prm.t27:511:pub fn check_syntax(code: string, lang: string)
specs/igla/coder/prm.t27:556:pub fn check_sacred_compliance(code: string, lang: string)
specs/igla/coder/tokenizer.t27:469:fn same_bytes(a: []const u8, b: []const u8)
specs/igla/coder/weights.t27:87:fn has_suffix(s: string, suffix: string)
specs/igla/coder/weights.t27:95:fn text_equal(a: string, b: string)
specs/igla/evaluation/multi_lang_harness.t27:55:    fn has(code: []const u8, pattern: []const u8)
specs/igla/own_model_board.t27:105:pub fn words_match(s: str, words: str)
specs/igla/race/backend.t27:239:fn contains_substring(haystack: string, needle: string)
specs/igla/race/eda.t27:51:fn strings_equal(a: string, b: string)
specs/igla/race/eda.t27:93:fn contains_substring(haystack: string, needle: string)
specs/igla/race/formal.t27:117:fn contains_substring(haystack: string, needle: string)
specs/igla/race/formal.t27:175:fn strings_equal(a: string, b: string)
specs/igla/race/yosys.t27:33:fn strings_equal(a: string, b: string)
specs/lab/plan.t27:471:fn lab_test_fn_row(pat: string, want: string)
specs/lab/plan.t27:488:fn lab_test_watch_row(watch: string, want: string)
specs/lab/plan.t27:508:fn lab_test_heads(ls: string, want: string)
specs/lab/verdict.t27:435:fn lab_test_red(rows: string, want: string)
specs/network/job_rules.t27:181:pub fn results_agree(a: string, b: string)
specs/pins/parser.t27:821:    fn bytes_eq(a: []const u8, b: []const u8)
specs/pins/parser.t27:836:    fn bytes_contains(haystack: []const u8, needle: []const u8)
specs/port/bootstrap/src/compiler/return_unsized.t27:101:pub fn mentions_type(hay: str, name: str)
specs/port/bootstrap/src/compiler/return_unsized.t27:636:fn str_eq(a: str, b: str)
specs/port/bootstrap/src/compiler/return_unsized.t27:669:fn starts_with(s: str, prefix: str)
specs/port/bootstrap/src/compiler/return_unsized.t27:676:fn ends_with(s: str, suffix: str)
specs/port/bootstrap/src/enrichment/mod.t27:8:pub fn check_enrich_script_exists(notebooklm_dir: []const u8, script_name: []const u8)
specs/port/browseros/trios/agent-server/apps/server/src/lib/db/retry.t27:55:fn contains(haystack: str, needle: str)
specs/port/browseros/trios/agent-server/apps/server/src/lib/provider-error-classifier.t27:72:fn contains_ignore_case(haystack: []const u8, needle: []const u8)
specs/port/browseros/trios/agent-server/apps/server/src/lib/vm/lima-config.t27:73:fn contains(haystack: []const u8, needle: []const u8)
specs/port/scripts/check-conflicts.t27:66:fn is_branch_merged(branch: []const u8, target: []const u8)
specs/port/scripts/check_first_party_doc_language.t27:17:fn strings_equal(a: []const u8, b: []const u8)
specs/port/scripts/ci/loop-tools-tracked.t27:125:pub fn is_resolution(cmd: []const u8, target: []const u8)
specs/port/scripts/gen_w309.t27:35:pub fn stripped_starts_with(line: []const u8, prefix: []const u8)
specs/port/scripts/gen_w309.t27:85:fn bytes_contain(haystack: []const u8, needle: []const u8)
specs/port/scripts/gen_w312.t27:21:fn slices_equal(a: []const u8, b: []const u8)
specs/port/scripts/gen_w312.t27:4:fn line_starts_with(line: []const u8, prefix: []const u8)
specs/port/scripts/gen_w374.t27:230:fn mem_eql(a: []const u8, b: []const u8)
specs/port/scripts/gen_w384_lean.t27:48:    fn contains(hay: []const u8, needle: []const u8)
specs/port/scripts/gen_w392.t27:119:fn mem_eql(a: []const u8, b: []const u8)
specs/port/scripts/tri_loop/claims.t27:47:fn contains_exact(s: []const u8, substr: []const u8)
specs/port/scripts/tri_loop/claims.t27:74:fn starts_with_exact(s: []const u8, prefix: []const u8)
specs/port/tools/builtin_parity_table.t27:48:pub fn declares(out: []const u8, name: []const u8)
specs/port/tools/dedupe_identical_tests.t27:181:pub fn lines_equal(a: []const u8, b: []const u8)
specs/port/tools/published_figures.t27:19:pub fn starts_with(s: []const u8, prefix: []const u8)
specs/port/tools/published_figures.t27:26:pub fn ends_with(s: []const u8, suffix: []const u8)
specs/port/tools/queen/feed_untested.t27:44:pub fn validate_shell_counts(fn_now: str, test_now: str)
specs/port/tools/queen/feed_untested.t27:67:pub fn signature_present(text: str, name: str)
specs/port/tools/trinity_fpga_adapter.t27:71:fn starts_with(s: str, prefix: str)
specs/port/tools/verify_emit_bitexact.t27:13:pub fn _check_files_equal(file1: []const u8, file2: []const u8)
specs/port/trinity/src/cli/railway_deployment_create.t27:32:pub fn contains(haystack: []const u8, needle: []const u8)
specs/port/trinity/src/vibeec/bogatyrs_spec_structure.t27:131:fn eql(a: []const u8, b: []const u8)
specs/port/trinity/src/vibeec/soul_collector.t27:31:pub fn soul_collector_ends_with(path: []const u8, suffix: []const u8)
specs/port/trinity/src/vibeec/soul_collector.t27:45:pub fn soul_collector_contains(path: []const u8, marker: []const u8)
specs/port/trinity/src/vibeec/spec_loader.t27:53:pub fn ends_with(path: []const u8, suffix: []const u8)
specs/port/trinity/src/vibeec/validation_engine.t27:28:fn contains(code: []const u8, needle: []const u8)
specs/port/trios/apps/trios-macos/rings/RUST-03/clade-rollback/src/main.t27:36:fn starts_with(name: []const u8, prefix: []const u8)
specs/port/trios/apps/trios-macos/rings/RUST-03/clade-rollback/src/main.t27:50:fn ends_with(name: []const u8, suffix: []const u8)
specs/port/trios/apps/trios-macos/rings/RUST-13/clade-meshd/src/key_store.t27:293:fn env_name_is(value: []const u8, name: []const u8)
specs/port/trios/apps/trios-macos/rings/RUST-13/trios-mesh/tools/ad9361_config.t27:48:pub fn ssh(ip: []const u8, cmd: []const u8)
specs/port/trios/crates/tri-cli/src/main.t27:106:pub fn contains(s: []const u8, needle: []const u8)
specs/port/trios/crates/trios-bridge/src/github.t27:123:fn starts_with(s: []const u8, prefix: []const u8)
specs/port/trios/crates/trios-chat/common/helpers.t27:55:pub fn bytes_equal(a: []const u8, b: []const u8)
specs/port/trios/crates/trios-chat/rings/CR-CHAT-02/src/dh_output_validation.t27:64:fn bytes_equal(a: []const u8, b: []const u8)
specs/port/trios/crates/trios-chat/rings/CR-CHAT-06/src/agent_context_window_budget_guard.t27:54:fn ids_equal(a: []const u8, b: []const u8)
specs/port/trios/crates/trios-cli/src/cmd/submit.t27:48:fn ends_with(s: []const u8, suffix: []const u8)
specs/port/trios/crates/trios-doctor/src/doctor_loop.t27:177:pub fn order_targets_us(target_agent: str, cfg_agent_id: str)
specs/port/trios/crates/trios-igla-race/src/lessons.t27:80:pub fn lesson_is_more_severe(a: str, b: str)
specs/port/trios/crates/trios-phd/src/bin/migrate_rag.t27:128:fn contains(hay: str, needle: str)
specs/port/trios/crates/trios-rainbow-bridge/src/bridge.t27:90:pub fn channel_matches_payload(declared: str, implied: str)
specs/port/trios/crates/trios-scarab-types/rings/SR-04/src/lib.t27:102:fn str_eq(a: []const u8, b: []const u8)
specs/port/trios/crates/trios-train-cpu/src/bin/main.t27:105:fn starts_with(s: []const u8, prefix: []const u8)
specs/port/trios/crates/trios-train-cpu/src/bin/main.t27:90:fn str_eq(a: []const u8, b: []const u8)
specs/queen/review_log.t27:35:fn str_starts_with(haystack: string, needle: string)
specs/sandbox/https_enforce.t27:51:fn bytes_eq(a: []const u8, b: []const u8)
specs/sandbox/https_enforce.t27:67:fn starts_with(s: []const u8, prefix: []const u8)
specs/server/project.t27:169:    fn write_file(path: str, content: str)
specs/t27f/petal_issues_viz.t27:351:pub fn piv_vocab_has(vocab: str, name: str)
specs/t27mcp/service.t27:72:pub fn has_prefix(name: string, prefix: string)
specs/t27os/bench.t27:837:pub fn ends_with(t: string, suffix: string)
specs/t27os/devices.t27:1279:fn dv_has(t: []const u8, w: string)
specs/t27os/editors.t27:181:pub fn same_bytes(a: []const u8, b: []const u8)
specs/t27os/identity.t27:395:fn same(a: []const u8, b: []const u8)
specs/t27os/kanban.t27:881:fn list_has_word(t: string, w: string)
specs/t27os/keys.t27:1415:pub fn has_bytes(hay: []const u8, needle: []const u8)
specs/t27os/keys.t27:1421:pub fn has_code(hay: []const u8, code: []const u8)
specs/t27os/keys.t27:1457:pub fn begins_with(hay: []const u8, needle: []const u8)
specs/t27os/keysync.t27:163:pub fn contains(hay: []const u8, needle: []const u8)
specs/t27os/keysync.t27:198:pub fn doc_leaks(doc: []const u8, plain: []const u8)
specs/t27os/keysync.t27:236:pub fn fp_matches(digest: []const u8, fp: []const u8)
specs/t27os/keysync_host.t27:258:fn kc_put_hex(account: []const u8, priv: []const u8)
specs/t27os/lab_inputs.t27:68:pub fn specs_step_ok(env: []const u8, body: []const u8)
specs/t27os/lab_inputs.t27:74:pub fn sha_step_ok(env: []const u8, body: []const u8)
specs/t27os/lanes_view.t27:151:pub fn lv_has(text: []const u8, w: string)
specs/t27os/orbit_view.t27:511:fn ov_same(a: []const u8, b: string)
specs/t27os/provider_router.t27:27:pub fn pr_has(hay: []const u8, nd: []const u8)
specs/t27os/provider_router.t27:77:pub fn pr_in_chain(chain: []const u8, id: []const u8)
specs/t27os/pty_logic.t27:2413:fn word_is(w: []const u8, k: []const u8)
specs/t27os/pty_logic.t27:2511:pub fn has_text(hay: []const u8, needle: []const u8)
specs/t27os/queen_board.t27:166:fn has_sub(t: string, p: string)
specs/t27os/share.t27:234:fn contains_ci(name: []const u8, lit: []const u8)
specs/test_framework/runner.t27:496:fn str_contains(haystack: string, needle: string)
specs/test_framework/runner.t27:522:fn str_eq(a: string, b: string)
specs/tools/t27o/install.t27:84:pub fn text_has(hay: []const u8, needle: []const u8)
specs/tools/t27o/rust_convert.t27:3526:pub fn listed(w: str, t: []const u8)
specs/tools/t27o/rust_convert.t27:5350:pub fn has_word(src: []const u8, w: []const u8)
specs/tools/t27o/rust_std.t27:125:pub fn str_contains(hay: []const u8, needle: []const u8)
specs/tools/t27o/spice_convert.t27:407:pub fn ends_ci(name: []const u8, suffix: []const u8)
specs/tri/collections/array.t27:77:    fn equal(a: []const u8, b: []const u8)
specs/tri/collections/variant.t27:20:fn tag_equals(a: []const u8, b: []const u8)
specs/tri/crypto/hex.t27:98:    fn same(a: []const u8, b: []const u8)
specs/tri/io/filesystem.t27:127:    fn has_ext(path: []const u8, suffix: []const u8)
specs/tri/io/filesystem.t27:197:    fn norm_eq(path: []const u8, expected: []const u8)
specs/tri/mutate/plant.t27:116:pub fn same_text(a: string, b: string)
specs/tri/net/http.t27:275:    fn bytes_eq(a: []const u8, b: []const u8)
specs/tri/network/payout.t27:440:pub fn gh_read(t: string, sha: string)
specs/tri/pr/preflight.t27:251:pub fn changes_under(c: string, lit: string)
specs/tri/seals/merge_driver.t27:76:pub fn same_spec(a: string, b: string)
specs/tri/search/pattern.t27:113:    fn wildcard_match(pattern: []const u8, text: []const u8)
specs/tri/search/pattern.t27:23:    fn glob_match(pattern: []const u8, text: []const u8)
specs/tri/t27b/ast_walk.t27:103:pub fn word_in(text: []const u8, name: []const u8)
specs/tri/t27b/conformance/std_mem_byte_slice.t27:24:fn same(a: []const u8, b: []const u8)
specs/tri/t27b/conformance/stdmem.t27:24:fn smc_has_key(line: []const u8, key: []const u8)
specs/tri/t27b/lazy_sig_plan.t27:143:fn listed(name: []const u8, list: []const u8)
specs/tri/t27b/lazy_sig_plan.t27:65:fn begins(t: []const u8, word: []const u8)
specs/tri/t27b/literal_plan.t27:65:pub fn ends_with(t: []const u8, w: str)
specs/tri/t27b/opaque_plan.t27:106:fn begins(s: []const u8, w: str)
specs/tri/t27b/opaque_plan.t27:112:fn instance(t: []const u8, head: str)
specs/tri/t27b/stdmem.t27:158:fn t27b_stdmem_has_key(line: str, key: str)
specs/tri/t27b/type_text.t27:165:pub fn trimmed_is(s: []const u8, w: str)
specs/tri/t27b/type_text.t27:86:pub fn same(s: []const u8, w: str)
specs/tri/t27b/wide_plan.t27:491:fn spells(text: []const u8, word: []const u8)
specs/tri/trees/trie.t27:324:    fn bytes_eq(a: []const u8, b: []const u8)
specs/tri/utils/template.t27:72:    fn same(a: []const u8, b: []const u8)
specs/verified/clock_constraints.t27:346:fn xdc_is(v: string, w: string)
specs/verified/corpus_receipt.t27:261:pub fn text_is(a: string, b: string)
specs/verified/corpus_receipt.t27:348:pub fn t27b_is(v: string, w: string)
specs/verified/corpus_receipt.t27:354:pub fn reference_is(v: string, w: string)
specs/verified/corpus_receipt.t27:359:pub fn same_reference(a: string, b: string)
specs/verified/ddc_receipt.t27:211:pub fn same_front_end(a: string, b: string)
specs/widgets/bit_autopsy.t27:1265:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/bit_diff.t27:1774:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/bit_diff.t27:422:pub fn bytes_identical(a: []const u8, b: []const u8)
specs/widgets/bit_diff_logic.t27:178:pub fn bytes_identical(a: []const u8, b: []const u8)
specs/widgets/fasm_skyline.t27:1745:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/fsm_sketch.t27:788:fn same_span(a: []const u8, b: []const u8)
specs/widgets/fsm_sketch_logic.t27:901:pub fn has_sub(s: []const u8, w: []const u8)
specs/widgets/gatle.t27:2133:fn g_has(hay: []const u8, nd: []const u8)
specs/widgets/gf16_calc.t27:1324:fn eq_s(a: []const u8, b: []const u8)
specs/widgets/gf16_calc.t27:2348:fn contains(s: []const u8, w: []const u8)
specs/widgets/idcode.t27:1180:fn contains_ci(hay: []const u8, nee: []const u8)
specs/widgets/idcode.t27:1876:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/lut_depth.t27:1835:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/lut_explain.t27:2131:pub fn eq_s(a: []const u8, b: []const u8)
specs/widgets/lut_explain.t27:2141:pub fn starts_s(a: []const u8, p: []const u8)
specs/widgets/lut_explain.t27:2151:pub fn ends_s(a: []const u8, p: []const u8)
specs/widgets/lut_explain.t27:2167:fn eq_up(s: []const u8, w: []const u8)
specs/widgets/lut_explain.t27:3734:fn eq_low(s: []const u8, w: []const u8)
specs/widgets/lut_explain.t27:4548:fn contains(s: []const u8, w: []const u8)
specs/widgets/lut_treemap.t27:2434:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/lut_treemap.t27:814:pub fn lit_at(s: []const u8, w: []const u8)
specs/widgets/lut_treemap_logic.t27:594:pub fn lit_at(s: []const u8, w: []const u8)
specs/widgets/page_reader.t27:171:pub fn has_meta(html: []const u8, key: []const u8)
specs/widgets/page_reader.t27:291:pub fn view_listed(src: []const u8, view: []const u8)
specs/widgets/pin_map.t27:923:fn same_span(a: []const u8, b: []const u8)
specs/widgets/pin_map_logic.t27:338:pub fn glob_match(pat: []const u8, s: []const u8)
specs/widgets/pin_map_logic.t27:371:pub fn covers(pattern: []const u8, port: []const u8)
specs/widgets/placement_map.t27:418:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/placement_map.t27:444:fn starts(t: []const u8, w: []const u8)
specs/widgets/resource_bars.t27:1665:fn contains(s: []const u8, w: []const u8)
specs/widgets/resource_bars.t27:604:fn eq_s(a: []const u8, b: []const u8)
specs/widgets/resource_bars.t27:614:fn starts_s(a: []const u8, p: []const u8)
specs/widgets/sha256_hex.t27:135:fn sh_same(o: []const u8, want: []const u8)
specs/widgets/slack_waterfall.t27:1781:fn word_is(key: []const u8, want: []const u8)
specs/widgets/slack_waterfall.t27:2109:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/stat_card.t27:1833:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/t27c_stages.t27:1631:fn t_has(hay: []const u8, needle: []const u8)
specs/widgets/test_waves.t27:3566:pub fn same_text(a: []const u8, b: []const u8)
specs/widgets/test_waves.t27:496:fn word_is(key: []const u8, want: []const u8)
specs/widgets/test_waves.t27:612:fn tk_is(d: []const u8, lit: []const u8)
specs/widgets/text_meas.t27:371:fn m_same(a: []const u8, b: []const u8)
specs/widgets/vcd_wrapped.t27:1932:pub fn same_text(a: []const u8, b: []const u8)
```
