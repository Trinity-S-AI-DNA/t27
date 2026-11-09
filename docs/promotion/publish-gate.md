# Publish gate

Part of #13473. Issue #13485. The gate is closed. Nothing was published.

## Refuse unless all are true

From #13485 and `proof_ready` in `specs/promotion/t27l_promotion.t27`:

1. a widget
2. a screenshot
3. a passing test
4. a human approval for that named draft

From #13473, the asset also needs a claim, an artifact, a reproducible path, a visual proof, a CTA, and a metric (`asset_ready`). `may_publish` is false if either gate is false.

The owner's yes for a social post is already specified in `specs/skills/t27-social-visual-consent.t27`: explicit yes, in the owner's own chat, for that post and that channel, after the uniqueness gate. This pack does not copy that spec. No such yes was given for any draft here.

## This run

| check | state |
| --- | --- |
| widget for a new post | not rendered |
| screenshot | not taken |
| test of a post | the promotion gate spec passed; that is not a test of a sent post |
| human approval | absent |
| content-bank `sent` | false on all 9 drafts |
| X, LinkedIn, Telegram, Habr, Instagram, email | not called to send |

## Asset fields

| Field | Value |
| --- | --- |
| claim | N/A |
| artifact | `specs/promotion/t27l_promotion.t27` |
| reproducible path | `./target/debug/t27c test-report specs/promotion/t27l_promotion.t27` |
| visual | N/A |
| CTA | N/A -- not sent |
| metric | drafts sent: 0 |
