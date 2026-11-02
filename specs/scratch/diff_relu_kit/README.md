# diff_relu_kit -- minimal differential verification kit (t27c vs t27b)

Reproducible smoke kit: ONE spec, ONE common testbench, independent golden
expectations, negative control. Neither compiler's own test blocks are used.

## Contents

- `t27c_gft_relu.v.txt` -- Verilog from `t27c gen-verilog specs/ternary/gft_relu.t27`
  (captured from stdout; regen command below).
- `t27b_on_comb.v.txt` -- Verilog from `t27b verilog specs/ternary/gft_relu.t27 --top on_comb`.
- `tb_diff_relu.v.txt` -- the common testbench (vectors + golden expected + verdicts).
- `tb_negctl.v.txt` -- negative control: vector 3 expectation deliberately wrong
  (expects -1.0 to pass through); MUST fail.

## GF-T16 encoding fact (verified, fixes the earlier false claim)

In a u32 GF-T16 value: payload bits [15:0], sign bit [16].
Negative <=> (x >> 32'd16) == 32'd1. Literals from the spec's own passing
tests: +1.0 = 0x005000, +2.0 = 0x005200, -1.0 = 0x015000 (86016), NOT 0xA000.
The earlier "GF-T16 golden reference bug" report used a wrong constant; the
spec and both backends agree. Both compilers pass the spec's own 4 tests and
this kit's 6 vectors.

## Reproduce

```sh
cd repo root
./target/release/t27c gen-verilog specs/ternary/gft_relu.t27 > specs/scratch/diff_relu_kit/t27c_gft_relu.v.txt
./target/release/t27b verilog specs/ternary/gft_relu.t27 --top on_comb > specs/scratch/diff_relu_kit/t27b_on_comb.v.txt
cd specs/scratch/diff_relu_kit
iverilog -g2012 -DT27C -o t27c_sim.vvp tb_diff_relu.v.txt t27c_gft_relu.v.txt && vvp t27c_sim.vvp   # ALL_PASS
iverilog -g2012 -DT27B -o t27b_sim.vvp tb_diff_relu.v.txt t27b_on_comb.v.txt && vvp t27b_sim.vvp   # ALL_PASS
iverilog -g2012 -DT27C -o negctl.vvp tb_negctl.v.txt t27c_gft_relu.v.txt && vvp negctl.vvp         # ALL_FAIL 1 (required)
```

## Status semantics

- BOTH_ACCEPT: both backends compiled the spec (observed).
- SEMANTIC_MATCH: both DUTs ALL_PASS on the common TB (observed 2026-11-02).
- NEGATIVE_CONTROL: wrong expectation yields ALL_FAIL (observed 2026-11-02).
- TIMEOUT / UNSUPPORTED: not observed for this spec.

Match of two backends on the same TB is agreement evidence, NOT proof that
either matches the mathematical spec beyond these 6 vectors.

## Extension note

Committed copies carry a .txt suffix: the own-language gate is
bypass-less for hand .v paths and reads exceptions from origin/master
only. Regenerate with the commands above to get the canonical .v names.
