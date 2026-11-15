# t27b optimizer evidence -- bench_audit (2026-11-15)

Reproduce: `T27B_TRACE_PASSES=1 ./target/release/t27b asm specs/tri/shootout/bench_audit.t27`

## Trace (release binary): 8 passes FIRED
```
   5 pass=licm_body_const verdict=BAIL reason=LICM_NONE (needs >= 2 insns or no free reg)
   2 pass=ifconv verdict=FIRED reason=if converted to csel
   2 pass=ifconv verdict=BAIL reason=shape mismatch (var/const/reg/srcs guard)
   1 pass=licm_body_const verdict=FIRED reason=const hoisted to callee-saved reg
   1 pass=inline_verdict_fn verdict=FIRED reason=select-shaped callee inlined
   1 pass=inline_verdict_fn verdict=BAIL reason=should_inline_call or select_shape_ok false
   1 pass=countdown verdict=FIRED reason=loop i to countdown
   1 pass=cinc verdict=FIRED reason=cset+add folded to csinc
   1 pass=check_elide_c verdict=FIRED reason=loop accumulator
   1 pass=check_elide_c verdict=BAIL reason=predicate accumulator_elides false
   1 pass=check_elide_b verdict=FIRED reason=decrement after inc
   1 pass=check_elide_b verdict=BAIL reason=decrement_elides false or x+x-c absent
   1 pass=check_elide_a verdict=FIRED reason=loop induction i
```

## Hot loop 0x9c..0xc0, 10 words (release asm)
```
      98: 1400000a  b 0xc0
      9c: eb15027f  cmp x19, x21
      a0: 9a9606d6  csinc x22, x22, x22, eq
      a4: 91000673  add x19, x19, #1
      a8: d1000670  sub x16, x19, #1
      ac: d2807d11  movz x17, #0x3e8
      b0: f10fa27f  cmp x19, #1000
      b4: 9a9383f3  .word 0x9a9383f3
      b8: 9a908235  .word 0x9a908235
      bc: d1000694  sub x20, x20, #1
      c0: 35fffef4  cbnz w20, 0x9c
```

## Peeled first iteration keeps the check (adds+b.hs at 0x78/0x7c)
```
24:      78: b1000673  adds x19, x19, #1
25:      7c: 54000342  b.hs 0xe4
```
