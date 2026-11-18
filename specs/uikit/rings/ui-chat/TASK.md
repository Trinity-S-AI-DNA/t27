# TASK -- ui-chat (UR-03-UI-CHAT)

Order: UI_CHAT_RING_V0 (owner, 2026-11-16), mode SOURCE_FIRST_UI_KIT_FIRST, P0,
status APPROVED_TO_DRAFT.

## Done in V0 (local only)

- [x] README.md, TASK.md, AGENTS.md exist (trios AGENTS.md:96, I5).
- [x] `ui-chat.t27` draft contract with tests (L4).
- [x] Every rule cited or marked NOT_FOUND_IN_SOURCE.
- [x] Avatar reuses `uikit_avatar` states; no ring, no outline.

## Open (needs an owner decision)

| Item | Status |
|------|--------|
| partner_profile | OWNER_DECISION_REQUIRED (placeholder only) |
| manager avatar source | OWNER_DECISION_REQUIRED |
| avatar animation states | OWNER_DECISION_REQUIRED (needs an owner contract) |
| payment action | FORBIDDEN without separate approval |
| Telegram integration | OWNER_DECISION_REQUIRED; contact channel only; no avatar scraping |
| external publication | OWNER_DECISION_REQUIRED |
| ring home path in t27 vs trios `crates/<name>/rings/` | OWNER_DECISION_REQUIRED |
| UR-00 entry vs XX-00 core conflict | OWNER_DECISION_REQUIRED |

## Not done (blocked)

- Commit, push, issue, PR, publication: blocked until the owner says yes.
- Screenshot and visual regression: need a render through vf_frame_main and
  vf_diff; not produced in V0.
