# Funnel

Part of #13473. This card does not publish and does not take a payment.

The executable check is `specs/automation/promotion_kernel.t27`
(`FUNNEL_NAME`, `funnel_ctas`). Each step has one CTA.

| Step | Name | CTA |
|------|------|-----|
| 0 | X | read the next link; nothing is posted from this card |
| 1 | arxiv-github | open the paper or the repository |
| 2 | course | open the course |
| 3 | widget | open the lesson widget |
| 4 | good-first-issue | open one good-first issue |
| 5 | contributor | follow CONTRIBUTING |
| 6 | paid-product | not offered; paid checkout is off |
| 7 | b2b | not offered; paid checkout is off |

Steps 6 and 7 are in the map so the later ladder has a place. They do
not link to a payment. `PAID_CHECKOUT` is false.
