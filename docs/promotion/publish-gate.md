# Publish gate

Part of #13473. This card does not publish.

The executable check is `specs/automation/promotion_kernel.t27`
(`publish_gate`, `gate_transmits`).

## Rule

`publish_gate` is true only when all four are present:

1. a widget
2. a screenshot
3. a passing test
4. a human approval

Missing any one of the four refuses the gate.

`gate_transmits` is false. An open gate in this spec does not send a
post, a mail, or a paid call. A later host may ask the owner; this
card does not.
