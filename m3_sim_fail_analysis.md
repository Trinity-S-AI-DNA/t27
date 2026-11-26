# M3 Simulation Failure Analysis

**Date:** 2026-11-26T13:04:23+07:00
**Goal:** BRAM-backed 16×16 fixed-point matmul with sequential B-vector reads
**Current Status:** FAIL

## Failure Observation

Simulation (`test_m3_sim.vvp`) ran clean but `accum` remained `00000000`:

```
Time=0: accum=00000000
test_m3.v:62: $finish called at 200210000 (1ps)
```

Expected: `accum = 0x00000400` (16 × 0x0080 × 0x0080 >>> 8).

## Root Cause

**BRAM Reset vs. Preload Conflict**

In `/Users/playom/t27/rtl/lamb_labs/bram_a.sv` (and `bram_b.sv`):

```systemverilog
initial begin
  dout0 <= 16'h0080;  // Preload to 0x0080 (Q7.8)
  ...
end

always @(posedge clk or negedge rst_n) begin
  if (!rst_n) begin
    dout0 <= 16'd0;  // RESET: wipes initial to zero!
    ...
  end
end
```

The `if (!rst_n)` branch runs whenever `rst_n` is low, and the `initial` block only sets values at time 0 **before** the `always` block. Once `rst_n` goes low then high, the reset branch overwrites the initial values permanently. There is no reload path to restore `0x0080`.

**Consequence:** The FSM asserts `bram_en` and cycles through `addr = 0..15`, but `mac16` sees all-zero vectors and accumulates 0.

## Secondary Issue: Functional Dead Path

The FSM drives `bram_a_addr` and `bram_b_addr` from `cycle_count`:

```systemverilog
assign bram_a_addr = cycle_count;
assign bram_b_addr = cycle_count;
```

However, `mac16` is **combinational**, not sequential:

```systemverilog
always @(*) begin
  p0 = A0 * B0; ...
  sum = p0 + ... + p15;
end
always @(posedge clk or negedge rst_n) begin
  if (en)
    accum <= sum >>> 8;
end
```

This means `mac16` computes the full dot product in a single combinational step, regardless of how many cycles `bram_en` is asserted. The `bram_en` gating is currently redundant (the `mac16` `en` input is still required for accumulation, but the address increment path is not used by `mac16`).

**Design Implication:** The original M3 spec called for "sequential B-vector reads during MAC accumulation cycles" — but the current `mac16` architecture does not support that. To honor the spec, we would need:
- 16 DSP48E1 MAC stages pipelined with BRAM read addresses incrementing each cycle, OR
- Keep the combinational `mac16` but relax the spec to "full dot product in one cycle" (matching M2 evidence)

## Evidence

- **M2 Sim:** `FINAL TIME=350: accum = 0x00000400` ✅ PASS (with `rst_n` held high)
- **M3 Sim:** `accum` stuck at `00000000` (reset clobbers preload)
- **Testbench:** `test_m3.v` toggles `rst_n = 0` at time 0, triggering the reset branch

## Recommended Fix

**Option A (minimum change):** Fix BRAM reset/preload to preserve 0x0080 after reset.
- Change reset branch to restore the initial values:
  ```systemverilog
  if (!rst_n)
    dout0 <= 16'h0080;  // Restore, not zero
  ```
- This allows the current combinational `mac16` to see correct vectors.

**Option B (spec alignment):** Refactor to true sequential MAC.
- Replace combinational `mac16` with pipelined DSP48E1 chain (16 stages).
- Connect `bram_a_addr`/`bram_b_addr` to increment across cycles.
- This honors M3 spec but is a larger architectural change.

## Timeline Status

- `m3-review-gate`: done (1/1) — gate passed, but implementation incomplete
- `m3-implement`: pending — needs fix before simulation
- `m3-simulate`: pending — needs fix before running

## Next Step

Fix `bram_a.sv` and `bram_b.sv` reset branch to restore `0x0080`, recompile, and re-run simulation to verify `accum = 0x00000400`.
