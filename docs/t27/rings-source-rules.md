# RING rules found in source (Trinity-S-AI-DNA/trios)

Status: LOCAL DRAFT, not committed. Restored 2026-11-16 after a worktree recycle.
Every rule below cites a file and line in Trinity-S-AI-DNA/trios. Anything not in
this table is NOT_FOUND_IN_SOURCE and must not be invented.

## 1. Confirmed rules

| Id  | Rule | Source | Usable in t27? | Owner decision? |
|-----|------|--------|----------------|-----------------|
| R1  | A ring lives at `crates/<name>/rings/XX-NN/` | AGENTS.md:87-94 | Path shape only; t27 has no crates | Yes: t27 home path |
| R2  | A ring has README.md, TASK.md, AGENTS.md | AGENTS.md:87-94 | Yes | No |
| R3  | A ring missing any of its three .md files is a violation (I5) | AGENTS.md:96 | Yes | No |
| R4  | `XX-00` is the identity / core-types ring | AGENTS.md:98 | Yes | No |
| R5  | `XX-01..N` are layers; dependency order follows the ring number | AGENTS.md:99-100 | Yes | No |
| R6  | BR-OUTPUT / BR-APP / BR-BIN / BR-MODEL is the final artifact ring | AGENTS.md:101 | Yes | No |
| R7  | Ring documents are never renamed, merged or deleted | AGENTS.md:108-110 | Yes | No |
| R8  | arch-guard CI checks the ring layout | AGENTS.md:111 | No t27 equivalent yet | Yes |
| R9  | Source of truth: #238, #243, crates/trios-a2a | AGENTS.md:145-148 | Reference only | No |
| R10 | Silver tier = CR-* pure ring | CR-CHAT-LAWS/RING.md:3,27-28 | Yes | No |
| R11 | Bronze tier = BR-OUTPUT, re-exports only | BR-OUTPUT-CHAT/RING.md:3,23-26 | Yes | No |
| R12 | Logic goes into the pure ring first | BR-IO-CHAT-07/AGENTS.md:11 | Yes | No |
| R13 | Laws are append-only (ADR + hash + proof) | CR-CHAT-LAWS/AGENTS.md:3-9 | Yes | No |
| R14 | Doctor runs 7 checks per ring incl. RING.md, AGENTS.md, TASK.md | plans/trios-doctor-rings-254.md:7,27-39 | Reference only | No |
| R15 | UI rings UR-00..UR-08 + BR-APP; UR-03 = "Chat component (messages, input)", depends on UR-06 (state) and UR-08 (theme) | plans/trios-ui-plan.md:11-21,15,30 | Yes, as UR-03 | No |

Gold tier: named in the SR-01 Cargo.toml only; no file defines the tiers.
Tier definitions as a whole: NOT_FOUND_IN_SOURCE.

## 2. Context

- C1: In trios, `chat` is the encrypted Secure Chat messenger.
- C2: Its t27 port is already tracked in #8882, #8884, #8887, #8888 and #11994-#11997.
  `ui-chat` is NOT that port.
- C3: UR-03 in plans/trios-ui-plan.md is the only source for a chat UI ring.
- C4 (conflict): the UI plan treats UR-00 as the entry (top), while AGENTS.md:99
  treats XX-00 as the core. Unresolved; the owner decides.
- C5 (conflict): tasks/INVARIANTS.md:13 defines I5 as "single extension tree",
  which differs from AGENTS.md:96. Unresolved; the owner decides.

## 3. NOT_FOUND_IN_SOURCE

- a ring manifest file (e.g. `*.ring.t27`);
- a spec/ kit/ objects/ tests/ layout inside a ring;
- partner personalization;
- avatar presence states (idle, listening, thinking, speaking, offline);
- a visual-regression rule for rings;
- a T27L RING record schema;
- vendor/tri-mcp/rings beyond the SR-00..02 Rust crates (SR-02/TASK.md marks RING.md as not written);
- policies/, proofs/, templates/ and tasks/ hold no ring rules.

## 4. Implications for t27

- Ring name `ui-chat`, family `UR`, id `UR-03-UI-CHAT` (owner decision 2026-11-16,
  APPROVED_AS_CANDIDATE).
- Home: `specs/uikit/rings/ui-chat/` (owner order UI_CHAT_RING_V0).
- The ring carries README.md, TASK.md, AGENTS.md (R2, R3) and one contract spec.
- No ring renames, merges or deletes (R7). Logic goes into the pure spec first (R12).
