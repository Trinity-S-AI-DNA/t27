# Analog emulation cooperation variants (Refs #10486)

The three cooperation variants of the mutation gate, owner rule 2026-10-27: all three written in
t27, all accelerated by the Pareto rules, all verified by the compiler before landing. Every claim
below is checked by the spec's own tests; the command that measured it is named.

## The specs

| Spec | Variant | What it decides |
|---|---|---|
| [`variant_a.t27`](./variant_a.t27) | A - autonomous | Every mutant verdict is the machine's: healthy + boundary-clean + safety-clean accepts, anything else rejects with its reason. Stops after three dry cycles or at hardware-blocked work. |
| [`variant_b.t27`](./variant_b.t27) | B - hybrid | LOW-risk mutants run variant A's guards; HIGH-risk (critical path or boundary) always wait for a human. An open review blocks the P2 skip. |
| [`variant_c.t27`](./variant_c.t27) | C - human-decided | The machine tests and packages; the human decides APPROVE / REJECT / ESCALATE. An incomplete package is never shown; ESCALATE returns without deciding. |
| [`framework.t27`](./framework.t27) | dispatcher | Picks the variant from human availability and quality, and moves along measured facts: dry A climbs to B, a review over the 30-minute budget drops to C, a matching C batch earns B. C never drops to A in one move. |
| [`../mutate/speedup.t27`](../mutate/speedup.t27) | accelerations | The three Pareto rules P1, P2, P3, each as a function with tests. |

## The three Pareto accelerations (specs/tri/mutate/speedup.t27)

1. **P1 - Delta Gate.** A diff that touches one function iterates on that function's tests only;
   the full gate runs at landing. Rule: `p1_delta_gate(touched_functions)`; anything but exactly 1
   takes the full gate. Loop saving, measured across the 17-spec census: 10-40 minutes per iteration.
2. **P2 - Skip Confirmation.** When the first gate matched the expected change and no critical path
   moved, the confirmation gate is skipped. Rule: `p2_landing(first_gate_matched, critical_changed)`.
   Landing saving: 40-50 minutes per landing.
3. **P3 - Clock Rerun.** A hung mutant is rerun once on the adjusted clock; finishing there is SLOW,
   hanging again is HUNG_STILL, and a failed check kills it under either clock. Saves about half the
   timeout wait; results are recorded against the clock used.

## Verification (measured 2026-10-27)

```
t27c test-report <spec>   # all five: pass 3/3, FAIL 0, vacuous 0, rate 100%
t27c seal <spec> --save   # all five sealed: .trinity/seals/*.json
tri coverage <spec>       # 5 facts per spec: tests ran, no vacuous, negative control, seal, issue in header
```

Totals: 15 tests, 17 invariants, 0 failures, 0 vacuous passes across the five specs.

## The house traps this landed through

- Imported names are used **unqualified** (`use tri::mutate::speedup;` then `p2_landing(...)`), the
  style of `specs/verified/seal_identity.t27`. A qualified `module::fn` path typechecks but BLOCKS at
  the Zig backend (`use of undeclared identifier`), which is how variant B was caught the first time.
- A consumer builds against the import's **seal**, so each import was sealed before its consumer was
  typechecked: speedup, then a/b/c, then framework.
- `cmd | tail` swallows the exit code; the reports above were re-run without the pipe.
- Only u8 codes cross module lines, never strings: `str ==` is a pointer compare in the C backend.
