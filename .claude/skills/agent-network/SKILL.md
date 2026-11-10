---
id: agent-network
name: agent-network
description: Agent network v1. One spec has one owner, one agent has one domain, and a capability use is audited. Load before naming, registering, or classifying an agent command.
argument-hint: "[status|register|classify]"
---

# /agent-network -- one owner, one domain, an audit

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

The checked law is `specs/t27os/agent_network.t27`. Grants go through `specs/t27os/orchestrator.t27`. If this page and those specs disagree, the specs win.

The letter cards in `specs/agents/` stay the path owners. This layer does not replace them (`AN_REPLACES_LETTER_CARDS` is 0).

## Id

- Display form: `0x<ROLE>@<domain>.t27`.
- Checked form: `an_pack(role, domain)`.
- Roles in this layer: scout, orchestrator, worker.
- A domain is one of the 27 network domains. T27A registers. T27L audits. T27# is the kernel.

## Register

- A spec has exactly one owner.
- A domain may hold many agents.
- An empty owner count does not register.

## Capability

- A use is real only when the orchestrator allows the act and an audit bit is written.
- Merge, publish, cron, a lab run, a Mac test-report, and the window shell stay gated.
- `tri t27a spec scan` is classified to T27A and does not exist as a command.
- No command body moves in this layer (`AN_MIGRATED` is 0).

`an_window_stays_blocked` stays true. This skill does not open the window.
