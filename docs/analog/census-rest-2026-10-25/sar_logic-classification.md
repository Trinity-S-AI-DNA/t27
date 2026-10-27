# sar_logic.t27: the 108 mutation survivors, classified by reading

Date: 2026-10-25

Spec read: specs/analog/sar_logic.t27 (1686 lines; see "Not claimed" for how this copy relates to the censused one)

Census: the raw output of the `tri mutate spec` run on that spec (677 mutants: 526 killed, 108 survived, 31 hung, 12 unviable); the output file is not kept in the repo

Read by a subagent; nothing was run; expected values are hand-derived and unverified until a T27 OS pane runs them.

## Summary

| Class | Count | Meaning |
| --- | --- | --- |
| EQUIVALENT | 27 | the mutant cannot change any observable result |
| TEST-GAP | 80 | the mutant changes behaviour a test could observe; a smallest assert is given |
| DEAD | 1 | the line can never run, or its result is overwritten; the fix is removal |
| UNDECIDED | 0 | none: every survivor could be settled by reading |
| Total | 108 | matches the census line "108 SURVIVED" |

All 108 were reached; no line was skipped. By operator the census lists drop-guard 13, drop-step 4, flip-cmp 53, swap-arith 27, swap-logic 11 (checked by counting the operator tags in the first 329 lines of the census).

The 31 HUNG mutants (all drop-step) are out of scope: the brief covers the 108 survivors only.

Numbering: "#N" is the Nth survivor in census file order (1 to 108), the same order as the tables below. A "drop-guard" mutant is the census's "(line dropped)": the whole one-line `if` statement is removed, so execution falls through to the next statement.

### How the classes were applied

- EQUIVALENT is strict. It means that no state reachable through the sl_* functions on a converter-built deck, under any 0, 1 or X stimulus, changes a value anything reads. A difference that only a hand-built state could show, or only a contract violation could show, is EQUIVALENT with the caveat written in the row (#18, #60, #77). A difference in a bookkeeping slot nobody reads (a PREV slot at or beyond n_in, a spare net or ghost slot, a counter above zero versus zero) is EQUIVALENT.
- A difference that a legal but contrived deck, an input driven to X, or a direct call of a pure function (sl_pin_load, sl_row, sl_age, sl_within) can show is TEST-GAP, and the row says so.
- TEST-GAP tags: "fixture" means an existing test already runs the code and a tighter or new assert on a value it already computes would kill the mutant; "direct" means a direct call of a small function with hand-chosen arguments; "low" means the kill needs a contrived deck, a hand-made state vector, an input at X, or a margin that is thin; the row says which.
- DEAD means the guard can never fire for any input the code can reach.
- Every "expected" value below is "hand-derived, unverified": it was computed by hand from the spec's constants and arcs, nothing was run.

### Facts of the spec used throughout

- Drive codes S0=0, S1=1, X=2, Z=3, W0=4, W1=5. NONE = 4294967295. Gate kinds G_INV 0, G_NAND 1, G_NOR 2, G_NOR3 3, G_TGATE 4, G_DELAY 5; GATE_SLOTS 10 (kind, then pins at +1+p).
- Net numbering: the root subckt's ports are nets 0, 1, 2 and so on, then other nets in the order met. So net 0 is `en` in the hold netlist (fixture j), `a` in the glitch netlist (fixture k), and VDD in clock_gen and sar_clk.
- The converter's gate table has one ghost row at index NGATE. If `gt` starts zeroed (as every fixture's does) it reads as kind INV with every pin on net 0 and output net 0. Its DRV, PT and PV slots read 0 (S0, 0, 0), not NONE. Loops that stop at `g < NGATE` never see it; the `<=` mutants that do see it are discussed per row.
- Tests run as Zig Debug tests: an out-of-range index or an unsigned underflow panics and kills a mutant.

## sl_pin_load (lines 325-351): the load a pin puts on its net, thousandths of an inverter input

sl_loads (lines 627-649) calls it only for `p < n_in` and for a tgate's pin 3, so the output-pin branches (`return 0` after each guard) are reachable only by a direct call.

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 1 | 331 | flip-cmp `p < 2` -> `p <= 2` | TEST-GAP (direct, low) | a NAND or NOR pin 2 is the output net; the mutant gives it 1000 instead of 0, but no caller asks for an output pin of these kinds | `sl_pin_load(G_NOR, 2, SL_RISE) == 0` (mutant 1000); no fixture reaches it. P6 |
| 2 | 335 | flip-cmp `p < 3` -> `p <= 3` | TEST-GAP (direct, low) | the same for a NOR3's output pin 3 | `sl_pin_load(G_NOR3, 3, SL_RISE) == 0` (mutant 1000). P6 |
| 3 | 340 | drop-guard `if (p == 1) { return 250; }` | TEST-GAP (fixture, weak) | a tgate's pin 1 (nCLK on x1 and x7 of the dff) then falls to `return 0`, so each nCLK load is 250 short; the shift is about 8 ps on a 100 ps edge (327 * 250 / 10000, hand-derived), inside the 30 to 80 permille bounds of every fixture | `sl_pin_load(G_TGATE, 1, SL_RISE) == 250` (mutant 0). P6 |
| 4 | 349 | drop-guard `if (p < 4) { return 250; }` | TEST-GAP (direct, low) | delay-cell pins 1 to 3 (A, B, C) would load 0; in every converter deck those nets are driven by the stimulus, and sl_delay reads LDR and LDF only of a gate's own output net, so nothing reads the difference | `sl_pin_load(G_DELAY, 1, SL_RISE) == 250` (mutant 0). P6 |
| 5 | 349 | flip-cmp `p < 4` -> `p <= 4` | TEST-GAP (direct, low) | the delay cell's pin 4 is its output net; the mutant gives it 250 instead of 0 | `sl_pin_load(G_DELAY, 4, SL_RISE) == 0` (mutant 250). P6 |

One test, "pin loads", kills #1 to #5 (see P6 in the grouped list).

## sl_row (lines 354-361): the arc row of a gate whose pin p changed

No fixture builds a NAND (the deck text has no nand), so the NAND arm is used by nothing. The tgate pass arc (row 8, `p == 2`) is exercised by no asserted delay in any fixture: the mutant survived all of them (this is inferred from the survival, not traced).

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 6 | 356 | drop-guard `if (kind == G_NAND) { return 1 + p; }` | TEST-GAP (direct) | a NAND then falls to the tgate tail and gets row 9 | `sl_row(G_NAND, 1) == 2` (mutant 9). P11 |
| 7 | 356 | swap-arith `1 + p` -> `1 - p` | TEST-GAP (direct) | p = 0 gives 1 either way; p = 1 gives 0 instead of 2 (p = 2 would underflow and panic) | `sl_row(G_NAND, 1) == 2` (mutant 0); the same assert as #6. P11 |
| 8 | 359 | drop-guard `if (p == 2) { return 8; }` | TEST-GAP (direct and fixture) | the tgate pass arc becomes row 9, the enable arc | `sl_row(G_TGATE, 2) == 8`; or the pass-arc run in P3: q falls at 10064 (hand-derived, unverified) where the mutant gives 10078. P11, P3 |

## sl_want (lines 412-423): what a gate wants to drive

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 9 | 415 | drop-guard `if (kind == G_INV) { return sl_not(v0); }` | EQUIVALENT | with the line gone an inverter reads v1 and v2 (its output net and VDD, both valid indexes), matches none of the NAND, NOR, NOR3, TGATE arms, and reaches the final `return sl_not(v0)` at line 422, which is the same value; the line is redundant with the fall-through |  |
| 10 | 417 | drop-guard `if (kind == G_NAND) { return sl_nand_code(v0, v1); }` | TEST-GAP (direct) | a NAND then falls through to `sl_not(v0)`; no circuit has a NAND | hand-built gt row of kind G_NAND, pins on nets 0, 1, 2, VAL[0] = 1, VAL[1] = 0: `sl_want(...) == SL_S1` (mutant `sl_not(1)` = SL_S0); a one-NAND deck with `a=1, b=0` does the same. P11 |

## sl_trigger (lines 443-455): the input pin that made gate g change

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 11 | 448 | flip-cmp `j < n_in` -> `j <= n_in` | TEST-GAP (direct, low) | the extra pin n_in compares PREV[g*7+n_in] (never written, so it stays X from sl_init) with the gate's own output net; when no real pin differs the mutant returns n_in instead of the fallback 0 (line 452, which has no mutant); needs a gate rescheduled with no input changed | direct call: hold or chain gt, PREV equal to VAL on the real pins, VAL[out] = 1: `sl_trigger(...) == 0` (mutant n_in); no fixture shown to reach it. P20 |
| 12 | 449 | swap-logic `hit == NONE and` -> `or` | TEST-GAP (fixture) | the mutant keeps the LAST differing pin instead of the first; they differ when two pins change at once; at t = 0 of the hold fixture en, nen and d all change from X, the correct trigger is pin 0 (enable arc, row 9), the mutant's is pin 2 (pass arc, row 8) | hold netlist, d = 1, en = 0, nen = 1 at t = 0: `sl_edge(&ms, q, 1, 1, 0) == 137` (mutant 61); the existing fixture j only asserts `tq > 15000` and `tq < 15400` for the fall. P3 |
| 13 | 449 | swap-arith `ms[base + j]` -> `base - j` | TEST-GAP (fixture) | for j >= 1 the PREV read lands in the previous gate's slots (or below PREV for gate 0), so a tgate whose pass pin (2) changed is reported as pin 1 and then pin 0: the enable arc replaces the pass arc | pass-arc run (P3): hold netlist with en = 0, nen = 1 held, d = 1 at 0, d = 0 at 10000, run to 20000: `sl_edge(&ms, q, 0, 1, 1000) == 10064` (mutant 10078). P3 |
| 14 | 453 | flip-cmp `hit == 1` -> `hit != 1` | TEST-GAP (fixture) | a tgate whose trigger is pin 2 becomes 0 (row 9 instead of row 8); pin 0 and pin 1 give row 9 either way | the same pass-arc run as #13: fall at 10064, mutant 10078. P3 |

## sl_h_get (lines 458-475): the history table by linear interpolation

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 15 | 459 | flip-cmp `age <= HIST[tab]` -> `<` | EQUIVALENT | at age equal to the first table age the loop takes interval 1 with a0 equal to age, giving d0 + 0, which is the value the early return gives; the tables are continuous at their points |  |
| 16 | 465 | flip-cmp `age <= a1` -> `<` | EQUIVALENT | at age equal to a1 the mutant moves to the next interval, where the age is that interval's start and the result is its d0, which is this interval's d1 = d0 + (d1 - d0) * (a1 - a0) / (a1 - a0) exactly; for the last point the unfound default is the last delay, the same value |  |

## sl_age (lines 478-482)

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 17 | 480 | flip-cmp `c > t` -> `c >= t` | TEST-GAP (direct, low) | at c == t the correct age is 0, the mutant returns SL_AGE_INF; it needs a gate whose output changed at t and is re-evaluated in the same step | `ms[SL_TCH + g] = 5000; sl_age(&ms, g, 5000) == 0` (mutant 1000000). P21 |

## sl_high_in (lines 485-493)

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 18 | 488 | flip-cmp `j < n_in` -> `j <= n_in` | EQUIVALENT (caveat) | the only caller (line 528) asks about the nor3 that feeds a delay cell, at the moment the cell RISES; a rising cell means VAL of the cell's IN pin, which is the nor3's output net (the extra pin), is 0, so the extra term adds nothing. A hand-made direct call with that net at 1 and one input high would differ (1 vs 2 high), which no sl_* sequence on a converter deck produces |  |

## sl_delay (lines 503-563): the propagation delay of a change

Lines 516 and 517 are the tgate strength-only rule: a tgate that keeps its value and only changes strength takes 1 ps. One test, P1, kills #19 to #27.

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 19 | 516 | flip-cmp `ask == SL_S1` -> `!=` | TEST-GAP (direct) | with cur = W1 and ask = S1 the rule no longer fires, the full rise arc is returned | direct `sl_delay` on the hold netlist, `ms[SL_DRV+0] = SL_W1`, ask SL_S1: `== 1` (mutant 137). P1 |
| 20 | 517 | drop-guard (the whole S0 rule) | TEST-GAP (direct) | the S0/W0 strength-only rule is removed | DRV = W0, ask = S0: `== 1` (mutant: the 78 ps enable fall). P1 |
| 21 | 517 | flip-cmp `cur == SL_S0` -> `!=` | TEST-GAP (direct) | fails for cur = S0 with ask = W0 | DRV = S0, ask = W0: `== 1`. P1 |
| 22 | 517 | swap-logic `cur == S0 or cur == W0` -> `and` | TEST-GAP (direct) | never true | DRV = W0, ask = S0: `== 1`. P1 |
| 23 | 517 | flip-cmp `cur == SL_W0` -> `!=` | TEST-GAP (direct) | fails for cur = W0 | DRV = W0, ask = S0: `== 1`. P1 |
| 24 | 517 | swap-logic middle `and` -> `or` | TEST-GAP (fixture and direct) | any S0/W0 ask returns 1, so a tgate leaving Z towards 0 is instant; the existing fixture j falls q from Z at 15000 and asserts only `tq > 15000` and `tq < 15400`, which 15001 passes | DRV = Z, ask = S0: `== 78` (mutant 1); or tighten fixture j to `tq == 15078`. P1 |
| 25 | 517 | flip-cmp `ask == SL_S0` -> `!=` | TEST-GAP (direct) | fails for ask = S0 | DRV = W0, ask = S0: `== 1`. P1 |
| 26 | 517 | swap-logic `ask == S0 or ask == W0` -> `and` | TEST-GAP (direct) | never true | DRV = W0, ask = S0: `== 1`. P1 |
| 27 | 517 | flip-cmp `ask == SL_W0` -> `!=` | TEST-GAP (direct) | fails for ask = W0 | DRV = S0, ask = W0: `== 1`. P1 |
| 28 | 539 | swap-arith `load - 150` -> `+ 150` | TEST-GAP (fixture) | the tgate's own output-pin load (150) is subtracted before the arc; the mutant adds it; q in fixture j has load 1150 (the inverter 1000 plus the tgate's pin 3) so the arc sees 1000, the mutant 1300 | hold netlist: first q rise `== 137` (85 + 527 * 1000 / 10000; mutant 153) and fall `sl_edge(&ms, q, 0, 1, 1000) == 15078` (49 + 297 * 1000 / 10000; mutant 15087). P3 |
| 29 | 548 | drop-step `sl_age(ms, cell - 1, t)` -> `cell` | TEST-GAP (direct, low) | the age is read from the gate after the delay cell (the inverter) instead of the cell; in the direct setup below the inverter has no recorded change (TCH = NONE), so the mutant gets AGE_INF and the unscaled 197 + 2155 * 33380 / 10000 = 7390 ps; in a real chain run the inverter toggles with every pulse, so the mutant reads a real but different age, and the mutant survived 29 chains at 3 to 8 percent bounds (inferred from the survival, not traced) | direct call, clock_gen chain: nor3 g = 0, trigger Valid (PREV[1] != VAL), `ms[SL_DRV+1] = SL_S1`, `ms[SL_TCH+1] = t - 5000`, `sl_delay(ms, gt, 0, SL_S1, t) < 7390`; the exact value is about 6008 per an earlier hand-derivation that was not repeated. P22 |
| 30 | 556 | drop-step `fed - 1` -> `fed` | TEST-GAP (fixture, weak) | the trim code is read from the inverter itself (pins vout, vdd, vss, giving 2 or 6) instead of the delay cell (A, B, C): 466 or 436 instead of 626 for code 0 | chain netlist after the stimulus has run (vdd = 1, A = B = C = 0): `sl_delay(&ms, &gt, 2, SL_S1, t) == 626` (hand-derived: arc + 626 - arc at 600 cancels when the load is 600). P7 |
| 31 | 556 | swap-arith `fed - 1` -> `fed + 1` | TEST-GAP (fixture, weak) | gate 3 is the ghost row; its pins are net 0 = VDD = 1, which gives code 7 and SL_IV_R[7] = 657; needs the run applied so VDD reads 1 | the same assert (mutant 657). P7 |
| 32 | 556 | swap-arith `- sl_arc(0, RISE, EXT_8F)` -> `+` | TEST-GAP (fixture, weak) | the correction subtracts the fast-edge arc at 8 fF; the mutant adds it | the same assert (mutant 626 + 2 * 91 = 808, hand-derived). P7 |
| 33 | 561 | drop-guard `if (d < 1) { return 1; }` | DEAD | d is a usize; the smallest unscaled arc has d0 >= 17 (SL_ARC row 8 rise), the history scales (only on a nor3 that drives a cell) have a ratio of at least about 590 / 2672 = 0.22 on loads of 33380 or more, and the inverter-after-cell terms add 345 or 31; so d never reaches 0; remove the guard or turn it into an assert |  |
| 34 | 561 | flip-cmp `d < 1` -> `d <= 1` | EQUIVALENT | at d == 1 both return 1; the guard is dead anyway (#33) |  |

## sl_net (lines 568-581): look a net up by name in the converter's table

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 35 | 570 | flip-cmp `n < gt[GT_NNET]` -> `n <= gt[GT_NNET]` | TEST-GAP (direct, low) | the extra pass reads the spare table entry NNET. In a zeroed `gt` its a and b are both 0, so `b - a == 0`, which equals only the length of an empty name: every non-empty name (including the asserted `"nosuchnet"`) fails the length test and falls out, but the empty name returns NNET instead of NONE | `sl_net(&w, &gt, "") == NONE` on any built deck (mutant: NNET). Assumes `gt` is zeroed past the table, as every fixture's is, and that the empty literal slices cleanly (not run). This is the P23 test. P23 |

## sl_init (lines 584-603): the model at t = 0

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 36 | 587 | flip-cmp `n < NNET` -> `<=` | EQUIVALENT | the extra write is VAL[NNET] = X, a spare slot: every net number that is ever read comes from a pin or a table entry below NNET. (At 128 nets the write would land on CNT[0], which sl_step clears before it counts, so it is harmless there too) |  |
| 37 | 592 | flip-cmp `g < NGATE` -> `<=` | EQUIVALENT | the extra pass initialises the ghost row (DRV, PT, TCH, PREV of index NGATE); every loop that reads those tables stops at `g < NGATE`, and nothing indexes the ghost from outside (the only way to reach it is mutant #46 or #31) |  |
| 38 | 593 | swap-arith `SL_DRV + g` -> `SL_DRV - g` | TEST-GAP (direct) | X is written to DRV[0] and then to 1280 - g (the LDF table, rewritten by sl_loads, or a spare slot), so DRV[g] for g >= 1 keeps the zeroed 0, which is S0: every gate but the first starts as a strong 0 instead of X. The fixtures settle to the same state, so none notices | right after `sl_init` on any deck with at least two gates (the 2-gate hold deck will do): `ms[SL_DRV + 1] == SL_X` (mutant 0). Part of P8 |
| 39 | 594 | swap-arith `SL_PT + g` -> `SL_PT - g` | TEST-GAP (direct) | PT[g] for g >= 1 keeps 0 instead of NONE, so each of those gates matures at t = 0 with PV = 0 and takes S0; NONE lands in DRV[160 - g], which that same maturity step overwrites. Same end state in every fixture | `ms[SL_PT + 1] == NONE` after `sl_init` (mutant 0). Part of P8 |
| 40 | 597 | flip-cmp `j < 7` -> `j <= 7` | EQUIVALENT | the extra write PREV[g * 7 + 7] is slot 0 of the next gate, which the next pass rewrites with the same X right after; for the last gate it is the ghost's slot 0, which nothing reads |  |
| 41 | 598 | swap-arith `g * 7 + j` -> `g / 7 + j` | TEST-GAP (direct) | only slots 0 to about 22 are written (g / 7 + j); every later gate keeps PREV = 0 instead of X until its first evaluation rewrites slots j < n_in. The one reader before that is sl_trigger on the first evaluation, and no fixture is sensitive to it | on the 2-gate hold deck after `sl_init`: `ms[SL_PREV + 7] == SL_X` (the last gate's slot 0; mutant 0). Part of P8 |
| 42 | 598 | swap-arith `g * 7 + j` -> `g * 7 - j` | TEST-GAP (direct) | slot j of gate g goes to g * 7 - j, which is slot 7 - j of the previous gate for j >= 1; the last gate's slots 1 to 6 are never written and keep 0. Slots j >= n_in are never read, and slots j < n_in are rewritten by the first sl_sched, so only a direct post-condition sees it | on the 2-gate hold deck after `sl_init`: `ms[SL_PREV + 7 + 1] == SL_X` (the last gate's slot 1; mutant 0). Part of P8 |

## sl_links (lines 607-624): which gate feeds a delay cell, and which delay cell feeds an inverter

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 43 | 609 | flip-cmp `g < NGATE` -> `<=` | EQUIVALENT | the extra pass fills CELL[NGATE] and FED[NGATE], the ghost's slots, which nothing reads |  |
| 44 | 610 | swap-arith `SL_CELL + g` -> `SL_CELL - g` | TEST-GAP (direct, low) | CELL[g] is no longer reset to 0 before the h loop (visible only when ms already holds links), and the 0 goes to ms[5120 - g] = TCH[160 - g]. Because sl_build calls sl_init and then sl_loads, on the 113-gate sar deck this puts 0 ("changed at t = 0") into TCH[110] and its neighbours instead of NONE; a gate's first maturity out of X resets TCH to NONE (line 745), so no run reads the stale value | on a built 2-gate deck: `ms[SL_CELL + 1] = 7; ms[SL_FED + 1] = 7; sl_links(&ms, &gt); ms[SL_CELL + 1] == 0` (mutant 7; g = 1 is needed, since g = 0 makes `SL_CELL - 0` the same slot). Alternative on the sar deck right after `sl_build`: `ms[SL_TCH + 110] == NONE` (mutant 0; g = 50). Both hand-derived, unverified. P16 |
| 45 | 611 | swap-arith `SL_FED + g` -> `SL_FED - g` | TEST-GAP (direct, low) | the mirror: FED[g] is not reset, and the 0 goes to ms[5280 - g] = CELL[160 - g], which wipes a real link when the gate g' that feeds a delay cell has an index below 80 and the deck has more than 160 - g' gates (on the 113-gate sar deck: g' from 48 to 79). That the sar deck survives implies its feeder is outside that window (inferred from the survival, not traced) | the same test: `ms[SL_FED + 1] == 0` after `sl_links` (mutant 7). P16 |
| 46 | 613 | flip-cmp `h < NGATE` -> `<=` | TEST-GAP (direct, very low) | the extra h is the ghost INV (output net 0). Line 617 then sets FED[g] = NGATE + 1 for any delay cell g whose input pin is net 0, replacing the real feeder; sl_delay's two-inputs-high rule (527-528, scale by 806 / 1000) would then ask sl_high_in of the ghost, which is at most 1, and lose the scaling. In every deck the file builds, net 0 is VDD and no delay cell is fed from it | needs a new deck whose first root port is a nor3 output that feeds a delay cell; with two nor3 inputs high, the delay cell's rise must be the table value times 806 / 1000, not the table value. Expected value not derived here. P24 |
| 47 | 616 | drop-step `h + 1` -> `h` | TEST-GAP (fixture, weak) | FED of the inverter becomes the delay cell's index (1 in clock_gen) instead of index + 1 (2), so line 556 reads the trim code from gate 0, the nor3: pins 1 to 3 are Valid, CK1 and the nor3 output n1, not A, B, C. At the end of the existing run clks is high, n1 is 0 and Valid and CK1 are 0, so the nor3's code is 0 and equals the delay cell's code 0: the two readings agree. While clks is low, n1 is 1 and the nor3's code is 1: SL_IV_R[1] = 466 against SL_IV_R[0] = 626 | chain deck, trim code 000: run to t = 50000 (clks fell at 10100, n1 has risen, the cell's output has fallen) and assert `sl_delay(&ms, &gt, 2, SL_S1, 50000) == 626` (mutant 466; the arc terms cancel at the 600 load of vout). Hand-derived, unverified. P7 |

## sl_loads (lines 627-649): the load on every net, from the pins on it and the capacitance the top added

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 48 | 630 | flip-cmp `n < NNET` -> `<=` | EQUIVALENT | the extra pass writes LDR[NNET] and LDF[NNET] from EXTL[NNET] (0), spare slots that no pin points at |  |
| 49 | 631 | swap-arith `SL_LDR + n` -> `SL_LDR - n` (the write) | TEST-GAP (fixture, weak) | LDR[n] is not set (it stays 0 on a fresh ms) and EXTL[n] goes to ms[1024 - n] = EXT[128 - n], a slot beyond the deck's nets (EXT[109] and EXT[108] for CK1 and clkc of the sar deck). The net's external load is lost for rising edges: 600 on clkc and vout, 300 on CK1. The effect is a few ps, inside every fixture's permille bound | clock_gen chain after `sl_ext_load(&ms, vout, SL_EXT_8F)` and `sl_loads`: `ms[SL_LDR + vout] == 600` (mutant 0). P5 |
| 50 | 631 | swap-arith `SL_EXTL + n` -> `SL_EXTL - n` (the read) | TEST-GAP (fixture, weak) | the read lands on ms[4830 - n], which is the log area or the spare slots of a fresh ms (all 0): the same loss of the external load | the same assert (mutant 0). P5 |
| 51 | 632 | swap-arith `SL_LDF + n` -> `SL_LDF - n` (the write) | TEST-GAP (fixture, weak) | the same for falling edges: LDF[n] starts at 0 and the value goes to LDR[128 - n], which is beyond NNET, or rewritten by that net's own pass later, or written 0 over a 0 | `ms[SL_LDF + vout] == 600` (mutant 0). P5 |
| 52 | 632 | swap-arith `SL_EXTL + n` -> `SL_EXTL - n` (the read) | TEST-GAP (fixture, weak) | as #50, for the fall load | the same assert (mutant 0). P5 |
| 53 | 636 | flip-cmp `g < NGATE` -> `<=` | TEST-GAP (fixture variant) | the ghost INV puts an input load of 1000 on net 0. In every existing deck net 0 is VDD or a stimulus input, whose load nobody reads. If net 0 is a gate's output, its delay grows: the glitch deck with `y` as its first port has net 0 = y, and the unloaded inverter's fall becomes 64 + 327 * 1000 / 10000 = 96 instead of 64 | glitch deck declared as `.subckt top y a vdd vss`: `sl_edge(&ms, y, 0, 1, 500) == 2064` (mutant 2096; hand-derived, unverified). P14 |
| 54 | 639 | flip-cmp `p < 4` -> `p <= 4` | EQUIVALENT | p = 4 reaches the inner test only if `p < n_in` (n_in is at most 4) or `kind == G_TGATE and p == 3`; neither holds, so nothing is added. The pin lookup itself stays inside the 10 slots of a gate row |  |
| 55 | 641 | flip-cmp `p < sl_n_in(kind)` -> `p <= sl_n_in(kind)` | EQUIVALENT | p = n_in is the gate's output pin (INV 1, NAND and NOR 2, NOR3 3) and sl_pin_load returns 0 there; for a tgate p = 3 is already counted by the `or`; the delay cell's p = 4 is outside the loop. Adding 0 changes nothing |  |
| 56 | 641 | swap-logic `kind == G_TGATE and p == 3` -> `or` | EQUIVALENT | the new condition is also true for every tgate pin (0 to 2 already counted by `p < 3`, counted once because it is one boolean) and for p = 3 of any kind; for p = 3 sl_pin_load is 0 for INV, NAND, NOR and NOR3, and the delay cell and the tgate already include it. No pin load changes |  |

## sl_ext_load (lines 651-653)

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 57 | 652 | swap-arith `SL_EXTL + net` -> `SL_EXTL - net` | TEST-GAP (fixture, weak) | the capacitance is stored below the table (in the log area, which is rewritten as rows are logged), so sl_loads sees EXTL = 0 and the net has no external load. Same few-ps effect as #49 | `ms[SL_LDR + vout] == 600` and `ms[SL_LDF + vout] == 600` after `sl_ext_load` and `sl_loads` (mutant 0). P5 |

## sl_set (lines 662-676): set a net's value at time t and log it if the net is watched

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 58 | 666 | flip-cmp `i < ms[SL_NWATCH]` -> `<=` | TEST-GAP (direct) | the extra pass reads WATCH[NWATCH], a zeroed slot, which names net 0: every change of net 0 is logged even though it is not watched (and with no watch at all, net 0 is still logged). No test counts NLOG or asks for net 0 | `sl_watch(&ms, 3); sl_set(&ms, 0, 1, 10);` then `ms[SL_NLOG] == 0` (mutant 1; on a fresh ms VAL[0] is 0, so the set is a change). P15 |
| 59 | 667 | flip-cmp `ms[SL_NLOG] < SL_LOGMAX` -> `<=` | TEST-GAP (direct, low) | at a full log the mutant writes row 640 into the spare slots 4820 to 4822 and counts it (NLOG = 641); no fixture comes near 640 rows | `ms[SL_NLOG] = SL_LOGMAX; sl_watch(&ms, 3); sl_set(&ms, 3, 1, 10);` then `ms[SL_NLOG] == 640` (mutant 641). P15 |

## sl_drive (lines 679-682): an input net takes a value from now on

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 60 | 680 | swap-arith `SL_EXT + n` -> `SL_EXT - n` | EQUIVALENT (contract caveat) | EXT[n] stays 0 and a 1 is written into a counter slot of a net beyond NNET (896 - n); sl_step then resolves the input like any net, finds no gate counted on it and `sl_resolve` returns `old`, which is the value sl_drive just set. Equivalent only while the contract at line 678 holds: the input has no driver in the circuit. The only readers of EXT are lines 680 and 768 (grep) |  |

## sl_resolve (lines 685-709): the value a net takes from the drivers counted on it

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 61 | 687 | swap-arith `b + SL_S0` -> `b - SL_S0` | EQUIVALENT | SL_S0 is 0, so both are b |  |
| 62 | 689 | swap-arith `b + SL_X` -> `b - SL_X` | TEST-GAP (direct) | the X counter is read two slots down: the previous net's W0 counter (or VAL[126] for net 0). A net that a gate drives to X is then not seen as X; it keeps its old value. No fixture pushes a defined net back to X | `ms[SL_CNT + SL_X] = 1; sl_resolve(&ms, 0, 0) == SL_X` (mutant 0: it reads slot 126, which is 0). P25 |

## sl_sched (lines 712-734): schedule or withdraw the change of a gate (the inertial rule)

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 63 | 717 | flip-cmp `ask == cur` -> `!=` | TEST-GAP (direct) | with a change pending, a new ask equal to the current value must cancel it. The mutant reschedules a change to the same value (PT = t + 48 for a rise) and cancels when ask differs from cur. The log and the end state of the glitch fixture are the same, because a same-value maturity changes nothing; only the PT slot differs while the phantom event is pending | glitch deck, run to t = 1050 (a rose at 1000, fell at 1030, the inverter's fall was due at 1064): `ms[SL_PT + 0] == NONE` (mutant 1078; 1030 + 48, hand-derived, unverified). Run to 1050, not 1100: by 1078 the phantom has matured and PT is NONE again. P17 |
| 64 | 718 | swap-arith `SL_PT + g` -> `SL_PT - g` | TEST-GAP (fixture variant) | the cancel is written to PT[0] only when g = 0; for g >= 1 the pending change is not cancelled (and NONE lands in DRV[160 - g]). The glitch fixture's inverter is gate 0, so it cannot see this | the glitch deck with one dummy inverter placed first (so the glitched inverter is gate 1): `ms[SL_PT + 1] == NONE` at t = 1050 (mutant 1064), and the fixture's own asserts `sl_edges(&ms, y, 0, 500) == 1` and `sl_edge(&ms, y, 0, 1, 500) == 2064` then fail in the mutant, which lets the 30 ps glitch through (y falls at 1064, so 2 edges and a first edge at 1064; hand-derived, unverified). P17 |
| 65 | 730 | flip-cmp `j < sl_n_in(kind)` -> `<=` | EQUIVALENT | the extra write is PREV slot n_in, at most slot 4 of 7, which sl_trigger never reads (it reads j < n_in); the pin lookup stays inside the gate row |  |
| 66 | 731 | swap-arith `g * 7 + j` -> `g * 7 - j` | TEST-GAP (fixture) | slot j >= 1 of a two- or three-input gate is written into the previous gate's slot 7 - j and its own slot j keeps the init value X, so sl_trigger sees pin 1 or 2 as changed whenever its net is known and picks the wrong arc row. Killed by the pass-arc and enable-arc run (P3) | see P3 |

## sl_step (lines 737-778): one event time

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 67 | 739 | flip-cmp `g < NGATE` -> `<=` | EQUIVALENT | the ghost's PT slot is 0, so at the t = 0 step it matures once into DRV[NGATE] and PT[NGATE] (was = 0 and now = 0: no TCH write); later steps never match; nothing reads the ghost's slots |  |
| 68 | 744 | swap-logic first `and` -> `or` | TEST-GAP (direct, low) | TCH is also written when the old value is known and the new one is not (or when the new one is known and differs): a known-to-X maturity then records a change time that line 744 says it must not | hand-made sl_step on the clock_gen deck after `sl_init`: `ms[SL_DRV + 1] = SL_S1; ms[SL_PV + 1] = SL_X; ms[SL_PT + 1] = 5000; ms[SL_TCH + 1] = NONE; sl_step(&ms, &gt, 5000);` then `ms[SL_TCH + 1] == NONE` (mutant 5000). P13 |
| 69 | 744 | swap-logic second `and` -> `or` | TEST-GAP (direct, low) | `was != now` alone then writes TCH, so S1 to X is recorded as a change | the same setup and assert (mutant 5000). P13 |
| 70 | 745 | swap-arith `SL_TCH + g` -> `SL_TCH - g` | TEST-GAP (direct, low) | the reset to NONE after a maturity out of X lands in TCH[160 - g]; TCH[g] keeps a stale time. Fresh runs never have a stale time, since sl_init sets NONE | hand-made: `ms[SL_DRV + 1] = SL_X; ms[SL_PV + 1] = SL_S1; ms[SL_PT + 1] = 5000; ms[SL_TCH + 1] = 100; sl_step(&ms, &gt, 5000);` then `ms[SL_TCH + 1] == NONE` (mutant 100). P13 |
| 71 | 752 | flip-cmp `n < NNET` -> `<=` | EQUIVALENT | the extra pass clears the six counters of net NNET, which are 0 already and read by no one |  |
| 72 | 754 | flip-cmp `c < 6` -> `<=` | EQUIVALENT | the extra clear zeroes slot 0 of the next net, which the next pass clears anyway; for the last net it is a spare slot |  |
| 73 | 761 | flip-cmp `g < NGATE` -> `<=` | TEST-GAP (fixture variant) | the ghost INV drives net 0 with its DRV slot, which reads 0 (S0): a phantom strong 0 on net 0 at every step. Net 0 is VDD or a stimulus input in every existing deck, so the step skips its resolution. If net 0 is a gate's output, the net goes X whenever the real gate drives 1 | glitch deck with `y` first: run to 500 and assert `ms[SL_VAL + y] == 1` (mutant 2, X). P14 |
| 74 | 763 | swap-arith read `o * 6` -> `o / 6` | EQUIVALENT | the written value is (another slot) + 1 instead of (this counter) + 1, always at least 1; sl_resolve and its callers test counters only against 0, so only the zero pattern matters, and the written slot is the same |  |
| 75 | 763 | swap-arith read `o * 6 + DRV` -> `o * 6 - DRV` | EQUIVALENT | the same: only the value read differs, and it is at least 1 after the +1. SL_CNT is 128, so the read index cannot go below 0 for o = 0 and DRV <= 5 |  |
| 76 | 767 | flip-cmp `n < NNET` -> `<=` | EQUIVALENT | net NNET has no counted driver (all counters 0), so sl_resolve returns the old VAL[NNET], and sl_set sees no change and logs nothing |  |
| 77 | 768 | swap-arith read `SL_EXT + n` -> `SL_EXT - n` | EQUIVALENT (contract caveat) | the read lands on a spare counter slot (0), so inputs are resolved too; with no gate counted on an input the result is its old value. Equivalent while an input has no driver in the circuit (line 678), as #60 |  |
| 78 | 774 | flip-cmp `g < NGATE` -> `<=` | EQUIVALENT | sl_sched runs on the ghost row: it may set PT[NGATE] and PV[NGATE] and PREV[NGATE * 7 ..], but the ghost's PT is never scanned (sl_run, sl_step and every other loop stop at NGATE) and sl_delay on the ghost reads only zeroed or spare slots without a bounds error |  |

## sl_run (lines 782-808): run the model to a time, applying the stimulus edges as they fall due

The hold fixture (lines 1412-1463) has stimulus edges at 0, 5000, 10000 and 15000 and ends its runs at 4900, 14900 and 20000, so no fixture run ends at an event time (inferred from the survival of #80, and from reading those three run times). No fixture comes near the step cap SL_MAXSTEP = 200000.

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 79 | 791 | flip-cmp `ms[SL_PT + g] < nxt` -> `<=` | EQUIVALENT | on a tie nxt is assigned the value it already holds; when both are NONE it stays NONE |  |
| 80 | 794 | flip-cmp `nxt > t_end` -> `>=` | TEST-GAP (fixture variant) | an event exactly at t_end is applied by the original and left pending by the mutant | hold fixture: insert `sl_run(&ms, &gt, &s_t, &s_n, &s_v, k, 5000);` followed by `assert(ms[SL_VAL + en] == 1);` between its 4900 run and its 14900 run (the stimulus at 5000 raises en; mutant: en is still 0, the stimulus of t = 0). P9 |
| 81 | 794 | flip-cmp `ms[SL_NSTEP] >= SL_MAXSTEP` -> `>` | TEST-GAP (direct, low) | at exactly the cap the original stops and the mutant takes one more step; no fixture is near the cap | hold fixture with `ms[SL_NSTEP] = SL_MAXSTEP - 1` set before `sl_run(.., 20000)`: `ms[SL_NSTEP] == SL_MAXSTEP` (original: it takes the t = 0 step, reaches the cap and stops; mutant: SL_MAXSTEP + 1, one step more). P9 |
| 82 | 798 | flip-cmp `si < nst` -> `si <= nst` | TEST-GAP (direct, low) | once the last stimulus edge is consumed the mutant also reads s_t[nst], s_n[nst], s_v[nst]; in a zeroed array that is (time 0, net 0, value 0), applied only when the step being run is t = 0. It shows only when the whole stimulus is at t = 0 and net 0 is read afterwards; in every fixture the stimulus runs past t = 0, so s_t[si] is not 0 when the t = 0 edges are used up | hold netlist (net 0 is `en`), zeroed 16-slot stimulus arrays, one edge `sl_put(.., 0, en, 1)` (k = 1), `sl_run(.., 1, 100)`: `ms[SL_VAL + en] == 1` and `ms[SL_SI] == 1` (mutant 0 and 2: it drives net 0 to 0 once more). P9 |
| 83 | 803 | drop-step `ms[SL_NSTEP] = ms[SL_NSTEP] + 1` | TEST-GAP (direct, low) | the counter is read only by the cap at line 794 and asserted by no test; with the line dropped the cap can never fire (the run would stop only at t_end) | the same assert as #81: with the counter preset to SL_MAXSTEP - 1, `ms[SL_NSTEP] == SL_MAXSTEP` after the run (mutant SL_MAXSTEP - 1, because it is never incremented); or plainly `ms[SL_NSTEP] > 0` after any run. P9 |

## sl_edges (lines 813-822) and sl_edge (lines 825-838): read the log

sl_edges counts the logged changes of net n to value v at or after tmin; sl_edge gives the time of the nth of them, NONE when there is none. All four survivors are in the two reading loops; both loops stop at `i < ms[SL_NLOG]`, and the log rows are 3 slots each from SL_LOG.

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 84 | 816 | flip-cmp `i < ms[SL_NLOG]` -> `<=` (in sl_edges) | TEST-GAP (direct) | the extra pass reads row NLOG, which is (0, 0, 0) in a zeroed state vector: it counts as "net 0 went to 0 at time 0". It matters only when the question is about net 0, value 0 and tmin 0, which no fixture asks (net 0 is VDD, `en` or `a`, and the callers use ids of other nets) | on a fresh zeroed state vector: `sl_edges(&ms, 0, 0, 0) == 0` (mutant 1). P10 |
| 85 | 818 | flip-cmp `ms[row] >= tmin` -> `>` (in sl_edges) | TEST-GAP (direct) | a change logged exactly at tmin is counted by the original and missed by the mutant; no fixture asks with tmin equal to a logged time | `sl_watch(&ms, 5); sl_set(&ms, 5, 1, 100);` then `sl_edges(&ms, 5, 1, 100) == 1` (mutant 0). P10 |
| 86 | 829 | flip-cmp `i < ms[SL_NLOG]` -> `<=` (in sl_edge) | TEST-GAP (direct) | the same extra row (0, 0, 0): with nth = 1 it sets found to the row's time 0 instead of leaving NONE | on a fresh zeroed state vector: `sl_edge(&ms, 0, 0, 1, 0) == NONE` (mutant 0). P10 |
| 87 | 831 | flip-cmp `ms[row] >= tmin` -> `>` (in sl_edge) | TEST-GAP (direct) | as #85 | after the log of #85: `sl_edge(&ms, 5, 1, 1, 100) == 100` (mutant NONE). P10 |

## sl_absdiff (lines 841-844)

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 88 | 842 | flip-cmp `a > b` -> `>=` | EQUIVALENT | at a == b the mutant returns a - b = 0 and the original falls through to b - a = 0; for a != b the branches are unchanged. No underflow either way |  |

## sl_within (lines 847-849) and sl_within_fall (lines 852-854): the error bounds

sl_within accepts when sl_err_pm(got, ref) <= SL_CK_BOUND_PM (50 permille) and the absolute difference is <= SL_CK_ABS_PS (30 ps); sl_within_fall the same with SL_FALL_BOUND_PM (250) and SL_FALL_ABS_PS (150). sl_err_pm is the absolute difference times 1000 divided by `ask`, integer division. The fixtures call them only with measured values far from the bounds (lines 1279, 1529 to 1544).

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 89 | 848 | flip-cmp `err <= SL_CK_BOUND_PM` -> `<` | TEST-GAP (direct) | differs exactly at 50 permille | `sl_within(420, 400) == true` (difference 20 ps, error 20 * 1000 / 400 = 50; mutant false). P2 |
| 90 | 848 | swap-logic `and` -> `or` | TEST-GAP (direct) | true when only one of the two bounds holds | `sl_within(10100, 10000) == false` (difference 100 ps fails the 30 ps bound, error 10 permille passes; mutant true). P2 |
| 91 | 848 | flip-cmp `absdiff <= SL_CK_ABS_PS` -> `<` | TEST-GAP (direct) | differs exactly at 30 ps | `sl_within(1030, 1000) == true` (difference 30 ps, error 30 permille; mutant false). P2 |
| 92 | 853 | flip-cmp `err <= SL_FALL_BOUND_PM` -> `<` | TEST-GAP (direct) | differs exactly at 250 permille | `sl_within_fall(500, 400) == true` (difference 100 ps, error 250; mutant false). P2 |
| 93 | 853 | swap-logic `and` -> `or` | TEST-GAP (direct) | as #90 | `sl_within_fall(11000, 10000) == false` (difference 1000 ps fails the 150 ps bound, error 100 permille passes; mutant true). P2 |
| 94 | 853 | flip-cmp `absdiff <= SL_FALL_ABS_PS` -> `<` | TEST-GAP (direct) | differs exactly at 150 ps | `sl_within_fall(1150, 1000) == true` (difference 150 ps, error 150 permille; mutant false). P2 |

## sl_cat (lines 873-880): copy text into a buffer, cut at its end

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 95 | 875 | flip-cmp `pos + i < dst.len` -> `<=` | TEST-GAP (direct) | the guard that cuts the copy at the end of dst never binds: every fixture's buffer (src of 1024 or 4096 bytes) is larger than the text. The mutant writes dst[dst.len], an out-of-range index that panics in a Debug test | `var small : [64]u8 = undefined;` then `sl_cat(&small, 0, SL_DECK_A) == 64` (SL_DECK_A is far longer than 64 bytes, read at line 863; the original stops at 64; the mutant panics on the write at index 64). P4 |

## sl_ids (lines 903-926): the nets the runs use, by name

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 96 | 914 | flip-cmp `i < 11` -> `<= 11` | EQUIVALENT | the extra pass i = 11 builds the name "ck0" (nm[2] = 48 + (11 - 11)), looks it up with sl_net (a pure lookup, lines 568-581) and stores the result, NONE, in ids[20]; line 925 then overwrites ids[20] with clkc. The index stays inside every caller's ids (24 slots) and nothing else is written |  |

## sl_build (lines 929-944): read the netlist and build the model of sar_clk

The guard at line 932 is the only refusal path. A refusal that is easy to cause by reading: a work buffer `w` of 16 bytes. The converter's `prepare` (spice_convert.t27 lines 687-689) then refuses the first card longer than 14 bytes with Y_SIZE at line 2, `emit_gates` writes the card `line 2: ...` through `put_reject` (line 2799) and returns a positive byte count; the converter has its own test of the same refusal for `inventory` (lines 3231-3236). No test of sar_logic.t27 passes sl_build a buffer that makes it refuse (inferred from the survival of #97 and #98).

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 97 | 932 | drop-guard `if (st[ERR_LINE] != 0 or n == 0) { return false; }` | TEST-GAP (direct) | without the guard a refused netlist is built anyway: `gt` is empty, every id is NONE, and `sl_ext_load(ms, ids[19], ..)` indexes the state vector with NONE, which returns true or panics in a Debug test | `sl_build(&src, &w16, &out, &st, &gt, &ms, &ids, false) == false` with `var w16 : [16]u8` (hand-derived, unverified: the converter refuses with ERR_LINE 2). P4 |
| 98 | 932 | swap-logic `or` -> `and` | TEST-GAP (direct) | with `and` the guard returns false only when the netlist is refused AND the byte count is 0; a refusal writes at least "line " so n > 0, and the mutant carries on as in #97. The `n == 0` half of the original guard is never needed in practice (see the spec observations): refusal rests on ERR_LINE alone, so this kill relies on put_reject writing bytes, which was read, not run | the same assert as #97. P4 |
| 99 | 939 | flip-cmp `i < 12` -> `<= 12` | TEST-GAP (direct) | the extra pass watches ids[21], which the caller's zeroed ids array holds as 0: net 0, VDD. The run is not disturbed (NWATCH 13 is below the 16 watch slots) but VDD's changes are now logged | after `sl_build` on the sar deck: `ms[SL_NWATCH] == 12` (mutant 13); also `sl_edges(&ms, ids[0], 1, 0) == 0` once the stimulus has run (mutant 1: VDD rising at t = 0 is logged). P26 |

## sl_order_ok (lines 970-989): do CK11 .. CK1 each rise once, never fall after the reset, and rise in ngspice's order

The swapped-dff fixture (lines 1499-1522) reaches it with a run on which at least two of the three checks fire, which is why dropping any one of them survives. The positive control is the right netlist at line 1513.

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 100 | 973 | drop-guard `if (sl_edges(.., 1, 0) != 1) { return false; }` | TEST-GAP (direct) | a CK that never rises, or rises twice, is no longer refused here; the pair check at 983 does not catch a CK1 that never rises (its first-rise time is NONE, which sorts last, exactly where CK1 belongs) | hand-made log, see P12: eleven nets 30 .. 40 watched and raised at 1000, 2000, .. 11000 except that net 40 (CK1, the last) never rises: `sl_order_ok(..) == false` (hand-derived, unverified; mutant true). P12 |
| 101 | 974 | drop-guard `if (sl_edges(.., 0, SL_TMIN) != 0) { return false; }` | TEST-GAP (direct) | a CK that falls again after the reset is no longer refused; the other two checks look at rises only | the all-rises-in-order log plus `sl_set(&ms, 33, 0, 30000)` after net 33's rise: `sl_order_ok(..) == false` (mutant true). P12 |
| 102 | 983 | drop-guard `if ((ri < rj) != (SL_NG_TCK[i] < SL_NG_TCK[j])) { return false; }` | TEST-GAP (direct) | rises in the wrong order are no longer refused; 973 and 974 count edges only | the log with the rises of nets 32 and 34 swapped (net 32 at 5000, net 34 at 3000): `sl_order_ok(..) == false` (mutant true). P12 |

## sl_chain_build (lines 1023-1033): the same read-and-build for clock_gen

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 103 | 1026 | drop-guard `if (st[ERR_LINE] != 0 or n == 0) { return false; }` | TEST-GAP (direct) | as #97: no test makes the chain build refuse; without the guard `sl_ext_load(ms, ids[8], ..)` indexes with NONE | `sl_chain_build(&src, &w16, &out, &st, &gt, &ms, &ids) == false` with a 16-byte `w16` (hand-derived, unverified). P4 |
| 104 | 1026 | swap-logic `or` -> `and` | TEST-GAP (direct) | as #98: the `n == 0` half is never needed; the kill relies on a refusal writing a positive byte count | the same assert as #103. P4 |

## sl_chain_stim (lines 1037-1057): the stimulus of one pulse-train chain

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 105 | 1047 | swap-arith `SL_H_REST + SL_H_FALL0` -> `SL_H_REST - SL_H_FALL0` | TEST-GAP (direct) | Valid's first fall moves from 51100 to 48900. The delay cell's rise takes about 10.5 ns (SL_DC_RISE starts at 10482 ps), so the start rise of the clkc net still lands before the first pulse at 99700 and no measured edge moves | `sl_chain_stim` with wh = 20200, wl = 19800, npulse = 11, ck1_last = true into zeroed 64-slot arrays: returns 32 and `s_t[8] == 51100` (mutant 48900); see P19 |
| 106 | 1052 | swap-arith `t + 654` -> `t - 654` | TEST-GAP (direct, fixture weak) | CK1's rise is stored right after Valid's last rise but timed 654 ps before it, so the stimulus array is no longer sorted by time (sl_run expects it sorted, lines 780-781). The chain k01 fixture (line 1635) measures only two clkc falls against bounds, and survived. The run-level effect was not re-derived (see "Not claimed") | the same call: `s_t[30] == 500354` (99700 + 10 * 40000 + 654; mutant 499046) and `s_n[30] == ids[4]`, `s_t[31] == 519900`. P19 |

## sl_chain_err_f (lines 1082-1087) and sl_chain_err_r (lines 1090-1095): the error of a clkc edge against ngspice

Both return 9999 when there is no such edge or the edge comes before the Valid edge that should cause it; an edge exactly at the Valid edge has delay 0 and error 0. The fixtures test 9999 only for edges that do not exist (lines 1678-1679).

| # | Line | Operator | Class | Reason | Proposed assert |
| --- | --- | --- | --- | --- | --- |
| 107 | 1085 | flip-cmp `e < tr` -> `<=` | TEST-GAP (direct) | at e == tr the original returns sl_err_pm(0, ng) = 0, the mutant 9999 | hand-made log (P18): net 30 watched, `sl_set(&ms, 30, 1, 60000)`, `sl_set(&ms, 30, 0, 99700)`, `sl_set(&ms, 30, 1, 119900)`, ids[8] = 30; `sl_chain_err_f(&ms, &ids, 20200, 19800, 0, 100) == 0` (tr = 99700 + 0 * 40000; mutant 9999). P18 |
| 108 | 1093 | flip-cmp `e < tf` -> `<=` | TEST-GAP (direct) | as #107 for the rise caused by Valid's fall | the same log: `sl_chain_err_r(&ms, &ids, 20200, 19800, 0, 100) == 0` (the second rise at or after 50000 is 119900 = tf = 99700 + 20200; mutant 9999). P18 |

## Observations about the spec itself

Each was found by reading; none was run. Line numbers are those of the 1686-line copy.

- Line 561, `if (d < 1) { return 1; }`: dead. d is a usize and no arc, table or correction on any path can make it 0 (survivor #33); #34 is equivalent for the same reason. Remove it or turn it into an assert.
- Line 415, `if (kind == G_INV) { return sl_not(v0); }`: duplicates the fall-through `return sl_not(v0)` at line 422 (survivor #9, equivalent).
- Lines 356 and 417, the NAND row and the NAND arm of sl_want: no deck the file builds contains a NAND (line 1168 asserts the NAND count of one deck is 0), so survivors #6, #7 and #10 sit on code no circuit reaches. Either test them directly (P11) or drop the NAND kind from this model.
- Lines 448 and 452, the `j < n_in` scan and its fallback `hit = 0` in sl_trigger: reached only by a gate that asks for a change when none of its input pins differ from the last evaluation; no test is shown to reach it (survivor #11).
- Lines 516 and 517, the tgate strength-only rule: nine survivors (#19 to #27) say no test reaches it with a hand-chosen drive; the pass arc, row 8 at line 359, appears to be exercised by no asserted delay (inferred from #8 surviving, not traced).
- Line 744, `was != now`: by reading, redundant. Whenever PT[g] is set (lines 720 to 726) PV[g] differs from DRV[g], and DRV[g] changes only at line 746, which clears PT[g]; so at maturity was != now always holds.
- Lines 273 and 274, SL_NETS = 128 and SL_GATES = 160: defined and used nowhere (grep finds only the two definitions), while the converter's tables hold 256 nets and 256 gates (spice_convert.t27 lines 2399 and 2401). sar_logic.t27 has no capacity check: a deck beyond 128 nets or 160 gates writes into the next table of `ms` without a message.
- Lines 587, 592, 609, 613, 636, 739, 761, 774 (all `< NGATE` or `< NNET` loops): the model relies on the row after the last gate of a zeroed `gt` being an inverter on net 0. Under the `<=` mutants that row is live. This is harmless while net 0 is VDD or a stimulus input (every deck in the file) and wrong when net 0 is a gate's output (survivors #53 and #73).
- Lines 932 and 1026: the `or n == 0` half of the refusal guard is redundant on a refusal, since put_reject writes the card text, so n > 0 there (survivors #98 and #104). Whether an accepted deck could give n == 0 was not checked.
- Line 1052, the comment says CK1 rises 654 ps after the last Valid rise; no test pins the stimulus array, so the order and the offset are untested (survivors #105 and #106). With `t - 654` the array is no longer sorted, which breaks the contract stated at lines 780 and 781.
- Lines 1279, 1529, 1532, 1538, 1541, 1544, the only callers of sl_within and sl_within_fall: every call passes a value far from the bound, so the boundary of the two comparisons and the `and` is untested (survivors #89 to #94).

## Proposed tests, grouped

26 distinct new tests, P1 to P26, ordered by the number of survivors each kills. The union kills all 80 TEST-GAP survivors (#8 is named by both P3 and P11 and is counted once; no other survivor is named twice, so 9+6+6+5+5+5+4+4+4+4+4+3+3+2*6+1*7 = 81, less the one shared survivor, gives 80). Every expected value is hand-derived and unverified: it was computed by reading the spec's constants and arcs, nothing was run. "Direct" means a call of the function with hand-set arguments or a hand-set state vector; "fixture" means an existing test with an added or tightened assert. The mutant values in brackets are what the mutant gives, by the same hand derivation.

### P1 - the strength-only rule of a tgate, direct sl_delay (kills 9: #19 to #27, lines 516 and 517)

- Function: sl_delay (lines 503-563) on the hold netlist of fixture j (lines 1412-1463), right after `sl_init` and `sl_loads` (gate 0 is the tgate; q has load 1150, so the arc sees 1000 after the `- 150`).
- Inputs and expected values:
  - `ms[SL_DRV + 0] = SL_W1`, ask `SL_S1`: `sl_delay(&ms, &gt, 0, SL_S1, 0) == 1` (kills #19; mutant 137, the row 9 rise 85 + 527 * 1000 / 10000).
  - `ms[SL_DRV + 0] = SL_W0`, ask `SL_S0`: `== 1` (kills #20, #22, #23, #25, #26; mutant 78, the row 9 fall 49 + 297 * 1000 / 10000).
  - `ms[SL_DRV + 0] = SL_S0`, ask `SL_W0`: `== 1` (kills #21 and #27; mutant 78).
  - `ms[SL_DRV + 0] = SL_Z`, ask `SL_S0`: `== 78` (kills #24; mutant 1). Alternatively tighten fixture j to `tq == 15078`.
- Hist is on after `sl_init` (it sets SL_HON), but the hold gate has CELL = 0 and FED = 0, so no history scaling applies.

### P2 - the bounds of sl_within and sl_within_fall, direct (kills 6: #89 to #94, lines 848 and 853)

- Functions: sl_within (847-849) and sl_within_fall (852-854); pure, no setup.
- `sl_within(420, 400) == true` (absdiff 20, error 20 * 1000 / 400 = 50; kills #89, mutant false); `sl_within(10100, 10000) == false` (error 10, absdiff 100; kills #90, mutant true); `sl_within(1030, 1000) == true` (absdiff 30, error 30; kills #91, mutant false).
- `sl_within_fall(500, 400) == true` (absdiff 100, error 250; kills #92, mutant false); `sl_within_fall(11000, 10000) == false` (absdiff 1000, error 100; kills #93, mutant true); `sl_within_fall(1150, 1000) == true` (absdiff 150, error 150; kills #94, mutant false).

### P3 - the pass arc and the enable arc of the tgate, fixture (kills 6: #8 line 359, #12 line 449, #13 line 449, #14 line 453, #28 line 539, #66 line 731)

- Function: sl_run on the hold netlist of fixture j with a new stimulus: vdd = 1, vss = 0, d = 1, en = 0, nen = 1 at t = 0, then d = 0 at 10000 and nothing else; run to 20000; q watched.
- Expected: `sl_edge(&ms, q, 1, 1, 0) == 137` (85 + 527 * 1000 / 10000; kills #12 [61] and #28 [153]) and `sl_edge(&ms, q, 0, 1, 1000) == 10064` (40 + 246 * 1000 / 10000 on the pass arc, row 8; kills #8, #13, #14 and #66 [10078, the enable arc 49 + 29 = 78], also #28 [10071]).
- Optional tightening of the existing fixture j: `sl_edge(&ms, q, 0, 1, 1000) == 15078` (kills #28 [15087]).

### P4 - the refusal path and the end of a buffer (kills 5: #95 line 875, #97 and #98 line 932, #103 and #104 line 1026)

- Functions: sl_build (929-944), sl_chain_build (1023-1033) and sl_cat (873-880).
- `var w16 : [16]u8 = undefined;` and the usual `src` (4096 or 1024 bytes), `out`, zeroed `st`, `gt`, `ms` and a 24-slot `ids`: `sl_build(&src, &w16, &out, &st, &gt, &ms, &ids, false) == false` (kills #97 and #98; the mutants go on with an empty net table and index the state vector with NONE, returning true or panicking) and, on fresh buffers, `sl_chain_build(&src, &w16, &out, &st, &gt, &ms, &ids) == false` (kills #103 and #104).
- `var small : [64]u8 = undefined;` and `sl_cat(&small, 0, SL_DECK_A) == 64` (kills #95; the mutant writes index 64 and panics in a Debug test).
- Caveat: the refusal of a 16-byte `w` was read by hand in the converter (spice_convert.t27 lines 687-688, 2783-2784, 2799-2800, and its own test at 3223-3237 for `inventory`), not run. If 16 does not give a clean refusal, use the smallest size at which the converter's Y_SIZE test of the same shape refuses.

### P5 - the external load reaches the net, direct state check (kills 5: #49 and #50 line 631, #51 and #52 line 632, #57 line 652)

- Functions: sl_ext_load (651-653) and sl_loads (627-649) on the clock_gen chain deck (fixture lines 1356-1410).
- After `sl_ext_load(&ms, vout, SL_EXT_8F); sl_loads(&ms, &gt);`: `ms[SL_LDR + vout] == 600` and `ms[SL_LDF + vout] == 600` (vout feeds no pin, so its load is the external 600 alone; each mutant gives 0).

### P6 - the pin loads, direct (kills 5: #1 line 331, #2 line 335, #3 line 340, #4 and #5 line 349)

- Function: sl_pin_load (325-351); pure.
- `sl_pin_load(G_NOR, 2, SL_RISE) == 0` (#1 [1000]); `sl_pin_load(G_NOR3, 3, SL_RISE) == 0` (#2 [1000]); `sl_pin_load(G_TGATE, 1, SL_RISE) == 250` (#3 [0]); `sl_pin_load(G_DELAY, 1, SL_RISE) == 250` (#4 [0]); `sl_pin_load(G_DELAY, 4, SL_RISE) == 0` (#5 [250]).

### P7 - the inverter after the delay cell reads the cell's trim code (kills 4: #30, #31, #32 line 556, #47 line 616)

- Function: sl_delay (rise of the inverter, lines 552-560) on the clock_gen chain deck, trim code 000.
- Setup: the `code == 0` iteration of the trim-code test (lines 1356-1410) with the run ended at 50000 instead of 190000 (clks fell at 10100; by 50000 the nor3 output n1 is 1, the delay cell's output has fallen and vout has risen).
- Expected: `sl_delay(&ms, &gt, 2, SL_S1, 50000) == 626`. The arc term 48 + 719 * 600 / 10000 = 91 added by the inverter's own arc cancels against the `- sl_arc(0, SL_RISE, SL_EXT_8F)` of line 556, leaving SL_IV_R[0] = 626. Mutants: #30 reads the inverter's own pins (code 6, vout high: 436); #31 reads the ghost row, whose pins are net 0 = VDD (code 7: 657); #32 gives 626 + 2 * 91 = 808; #47 reads the nor3, whose pin 3 is n1 = 1 (code 1: 466). At the end of the existing run (t = 190000, clks high) the two readings of #47 agree on code 0, which is why the existing test cannot see it.

### P8 - sl_init post-conditions (kills 4: #38 line 593, #39 line 594, #41 and #42 line 598)

- Function: sl_init (584-603) on the 2-gate hold deck (fixture j, line 1425).
- Right after `sl_init`: `ms[SL_DRV + 1] == SL_X` (#38 [0]); `ms[SL_PT + 1] == NONE` (#39 [0]); `ms[SL_PREV + 7] == SL_X` (#41 [0], the last gate's slot 0); `ms[SL_PREV + 7 + 1] == SL_X` (#42 [0], the last gate's slot 1).

### P9 - sl_run edge cases (kills 4: #80 and #81 line 794, #82 line 798, #83 line 803)

- Function: sl_run (782-808) on the hold deck.
- (a) #82: one stimulus edge only, `k = sl_put(&s_t, &s_n, &s_v, 0, 0, en, 1)` into zeroed 16-slot arrays, `sl_run(.., 1, 100)`: `ms[SL_VAL + en] == 1` and `ms[SL_SI] == 1` (mutant: it applies the zeroed slot 1 as net 0 = 0 once more, so VAL 0 and SI 2; net 0 is `en` in this deck).
- (b) #81 and #83: set `ms[SL_NSTEP] = SL_MAXSTEP - 1` before the fixture j stimulus and `sl_run(.., 20000)`: `ms[SL_NSTEP] == SL_MAXSTEP` (#81 [SL_MAXSTEP + 1]; #83 [SL_MAXSTEP - 1]).
- (c) #80: in fixture j insert `sl_run(&ms, &gt, &s_t, &s_n, &s_v, k, 5000);` followed by `assert(ms[SL_VAL + en] == 1);` between the 4900 run and the 14900 run (the mutant leaves the 5000 edge pending and en is still 0).

### P10 - sl_edges and sl_edge on a hand-made log (kills 4: #84 line 816, #85 line 818, #86 line 829, #87 line 831)

- Functions: sl_edges (813-822) and sl_edge (825-838).
- On a fresh zeroed `ms`: `sl_edges(&ms, 0, 0, 0) == 0` (#84 [1]) and `sl_edge(&ms, 0, 0, 1, 0) == NONE` (#86 [0]).
- Then `sl_watch(&ms, 5); sl_set(&ms, 5, 1, 100);`: `sl_edges(&ms, 5, 1, 100) == 1` (#85 [0]) and `sl_edge(&ms, 5, 1, 1, 100) == 100` (#87 [NONE]).

### P11 - the NAND row and arm, direct (kills 4: #6 and #7 line 356, #8 line 359, #10 line 417)

- Functions: sl_row (354-361) and sl_want (412-423).
- `sl_row(G_NAND, 1) == 2` (#6 [9], #7 [0]); `sl_row(G_TGATE, 2) == 8` (#8 [9]).
- On a zeroed `gt` set `gt[GT_GATE0] = G_NAND; gt[GT_GATE0 + 1] = 0; gt[GT_GATE0 + 2] = 1; gt[GT_GATE0 + 3] = 2;` and `ms[SL_VAL + 0] = 1; ms[SL_VAL + 1] = 0`: `sl_want(&ms, &gt, 0) == SL_S1` (#10 [SL_S0, the fall-through `sl_not(1)`]). #8 is also killed by P3.

### P12 - sl_order_ok on a hand-made log (kills 3: #100 line 973, #101 line 974, #102 line 983)

- Function: sl_order_ok (970-989).
- Setup: zeroed `ms` and a 24-slot zeroed `ids` with `ids[9 + i] = 30 + i` for i = 0 .. 10; `sl_watch(&ms, 30 + i)` for each.
- Positive control: `sl_set(&ms, 30 + i, 1, 1000 * (i + 1))` for i = 0 .. 10: `sl_order_ok(&ms, &ids) == true`.
- #100: the same but net 40 (CK1) never rises: `== false` (mutant true: the pair check at 983 cannot see a CK that never rises last, since NONE sorts last).
- #101: the positive control plus `sl_set(&ms, 33, 0, 30000)`: `== false` (mutant true).
- #102: the positive control with net 32 rising at 5000 and net 34 at 3000: `== false` (mutant true).

### P13 - sl_step maturity bookkeeping, hand-made state (kills 3: #68 and #69 line 744, #70 line 745)

- Function: sl_step (737-778) on any converter-built deck of at least two gates right after `sl_init` and `sl_loads` (the clock_gen deck, or the hold deck).
- Setup A (#68, #69): `ms[SL_DRV + 1] = SL_S1; ms[SL_PV + 1] = SL_X; ms[SL_PT + 1] = 5000; ms[SL_TCH + 1] = NONE; sl_step(&ms, &gt, 5000);` then `ms[SL_TCH + 1] == NONE` (both mutants write 5000).
- Setup B (#70): `ms[SL_DRV + 1] = SL_X; ms[SL_PV + 1] = SL_S1; ms[SL_PT + 1] = 5000; ms[SL_TCH + 1] = 100; sl_step(&ms, &gt, 5000);` then `ms[SL_TCH + 1] == NONE` (mutant 100).

### P14 - net 0 is a gate output, the glitch deck with y first (kills 2: #53 line 636, #73 line 761)

- Function: sl_loads and sl_step on the glitch deck of fixture k (lines 1465-1497) with only the port order changed to `.subckt top y a vdd vss` (so y is net 0 and a is net 1; the instance line is unchanged).
- Expected: `sl_edge(&ms, y, 0, 1, 500) == 2064` (#53 [2096, the ghost's 1000 load makes the fall 64 + 327 * 1000 / 10000 = 96]) and, with a run to 500, `ms[SL_VAL + y] == 1` (#73 [2: the ghost's phantom strong 0 fights the real 1]).

### P15 - sl_set on its own (kills 2: #58 line 666, #59 line 667)

- Function: sl_set (662-676) on a fresh `ms`.
- `sl_watch(&ms, 3); sl_set(&ms, 0, 1, 10);` then `ms[SL_NLOG] == 0` (#58 [1]).
- `ms[SL_NLOG] = SL_LOGMAX; sl_watch(&ms, 3); sl_set(&ms, 3, 1, 10);` then `ms[SL_NLOG] == 640` (#59 [641]).

### P16 - sl_links resets its tables, dirty state (kills 2: #44 line 610, #45 line 611)

- Function: sl_links (607-624) on a built 2-gate deck.
- `ms[SL_CELL + 1] = 7; ms[SL_FED + 1] = 7; sl_links(&ms, &gt);` then `ms[SL_CELL + 1] == 0` and `ms[SL_FED + 1] == 0` (#44 and #45 [7]; gate 1 is needed, since for g = 0 the mutated and original slots coincide).

### P17 - cancelling a pending change, glitch deck with a dummy first inverter (kills 2: #63 line 717, #64 line 718)

- Function: sl_sched (712-734) through sl_run on the glitch deck (fixture k) run to t = 1050 (a rose at 1000, fell at 1030; the inverter's fall was due at 1064 and must be withdrawn).
- #63: `ms[SL_PT + 0] == NONE` at t = 1050 (mutant 1078 = 1030 + 48; run to 1050, because by 1078 the phantom has matured and PT is NONE again).
- #64: add one dummy inverter before the glitched one so that it is gate 1: `ms[SL_PT + 1] == NONE` at t = 1050 (mutant 1064); the fixture's own asserts (one falling edge of y, at 2064) then fail in the mutant too.

### P18 - the log error on a tie, hand-made log (kills 2: #107 line 1085, #108 line 1093)

- Functions: sl_chain_err_f (1082-1087) and sl_chain_err_r (1090-1095).
- Setup: zeroed `ms`, `ids[8] = 30`, `sl_watch(&ms, 30)`, `sl_set(&ms, 30, 1, 60000)`, `sl_set(&ms, 30, 0, 99700)`, `sl_set(&ms, 30, 1, 119900)`.
- Expected: `sl_chain_err_f(&ms, &ids, 20200, 19800, 0, 100) == 0` (#107 [9999]; tr = 50000 + 49700) and `sl_chain_err_r(&ms, &ids, 20200, 19800, 0, 100) == 0` (#108 [9999]; tf = 99700 + 20200 = 119900 is the second rise at or after 50000).

### P19 - the chain stimulus, direct (kills 2: #105 line 1047, #106 line 1052)

- Function: sl_chain_stim (1037-1057) into zeroed 64-slot arrays and a hand-filled 24-slot `ids`.
- `sl_chain_stim(.., 20200, 19800, 11, true)` returns 32; `s_t[8] == 51100` (#105 [48900]); `s_t[29] == 499700`; `s_t[30] == 500354` with `s_n[30] == ids[4]` (#106 [499046, which also makes the array unsorted]); `s_t[31] == 519900`.

### P20 to P26 - one survivor each

- P20 (#11, line 448): sl_trigger on the hold deck (after `sl_init`, `sl_loads`), gate 1 (the inverter, n_in = 1): `ms[SL_VAL + y] = 1` with PREV still X from `sl_init` and the real pin q equal to its PREV: `sl_trigger(&ms, &gt, 1) == 0` (mutant 1: the extra pin 1 is the output y, which differs from its X in PREV).
- P21 (#17, line 480): sl_age: `ms[SL_TCH + 0] = 5000; sl_age(&ms, 0, 5000) == 0` (mutant 1000000).
- P22 (#29, line 548): sl_delay on the clock_gen chain deck, nor3 g = 0 with Valid as trigger (PREV[1] != VAL): `ms[SL_DRV + 1] = SL_S1; ms[SL_TCH + 1] = t - 5000;` then `sl_delay(&ms, &gt, 0, SL_S1, t) < 7390` (the mutant reads the inverter's TCH = NONE, so age 1000000 and the unscaled 7390; the exact value of the original, about 6008, was not re-derived).
- P23 (#35, line 570): sl_net on any built deck: `sl_net(&w, &gt, "") == NONE` (mutant NNET).
- P24 (#46, line 613): a new deck whose first root port is a nor3 output feeding a delay cell; with two nor3 inputs high the delay cell's rise must be the table value times 806 / 1000. The expected value was not derived.
- P25 (#62, line 689): sl_resolve: `ms[SL_CNT + SL_X] = 1; sl_resolve(&ms, 0, 0) == SL_X` (mutant 0: it reads slot 126, which is 0).
- P26 (#99, line 939): after `sl_build` on the sar deck, `ms[SL_NWATCH] == 12` (mutant 13) and, after the stimulus has run, `sl_edges(&ms, ids[0], 1, 0) == 0` (mutant 1: ids[21] is 0 = VDD, whose rise at t = 0 is then logged).

## Not claimed

- Nothing was run. No compiler, test, mutation tool or script was started; every value, in particular every expected value and every "mutant gives" value, was derived by hand from the spec and is unverified until a T27 OS pane runs it.
- Source of the spec: lane-12116 was gone, so lane-12137's copy of /specs/analog/sar_logic.t27 (1686 lines) was read. Its code lines 1 to 1095 match the census (checked by reading the mutated lines against the census descriptions). Its tests region differs by one line from the censused 1687-line spec, so line numbers of survivors in the tests region were not needed and none are claimed.
- The 31 HUNG mutants (all drop-step) and the 12 unviable ones are out of scope; they were not classified.
- Capacity edge of the `<=` loop mutants (#36, #37, #40, #43, #48, #54 to #56, #65, #67, #71, #72, #76, #78): their equivalence assumes a deck below 128 nets and 160 gates. Decks at the capacity were not traced. At 160 gates the extra DRV and PT writes of sl_init would alias PT[0] and PV[0]; at 128 nets the extra writes alias CNT[0], LDF[0] and EXT[0].
- The ghost-row analysis (the row after the last gate of `gt`) assumes `gt` starts zeroed, as every fixture's does.
- #60 and #77 are equivalent only under the contract at line 678 (an input has no driver in the circuit); #18 only for calls made by sl_delay on a converter deck (a hand-built call with the cell's input net at 1 would differ).
- #106: the run-level effect (the chain k01 fixture at line 1635 measures clkc falls against bounds, with a margin of about 60 ps) was not re-derived. Only the direct assert on the stimulus array is proposed.
- The refusal of a 16-byte work buffer in `emit_gates` (P4) was read at spice_convert.t27 lines 687-688, 2783-2784 and 2799-2800 and at its own test 3223-3237, not run. The kills of #98 and #104 also rely on a refusal writing at least "line " (n > 0), read at put_reject (1637-1648).
- #29 and P22: the exact original value (about 6008) is from an earlier hand derivation that was not repeated; only the bound `< 7390` is claimed, and the inference that the mutant survived 29 chains because of thin bounds is from the survival, not traced.
- That fixture f's 50 permille bounds mask #3, and that the pass arc (row 8) is exercised by no asserted delay, are inferred from the survival of those mutants, not traced.
- #33 (DEAD): the lower bound of about 0.22 on the history ratio was read from the history table by hand; the claim is that d never reaches 0, not that it was measured.
- #35 (P23) assumes `sl_net` accepts an empty-string slice; #44's alternative assert (TCH[110] == NONE on the sar deck) assumes sl_build runs sl_loads after sl_init, which line 937 shows, and that gate 50 is a gate whose mirrored write lands there (hand-derived).
- #46 / P24 needs a new deck; no expected value was derived. #24 can be killed by a one-line tightening of fixture j, `tq == 15078`, but that value comes from the P1 hand derivation.
- #31's kill relies on net 0 = VDD = 1 after the stimulus has run.
- The values 137, 61, 153, 78, 10064, 10078, 15078, 15087 in P1 and P3 were recomputed for this note from the arc table (SL_ARC rows 8 and 9 and sl_arc at line 364), with the tgate's own pin load of 150 subtracted at line 539; the load of q (1150) comes from sl_pin_load and was not run.
- #98 and #104: the `n == 0` half of the guards at 932 and 1026 is redundant on a refusal; whether n can be 0 on an accepted deck was not checked.
