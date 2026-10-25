# Mutation census of specs/analog/raw_writer.t27, 2026-10-25

Issue #12273 (batch 1 of 2, groups A to D) and issue #12274 (batch 2 of 2, groups E to J, the two dead lines), part of
epic #10486 (weak spots of the analog v0.1 release). The two lanes wrote the missing asserts and the accepted file. The
survivor gate was not run on this tree and the seal was not re-saved. Where each number comes from:

- **Before** (526 mutants, 412 killed, 83 survived, 31 hung, 0 unviable) is the analog-loop census of the unmodified spec,
  10:39-10:55 +07, exit 0, 15 min 50 s, `tri mutate spec` 3 mutants at a time with zig `-j2` each, t27c 0.5.2 (the
  census text does not record the build commit), on the tree at commit 035df5ce60. The spec at the base of this lane
  (707240639f) is byte-identical to that tree's (`git diff 035df5ce60 707240639f -- specs/analog/raw_writer.t27` is
  empty), so the census line numbers hold. The classes of the 83 survivors are from reading the spec and its helpers;
  no mutant was re-run in this lane.
- **After** is a forecast, not a measurement: if the 62 asserts below (30 of batch 1, 32 of batch 2) kill what they
  target, 526 mutants, 474 killed, 21 survived (19 equivalent + 2 dead lines), 31 hung, 0 unviable, and 52 accepted
  entries alive (21 survivors + 31 hung), 0 accepted lines now killed. The first gate run is what turns this into a
  number; an unaccepted survivor on one of the 62 lines in the tables below would mean that assert does not kill its
  mutant. Batch 2 did one thing the gate does not: it ran each of its 32 mutants by hand on a scratch copy of the spec
  (section "The 32 survivors closed by batch 2"), and all 32 failed a test; batch 1's 30 were not run this way.
- **Seal**: `.trinity/seals/analog_AnalogRawWriter.json` was not touched, so it is stale after the spec edit until the
  orchestrator re-saves it with a master-built t27c (importers of the module may need the same).

Kinds in the census (before): drop-guard 44, flip-cmp 148, swap-logic 32, drop-step 80, swap-arith 187, ret-default 35.

## The 83 survivors, and what became of each

| Lines | Mutant | Verdict | Outcome |
| --- | --- | --- | --- |
| 79 | `y < 0.0` to `<=` in raw_parts | equivalent: raw_sig hands it a nonzero value, and at 0.0 the branch computes 0.0 - 0.0 | accepted |
| 157, 163, 167 | `e < 0`, `left >= chunk`, `left > 0` flipped in raw_expand | equivalent: at the edge the multiplier is 1 or exactly `big`, the limbs are the same | accepted |
| 228, 240 | `p.e < 0` and `i < RAW_SIG` flipped in raw_sig | equivalent: adds 0; the extra pass writes d[16] only when it is never read | accepted |
| 297, 333, 338 | `x < 0.0` to `<=` in raw_put_f64 (297) and raw_to_bits (333, 338) | equivalent: x is an infinity or nonzero there | accepted |
| 347 | `be >= 1` to `> 1` in raw_to_bits | equivalent: at be = 1 both paths build the same word | accepted |
| 645, 665 | `i < to` to `<=` for the minus sign and the exponent sign in raw_value_ok | equivalent: no digit follows either way | accepted |
| 683 | `at >= n` to `> n` in raw_check_ascii | equivalent: raw_eol returns n without reading, line 685 gives the same code | accepted |
| 690 | `t < e` to `<=` in raw_check_ascii | equivalent: buf[e] is a line feed, line 691 sees t >= e either way | accepted |
| 719, 720, 722, 723 | `t1 < e`, `t1 >= e`, `t2 < e`, `t2 >= e` flipped in raw_check_vars | equivalent: the next test returns RAW_NONE for the extra position too | accepted |
| 917 | `j < RLC_NVARS` to `<=` in raw_cell_trace | equivalent: the extra write is overwritten by the first step | accepted |
| 867 x3, 873 x2, 876 | raw_nibble and raw_at_hex (group A) | test gap | 6 killed by the new tests |
| 715 x2, 717 x2, 720 x2, 723 x7, 726 | raw_check_vars (group B) | test gap | 14 killed by the new test |
| 301, 355 | raw_put_f64 of zero, raw_from_bits of an infinity (group C) | test gap | 2 killed by the new test |
| 624 x2, 628, 629, 630, 636, 638 x2 | raw_uint and raw_count_line (group D) | test gap | 8 killed by the new test |
| 685 x2, 688, 691 x2, 701 x2 | raw_check_ascii (group E) | test gap | 7 killed by the new tests of 12274 |
| 387 flip, 609, 615, 623, 629 flip, 646, 654 x2, 657, 663, 665 swap-logic | reader helpers (group F) | test gap | 11 killed by the new tests of 12274 |
| 230, 233 | raw_sig reads a stale digit (group G) | test gap | 2 killed by the new test of 12274 |
| 938 | scale of the RLC current column (group H) | test gap | 1 killed by the new test of 12274 |
| 258 x2 | carry-out of sixteen 9s (group I) | test gap | 2 killed by the new test of 12274 |
| 140, 270, 387 drop, 543 x2, 560, 571, 715 flip, 873 drop | exact-size buffers (group J) | test gap, because the test build traps | 9 killed by the new tests of 12274 |
| 683 drop, 954 | `if (at >= n) { return RAW_E_SHORT; }` in raw_check_ascii; `if (s != RLC_EXP_S) { return 0; }` in raw_rlc_file | dead line | kept, accepted (decision below) |

Totals: 19 equivalent (accepted) + 62 test gap (30 killed by batch 1: A 6, B 14, C 2, D 8; 32 killed by batch 2: E 7,
F 11, G 2, H 1, I 2, J 9) + 2 dead line (kept, accepted) = 83. The 31 hung mutants are 30 `x = x + 1` turned into `x = x` in a loop
counter or cursor (lines 136, 176, 235, 242, 280, 309, 364, 368, 380, 391, 477, 500, 546, 549, 562, 609, 629, 650, 658,
668, 690, 719, 722, 725, 804, 859, 877, 888, 919, 939) and one `while (carry > 0)` turned into `>= 0` (line 139), which
cannot end; the loop never ends, so no test can go red and only the clock ends the mutant. They are accepted by
construction.

## The accepted file

`docs/analog/mutation-accepted-raw_writer.txt` has 52 entries: 19 equivalent, 2 dead line, 31 hung. An entry is one line
and kind, as `tri mutate spec --accepted` reads it (`specs/tri/mutate/survivors.t27`), and the gate exits 2 on an
accepted line whose mutants are all killed. The 62 mutants of groups A to J have no entry on purpose: they are meant to
be killed. Checked by hand: no key is listed twice; the 21 survivor entries plus the 62 closed ones are the 83 survivors
of the census; the 31 hung entries are the census's 31. SC-004 of 12274 reads "52 base entries plus one per survivor
moved to equivalent or left out"; no survivor was moved or left out, so the count is 52. A line with mutants of two
kinds has two entries (683 has the equivalent flip and the dead guard; 665 has the equivalent flip of the exponent sign
and, killed, the swap-logic of its `and`). The 84-entry file of batch 1 had 32 `pending: #12274` lines; batch 2 deleted
them and appended its tests after those of batch 1, at the end of the spec, so no accepted line number moved (the
function bodies the entries name are byte-identical to the base of this lane).

## The 30 closed survivors, and the assert that kills each

S<n> is the row of the census table (it is also the tag in the comments of the spec). "Mutant gives" is the value the
mutated function returns on the assert's input, derived by hand from the function body.

| S | Line | Mutant | Test and assert | Mutant gives |
| --- | --- | --- | --- | --- |
| 74 | 867 | raw_nibble: drop `if (c >= 65) { return c - 55; }` | raw_at_hex_checks_its_bytes: `raw_nibble(65) == 10`, `raw_nibble(70) == 15` | 17, 22 |
| 75 | 867 | `c >= 65` to `c > 65` | `raw_nibble(65) == 10` | 17 |
| 76 | 867 | `c - 55` to `c + 55` | same two asserts | 120, 125 |
| 78 | 873 | raw_at_hex: `hex.len / 2` to `hex.len * 2` | `raw_at_hex(&o, 0, "abcd")` on the exact `[2]u8` built by `raw_put_hex` | false, a match that ends at the buffer end is refused |
| 79 | 873 | `>` to `>=` in the same bound | same assert | false |
| 80 | 876 | raw_at_hex: drop the byte compare | `!raw_at_hex(&o, 0, "abce")` on the same buffer; and a_changed_last_byte_is_not_the_binary_reference: the SM binary file with its last byte changed is not `SM_H4` | true both times |
| 55 | 715 | raw_check_vars: drop `if (at >= n or buf[at] != 9) { return RAW_NONE; }` | raw_check_vars_refuses_malformed_lines, "x0" TAB "time" TAB "time" LF: `== RAW_NONE` | 13 |
| 57 | 715 | `or` to `and` in that guard | same assert | 13 |
| 58 | 717 | drop `if (e >= n) { return RAW_NONE; }` | TAB "0" TAB "time" TAB "time", no LF (n = 12): `== RAW_NONE` | 13 |
| 59 | 717 | `e >= n` to `e > n` | same assert | 13 |
| 61 | 720 | drop the index test | TAB "1" TAB "time" TAB "time" LF (index 1 for 0): `== RAW_NONE` | 13 |
| 63 | 720 | `or` to `and` | same assert | 13 |
| 65 | 723 | drop the empty-name and empty-type test | TAB "0" TAB "time" TAB LF (empty type) and TAB "0" TAB TAB "time" LF (empty name), each n = 9 | 9 both |
| 67 | 723 | first `or` to `and` | the empty-name line (the empty type is refused by the third clause anyway) | 9 |
| 68 | 723 | `t2 == t1 + 1` to `t2 == t1` | the empty-name line | 9 |
| 69 | 723 | `t2 == t1 + 1` to `t2 == t1 - 1` | the empty-name line | 9 |
| 70 | 723 | second `or` to `and` | either line | 9 |
| 71 | 723 | `e == t2 + 1` to `e == t2` | the empty-type line | 9 |
| 72 | 723 | `e == t2 + 1` to `e == t2 - 1` | the empty-type line | 9 |
| 73 | 726 | drop `if (t3 != e) { return RAW_NONE; }` | TAB "0" TAB "a" TAB "b" TAB "c" LF (a fourth field, n = 9): `== RAW_NONE` | 9 |
| 14 | 301 | raw_put_f64: `x < 0.0` to `x <= 0.0` for the sign byte | zero_is_written_unsigned_and_nonfinite_bits_read_as_zero: `raw_put_f64(&b, 0, 0.0, &a, &d) == 21` and `raw_at(&b, 0, "0.000000000000000e+00")` | 22 bytes, a '-' first |
| 18 | 355 | raw_from_bits: drop `if (ex == 2047) { return 0.0; }` | `raw_from_bits(0x7FF0000000000000) == 0.0` and `raw_from_bits(0x7FF8000000000000) == 0.0` | infinity |
| 28 | 624 | raw_uint: drop `if (i - from >= 18) { return RAW_BAD; }` | numbers_and_counts_at_their_limits: `raw_uint` of "1000000000000000000" (19 digits) `== RAW_BAD` | 10^18 |
| 29 | 624 | `>= 18` to `> 18` | same assert | 10^18 |
| 30 | 628 | drop `if (i == from) { return RAW_BAD; }` | `raw_uint(&be, 0, 0) == RAW_BAD` and `raw_uint` of "  " `== RAW_BAD` | 0 |
| 32 | 629 | `and` to `or` in the space run | `raw_uint` of "12x" `== RAW_BAD` | 12 |
| 33 | 630 | drop `if (i != to) { return RAW_BAD; }` | same "12x" assert | 12 |
| 34 | 636 | raw_count_line: drop the prefix check | `raw_count_line` of "No. Variablez: 1" LF with the label "No. Variables: " `== RAW_BAD` | 1 |
| 35 | 638 | drop `if (v > RAW_MAX) { return RAW_BAD; }` | "No. Variables: 1000000001" LF `== RAW_BAD` | 1000000001 |
| 36 | 638 | `v > RAW_MAX` to `v >= RAW_MAX` | "No. Points: 1000000000" LF `== 1000000000` | RAW_BAD |

The census rows S60, S62, S64 and S66 (equivalent flips) are deliberately not hit by any new assert: they stay accepted,
and a test that killed one of them would make the gate fail on a stale accepted line. S56 (715 flip, group J) was left to
batch 2 and is closed there.

## The 5 tests added (appended after the last original test)

`tri test` counts the runtime asserts it executes per test; these are the five new ones.

- `raw_at_hex_checks_its_bytes` (7 asserts, group A). Upper-case digits through `raw_nibble`, and an exact 2-byte buffer
  written by `raw_put_hex(&o, 0, "abcd")`: the write returns 2 and the bytes are 171 and 205 (0xAB, 0xCD), so the buffer
  holds exactly what the compare must match; `raw_at_hex` is true for "abcd" and false for "abce".
- `a_changed_last_byte_is_not_the_binary_reference` (3 asserts, group A). The negative control the binary byte-for-byte test
  lacks (its ASCII twin changes one digit; the binary one only asserts positively, so it passes against a `raw_at_hex`
  that checks nothing). It rebuilds the same binary file with `raw_write` from the SM bit patterns (799 bytes), asserts
  the last header block `SM_H4` is found at the end, adds 1 to the last byte, and asserts it is no longer found.
  The existing test is not edited: the issue and the classification suggested editing it, and a new test keeps every
  accepted line number and every existing assert as they were.
- `raw_check_vars_refuses_malformed_lines` (14 asserts, group B). Seven lines on 32-byte zero-filled buffers: the good
  line (control, returns 13) and six malformed ones that return RAW_NONE (no leading TAB, no final LF, a wrong index, an
  empty name, an empty type, a fourth field). Each length is asserted from what `raw_puts` returns.
- `zero_is_written_unsigned_and_nonfinite_bits_read_as_zero` (4 asserts, group C). Zero prints as 21 bytes with no sign;
  the bit patterns of +infinity and a quiet NaN read as 0.0. The patterns are bound to local constants first, because
  `tri test` does not count an assert on a module-constant table element.
- `numbers_and_counts_at_their_limits` (15 asserts, group D). `raw_uint` on 19 digits, an empty range, only spaces and
  "12x"; `raw_count_line` on a good line (control), a wrong label of the same length, a count over RAW_MAX and a count
  of exactly RAW_MAX.

`tri test specs/analog/raw_writer.t27 --t27c <build t27c>`, in a T27 OS pane, 12:08-12:09 +07: 23 tests pass (18
before), 0 fail, 0 vacuous passes of 23, exit 0. The five new tests execute 7, 3, 14, 4 and 15 runtime asserts (43).

## The 32 survivors closed by batch 2, and the assert that kills each

Same tags as above. "Mutant gives" is what the mutated function returns on the assert's input (by hand from the function
body; for group J it is the out-of-range access that traps). "Killed by" is the result of the hand-run described below:
the mutant was applied alone to a scratch copy of the spec with the 14 new tests, `tri test` was run on it, and the test
named here is the one that went red (every other test, old and new, stayed green in all 32 runs). The scratch copies
are not committed.

| S | Line | Mutant | Test and assert | Mutant gives | Killed by |
| --- | --- | --- | --- | --- | --- |
| 47 | 685 | raw_check_ascii: drop `if (e >= n) { return RAW_E_SHORT; }` | the_ascii_body_is_judged_line_by_line: `raw_check_ascii` of " 0" (n = 2) `== RAW_E_SHORT` | RAW_E_INDEX | the_ascii_body_is_judged_line_by_line |
| 48 | 685 | `e >= n` to `e > n` | same assert | RAW_E_INDEX | same |
| 49 | 688 | drop `if (buf[at] != 32) { return RAW_E_INDEX; }` | "x0" TAB "1.0e+00" LF LF (n = 12) `== RAW_E_INDEX` | RAW_OK | same |
| 51 | 691 | drop `if (t >= e) { return RAW_E_INDEX; }` | " 0" LF LF (n = 4) `== RAW_E_INDEX` | RAW_E_VALUE | same |
| 52 | 691 | `t >= e` to `t > e` | same assert | RAW_E_VALUE | same |
| 53 | 701 | drop `if (at >= n) { return RAW_E_SHORT; }` | " 0" TAB "1.0e+00" LF (n = 11, no empty line) `== RAW_E_SHORT` | RAW_E_SHAPE | same |
| 54 | 701 | `at >= n` to `at > n` | same assert | RAW_E_SHAPE | same |
| 20 | 387 | raw_at: `at + s.len > out.len` to `>=` | the_line_readers_stop_at_the_end_of_the_data: `raw_at` of "abc" at 0 on an exact `[3]u8` is true | false | the_line_readers_stop_at_the_end_of_the_data, and raw_at_reads_nothing_past_the_end_of_the_buffer (its `raw_at(&b, 2, "c")` is the same edge) |
| 25 | 609 | raw_eol: `i < n` to `i <= n` | `raw_eol(&a1, 2, 0) == 2` for "abz" | 3 | the_line_readers_stop_at_the_end_of_the_data |
| 26 | 615 | raw_has: drop `if (at + s.len > n) { return false; }` | `!raw_has(&a1, 2, 0, "abz")` | true | same |
| 27 | 623 | raw_uint: `i < to` to `<=` in the digit run | `raw_uint(&u0, 0, 2) == 12` for "123" | RAW_BAD | same |
| 31 | 629 | `i < to` to `<=` in the space run | `raw_uint(&u1, 0, 3) == 12` for "12  " | RAW_BAD | same |
| 38 | 646 | raw_value_ok: `and` to `or` after the "inf" test | a_number_is_judged_inside_its_range: `!raw_value_ok(&v0, 0, 3)` for "abc" | true | a_number_is_judged_inside_its_range |
| 39 | 654 | `i < to` to `<=` at the '.' | `raw_value_ok(&v1, 0, 1)` for "1." (range "1") | false | same |
| 40 | 654 | `and` to `or` at the '.' | `raw_value_ok(&v3, 0, 4)` for "1e+5" and `!raw_value_ok(&v4, 0, 3)` for "1x5" | false, and true | same |
| 41 | 657 | `i < to` to `<=` in the fraction | `raw_value_ok(&v5, 0, 3)` for "1.57" | false | same |
| 42 | 663 | `i < to` to `<=` at the 'e' | `raw_value_ok(&v2, 0, 1)` for "1e" (range "1") | false | same |
| 44 | 665 | `and` to `or` at the exponent sign | `raw_value_ok(&v6, 0, 3)` for "1e5" | false | same |
| 7 | 230 | raw_sig: `nd > RAW_SIG` to `>=` | raw_sig_ignores_digits_left_by_the_last_call: d[16] = 9, then `raw_sig` of 2^53 (bits 0x4340000000000000): `== 15`, d[0] = 9, d[15] = 2 | d[15] = 3 | raw_sig_ignores_digits_left_by_the_last_call |
| 8 | 233 | `j < nd` to `<=` in the sticky scan | d[17] = 7, then `raw_sig` of 3000000000000000.5 (bits 0x432550F7DCA70001): `== 15`, d[0] = 3, d[15] = 0 | d[15] = 1 | same |
| 10 | 258 | `ex = ex + 1` to `ex = ex` | raw_sig_carries_out_of_sixteen_nines: `raw_sig` of bits 0x3EB0C6F7A0B5ED8D `== -6` | -7 | raw_sig_carries_out_of_sixteen_nines |
| 11 | 258 | `ex = ex + 1` to `ex = ex - 1` | same assert | -8 | same |
| 82 | 938 | raw_cell_trace: `/ RLC_ZS` to `* RLC_ZS` | the_rlc_current_is_in_amps: peak of column 4 over 201 points, `0.005 < imax < 0.006` | a peak near 53 | the_rlc_current_is_in_amps |
| 2 | 140 | raw_mul: `cnt < a.len` to `<=` | a_carry_past_the_last_limb_is_dropped_not_written: `[1]u64` holding 999999999, `raw_mul(&a, 1, 2) == 2`, a[0] = 999999998 | writes a[1], traps | a_carry_past_the_last_limb_is_dropped_not_written |
| 12 | 270 | raw_put: `w < out.len` to `<=` | a_full_buffer_counts_the_byte_that_does_not_fit_and_writes_none: `raw_puts` of "abc" into `[2]u8` returns 3, b = 97, 98 | writes b[2], traps | a_full_buffer_counts_the_byte_that_does_not_fit_and_writes_none |
| 19 | 387 | raw_at: drop `if (at + s.len > out.len) { return false; }` | raw_at_reads_nothing_past_the_end_of_the_buffer: `!raw_at(&b, 2, "cd")` on an exact `[3]u8` | reads b[3], traps | raw_at_reads_nothing_past_the_end_of_the_buffer |
| 21 | 543 | raw_put_ascii: `pt * nv + v` to `pt / nv + v` in the guard | a_short_value_table_reads_as_zero_in_ascii: nv = 2, np = 2, three values: 98 bytes, the fourth cell reads "0.000000000000000e+00" | reads vals[3], traps | a_short_value_table_reads_as_zero_in_ascii |
| 22 | 543 | guard `<` to `<=` | same, and nv = 1, np = 2, one value: 52 bytes | reads vals[1] (second call), traps | same |
| 23 | 560 | raw_put_binary: `i < vals.len` to `<=` | a_short_value_table_reads_as_zero_in_binary: nv = 1, np = 2, one value: 16 bytes, the second 8 are zero | reads vals[1], traps | a_short_value_table_reads_as_zero_in_binary |
| 24 | 571 | raw_write: `nv > 0` to `>= 0` | a_file_with_no_variables_is_written_and_read_back: empty `vars`, 91 bytes, `raw_check` gives RAW_OK, nv 0, np 0 | divides by 0, traps | a_file_with_no_variables_is_written_and_read_back |
| 56 | 715 | raw_check_vars: `at >= n` to `at > n` | raw_check_vars_reads_nothing_at_the_end_of_the_data: `raw_check_vars(&b, 3, 3, 1) == RAW_NONE` on an exact `[3]u8` | reads b[3], traps | raw_check_vars_reads_nothing_at_the_end_of_the_data |
| 77 | 873 | raw_at_hex: drop `if (at + hex.len / 2 > out.len) { return false; }` | raw_at_hex_reads_nothing_past_the_end_of_the_buffer: `!raw_at_hex(&o, 0, "abcdef")` on the exact `[2]u8` of `raw_put_hex(&o, 0, "abcd")` | reads o[2], traps | raw_at_hex_reads_nothing_past_the_end_of_the_buffer |

All 32 rows are killed, so none was moved to equivalent and none was left out. S20 is the only mutant that two tests
catch. S40 has two asserts because its mutant `or` gives a different wrong answer on each (false on "1e+5", true on
"1x5"); the scratch run does not say which of the two went red inside the test, only that the test did.

How the 32 were checked, because the gate was not run: a scratch copy of the spec with the 14 new tests was copied 32
times and one mutation applied to each by hand edit (the diff of each copy against the unmutated scratch is exactly the
one line of the table, checked), then `tri test` was run on each copy in a T27 OS pane, one at a time, 13:40-13:43 +07
(2 min 30 s for all 32). Every copy exited 1 with a failing test, 31 with one test and S20 with two; no test of the
batch-1 or original tests failed in any copy. This is not the mutation gate: the gate builds its own mutants and
decides by its own build mode, which is assumed to be the build `tri test` uses, and only the gate can say that its 526
mutants come out as the forecast in the head of this note.

## The two dead lines

Lines 683 and 954 of `specs/analog/raw_writer.t27` are not touched and stay in the accepted file as `[drop-guard]`
entries with their reasons. Why no assert can kill either: dropping line 683 (`if (at >= n) { return RAW_E_SHORT; }`
in `raw_check_ascii`) changes nothing, because for `at >= n` `raw_eol` returns `at` without reading, `e >= n` holds, and
line 685 returns the same RAW_E_SHORT; dropping line 954 (`if (s != RLC_EXP_S) { return 0; }` in `raw_rlc_file`)
changes nothing because the function takes no tables, always folds the cell's own run, and
`the_rlc_trace_is_the_cells_own_run` already asserts that the fold equals RLC_EXP_S, so the guard fires only after an
edit of `linear.t27` that this test would report first. The issue's batch-1 choice, followed here, is to keep the code
and the accepted entries: keep or delete is the owner's call, and the default taken (no deletion) costs two accepted
lines and no behaviour. If the owner deletes a line, the matching entry goes with it, and every entry below that line is
re-keyed in the same commit, because the accepted file is keyed by line number.

## The 14 tests added by batch 2 (appended after the 5 of batch 1, at the end of the spec)

Runtime asserts per test, from the `tri test` report. 14 tests, 101 asserts. Group J is eight tests and not the
one `the_writers_stay_inside_their_buffers` the classification proposed: a trap ends a test at the first bad access, and
one test per function names the function that trapped.

- `the_ascii_body_is_judged_line_by_line` (10, group E). Five direct calls of `raw_check_ascii` with nv = 1, np = 1: the
  whole point (control, RAW_OK) and the four malformed bodies listed in the table, each in its own zero-filled buffer.
- `the_line_readers_stop_at_the_end_of_the_data` (13, group F). `raw_at` on an exact `[3]u8`; `raw_eol` and `raw_has` over
  "abz" with n = 3 (control) and n = 2; `raw_uint` over "123" and "12  " with the range cut inside the number and inside
  the space run.
- `a_number_is_judged_inside_its_range` (14, group F). `raw_value_ok` on "abc", "1." (range "1"), "1e" (range "1"),
  "1e+5", "1x5", "1.57" (range [0, 3), that is "1.5") and "1e5".
- `raw_sig_ignores_digits_left_by_the_last_call` (6, group G). Two `raw_sig` calls on one pair of scratch arrays, with
  d[16] and d[17] preset to stale digits. The two numbers are built from bit patterns with `raw_from_bits`, as the
  vectors are, not from decimal literals.
- `raw_sig_carries_out_of_sixteen_nines` (19, group I). The double 0x3EB0C6F7A0B5ED8D (the one nearest 1e-6, exactly
  9.99999999999999954748...e-7): `raw_sig` returns -6 with d[0] = 1 and d[1..15] = 0 (15 asserts in a loop), and
  `raw_put_f64` writes the 21 bytes "1.000000000000000e-06". This is the carry-out branch no vector reaches.
- `the_rlc_current_is_in_amps` (3, group H). The RLC trace of the cell: the fold equals RLC_EXP_S (control), and the peak
  of column 4 over the 201 points lies strictly between 0.005 and 0.006.
- `a_carry_past_the_last_limb_is_dropped_not_written` (2), `a_full_buffer_counts_the_byte_that_does_not_fit_and_writes_none`
  (3), `raw_at_reads_nothing_past_the_end_of_the_buffer` (3), `a_short_value_table_reads_as_zero_in_ascii` (8),
  `a_short_value_table_reads_as_zero_in_binary` (11), `a_file_with_no_variables_is_written_and_read_back` (5),
  `raw_check_vars_reads_nothing_at_the_end_of_the_data` (2) and `raw_at_hex_reads_nothing_past_the_end_of_the_buffer` (2)
  (group J): each gives one function an array of exactly the size its test needs, or a table with fewer values than
  nv * np, and asserts what the original returns. They pass on the original because it never touches the cell past the
  end; the mutant touches it and the build traps.

`tri test specs/analog/raw_writer.t27 --t27c <build t27c>`, in a T27 OS pane, 13:34 +07: 37 tests, 37 pass, 0 fail, 0
vacuous passes of 37, exit 0 (23 tests before this batch, 14 added).

## Lab results behind groups H, I and J

Three values were not derivable by hand with certainty, so a throwaway spec of 18 tests was run in a pane at 13:28 +07
(t27 only, not committed; the 9 passes and 9 expected fails below are its whole result).

- **The build traps** (group J). Three probe tests read one cell past a 2-byte array, write one cell past it, and divide
  by zero; all three went red. The 32 mutant runs agree: the S2 mutant writes a[1] of a 1-limb array, and the one test
  that goes red has only the asserts `n == 2` and `a[0] == 999999998`, which the original passes; the only thing the
  mutant adds is the write, so the failure is the trap and not a wrong value. A trapping test fails by itself and the
  other tests still pass.
- **The RLC peak** (group H). Thresholds on the peak of column 5 * p + 4: the peak is above 0.0010, 0.0030, 0.0040,
  0.0045, 0.0050 and 0.0052 (all pass) and not above 0.0054, 0.0056, 0.0058, 0.0060, 0.0070 and 0.0100 (all fail), so
  the peak lies in (0.0052, 0.0054] A. The committed window 0.005 to 0.006 contains it with margin on both sides and
  is far from the mutant's peak near 53.
- **The carry-out double** (group I). The probe ran the double named above through `raw_sig` (17 asserts) and
  `raw_put_f64` (2 asserts): the exponent is -6, the digits are 1 and fifteen 0s, and the text is
  "1.000000000000000e-06" (21 bytes). The classification said no double was named; this is the one.

## Deviations from the issue, on purpose

- **The three lengths.** The issue and the classification give n = 11, 11 and 10 for the empty-name, empty-type and
  fourth-field lines; the true byte counts are 9, 9 and 9. The tests assert `n == 9` from `raw_puts`, and the run agrees.
  The expected return of every line was re-derived from the function bodies, not copied from the classification.
- **Count lines end in a line feed.** The classification gives "No. Variablez: 1", "No. Variables: 1000000001" and "No.
  Points: 1000000000" with no line feed. Without one, `raw_eol` runs to n, which is the case survivor S25 (609) of
  group F is about; batch 1 ends each count line with LF so its tests do not decide it, and batch 2 decides it with
  `raw_eol(&a1, 2, 0)` on a range cut inside "abz".
- **A fresh zero-filled buffer per case.** A reused buffer could leave a byte of an earlier text past the data; S27
  (623) and S31 (629) of group F read one byte past the range, so a clean tail kept them for batch 2, which kills them
  with ranges cut inside the number and inside the space run.
- **Group A is a new test.** The issue allowed editing `the_binary_file_is_ngspices_byte_for_byte`; the changed-byte
  check is a separate test instead.
- **Batch 2: doubles are bit patterns.** The classification names 9007199254740992.0 and 3000000000000000.5; the tests
  build them as `raw_from_bits(0x4340000000000000)` and `raw_from_bits(0x432550F7DCA70001)`, the way every vector of
  the spec does. Both decimals are exact doubles (2^53, and 3e15, which lies in [2^51, 2^52) where the spacing is 0.5).
- **Batch 2: the RLC window is the lab's, not the classification's.** The classification proposed
  `0.001 < imax < 0.01` around an estimated 5.5 mA; the lab measured a peak in (0.0052, 0.0054] A, and the test uses
  `0.005 < imax < 0.006`. Both windows kill S82; the committed one is tight enough to catch a scale error of 2.
- **Batch 2: group F is two tests and group J is eight.** The classification proposed one test for each of
  `reader_helpers_stop_at_their_range` and `the_writers_stay_inside_their_buffers`; a trap ends its test, so group J
  has one test per function, and group F splits the integer readers from the number reader.
- **Batch 2: `raw_at(&b, 2, "c")` is added to the `raw_at` cases.** The classification names the `raw_at` of "cd" for S19
  and "abc" at 0 for S20; the extra true case at the buffer end is what the scratch run showed to be the second
  catcher of S20, not a new target.
- **Batch 2: the dead lines are kept.** The classification leaves 683 and 954 to the owner; no assert can kill either,
  so both stay accepted (section "The two dead lines").

## Not claimed

- The gate was not run on this tree and the seal was not saved. The forecast (474 killed, 21 survived, 31 hung) is the
  sum of the census and the 62 targeted kills. For the 32 of batch 2 that each assert kills its mutant is shown by a
  hand-run of the mutant on a scratch copy (section "The 32 survivors closed by batch 2"); for the 30 of batch 1 it is
  shown only by hand against the mutated line, and the gate will not name a killing test.
- The 19 equivalent survivors are classified by reading the helpers (`raw_pow(base, 0)` is 1, `raw_eol` does not read
  when `from` >= n, the high bit of m at be = 1), not proved equivalent by a tool or run. The gate is the first run that
  can contradict one of them.
- Every expected value in the new tests was hand-derived from the function bodies, except the three the lab decided (the
  RLC peak window, the carry-out double, the trap). `tri test` now agrees with all of them (37 of 37 pass), but a
  passing assert says only that the original function gives that value, not that the mutant does not; the mutant side
  is the scratch run for batch 2 and reading alone for batch 1.
- The hand-run of the 32 mutants used `tri test`'s build. The gate builds its mutants itself; that the two build modes
  are the same (Debug, which traps) is assumed, not read. If the gate's build does not trap, the nine group J mutants
  would survive it as unkilled and the first gate run would show them.
- The RLC window is a measurement of this cell at this tree: the peak lies in (0.0052, 0.0054] A. The committed bounds
  0.005 and 0.006 are rounder than the measurement. The bounds tie the test to the cell's present constants: an edit of
  the cell that moves the peak outside them turns it red, and the fold assert next to it would report that edit too.
  The continuous-time estimate of 5.5 mA that the classification gave is not what the trace shows; the trace is the
  cell's own backward-Euler run.
- The carry-out double is one double, confirmed by one probe; no search over other doubles is claimed. The decimal
  expansion in the comment of the test (9.99999999999999954748...e-7) is not checked digit by digit by any run; the run
  confirms its consequence, that `raw_sig` carries out of sixteen 9s and `raw_put_f64` prints
  "1.000000000000000e-06".
- The multi-line `if (...) { inf.err = ...; return inf; }` blocks of `raw_check` (title, date, plot, flags, the two
  counts, the variables header) are not mutated by the census, so their absence from the survivors is not a finding; a
  read of the tests shows no assert on RAW_E_DATE, RAW_E_PLOT, RAW_E_NVARS or RAW_E_NPTS, an observation and not a
  measured gap.
- Group J (the nine exact-size and short-input survivors) rests on the premise that the `tri test` build traps on an
  out-of-range read, an out-of-range write and a divide by zero. Batch 1 carried it as inferred from two earlier
  censuses; it is now measured: three probe tests went red on exactly those accesses, and the nine scratch mutants each
  went red on their one test. What is still an inference is only that the gate's own build behaves the same way.
- The 31 hung mutants are classified by construction: the tool reports each as outliving the timeout, which is consistent
  with an endless loop but was not shown by a run that was let to finish.
- `tri coverage --strict` was not run locally; CI runs it.
- No pull request is opened by this lane; the branch is pushed for the orchestrator to land.

## Not done

- The survivor gate (`tri mutate spec --accepted`) was not run on this tree. The orchestrator's first run is what turns
  the forecast into a number and what would contradict any of the 19 equivalents.
- The seal `.trinity/seals/analog_AnalogRawWriter.json` was not saved and is stale after the spec edit; importers of the
  module may need the same.
- No pull request was opened.
- The 30 mutants of batch 1 were not hand-run on a scratch copy as the 32 of batch 2 were.
- Lines 683 and 954 are not deleted (the owner's call; see "The two dead lines").
- The unmutated multi-line error returns of `raw_check` (RAW_E_DATE, RAW_E_PLOT, RAW_E_NVARS, RAW_E_NPTS) are not
  asserted by any test; the census does not mutate them, so they were out of scope and are not a measured gap.
