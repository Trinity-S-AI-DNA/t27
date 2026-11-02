# Single full-run plan (NOT executed; awaits review decision)

Candidate design (concrete, no wildcards):
- spec: specs/port/trinity/fpga/openxc7-synth/blink_test_top.t27
  (module BlinkTestTop; seal openxc7-synth_blink_test_top.json, all five
  hashes MATCH under t27c sha256 041fb69cd5fe45b2...)
- xdc: fpga/openxc7-synth/test_top.xdc -- matches this design's ports
  exactly: clk -> F22 (LVCMOS33, create_clock 20ns), led -> J26
  (LVCMOS33). This is the committed pair used for the committed
  test_top.bit evidence; constraints are NOT invented.
- part: xc7a200tfbg676-1; chipdb build/fpga/openxc7/xc7a200tfbg676-1.bin;
  part.yaml build/fpga/openxc7/prjxray-db/artix7/xc7a200tfbg676-1/part.yaml
- ring-osc flag: NOT needed for this candidate (real clk, no comb loop);
  --ignore-loops stays off and is recorded as false in the receipt.
- attempt dir: build/fpga/synth-spec-attempt-<unixts>/ via --build-dir
  (outside target/t27os; server 46075 untouched; no board/flash actions).
Command:
  ./target/release/tri fpga synth-spec \
    --spec specs/port/trinity/fpga/openxc7-synth/blink_test_top.t27 \
    --part xc7a200tfbg676-1 \
    --xdc fpga/openxc7-synth/test_top.xdc \
    --build-dir build/fpga/synth-spec-attempt-1
Expected per stage: gen-verilog log+hash == seal gen_hash_verilog;
yosys -> St...json; nextpnr -> .fasm (note: --ignore-loops ABSENT by
default; if nextpnr rejects a loop the run FAILS CLOSED and the flag
becomes an explicit decision, not a silent default); fasm2frames ->
.frames; xc7frames2bit -> non-empty .bit; receipt admitted=true,
outcome=bitstream_produced, hardware_verified=false, ignore_loops=false.
Timeouts/stop: wall-clock guard 20 min per run; any stage failure or
empty bit -> no retry, receipt kept, run reported as failed.
Known open gap: tool versions are only in stage logs, not parsed into
the receipt -- acceptable for this run; structured versions deferred.
