# JSON integers above 2^53 - 1: measured 2026-10-18

Rule and scanner: [`specs/policy/json_int53.t27`](../../specs/policy/json_int53.t27)
(module `PolicyJsonInt53`). Idea taken from morluto/rea 6.6.0, which fixed the same defect for
signed 64-bit TOML integers, Android `version_code` and plist integers.

## The defect

A JavaScript reader (`JSON.parse`, trinity's website, the t27f wasm host) turns every JSON number
into an f64. An integer above 2^53 - 1 = 9007199254740991 is rounded to a neighbour and nothing
throws. Negative control, computed by the spec's own test
`the_negative_control_9007199254740993_reads_as_9007199254740992`:

| written | read back |
|---|---|
| 9007199254740993 | 9007199254740992 |
| 9007199254740995 | 9007199254740996 |
| 0x7FF8000000000001 (f64 NaN payload) | 0x7FF8000000000000 |
| 446378386939836973 (`SUPPLY_NGONKA`) | not equal to itself after the round trip |

## The rule

A 64-bit or wider integer goes out as a decimal or 0x-hex **string**, or it carries a proof that it
is at most 2^53 - 1 (`form_of()`), and then it may stay a number. Floats are out of scope.

## Method

The scan covered what t27 tools write and what `t27c gen-js` prints, at `03c6be104`:

- every `*.json` tracked in the repo, for a bare integer run above 2^53 - 1 (digits compared as
  text, the way the spec's scanner compares them);
- `t27c gen-js` over the 57 specs that hold a 16-digit-or-longer integer literal, run on the
  Railway lab, reading every `export const` line.

## A. Committed JSON: 35 fields in 32 files, 2225 bare integers

All of these sit in `conformance/vectors/` except the last row. Every one is a bare number a JS
reader rounds. Status for all: **open, writer not fixable in this PR** (see "Writers" below).

| file | field | bare integers above 2^53 - 1 |
|---|---|---|
| binary128_conformance_v0.json | `binary128_bits_int` | 7 |
| binary256_conformance_v0.json | `binary256_bits_int` | 7 |
| binary64_conformance_v0.json | `binary64_bits_int` | 7 |
| bnf64_conformance_v0.json | `bnf64_bits_int` | 7 |
| cray_float_conformance_v0.json | `cray_float_bits_int` | 7 |
| decimal128_conformance_v0.json | `decimal128_bits_int` | 8 |
| decimal64_conformance_v0.json | `decimal64_bits_int` | 7 |
| double_double_conformance_v0.json | `double_double_bits_int` | 7 |
| gf1024_conformance_v0.json | `bias` | 1 |
| gf1024_conformance_v0.json | `bits` | 13 |
| gf128_conformance_v0.json | `bits` | 13 |
| gf256_conformance_v0.json | `bias` | 1 |
| gf256_conformance_v0.json | `bits` | 2018 |
| gf512_conformance_v0.json | `bias` | 1 |
| gf512_conformance_v0.json | `bits` | 13 |
| gf64_conformance_v0.json | `gf64_bits_int` | 7 |
| gf96_conformance_v0.json | `bits` | 13 |
| gft1024_conformance_v0.json | `bias` | 1 |
| gft128_conformance_v0.json | `bias` | 1 |
| gft256_conformance_v0.json | `bias` | 1 |
| gft512_conformance_v0.json | `bias` | 1 |
| ibm_hfp128_conformance_v0.json | `ibm_hfp128_bits_int` | 7 |
| ibm_hfp64_conformance_v0.json | `ibm_hfp64_bits_int` | 7 |
| int128_conformance_v0.json | `int128_bits_int` | 2 |
| int64_conformance_v0.json | `int64_bits_int` | 2 |
| lns64_conformance_v0.json | `lns64_bits_int` | 2 |
| ms_mbf64_conformance_v0.json | `ms_mbf64_bits_int` | 7 |
| posit64_conformance_v0.json | `posit64_bits_int` | 7 |
| quad_double_conformance_v0.json | `quad_double_bits_int` | 7 |
| takum64_conformance_v0.json | `takum64_bits_int` | 14 |
| vax_d_conformance_v0.json | `vax_d_bits_int` | 7 |
| vax_g_conformance_v0.json | `vax_g_bits_int` | 7 |
| vax_h_conformance_v0.json | `vax_h_bits_int` | 7 |
| x87_fp80_conformance_v0.json | `x87_fp80_bits_int` | 7 |
| conformance/tmem_bridge.json | `id` | 1 (the vector id 9007199254740992 = 2^53) |

The `tmem_bridge.json` row is deliberate: it is the boundary vector next to `id = 9007199254740991`
and `id = -9007199254740991`, which sit exactly on the safe bound and pass. The bridge spec
(`specs/memory/tmem/bridge.t27`, `TMS_RPC_ID_MAX`) already states the same 2^53 - 1 limit, so that
file is the in-repo precedent for this rule; the 2^53 vector is its refusal test, and the scanner
flags it by design (an allowlist entry, not a defect).

`gf256_conformance_v0.json` `bits` is 2018 of the 2225: one field of one file.

Written by hand-written Python (`conformance/vectors/gen_all_formats.py`,
`conformance/gf_wide_independent_witness.py`, `conformance/gen_structural_instances.py` and the
`gen_*.py` siblings in `conformance/vectors/`). Python emits `int` as an arbitrary-precision JSON
number, so the file is exact and the reader is where the value is lost.

## B. JSON that is already safe

| document | why |
|---|---|
| `.trinity/seals/*.json` | hashes are hex strings; counts are small numbers |
| `t27c test-report` JSON counts (tests, pass, FAIL, asserts) | counts, far below 2^53 |
| `docs/reports/t27b_expectations.json` | counts and names; no integer run above 2^53 - 1 |
| `conformance/tmem_bridge.json` ids at +/- 9007199254740991 | exactly on the bound |

## C. `t27c gen-js` output: 121 literals from 53 specs

`gen-js` prints a `u64`/`i64`/wider `export const` as a bare JS number literal (hex stays a number
literal too), so the module the host imports holds the rounded value. Spec, then constants:

| spec (under `specs/`) | constants |
|---|---|
| ternary/comb_bitnet_layer.t27 | W_P |
| tools/t27o/rust_convert.t27 | NUM_REFUSED |
| tools/t27o/rust_std.t27 | I64_MAX |
| tri/crypto/hex.t27 | NONE |
| tri/crypto/sha512.t27 | ALL |
| tri/network/payout.t27 | SPLIT_MAX, NOT_A_NUMBER, ACCOUNT_MAX, HELD_BIT, POOL_ID, ID_ACCOUNT_MAX, NO_EPOCH, ALL |
| tri/utils/args.t27 | NONE |
| tri/utils/template.t27 | NONE |
| tri/utils/random.t27 | XORSHIFT_MULTIPLIER, DEFAULT_SEED |
| tri/t27b/comptime_float.t27 | TOP64 |
| tri/t27b/source_text.t27 | I128_MAX, U64_MAX |
| tri/t27b/libm.t27 | qnan64, inf64, ones64, negzero64 (frac64 = 2^52 - 1 is safe) |
| tri/t27b/i128_plan.t27 | max64, sign64, two64, MAX128 |
| tri/t27b/type_text.t27 | LEN_NONE |
| tri/t27b/fuzz.t27 | ALL, GAMMA, MIX1, MIX2, CASE_MIX |
| tri/t27b/verilog_plan.t27 | U64_MAX |
| tri/t27b/blockers.t27 | FNV_OFFSET |
| tri/t27b/conformance/i128_arith.t27 | I128_MAX, U64_MAX, TWO_64 |
| tri/t27b/conformance/wide_int_const.t27 | WC_HALF, WC_FULL, WC_TOP, WC_MIN, WC_MAX, WC_HEX, WC_256, WC_256_TWICE |
| tri/t27b/conformance/i128_ops.t27 | I128_MAX, TWO_64, U64_MAX |
| tri/t27b/conformance/numeric_coercion.t27 | NC_TWO_60 |
| tri/misread/shapes.t27 | NO_FIELD |
| memory/tmem/bridge.t27 | TMS_BRIDGE_DOT_MAX_WIDTH (RPC_ID_MAX/MIN are on the bound and safe) |
| verified/device_dna.t27 | DNA_OBSERVED, DNA_OBSERVED_B |
| verified/payout.t27 | PRICE_OVERFLOW |
| network/usage.t27 | NO_EPOCH, ACCOUNT_MAX, HELD_BIT, POOL_ID |
| network/pool.t27 | AMOUNT_MAX, PRICE_OVERFLOW |
| network/identity.t27 | ID_ACCOUNT_MAX, ALL |
| network/quorum.t27 | LAB1_KEY_ID, LAB2_KEY_ID |
| providers/tri_gnk_pair.t27 | TOTAL_SUBSIDIES_NGONKA, SUPPLY_NGONKA (TOTAL_BURNED_NGONKA is safe) |
| network/attribution.t27 | POOL_ID, HELD_BIT, ACCOUNT_MAX |
| network/mvp.t27 | HEADROOM_OVERRUN, PRICE_OVERFLOW |
| numeric/gft128.t27 | EXP_OFFSET, OFFSET_MAX |
| numeric/paper_claims.t27 | E16, E18 |
| fpga/ddr3_head_bandwidth.t27 | LANE_SALT |
| fpga/rc_filter_ax7203.t27 | EXP_S |
| fpga/trinet_node.t27 | SIP_C0..SIP_C3, PRE_LEN_TOP, EVEN, M2, M4, M8 |
| benchmarks/gf16_bfloat16_nmse_run.t27 | SM_GAMMA, SM_MIX1, SM_MIX2, XO_FIRST_FROM_ZERO |
| hosting/credit.t27 | BALANCE_MAX |
| igla/coder/tokenizer.t27 | NONE |
| igla/coder/pipeline.t27 | PIPE_NONE, NONE |
| port/trinity/src/tri/gen_fuzz.t27 | RNG_ZERO_SEED_STATE |
| port/trios/crates/trios-igla-trainer/src/main.t27 | RNG_MULTIPLIER |
| port/trios/crates/trios-igla-trainer/src/jepa_runner.t27 | LCG_MULT, LCG_INCR |
| port/trios/crates/trios-cli/src/main.t27 | U64_MAX_DIV_10 |
| port/trios/crates/trios-chat/rings/CR-CHAT-02/src/epoch_rollover_wraparound_guard.t27 | EPRW_U64_MAX |
| port/trios/crates/trios-chat/rings/CR-CHAT-00/src/lib.t27 | CHAT_U64_MAX |
| port/conformance/witness/gf48_fp64/gf_decode_param_fp64.t27 | FP64_QNAN, FP64_POS_INF, FP64_NEG_INF |
| effects/clock_cap.t27 | CLOCK_NS_MAX |
| compiler/core/t27core.t27 | W_CONTINUE, W_INVARIA, W_UNDEFINE, W_DEFAULT_, W_VALID_IN, W_ASSERT_E, W_T27_ASSE, W_ENDMODUL |
| queen/actors_hw.t27 | HW_TAG_BITS |
| queen/actor_view.t27 | T_JOB, T_REVIEW |
| queen/actors_hw_jtag.t27 | LCG_MUL, LCG_ADD, EXPECTED_SIG |

Safe and excluded from the 121: `W_Z`, `_Z` (6004799503160661), `TS_FOLD_MAX` (2^52),
`TMS_RPC_ID_MAX`, `TMS_RPC_ID_MIN`, `TOTAL_BURNED_NGONKA`, `t27b_libm_frac64`.

Most of the 121 are sentinels (`u64::MAX`, `NONE`), PRNG and hash multipliers, and the 8-byte
`t27core` keyword words. They matter to a JS host only when the host reads the value and compares
or prints it: a hash seed rounded in JS no longer reproduces the t27 test's expected value. The
one that carries money-shaped meaning is `providers/tri_gnk_pair.t27`: `SUPPLY_NGONKA =
446378386939836973` is rounded by `gen-js` to a different integer.

## D. Unknown (not measurable from this repository)

| surface | status |
|---|---|
| t27-mcp tool results (`t27mcp`) | unknown: the server lives outside this tree; no integer field of that wire format is above 2^53 in anything checked in here, but the live responses were not sampled |
| trinity-side consumers of the conformance vectors and of `gen-js` modules | unknown: other repository (gHashTag/trinity) |
| corpus receipts written to Railway volumes | unknown: not in the tree, not read in this run |
| `t27c gen-json` IGLA board events | unknown: values go through the `gen-js` layer (`run_gen_json`), so any `u64` field above 2^53 - 1 in a board event inherits section C; no such event was captured |

## Writers

None of the section A or C writers can be fixed in this PR under the only-t27 rule:

- Section A: hand-written Python. The fix is a `.t27` generator that emits the vectors with
  `*_bits_int`, `bits` and `bias` as hex or decimal strings, then deleting the Python.
- Section C: `bootstrap/src/codegen_js.rs` and `codegen_ts.rs`. The fix is `gen-js` and `gen-ts`
  printing a constant above 2^53 - 1 as a `BigInt` literal (`123n`) or a string; editing existing
  Rust needs the `owner-approved-foreign` label.

Both are filed as follow-ups in the linked issue. This PR lands the rule, its scanner with tests and
negative control, and this list.
