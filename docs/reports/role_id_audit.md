# Role id audit

Read-only extraction at revision `ed38624709` (2026-11-15). Pattern searched under `specs`:
`0x[A-Za-z0-9_]+@[A-Za-z0-9_#-]+\.t27` (the hyphen is needed for `zero-trust`). Verdicts on `an_id_ok` are
by reading the rule (`specs/queen/agent_registry.t27` L978-990, L1008-1017), not by a run.

## Result

```
DISTINCT_IDS_UNDER_SPECS: 124
IN_ROLE_TABLE: 106 (AN_ROLE_IDS, L1593-1621)
VALID_BUT_OUTSIDE_ROLE_TABLE: 3
NEGATIVE_CONTROLS: 7
DECLARED_AS_AN_ID_AND_FAIL_THE_RULE: 8
```

106 + 3 + 7 + 8 = 124.

## The rule

`0x<ROLE>@<slug>.t27`; the role is CamelCase, upper-case first letter, 3..24 letters and digits (L978-990);
the slug is one of the 27 `AN_SLUGS` (L966, L1008-1017).

## The 106 role ids (second section 9f, role table)

`AN_ROLE_IDS`, one domain per line. "Every role below is DEFINED only: there is no live worker for it and no
capability grant" (L1589). Each id is `0x<Role>@<slug>.t27`.

| Line | Slug | Roles |
| --- | --- | --- |
| L1594 | agents | Identity, Reputation, Market, Guard |
| L1595 | build | Compiler, Linker, Optimizer, Artifact |
| L1596 | contracts | Contractor, Schema, Versioner, Compat |
| L1597 | design | Designer, Builder, Reviewer |
| L1598 | economics | CostAnalyst, BudgetGuard, ValueAnalyst |
| L1599 | files | Archivist, Indexer, Migrator, RetentionPolicy |
| L1600 | governance | Governor, PolicyMaker, Arbiter, Compliance |
| L1601 | health | Diagnostician, Profiler, Forecaster, Triage |
| L1602 | integration | Adapter, Bridge, Translator, Webhook |
| L1603 | jobs | JobRunner, Pipeline, Retryer, Scheduler |
| L1604 | memory | Memory, Curator, Retriever, Consolidator |
| L1605 | telemetry | Logger, Tracer, Meter, Observer |
| L1606 | actors | Actor, Router, Mailbox, Supervisor |
| L1607 | network | Connector, Proxy, RateLimiter, Resilience |
| L1608 | operations | Doctor, Monitor, Recoverer, Incident |
| L1609 | product | Planner, Prioritizer, Roadmapper, Backlog |
| L1610 | quality | Verifier, Tester, QualityGate, Fuzz |
| L1611 | research | Scout, Analyst, Synthesizer, Decider |
| L1612 | security | Guardian, Vault, Auditor, Incident |
| L1613 | time | Chronos, Scheduler, Timer, Decay |
| L1614 | ux | Navigator, Onboarder, AccessibilityAgent, Copywriter |
| L1615 | visual | BrowserSteward, CaptureAgent, Snapshotter, Comparator |
| L1616 | workflow | Automator, Orchestrator, Delegator, Approver |
| L1617 | extensions | PluginManager, Extension, Sandbox, Registry |
| L1618 | delivery | Releaser, Packager, Deployer, Rollbacker |
| L1619 | zero-trust | Sentinel, Authorizer, Redactor, Quarantine |
| L1620 | kernel | Constitution, Registry, Auditor, Resolver |

Four role names appear in two domains each: Incident (operations, security), Scheduler (jobs, time),
Registry (extensions, kernel), Auditor (security, kernel). Related test: `an_role_table_repeated_names`,
L1663.

## Valid by the rule, outside the role table (3)

| Id | Where | Note |
| --- | --- | --- |
| `0xOrchestrator@memory.t27` | `AN_OWNER_IDS`, L1687 | Owner's id for `specs/queen/claim_msg.t27` (L1686), status "active" (L1690). The role table has `0xOrchestrator@workflow.t27` (L1616) and `0xMemory@memory.t27` (L1604). |
| `0xHyperSupervisor@kernel.t27` | `AN_TREE_IDS`, L1713 | Pilot tree root, defined only. |
| `0xBuildSupervisor@build.t27` | `AN_TREE_IDS`, L1713 | Pilot supervisor, defined only. |

## Negative controls (7)

Written into tests so that a refusal is asserted. None is a claimed id.

| Id | Where |
| --- | --- |
| `0xLogger@nowhere.t27` | `specs/queen/agent_registry.t27` L1387, L1394 |
| `0xlogger@telemetry.t27` | `specs/queen/agent_registry.t27` L1388 |
| `0xIncident@knowledge.t27` | `specs/queen/agent_registry.t27` L1677-1678 |
| `0xHyperSupervisor@queen.t27` | `specs/queen/agent_registry.t27` L1756 |
| `0xBuildSupervisor@architecture.t27` | `specs/queen/agent_registry.t27` L1757 |
| `0xA@architecture.t27` | `specs/queen/agent_registry.t27` L1758 |
| `0xNobody@build.t27` | `specs/t27os/t27os_shell.t27:353` (the shape is valid by reading; `sh_agent_tab_ok` refuses it) |

## Declared as an id in a spec and fail the rule (8)

Each is a `pub const` that a test in the same file asserts. None is in the registry.

| Id | Declared at | Why it fails `an_id_ok` |
| --- | --- | --- |
| `0xG@gitlab_adapter.t27` | `specs/ci/gitlab_adapter.t27:19` (`GA_ID`) | role `G` is 1 character; slug not in `AN_SLUGS` |
| `0xG@governance.t27` | `specs/ci/gitlab_adapter.t27:22` (`GA_DOMAIN_ID`) | role `G` is 1 character (the slug is valid) |
| `0xG@mega-epic.t27` | `specs/domains/domain-g.t27:14` | role is 1 character; slug not in `AN_SLUGS` |
| `0xPR@paretO.t27` | `specs/domains/domain-pr.paretO.t27:15` | role is 2 characters; slug not in `AN_SLUGS` |
| `0xM@mega_epic.t27` | `specs/mega_epic/mega_epic_domain.t27:14` | role is 1 character; slug not in `AN_SLUGS` |
| `0xQ@queen.t27` | `specs/queen/queen_domain.t27:14` | role is 1 character; slug not in `AN_SLUGS` |
| `0xS@skorobey.t27` | `specs/skorobey/skorobey_domain.t27:14` | role is 1 character; slug not in `AN_SLUGS` |
| `0xskorobey@army.t27` | `specs/skorobey/skorobey_army.t27:13` | role starts lower-case; slug not in `AN_SLUGS` |

`specs/domains/` holds two files (`domain-g.t27`, `domain-pr.paretO.t27`); it is not a registry of the 27
domains.

## Outside specs

- `.trinity/agent_events.jsonl`: 14 occurrences of `0xA@architecture.t27` as `canonical_id` in 18 lines
  (commits `cc937f9106`, `e841ec909d`). A tracked file still states the refused id as a fact.
- `AGENTS.md:707`: names the same id only to record that the 2026-11-13 note was false.

## Lines that use a valid role id outside the registry

| Id | File | Lines |
| --- | --- | --- |
| `0xCompiler@build.t27` | `specs/t27os/t27os_shell.t27` | 9 |
| `0xDoctor@operations.t27` | `specs/t27os/t27os_shell.t27` | 1 |
| `0xPlanner@product.t27` | `specs/t27os/t27l.t27` | 1 |
| `0xPlanner@product.t27` | `specs/tri/trigit/roadmap_dashboard_sync.t27` | 1 |

## Missing

- A gate that applies `an_id_ok` to every id constant under `specs` (the eight above are on master).
- A decision on the eight ids: rename each to a canonical id, or stop calling them canonical ids.
- A decision on `.trinity/agent_events.jsonl`: remove the 2026-11-13 lines or mark the file as not a journal.
