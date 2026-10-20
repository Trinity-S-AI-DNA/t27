# The external team's per-block checks applied to our SAR ADC blocks, 2026-10-20

Issue #11212, milestone M5c of the analog epic #10486, night loop #11063. English, ASCII only.

The external team's report of 2026-10-16 lists, for each of the 13 blocks of the golden SAR ADC, a measured value and the limit that its check enforces (conditions: tt corner, 27 C, VDD 1.5 V, cells taken from the integrated converter's netlist). The team confirmed on 2026-10-20 that the checks and their limits may be used and published here. `specs/analog/accept_sar.t27` holds those limits as named constants, one pure check function per quantity, and one function per block that measures the quantity on OUR model by `use` of the spec that holds it. This document is the result. Every "our value" below is computed by a named function of that spec and pinned by a test; every "report" value is the report's, and is also in the spec as the positive control of its check (the report's own number must pass the report's own check).

What this document does not say: nothing about a transistor-level result for any gate-level block, nothing about the voltage level of any digital node, nothing about power, noise, mismatch, another corner or temperature, or speed. No model was tuned to turn a FAIL into a PASS, and none was changed to make a number come out.

## 1. The verdict rule

A row is one quantity of the report's check.

- PASS: our model produced the value and it is inside the report's limit.
- FAIL: our model produced the value and it is outside the limit. It is reported as measured.
- NOT MODELLED: our model does not produce the quantity the check enforces. Nothing is compared, and the row is never a pass, whatever a constant might have said (`as_verdict(false, x)` is NOT MODELLED for both values of `x`; a test pins it).

A block is FAIL when any row fails, PASS only when it has rows and every row is PASS, and NOT MODELLED otherwise. A block whose threshold passes and whose delay is not modelled is therefore not a pass: it is NOT MODELLED, and its passing row is still shown.

## 2. The counts

Over the 13 blocks of the report (inverter, nand, nor, tgate, dff, dynamic_comparator, tracking_switch, clock_gen, SAR_logic, DAC_control, DAC_control_top, neg_bottom_plate_logic, the 11-bit integration run):

| verdict | blocks | which |
|---|---|---|
| PASS | 3 | clock_gen, SAR_logic, integration |
| FAIL | 1 | dynamic_comparator (decision time) |
| NOT MODELLED | 9 | inverter, nand, nor, tgate, dff, tracking_switch, DAC_control, DAC_control_top, neg_bottom_plate_logic |

Over the 30 rows: 16 PASS, 1 FAIL, 13 NOT MODELLED. Five of the nine NOT MODELLED blocks have at least one measured, passing row (inverter 1, nand 1, nor 1, dff 2, tracking_switch 1); four have none (tgate, DAC_control, DAC_control_top, neg_bottom_plate_logic). The PASS of the three passing blocks is the PASS of the model named in section 4: gate level for clock_gen and SAR_logic, a behavioural converter fitted to the golden deck for integration.

The report's corners smoke suite (SAR inverter delay 62 to 124 ps, comparator decision 467 to 829 ps over 5 corners at -40, 27 and 125 C) is a test run, not a block, and is not in the 13. It is NOT MODELLED (last row of the table).

## 3. The table

Times in ps or ns as written. "Report" is the report's measured value; "limit" is what its check enforces. Our value is from the function named in the last column.

| block | check | our value | report | limit | verdict | function |
|---|---|---|---|---|---|---|
| inverter | switching threshold | 0.6892 V | 0.69 V | 0.5 to 1.0 V | PASS | `as_inv_vth_of` |
| inverter | delays fall / rise into 20 fF | not produced | 81 / 100 ps | 1 to 500 ps | NOT MODELLED | |
| nand | truth table 00, 01, 10, 11 (logic level) | 1, 1, 1, 0 | rows right | exact | PASS | `sl_nand_code` |
| nand | levels into 10 fF | not produced | 1.5 V for a 1, microvolts for a 0 | 1.45 to 1.55 V; +-50 mV | NOT MODELLED | |
| nor | truth table 00, 01, 10, 11 (logic level) | 1, 0, 0, 0 | rows right | exact | PASS | `sl_nor_code` |
| nor | levels into 10 fF | not produced | 1.5 V for a 1, microvolts for a 0 | 1.45 to 1.55 V; +-50 mV | NOT MODELLED | |
| tgate | Ron at 0.75 V | not produced | 2.44 kohm | <= 30 kohm | NOT MODELLED | |
| tgate | worst Ron over 0.1 to 1.4 V | not produced | 2.78 kohm | <= 50 kohm | NOT MODELLED | |
| tgate | off leakage | not produced | 0.29 nA | <= 1 nA | NOT MODELLED | |
| dff | edge capture, hold, set, reset (logic level, 11 stages) | all right | all right | exact | PASS | `as_sar_run`, `as_dff_set_reset` |
| dff | levels of the 8 observation points | not produced | all right | +-50 mV | NOT MODELLED | |
| dff | clock-to-Q | 546 ps | 524 ps | 1 to 3000 ps | PASS | `as_sar_run` |
| dynamic_comparator | sign at +-2 mV, 0.75 V common mode | +2 mV: dp wins; -2 mV: dn wins | right | right | PASS | `as_cmp` |
| dynamic_comparator | levels after the decision | winner 1.49996 V, loser 3.6 mV | right | +-50 mV | PASS | `as_cmp` |
| dynamic_comparator | reset | not produced | right | +-50 mV | NOT MODELLED | |
| dynamic_comparator | decision time at +-2 mV | 5.35 ps | 607 ps | 10 to 10000 ps | **FAIL** | `as_cmp` |
| tracking_switch | tracking error, DC (our levels 0.2 V and 1.3 V into 3.93 pF) | -1.97 uV at both | < 0.001 mV at 0.3 V and 1.2 V into 2 pF | +-5 mV | PASS | `as_sw` |
| tracking_switch | hold step | not produced | -1.6 / -1.7 mV | +-30 mV | NOT MODELLED | |
| tracking_switch | droop | not produced | none | +-5 mV | NOT MODELLED | |
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

## 4. What each model is, and what that means for the verdict

- inverter. The PSP103 tables of the SG13G2 NMOS (W 1.0 um) and PMOS (W 3.0 um), tt, 27 C, current in the log domain (`mos_psp103.t27`, M3c-2). The threshold is the input at which the DC transfer falls through Vdd / 2: 0.689243 V on the log tables, 0.689252 V on the linear ones; ngspice's trip point is 0.689 V and the report's 0.69 V. The delays into 20 fF are NOT MODELLED: the model's transient is validated at 500 fF (tpLH 1.344 ns, tpHL 1.018 ns) and 10 pF only, and a gate edge kicks the output 5 times further at 20 fF than at the 100 fF that `mos_psp103.t27` already says leaves the tables' 0 to Vdd range. Those 500 fF delays are not compared with the 1 to 500 ps limit: they are a different load.
- nand, nor, tgate, dff. Gate level (`sar_logic.t27`): a gate is a delay and a truth table, a transmission gate a switch with a stored node, a value is 0, 1 or X and never a voltage. Truth tables, capture, hold, set, reset and the clock-to-Q delay are measured on it. The levels in volts are NOT MODELLED, and so are the tgate's Ron and leakage (no spec models them; the 2592 ohm of a golden tgate in `sar_kick.t27` is an ngspice measurement, not ours). The dff clock-to-Q is the first stage of the sequencer, Valid rise to CK11 rise with D = VDD, 546 ps (ngspice's own is 550 ps).
- dynamic_comparator. The PSP103 current tables at 50 C, the golden deck's temperature, with M4b's solver (`sar_comparator_psp.t27`). It has no charge: the devices' own capacitance is not in it, and against ngspice on the golden comparator with its charge its decision comes 98 times too fast (5.35 ps against 523 ps at +2 mV; the model is within 6.5 percent of ngspice with the charge swamped, which is the comparison the tables were built for). So the decision time FAILS the 10 ps lower limit by 4.65 ps, and the FAIL is the model's: ngspice's 523 ps on the same comparator with its charge is inside 10 to 10000 ps. The same run on the 27 C tables, the report's temperature, gives 5.12 ps and fails too. The reset row is NOT MODELLED because the run is one evaluation phase of 14 ps and the clock never falls.
- tracking_switch. A SQUARE-LAW bootstrapped switch (`sar_switch.t27`, MOSFET level 1). The tracking error at DC is measured: 1.97 uV, which is the fixed-point arithmetic floor (ngspice's own is 0.03 to 0.05 uV). The hold step and the droop are NOT MODELLED because the square law has no gate capacitance, so no charge injection, and no leakage: the model returns a step of 0.18 uV and a droop of exactly 0, which sit far inside +-30 mV and +-5 mV by having no effect at all. That is not a measurement of the effect the report's -1.6 and -1.7 mV show, so the rows are not passes. A model with the PSP103 switch is what the two rows need.
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

- Tracking switch: the report drives 2 pF at 0.3 V and 1.2 V; ours drives 3.93 pF (the DAC's top plate) at 0.2 V and 1.3 V. The tracking error scales with R C dVin/dt and is zero at DC in both; the number is ours at our conditions.
- Comparator: the report is at 27 C, the validated model is at 50 C (the golden deck's temperature). Both temperatures fail.
- Inverter threshold: taken as the input at which the output crosses Vdd / 2 (the quantity ngspice's trip points in `mos_psp103.t27` are). The other usual definition, the input at which the output equals the input, is not computed here.
- Integration: see above. The report's sample points are not in the material this was made from.
- The gate-level rows run on delays measured in ngspice at tt, 50 C (the golden deck's temperature), not the report's 27 C; the square-law switch has no temperature.

## 6. Negative controls

Every check function accepts the report's own value and refuses a broken one, so none is always-FAIL or always-PASS. Where a model can be broken to leave the limit, that is done too, and the same check refuses it:

| check | the break | its value |
|---|---|---|
| inverter threshold | PMOS table zeroed | the output never falls through Vdd / 2: the reader's no-crossing value, -0.031 V, refused by 0.5 V |
| comparator sign | the inputs exchanged | the sign flips, refused |
| comparator time | a missed decision, 20 ns | refused on both sides |
| tracking error | boost capacitor removed | -25.4 mV, refused by 5 mV |
| clock ratio | the trim code ignored | 1.0, refused by 1.2 |
| SAR_logic counts and order | the fifth dff with Set and Reset exchanged | 2, 5, 11 high instead of 1, 4, 11, wrong order, refused |
| dff set and reset | the same dff | the set row loses CK7, the clks reset sets it, refused |
| integration error | the MSB capacitor halved | 237 LSB, refused by 64 |
| nand, nor | the other gate's table, one row flipped | refused |
| DAC_control_top | a wrong code, a wrong plate code, two bits swapped, code and plate exchanged | refused (1430 + 617 = 2047, the complement pair) |

A limit of the control: some of the report's limits are wide, and a model error inside the limit is not seen by that check. A bridge capacitor of 2 units gives a worst error of 45.5 LSB, which the +-64 LSB limit accepts; the inverter threshold limit (0.5 to 1.0 V) accepts the two tables in each other's slots (0.811 V) and every corner's trip point (ngspice's 0.663 to 0.718 V). `sar_adc.t27` sees the bridge by the codes (0 of 31 match), `mos_psp103.t27` sees the corners by the delays.

## 7. Not done, and why

- The 20 fF inverter delays, the tgate's Ron and leakage, the comparator's reset, the tracking switch's hold step and droop, and the three DAC blocks are NOT MODELLED. Each needs a model that does not exist: a charge-aware inverter at 20 fF, a transmission gate from the PSP103 tables, a second clock phase for the comparator, a switch with gate capacitance and leakage, circuit-level plate drivers and latches. They are not approximated here.
- The comparator's FAIL stays a FAIL. The cause is known and measured (no charge: 98 times too fast); the charge tables for the comparator are not made.
- The corners suite needs tables at -40 and 125 C.
- A defect of the Zig emitter, found while making this, was worked around by renaming one identifier in `specs/analog/sar_kick.t27` (the field and local `half` of `KickThr` to `hw`, no value changed): the emitter types a local by the NAME of a variable or field in any spec of the closure, so a `half: f64` field in one spec made `(half as u64)` in `mos_table.t27`'s `blend1_w` (where `half` is a `u32`) a float conversion, and no spec could use both. It does not appear in any earlier closure because none had both. The repro is any spec that `use`s a function of `sar_kick.t27` and one of `mos_table.t27`; to be filed on #5980.

## 8. How to run it

```
./target/release/t27c test-report specs/analog/accept_sar.t27
./target/release/t27b test specs/analog/accept_sar.t27 --check
```

The spec has 13 tests (the verdict rule, one for each block or group of blocks, the corners line and the counts); every number in section 3 is pinned in them. `t27b test --check` exits 0 with no jit/interp mismatch.
