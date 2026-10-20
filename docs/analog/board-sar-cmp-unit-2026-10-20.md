# One evaluator unit of the 27 C comparator on the ALINX AX7203: bit-exactness, resources, how to rerun, 2026-10-20

Issue #11380 (M3f, phase 2a of the analog epic #10486). English, ASCII only.

This is the first half of the comparator on the board: ONE evaluator unit (`specs/fpga/sar_cmp_qi_unit.t27`: one device's `qi_eval` of `specs/analog/sar_comparator_qi.t27` as a clocked machine) with its table ROM (`specs/fpga/sar_cmp_qi_tab_n.t27`, and `sar_cmp_qi_tab_p.t27` for a PMOS device), placed and routed on the `xc7a200tfbg484-2` by the probe design `specs/fpga/sar_cmp_qi_unit_probe_ax7203.t27`. The 8 x 8 solver, the board top that runs several units, the receipt and the board run are phase 2b and are not here. Nothing was loaded on the board for this document; every number below is a synthesis, place-and-route or test number.

What this document does not say: nothing about speed, power or the comparator's decision time (the unit's words are the spec's, `sar_comparator_qi.t27` gives 613.07 ps and 613.15 ps). Fmax is nextpnr's estimate for one placement of one netlist. The tests say the unit is the spec at the points they ran, not at every point.

## 1. The unit is the spec, bit for bit, at the points tested

The unit holds no table: a ROM instance beside it answers the address it asked for (one clock after the address register, registered, the word of every table at that address). Every product of the spec's reads goes through one multiply-add of two products (`r_ws = a wa + b wb`, two DSP48 multipliers), every sign-magnitude add through one add (`r_sr`), one of each a phase. The unit evaluates one device: the four current reads of `qi_eval`, the charge reads of both tables with their extensions past the edges of the tables, then the record, and serves the 20 stamps of the record (the words that go into the Newton matrix, listed in the header of the spec).

`t27b-master test specs/fpga/sar_cmp_qi_unit.t27` and `t27c test-report` (6 tests, 6 pass, 0 fail, 0 vacuous, 73387 runtime asserts):

| test | what it compares with the spec |
|---|---|
| current read | `sq_i` at 684 points over the range and beyond it (the edges, a gate drive and Vds above the supply, a source-to-bulk voltage above the supply), both device kinds: all equal |
| charge reads | `sq_qw (id_of)` and `sq_slope` of the drain and gate tables, both kinds, 3 operations, about 1800 points: all equal |
| every device, +2 mV run | all 15 devices at 18 steps of the run (`sq_run`, the clock edge and its kick, the regeneration, the settled outputs), in full at the state after the step and charge-only at the state before it: 540 evaluations, the 15 words of the record equal to `qi_eval`'s one by one, and the 72 entries of the 8 x 9 matrix that the 20 stamps make equal to the matrix of the registers of `qi_scatter` (46980 words compared, 0 differ) |
| -2 mV run and no-charge mode | the same for 6 steps of the run with the inputs exchanged, and for the mode with the charge removed (which the board's negative control will run): 270 evaluations, 0 differ |
| NEGATIVE CONTROL | one word of the log table the unit reads (bit 14 of the word at the cell of device 2 at step 150) flipped in the ROM: the current of the record differs from the spec's; one word of the drain charge table flipped: the charge differs; the ROM whole again: equal |
| slot program | the extension reads the program makes are the spec's `qi_qt_ext` count |

The branches of the program are all run by the states tested (a gate beyond its source, a gate drive above the supply, a Vds above the supply, a forward bulk, a source-to-bulk voltage above zero: each counted and each non-zero). `t27b-master test --check` exits 0 with no MISMATCH for all four specs; for the unit it prints `runtime asserts unknown`, because t27b's interpreter (20 million steps a file) runs out of fuel in the long tests, the JIT ran all six and they pass.

Clocks, counted by the tests with the ROM answering as the board's does (the go clock included):

| | clocks |
|---|---|
| one current read (`sq_i`) | 43 to 50 |
| one charge read | 16 (a slope) or 17 (a value) |
| one device, full (the Jacobian wanted) | **506 to 674** |
| one device, charge-only (the previous step's charge) | **179 to 347** |

A step of the comparator is 15 charge-only evaluations and 4 x 15 full ones, plus 4 solves (phase 2b); with `U` units in parallel and the devices spread evenly the evaluations are about `ceil(15 / U)` times (179 to 347 + 4 x (506 to 674)) clocks, none of which is measured here.

## 2. Resources

The ROM is reported apart from the logic, because the tables can be shared between units (one ROM per device kind could serve several units if the top arbitrates the address; this probe has one ROM for its one unit).

yosys 0.63, `synth_xilinx -family xc7 -flatten -abc9 -nosrl` (DSP slices on), the memory mapping of section 3. LUT is the sum of LUT2..LUT6.

| | LUT | FF | CARRY4 | DSP48E1 | RAMB18E1 | RAMB36E1 |
|---|---|---|---|---|---|---|
| the unit alone (logic, no ROM, free ports) | 9850 | 5084 | 624 | 15 | 0 | 0 |
| the NMOS table ROM alone | 239 | 2 | 0 | 0 | 105 | 2 |
| the PMOS table ROM alone | 239 | 2 | 0 | 0 | 105 | 2 |
| the probe (unit + NMOS ROM + top: device 2, the inputs partly constant) | 9525 | 5228 | 632 | 15 | 105 | 2 |

The ROM's block RAM is 105 RAMB18E1 and 2 RAMB36E1, 54.5 RAMB36 equivalents for the nine tables of one kind (the seven slices of the log current table, 4851 words each, and the drain and gate charge tables, 625 words each). That is 14.9 percent of the device's 730 RAMB18; it is the same for either kind.

After placement and routing (nextpnr-xilinx, `--freq 100`, the probe design, the third attempt below):

| | |
|---|---|
| SLICE_LUTX (nextpnr's LUT sites) | 14208 of 269200 (5 percent) |
| SLICE_FFX | 5228 of 269200 (1 percent) |
| CARRY4 | 648 of 33650 |
| DSP48E1 | 15 of 740 (2 percent) |
| RAMB18E1 / RAMB36E1 | 105 of 730 / 2 of 365 |
| Fmax before routing (first line) | 54.68 MHz (FAIL at 100) |
| **Fmax after routing (second line)** | **59.46 MHz (FAIL at 100)** |

**The unit does not close 100 MHz; it closes about 59 MHz.** Three builds of the same probe, the unit's phases rearranged between them:

| build | what changed | LUT (yosys) | Fmax before / after routing | what limited it |
|---|---|---|---|---|
| 1 | as written | 11016 | 20.08 / 29.97 MHz | a chain of four sign-magnitude adds in the source-charge closure, in one phase (33 ns) |
| 2 | one sign-magnitude add a phase (`r_sr`, registered like `r_ws`), the sums of the stamps made once at the finish, the cell address and the exponential's table read in two phases | 9476 | 46.83 / 55.79 MHz | the exponent branch of the current: an add of the product, a shift and a compare in one phase (18 ns) |
| 3 | that branch, the table's interpolation, the series and the charge word each in two phases, the direction of a slope difference kept in a register | 9525 | 54.68 / 59.46 MHz | the decode of the phase register into the enables of the registers: about nine levels of LUT (16.8 ns) |

A further gain wants the phase register one-hot (or the decode registered), which is a change to the unit's control and not a rearrangement of its arithmetic; it was not made here. The top of phase 2b can run this unit at 50 MHz (the PLL's divide of 32) with a margin, and the clocks above are the same at any clock.

### What the numbers mean for phase 2b (arithmetic on the numbers above, NOT measured)

- One unit per device would be 15 units: 15 x 9850 = 147750 LUT by the yosys count, more than the device's 134600 six-input LUTs (33650 slices of 4). **Fifteen units do not fit as this unit is written** (nextpnr's own site count, 14208 a unit of 269200 sites, would make 15 of them 79 percent, but a site is half a LUT6 and the yosys count is the bound). Five units are 49250 LUT (37 percent), eight are 78800 (59 percent), by the yosys count.
- Tables: a private ROM for each unit is 54.5 RAMB36 equivalents a unit (about 15 percent of the device each, the same for an NMOS and a PMOS ROM); a ROM shared by the units of a kind is 54.5 for the kind, 109 for both kinds (30 percent), at the price of a top that arbitrates the address between the units. The whole of the six units' private ROMs, 327 RAMB36 equivalents, is 90 percent of the device.
- The unit's 15 DSP48E1 are the two multipliers of the multiply-add (`ws_prod`, operands of 48 and 32 bits) and the small products around it, not itemized; at 15 a unit, eight units are 120 of the 740 slices.

## 3. How it was built and how to rerun it

`$O` is the shared openXC7 build directory, `$B` a scratch output directory, `$T` a t27b built from master at or after 2195f7486 (rung 1 of the Verilog path, #11376: it reads the PSP103 table consts by `use` and writes them as ROM functions, signed integers, constant division; the t27b that predates it refuses an index in `on_clock`). Never build in the repository.

```
$T verilog specs/fpga/sar_cmp_qi_unit.t27 > $B/unit.v                       # module FpgaSarCmpQiUnit, about 2300 lines
$T verilog specs/fpga/sar_cmp_qi_tab_n.t27 > $B/tab_n.v                     # module FpgaSarCmpQiTabN, a ROM function for each of the nine tables
$T verilog specs/fpga/sar_cmp_qi_unit_probe_ax7203.t27 > $B/probe.v         # module FpgaSarCmpQiUnitProbe: PLL, one unit, one ROM, the pins as attributes
yosys -l $B/probe.log $B/probe.ys
  # probe.ys:
  read_verilog -sv $B/probe.v $B/unit.v $B/tab_n.v
  synth_xilinx -family xc7 -top FpgaSarCmpQiUnitProbe -flatten -abc9 -nosrl -run :coarse
  techmap -map +/cmp2lut.v -map +/cmp2lcu.v -D LUT_WIDTH=6
  alumacc
  opt
  memory -nomap
  opt_clean
  setattr -set ram_style "block" t:$mem_v2 r:SIZE>=512 %i
  synth_xilinx -family xc7 -top FpgaSarCmpQiUnitProbe -flatten -abc9 -nosrl -run map_memory:
  stat
  write_json $B/probe.json
$O/np-main/build/nextpnr-xilinx --chipdb $O/xc7a200tfbg484-2.bin --json $B/probe.json --fasm $B/probe.fasm --freq 100 --log $B/probe.pnr.log
PYTHONPATH=$O/prjxray $O/venv/bin/python3 $O/prjxray/utils/fasm2frames.py --db-root $O/prjxray-db/artix7 --part xc7a200tfbg484-2 $B/probe.fasm $B/probe.frames
```

The tests: `$T test specs/fpga/sar_cmp_qi_unit.t27` (and `--check`), `t27c test-report specs/fpga/sar_cmp_qi_unit.t27`; the same for `sar_cmp_qi_tab_n.t27`, `sar_cmp_qi_tab_p.t27` and `sar_cmp_qi_unit_probe_ax7203.t27` (1 test each, all pass, 0 vacuous).

Third build, for the record: `unit.v` sha256 `7ca369dda2152bff...`, `tab_n.v` `0b5b11e5a403725f...`, `probe.v` `043b436b021ad18e...`, `$T` `b36baec6e72b129c...`, yosys 0.63 (git 70a11c6bf0e8dd669f56c7da3587f78b405138e2), chipdb `xc7a200tfbg484-2.bin`. `fasm2frames` ran to the end on the routed design (20230 frames lines, as the inverter's); the frames were not made into a bitstream and nothing was loaded. The `.fasm`, `.frames` and `.json` were deleted after each run.

### Traps found

1. **A phase chain written as `else if` is read by yosys in tens of minutes.** The first form of the unit (one `if .. else if ..` chain of ninety phases) did not finish in the Verilog frontend after ten minutes; the same unit with the phase held in a constant at the start of the call (`const p = r_ph;`) and every phase its own `if` reads in seconds and is the same machine (the tests run it unchanged).
2. **`setattr -set ram_style "block"` on every `$mem_v2` fails** (`no valid mapping found for memory`) as soon as the design has a small asynchronous ROM (the 129-word table of 2^-x): such a memory cannot be a block RAM. The attribute is set only on the memories of 512 words or more (`r:SIZE>=512 %i`). The ROM's output registers must also have no reset and no enable, or yosys does not merge them into the block RAM's read register: both ports of a ROM module are tied by constants named `rst_n` and `en`.
3. **A table read twice in one clock is two ROMs.** Two reads of VSB_N1 in one module mapped to two block RAMs; the ROM module reads each table once a clock and the unit takes four clocks to fetch the four nodes of a cell.
4. **`x % 3` and `x / 6` with a constant divisor are a 64 x 65 multiplier** in the t27b Verilog (the multiply-shift of the plan); the unit has no `%` and does its one `/ 6` on a 16-bit word.
5. **The merge hook path.** In a worktree the configured hooks path is the main checkout's, which wants a t27c at the worktree root; `git -c core.hooksPath=.githooks commit` runs the worktree's own.

## 4. Not measured

- Anything on the board: no bitstream was made, nothing was loaded, the unit ran only in the tests (a software run of the same functions, clock by clock, with the ROM modelled).
- More than one unit, the arbitration of a shared ROM, the 8 x 8 solver, the Newton step, the receipt: phase 2b.
- Another placement: the Fmax is one run of nextpnr's default seed on one netlist; the three builds above differ in the netlist, not the seed.
- The unit at the Newton iterates (the tests evaluate it at the converged states of the run, 18 steps of 438, and at the 6 steps of the other run), or at an input below 10 uV.
- Power, noise, speed against ngspice (the unit's clocks are a count of this machine, not a comparison).
- The routed design's behavior on silicon: DSP48E1 (15 of them) are used here for the first time in this repository's open-flow builds (the inverter's had none); that they place, route and convert to frames is shown, that they compute on the die is not.
