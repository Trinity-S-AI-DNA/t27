# The external team's per-block checks applied to our SAR ADC blocks, 2026-10-20

Issue #11212, milestone M5c of the analog epic #10486, night loop #11063; M5d (issue #11263) adds the transistor-level rows of the nand, nor, tgate and tracking_switch, cross-checked against ngspice. English, ASCII only.

The external team's report of 2026-10-16 lists, for each of the 13 blocks of the golden SAR ADC, a measured value and the limit that its check enforces (conditions: tt corner, 27 C, VDD 1.5 V, cells taken from the integrated converter's netlist). The team confirmed on 2026-10-20 that the checks and their limits may be used and published here. `specs/analog/accept_sar.t27` holds those limits as named constants, one pure check function per quantity, and one function per block that measures the quantity on OUR model by `use` of the spec that holds it. This document is the result. Every "our value" below is computed by a named function of that spec and pinned by a test; every "report" value is the report's, and is also in the spec as the positive control of its check (the report's own number must pass the report's own check).

What this document does not say: nothing about a transistor-level result for the gate-level blocks (dff, clock_gen, SAR_logic), nothing about the voltage level of any digital node but the nand's and the nor's, nothing about the delay of a gate into a load, nothing about the tgate's resistance, nothing about power, noise, mismatch, another corner or temperature, or speed. No model was tuned to turn a FAIL into a PASS, and none was changed to make a number come out.

## 1. The verdict rule

A row is one quantity of the report's check.

- PASS: our model produced the value and it is inside the report's limit.
- FAIL: our model produced the value and it is outside the limit. It is reported as measured.
- NOT MODELLED: our model does not produce the quantity the check enforces. Nothing is compared, and the row is never a pass, whatever a constant might have said (`as_verdict(false, x)` is NOT MODELLED for both values of `x`; a test pins it).

A block is FAIL when any row fails, PASS only when it has rows and every row is PASS, and NOT MODELLED otherwise. A block whose threshold passes and whose delay is not modelled is therefore not a pass: it is NOT MODELLED, and its passing row is still shown.

The cross-check rule (M5d). A row from a transistor-level model is a verdict only when the same cell, run in ngspice-47 with the foundry's PSP103 (`specs/analog/decks/accept_*.cir`, on the reference host: a container with 2 threads), agrees with our number within a bar that is stated with the number: 10 uV for a level, 10 percent for a resistance or a current. A value without an ngspice run, or that does not agree with it, is shown in the table and is NOT MODELLED, whatever the limit says. The bars are far below the limits (50 mV, 30 kohm, 1 nA), so they decide only whether our number is the cell's number, never whether it passes the report's check.

## 2. The counts

Over the 13 blocks of the report (inverter, nand, nor, tgate, dff, dynamic_comparator, tracking_switch, clock_gen, SAR_logic, DAC_control, DAC_control_top, neg_bottom_plate_logic, the 11-bit integration run):

| verdict | blocks | which |
|---|---|---|
| PASS | 5 | clock_gen, SAR_logic, integration, nand, nor |
| FAIL | 1 | dynamic_comparator (decision time) |
| NOT MODELLED | 7 | inverter, tgate, dff, tracking_switch, DAC_control, DAC_control_top, neg_bottom_plate_logic |

Over the 30 rows: 19 PASS, 1 FAIL, 10 NOT MODELLED (a test pins both counts: `as_all_blocks`, `as_all_rows`). Before M5d the counts were 3 / 1 / 9 blocks and 16 / 1 / 13 rows. The new PASS rows are the nand's and the nor's levels (2) and the tgate's off leakage (1); the tgate's two resistances are measured now but stay NOT MODELLED, with their numbers and the reason (section 3b). Four of the seven NOT MODELLED blocks have at least one measured, passing row (inverter 1, tgate 1, dff 2, tracking_switch 1); three have none (DAC_control, DAC_control_top, neg_bottom_plate_logic). The PASS of clock_gen and SAR_logic is the PASS of the gate-level model named in section 4, that of integration the PASS of a behavioural converter fitted to the golden deck, and that of the nand and the nor the PASS of a DC solve on the PSP103 tables, each level within 3 uV of ngspice's.

The report's corners smoke suite (SAR inverter delay 62 to 124 ps, comparator decision 467 to 829 ps over 5 corners at -40, 27 and 125 C) is a test run, not a block, and is not in the 13. It is NOT MODELLED (last row of the table).

## 3. The table

Times in ps or ns as written. "Report" is the report's measured value; "limit" is what its check enforces. Our value is from the function named in the last column.

| block | check | our value | report | limit | verdict | function |
|---|---|---|---|---|---|---|
| inverter | switching threshold | 0.6892 V | 0.69 V | 0.5 to 1.0 V | PASS | `as_inv_vth_of` |
| inverter | delays fall / rise into 20 fF | not produced | 81 / 100 ps | 1 to 500 ps | NOT MODELLED | |
| nand | truth table 00, 01, 10, 11 (logic level) | 1, 1, 1, 0 | rows right | exact | PASS | `sl_nand_code` |
| nand | levels into 10 fF (DC solve, worst level of the 4 inputs) | 1 levels 1.5 V - 0.18 / 0.48 / 0.48 uV; 0 level 0.18 uV | 1.5 V for a 1, microvolts for a 0 | 1.45 to 1.55 V; +-50 mV | PASS (within 2.4 uV of ngspice) | `gt_levels` |
| nor | truth table 00, 01, 10, 11 (logic level) | 1, 0, 0, 0 | rows right | exact | PASS | `sl_nor_code` |
| nor | levels into 10 fF (DC solve, worst level of the 4 inputs) | 1 level 1.5 V - 1.85 uV; 0 levels 0.06 uV | 1.5 V for a 1, microvolts for a 0 | 1.45 to 1.55 V; +-50 mV | PASS (within 1.1 uV of ngspice) | `gt_levels` |
| tgate | Ron at 0.75 V (10 mV across it) | 1718 ohm, no body effect; ngspice's real cell 2460 ohm, 30.2 % above | 2.44 kohm | <= 30 kohm | NOT MODELLED (cross-check bar 10 %) | `gt_tg_ron` |
| tgate | worst Ron over 0.1 to 1.4 V (27 points) | 2109 ohm at 1.15 V, no body effect; ngspice's real cell 2776 ohm at 1.0 V, 24.0 % above | 2.78 kohm | <= 50 kohm | NOT MODELLED (cross-check bar 10 %) | `gt_tg_worst` |
| tgate | off leakage (full swing) | 291.0 pA; ngspice 285.6 pA at the output source, 271.4 pA at the input source | 0.29 nA | <= 1 nA | PASS (within 7.2 % of ngspice) | `gt_tg_off` |
| dff | edge capture, hold, set, reset (logic level, 11 stages) | all right | all right | exact | PASS | `as_sar_run`, `as_dff_set_reset` |
| dff | levels of the 8 observation points | not produced | all right | +-50 mV | NOT MODELLED | |
| dff | clock-to-Q | 546 ps | 524 ps | 1 to 3000 ps | PASS | `as_sar_run` |
| dynamic_comparator | sign at +-2 mV, 0.75 V common mode | +2 mV: dp wins; -2 mV: dn wins | right | right | PASS | `as_cmp` |
| dynamic_comparator | levels after the decision | winner 1.49996 V, loser 3.6 mV | right | +-50 mV | PASS | `as_cmp` |
| dynamic_comparator | reset | not produced | right | +-50 mV | NOT MODELLED | |
| dynamic_comparator | decision time at +-2 mV | 5.35 ps | 607 ps | 10 to 10000 ps | **FAIL** | `as_cmp` |
| tracking_switch | tracking error, DC (our levels 0.2 V and 1.3 V into 3.93 pF) | -1.97 uV at both; ngspice PSP103 +0.29 / -0.17 uV | < 0.001 mV at 0.3 V and 1.2 V into 2 pF | +-5 mV | PASS (within 2.3 uV of ngspice) | `as_sw` |
| tracking_switch | hold step | not produced; ngspice PSP103 cell -1.60 / -1.73 mV at the report's conditions | -1.6 / -1.7 mV | +-30 mV | NOT MODELLED | |
| tracking_switch | droop | not produced; ngspice PSP103 cell under 10 uV over 413 ns | none | +-5 mV | NOT MODELLED | |
| clock_gen | slow delay (all trims 0), clks fall to clkc rise | 16.145 ns | 15.0 ns | 0.05 to 50 ns | PASS | `as_clk_golden` |
| clock_gen | fast delay (all trims 1) | 9.822 ns | 9.2 ns | 0.02 to 40 ns | PASS | `as_clk_delay_ps` |
| clock_gen | ratio slow / fast | 1.644 | 1.64 | >= 1.2 | PASS | |
| clock_gen | stops after Valid | 5.704 ns | 10.7 ns | <= 30 ns | PASS | `as_clk_golden` |
| SAR_logic | one more CK per Valid edge, CK11 first; 1, 4, 11 high after 1, 4, 11 edges | 1, 4, 11; thermometer in order at all 11 edges, held between edges | exact counts | exact | PASS | `as_sar_run` |
| SAR_logic | CK11 to CK1 | 200.126 ns | 200 ns | 199 to 201 ns | PASS | `as_sar_run` |
| DAC_control | idle 0.75 V; 0 V / 1.5 V after decisions 1 / 0 | not produced | right | 0.7 to 0.8 V; +-50 mV | NOT MODELLED | |
| DAC_control_top | 11 bits latch in CK order; code, plate code | not produced | 1430, 617 | exact | NOT MODELLED | |
| neg_bottom_plate_logic | Bit_n follows Bit while CK11 is low, complement after; VCM | not produced | 0.75 V | 0.7 to 0.8 V; +-50 mV | NOT MODELLED | |
| integration | worst code error, -1 V to +1 V ramp of 8 conversions | 18.5 LSB | 13.8 LSB | +-64 LSB | PASS | `as_ramp_literal` |
| integration | smallest step between conversions | 198 LSB | 154 LSB (ideal 152) | 60 to 250 LSB | PASS | `as_ramp_literal` |
| corners (smoke suite, not a block) | inverter delay 62 to 124 ps, comparator decision 467 to 829 ps, 5 corners x 3 temperatures | not produced | within | as stated | NOT MODELLED | |

## 3b. The new rows of M5d, with ngspice's number beside ours

Every number in this table is printed by a deck of `specs/analog/decks` run in ngspice-47 with the IHP SG13G2 PSP103 (IHP-Open-PDK 21133f21, mos_tt, 27 C) on the same cell, and pinned in `specs/analog/mos_psp103_gates.t27` (nand, nor, tgate) or `specs/analog/accept_sar.t27` (tracking switch). "Difference" is ours minus ngspice's. The bar is what the cross-check rule of section 1 allows; the limit is the report's.

| row | our value | ngspice | difference | bar | report | limit | verdict |
|---|---|---|---|---|---|---|---|
| nand 00, 1 level (droop under 1.5 V) | 0.179 uV | 0.316 uV | -0.138 uV | 10 uV | 1.5 V | 1.45 to 1.55 V | PASS |
| nand 01, 1 level | 0.477 uV | 0.721 uV | -0.244 uV | 10 uV | 1.5 V | 1.45 to 1.55 V | PASS |
| nand 10, 1 level | 0.477 uV | 0.687 uV | -0.210 uV | 10 uV | 1.5 V | 1.45 to 1.55 V | PASS |
| nand 11, 0 level | 0.179 uV | 2.537 uV | -2.358 uV | 10 uV | microvolts | +-50 mV | PASS |
| nor 00, 1 level (droop under 1.5 V) | 1.848 uV | 2.883 uV | -1.036 uV | 10 uV | 1.5 V | 1.45 to 1.55 V | PASS |
| nor 01, 0 level | 0.060 uV | 0.634 uV | -0.575 uV | 10 uV | microvolts | +-50 mV | PASS |
| nor 10, 0 level | 0.060 uV | 0.617 uV | -0.557 uV | 10 uV | microvolts | +-50 mV | PASS |
| nor 11, 0 level | 0.060 uV | 0.611 uV | -0.552 uV | 10 uV | microvolts | +-50 mV | PASS |
| tgate Ron at 0.75 V | 1718.0 ohm | 2459.6 ohm (real cell) | -741.6 ohm, -30.2 % | 10 % | 2.44 kohm | <= 30 kohm | NOT MODELLED |
| tgate worst Ron, 27 points 0.10 to 1.40 V | 2109.3 ohm (1.15 V) | 2776.1 ohm (real cell, 1.0 V) | -666.8 ohm, -24.0 % | 10 % | 2.78 kohm | <= 50 kohm | NOT MODELLED |
| tgate off leakage, 1.5 V across | 291.0 pA | 285.6 pA (output source), 271.4 pA (input source) | +5.4 pA (+1.9 %), +19.7 pA (+7.2 %) | 10 % | 0.29 nA | <= 1 nA | PASS |
| tracking error, 0.2 V into 3.93 pF | -1.967 uV | +0.287 uV | -2.253 uV | 10 uV | < 1 uV at 0.3 V and 1.2 V into 2 pF | +-5 mV | PASS |
| tracking error, 1.3 V into 3.93 pF | -1.967 uV | -0.174 uV | -1.793 uV | 10 uV | as above | +-5 mV | PASS |
| hold step, 0.3 V into 2 pF | not produced | -1.596 mV | | | -1.6 mV | +-30 mV | NOT MODELLED |
| hold step, 1.2 V into 2 pF | not produced | -1.730 mV | | | -1.7 mV | +-30 mV | NOT MODELLED |
| droop over the hold, all four runs | not produced | under 10 uV (zero at the 6 digits printed) | | | none | +-5 mV | NOT MODELLED |

What the numbers say, one by one.

- nand and nor levels. All eight levels, ours and ngspice's, are microvolts from the rail (1.5 V or 0 V): the report's "microvolts for a 0" is ngspice's too. The largest difference is 2.358 uV (the nand's 11, where the model's current floor leaves 0.18 uV and ngspice 2.54 uV); the bar of 10 uV was set at four times the worst difference and the verdict does not depend on it, since the limit is 50 mV. The levels are the DC solution with no load, which is the level a gate settles to into 10 fF; no transient is run, so nothing is said about how fast a gate gets there.
- tgate off leakage. Ours is the sum of the two table words at Vgs = 0, Vds = 1.5 V (the NMOS 261 pA, the PMOS 30 pA). ngspice's two sources differ by 14 pA (285.6 pA at the output, 271.4 pA at the input), the size of a junction leakage that sits in different terminals of the two; the terms are not separated here, and our number is within the 10 percent bar of both, and the report's 0.29 nA is ngspice's output-source number rounded.
- tgate Ron. Both resistances are NOT MODELLED because the model is 30 and 24 percent under the real cell, and the cause is measured, not guessed: the tables have no source-to-bulk voltage, and the real tgate has its bulks on the rails, so at 0.75 V the NMOS source sits 0.75 V above its bulk and the PMOS source 0.75 V below its bulk, which raises both thresholds. The deck `accept_tgate.cir` runs the same two devices again with each bulk tied to its own source, the condition the tables were made under: there ngspice gives 1717.2 ohm at 0.75 V and 2108.4 ohm at the worst point, and the model is within 0.06 percent of it at all 27 points (0.0588 percent at the largest). The gap to the real cell is largest at 0.85 V (31.1 percent) and the model is under the real cell at every one of the 27 points. Both resistances are far inside their limits (30 and 50 kohm); the rows are not PASS because the number is not the cell's number.
- tracking error. The square-law model (`sar_switch.t27`) tracks to -1.967 uV, the arithmetic floor; the PSP103 cell tracks to +0.287 and -0.174 uV (a settling residue, either sign); the 2.3 uV between them is under the bar of 10 uV and 2000 times under the limit. At the report's own conditions (0.3 V and 1.2 V into 2 pF) the PSP103 cell tracks to +0.346 and +0.350 uV; our model is not run at those two conditions (its load and its levels are fixed in `sar_switch.t27`), and the DC error is the model's floor at every level it has been run at.
- hold step and droop. The model has no gate capacitance and no leakage, so it produces neither, and the rows stay NOT MODELLED as before. What is new is ngspice's number for the same cell with the foundry's model: -1.596 and -1.730 mV at the report's conditions (the report's -1.6 and -1.7 mV), and -0.812 and -0.890 mV into our 3.93 pF (the charge injection spreads over a larger capacitor). The droop is zero at the six digits ngspice prints (under 10 uV) over the 413 ns of hold.

### The reference run reproduces the report

The same ngspice runs that serve as the reference give the numbers the external team measured on the same cells, so the cells we cross-check against are the report's cells (a test, `the_reference_run_of_the_real_tgate_and_the_psp103_switch_reproduces_the_reports_own_numbers`, pins each):

| quantity | the report | our ngspice-47 run | difference |
|---|---|---|---|
| tgate Ron at 0.75 V | 2.44 kohm | 2459.6 ohm | +0.8 % |
| tgate worst Ron over 0.1 to 1.4 V | 2.78 kohm | 2776.1 ohm | -0.14 % |
| tgate off leakage | 0.29 nA | 0.2856 nA | -1.5 % |
| switch hold step at 0.3 V into 2 pF | -1.6 mV | -1.596 mV | +0.004 mV |
| switch hold step at 1.2 V into 2 pF | -1.7 mV | -1.730 mV | -0.030 mV (the report gives two digits) |
| switch tracking error, both levels | < 0.001 mV | 0.35 uV | inside |

## 4. What each model is, and what that means for the verdict

- inverter. The PSP103 tables of the SG13G2 NMOS (W 1.0 um) and PMOS (W 3.0 um), tt, 27 C, current in the log domain (`mos_psp103.t27`, M3c-2). The threshold is the input at which the DC transfer falls through Vdd / 2: 0.689243 V on the log tables, 0.689252 V on the linear ones; ngspice's trip point is 0.689 V and the report's 0.69 V. The delays into 20 fF are NOT MODELLED: the model's transient is validated at 500 fF (tpLH 1.344 ns, tpHL 1.018 ns) and 10 pF only, and a gate edge kicks the output 5 times further at 20 fF than at the 100 fF that `mos_psp103.t27` already says leaves the tables' 0 to Vdd range. Those 500 fF delays are not compared with the 1 to 500 ps limit: they are a different load.
- nand, nor. Two rows each. The truth table is at gate level (`sar_logic.t27`: a gate is a delay and a truth table, a value is 0, 1 or X and never a voltage). The levels in volts are the DC solution of the two-input gate on the same PSP103 log tables as the inverter (`mos_psp103_gates.t27`, M5d): the nand is two NMOS in series against two PMOS in parallel, the nor the same seen from the supply, each device the W and L of the golden netlist's (NMOS 1.0 um, PMOS 3.0 um, L 0.45 um), so no new table was made. The solver is the inverter's bisection with the stack's middle node solved inside it (a second bisection); the currents are read by the integer log-domain reader in steps of 1.164 pA and the output has a resolution of 59.6 nV. Every level of both gates is within 3 uV of ngspice's run of the same cell, and microvolts from its rail.
- tgate. The same tables and the same reader (`mos_psp103_gates.t27`): conducting, the NMOS reads (Vdd - Vout, 10 mV) and the PMOS (Vin, 10 mV), and R is 10 mV over the sum, at 27 outputs from 0.10 V to 1.40 V (the measurement of `sar_off_ron.cir`); off, both devices sit at Vgs = 0, Vds = 1.5 V and the leakage is the sum of their two table words. The leakage is a row. The two resistances are NOT MODELLED: the tables are made with source and bulk on one rail and have no body effect, and the real cell's bulks are on the rails, so ngspice's real cell is 30 and 24 percent above the model (section 3b); with each bulk tied to its own source ngspice and the model agree to 0.06 percent, which is how the cause was found. The 2592 ohm of a golden tgate in `sar_kick.t27` is a different measurement (50 C, the golden deck's own bias) and is not compared.
- dff. Gate level (`sar_logic.t27`): capture, hold, set, reset and the clock-to-Q delay are measured on it; the levels in volts are NOT MODELLED. The dff clock-to-Q is the first stage of the sequencer, Valid rise to CK11 rise with D = VDD, 546 ps (ngspice's own is 550 ps).
- dynamic_comparator. The PSP103 current tables at 50 C, the golden deck's temperature, with M4b's solver (`sar_comparator_psp.t27`). It has no charge: the devices' own capacitance is not in it, and against ngspice on the golden comparator with its charge its decision comes 98 times too fast (5.35 ps against 523 ps at +2 mV; the model is within 6.5 percent of ngspice with the charge swamped, which is the comparison the tables were built for). So the decision time FAILS the 10 ps lower limit by 4.65 ps, and the FAIL is the model's: ngspice's 523 ps on the same comparator with its charge is inside 10 to 10000 ps. The same run on the 27 C tables, the report's temperature, gives 5.12 ps and fails too. The reset row is NOT MODELLED because the run is one evaluation phase of 14 ps and the clock never falls.
- tracking_switch. A SQUARE-LAW bootstrapped switch (`sar_switch.t27`, MOSFET level 1). The tracking error at DC is measured: 1.97 uV, which is the fixed-point arithmetic floor (ngspice's own on the square-law deck is 0.03 to 0.05 uV), and it is a row because ngspice's PSP103 run of the same cell at the same conditions (`accept_switch.cir`) gives +0.29 and -0.17 uV, 2.3 uV from it. The hold step and the droop are NOT MODELLED because the square law has no gate capacitance, so no charge injection, and no leakage: the model returns a step of 0.18 uV and a droop of exactly 0, which sit far inside +-30 mV and +-5 mV by having no effect at all. That is not a measurement of the effect the report's -1.6 and -1.7 mV show, so the rows are not passes; the same cell in ngspice with the foundry's model has the effect (-1.60 and -1.73 mV) and the doc shows it beside the rows. A model with the PSP103 switch is what the two rows need.
- clock_gen, SAR_logic. Gate level with delay tables measured on the golden gates in ngspice (`sar_logic.t27`, 113 gates, the delay cell with its history); the delays are the model's own run, within 8 percent of ngspice's for every clkc edge. The slow delay is 16.145 ns (ngspice 16.147 ns), the fast one 9.822 ns (ngspice 9.669 ns, 1.6 percent), the ratio 1.644 (the report's 1.64). The stop time is taken from the eleventh Valid rise to the last clkc fall at the golden pace (40 ns): 5.704 ns (ngspice 5.807 ns); the report's 10.7 ns is not reproduced by this definition and the report's definition is not in the material this was made from, so the two are not compared. SAR_logic is driven by eleven Valid pulses of 10 ns every 20 ns, so CK11 to CK1 spans ten periods plus the 126 ps by which CK1's load makes it later than CK11.
- DAC_control, DAC_control_top, neg_bottom_plate_logic. NOT MODELLED. The DAC (`sar_cdac.t27`) has ideal capacitors driven by ideal plate sources: the level a plate reaches is its input, not an output. The eleven latches with their plate lines and the N half's follow-then-complement logic are not circuits in any spec. The polarity (the plate goes to the complement of the decision) is exercised end to end by `sar_adc.t27`, whose behavioural converter matches the golden deck's codes 30 of 31 times; that is not these blocks' check.
- integration. The behavioural converter of `sar_adc.t27`: ideal DAC capacitors, one fitted top-plate parasitic (0.63 of a 9 um x 9 um unit) and the comparator offset measured on the netlist (6.776 mV, `sar_kick.t27`), on an ideal sampling input. The 8 conversions are -1 V, -0.714 V, ... +1 V. The worst error is |code + 0.5 - ideal| = 18.5 LSB (at +1 V: the model's gain is 1.9 percent above ideal, which is what the fit found in the golden deck) and the smallest step 198 LSB (ideal 195). This is a PASS of the model that was fitted to the golden deck's 30 of 31 codes; it is not a circuit-level result.

### Context, not a verdict: the report's own ramp

The report's ideal end codes are 517 and 1579 and its ideal step is 152 LSB, which is 7 equal steps of 151.7 LSB, so its 8 conversions are equally spaced and do not start at -1 V. Rebuilt from those two numbers (the inputs are inferred, not given), our converter on the same ramp gives:

| quantity | our value | report |
|---|---|---|
| first code (ideal 517) | 512 | 510 |
| last code (ideal 1579) | 1595 | 1593 |
| worst error | 16.0 LSB | 13.8 LSB |
| smallest step | 154 LSB | 154 LSB (ideal 152) |

The four numbers agree within a few LSB. Without the measured offset the end codes are 507 and 1590. The literal -1 V to +1 V ramp is the one the verdict uses, because it is the check as stated.

## 5. Conditions that differ from the report

- Tracking switch: the report drives 2 pF at 0.3 V and 1.2 V; ours drives 3.93 pF (the DAC's top plate) at 0.2 V and 1.3 V. The tracking error scales with R C dVin/dt and is zero at DC in both; the number is ours at our conditions. ngspice's PSP103 cell is run at both sets (`accept_switch.cir`): the tracking error is under 0.4 uV at all four, and the hold step at the report's conditions is the report's. Our model was not run at 0.3 V, 1.2 V and 2 pF: the load and the levels are constants of `sar_switch.t27`, which M5d does not touch.
- Comparator: the report is at 27 C, the validated model is at 50 C (the golden deck's temperature). Both temperatures fail.
- Inverter threshold: taken as the input at which the output crosses Vdd / 2 (the quantity ngspice's trip points in `mos_psp103.t27` are). The other usual definition, the input at which the output equals the input, is not computed here.
- Integration: see above. The report's sample points are not in the material this was made from.
- The gate-level rows run on delays measured in ngspice at tt, 50 C (the golden deck's temperature), not the report's 27 C; the square-law switch has no temperature. The M5d rows are at tt, 27 C, the report's temperature.
- nand and nor: the levels are the DC solution with no load, which is the level a gate settles to into 10 fF; the report's measurement is a settled level into 10 fF, and no transient is run here.
- tgate: Ron is 10 mV over the current with 10 mV across the gate (the measurement of `sar_off_ron.cir`, at 27 C here), on 27 outputs 50 mV apart from 0.10 V to 1.40 V; the report's sweep grid and its definition of Ron are not in the material this was made from, and the reference run (ngspice, same definition, same grid) gives the report's 2.44 and 2.78 kohm to 0.8 and 0.14 percent. The leakage is at 1.5 V across the off gate, the bias at which ngspice gives the report's 0.29 nA.

## 6. Negative controls

Every check function accepts the report's own value and refuses a broken one, so none is always-FAIL or always-PASS. Where a model can be broken to leave the limit, that is done too, and the same check refuses it:

| check | the break | its value |
|---|---|---|
| inverter threshold | PMOS table zeroed | the output never falls through Vdd / 2: the reader's no-crossing value, -0.031 V, refused by 0.5 V |
| comparator sign | the inputs exchanged | the sign flips, refused |
| comparator time | a missed decision, 20 ns | refused on both sides |
| tracking error | boost capacitor removed | -25.4 mV, refused by 5 mV and, against ngspice's PSP103 cell, by the 10 uV bar |
| clock ratio | the trim code ignored | 1.0, refused by 1.2 |
| SAR_logic counts and order | the fifth dff with Set and Reset exchanged | 2, 5, 11 high instead of 1, 4, 11, wrong order, refused |
| dff set and reset | the same dff | the set row loses CK7, the clks reset sets it, refused |
| integration error | the MSB capacitor halved | 237 LSB, refused by 64 |
| nand, nor truth table | the other gate's table, one row flipped | refused |
| nand levels | the PMOS table zeroed / the NMOS table zeroed | the 1 levels fall to under 0.4 mV, 1.5 V off / the 0 level rises to 1.5 V; refused by the report's check and by the cross-check |
| nand, nor levels | the wrong gate (the nand's 1 read where the nor has a 0) | 1.5 V apart |
| tgate leakage | the NMOS table in both slots | 521.5 pA: inside the report's 1 nA, 82.6 percent over ngspice's, refused by the 10 percent bar |
| tgate Ron | the PMOS table zeroed / the NMOS table zeroed, against the no-body-effect cell | 2895 / 4225 ohm at 0.75 V, 69 and 146 percent over, refused by the 10 percent bar |
| DAC_control_top | a wrong code, a wrong plate code, two bits swapped, code and plate exchanged | refused (1430 + 617 = 2047, the complement pair) |

Four mutants of the new code were also run and each fails tests (not committed): the nor's output not mirrored (2 tests fail), the 10 percent bar loosened to 50 percent (3), the tgate's NMOS gate drive wrong (2), and the tgate's Ron verdict made to ignore the cross-check (2, the tgate's and the counts').

A limit of the control: some of the report's limits are wide, and a model error inside the limit is not seen by that check. A bridge capacitor of 2 units gives a worst error of 45.5 LSB, which the +-64 LSB limit accepts; the inverter threshold limit (0.5 to 1.0 V) accepts the two tables in each other's slots (0.811 V) and every corner's trip point (ngspice's 0.663 to 0.718 V). `sar_adc.t27` sees the bridge by the codes (0 of 31 match), `mos_psp103.t27` sees the corners by the delays. Two more, from M5d. The level cross-check of a gate does not see a device's width: the two tables in each other's slots (a 3 um stack against a 1 um pair) leave every level within 0.69 uV (nand) and 2.7 uV (nor) of ngspice's, because a level is the leakage of the off side times the resistance of the on side and both are microvolts for either width; the leakage and the resistance do see it. And the choice of the reference cell matters: against the no-body-effect cell (each bulk on its source) the model's Ron is 0.06 percent off and would have PASSED, with a number that is 30 percent from the real cell; the reference is the real cell (bulks on the rails), on purpose.

## 7. Not done, and why

- The 20 fF inverter delays, the tgate's two resistances, the comparator's reset, the tracking switch's hold step and droop, and the three DAC blocks are NOT MODELLED. Each needs a model that does not exist: a charge-aware inverter at 20 fF, a transmission gate with the body effect (the tables have none: a source-to-bulk dimension or a measured threshold shift per device, which the two tgate decks of M5d can validate on the real cell and on the cell with the bulks tied), a second clock phase for the comparator, a switch with gate capacitance and leakage (ngspice's PSP103 numbers for that cell, -1.60 and -1.73 mV and no droop, are in section 3b as the target), circuit-level plate drivers and latches. They are not approximated here.
- The nand's, the nor's and the tgate's delays into a load, and their levels in a transient: M5d does the DC levels only.
- The tracking error of our model at the report's own conditions (0.3 V and 1.2 V into 2 pF), see section 5.
- The comparator's FAIL stays a FAIL. The cause is known and measured (no charge: 98 times too fast); the charge tables for the comparator are not made.
- The corners suite needs tables at -40 and 125 C.
- A defect of the Zig emitter, found while making this, was worked around by renaming one identifier in `specs/analog/sar_kick.t27` (the field and local `half` of `KickThr` to `hw`, no value changed): the emitter types a local by the NAME of a variable or field in any spec of the closure, so a `half: f64` field in one spec made `(half as u64)` in `mos_table.t27`'s `blend1_w` (where `half` is a `u32`) a float conversion, and no spec could use both. It does not appear in any earlier closure because none had both. The repro is any spec that `use`s a function of `sar_kick.t27` and one of `mos_table.t27`; to be filed on #5980.

## 8. How to run it

```
./target/release/t27c test-report specs/analog/accept_sar.t27
./target/release/t27b test specs/analog/accept_sar.t27 --check
./target/release/t27b test specs/analog/mos_psp103_gates.t27 --check
```

`accept_sar.t27` has 14 tests (the verdict rule, one for each block or group of blocks, the reference run against the report, the corners line and the counts of blocks and of rows); `mos_psp103_gates.t27` has 6 (the nand, the nor, the gates' negative controls, the tgate without and with the body effect, the leakage). Every number in sections 3 and 3b is pinned in them. `t27b test --check` exits 0 with no jit/interp mismatch.

The ngspice numbers are the printed output of three decks, none changed after printing: `IHP_PDK=<checkout> IHP_OSDI=<osdi dir> ngspice -b specs/analog/decks/accept_gates.cir` (the eight gate levels and the off leakage), `accept_tgate.cir` (the conducting tgate, the real cell and the cell with its bulks on its sources, 27 points) and `accept_switch.cir` (the PSP103 tracking switch, four copies: tracking error, hold step, droop); ngspice-47 with the PSP103 of IHP-Open-PDK 21133f21, mos_tt, 27 C. The decks contain the W, L and m of the cells (facts of the circuit) and none of the golden netlist's text.
