# t27 analog emulation v0.1 -- one-page summary for the external team

Date 2026-10-24. Tag `analog-v0.1.0`. Full notes with every source: `docs/analog/v0.1-release-notes.md`. English, ASCII only. Epic #10486.

## What you can do with v0.1

Give a SPICE netlist with IHP SG13G2 (PSP103) devices; get `.t27` specs that are at once the software reference and the FPGA design, checked against ngspice-47 and VACASK with the same OSDI model. The reference circuit is the SAR ADC of the golden deck (16 subcircuits, 1265 MOS). The AX7203 board is the bench.

## Where the numbers stand

| question | answer | source |
|---|---|---|
| Do your per-block checks run on our models? | yes: 13 blocks, 9 PASS, 0 FAIL, 4 NOT MODELLED (dff, DAC_control, DAC_control_top, neg_bottom_plate_logic); 30 rows, 26 / 0 / 4 | #11215, #11421 |
| Is the dynamic comparator right? | decision time 613.07 ps against ngspice's 602.54 ps (+1.75 %) and your report's 607 ps; the same model with its charge removed decides in 5.40 ps and is refused | #11325 |
| Does a PSP103 circuit run on the board? | yes, one: the tt inverter, 1217 of 1217 receipt lines equal to the spec, 1426 LUT, 1219 FF, 0 DSP, tables in block RAM, 103.61 MHz after routing | #11220 |
| Which corners? | tables and ngspice checks for all five (tt, ss, ff, fs, sf), each within 0.734 % at worst; a bitstream for tt only | #11123, #11214 |
| Does the converter reproduce your published run? | our ngspice-47 run of the golden deck gives 30 of 31 codes of your RAW file; the converter model is predicted only with two fitted numbers | #10854, #10953 |
| Are the waveforms usable in your tools? | yes: a `.t27` RAW writer; ngspice reads our files, largest difference 0 on the 8-conversion trace | #10940, #11151 |
| Is it fast? | no claim: one 100 ps step takes 17.0 to 19.6 us on the board, 170000 to 196000 times slower than real time; ngspice is 195623 times slower on the same circuit, for different work | #11313 |

## What is open

- Four corners have no bitstream yet.
- The comparator step on the board is one routed unit that does not meet 100 MHz (59.46 MHz) and was never loaded; the board version waits for the native backend rung 1 (#11376).
- The late decisions of the converter are met against ngspice's in-situ balance but 3 of 4 against the pooled thresholds of the issue (#10942).
- The full demo of your VACASK example stops in hbac1 on the VACASK we obtained (wheel 0.3.4.dev3).
- Your own evaluator and testbenches were not run by us; `accept_sar.t27` implements the checks and limits you confirmed we may publish.

## What would help us most

1. The block and the criterion for your second circuit (asked 2026-10-20, no answer yet); we continue on the SAR ADC until then.
2. A newer VACASK wheel, if one exists, so that the full demo runs.
3. Your word on which of the four NOT MODELLED blocks matters first.

Unless told otherwise we take the order: dff levels, DAC_control, then one more corner on the board.
