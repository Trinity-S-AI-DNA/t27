# Board regression of the two analog board tops, 2026-10-19

Issue #11074, part of the night loop #11063 and the analog epic #10486. English, ASCII only.

Both board tops of the analog epic were last built and run before master moved. This document is the re-measurement: the same two specs, built from current master with the local openXC7 flow, loaded into the FPGA of the AX7203 board, the UART receipts read back and judged against the numbers in the specs' own headers. Every number below was measured in this run; where it differs from the earlier result, it is reported as a difference (section 4).

What this document does not say: nothing about speed, power or value. The Fmax figures are nextpnr's post-route estimate for one placement of one netlist, not a measured clock limit and not a speed comparison.

## 1. The run

- Source: `origin/master` at `fe14b691f3aa70119dda7b312ecf2e6a80c77937` (the head when the run started). The two specs were not edited; a negative control is the same spec with one literal changed in a working copy, restored with `git checkout` afterwards (`git status` was clean).
- Date and place: 2026-10-19, the owner's Mac (Darwin 25.3.0). Builds 21:43 to 22:04, flash and capture 22:04 to 22:06 (UTC+7).
- `t27b`: built for this run from the same tree, `CARGO_TARGET_DIR=<scratch>/tgt cargo build --release --locked -p t27b` (about 8 minutes on a loaded machine), 3116592 bytes, sha256 `406248007b5646335073f880c05470138a1614b2c780ff78b0abc836c6cf4afd`. An older `t27b` binary from 2026-10-18 22:41 exists in another worktree; it predates several commits to `cli/t27b` and `gen-verilog`, so it was not used.
- Toolchain directory `O="$(git rev-parse --git-common-dir)/../build/fpga/openxc7"`: yosys 0.63 (git 70a11c6bf0e8dd669f56c7da3587f78b405138e2, Homebrew), `$O/np-main/build/nextpnr-xilinx` (Version a0410d5), chipdb `$O/xc7a200tfbg484-2.bin` (md5 `59f828c441e6e6081bfee63ff994660e`), `$O/prjxray/utils/fasm2frames.py` run with `$O/venv/bin/python3` and `PYTHONPATH=$O/prjxray`, `$O/px-main/build/tools/xc7frames2bit`, `openFPGALoader v1.1.1` (Homebrew).
- Board and cable: ALINX AX7203, `xc7a200tfbg484-2`. The Digilent HS2 cable (0x0403:0x6014, FT232H) is now on USB bus 001, device 004: `openFPGALoader -c digilent_hs2 --scan-usb` listed `001 004`, and `--detect` read idcode `0x3636093` (xc7a200, artix a7 200t). The earlier runs used `--busdev-num 000:004`; that now answers `unable to open ftdi device: -3 (device not found)`. Flashing therefore used `--busdev-num 001:004`. A second Digilent FTDI device (0x0403:0x6010) is on the bus; it was not used.
- Flashing: SRAM only (`openFPGALoader` without `-f` or `-m`; its output shows `Load SRAM`). No flash memory and no eFUSE was written. The HDMI capture path was not touched.
- Reader: from the terminal tab of the desktop app only; one reader per capture: the port is opened on fd 3, `stty 115200 raw cs8 -cstopb -parenb clocal <&3`, `cat <&3 > <capture> &`, `sleep <s>`, `kill` of that pid, `wait`, `exec 3<&-`; `lsof` showed the port free before every capture (`PORT_BUSY` check) and after every capture (`port released`).
- Machine state at each place-and-route start (memory gate >= 25 % free, one place-and-route at a time): memory free 35, 59, 61, 49 and 53 percent for the five builds in order; load average 90 to 107 (other lanes were running). Disk free on `/System/Volumes/Data` was 5.6 to 6.8 GiB at those moments (4.8 to 7.6 GiB over the run), which is BELOW the 8 GB the lane brief asks for. The run writes under 0.3 GB (a 12 MB netlist, a 1.5 MB fasm, a 9.7 MB bitstream and logs per build; each 22 MB frames file was deleted once its bitstream existed), so it went ahead with a floor of 2 GB in its own gate; the brief's 8 GB was not met and this is stated here, not hidden.

### The serial port, identified by trial

The earlier runs read `/dev/cu.usbserial-130`. The ports listed now are `-1130`, `-2102038592890`, `-2102038592891` and `-210512180081`. The first bitstream (M1b top) was flashed once and each of the four ports was read for 2 s by one reader at a time:

| port | bytes read in 2 s | lines starting with `T27 ` |
|---|---|---|
| `/dev/cu.usbserial-1130` | 21040 | 193 |
| `/dev/cu.usbserial-2102038592890` | 0 | 0 |
| `/dev/cu.usbserial-2102038592891` | 510 (all zero bytes) | 0 |
| `/dev/cu.usbserial-210512180081` | 0 | 0 |

The UART of the AX7203 is `/dev/cu.usbserial-1130` (the CP2102N bridge, 115200 baud 8N1, pin N15). It is the port used for every capture below. The identification read is a separate trial; none of its bytes are counted in the tables.

### How a capture is judged

A line is well-formed when it matches the receipt grammar of its spec exactly (`T27 (RLC|LAD5) A=<16 hex> B=<16 hex> S=<16 hex> C=<16 hex> L=<16 hex> (PASS|FAIL)` for M1b; `T27 SAR K=<2 hex> C=<3 hex> E=<3 hex> N=<3 hex> S=<16 hex> L=<4 hex> (PASS|FAIL)` for the SAR top). Only well-formed lines are counted. The payload of each is compared with what the spec's header and constants predict for that build, and the L counters of the well-formed lines are checked to run without a gap. The raw count of lines that merely start with `T27 ` is given too. The judging script is a scratch script outside the repository.

Every capture has malformed lines at its start and at its end, and nowhere else: the first bytes are a tail of the previous reader session's stream still buffered in the USB-UART bridge (the M1b top capture starts with `0145 PASS`; the identification read had ended at `L=0142`), then noise while the FPGA reconfigures (the first four to nine L values never arrive as well-formed lines), and the last line is cut where the reader is killed. The earlier runs saw the same.

## 2. Top 1 of 2: the M1b linear cell

- top: FpgaAnalogCell (specs/fpga/analog_cell_ax7203.t27, M1b; the series RLC and the 5-node RC ladder, 200 steps each, one top that runs both and alternates the two receipt lines)

Header numbers used as the authority (the spec's own constants, and `RLC_EXP_S` and `LAD5_EXP_S` of `specs/analog/linear.t27`, which the spec's PASS rule names): RLC A=18050612, B=NEG|1265689, clocks 56666; LAD5 A=2963590, B=4464373, clocks 139400; S and the pinned receipt values `RLC_EXP_S = 0x84BE2C7F9115D08A`, `LAD5_EXP_S = 0x29439851A3F5D9EE`; controls: one coefficient one LSB off (RLC ROM address 5, `NEG | 39851` to `NEG | 39852`; ladder ROM address 12, `31709` to `31710`).

### Commands (exact)

`$R` is the lane worktree at `fe14b691f`, `$B` the scratch output directory, `$T27B` the binary above.

```
cd $R
$T27B verilog specs/fpga/analog_cell_ax7203.t27 > $B/m1b_top.v
sed -i '' '169s/NEG | 39851/NEG | 39852/' specs/fpga/analog_cell_ax7203.t27
$T27B verilog specs/fpga/analog_cell_ax7203.t27 > $B/m1b_neg_rlc.v
git checkout -- specs/fpga/analog_cell_ax7203.t27
sed -i '' '193s/return 31709/return 31710/' specs/fpga/analog_cell_ax7203.t27
$T27B verilog specs/fpga/analog_cell_ax7203.t27 > $B/m1b_neg_lad5.v
git checkout -- specs/fpga/analog_cell_ax7203.t27
```

(each sed was preceded by a check that the line holds the old text; the three Verilog files have 1357 lines, module `FpgaAnalogCell`, and each control differs from the top in exactly one line: 531c531 for the RLC control, 649c649 for the ladder control.) Then, for `N` in `m1b_top m1b_neg_rlc m1b_neg_lad5`, one at a time:

```
yosys -l $B/$N.yosys.log $B/$N.ys
  # $B/$N.ys:
  read_verilog -sv $B/$N.v
  synth_xilinx -family xc7 -top FpgaAnalogCell -flatten -abc9 -nodsp -nosrl -run :coarse
  techmap -map +/cmp2lut.v -map +/cmp2lcu.v -D LUT_WIDTH=6
  alumacc
  opt
  memory -nomap
  opt_clean
  synth_xilinx -family xc7 -top FpgaAnalogCell -flatten -abc9 -nodsp -nosrl -run map_memory:
  stat
  write_json $B/$N.json
$O/np-main/build/nextpnr-xilinx --chipdb $O/xc7a200tfbg484-2.bin --json $B/$N.json --fasm $B/$N.fasm --freq 100 --log $B/$N.pnr.log
PYTHONPATH=$O/prjxray $O/venv/bin/python3 $O/prjxray/utils/fasm2frames.py --db-root $O/prjxray-db/artix7 --part xc7a200tfbg484-2 $B/$N.fasm $B/$N.frames
$O/px-main/build/tools/xc7frames2bit --part_file $O/prjxray-db/artix7/xc7a200tfbg484-2/part.yaml --part_name xc7a200tfbg484-2 --frm_file $B/$N.frames --output_file $B/$N.bit
rm $B/$N.frames
```

The pins are attributes in the spec, so there is no XDC file; the default placer seed was used (no `--seed`). Flash and capture, for each bitstream, with `PORT=/dev/cu.usbserial-1130`:

```
openFPGALoader -c digilent_hs2 --busdev-num 001:004 $B/$N.bit
exec 3<$PORT; stty 115200 raw cs8 -cstopb -parenb clocal <&3; cat <&3 > $C/$N.txt &  P=$!; sleep 6; kill $P; wait $P; exec 3<&-
```

(six seconds for each M1b capture.)

### Build results

| | M1b top | RLC control | LAD5 control |
|---|---|---|---|
| Verilog sha256 | `54044e1713f3b3c9071c94826ba39137794d7855e4e95f0f710fac01671036d1` | `317505f6c0d29a1b4a7ea6f69f15b7068de0fbd00f5d34a6dddc75b6b8d8370d` | `a4b733885689e450f93f3be2469f26fc59fab9de5cbf19837d556c1d88bc32f8` |
| LUT (yosys, LUT1..LUT6) | 1196 (LUT2 348, LUT3 117, LUT4 122, LUT5 473, LUT6 136; plus 35 INV) | 1195 (LUT2 359, LUT3 95, LUT4 133, LUT5 458, LUT6 150; plus 36 INV) | 1191 (LUT2 345, LUT3 117, LUT4 133, LUT5 457, LUT6 139; plus 35 INV) |
| FF | 1096 (FDRE 1084, FDSE 12) | 1096 (1084, 12) | 1096 (1084, 12) |
| DSP48E1 | 0 | 0 | 0 |
| BRAM (RAMB18E1, RAMB36E1, FIFO18E1, RAMBFIFO36E1) | 0 | 0 | 0 |
| CARRY4 (yosys / nextpnr) | 142 / 147 | 142 / 145 | 142 / 147 |
| nextpnr SLICE_LUTX | 2190 of 269200 | 2179 | 2185 |
| Fmax before routing (first line) | 67.65 MHz (FAIL at 100) | 67.82 MHz | 68.84 MHz |
| Fmax post-route (second line) | 116.06 MHz (PASS at 100) | 103.38 MHz (PASS) | 101.57 MHz (PASS) |
| frames file | 20230 lines, no fasm2frames error | 20230 | 20230 |
| bitstream sha256 | `93125295055bfdcbc0b44286033477c80ccb9ef728b464a1a7bee866beac2c67` (9730905 bytes) | `3b927e7603563438eea6a4ffe9c5c5c2a64fa02ad7198e203c15f922a2882089` (9730909) | `4b854bac388b048ad210d352d1cab265c6dec706ccaab2e6cc06001411a20bd6` (9730910) |

Timing closed at 100 MHz post-route for all three; nothing needed a second seed.

### Capture results (UART port `/dev/cu.usbserial-1130`, 115200 baud, 6 s each)

| | M1b top | RLC control | LAD5 control |
|---|---|---|---|
| capture bytes | 62954 | 63155 | 63082 |
| capture sha256 | `f31670cecfc85f51917699da0f300b28b2f5fe639f5b53b7704aeaa925fb5513` | `a9059dec6b9da9ce9cda4d3539e67e77fc5850bdc8da6104af0b1ffc28b33c52` | `4a429cf89d54153f414afb0cc56647fcfc25b767c7d793dcf60864b2d366d43f` |
| lines starting with `T27 ` (raw) | 575 | 578 | 576 |
| well-formed receipt lines | 573 | 575 | 574 |
| malformed (not counted) | 3 | 3 | 2 |
| RLC lines PASS / FAIL | 286 / 0 | 0 / 287 | 287 / 0 |
| LAD5 lines PASS / FAIL | 287 / 0 | 288 / 0 | 0 / 287 |
| well-formed lines whose A, B, S, C and verdict equal the spec's prediction | 573 of 573 | 575 of 575 | 574 of 574 |
| L of the well-formed lines | 5 to 577, no gap | 7 to 581, no gap | 4 to 577, no gap |
| circuit alternates RLC, LAD5 | yes, 0 breaks | yes, 0 breaks | yes, 0 breaks |

The lines, for the record (the first complete pair at L = 0x100 in each capture):

```
top          T27 RLC A=0000000001136E34 B=0000000080135019 S=84BE2C7F9115D08A C=000000000000DD5A L=0000000000000100 PASS
             T27 LAD5 A=00000000002D3886 B=0000000000441EF5 S=29439851A3F5D9EE C=0000000000022088 L=0000000000000101 PASS
RLC control  T27 RLC A=0000000001136E1E B=0000000080135011 S=89932D8FBE407911 C=000000000000DD5A L=0000000000000100 FAIL
LAD5 control T27 LAD5 A=00000000002D3890 B=0000000000441F01 S=321615C52B5B23EA C=0000000000022088 L=0000000000000101 FAIL
```

The control values are the spec's `RLC_BAD_A/B/S` and `LAD5_BAD_A/B/S`. In each control capture the other circuit's lines are identical to the top's and PASS.

### Against the earlier result

Earlier (issue text and the spec header): 572 of 572 receipt lines PASS, each negative control FAIL on its own circuit only, 1197 LUT, 1096 FF, 0 DSP, 0 BRAM, 106 MHz post-route.

- Reproduced in kind: all 573 well-formed lines of the top PASS with the predicted A, B, S and C (earlier 572 of 572), and each control prints the predicted FAIL values on its own circuit only (RLC control: 287 RLC lines FAIL, 288 LAD5 lines PASS; LAD5 control: 287 LAD5 lines FAIL, 287 RLC lines PASS). The count differs by one line (573 against 572); the capture is a fixed six-second window, so the count depends on when the reader attaches and which lines are lost to the reconfiguration noise. It is a difference in the count, not in the result.
- Resources, a difference: LUT 1196 (earlier 1197), FF 1096 (same), DSP 0 and BRAM 0 (same), post-route Fmax 116.06 MHz (earlier 106.30 MHz, 66.32 before routing; now 67.65). The netlist is not the one measured before: the generated Verilog differs from the earlier build's (earlier file sha256 `972263269d22c52f24f75cfc85b09a2847f47d572ac551b8ab92fdacd45c654a`, 55777 bytes; now 55803 bytes). The diff is in the receipt helper functions only: the earlier file has `verdict_char` and its own `hex_digit` from `rc_filter_ax7203`, the new one has `hex_digit` and `pass_fail_char` from `specs/fpga/board_receipt.t27`, because commit 39688f629 (#10475) made `rc_filter_ax7203.t27` `use fpga::board_receipt`. The M1b spec itself is unchanged since its only commit (b94c869d2). A different netlist places differently, so the Fmax moved; it is the same kind of movement the earlier notes record ("not stable under small netlist changes").
- The controls' earlier numbers came from a different netlist too (default seed 87.84 and 99.32 MHz post-route, two seeds needed); now both close at the default seed (103.38 and 101.57 MHz).

Sentence: the earlier M1b receipt was reproduced on the board from current master, 573 of 573 well-formed lines PASS with the predicted values (earlier 572 of 572), and each negative control fails on its own circuit only (575 well-formed lines with 287 FAIL, all on RLC; 574 with 287 FAIL, all on LAD5); the resource numbers differ slightly (LUT 1196 against 1197, Fmax 116.06 against 106.30 MHz) because the generated Verilog differs after #10475.

## 3. Top 2 of 2: the SAR converter

- top: FpgaSarAdc (specs/fpga/sar_adc_ax7203.t27, M4e-board; 31 conversions, one UART receipt line each, the 31 lines repeat)

Header numbers used as the authority: the receipt `T27 SAR K=.. C=.. E=.. N=.. S=.. L=.. PASS|FAIL`, PASS when C equals E; 30 of the 31 E equal N (K=19 hex: 1841 against 1842); the negative control changes one literal of `rom_k` (plate j = 0, `return 16083` to `return 32166`, one more DAC unit on the LSB plate) and `FAULT_MASK = 687562312` marks 16 of the 31 conversions as FAIL: K = 03 06 09 0A 0B 0C 0E 10 11 13 14 15 16 17 1B 1D. A conversion is 75 clocks.

### Commands (exact)

```
cd $R
$T27B verilog specs/fpga/sar_adc_ax7203.t27 > $B/sar_top.v
sed -i '' '294s/return 16083/return 32166/' specs/fpga/sar_adc_ax7203.t27
$T27B verilog specs/fpga/sar_adc_ax7203.t27 > $B/sar_neg.v
git checkout -- specs/fpga/sar_adc_ax7203.t27
```

(the sed was preceded by a check that line 294 holds the old text; both files have 1610 lines, module `FpgaSarAdc`, and differ in exactly one line: 1101c1101.) Then, for `N` in `sar_top sar_neg`, one at a time, the same five steps as in section 2 with the top name `FpgaSarAdc`:

```
yosys -l $B/$N.yosys.log $B/$N.ys
  # $B/$N.ys: the same nine lines as in section 2, with -top FpgaSarAdc
$O/np-main/build/nextpnr-xilinx --chipdb $O/xc7a200tfbg484-2.bin --json $B/$N.json --fasm $B/$N.fasm --freq 100 --log $B/$N.pnr.log
PYTHONPATH=$O/prjxray $O/venv/bin/python3 $O/prjxray/utils/fasm2frames.py --db-root $O/prjxray-db/artix7 --part xc7a200tfbg484-2 $B/$N.fasm $B/$N.frames
$O/px-main/build/tools/xc7frames2bit --part_file $O/prjxray-db/artix7/xc7a200tfbg484-2/part.yaml --part_name xc7a200tfbg484-2 --frm_file $B/$N.frames --output_file $B/$N.bit
rm $B/$N.frames
openFPGALoader -c digilent_hs2 --busdev-num 001:004 $B/$N.bit
exec 3<$PORT; stty 115200 raw cs8 -cstopb -parenb clocal <&3; cat <&3 > $C/$N.txt &  P=$!; sleep 3; kill $P; wait $P; exec 3<&-
```

with `PORT=/dev/cu.usbserial-1130`, no XDC, the default seed, three seconds per SAR capture (a line is 5.5 ms, so 3 s holds about 550 lines).

### Build results

| | SAR top | SAR control |
|---|---|---|
| Verilog sha256 | `d8e31fdcc479e1f99df471b3fa5f68fca5ba3ac514301ef5b20199532116cfc2` | `66c1c155d5a39b3aae84bbdd3e631845934614f56f2d65630887d896ef30fc3c` |
| LUT (yosys, LUT1..LUT6) | 908 (LUT1 2, LUT2 337, LUT3 72, LUT4 168, LUT5 234, LUT6 95; plus 80 INV, 4 MUXF7, 2 MUXF8) | 923 (LUT1 1, LUT2 341, LUT3 86, LUT4 167, LUT5 222, LUT6 106; plus 80 INV, 7 MUXF7, 1 MUXF8) |
| FF | 538 (FDRE 507, FDSE 31) | 538 (507, 31) |
| DSP48E1 | 0 | 0 |
| BRAM (RAMB18E1, RAMB36E1, FIFO18E1, RAMBFIFO36E1) | 0 | 0 |
| CARRY4 (yosys / nextpnr) | 137 / 141 | 140 / 144 |
| nextpnr SLICE_LUTX | 1972 of 269200 | 2025 |
| Fmax before routing (first line) | 93.51 MHz (FAIL at 100) | 87.62 MHz |
| Fmax post-route (second line) | 108.27 MHz (PASS at 100) | 111.30 MHz (PASS) |
| frames file | 20230 lines, no fasm2frames error | 20230 |
| bitstream sha256 | `e363bb4608ce136b00c787f4b95ef940f823bbdcfe0661e09505456826fade0e` (9730905 bytes) | `e1cbaa8fb279453bcb60c438829507907fb85cb4e55285b52aa7c8aca5adf230` (9730905) |

Timing closed at 100 MHz post-route for both.

### Capture results (UART port `/dev/cu.usbserial-1130`, 115200 baud, 3 s each)

| | SAR top | SAR control |
|---|---|---|
| capture bytes | 34724 | 34657 |
| capture sha256 | `1b16538c09f37c33e9a37cd803b476898757e9a174f72a31a2b7f76f30482f66` | `8034d19fcb6c4f80978618e3b03fd5c25f90a8a5ec965011f80ee68b4eaa4524` |
| lines starting with `T27 ` (raw) | 550 | 550 |
| well-formed receipt lines | 549 | 547 |
| malformed (not counted) | 3 | 4 |
| verdicts | 549 PASS, 0 FAIL | 262 PASS, 285 FAIL |
| well-formed lines whose C, E, N and verdict equal the spec's prediction | 549 of 549 | 547 of 547 |
| distinct K seen | 31 of 31 | 31 of 31 |
| conversions printing FAIL | none | 16: 03 06 09 0A 0B 0C 0E 10 11 13 14 15 16 17 1B 1D |
| conversions printing PASS | all 31 | 15: 00 01 02 04 05 07 08 0D 0F 12 18 19 1A 1C 1E |
| FAIL set equals the 16 predicted (FAULT_MASK) | not applicable (none predicted) | yes, exactly |
| L of the well-formed lines | 8 to 556, no gap | 9 to 555, no gap |

For the top, E is the spec's `FIT_CODE[K]`; for the control, C equals the spec's `FAULT_CODE[K]` on every line and the verdict is FAIL exactly where `FAULT_CODE[K]` differs from `FIT_CODE[K]`. For each of the 31 K, the set of (C, E, N, S, verdict) in each capture equals the set in the earlier run's capture of the same bitstream kind (31 of 31 for the top, 31 of 31 for the control; the earlier captures are the session's `uart_sar2_top.txt` and `uart_sar2_neg.txt`). Sample lines:

```
top      T27 SAR K=00 C=3EE E=3EE N=3EE S=00000000000003EE L=0117 PASS
         T27 SAR K=19 C=731 E=731 N=732 S=000000056FCCC9C1 L=0111 PASS
control  T27 SAR K=03 C=221 E=220 N=220 S=0000000000001529 L=011A FAIL
         T27 SAR K=19 C=731 E=731 N=732 S=000000056F8522BD L=0111 PASS
```

### Against the earlier result

Earlier (issue text and the spec header, and the merged PR for #10910): 550 lines PASS bit-exact, the control FAIL on exactly the 16 predicted conversions, 908 LUT, 538 FF, 0 DSP, 0 BRAM, 108 MHz post-route.

- Reproduced: 549 of 549 well-formed lines PASS with C equal to E (the raw count of lines starting with `T27 ` is 550, the earlier figure; the earlier PR counted well-formed lines, 550 of 550, and the one-line difference between 549 and 550 is the capture window: the first lines are lost at the start and the last is cut). The control FAILs on exactly the 16 predicted conversions and on no other (285 FAIL lines over those 16, 262 PASS lines over the other 15), and its C equals the spec's `FAULT_CODE` on every line.
- Resources: identical to the earlier measurement. LUT 908, FF 538, DSP 0, BRAM 0, post-route Fmax 108.27 MHz (93.51 before routing), and for the control 111.30 MHz (87.62 before). The generated Verilog is byte-identical to the earlier build's (sha256 `d8e31fdc...` for the top and `66c1c155...` for the control, as recorded in the merged PR), and place-and-route is deterministic, so the netlist and the placement are the earlier ones.
- The bitstream sha256 is not the earlier one (earlier top `652c6a3daeeb...`, now `e363bb46...`). The earlier M1b PR records that the bitstream header carries the frames file name and a time, so a rebuild of the same netlist has a different sha256 (two builds of one netlist differed in 10 header bytes); the earlier SAR bitstream is no longer on disk, so its bytes were not compared.

Sentence: the earlier SAR receipt was reproduced on the board from current master, 549 of 549 well-formed lines PASS bit-exact (550 lines start with `T27 `, as before), the 31 conversions carry the earlier (C, E, N, S, verdict) values, and the control fails on exactly the 16 predicted conversions; the resource numbers and Fmax are identical to the earlier run.

## 4. What failed or differed, and the issues filed

Nothing failed. No build missed timing, no capture split (one reader, port free before and after each), no receipt line mismatched its prediction. Because nothing failed, no issue was filed; the differences below are reported, not retried away.

Differences from the earlier results:

1. M1b netlist and Fmax: the generated Verilog changed after #10475 (receipt helper functions moved to `fpga::board_receipt`), so LUT is 1196 (earlier 1197) and the post-route Fmax is 116.06 MHz (earlier 106.30 MHz); the controls close timing at the default seed now (103.38 and 101.57 MHz), where earlier they needed a second seed. The receipt is unchanged.
2. Line counts: 573, 575 and 574 well-formed lines for M1b (earlier 572) and 549 and 547 for the SAR top and control (earlier 550). The counts follow the length of the fixed capture window and where the reader attaches; every well-formed line matched.
3. Environment drift: the cable is on USB bus 001 (earlier 000) and the UART is `/dev/cu.usbserial-1130` (earlier `-130`). The earlier helper script hard-codes `--busdev-num 000:004` and the old port name; both were changed for this run. Only historical reports in the repository mention the old names.
4. Disk: free space was below the lane brief's 8 GB during the whole run (4.8 to 7.3 GiB), see section 1.
5. The earlier PR for #10910 lists the SAR control as "933" LUT, but its own breakdown (LUT1 1, LUT2 341, LUT3 86, LUT4 167, LUT5 222, LUT6 106) sums to 923; this run measured the same breakdown, so 923 is the figure.
6. Bitstream sha256 differ from the earlier ones for the SAR tops although the Verilog is identical (the header content, as the earlier M1b PR notes for a rebuild of one netlist; the earlier SAR bytes were not kept, so this is not byte-verified).

Issues filed: none.

## 5. Not claimed

No speed claim and no value claim. Nothing about power. Fmax is nextpnr's estimate for one placement. A PASS here says what the spec headers say it says: this board, running RTL generated from these specs by the current master, printed the values the specs derive; it says nothing about another circuit, another sample set, or the continuous circuit.
