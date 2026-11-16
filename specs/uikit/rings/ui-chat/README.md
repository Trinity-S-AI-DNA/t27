# ui-chat (UR-03-UI-CHAT)

Status: DRAFT CONTRACT, APPROVED_AS_CANDIDATE (owner, 2026-11-16). Local, not committed.

A reusable conversational assistant UI module for any T27 application.

It is NOT Secure Chat (the encrypted trios messenger) and NOT the trios-chat port
(#8882, #8884, #8887, #8888, #11994-#11997).

## Source

- Ring layout and the three documents: Trinity-S-AI-DNA/trios AGENTS.md:87-96 (R1-R3).
- Family and number: plans/trios-ui-plan.md:15 names UR-03 "Chat component
  (messages, input)"; line 30 says it depends on UR-06 (state) and UR-08 (theme) (R15).
- Full rule table: `docs/t27/rings-source-rules.md`.

## What the ring is made of

Only existing primitives, reused, never copied:

| Part | Spec |
|------|------|
| Avatar | specs/uikit/avatar.t27 (`uikit_avatar`) |
| Send button | specs/uikit/button.t27 (`uikit_button`) |
| Input line | specs/uikit/input_model.t27 (`uikit_input_model`) |
| Message text | specs/uikit/text.t27 (`uikit_text`) |
| Message bubble | specs/uikit/card.t27 (`uikit_card`) |
| Message stream | specs/t27f/live_chat.t27 (`live_chat`) |
| Frame render | specs/t27os/vf_frame_main.t27 |
| Visual regression | specs/t27f/vf_diff.t27 |
| Layout, meaning, tokens, theme | specs/t27f/vf_layout.t27, vf_meaning.t27, vf_tokens.t27, vf_theme.t27 |

The contract is `ui-chat.t27` in this directory.

## Avatar rule

Owner: there is no ring and no outline around the avatar. No border, glow ring or
halo. Only the existing `uikit_avatar` interactive states are used (6 states).

## Visual target

The visual language of t27.ai vibecoding pages, built only from the primitives
above. No proprietary assets, fonts, images or code are copied.

## Not in this ring

Partner personalization, manager avatar source, avatar animation states,
payment, Telegram integration and external publication are
OWNER_DECISION_REQUIRED. See TASK.md.
