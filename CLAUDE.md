# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository: instructions for Claude Code and autonomous agents (t27).

Use this file **together with** `[AGENTS.md](AGENTS.md)`. Repo-specific law always overrides generic tooling defaults.

---

## Autonomous Execution Loop (AEL v2.0)

When operating as the Trinity Agent (Queen), follow this 6-phase loop:

```
┌─────────────────────────────────────────────────────────────┐
│  OBSERVE → PLAN → DELEGATE → VERIFY → SYNTHESIZE → LEARN   │
│         ↓       ↓        ↓        ↓         ↓         ↓    │
│  [E]     [T]     [C/V]    [V]      [L]      [L]           │
└─────────────────────────────────────────────────────────────┘
```

### Phase 1: OBSERVE
- Call Experience Agent (E) for context
- Read `.trinity/current-issue.md` for issue details
- Check ring and phase state from branch name
- Gather relevant files and context

### Phase 2: PLAN
- Break down task into subtasks
- Identify required skills: `/phi-loop`, `/tri-pipeline`, `/experience-save`
- Determine which agents to delegate to
- Estimate complexity and dependencies

### Phase 3: DELEGATE
- Delegate implementation to Creator Agent (C)
- Delegate validation to Verifier Agent (V)
- Coordinate parallel execution where possible
- Monitor agent progress

### Phase 4: VERIFY
- Review agent outputs
- Run conformance tests via `/tri-pipeline`
- Check L1-L7 law compliance
- Ensure quality standards

### Phase 5: SYNTHESIZE
- Combine agent results
- Resolve conflicts
- Create cohesive solution
- Prepare for integration

### Phase 6: LEARN
- Call Learner Agent (L) for pattern extraction
- Update `.trinity/experience.md` via `/experience-save`
- Save ring-specific learnings
- Improve future execution

---

## 1. Mandatory read order for this repository

1. `[AGENTS.md](AGENTS.md)` — entry point and constitutional stack.
2. `[SOUL.md](SOUL.md)` — canonical law (TDD, language, validation).
3. `[docs/T27-CONSTITUTION.md](docs/T27-CONSTITUTION.md)` — **SSOT-MATH**, **LANG-EN**, **DOCS-TREE**.
4. `[TASK.md](TASK.md)` and `[docs/coordination/TASK_PROTOCOL.md](docs/coordination/TASK_PROTOCOL.md)` — if the task touches coordination, locks, or shared hot paths.
5. Nearest `[OWNERS.md](OWNERS.md)` for the directories you edit.

Do **not** add parallel math/physics implementations in ad-hoc scripts when the same belongs in `*.t27` and the **`tri`** pipeline (`./scripts/tri`).

---

## 2. Engineering workflow

- **Bootstrap compiler:** from repo root, `cargo build --release -p t27c` (runs `build.rs` language checks). `bootstrap/` is a **workspace member**, so the binary lands at the workspace root: `./target/release/t27c` — *not* `./bootstrap/target/release/t27c`.
- **Iterating on the compiler:** do not rebuild release after every edit. [CONTRIBUTING.md, Build speed](CONTRIBUTING.md#build-speed-the-t27c-inner-loop) covers the faster builds, pointing `./scripts/tri` at a debug binary, and moving the `compiler.rs` seal.
- **Local sweep (CI-like):** from repo root, `./scripts/tri test` or `./target/release/t27c suite --repo-root .` (Rust runner; no shell test harness under `tests/`).
- **Generated code:** under `gen/` — do not hand-edit for routine fixes; change specs and regenerate.
- **Pull requests:** follow project Issue Gate and linking policy; **do not approve** PRs unless explicitly authorized.
- **Specs with `@extern` (T27 OS host files such as `specs/t27os/glass_window.t27`)** cannot be compiled by `t27c`; test them with `t27b test <spec>` and build the app with `make t27os` (an extern-capable `t27b`; a binary without `@extern` support emits trapping stubs). `tri land` refuses such a branch, so it lands as a plain PR.
- **Seals** come from a master-built `t27c` (`t27c seal <spec> --save`, then `tri seals sync-twins`); a seal saved from a worktree under `/Users/...` can embed the path and fail the `scan` check.
- **`gen/` is gitignored:** a generated file that a hand crate includes through `#[path = ".../gen/..."]` must be committed with `git add -f`, or master breaks after the squash (#11324 -> #11328).
- **Landing:** `tri land [--issue N]` is the GitHub leg for one branch (merge master, tests with no vacuous pass, seals, types ratchet, dupe subset, PR, auto-merge). `tri golden land | push | status` is our own git (`specs/queen/golden_chain.t27` section 6, `specs/tri/land/golden_land.t27`): small `claude/evolve-*` links are verified locally, land on `refs/golden/head` by compare-and-swap (a stale seal is re-saved as REGEN), and reach GitHub as one batch PR per 15-minute window.
- **Before the first push** check what CI will refuse: foreign-line budget (at most 40 added hand lines per non-t27 file, 80 per PR), home paths, stale seals, duplicate-body growth (`python3 tools/dupe_scan.py`), absurd Verilog widths. `--auto` merge can land commit 1 alone if you push twice.

---

## 3. PHI LOOP Execution

Follow the 9-phase PHI LOOP for ring-based development:

1. **Issue** - Define problem or requirement
2. **Spec** - Write .t27 specification
3. **TDD** - Write tests in spec before implementation
4. **Code/Impl** - Implement according to spec
5. **Gen** - Run `tri gen` to generate code from spec
6. **Seal** - Verify generated code and seal hash
7. **Verify** - Run `tri test` or conformance checks
8. **Land** - Merge changes to main branch
9. **Learn** - Capture learnings and update knowledge base

### Phase Completion Marker

When a phase is complete, include in your output:
```
Phase complete: [phase name]
→ Phase [next phase number]: [next phase name]
```

This triggers automatic branch creation for the next phase.

---

## 4. Autonomous subagent behavior (when spawned unattended)

- Finish the assigned task without waiting for clarification unless the repo's own rules require human input.
- If blocked after reasonable retries, stop and report what failed (logs, commands, file paths).
- Prefer small, reviewable diffs; match existing style and naming in touched files.
- **Output persistence:** when the parent workflow requires it, write the full final report to `/tmp/claude_code_output.md` (analysis, commands, diffs summary).

---

## Proactivity: request, do not ask

Owner rule, 2026-10-19: do not ask, request; everything not forbidden is allowed.

1. **Forbidden?** If the action is not on the list below, do it and report in one line afterwards.
2. **Blocked?** A missing fact, consent, number, credential or time slot is not a question for the owner. Name who holds it and send that party a request: what is needed, the options with their cost, the default taken at a stated time, and what happens meanwhile.
3. **Default.** Continue on the safe default; write it in the ledger. An unanswered request is chased once, then the default stands.

The owner is asked only for a password, a sign-in, a payment, a legal commitment or a choice between two businesses, once, as an exact value or click to provide.

Forbidden: typing or reusing a password, token or key from chat or a signed-in session; payment data, trades, transfers; minting or promising $TRI, or any value claim for XP; partner names, budgets or sentiment on public GitHub; secrets or PII on a machine that is not ours; using another party's machine without telling that party first; a speed or accuracy claim without a measurement and its command; a second spec where one exists; hand-written non-t27 logic over the caps; permanent deletion of anything that is not your own merged lane tree.

Every loop tick ends with `Requests out:` (who, what, sent when, default, due).

## 5. Skills and tooling

### Available Skills

- `/phi-loop` - Execute 9-phase PHI LOOP
- `/tri-pipeline` - Execute tri commands (gen, test, verify, seal)
- `/experience-save` - Save learnings to persistent memory
- `/self-cleanup` - Remove your own merged lane worktrees, review checkouts and caches; end of every tick and after every merge

Load these skills when their functionality matches the task.

### Model routing (owner directive 2026-10-20, replacing 2026-10-19)

Sub-tasks (implement, port, search, test, docs, fix) start in a **free T27 OS pane** (the key master's free
tier, visible in the window), go to a **Sonnet 5.5** subagent after 2 failed attempts and to **Opus 5.5**
after 4 or on the owner's word; the orchestrating session plans and reviews on Sonnet 5.5. The law is
`specs/claw/model_route.t27` (TIER_PANE, #11317); how to follow it is the `model-routing` skill, and a lane
as a pane is the `t27os-orchestra` skill.
`.claude/settings.json` sets `CLAUDE_CODE_SUBAGENT_MODEL` and each agent in `.claude/agents/` names its
model.

---

## FPGA hardware & flashing (SSOT)

**`[fpga/HARDWARE_SSOT.md](fpga/HARDWARE_SSOT.md)` is the single source of truth**
for the FPGA board, JTAG cable, host toolchain, and program/flash path. Read it
before touching anything under `fpga/`. Non-negotiables:

- **Measured 2026-10-08 (#7788): the bench board is an ALINX AX7203, `xc7a200tfbg484-2`** -- a
  camera read "ALINX" off the PCB and the AX7203 gf8 bitstream answered 512/512 over its UART.
  JTAG-only results stand; any design that uses package pins must target fbg484. The Wukong
  text below is what the SSOT said before; HARDWARE_SSOT.md §2026-10-08 has the evidence.
- **Bench on this Mac (2026-10-20):** the AX7203 answers at HS2 `--busdev-num 0:4` (IDCODE `0x3636093`, cable serial `210512180081`); a Zynq-7020 board (ARM `0x4ba00477` + PL `0x03727093`) answers at `0:2` on a Digilent Adept cable. The USB bus renumbers (it was `1:x`), so always run `openFPGALoader --detect` first. The Wukong text that used to stand here is history in HARDWARE_SSOT.md; pins and PnR follow the AX7203 (fbg484).
- **Flash with `openFPGALoader -c digilent_hs2`.** The attached cables are
  **Digilent FTDI `0x0403:0x6014`**, not the Xilinx `0x03FD` Platform Cable, and
  `cli/dlc10` speaks only to `0x03FD` — it answers `DLC10 cable not found` on this
  bench. Cables that share a serial cannot be separated by serial, only by `--busdev-num`.
  See `fpga/HARDWARE_SSOT.md` §"Note" and §"Programming".
  *Corrected W791 (T460/T461): the previous text here forbade `openFPGALoader` on
  the grounds that it cannot drive a `0x03FD` cable — true, and irrelevant, since
  no `0x03FD` cable is attached. That sentence contradicted the SSOT this file
  names as authoritative, and it was reported as a hardware blocker for thirteen
  waves. `cli/dlc10` remains correct for a DLC10 bench and is not removed.*
- No native macOS Vivado exists. Synthesis is Vivado-in-Docker or OpenXC7 only.
- If any other FPGA doc contradicts the SSOT, the SSOT wins — fix the other doc.

---

## 6. Security and secrets

- Never commit secrets. See `[SECURITY.md](SECURITY.md)`. Root `.env` patterns are gitignored; use `.env.example` patterns only in docs.

---

## Swarm, Queen and keys (state 2026-10-20)

- **Who does the work:** the bees (the Queen's lanes) and T27 OS panes on free tiers (z.ai `glm-4.5-flash`, OpenRouter `:free`) through the key master (`specs/t27os/keys.t27`); Claude sessions land, verify and unblock (`swarm-economy` skill). Remaining work is filed as 4-heading issues (Boundary / User Scenarios / Requirements / Success Criteria) under the T27 OS epics listed in AGENTS.md "Swarm focus".
- **The Queen picks by labels** (`specs/queen/priority.t27`): `P0` (at most 3 count as critical), `P1`, unlabelled last. Two open issues whose Boundary names the same file are run one after the other (whole-file holds), so give each issue its own files or a `path :: fn NAME` region.
- **Where the live Queen runs today:** Railway project `999`, service `trios-agent-server` (gHashTag/trios `trios/agent-server`, api.t27.ai). The region-hold code there is behind `TRIOS_QUEEN_REGIONS`, `TRIOS_QUEEN_DISPATCH_BATCH`, `TRIOS_QUEEN_DERIVED_FREE`. The project is leaving Railway: the dispatcher moves onto our own actors in the T27 OS kernel (`specs/t27os/kernel.t27`, `specs/queen/actors.t27`, epic #11377).
- **Contributed keys** (added by users on app.t27.ai) are stored sealed by the Queen; work on them is managed by the owner and the Queen with the contributor's consent, and the contributor earns PROVIDER_FEE in TRI credit for verified work (`specs/network/mvp.t27`, `specs/verified/payout.t27`; minting is the founder's alone, `specs/trinet/mint_authority.t27`). Never read, print or move a key value.
- **Numbers and the brand:** number formats (TNF, GoldenFloat, Golden Ruler, Z[phi]) and the brand visual (verified formulas orbiting the 27-petal logo) are sections of AGENTS.md; follow them, do not restate them.

## Sandbox and shell traps

- In a sandboxed agent, `gh` can fail with `x509: OSStatus -26276`, and `git worktree add`, branch deletion and `.git/config` writes can be refused; run those through a Terminal tab, or ask the orchestrator to fetch CI logs (`gh run view <id> --log-failed`) into a file for you.
- zsh: write `${b}` before a colon (`"+refs/heads/${b}:refs/heads/${b}"`), because `$b:r` is a modifier; never put backticks inside a double-quoted `--body` (they run), use `--body-file`; BSD `grep` has no `-P`.
- `Closes #N` in a commit or PR body closes N on merge: never name an epic that way (use `see #N`).

## The 7 Invariant Laws

| Law | Name | Description |
|------|------|-------------|
| L1 | TRACEABILITY | No code merged without `Closes #N` |
| L2 | GENERATION | Files under `gen/` are generated; edit specs instead |
| L3 | PURITY | Source files must be ASCII-only with English identifiers |
| L4 | TESTABILITY | Every `.t27` spec must contain `test`/`invariant`/`bench` |
| L5 | IDENTITY | φ² = φ + 1; φ² + φ⁻² = 3; IEEE f64 checks use tolerance |
| L6 | CEILING | `FORMAT-SPEC-001.json` + `gf16.t27` are numeric SSOT |
| L7 | UNITY | No new `*.sh` on critical path; use `tri`/`t27c` |

See [`docs/T27-CONSTITUTION.md`](docs/T27-CONSTITUTION.md#2--invariant-laws-never-change-without-constitutional-amendment) for full details.

---

**Repository:** Trinity S³AI — **t27** (spec-first ternary / TRI-27). **φ² + 1/φ² = 3 | TRINITY**

## T27F is t27's frontend

T27F is t27's native visualization, made through several tools: the spec diagram (its core, the
visual spec of `.t27`, gHashTag/trinity#1648, RFC gHashTag/trinity#1649), map stills, the graph
page, particle reels and stills, the hot-reload stream and the `claw/t27f` check-run. TRI CLAW
serves its MCP tools. Its surfaces, their status and which one answers an intent are the law in
`specs/t27f/frontend.t27`. It is **not** the topic-to-value engine: gHashTag/trinity#1677 is an
application drawn through T27F. Owner corrections, 2026-10-17 and 2026-10-18.

This is a pointer. The definition, and what exists versus what is still missing, live in one
place: `.claude/skills/t27f/SKILL.md`, section "What T27F is".

## Own language first

When this project publishes something about itself, it publishes in **this
project's own language and format** -- not translated into somebody else's.

Owner's rule, 2026-09-20: stop writing in other people's languages, we have our
own.

This bites on any file whose only reason to exist is that an outside tool
expects that shape: `llms.txt`, `agents.json`, `ai.txt`, `.well-known/*.json`,
A2A agent cards, `ai-plugin` manifests, OpenAPI stubs, JSON-LD blocks, a README
that restates a spec. The reflex is to write four of them in four foreign
formats, and the reflex is wrong: a project whose claim is "here is a language
worth writing" and which then describes itself in three of other people's
formats has published three documents that are not true of it.

**The move:** find the address the outside world already fetches, then serve our
own language at it. `/llms.txt` at t27.ai **is** a t27 module -- `llms.txt`
requires nothing but text, and every prose line of a `.t27` file is a `;`
comment, so it stays readable to anything that cannot compile it.

**Three qualifications, so the rule stays honest:**

- A format a resolver genuinely parses -- a sitemap, `package.json`, a lockfile
  -- is machinery, not a description. **Generate** it from our own source; never
  hand-write it into a second home for the truth.
- Code against someone else's API uses their types. Prose for a human who has
  never heard of the project uses that human's language.
- If a format demands a claim we cannot back, **publish nothing**. An A2A card
  with no A2A server behind it is a false claim, and a missing file is more
  honest than a lying one.

The test: *is this file the project speaking about itself?* If yes, it speaks
our language. If it is plumbing, it speaks the plumbing's.

**Worked example, compiler-checked rather than asserted:** in `gHashTag/trinity`,
`apps/website/public/t27/files/specs/catalog/onboarding.t27` generates
`/llms.txt` and `/agents.t27` byte-identically, gated in CI as
`check:onboarding`. The generator evaluates the spec's own `test` blocks --
`typecheck.ok` stays true for `assert 1 > 2`, so a compiler saying "this parses"
is not a compiler saying "this is true" -- and re-compiles the rendered document
before writing it.

**The full rule lives in exactly one place: the `own-language-first` skill**
(`~/.claude/skills/own-language-first/SKILL.md`). It carries the consent gate for
documents addressed to other people's agents, the six negative controls, and the
`;`-alone-on-a-line trap that silently discards a `module` declaration. This
section is a pointer, not a copy -- the recorded defect in this codebase family
is the hand-copied rule that only two of its three homes knew about.
