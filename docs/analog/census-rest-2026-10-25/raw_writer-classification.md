# raw_writer: the 83 survivors and 31 hung mutants of the census, by class

## raw_writer (measured 10:39-10:55 +07, exit 0, 15 min 50 s)

526 mutants, 412 killed (all by a failing test), 83 survived, 31 hung, 0 unviable. Kinds: drop-guard 44, flip-cmp 148, swap-logic 32, drop-step 80, swap-arith 187, ret-default 35. Run: `tri mutate spec` on `specs/analog/raw_writer.t27`, 3 at a time, zig -j2 each; the census ran with t27c 0.5.2, build commit unknown, and the tree was at commit 035df5ce60 (the version and head notes and the raw output file are kept outside the repo). The classes below are from reading the spec in a checkout of master 1c783b0b31; the spec is unchanged since the census (`git diff 035df5ce60 1c783b0b31 -- specs/analog/raw_writer.t27` is empty, re-run for this file), so the line numbers are the census's; nothing was re-run.

Facts the table leans on: `raw_sig` works in two scratch buffers, `a` (90 limbs) and `d` (810 digits), that every test and the writer reuse, so after a long number the cells d[16] and d[17] hold stale digits, and only a test that sets them can tell a read of a stale cell from a fresh write (the vectors leave d[16] = 2 and d[17] = 0 behind); `raw_expand` multiplies in chunks of 2^30 or 5^13, and those two are exactly `big`, so a loop that stops one chunk early and finishes with `raw_pow` yields the same limbs; the test buffers are oversized and zero-filled (`var b : [32]u8 = [0] ** 32;` then `raw_puts(&b, 0, "...")`) and no test fills a buffer exactly or overflows one, so a read at index n sees a 0 byte, and an off-by-one at the end of the data is invisible unless it changes a result; the mutants that read or write one cell past the end are killed only if the build traps on it, and the tests run as Zig Debug tests, where an out-of-range index, an unsigned underflow or a divide by zero panics (inferred, not read: it is the finding of `sar_logic-classification.md` in this directory (the line "Tests run as Zig Debug tests") and of the pre-audit linear_zoh classification, which read it off the survivor and hung lists, and a grep of the mutate specs for the build mode found nothing; one scratch test on the lab would settle it); `raw_eol` does not read when `from` is at or past n; `and` binds tighter than `or` in the swap-logic rows of line 723 (the empty-name assert kills them under either reading); `raw_put_f64` always writes a '.' after the first digit, so `raw_value_ok` has never been shown an exponent without a '.' or a range that ends inside a number; the byte-for-byte vectors (FMT_BITS, SM_A, SM_H) hold 35 bit patterns and no zero; the binary byte-for-byte test (line 1011) calls `raw_at_hex` only positively, where its ASCII twin (lines 1006-1008) changes one byte and asserts the compare turns false, so a `raw_at_hex` that returns true whenever the bytes fit passes every assert in it; the multi-line `if (...) { inf.err = ...; return inf; }` blocks of `raw_check` (title, date, plot, flags, the two counts, the variables header) are not mutated by the census, so they cannot be listed here and their absence is not a finding.

The 31 hung are 30 drop-step mutants of loop counters and cursors (`x = x + 1` becomes `x = x`, lines 136, 176, 235, 242, 280, 309, 364, 368, 380, 391, 477, 500, 546, 549, 562, 609, 629, 650, 658, 668, 690, 719, 722, 725, 804, 859, 877, 888, 919, 939) and 1 flip-cmp (line 139, `while (carry > 0)` to `>= 0`, which cannot end); each loop never ends, so they are the mutant's hang, not a test gap.

| line | mutant | class | why |
| --- | --- | --- | --- |
| 1 / 79 | `y < 0.0` to `y <= 0.0` in `raw_parts` | equivalent | `raw_sig` hands it a nonzero finite value, and at y = 0.0 the branch computes 0.0 - 0.0, the same 0.0 |
| 2 / 140 | `cnt < a.len` to `cnt <= a.len` in `raw_mul` | test gap (group J, low value) | the two differ only when a carry lands at cnt == a.len: the original drops it (the doc comment says a carry past the end is never written), the mutant writes `a[a.len]` and traps in a Debug test (inferred); no call fills the array. Missing assert: `a` of 1 limb holding 999999999, `raw_mul(&a, 1, 2)` returns 2 and `a[0]` is 999999998; hand-derived and unverified |
| 3 / 157 | `e < 0` to `e <= 0` in `raw_expand` | equivalent | at e = 0 the branch sets left = 0 and the 5-power constants, and with left = 0 neither multiply (163, 167) runs, so the limbs are the same |
| 4 / 163 | `left >= chunk` to `left > chunk` in `raw_expand` | equivalent | at left == chunk the loop is skipped and line 167 multiplies by `raw_pow(base, chunk)`, which is 2^30 or 5^13, exactly `big`; same limbs |
| 5 / 167 | `left > 0` to `left >= 0` in `raw_expand` | equivalent | at left = 0 it multiplies by `raw_pow(base, 0)` = 1, which changes no limb and leaves no carry |
| 6 / 228 | `p.e < 0` to `p.e <= 0` in `raw_sig` | equivalent | at p.e = 0 it adds 0 to ex |
| 7 / 230 | `nd > RAW_SIG` to `nd >= RAW_SIG` in `raw_sig` | test gap (group G, low value) | at nd = 16 the mutant enters the rounding and reads d[16], a cell this call never wrote, so the result depends on the previous call's digit. Missing assert: set d[16] = 9, then `raw_sig(9007199254740992.0, &a, &d)` (2^53, 16 digits) returns 15 and d[15] is 2 (the mutant rounds up to 3); hand-derived and unverified |
| 8 / 233 | `j < nd` to `j <= nd` in `raw_sig` | test gap (group G, low value) | the mutant also reads d[nd], a stale cell, into the sticky flag `rest`; it changes a result only on an exact tie whose real tail is zero. Missing assert: set d[17] = 7, then `raw_sig(3000000000000000.5, &a, &d)` (digits 3, fifteen 0, 5: an exact tie, d[15] even) returns 15 and d[15] is 0 (the mutant sees `rest` and gives 1); hand-derived and unverified |
| 9 / 240 | `i < RAW_SIG` to `i <= RAW_SIG` in `raw_sig` | equivalent | the extra pass zeroes d[16] only when nd <= 16, and d[16] is read only when nd > 16, after `raw_digits` has written it |
| 10 / 258 | `ex = ex + 1` to `ex = ex` in `raw_sig` | test gap (group I, needs the lab) | the carry-out branch (sixteen 9s rounded up, `d[0] = 1`, exponent plus one) is reached by no vector, though the section comment promises "the carries (9.9999...95e-1)"; the vectors' carries stop inside the digits (3999999999999999.5). Missing assert: `raw_sig` of a double whose 16 digits are all 9 and whose tail rounds up returns N + 1 with d[0] = 1 and d[1..15] = 0; the double comes from a lab search near 10^N, and none is named here |
| 11 / 258 | `ex = ex + 1` to `ex = ex - 1` in `raw_sig` | test gap (group I, needs the lab) | same branch, same assert; the mutant returns N - 1 |
| 12 / 270 | `w < out.len` to `w <= out.len` in `raw_put` | test gap (group J, low value) | differs only at w == out.len, where the mutant writes `out[out.len]` and traps (inferred); no test writes into an exactly full buffer. Missing assert: `var b : [2]u8 = [0] ** 2`, `raw_puts(&b, 0, "abc")` returns 3 (the cursor counts the byte that did not fit), `b[0]` is 97 and `b[1]` is 98; hand-derived and unverified |
| 13 / 297 | `x < 0.0` to `x <= 0.0` before "-inf" in `raw_put_f64` | equivalent | on this line x is an infinity (a NaN returned at 295, a finite value fails `x - x != 0.0`), and an infinity is never 0.0 |
| 14 / 301 | `x < 0.0` to `x <= 0.0` (the sign byte) in `raw_put_f64` | test gap (group C, high value) | at x = 0.0 the mutant writes a '-' before the zero: 22 bytes, "-0.000000000000000e+00". No test writes 0.0: the vectors hold no zero, and the RLC trace (point 0 is all zero) is judged by `raw_check`, which accepts a leading '-'. Missing assert: `raw_put_f64(&b, 0, 0.0, &a, &d)` returns 21 and `raw_at(&b, 0, "0.000000000000000e+00")`; hand-derived and unverified |
| 15 / 333 | `x < 0.0` to `x <= 0.0` before 0xFFF0... in `raw_to_bits` | equivalent | x is an infinity here, never 0.0 |
| 16 / 338 | `x < 0.0` to `x <= 0.0` (the sign word) in `raw_to_bits` | equivalent | zero already returned at 336, so x is nonzero here |
| 17 / 347 | `be >= 1` to `be > 1` in `raw_to_bits` | equivalent | at be = 1 the line-348 path gives the sign word OR-ed with m shifted by 0, and m already has bit 52 set, which is the same word the be == 1 branch builds |
| 18 / 355 | drop `if (ex == 2047) { return 0.0; }` in `raw_from_bits` | test gap (group C, high value) | the doc comment says an infinity or NaN pattern reads as 0.0, and no test feeds one; without the guard the exponent field 2047 builds 2^52 * 2^972 = infinity. Missing assert: `raw_from_bits(0x7FF0000000000000)` and `raw_from_bits(0x7FF8000000000000)` both equal 0.0; hand-derived and unverified |
| 19 / 387 | drop `if (at + s.len > out.len) { return false; }` in `raw_at` | test gap (group J, low value) | the guard stops a read past the end; with the oversized zero-filled buffers the unguarded read finds a mismatch and returns false anyway, so only an exactly sized buffer shows it (a trap, inferred). Missing assert: `var b : [3]u8 = [0] ** 3` holding "abc", `raw_at(&b, 2, "cd")` is false (the mutant reads `b[3]`) |
| 20 / 387 | `>` to `>=` in `raw_at` | test gap (group F, low value) | the mutant refuses a string that ends exactly at the end of the buffer; value-level, no trap. Missing assert: `var b : [3]u8 = [0] ** 3` holding "abc", `raw_at(&b, 0, "abc")` is true |
| 21 / 543 | `pt * nv + v < vals.len` to `pt / nv + v < vals.len` in `raw_put_ascii` | test gap (group J, low value) | the guard implements the doc comment "a missing value is 0" and no test passes fewer values than nv * np; with a short `vals` the mutant's guard is true too often and it reads `vals[pt * nv + v]` out of range (a trap, inferred). Missing assert: nv = 2, np = 2, `vals` of 3 values, the last cell of the output is "0.000000000000000e+00" (the mutant reads `vals[3]`); hand-derived and unverified |
| 22 / 543 | `<` to `<=` in `raw_put_ascii` | test gap (group J, low value) | same short-`vals` call: nv = 1, np = 2, `vals` of 1 value; at pt = 1 the mutant reads `vals[1]` |
| 23 / 560 | `i < vals.len` to `i <= vals.len` in `raw_put_binary` | test gap (group J, low value) | `raw_write` always gives it nv * np <= vals.len, so only a direct call with nv * np > vals.len shows it. Missing assert: `raw_put_binary(&out, 0, 1, 2, &vals)` with one value writes 16 bytes, the last 8 being zero bits (the mutant reads `vals[1]`); hand-derived and unverified |
| 24 / 571 | `nv > 0` to `nv >= 0` in `raw_write` | test gap (group J, low value) | with an empty variable table nv = 0, and the mutant divides `vals.len` by 0 (a trap, inferred); no test writes a file with no variables. Missing assert: `raw_write` with `vars` = "" returns a length, and `raw_check` of it has err RAW_OK, nv 0, np 0; hand-derived and unverified |
| 25 / 609 | `i < n` to `i <= n` in `raw_eol` | test gap (group F, low value) | only a line with no line feed before n reaches i = n, and every test line ends in one. Missing assert: `raw_eol(&b, 2, 0)` is 2 for b = "abz" (the mutant gives 3) |
| 26 / 615 | drop `if (at + s.len > n) { return false; }` in `raw_has` | test gap (group F, low value) | `raw_has` differs from `raw_at` only by this bound against n instead of the buffer length, and the buffers are longer than n. Missing assert: `raw_has(&b, 2, 0, "abz")` is false for b = "abz" (the mutant reads the third byte and says true) |
| 27 / 623 | `i < to` to `i <= to` in `raw_uint` | test gap (group F, low value) | the digit run may not pass `to`; every call is on a range that ends at a non-digit. Missing assert: `raw_uint(&b, 0, 2)` is 12 for b = "123" (the mutant takes the third digit and returns RAW_BAD) |
| 28 / 624 | drop `if (i - from >= 18) { return RAW_BAD; }` in `raw_uint` | test gap (group D, high value) | no number of 19 or more digits is ever read. Missing assert: `raw_uint` of "1000000000000000000" (1 and 18 zeros, 19 digits) is RAW_BAD (the mutant returns 10^18) |
| 29 / 624 | `>= 18` to `> 18` in `raw_uint` | test gap (group D, high value) | the same 19-digit assert: the mutant lets the 19th digit through and returns 10^18 |
| 30 / 628 | drop `if (i == from) { return RAW_BAD; }` in `raw_uint` | test gap (group D, high value) | an empty or all-space range is a bad count; without the line it reads as 0. Missing assert: `raw_uint(&b, 0, 0)` is RAW_BAD, and `raw_uint` of "  " over [0, 2) is RAW_BAD |
| 31 / 629 | `i < to` to `i <= to` (spaces) in `raw_uint` | test gap (group F, low value) | the space run may not pass `to`. Missing assert: `raw_uint(&b, 0, 3)` is 12 for b = "12  " (the mutant takes the fourth byte and returns RAW_BAD) |
| 32 / 629 | `i < to and buf[i] == 32` to `or` in `raw_uint` | test gap (group D, high value) | the `or` skips any byte while i < to, so junk after the digits is eaten as padding. Missing assert: `raw_uint(&b, 0, 3)` is RAW_BAD for b = "12x" (the mutant returns 12) |
| 33 / 630 | drop `if (i != to) { return RAW_BAD; }` in `raw_uint` | test gap (group D, high value) | the same "12x" assert: without the line the trailing junk is accepted and 12 comes back |
| 34 / 636 | drop `if (!raw_has(buf, n, at, prefix)) { return RAW_BAD; }` in `raw_count_line` | test gap (group D, high value) | the prefix is never checked, only its length is skipped; no test has a wrong count-line label of the same length. Missing assert: `raw_count_line(&b, 16, 0, "No. Variables: ")` is RAW_BAD for b = "No. Variablez: 1" (the mutant returns 1) |
| 35 / 638 | drop `if (v > RAW_MAX) { return RAW_BAD; }` in `raw_count_line` | test gap (group D, high value) | no count above RAW_MAX (1000000000) is read. Missing assert: `raw_count_line` of "No. Variables: 1000000001" is RAW_BAD (the mutant returns the number) |
| 36 / 638 | `v > RAW_MAX` to `v >= RAW_MAX` in `raw_count_line` | test gap (group D, high value) | the count RAW_MAX itself is allowed and no test sits on that edge. Missing assert: `raw_count_line` of "No. Points: 1000000000" is 1000000000 (the mutant returns RAW_BAD) |
| 37 / 645 | `i < to` to `i <= to` (the minus sign) in `raw_value_ok` | equivalent | at i == to the mutant would step i past `to` only if `buf[to]` is '-'; then no digit is counted and the function returns false, as the original does for an empty range |
| 38 / 646 | `and` to `or` in the "inf" test of `raw_value_ok` | test gap (group F, low value) | the `or` accepts any 3-byte token. Missing assert: `raw_value_ok(&b, 0, 3)` is false for b = "abc" (the mutant says true) |
| 39 / 654 | `i < to` to `i <= to` (the '.') in `raw_value_ok` | test gap (group F, low value) | the mutant looks one byte past the range. Missing assert: `raw_value_ok(&b, 0, 1)` is true for b = "1." (the range is just "1"; the mutant reads the '.', finds no fraction digit and says false) |
| 40 / 654 | `and` to `or` in the '.' test of `raw_value_ok` | test gap (group F, low value) | the `or` treats any byte as the point, and the writer never emits an exponent without a '.'. Missing assert: `raw_value_ok(&b, 0, 4)` is true for "1e+5" (the mutant demands fraction digits and says false); also false for "1x5" (the mutant takes 'x' for the point and accepts) |
| 41 / 657 | `i < to` to `i <= to` (fraction digits) in `raw_value_ok` | test gap (group F, low value) | the fraction run may not pass `to`. Missing assert: `raw_value_ok(&b, 0, 3)` is true for b = "1.57" (the mutant takes the '7' and ends past `to`) |
| 42 / 663 | `i < to` to `i <= to` (the 'e') in `raw_value_ok` | test gap (group F, low value) | the mutant looks one byte past the range. Missing assert: `raw_value_ok(&b, 0, 1)` is true for b = "1e" (the mutant reads the 'e', finds no exponent digit and says false) |
| 43 / 665 | `i < to` to `i <= to` (the exponent sign) in `raw_value_ok` | equivalent | at i == to there is no exponent digit either way, so both versions return false |
| 44 / 665 | `and` to `or` (the exponent sign) in `raw_value_ok` | test gap (group F, low value) | the `or` takes the first exponent digit for a sign. Missing assert: `raw_value_ok(&b, 0, 3)` is true for b = "1e5" (the mutant skips the 5 and says false) |
| 45 / 683 | drop `if (at >= n) { return RAW_E_SHORT; }` in `raw_check_ascii` | dead line | when at >= n, `raw_eol` returns at without reading, so e >= n and line 685 returns RAW_E_SHORT, the same code; the line only skips the call. Keep or delete is the owner's call |
| 46 / 683 | `at >= n` to `at > n` in `raw_check_ascii` | equivalent | at at == n the mutant falls to `raw_eol`, which returns n, and line 685 gives RAW_E_SHORT: the same code as the original |
| 47 / 685 | drop `if (e >= n) { return RAW_E_SHORT; }` in `raw_check_ascii` | test gap (group E, high value) | a value line with no line feed before the end of the data is judged short; every truncation the tests make leaves a line feed. Missing assert: `raw_check_ascii(&b, 2, 0, 1, 1)` is RAW_E_SHORT for b = " 0" (the mutant says RAW_E_INDEX) |
| 48 / 685 | `e >= n` to `e > n` in `raw_check_ascii` | test gap (group E, high value) | `raw_eol` returns exactly n for such a line, so the same " 0" assert kills it (the mutant says RAW_E_INDEX) |
| 49 / 688 | drop `if (buf[at] != 32) { return RAW_E_INDEX; }` in `raw_check_ascii` | test gap (group E, high value) | the leading space of a point line is never checked: the mutant skips the first byte whatever it is. Missing assert: `raw_check_ascii(&b, 12, 0, 1, 1)` is RAW_E_INDEX for b = "x0" + TAB + "1.0e+00" + two line feeds (the mutant says RAW_OK) |
| 50 / 690 | `t < e` to `t <= e` in `raw_check_ascii` | equivalent | at t == e `buf[e]` is a line feed, not a TAB, so the original stops at t == e and the mutant steps once more to e + 1; either way line 691 sees t >= e and returns RAW_E_INDEX |
| 51 / 691 | drop `if (t >= e) { return RAW_E_INDEX; }` in `raw_check_ascii` | test gap (group E, high value) | an index with no TAB after it is never presented. Missing assert: `raw_check_ascii(&b, 4, 0, 1, 1)` is RAW_E_INDEX for b = " 0" + two line feeds (the mutant says RAW_E_VALUE) |
| 52 / 691 | `t >= e` to `t > e` in `raw_check_ascii` | test gap (group E, high value) | the same " 0" + two line feeds assert (the mutant falls through at t == e and says RAW_E_VALUE) |
| 53 / 701 | drop `if (at >= n) { return RAW_E_SHORT; }` in `raw_check_ascii` | test gap (group E, high value) | a file that ends right after the last value line, with no empty line after the point, is judged short; the tests that cut a file cut inside a line. Missing assert: `raw_check_ascii(&b, 11, 0, 1, 1)` is RAW_E_SHORT for b = " 0" + TAB + "1.0e+00" + one line feed (the mutant reads the byte at n, a 0, and says RAW_E_SHAPE) |
| 54 / 701 | `at >= n` to `at > n` in `raw_check_ascii` | test gap (group E, high value) | the same assert: at == n falls through to `buf[n]` and the mutant says RAW_E_SHAPE |
| 55 / 715 | drop `if (at >= n or buf[at] != 9) { return RAW_NONE; }` in `raw_check_vars` | test gap (group B, high value) | a variable line that does not start with a TAB is taken for one: "x0" + TAB + "time" + TAB + "time" parses as index 0, the 'x' standing in for the TAB. Missing assert: `raw_check_vars(&b, 13, 0, 1)` is RAW_NONE for that line (the mutant returns 13) |
| 56 / 715 | `at >= n` to `at > n` in `raw_check_vars` | test gap (group J, low value) | differs only at at == n, where the mutant reads `buf[n]`; the oversized buffer holds a 0 there, so only an exactly sized buffer traps (inferred). Missing assert: `var b : [3]u8 = [0] ** 3`, `raw_check_vars(&b, 3, 3, 1)` is RAW_NONE |
| 57 / 715 | `or` to `and` in `raw_check_vars` | test gap (group B, high value) | with `and`, a line is refused only if it is both past the end and not a TAB, which never happens; the same "x0..." assert as row 55 kills it (the mutant returns 13) |
| 58 / 717 | drop `if (e >= n) { return RAW_NONE; }` in `raw_check_vars` | test gap (group B, high value) | a last variable line with no line feed before the end of the data is refused; without the line it is read as complete. Missing assert: `raw_check_vars(&b, 12, 0, 1)` is RAW_NONE for b = TAB + "0" + TAB + "time" + TAB + "time" with no line feed (the mutant returns 13, so `raw_check` goes on to say RAW_E_MODE) |
| 59 / 717 | `e >= n` to `e > n` in `raw_check_vars` | test gap (group B, high value) | `raw_eol` returns exactly n for that line, so the same assert kills it |
| 60 / 719 | `t1 < e` to `t1 <= e` in `raw_check_vars` | equivalent | at t1 == e the original and the mutant both stop (`buf[e]` is a line feed, and for the mutant t1 reaches e + 1), and line 720 returns RAW_NONE for t1 >= e either way |
| 61 / 720 | drop `if (t1 >= e or raw_uint(...) != (k as u64)) { return RAW_NONE; }` in `raw_check_vars` | test gap (group B, high value) | the index of a variable line is never wrong in a test, and a line with no second TAB is never presented. Missing assert: `raw_check_vars(&b, 13, 0, 1)` is RAW_NONE for a line numbered 1 (TAB, "1", TAB, "time", TAB, "time", line feed); the mutant returns 13 |
| 62 / 720 | `t1 >= e` to `t1 > e` in `raw_check_vars` | equivalent | at t1 == e the mutant takes `raw_uint` over the rest of the line, and line 723 then returns RAW_NONE because t2 = t1 + 1 > e |
| 63 / 720 | `t1 >= e or raw_uint(...) != k` with `or` to `and` in `raw_check_vars` | test gap (group B, high value) | the `and` refuses a line only if it has no second TAB and a wrong index at once; the same wrong-index assert as row 61 kills it |
| 64 / 722 | `t2 < e` to `t2 <= e` in `raw_check_vars` | equivalent | at t2 == e the mutant steps t2 to e + 1 and line 723 returns RAW_NONE for t2 >= e, as it does for t2 == e in the original |
| 65 / 723 | drop `if (t2 >= e or t2 == t1 + 1 or e == t2 + 1) { return RAW_NONE; }` in `raw_check_vars` | test gap (group B, high value) | a variable with an empty name or an empty type is never presented. Missing assert: `raw_check_vars(&b, 11, 0, 1)` is RAW_NONE for TAB, "0", two TABs, "time", line feed (empty name), and for TAB, "0", TAB, "time", TAB, line feed (empty type, 11 bytes); the mutant returns the length in both |
| 66 / 723 | `t2 >= e` to `t2 > e` in `raw_check_vars` | equivalent | at t2 == e the mutant goes on to the third field, and line 726 (`t3 != e`) returns RAW_NONE because t3 starts at e + 1 |
| 67 / 723 | `t2 >= e or t2 == t1 + 1` with `or` to `and` | test gap (group B, high value) | the empty-name test is lost; the empty-name assert of row 65 kills it |
| 68 / 723 | `t2 == t1 + 1` to `t2 == t1` | test gap (group B, high value) | t2 > t1 always, so the empty-name test never fires; the empty-name assert kills it |
| 69 / 723 | `t2 == t1 + 1` to `t2 == t1 - 1` | test gap (group B, high value) | the same: the empty-name test never fires; the empty-name assert kills it |
| 70 / 723 | `t2 == t1 + 1 or e == t2 + 1` with `or` to `and` | test gap (group B, high value) | an empty name or an empty type alone no longer refuses the line; either assert of row 65 kills it |
| 71 / 723 | `e == t2 + 1` to `e == t2` | test gap (group B, high value) | t2 < e always, so the empty-type test never fires; the empty-type assert kills it |
| 72 / 723 | `e == t2 + 1` to `e == t2 - 1` | test gap (group B, high value) | the same: the empty-type test never fires; the empty-type assert kills it |
| 73 / 726 | drop `if (t3 != e) { return RAW_NONE; }` in `raw_check_vars` | test gap (group B, high value) | a variable line with a fourth field is accepted. Missing assert: `raw_check_vars(&b, 10, 0, 1)` is RAW_NONE for TAB, "0", TAB, "a", TAB, "b", TAB, "c", line feed (the mutant returns 10) |
| 74 / 867 | drop `if (c >= 65) { return c - 55; }` in `raw_nibble` | test gap (group A, high value) | no test digit is an upper-case letter A-F: the vectors write lower case, so 'A' falls to `c - 48`. Missing assert: `raw_nibble(65)` is 10 and `raw_nibble(70)` is 15 (the mutant gives 17 and 22) |
| 75 / 867 | `c >= 65` to `c > 65` in `raw_nibble` | test gap (group A, high value) | differs only at 'A' itself (65): the mutant returns 17. Missing assert: `raw_nibble(65)` is 10 (this one alone kills it) |
| 76 / 867 | `c - 55` to `c + 55` in `raw_nibble` | test gap (group A, high value) | the upper-case branch is never run; the same two asserts kill it (the mutant gives 120 and 125) |
| 77 / 873 | drop `if (at + hex.len / 2 > out.len) { return false; }` in `raw_at_hex` | test gap (group J, low value) | with oversized buffers the unguarded read finds a mismatch before the end; only an exactly sized buffer that matches up to the end shows it (a trap, inferred). Missing assert: `var o : [2]u8 = [0] ** 2` built by `raw_put_hex(&o, 0, "abcd")`, `raw_at_hex(&o, 0, "abcdef")` is false (the mutant reads `o[2]`) |
| 78 / 873 | `hex.len / 2` to `hex.len * 2` in `raw_at_hex` | test gap (group A, high value) | the bound becomes four times too big, so a match that ends at the end of the buffer is refused; no test is that tight. Missing assert: the same exact 2-byte buffer, `raw_at_hex(&o, 0, "abcd")` is true |
| 79 / 873 | `>` to `>=` in `raw_at_hex` | test gap (group A, high value) | the same exact-buffer assert: at + 2 == out.len is refused by the mutant |
| 80 / 876 | drop `if (out[at + i] != raw_nibble(hex[2 * i]) * 16 + raw_nibble(hex[2 * i + 1])) { return false; }` in `raw_at_hex` | test gap (group A, high value) | the byte compare itself: without it `raw_at_hex` says true whenever the bytes fit, and every assert in `the_binary_file_is_ngspices_byte_for_byte` is positive, so the test passes against a function that checks nothing. Missing asserts: `raw_at_hex(&o, 0, "abce")` is false on the exact buffer above; and in the binary byte-for-byte test, `out[n - 1] = out[n - 1] + 1;` then `raw_at_hex(&out, n - want[4].len / 2, want[4])` is false, mirroring lines 1006-1008 of the ASCII twin; hand-derived and unverified |
| 81 / 917 | `j < RLC_NVARS` to `j <= RLC_NVARS` in `raw_cell_trace` | equivalent | the extra write is `vals[5] = 0.0`, and the first step (k = 0) overwrites `vals[5]` with the time of point 1 |
| 82 / 938 | `sm_f64(y2) / RLC_ZS` to `* RLC_ZS` in `raw_cell_trace` | test gap (group H, low value) | the current column is divided by 100 Ohm to leave volts and amps; the mutant makes it 10^4 times larger. The fold pins the state words, not this column, and `raw_binary_value == vals[i]` compares the trace to itself; the test asserts only `imax > 0.0` and `imin < 0.0`. Missing assert: `imax < 0.01` and `imax > 0.001` over column 4 of the trace (the peak is about 5.5 mA from the circuit's R, L, C and the 1 V step, an estimate not computed from the samples; the mutant's peak is about 55; take the window from the lab) |
| 83 / 954 | drop `if (s != RLC_EXP_S) { return 0; }` in `raw_rlc_file` | dead line | `raw_rlc_file` takes no tables, so it always runs the cell's own tables, and the test `the_rlc_trace_is_the_cells_own_run` already asserts that fold equals RLC_EXP_S; the guard can fire only after a future edit of linear.t27 that this test would report first, and no call can drive it. Keep or delete is the owner's call |

Counts of the 83 survivors: 19 equivalent (79; 157, 163, 167; 228; 240; 297; 333, 338, 347; 645; 665; 683 flip; 690; 719; 720 flip; 722; 723 flip; 917), 62 test gap (group A: 867 x3, 873 swap-arith and flip, 876 = 6; group B: 715 x2 (drop, swap-logic), 717 x2, 720 x2 (drop, swap-logic), 723 x7 (rows 65 and 67-72: drop-guard, swap-logic x2, drop-step x2, swap-arith x2), 726 = 14; group C: 301, 355 = 2; group D: 624 x2, 628, 629 swap-logic, 630, 636, 638 x2 = 8; group E: 685 x2, 688, 691 x2, 701 x2 = 7; group F: 387 flip, 609, 615, 623, 629 flip, 646, 654 x2, 657, 663, 665 swap-logic = 11; group G: 230, 233 = 2; group H: 938 = 1; group I: 258 x2 = 2; group J: 140, 270, 387 drop, 543 x2, 560, 571, 715 flip, 873 drop = 9), 2 dead line (683 drop, 954), 0 not decided.

Not run, so inferred: every "missing assert" value is hand arithmetic from the function bodies (digit counts, byte lengths such as 13 for a variable line and 21 for the zero text, the error code each mutant would return), labelled hand-derived and unverified; the lab must confirm each before an assert is written. Group J rests on one premise, that the tests run in a build that traps on an out-of-range index, an underflow and a divide by zero (two earlier censuses infer it from their survivor lists; nothing here confirms it): if the build does not trap, those 9 rows become equivalent (28 equivalent, 53 test gap). Group I needs a double whose 16 rounded digits are all 9; none is named, and it is a lab search. Group H's peak current of 5.5 mA is an estimate from the circuit, not from the trace. The equivalent class rests on reading the helpers (`raw_pow(base, 0)` is 1, `raw_mul` by 1 changes nothing, `raw_eol` does not read when from >= n, the high bit of m at be = 1), not on running them. A lab finding that a group F or G assert cannot be made to differ would move that row to equivalent.

Follow-up (one issue per test-gap group, highest value first; none filed here):

1. "analog: give raw_at_hex a negative control and exact-size cases; test upper-case hex" -- group A, kills 6 mutants (867 x3, 873 swap-arith and flip, 876); the binary byte-for-byte test passes today against a `raw_at_hex` that checks nothing; no lab value needed.
2. "analog: raw_check_vars refuses each malformed variable line" -- group B, kills 14 mutants (715 x2, 717 x2, 720 x2, 723 x7, 726); six hand-built lines, no lab value.
3. "analog: raw_put_f64 of zero and raw_from_bits of the non-finite patterns" -- group C, kills 2 mutants (301, 355); the zero text is one visible byte on every zero the writer prints.
4. "analog: header counts and numbers at their limits (raw_uint, raw_count_line)" -- group D, kills 8 mutants (624 x2, 628, 629 swap-logic, 630, 636, 638 x2); no lab value.
5. "analog: ASCII body shape, truncated and malformed point lines (raw_check_ascii)" -- group E, kills 7 mutants (685 x2, 688, 691 x2, 701 x2); four tiny hand-built bodies.
6. "analog: reader helpers on tight ranges (raw_at, raw_eol, raw_has, raw_uint, raw_value_ok)" -- group F, kills 11 mutants; low value, each assert is one line.
7. "analog: raw_sig must not read stale scratch digits" -- group G, kills 2 mutants (230, 233); two calls with a pre-set d[16] and d[17].
8. "analog: pin the scale of the RLC current column" -- group H, kills 1 mutant (938); one value window from the lab.
9. "analog: raw_sig carry-out of sixteen 9s" -- group I, kills 2 mutants (258 x2); needs a double from the lab.
10. "analog: exact-size and short-input calls of the writers (needs a trapping build)" -- group J, kills 9 mutants (140, 270, 387 drop, 543 x2, 560, 571, 715 flip, 873 drop); low value, and only if the build traps.

The two dead-line decisions (683: the early `at >= n` return that line 685 already covers; 954: the fold guard in `raw_rlc_file`, which no call can reach) need an owner's call, keep or delete, not an assert.

### Proposed issues

Each needs the fourth heading the issue-shape gate wants (User Scenarios) added by whoever files it. All ten touch `specs/analog/raw_writer.t27`, so the Queen runs them one after the other; each Boundary names its own new `test` block, and none changes a function.

1. Title: analog: give raw_at_hex a negative control and exact-size cases (group A)
   - Boundary: `specs/analog/raw_writer.t27 :: test raw_at_hex_checks_its_bytes` (new); the existing binary byte-for-byte test gains one changed byte and one negated assert.
   - Boundary: no function changes; `raw_nibble`, `raw_at_hex`, `raw_put_hex` are read only.
   - Requirements: assert `raw_nibble(65)` is 10 and `raw_nibble(70)` is 15; on an exact `[2]u8` built with `raw_put_hex(&o, 0, "abcd")` assert `raw_at_hex(&o, 0, "abcd")` and `!raw_at_hex(&o, 0, "abce")`.
   - Requirements: in `the_binary_file_is_ngspices_byte_for_byte`, add `out[n - 1] = out[n - 1] + 1;` and `assert(!raw_at_hex(&out, n - want[4].len / 2, want[4]));`, as lines 1006-1008 do for ASCII.
   - Success Criteria: a mutate run of the spec on the lab kills rows 74, 75, 76, 78, 79, 80 (6 mutants).
   - Success Criteria: every existing test still passes, and the expected values are checked on the lab before the commit.

2. Title: analog: raw_check_vars refuses each malformed variable line (group B)
   - Boundary: `specs/analog/raw_writer.t27 :: test raw_check_vars_refuses_malformed_lines` (new); no function changes.
   - Boundary: direct calls `raw_check_vars(&b, n, 0, 1)` on a 32-byte zero-filled buffer filled with `raw_puts`.
   - Requirements: RAW_NONE for six lines: no leading TAB ("x0" TAB "time" TAB "time" LF), no final LF (n = 12), wrong index (1 for 0), empty name, empty type, a fourth field; and 13 for the good line TAB "0" TAB "time" TAB "time" LF.
   - Requirements: the expected values are hand-derived; confirm them on the lab before the commit.
   - Success Criteria: rows 55, 57, 58, 59, 61, 63, 65, 67, 68, 69, 70, 71, 72, 73 (14 mutants) are killed on a lab mutate run.
   - Success Criteria: no existing test changes.

3. Title: analog: raw_put_f64 of zero and raw_from_bits of the non-finite patterns (group C)
   - Boundary: `specs/analog/raw_writer.t27 :: test zero_is_written_unsigned_and_nonfinite_bits_read_as_zero` (new); no function changes.
   - Boundary: uses the 90-limb and 810-digit scratch arrays of the neighbouring tests.
   - Requirements: `raw_put_f64(&b, 0, 0.0, &a, &d)` returns 21 and `raw_at(&b, 0, "0.000000000000000e+00")`; `raw_from_bits(0x7FF0000000000000) == 0.0` and `raw_from_bits(0x7FF8000000000000) == 0.0`.
   - Requirements: expected values hand-derived; confirm on the lab.
   - Success Criteria: rows 14 and 18 (2 mutants) are killed on a lab mutate run.
   - Success Criteria: no existing test changes.

4. Title: analog: header counts and numbers at their limits (group D)
   - Boundary: `specs/analog/raw_writer.t27 :: test numbers_and_counts_at_their_limits` (new); no function changes.
   - Boundary: direct calls of `raw_uint` and `raw_count_line` on a 32-byte zero-filled buffer.
   - Requirements: `raw_uint` is RAW_BAD for "1000000000000000000" (19 digits), for an empty range and for "12x" over [0, 3); `raw_count_line` is RAW_BAD for "No. Variablez: 1" and for "No. Variables: 1000000001", and 1000000000 for "No. Points: 1000000000".
   - Requirements: expected values hand-derived; confirm on the lab.
   - Success Criteria: rows 28, 29, 30, 32, 33, 34, 35, 36 (8 mutants) are killed on a lab mutate run.
   - Success Criteria: no existing test changes.

5. Title: analog: ASCII body shape, truncated and malformed point lines (group E)
   - Boundary: `specs/analog/raw_writer.t27 :: test the_ascii_body_is_judged_line_by_line` (new); no function changes.
   - Boundary: direct calls `raw_check_ascii(&b, n, 0, 1, 1)` on tiny bodies.
   - Requirements: RAW_E_SHORT for " 0" and for " 0" TAB "1.0e+00" LF (no empty line); RAW_E_INDEX for "x0" TAB "1.0e+00" LF LF and for " 0" LF LF.
   - Requirements: expected codes hand-derived; confirm on the lab.
   - Success Criteria: rows 47, 48, 49, 51, 52, 53, 54 (7 mutants) are killed on a lab mutate run.
   - Success Criteria: no existing test changes.

6. Title: analog: reader helpers on tight ranges (group F)
   - Boundary: `specs/analog/raw_writer.t27 :: test reader_helpers_stop_at_their_range` (new); no function changes.
   - Boundary: direct calls on short strings in a 32-byte zero-filled buffer; one exact `[3]u8` for `raw_at`.
   - Requirements: `raw_at` of "abc" at 0 on the exact buffer is true; `raw_eol(&b, 2, 0)` is 2 and `raw_has(&b, 2, 0, "abz")` is false for "abz"; `raw_uint` is 12 for "123" over [0, 2) and "12  " over [0, 3).
   - Requirements: `raw_value_ok` is false for "abc", true for "1." over [0, 1), "1e+5", "1.57" over [0, 3), "1e" over [0, 1), "1e5", and false for "1x5".
   - Success Criteria: rows 20, 25, 26, 27, 31, 38, 39, 40, 41, 42, 44 (11 mutants) are killed on a lab mutate run.
   - Success Criteria: no existing test changes.

7. Title: analog: raw_sig must not read stale scratch digits (group G)
   - Boundary: `specs/analog/raw_writer.t27 :: test raw_sig_ignores_digits_left_by_the_last_call` (new); no function changes.
   - Boundary: its own `a` and `d` arrays.
   - Requirements: with d[16] = 9, `raw_sig(9007199254740992.0, &a, &d)` returns 15 and d[15] is 2; with d[17] = 7, `raw_sig(3000000000000000.5, &a, &d)` returns 15 and d[15] is 0.
   - Requirements: expected values hand-derived; confirm on the lab.
   - Success Criteria: rows 7 and 8 (2 mutants) are killed on a lab mutate run.
   - Success Criteria: no existing test changes.

8. Title: analog: pin the scale of the RLC current column (group H)
   - Boundary: `specs/analog/raw_writer.t27 :: test the_rlc_current_is_in_amps` (new); no function changes.
   - Boundary: the 1005-value trace buffer of `the_rlc_trace_is_the_cells_own_run`.
   - Requirements: take the peak of column 4 over the 201 points and assert it lies in a window around the lab-measured peak (about 5.5 mA is the estimate, not a measurement).
   - Requirements: the window is set from a lab run, not from this file.
   - Success Criteria: row 82 (1 mutant) is killed on a lab mutate run.
   - Success Criteria: no existing test changes.

9. Title: analog: raw_sig carry-out of sixteen 9s (group I)
   - Boundary: `specs/analog/raw_writer.t27 :: test raw_sig_carries_out_of_sixteen_nines` (new); no function changes.
   - Boundary: its own `a` and `d` arrays; the double is a constant found on the lab.
   - Requirements: find on the lab a double whose %.15e text rounds up to a power of ten (search the neighbours of 10^N); assert `raw_sig` returns N + 1 with d[0] = 1 and d[1..15] = 0, and that `raw_put_f64` prints "1.000000000000000e+" followed by the exponent.
   - Requirements: record the double's bits and the Python/C reference text in the spec comment, as the other vectors do.
   - Success Criteria: rows 10 and 11 (2 mutants) are killed on a lab mutate run.
   - Success Criteria: no existing test changes.

10. Title: analog: exact-size and short-input calls of the writers (group J, needs a trapping build)
   - Boundary: `specs/analog/raw_writer.t27 :: test the_writers_stay_inside_their_buffers` (new); no function changes.
   - Boundary: file this only after one scratch test on the lab confirms that a mutant of this kind traps.
   - Requirements: `raw_mul` of a 1-limb array [999999999] by 2 returns 2; `raw_puts` of "abc" into a `[2]u8` returns 3; `raw_at` and `raw_at_hex` past the end of an exact buffer are false; `raw_put_ascii` and `raw_put_binary` with too few values write zeros; `raw_write` with empty `vars` passes `raw_check`; `raw_check_vars(&b, 3, 3, 1)` on an exact `[3]u8` is RAW_NONE.
   - Requirements: expected values hand-derived; confirm on the lab.
   - Success Criteria: rows 2, 12, 19, 21, 22, 23, 24, 56, 77 (9 mutants) are killed on a lab mutate run.
   - Success Criteria: if the lab shows that the build does not trap, close the issue and move the 9 rows to equivalent.

### Not claimed

- That any expected value above is right: each is hand-derived and unverified, and none was run.
- That the Debug build traps on an out-of-range index, an underflow or a divide by zero: inferred from two earlier censuses; group J depends on it.
- That any double produces a carry-out in `raw_sig`: none is named; group I needs the lab.
- That the RLC peak current is 5.5 mA: an estimate from the circuit, not from the trace.
- That the unmutated parts of `raw_check` are tested: the multi-line returns for the title, date, plot, flags, variable-count and point-count errors (RAW_E_TITLE, RAW_E_DATE, RAW_E_PLOT, RAW_E_FLAGS, RAW_E_NVARS, RAW_E_NPTS) are not census survivors, so this file says nothing about them; a read of the tests shows no assert on RAW_E_DATE, RAW_E_PLOT, RAW_E_NVARS or RAW_E_NPTS, which is an observation, not a measured gap.
- That the 31 hung mutants are covered: they are the mutant's hang, and they say nothing about the tests.
- That the census and this reading used the same build of t27c: the census ran with t27c 0.5.2, build commit unknown, and only the spec text was compared.
