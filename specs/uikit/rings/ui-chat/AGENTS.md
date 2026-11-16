# AGENTS -- ui-chat (UR-03-UI-CHAT)

Rules for any agent that touches this ring. Each rule is cited; see
`docs/t27/rings-source-rules.md` for the full table.

1. Keep README.md, TASK.md and AGENTS.md. Never rename, merge or delete them
   (trios AGENTS.md:96, 108-110).
2. Logic goes into `ui-chat.t27` first, with tests (trios BR-IO-CHAT-07/AGENTS.md:11; t27 L4).
3. Reuse UI Kit and T27F primitives. Do not add a second avatar, button, input,
   text or card spec.
4. Avatar: no ring, no outline, no border, no glow ring, no halo. Use only the
   six `uikit_avatar` states. New avatar states need an owner contract.
5. Do not invent what is NOT_FOUND_IN_SOURCE: a ring manifest, a spec/kit/objects/tests
   layout, presence states, a ring VR schema, a T27L RING schema.
6. partner_profile stays a placeholder with status OWNER_DECISION_REQUIRED. No
   referral parsing, manager avatar loading, Telegram, payments or external identity.
7. No payment integration. Telegram is a contact channel only; never scrape
   Telegram avatars.
8. No proprietary assets, fonts, images or code.
9. No commit, push, issue, PR or publication without the owner's explicit yes.
10. English and ASCII only (L3).
