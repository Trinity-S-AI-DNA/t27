# Board run of the PSP103 inverter, fs corner, tables in block RAM, 2026-10-24

Epic #10486 (analog emulation v0.1), success criterion SC-003. English, ASCII only.

The tt note (`board-psp103-inv-2026-10-20.md`) is the method and the grammar; this note only adds what is the fs corner's own. The same `specs/fpga/psp103_inv_ax7203.t27` runs the 500 fF CMOS inverter (400 backward Euler steps of dt = 100 ps, three Newton iterations a step) on the ALINX AX7203, with the fs current and drain-charge tables in block RAM, and prints one UART line per step. The corner is one literal in the data spec: `BOARD_CORNER` in `specs/analog/mos_psp103_data.t27` (TT = 0, FF = 1, SS = 2, FS = 3, SF = 4); the generated `data.v` of two corners differs in exactly the one line `parameter [31:0] BOARD_CORNER = N;`. There is one spec and one table choice, not a fork. Every number below was measured in this run.

What this document does not say: nothing about speed, power or value. Fmax is nextpnr's post-route estimate for one placement of one netlist. A line of the board agrees with the spec's own fixed-point run; the spec's distance to ngspice at fs is what `mos_psp103.t27` and the tables PRs (#11123, #11214) measure, not this note.

## 1. Receipt status

| | what the board was compared with | result |
|---|---|---|
| Build A: fs tables, tt samples in the RAM | the fs run of the spec, offline | **1225 of 1225** well-formed lines have Y equal to the spec's fs sample; S at N = 18F is `0xA45CC48E565AD2EB` = `EXP_S_FS_Q` = `PIN_FS_RUN` |
| Build B: fs tables, fs samples in the RAM | the board itself (the form of the tt receipt) | see section 6 |

Build A is the form of the ss control in the tt note. It is a receipt of the fs tables on the board: the Y column of every line is the spec's fs sample, and the fold S pins the whole round. Build B is the form of the tt receipt (the board's own verdict is PASS on every line).

## 2. The run

- Source: the data spec as it stood before the four runs were typed into it, with `BOARD_CORNER = 3` and the expected word read from the tt run (`ex <= TT_RUN[idx]` in the generated `data.v`). The committed spec differs from that in the runs and the tests it adds, in the committed value of `BOARD_CORNER` (0), and in `ex = rom_ex(BOARD_CORNER, idx)`, which is the form of Build B. `top.v` is `t27b verilog` of the board spec, `data.v` is `t27c gen-verilog` of the data spec.
- Toolchain: yosys 0.67+post (git b8e7da6f40ae8f552c116bf6c359b07c6533e159), nextpnr-xilinx git b608fd2c950f3b4e2814540efbbc5841dd35263e, prjxray c9f02d8576042325425824647ab5555b1bc77833 with prjxray-db-full 517d66a383676cb971177ea92b0ff3b6ea6e8690, chipdb `xc7a200tfbg484-2.bin` md5 `59f828c441e6e6081bfee63ff994660e`, `openFPGALoader v1.1.1`. The yosys script, the nextpnr, fasm2frames and xc7frames2bit commands are the ones of section 2 of the tt note, run in the T27 OS pane.
- Board and cable: ALINX AX7203, `xc7a200tfbg484-2`; Digilent HS2 found by `openFPGALoader -c digilent_hs2 --detect` (idcode 0x3636093) at `--busdev-num 0:4` (the bus renumbers, so it is detected first). UART: CP2102N, 115200 baud 8N1, pin N15, one reader started before the load, 7 s capture.
- Programming: SRAM only (no `-f`, no `-m`). No flash and no eFUSE was written.

## 3. The board table, Build A

| | fs, Build A |
|---|---|
| `top.v` sha256 | `7a3ad026f1f6a2461647d15e48c511ef4637251159be0384b4850f3c5ec8acfd` (the same file as the tt top) |
| `data.v` sha256 | `6e9df493a7f6eda89e09bd550ae45de0eb7de31614ba61ed2b8a2c6300aadbeb` |
| LUT (yosys, LUT2..LUT6) | 1403 (LUT2 461, LUT3 275, LUT4 158, LUT5 290, LUT6 219; plus 173 INV) |
| FF | 1219 (FDRE 1196, FDSE 23) |
| DSP48E1 | 0 |
| BRAM | 4 RAMB36E1 + 1 RAMB18E1 |
| CARRY4 (yosys) | 208 |
| nextpnr SLICE_LUTX / SLICE_FFX | 2758 / 1219 of 269200 |
| seed | 2 (the default seed closed at 88.83 MHz after routing, FAIL at 100; its first line was 76.42 MHz) |
| Fmax before routing | 77.37 MHz (FAIL at 100) |
| Fmax after routing | **105.98 MHz** (PASS at 100) |
| frames file | 20230 lines, no fasm2frames error |
| bitstream sha256 (the header carries a time) | `56cddb7753d872c21ea762c8b15b150a8bbe2e2ca3750430f98197fc94740bb0` |
| well-formed lines, L | 1225 (L = 0 to 1224, no gap) |
| lines whose Y equals the spec's fs sample | **1225 of 1225** |
| rounds | 3 complete rounds of 400, byte-equal (Y, E, S) to each other, then 25 lines of a fourth equal to its head |
| PASS / FAIL (the samples in the RAM are tt's) | 246 / 979 |
| S at N = 18F | `0xA45CC48E565AD2EB` = `EXP_S_FS_Q` |

The LUT count is of the same order as the tt top (1426); the tables are in block RAM, not in LUTs. Only the seed-2 bitstream was loaded; the default-seed placement was not loaded because it failed the 100 MHz estimate.

## 4. The capture

One reader, 7 s, started before the load: 83393 bytes, sha256 `d51c8af4447ea6c812a1c29182d7e3b0e71dd6053295f3ad834765d2d110b506`, 1249 lines that contain `T27 `. A line is well-formed when it matches `T27 PSP N=<3 hex> Y=<7 hex> E=<7 hex> S=<16 hex> L=<4 hex> (PASS|FAIL)`; only those are counted. Before the new configuration took over, the design loaded before it was still printing and the UART showed noise bytes (24 lines that contain `T27 ` precede the first line of this design, none of them well-formed lines of it); the judge starts at the first `N=000 .. L=0000` line, byte offset 2505 of the capture. The last line is cut where the reader is killed.

```
T27 PSP N=000 Y=00A080B E=00A8451 S=00000000000A080B L=0000 FAIL
T27 PSP N=001 Y=018C79F E=019C356 S=00000000000CD789 L=0001 FAIL
```

Y is the fs sample and E the tt one, so the FAIL lines here are the prediction, not a defect: the fs run differs from the tt run at 318 of the 400 steps, and the board shows 82 PASS and 318 FAIL in each of the three rounds.

## 5. Tests

`t27c test-report specs/fpga/psp103_inv_ax7203.t27`: 16 tests, 16 pass, 0 fail, 0 vacuous; the new `the_spec_runs_of_the_other_four_corners_are_the_words_the_board_printed` runs the machine on each corner's tables and checks every one of the 400 samples and the fold against `EXP_S_<c>_Q` and `PIN_<c>_RUN`. `t27c test-report specs/analog/mos_psp103_data.t27`: 13 tests, 13 pass, 0 fail, 0 vacuous; the new `the_rom_choice_reads_each_corners_tables_and_run` reads all five corners through the ROM port the board uses.

Both counts are from the T27 OS lab (`Trinity-S-AI-DNA/t27os`, workflow `lab.yml`, run on commit `6d705836fedebbf5bdfe1c8ec3418814d5f78954`, receipt `runs/6d705836fedebbf5bdfe1c8ec3418814d5f78954.5ff8b0d1.summary.txt`), on the t27c leg and on the t27b leg; they were not run on the Mac.

## 6. Build B

Run on 2026-10-24 in PR #12063 (task #12050); the receipt is `board-psp103-inv-buildb-2026-10-24.md`: 1219 of 1219 well-formed lines of the fs design read PASS, and the fold S at N = 18F equals the pin in three rounds. The first version of this section (PR #12031) said Build B had not been run, because the pane server of the T27 OS app had stopped answering at 21:50 on 2026-10-24; that was true when it was written. The Build A numbers of this note are unchanged.

## 7. Not claimed

No speed claim (a step is about 1700 to 1960 clocks; nothing here is compared with ngspice's wall time). No power. This is the fs corner at one gate waveform (two levels, an ideal step) and one load (500 fF with dt = 100 ps). A line of the board is agreement with the spec's fixed-point run of the same tables, not with ngspice. The fs top closed 100 MHz only with seed 2 in this run (default seed: 88.83 MHz), so its Fmax is one placement's estimate and not a property of the netlist.
