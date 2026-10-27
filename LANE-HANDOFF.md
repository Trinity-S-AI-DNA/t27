# Lane handoff: the float floor (P4) -- f32/f64.const as BIT WORDS (the fleet's last refusal)

Branch spec/wasm-float-const (off P3's spec/wasm-i64-const; P3 is PR #12033). Goal:
OP_F32_CONST=0x43 (four raw IEEE-754 bytes, NO LEB) and OP_F64_CONST=0x44 (eight
raw bytes) push the bits as the u32 word -- f32 wholesale, f64 as the low 32, the
same honest floor as i64. This closes the fleet's last named refusal (f_f32 2.5 ->
word 0x40200000 -> die answer low24 0x200000).

The trap that ate my attempt: the spec's dispatch in step_all is a DEEPLY NESTED
if/else chain with (probably) TWO opcode-dispatch loops (the MVP code[] runner and
the user-program uprog[] runner -- grep `scrap_op == OP_I32_EQZ` for ALL sites).
My sibling-arm insertion via regex matched one `} else { if EQZ` boundary and
produced 29 warnings (baseline 1) incl. `test roundtrip_errors declared 2 times`
-- structural damage that still typechecked. Place the arm in EVERY dispatch loop
that handles OP_I32_CONST, count braces by hand or use a proper parser pass, and
add the behavioral test in the p3_i64_wire_programs idiom: programs
`43 00 00 20 40 0B` (f32 2.5) and `44 <8 bytes of 2.5 f64> 0B`, save/restore
page 0, expect cap words 00 00 20 (low24 of 0x40200000) and the f64's low-32.
Verify: t27c check warnings == baseline 1, test-report green, then the full loop
(gen-verilog -> build_trinet_node.py --no-node-params --nosrl -> SRAM flash via
openFPGALoader -m -> fleet_crossing.py + wasm_board_p2.py both better/equal).
