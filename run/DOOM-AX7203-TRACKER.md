; DOOM on AX7203 -- tracker of the open facts, written 2026-11-25
; Each line is one open fact with the evidence that pins it down. Nothing here is a claim
; without a command or a file behind it. The rules: own language first (this file is a .t27
; module, readable as text because every line is a comment), ASCII only (L3).

module DoomAx7203Tracker;

; FACT 1. The CI job that builds the AX7203 bitstream exists and is the cheapest path.
;   Evidence: the workflow file .github/workflows/fpga-build.yml, job fpga-synth-ax7203.
;   Lines 649 (the gate), 664 (yosys on the cell), 682 (yosys on the board top),
;   689 (nextpnr-xilinx, 12600 s budget), 697 (fasm2frames + xc7frames2bit),
;   700-701 (upload-artifact, name wasm-board-ax7203, path build/fpga/ax7203/).
;   The artifact is a .zip with the .bit inside when the job gets past step 697.
;
; FACT 2. The CI job is gated on the diff touching one of four paths.
;   Evidence: line 649, the regex
;     specs/fpga/wasm_(board_ax7203|opcodes).t27 | fpga/verilog/wasm_board_ax7203_top.v
;     | .github/workflows/fpga-build.yml
;   A pull request that touches none of them leaves every step skipped and the job reads
;   "success" while it built nothing. Run 45328688829 is the example: fpga-synth-ax7203
;   concluded success with steps 4..13 all skipped, and no wasm-board-ax7203 artifact.
;
; FACT 3. No CI run has ever produced an AX7203 bitstream for the WASM board.
;   Evidence: the artifact list of run 45328688829 holds fpga-bitstream-6a6bc3fc... (7122 B),
;   fpga-synthesis, fpga-formal and fpga-verilog -- but no wasm-board-ax7203.
;   The run that failed on 2026-11-15 (43690914963) died at yosys with exit 124, the 3600 s
;   timeout, and its artifact holds only yosys.log.
;
; FACT 4. A second, working recipe for AX7203 bitstreams lives in the sibling repository.
;   Evidence: trinity-fpga/.github/workflows/build-ax7203-bitstream.yml, workflow_dispatch,
;   inputs.design. It runs the same openXC7 steps inside regymm/openxc7 on an x86 runner
;   (no QEMU), caches the chipdb under key chipdb-xc7a200tfbg484-2-v2, and uploads
;   <design>_bitstream. It is the same flow that produced artifacts/bitstreams/gf8_clean_ax7203.bit.
;
; FACT 5. The HDMI TX pins of the AX7203 are not in this repository.
;   Evidence: trinity-fpga/specs/fpga/constraints/ax7203.xdc holds 39 lines -- clock R4/T4,
;   reset T6, LEDs B13/C13/D14/D15, UART N15/P20 -- and no HDMI, VGA or DVI pin.
;   The header of specs/fpga/vga_text_core.t27 states it outright:
;   "HOW VGA REACHES THE BOARD IS NOT ESTABLISHED in this repository (fpga/HARDWARE_SSOT.md
;   and every .xdc here carry no VGA, HDMI or DVI pin), so this cell has no pins and no top".
;   specs/fpga/tmds_encoder.t27 and specs/fpga/hdmi_timing_640x480.t27 say the same:
;   "No pin numbers here (they come from the schematic)."
;
; FACT 6. The cells that carry the video chain exist and are complete.
;   Evidence: specs/fpga/tmds_encoder.t27 (167 lines), specs/fpga/hdmi_timing_640x480.t27
;   (130 lines), specs/fpga/vga_text_core.t27 (212 lines), specs/fpga/wasm_fb.t27 (296 lines,
;   320x200 palette framebuffer), specs/fpga/i2c_master.t27 (254 lines, one three-byte write),
;   specs/fpga/mmcm.t27 (271 lines).
;   What is missing is the SI9134 init sequence and the top that binds the cells to pins.
;
; FACT 7. The DOOM software itself is a WASM module, and the board cell cannot hold it yet.
;   Evidence: specs/fpga/wasm_board_ax7203.t27 implements P3..P7 of the WASM MVP over ONE
;   64 KiB linear-memory page (P5). DOOM needs 16 MiB, which is why specs/fpga/ddr3_head_bandwidth.t27
;   and specs/fpga/ddr3_ternary_matvec.t27 exist as the DDR3 starting point.
;   The local toolchain has no emcc, no rustc and no wasm-ld; only zig can target wasm32.
;
; FACT 8. The HDMI capture dongle behind the USB hub does not enumerate on this Mac.
;   Evidence: ioreg -p IOUSB -c IOUSBHostDevice -w0 lists three devices -- USB2.0 HUB,
;   CP2102N USB to UART Bridge Controller, Digilent USB Device. No video device.
;   ffmpeg -f avfoundation -list_devices true lists only FaceTime HD Camera, OBS Virtual Camera
;   and Capture screen 0.
;
; BLOCKER ORDER. The HDMI pins (FACT 5) gate every visible milestone. The bitstream (FACT 3)
; gates every milestone. The WAD-sized memory (FACT 7) gates the game itself.
;
end DoomAx7203Tracker;
