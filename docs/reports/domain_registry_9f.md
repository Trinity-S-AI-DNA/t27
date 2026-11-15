# Domain registry study: naming V3 in specs/queen/agent_registry.t27

Read-only extraction at revision `ed38624709` (2026-11-15). Every row cites `file:line` at that revision.
This file is a dated reading, not a rule: the registry stays the only home of the names. A verdict marked
"by reading" was read from the rule, not produced by a run.

## Result

```
REGISTRY_PATH: specs/queen/agent_registry.t27 (1857 lines, module QueenAgentRegistry, L36)
DOMAIN_COUNT: 27 (AN_DOMAINS, L957; AN_CODES, L962; AN_SLUGS, L966)
SECTION_9F_ENTRY_COUNT: 106 role ids on 27 domain lines (AN_ROLE_COUNT, L1591; AN_ROLE_IDS, L1593-1621)
DOMAIN_FIELDS_PRESENT: code, slug, domain number (index + 1, L961)
DOMAIN_FIELDS_ABSENT: purpose, layer, register
```

## Where the facts sit

| Fact | Line |
| --- | --- |
| Section 9 heading, naming convention V3 | L938 |
| "nothing renames an agent and nothing grants a capability" | L940 |
| `AN_DOMAINS = 27`, `AN_ORPHAN = 0`, `AN_D_KERNEL = 27` | L957-959 |
| "T27# is the cross-cutting kernel, not a 28th domain" | L961 |
| `AN_CODES : [27]str` | L962 |
| `AN_SLUGS : [27]str`, "The owner fixed all 27 of them on 2026-11-09" | L964-966 |
| `an_role_ok`: CamelCase role, 3..24 characters | L978-990 |
| `an_id_ok`: `0x<ROLE>@<slug>.t27`, slug from `AN_SLUGS` | L1008-1017 |
| Heading "9f. SPEC_SCAN_FIX_PROTOCOL" | L1296 |
| Heading "9g. tests" | L1340 |
| Heading "9f. Role table (naming V3)" | L1588 |
| "Every role below is DEFINED only: there is no live worker for it and no capability grant" | L1589 |
| `AN_ROLE_DOM : [106]u8` | L1624-1632 |
| Heading "9g. The owner's three named spec owners" | L1681 |
| Heading "9h. Pilot tree" | L1703 |

Two findings about the section labels:

- The file has two headings labelled 9f (L1296, L1588) and two labelled 9g (L1340, L1681). "Section 9f" is
  ambiguous until one pair is renumbered.
- The domain codes and slugs are in the section 9 preamble (L957-966), not in 9f. The second 9f (L1588) holds
  the role ids. `AGENTS.md:710` points at "section 9f" for the domains.

## DOMAINS

Code from `AN_CODES` (L962), slug from `AN_SLUGS` (L966), number = index + 1 (L961). The registry stores no
purpose, layer or register for a domain; the slug is the only word it gives. "Role line" is the line of
`AN_ROLE_IDS` that lists the domain's role ids.

| N | CODE | SLUG | PURPOSE | LAYER | REGISTER | Role ids | Role line |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | T27A | agents | absent | absent | absent | 4 | L1594 |
| 2 | T27B | build | absent | absent | absent | 4 | L1595 |
| 3 | T27C | contracts | absent | absent | absent | 4 | L1596 |
| 4 | T27D | design | absent | absent | absent | 3 | L1597 |
| 5 | T27E | economics | absent | absent | absent | 3 | L1598 |
| 6 | T27F | files | absent | absent | absent | 4 | L1599 |
| 7 | T27G | governance | absent | absent | absent | 4 | L1600 |
| 8 | T27H | health | absent | absent | absent | 4 | L1601 |
| 9 | T27I | integration | absent | absent | absent | 4 | L1602 |
| 10 | T27J | jobs | absent | absent | absent | 4 | L1603 |
| 11 | T27K | memory | absent | absent | absent | 4 | L1604 |
| 12 | T27L | telemetry | absent | absent | absent | 4 | L1605 |
| 13 | T27M | actors | absent | absent | absent | 4 | L1606 |
| 14 | T27N | network | absent | absent | absent | 4 | L1607 |
| 15 | T27O | operations | absent | absent | absent | 4 | L1608 |
| 16 | T27P | product | absent | absent | absent | 4 | L1609 |
| 17 | T27Q | quality | absent | absent | absent | 4 | L1610 |
| 18 | T27R | research | absent | absent | absent | 4 | L1611 |
| 19 | T27S | security | absent | absent | absent | 4 | L1612 |
| 20 | T27T | time | absent | absent | absent | 4 | L1613 |
| 21 | T27U | ux | absent | absent | absent | 4 | L1614 |
| 22 | T27V | visual | absent | absent | absent | 4 | L1615 |
| 23 | T27W | workflow | absent | absent | absent | 4 | L1616 |
| 24 | T27X | extensions | absent | absent | absent | 4 | L1617 |
| 25 | T27Y | delivery | absent | absent | absent | 4 | L1618 |
| 26 | T27Z | zero-trust | absent | absent | absent | 4 | L1619 |
| 27 | T27# | kernel | absent | absent | absent | 4 | L1620 |

Sum of role ids: 25 x 4 + 2 x 3 = 106, equal to `AN_ROLE_COUNT` (L1591).

## Agents are a different object

`specs/agents/` holds 27 agent specs (`a.t27` .. `z.t27`, `ti.t27`) plus `README.md`. LAYER and REGISTER are
fields of an agent, not of a domain. In every file: `KIND = "agent"` L12, `ID` L13, `NAME` L18, `DOMAIN` L19,
`REGISTER` L22, `LAYER` L24.

| File | ID | NAME | DOMAIN (agent field) | REGISTER | LAYER |
| --- | --- | --- | --- | --- | --- |
| a.t27 | t27/A | Alpha (Architecture) | Architecture / ADR / SOUL | R0 | Archetypal |
| b.t27 | t27/B | Beta (Build) | Build / Pipeline | R1 | Archetypal |
| c.t27 | t27/C | Gamma (Compiler Core) | Compiler Core / AR Language | R2 | Archetypal |
| d.t27 | t27/D | Delta (De-Zigfication) | De-Zigfication / Migration | R3 | Archetypal |
| e.t27 | t27/E | Epsilon (Experience) | Experience / XAI | R4 | Archetypal |
| f.t27 | t27/F | Phi (Formal Conformance) | Formal Conformance | R5 | Archetypal |
| g.t27 | t27/G | Gamma (Graph) | Graph / ArchBench | R6 | Archetypal |
| h.t27 | t27/H | Eta (HSLM) | HSLM / NN Architectures | R7 | Archetypal |
| i.t27 | t27/I | Iota (ISA) | ISA / Registers | R8 | Archetypal |
| j.t27 | t27/J | Iota-extended (Jobs) | Jobs / Task Routing | R9 | Spiritual |
| k.t27 | t27/K | Kappa (Kernel) | Kernel / FPGA MAC | R10 | Spiritual |
| l.t27 | t27/L | Lambda (Language) | Language / Syntax vNEXT | R11 | Spiritual |
| m.t27 | t27/M | Mu (Metrics) | Metrics / Telemetry | R12 | Spiritual |
| n.t27 | t27/N | Nu (Numeric) | Numeric / GoldenFloat | R13 | Spiritual |
| o.t27 | t27/O | Omicron (Orchestration) | Orchestration / Phases | R14 | Spiritual |
| p.t27 | t27/P | Pi (Physics) | Physics / SacredPhysics | R15 | Spiritual |
| q.t27 | t27/Q | Theta (Queue) | Queue / Scheduling | R16 | Spiritual |
| r.t27 | t27/R | Rho (Runtime) | Runtime / Bootstrap | R17 | Spiritual |
| s.t27 | t27/S | Sigma (Specs) | Specs / Standardization / AR Specs | R18 | Physical |
| t.t27 | t27/T | Tau (TRINITY Queen) | Queen / Lotus Orchestrator | R19 | Physical |
| u.t27 | t27/U | Upsilon (Universe) | Universe Levels / Domains | R20 | Physical |
| v.t27 | t27/V | Vau (Verdict) | Verdict / Bench / AR Validation | R21 | Physical |
| w.t27 | t27/W | Double-Vav (Workflow) | Workflow / tri cell | R22 | Physical |
| x.t27 | t27/X | Chi (External) | External Bindings / Interop | R23 | Physical |
| y.t27 | t27/Y | Psi (Yield/DePIN) | Yield / DePIN / Fitness | R24 | Physical |
| z.t27 | t27/Z | Zeta (Zero-Touch) | Zero-Touch UX / Docs | R25 | Physical |
| ti.t27 | t27/TI | Ti (Security) | Security / Reserve | R26 | Physical |

Layers: 9 Archetypal (R0-R8), 9 Spiritual (R9-R17), 9 Physical (R18-R26). The word "Spatial" has 0 hits under
`specs/agents`.

## Letter alignment (open, not settled here)

- `an_letter_status` (L1279-1283) answers SAME for B, J, Y and CONFLICT for every other letter.
- The comment at L943-944 compares the domain letters with the meanings doctor, frontend, compiler and
  Ouroboros. Those are not the `DOMAIN` values of `specs/agents/*.t27` (D is "De-Zigfication / Migration",
  F is "Formal Conformance", O is "Orchestration / Phases"). The registry does not name the source of the
  meanings it compares.
- Read against the agent files, two letters differ from the table: agent W is "Workflow / tri cell" and T27W
  is `workflow` (the table says CONFLICT); agent Y is "Yield / DePIN / Fitness" and T27Y is `delivery` (the
  table says SAME). By reading only; nothing in the tree settles it.

## Tests

| Test | State | Evidence |
| --- | --- | --- |
| `verify_domain_letter_alignment` | not_implemented | 0 hits under `specs` and `docs` |
| nearest existing: `an_domains_and_slugs` | exists, not run here | L1351 (lengths, first and last code, slug lookups) |
| nearest existing: `an_aliases_and_letters` | exists, not run here | L1528 |

## Missing

- A purpose, layer and register per domain (no such field).
- One unambiguous label for the role table section (two 9f, two 9g).
- A named source for the legacy letter meanings that `an_letter_status` compares.
- `verify_domain_letter_alignment`.
