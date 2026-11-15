# Subagent roster study

Read-only extraction at revision `ed38624709` (2026-11-15). Every row cites `file:line` at that revision.
No name, parent, capability or scope below was inferred: a field with no line is `absent`.

## Result

```
ROSTER_PATH: none (no roster spec with entries exists)
SUBAGENT_COUNT: 0
SUBAGENT_NAMES: none_registered
BLOCKER: subagent roster entries absent
DEFINED_ONLY_TREE_NODES: 3, of which 1 is of kind SUBAGENT (specs/queen/agent_registry.t27 L1712-1773)
HOST_DEFINITION_FILES: 12 (.claude/agents/*.md)
NEXT:
- define subagent registration schema
- register real agents
```

A registered entry here means a record that carries the eight fields asked for: name, parent, domain,
role id, capabilities, write scope, worktree, status. No file in the tree holds such a record.

## What "roster" names in the tree

| Path | What it is | Evidence |
| --- | --- | --- |
| `specs/queen/agent_registry.t27` sections 1-8 | Decision table for registering an agent. The store is "Not here". `AR_MAX_AGENTS = 27`, `AR_MAX_PER_OWNER = 9`. No stored entry. | L4-5, L61, L65 |
| `docs/agents/AGENTS_ALPHABET.md` | The 27-agent alphabet; the agent README calls it the place "where the roster is defined". These are agents, not subagents. | `specs/agents/README.md:58`, `:130` |
| `specs/queen/team_msg.t27` | Team message contract with member status transitions. No member entries. | L224, L623 |
| `specs/port/trios/crates/trios-cli/src/cmd/roster.t27` | Port stub: the bodies of `roster_update` and `roster_show` are `undefined`. | L20, L24 |
| `.trinity/seals/cmd_roster.json` | Seal of the stub above. | file name |
| `specs/queen/lotus.t27` | One comment that uses the word. | L167 |

## Contract level: the pilot tree (defined only)

`specs/queen/agent_registry.t27` section 9h: "defined only: no capability is granted and no process is
started" (L1706, L1766).

| Index | Id | Kind | Parent | Evidence |
| --- | --- | --- | --- | --- |
| 0 | `0xHyperSupervisor@kernel.t27` | HYPER (0) | itself (root) | L1713, L1715, L1716, L1773 |
| 1 | `0xBuildSupervisor@build.t27` | SUPERVISOR (1) | index 0 | L1713, L1715, L1773 |
| 2 | `0xCompiler@build.t27` | SUBAGENT (2) | index 1 | L1713, L1715, L1773, L1824 |

The one node of kind SUBAGENT, in the requested shape:

```
1. NAME=absent PARENT=0xBuildSupervisor@build.t27 DOMAIN=T27B (build) ROLE_ID=0xCompiler@build.t27
   CAPABILITIES=absent (no per-node grant) WRITE_SCOPE=absent WORKTREE=absent STATUS=defined only, not runnable
```

It is not counted as a registered subagent: five of the eight fields have no value.

- Kinds and authority: `AN_KIND_HYPER`, `AN_KIND_SUPERVISOR`, `AN_KIND_SUBAGENT` L1768-1770;
  `an_kind_may_execute` is true for SUBAGENT only (L1796-1798); `an_kind_may_own` allows (HYPER, SUPERVISOR)
  and (SUPERVISOR, SUBAGENT) only (L1801-1805).
- Slots: `specs/queen/actor_pattern.t27` requires all ten slots for a subagent (`AP_ALL_SLOTS = 1023`, L79,
  L85). Filled today: id, parent, kind (`AP_TODAY = 7`, L105; `AP_NODE_FILLED = [7, 7, 7]`, L107). Unfilled:
  capability grant, write scope, message contract, restart policy, heartbeat, T27L event, journal replay
  (L99-104). `ap_row_runnable` needs every required slot (L127-129), so no node is runnable.
- Capability ceiling, not a grant: `cc_ceiling` in `specs/queen/agent_card.t27` gives a kind that may execute
  at most read (L333), peer mail and tasks (L343), edit and run (L345) and web (L347); the lead and subagent
  bits need `an_kind_may_start_supervisor` or `an_kind_may_assign` (L335-338). "no per-node grant exists"
  (`actor_pattern.t27` L101). `CARD_IS_A_REGISTRY = false` (`agent_card.t27` L171).

## Host level: Claude Code agent definition files

Twelve files under `.claude/agents/`. Each has a `name:` line and `model: sonnet`. None has a parent, domain,
role id, write scope, worktree or status field, and none is bound to a tree node.

| File | name: | Line | Alias candidate in the registry |
| --- | --- | --- | --- |
| agent-c-compiler.md | Agent C - Compiler | 4 | old name `agent-c` -> `0xCompiler@build.t27` |
| agent-e-experience.md | Agent E - Experience | 4 | old name `agent-e` -> `0xMemory@memory.t27` |
| agent-l-lsp.md | Agent L - LSP Validator | 4 | none |
| agent-v-verify.md | Agent V - Verification | 4 | old name `agent-v` -> `0xVerifier@quality.t27` |
| creator.md | creator | 2 | none |
| experience.md | experience | 2 | none |
| learner.md | learner | 2 | none |
| queen-browser-steward.md | queen-browser-steward | 2 | none |
| review-warden.md | review-warden | 2 | old name `review-warden` -> `0xQualityGate@quality.t27` |
| t27c-steward.md | t27c-steward | 2 | none |
| trinity.md | trinity | 2 | none |
| verifier.md | verifier | 2 | none |

Aliases: `AN_ALIAS_OLD` and `AN_ALIAS_NEW`, `specs/queen/agent_registry.t27` L1247-1248, "candidates, none
activated by this file" (L1242). Only `review-warden` equals a `name:` value byte for byte; `agent-c`,
`agent-v` and `agent-e` match a file-name prefix, not a `name:` value.

## A file that reads like a journal and is not evidence

`.trinity/agent_events.jsonl` (18 lines, last touched by `cc937f9106` and `e841ec909d`) holds events such as
`subagent.spawn` and `tri.subagent.spawn` dated 2026-11-13 with `canonical_id` `0xA@architecture.t27`
(14 occurrences). That id fails `an_id_ok`; the registry asserts the refusal
(`specs/queen/agent_registry.t27` L1758), and `AGENTS.md:707` records that nothing in the 2026-11-13 note
exists. The file is named in `AGENTS.md`, `docs/agents/AGENTS_ALPHABET.md`, `docs/system/queen.md` and five
domain specs. It is not evidence of a spawned subagent.

## Tests

| Test | State | Evidence |
| --- | --- | --- |
| `verify_subagent_registry_completeness` | not_implemented | 0 hits under `specs` and `docs` |
| nearest existing: `an_pilot_tree` | exists, not run here | `specs/queen/agent_registry.t27` L1718 |
| nearest existing: `an_pilot_roles` | exists, not run here | `specs/queen/agent_registry.t27` L1807 |

## Missing

- A registration schema for a subagent (the eight fields above in one record).
- Any registered subagent entry.
- A name, a capability grant, a write scope, a worktree and a status for the pilot leaf.
- A binding between a `.claude/agents` definition and a tree node (the four aliases are candidates only).
- `verify_subagent_registry_completeness`.
