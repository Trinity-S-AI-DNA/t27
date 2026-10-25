# Mutation census of specs/analog/raw_writer.t27, 2026-10-25

Issue #12273 (batch 1 of 2, groups A to D), part of epic #10486 (weak spots of the analog v0.1 release); the second batch,
groups E to J, is issue #12274. This lane wrote the missing asserts and the accepted file. The survivor gate was not run
on this tree and the seal was not re-saved. Where each number comes from:

- **Before** (526 mutants, 412 killed, 83 survived, 31 hung, 0 unviable) is the analog-loop census of the unmodified spec,
  10:39-10:55 +07, exit 0, 15 min 50 s, `tri mutate spec` 3 mutants at a time with zig `-j2` each, t27c 0.5.2 (the
  census text does not record the build commit), on the tree at commit 035df5ce60. The spec at the base of this lane
  (707240639f) is byte-identical to that tree's (`git diff 035df5ce60 707240639f -- specs/analog/raw_writer.t27` is
  empty), so the census line numbers hold. The classes of the 83 survivors are from reading the spec and its helpers;
  no mutant was re-run in this lane.
- **After** is a forecast, not a measurement: if the 30 asserts below kill what they target, 526 mutants, 442 killed, 53
  survived, 31 hung, 0 unviable, and 84 accepted lines alive (53 survivors + 31 hung), 0 accepted lines now killed. The
  first gate run is what turns this into a number; an unaccepted survivor on one of the 30 lines in the table below
  would mean that assert does not kill its mutant.
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
| 685 x2, 688, 691 x2, 701 x2 | raw_check_ascii (group E) | test gap | pending #12274, accepted for now |
| 387 flip, 609, 615, 623, 629 flip, 646, 654 x2, 657, 663, 665 swap-logic | reader helpers (group F) | test gap | pending #12274, accepted for now |
| 230, 233 | raw_sig reads a stale digit (group G) | test gap | pending #12274, accepted for now |
| 938 | scale of the RLC current column (group H) | test gap | pending #12274, accepted for now |
| 258 x2 | carry-out of sixteen 9s (group I) | test gap | pending #12274, accepted for now |
| 140, 270, 387 drop, 543 x2, 560, 571, 715 flip, 873 drop | exact-size buffers (group J) | test gap, if the Debug build traps | pending #12274, accepted for now |
| 683 drop, 954 | `if (at >= n) { return RAW_E_SHORT; }` in raw_check_ascii; `if (s != RLC_EXP_S) { return 0; }` in raw_rlc_file | dead line | kept, accepted |

Totals: 19 equivalent (accepted) + 62 test gap (30 killed here: A 6, B 14, C 2, D 8; 32 pending: E 7, F 11, G 2, H 1,
I 2, J 9) + 2 dead line (kept, accepted) = 83. The 31 hung mutants are 30 `x = x + 1` turned into `x = x` in a loop
counter or cursor (lines 136, 176, 235, 242, 280, 309, 364, 368, 380, 391, 477, 500, 546, 549, 562, 609, 629, 650, 658,
668, 690, 719, 722, 725, 804, 859, 877, 888, 919, 939) and one `while (carry > 0)` turned into `>= 0` (line 139), which
cannot end; the loop never ends, so no test can go red and only the clock ends the mutant. They are accepted by
construction.

## The accepted file

`docs/analog/mutation-accepted-raw_writer.txt` has 84 entries: 19 equivalent, 2 dead line, 32 `pending: #12274`, 31
hung. An entry is one line and kind, as `tri mutate spec --accepted` reads it (`specs/tri/mutate/survivors.t27`), and the
gate exits 2 on an accepted line whose mutants are all killed. The 30 mutants of groups A to D have no entry on purpose:
they are meant to be killed. Checked by hand: no key is listed twice; the 53 survivor entries plus the 30 closed ones
are the 83 survivors of the census; the 31 hung entries are the census's 31. A line with mutants of two kinds has two
entries (683 has the equivalent flip and the dead guard; 387, 543, 654, 665 and 685 have two kinds each). When 12274 lands
it removes its `pending` lines and appends its tests after these; the new tests sit at the end of the spec so no accepted
line number moved.

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

The census rows S56 (715 flip, group J), S60, S62, S64 and S66 (equivalent flips) are deliberately not hit by any new
assert: S56 stays pending and the others stay accepted, and a test that killed one of them would make the gate fail on a
stale accepted line.

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

## Deviations from the issue, on purpose

- **The three lengths.** The issue and the classification give n = 11, 11 and 10 for the empty-name, empty-type and
  fourth-field lines; the true byte counts are 9, 9 and 9. The tests assert `n == 9` from `raw_puts`, and the run agrees.
  The expected return of every line was re-derived from the function bodies, not copied from the classification.
- **Count lines end in a line feed.** The classification gives "No. Variablez: 1", "No. Variables: 1000000001" and "No.
  Points: 1000000000" with no line feed. Without one, `raw_eol` runs to n, which is the case the pending
  survivor 609 of group F waits for; the tests end each count line with LF so they do not decide it.
- **A fresh zero-filled buffer per case.** A reused buffer could leave a byte of an earlier text past the data; the
  pending survivors 623 and 629 (group F) read one byte past the range, so a clean tail keeps them pending.
- **Group A is a new test.** The issue allowed editing `the_binary_file_is_ngspices_byte_for_byte`; the changed-byte
  check is a separate test instead.

## Not claimed

- The gate was not run on this tree and the seal was not saved. The forecast (442 killed, 53 survived, 31 hung) is the
  sum of the census and the 30 targeted kills; no mutant was run in this lane, so that each assert kills its mutant is
  shown by hand against the mutated line, and the tool will not name a killing test.
- The 19 equivalent survivors are classified by reading the helpers (`raw_pow(base, 0)` is 1, `raw_eol` does not read
  when `from` >= n, the high bit of m at be = 1), not proved equivalent by a tool or run. The gate is the first run that
  can contradict one of them.
- Every expected value in the new tests was hand-derived. `tri test` now agrees with all of them (23 of 23 pass), but a
  passing assert says only that the original function gives that value, not that the mutant does not.
- The multi-line `if (...) { inf.err = ...; return inf; }` blocks of `raw_check` (title, date, plot, flags, the two
  counts, the variables header) are not mutated by the census, so their absence from the survivors is not a finding; a
  read of the tests shows no assert on RAW_E_DATE, RAW_E_PLOT, RAW_E_NVARS or RAW_E_NPTS, an observation and not a
  measured gap.
- Group J (the nine exact-size and short-input survivors) rests on the premise that the Debug test build traps on an
  out-of-range index, an unsigned underflow and a divide by zero; it is inferred from two earlier censuses, not
  measured here. If the build does not trap, those nine become equivalent. They stay pending and are 12274's.
- The 31 hung mutants are classified by construction: the tool reports each as outliving the timeout, which is consistent
  with an endless loop but was not shown by a run that was let to finish.
- `tri coverage --strict` was not run locally; CI runs it.
- No pull request is opened by this lane; the branch is pushed for the orchestrator to land.
