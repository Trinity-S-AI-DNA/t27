# T27 OS on the FPGA: feasibility and plan

Refs gHashTag/t27#10782 (T27 OS epic), PR #10783 (the T27 OS specs). Owner, 2026-10-19: "ideally T27 OS should
run on the FPGA".

Status words used below: **measured** = run in this work on the Railway lab; **read** = judged from the spec's
text, not lowered or run; **not established** = nothing in the repository says.

## 1. The answer in one paragraph

The decisions of T27 OS can run on the bench AX7203 today; its window cannot, and should not. Four panes whose
herdr status is advanced by the real `pane_step` from `specs/t27os/pane_state.t27` lower to about 470 LCs and 270
flip-flops (**measured**, yosys `synth_xilinx`), and their screen leaves the board as VT100 on the UART, where the
T27 OS terminal pane (or any terminal) shows it. That is `specs/fpga/t27os_uart_shell_ax7203.t27`. The window stays
on the Mac, as a client of the board, exactly as it is a client of the PTY server today.

## 2. What T27 OS is made of, and where each part can go

T27 OS specs on `feat/t27os-petal-menu` (PR #10783). Counts are from the spec text: `@extern` imports, slice
types, `while` loops, `f64`.

| spec | what it decides | host-bound? | on the board |
|---|---|---|---|
| `agents.t27` | agent table, session states S_*, restore actions | no (scalars, two `str` consts) | **measured**: its S_* constants are used by the shell spec; the fns are scalar |
| `pane_state.t27` | pane status word and `pane_step`, `shown_state`, `pane_needs_you` | no | **measured**: `pane_step`, `shown_state`, `pane_needs_you`, `ps_state` lower inside the shell spec (`hook_event` takes a slice, so it is left out) |
| `vt.t27` | VT/ANSI grid: bytes in, cells out | no | **measured as a test-side reader only**: it uses slices and loops, which `t27b verilog` does not take in the clocked path. The board only writes bytes; it never parses them. A grid on silicon is stage 3 |
| `windows.t27` | the window model: spaces, sessions, the sidebar | no (no `@extern`, no slices; 17 loops, 19 `f64`) | **read**: pure logic, but loop-heavy; it needs a ROM/FSM rewrite of its loops and a fixed-point rewrite of its `f64` before it can lower |
| `queen.t27` | Queen panel, suggestions, GOD MODE | no (6 loops, 60 `f64`) | **read**: same as `windows.t27`; `f64` is the cost |
| `layout.t27`, `term_pane.t27` | placement of cells and panels in pixels; key bytes to the PTY | no, but pixel geometry only matters to a display | **read**: `layout` is `f64` geometry, useless without a display; `term_pane`'s key-to-bytes table is a ROM and could lower |
| `doctor.t27` | repositories on the machine, disk cost | yes in spirit (it reads the file system through the host) | stays on the host |
| `pty_proto.t27`, `pty_logic.t27`, `session_store.t27` | wire format, slot allocation, the manifest | `pty_proto` is pure but slice-heavy; `pty_logic` has 2 `@extern`; `session_store` is a file format | `pty_proto`'s frame codec is the natural wire format for stage 2 (Ethernet or UART framing); the rest stays on the host |
| `pty_server.t27`, `pty_client.t27`, `pty_sys.t27` | the PTY server and its client over libSystem | **yes**: `forkpty`, `poll`, Unix sockets (32 `@extern` in `pty_sys`) | replaced on the board by the UART/Ethernet link (see section 3) |
| `glass_window.t27`, `hot_reload.t27` | the AppKit window, hot reload | **yes**: AppKit, CoreGraphics, CoreText, `ld` | stays on the Mac; it becomes the display client of the board |

Two routes lower pure logic, and both exist in the repository:

1. **t27 to Verilog to bitstream.** `t27b verilog` (the pure-t27 board flow, as `specs/fpga/rc_filter_ax7203.t27`).
   Limits that decide what lowers: no signed types, no slices and no array indexing in the clocked path, tables
   become ROMs, no loops in the tick body. `pane_state` and `agents` meet these; `vt`, `windows`, `queen` do not
   yet.
2. **t27 to WASM on the existing wasm board cell** (`specs/fpga/wasm_board_ax7203.t27`). Its wire format and
   `on_clock(rxd)` step loop would run a compiled `windows`/`queen` unchanged in source; its top is outside this
   repository, so this route was **not** exercised here. It is the way to run the loop-heavy specs without
   rewriting them, at the price of a WASM core on the die.

## 3. What replaces the host-bound parts

| host-bound part | on the board |
|---|---|
| AppKit window, CoreText drawing | the T27 OS window as a client: it already hosts a terminal pane, so it shows the board's VT100 stream. HDMI is **not established**: no AX7203 HDMI pin table or schematic is in the repository (`fpga/HARDWARE_SSOT.md` lists UART, LEDs, the 200 MHz clock, the two RGMII PHYs and DDR3 only) |
| `forkpty` and the agent processes | nothing: an agent CLI cannot run on an FPGA. The board holds the agents' **status**, not the agents. A hook from the host (Claude Code's `UserPromptSubmit`, `Notification`, `Stop`) becomes one event byte on the link |
| Unix socket to the PTY server | UART (stage 1), then Ethernet (stage 2) |
| Keychain | not replaced; secrets stay on the host. The board never holds a credential |
| the manifest on disk | the board has no disk; the 16 MiB N25Q128 flash is on the AX7203 (`fpga/HARDWARE_SSOT.md`), but a manifest in flash is **not established** as a plan |

## 4. I/O on the AX7203

From `fpga/HARDWARE_SSOT.md` (the bench board is an ALINX AX7203, `xc7a200tfbg484-2`):

- **UART**: TX N15, RX P20, LVCMOS33. 115200 8N1 from a 100 MHz PLL clock (divider 868) is what
  `rc_filter_ax7203.t27` already measured on this board.
- **Ethernet**: two KSZ9031 RGMII PHYs (the cabled PHY 1 runs at 100 Mb/s). The repository has `eth_beacon`,
  `eth_rx`, `eth_reply`, `eth_crc`, and a UDP node on this board was measured on 2026-10-10 (HARDWARE_SSOT.md).
  That is the stage 2 transport.
- **LEDs**: B13 C13 D14 D15, active low. The shell uses them as blocked / working / done / heartbeat.
- **Button**: T6 (active low), LVCMOS15; resets the shell.
- **HDMI output**: an SiI9134 transmitter fed by a parallel RGB bus, 29 pins checked against the official AX7203 manual pp. 31-32 (section 5b); I2C balls still second-hand.
- **DDR3**: 1 GB present; two DDR3 cells exist (`ddr3_head_bandwidth`, `ddr3_ternary_matvec`). Not needed before a
  scrollback ring per pane (stage 3).

## 5. Staged plan

| stage | what runs on the board | what it needs | state |
|---|---|---|---|
| 0 | `pane_step`/`shown_state` on silicon, screen as VT100 over UART; keys and a script drive the events | this PR | spec, tests and RTL lowering **measured**; bitstream and bench run **blocked** (section 6) |
| 1 | the host feeds real events: a small host-side client sends one byte per Claude Code hook to the board's RX, so the board's screen shows the user's real panes | a host client in `.t27` (the window's `pty_client.t27` already speaks to a server; a UART transport is a new `@extern` set in `pty_sys.t27`) | next |
| 2 | the same over Ethernet with `pty_proto.t27`'s frame codec | lower `pty_proto` encode/decode (slice-free subset), reuse the UDP node | later |
| 3 | `windows.t27` (spaces, sidebar) and a grid (`vt`) on the board | rewrite loops as FSMs or run them on the wasm cell; `f64` to fixed point; DDR3 for scrollback | later |
| 4 | display | parallel-RGB scanout (`vga_text_core.t27`, done) plus an SiI9134 I2C initialiser and a pin table checked against the manual | core **measured**; board top blocked (section 5b) |

## 5b. Stage 4: a text screen on the owner's VGA monitor (2026-10-19)

The owner's monitor is VGA, attached to the board's HDMI output through an active HDMI-to-VGA adapter.

`specs/fpga/vga_text_core.t27` is the part that does not depend on how the pixels leave the chip: 640x480@60
timing (800x525 total, syncs low), an 8x8 glyph ROM and a text scanout, one `on_clock` per pixel. **Measured** on
the lab: 7 tests pass (96 low hsync ticks a line, 800 ticks a line, 2 low vsync lines, 525 lines, 307200 visible
pixels, the first glyph row of "T" on the stream, the stream equal to `pixel_at` for 8 lines, blanking black);
`t27b verilog` lowers it (636 lines); yosys `synth_xilinx` passes (392 cells).

### The AX7203 HDMI output is not a TMDS output from the FPGA

Two community projects that read the ALINX manual (pages 31-32 for the video part) agree, pin for pin, that the
board's HDMI output is an **SiI9134 HDMI transmitter fed by a parallel bus**: 24 data bits, a pixel clock, DE,
HSYNC and VSYNC, plus a reset and an I2C configuration bus. The FPGA does not drive TMDS pairs, so a TMDS
encoder and 10:1 serialiser in `.t27` would have no pin to drive on this board. The route is a **parallel RGB
scanout** (what the core already is) plus an **I2C initialiser** for the SiI9134. The adapter then sees an
ordinary HDMI/DVI signal from the transmitter chip.

Sources: the pin table below is **checked against the official manual** (see the record under the table); the
first two were the second-hand sources it was found in:

- `LilithSemi/harbor`, `packages/harbor/lib/src/soc/board.dart` (AX7203 pin table, "p.31-32").
- `adilsondias-engineer/20-fpga-order-book`, `constraints/ax7203_rgmii.xdc` (lines 324-345 and after).
- The ALINX manual itself (see the record under the table).

Both lists agree, and so does the manual (Table 3.4.1, pp. 31-32), on every HDMI-output ball below (all LVCMOS33). Only the two I2C rows are not in the manual text:

| signal | ball | signal | ball |
|---|---|---|---|
| hdmi_clk (to IDCK) | M13 | hdmi_d[12] | L14 |
| hdmi_de | V13 | hdmi_d[13] | L15 |
| hdmi_hsync | T15 | hdmi_d[14] | M15 |
| hdmi_vsync | T14 | hdmi_d[15] | M16 |
| hdmi_reset_n | J19 | hdmi_d[16] | L18 |
| hdmi_d[0] | V14 | hdmi_d[17] | M18 |
| hdmi_d[1] | H14 | hdmi_d[18] | N18 |
| hdmi_d[2] | J14 | hdmi_d[19] | N19 |
| hdmi_d[3] | K13 | hdmi_d[20] | M20 |
| hdmi_d[4] | K14 | hdmi_d[21] | N20 |
| hdmi_d[5] | L13 | hdmi_d[22] | L21 |
| hdmi_d[6] | L19 | hdmi_d[23] | M21 |
| hdmi_d[7] | L20 | i2c scl | E16 |
| hdmi_d[8] | K17 | i2c sda | F16 |
| hdmi_d[9] | J17 | | |
| hdmi_d[10] | L16 | | |
| hdmi_d[11] | K16 | | |

**Record of the check (2026-10-19).** File `AX7203 V1.0 User Manual.pdf`, 2,814,987 bytes, sha256
`361a512dc683be0f7f7d415362259ea8ed34aedff55f6469e20542fe6f4ddce8`, 43 pages, "Copyright Alinx 2026", body text
names the AX7203 (the PDF's Subject metadata says AX7101; it is a copy-over, the content is the AX7203). Source:
ALINX's own Download page (`https://www.en.alinx.com/` > Download), which links the public ALINX Dropbox folder
`_Standard Products / Artix-7 / AX7203 / AX7203_2023.1`. The PDF is not committed (licence); this table is.
Section 3.4 "HDMI Output Interface" names the chip as the SIL9134 ("supports up to 1080P@60Hz output and 3D
output"), says its IIC configuration interface is wired to FPGA I/O and that it is "initialized and controlled
through FPGA programming", and lists the 29 balls in Table 3.4.1 on pp. 31-32. Every one equals the table above.
Section 3.5 is the HDMI **input** (SIL9013), a different connector.

The data bits are not named by colour in the manual: the chip's input setup (over I2C) decides which group is
red, green or blue. The I2C balls come from LiteX, not from the manual ("the manual draws the nets but gives no
balls"), and the manual gives no I2C address. The initialiser in `harbor` (`sii9134_initializer.dart`) writes
two registers, `0x72 <- reg 0x08 = 0x35` and `0x7A <- reg 0x2F = 0x00`, after a reset low of at least 50 us; that
sequence is **third-party and not run on this bench**.

### What is still missing before a board top

1. ~~The pin table checked against the official manual~~ **Done 2026-10-19** for the 29 HDMI-output pins. Still
   open: the two I2C balls (E16, F16), which the manual draws only in Figure 3.4.1 and does not list.
2. The SiI9134 input setup for 24-bit RGB with separate syncs, and the I2C address and registers, from the
   manual or the chip's reference. The two-register sequence above is not a verified picture-on sequence.
3. A way to drive an open-drain I2C pad from `.t27`: no tristate primitive was found in the repository, and
   `t27b verilog` was not shown to emit one. This needs a look in `specs/fpga/` for an `IOBUF`-style cell.
4. A 25 MHz pixel clock: from the 200 MHz input, the PLL used by the other cells gives 25 MHz with a VCO at
   800 MHz and an output divide of 32 (to be checked in the PLL spec, not assumed).

The monitor needs plugging in only when the board top for this route is merged and its bitstream is
programmed. The cable to connect is **the board's HDMI output (the connector marked HDMI output, not HDMI input)
to the HDMI-to-VGA adapter, and the adapter to the monitor's VGA cable**.

## 6. Why there is no bench transcript in this PR

What was done: the spec's 8 tests pass at run time (`t27b test` under qemu-aarch64 on the lab, since the t27b JIT
runs on arm64 only), the RTL lowers (`t27b verilog`, 1074 lines), yosys synthesises it for xc7 (1274 cells, about
468 LCs, 0 problems from its check pass), and the pin stream is checked byte for byte against the screen function.

What is missing, exactly:

1. **Place and route.** The lab has yosys and iverilog but no `nextpnr-xilinx`, no `xc7a200tfbg484-2.bin` chipdb and
   no prjxray database (`t27c preflight` there: "T27_OPENXC7 is not set"). The openXC7 toolchain and that chipdb
   exist on the owner's Mac (`~/openxc7-src`), where the standing rule forbids heavy runs. The fix is to install
   the toolchain on the lab, or to approve one place-and-route on the Mac. Until then no bitstream exists.
2. **The board.** `openFPGALoader --scan-usb` on the Mac found no USB device when it was run, so the cable was not
   visible.
3. **RTL-level simulation of the pins.** The spec's own test already decodes the pin the machine drives and
   compares it to the screen function; a simulation of the lowered Verilog through the PLL primitive was not done
   (the lab has no unisim model of `PLLE2_ADV`, and a hand-written Verilog shim is barred by the only-t27 rule).

No UART transcript is shown because none was captured. The expected screen is in the spec header and is asserted,
cell by cell, by `the_screen_is_the_t27os_vt_grid_as_the_layout_says`.

## 7. Dependency

The spec uses `specs/t27os/{agents,pane_state,vt}.t27` by `use`; those files are on `feat/t27os-petal-menu`
(PR #10783) and not yet on `master`. Until that PR lands, `t27b` cannot resolve the imports on `master` alone.
