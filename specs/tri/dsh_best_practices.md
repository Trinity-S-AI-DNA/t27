# DSH best-practice adoption pipeline, tested against the Phase 0 pin

Lane: Trinity-S-AI-DNA/t27#12952, epic #12945 (T27 M, the Golden Builder), after the Phase 0 baseline of #12948.
Date of every run: 2026-10-31. Machine of every run: Apple M1 Pro 16 GB, Darwin 25.5.0, Node v22.22.0.

The goal is not to copy DeepSeek Harness (dsh). It is to read the official documents and source first, extract the practices
worth testing, put each on a matryoshka layer, test it against the fixed baseline and adopt only what was shown to hold.
The gate and the fifteen records are a spec with tests, `specs/tri/dsh_practice_adoption_plan.t27`; this file carries the
prose, the sources, the commands and the receipts, and the two are checked against each other by the counting commands at
the end.

Result: **15 practices evaluated, 9 adopted, 2 rejected, 4 deferred.** Only 2 of the 9 adoptions carry a measured saving
(P4, P9); the other 7 are adopted because they keep the baseline and have evidence, and their saving reads "not measured".

## 1. The pin and the sandbox

| Item | Value |
|---|---|
| dsh | clone of `deepseek-ai/deepseek-harness` at commit `ce7073d512` = `dsh-v0.2.2-alpha.2-351` (2026-10-30) |
| Profile | headless, approval policy `never`, no live instance touched |
| Sandbox mode | **read-only** for every run (`DSH_PERMISSION_MODE=read-only`); the `sandbox/mode` event of 7 of 7 new logs reads `read-only` |
| Telemetry | `DSH_TELEMETRY_MODE=DISABLED` on R8 (the adopted settings); the default is `FEEDBACK_ONLY` (see section 8) |
| Launch | `DSH_PERMISSION_MODE=read-only tsx --tsconfig <clone>/tsconfig.json <clone>/apps/cli/src/bin.ts --profile headless --patch <overlay.yml> [--patch <layer2.yml>] --approval-policy never --json - < task.txt`, under `timeout 240` |
| Provider keys | read from the key master's environment by variable name only; no value is stored, printed or pasted anywhere |
| Load guards | `uptime` load about 12.8 (limit 40), 8 GB free (limit 4 GB), one dsh run at a time |
| Upstream edits | none: no commit in the clone, none on `gHashTag/deepseek-harness` |

The upstream `SAFETY.md` was reread before any practice that touches permissions or tools (P1, P2, P5). Nothing was run
that widens the tool set: the plugin manager row stayed disabled, the schedule tool was not exposed, no plugin was installed.

## 2. Sources read, in the order the issue sets

| # | Source | Resolved | What was read |
|---|---|---|---|
| 1 | https://deepseek-harness.github.io/deepseek-harness/ | yes, HTTP 200, 57575 bytes | the fetch tool returned only the page title ("DeepSeek Harness"): the site is rendered by script and no section names, version or page list came through. **Unresolved as a source of content.** The same documents were read from the clone at the pin (`docs/`, next row) |
| 2 | https://github.com/deepseek-ai/deepseek-harness | yes, HTTP 200 | public repository, default branch `master`, no GitHub release listed; top level `docs/`, `apps/`, `packages/`, `benchmarks/`, `SAFETY.md`, `AGENTS.md`. The 14 official documents cited in section 7 were read in the clone at `ce7073d512`; every cited path exists at the pin |
| 3 | https://www.deepseek.com/en/harness/ | yes, HTTP 200 | product page: public preview, "everything is a plugin" design, plugins installable or created in Creator Mode, official plugins (agent teams, auto approval reviewer, scheduled tasks, voice input) all marked experimental, named sessions, a scheduled-report demo, a trajectory view for tool calls; no version number, no eval claim, no safety claim |
| 4 | practitioner posts | one candidate | arXiv 2607.28802, "Model or Harness? An Interaction-Centric Taxonomy for Localizing Agent Failures" (submitted 2026-07-30). Used as a candidate only (P15), `source_type` community |

No source was found that failed to resolve, but row 1 gave no content, so no claim in this file rests on it.

## 3. Layer map (FR-003)

| Layer | Role | Practices |
|---|---|---|
| L0 | owner approval, the permission gate | P2 |
| L1 | Queen / orchestrator: the orchestrator agent and the budget policy | P6, P11, P12 |
| L2 | planner and reviewer | P8, P10, P15 |
| L3 | worker / coder | P13 |
| L4 | extractor / formatter, the cheap-model skill | P14 |
| L5 | deterministic tools: tests, parsers, CI, seals, the ledger | P7, P9 |
| cross-cutting | trace, session log, scheduled audit, plugin manifest | P1, P3, P4, P5 |

The layer of each practice is my assignment, by where its implementation would live on the T27 side. It is not stated by dsh.

## 4. The adoption gate (FR-004)

A practice is adopted only when all four hold. The gate is `verdict()` in the spec; the recorded status of every record must equal it.

| Condition | Meaning | Field in the spec |
|---|---|---|
| A | applies to the pinned version and to the profile a lane can run | `applicable` |
| B | does not break the Phase 0 baseline: the cold-start row and the task rows rerun and compared, or the practice changes no run (trace analysis) | `baseline` (B_KEPT / B_UNTESTED / B_BROKEN) |
| C | has a test, a receipt or a measured improvement | `has_evidence` |
| D | needs no change to the upstream fork | `fork_free` |

Rejected when A fails, B is broken or D fails (the pin cannot take it). Deferred when B is untested or C fails (it may be right,
it is not shown). Adopted when all four hold. The spec tests include negative controls for every condition.

## 5. Summary

| P | Practice | Layer | A | B | C | D | Decision |
|---|---|---|---|---|---|---|---|
| 1 | plugin manifest, permissions, versioning, rollback | cross | yes | untested | no | yes | deferred |
| 2 | Creator Mode safety rails | L0 | no | untested | yes | yes | rejected |
| 3 | trace capture for every model and tool call | cross | yes | kept | yes | yes | adopted |
| 4 | append-only session logs with resume, fork, replay | cross | yes | kept | yes | yes | adopted (resume only) |
| 5 | scheduled-task contract | cross | no | untested | yes | yes | rejected |
| 6 | layered configuration | L1 | yes | kept | yes | yes | adopted |
| 7 | pinned version, model id, prompt SHA, tool version | L5 | yes | kept | yes | yes | adopted |
| 8 | prompts as versioned code | L2 | yes | untested | yes | yes | deferred |
| 9 | fast eval gate per tick, deep eval before a release | L5 | yes | kept | yes | yes | adopted |
| 10 | failure-trace review | L2 | yes | kept | yes | yes | adopted |
| 11 | token, cost, latency accounting per step | L1 | yes | kept | yes | yes | adopted |
| 12 | model routing with escalation | L1 | yes | kept | no | yes | deferred |
| 13 | context compaction without loss | L3 | yes | untested | yes | yes | deferred |
| 14 | provider fallback with provenance | L4 | yes | kept | yes | yes | adopted (provenance only) |
| 15 | structured failure taxonomy | L2 | yes | kept | yes | yes | adopted |

## 6. Run receipts

Every row: dsh `ce7073d512` (`dsh-v0.2.2-alpha.2-351`), Node v22.22.0, Apple M1 Pro 16 GB, Darwin 25.5.0, read-only sandbox,
approval never, 2026-10-31. Wall time includes boot. Tokens are the sum over steps of the `usage` of each assistant message:
inputTokens (cache reads excluded) / outputTokens. The Phase 0 column is the same task in #12948.

The task prompts are reworded copies (the Phase 0 originals are not in this worktree), so a time against Phase 0 is a
comparison of the same kind of task, not of the same bytes. Task 1: "Read the file specs/t27f/frontend.t27 ... summarize it in at
most 120 words: name its sections and list the names of its test blocks." Task 4b: "List by name every tool you have from the t27
MCP server ... end with the line Total count: N." Task 5b: "Use the t27 MCP status tool once and report the t27 source commit."

| Run | Command (shape) | Wall | Tokens in / out | Result | Phase 0 |
|---|---|---|---|---|---|
| R1 boot proxy | `dsh --dump-config` with the overlay | 0.41 s, rc 0 | none | merged config, 435 lines | HTTP cold start (200 rows) not rerun |
| R2 spec check | `t27c check <spec>`, 3 runs | 0.016 to 0.017 s | none | typecheck ok | none |
| R3 5b control | overlay only, asks glm-5.3-flash | 24 s | 34313 / 155 | correct, commit `1d9c5960d357` | 13.6 s, 23197 / 82 |
| R4 5b layered | overlay + routing layer (glm-4.5-flash) | 18 s | 24580 / 190 | correct | none |
| R5 4b control | overlay only | 14 s | 6407 / 326 | **incomplete**: 0 tool calls, one sentence, rc 0 | 27.0 s, 14245 / 804, 27 names |
| R6 4b layered | overlay + routing layer | 60 s | 69479 / 685 | correct, 27 names and "Total count: 27"; it called all 27 MCP tools, 14 results ok, 13 isError (all 7 `tg_*` tools errored); no approval event | none |
| R7 1 control | overlay only | 62 s | 11879 / 342 | names correct; the summary text said 8 test blocks, the spec has 10 | 34.7 s, 11051 / 377 |
| R8 1 adopted settings | overlay + routing layer + telemetry DISABLED | 28 s | 12440 / 386 | 10 names, 0 stderr bytes | none |
| R9 window | routing layer + 20000 token window, `thresholdRatio` 0.5, 2-tool task | 142 s, 5 steps | not summed | correct; 4 tool calls for 2 needed; compaction events: 8 start, 6 summary, 8 end, 4 prune | none |
| R10 resume | `--session-id` of the R3 session, no tool allowed | 8 s | not summed | recalled `1d9c5960`; log grew 27 to 37 lines, first 27 lines byte-identical (`cmp`) | none |

The layered rows 4b, 5b and 1 (R6, R4, R8) sum to 106 s (60 + 18 + 28). Reliability: 7 new session logs (R3 to R9) plus the resume, 2 of the 7 with a bad
outcome that only the trace shows (R5 incomplete, R6 unsafe tool use); each row is one attempt, so there is no retry count to
report. Quality is the row "Result", judged by reading the log, not a score.

## 7. Records (FR-001)

Each record: practice, source (all official documents are under `https://github.com/deepseek-ai/deepseek-harness/blob/master/`,
read at `ce7073d512`), source_type, applicable, adoption_status, reason, expected_benefit, measured_benefit, risk, test,
and for adopted ones the chain "dsh practice -> matryoshka implementation -> measured saving" (FR-005).

### P1. Plugin manifest, permissions, versioning and rollback

- Source: `docs/user/guide/plugin-market.md`, official. Layer: cross-cutting. Applicable: yes (the document exists at the pin).
- Decision: **deferred**. B untested, C no.
- Reason: a market install writes to the shared dsh state under the home folder, which this lane may not touch, and the headless
  profile has no plugin manager. Versions and rollback (`engines.dsh`) exist in the documents and were read, not run.
- Expected benefit: a plugin declares its permissions and its dsh range, so it can be pinned and rolled back.
- Measured benefit: not measured. Risk: an install writes shared state and a plugin widens the tool set.
- Test: none run; reopen when a dsh run can use a scratch state folder.

### P2. Creator Mode safety rails

- Source: `.agents/notes/implemented/architecture/2026-09-16-creator-persistent-plugin-management.md`, official. Layer: L0.
- Decision: **rejected**. A fails.
- Reason: at the pin the `tool-plugin-manager` row is disabled in the headless profile: 0 of 54 exposed tools are `plugin_manager`
  in 7 of 7 logs. Turning the row on widens the tool set, which FR-006 forbids without a test. The product page itself says
  the plugin APIs are still evolving.
- Expected benefit: an approval gate in front of every plugin change. Measured: 0 `plugin_manager` among 54 tools in 7 of 7
  logs (`grep -c plugin_manager` on each log). Risk: enabling the row exposes plugin install and edit to a model.
- Implementation: none; L0 stays the owner approval gate. Test: static, the tool list in `request/header` and `--dump-config`.

### P3. Trace capture for every model and tool call

- Source: `docs/subsystems/otel.md`, official. Layer: cross-cutting. Decision: **adopted**.
- Reason: the local session log already records every model step and tool call. In the Phase 0 logs 49 of 49 tool calls have a
  result; 3 of 59 steps are open, and those are the 3 runs killed by the timeout. In the 7 new logs step start equals step end
  and every call is matched by a result. The default export (`FEEDBACK_ONLY`) targets a third-party collector, so lanes set
  `DSH_TELEMETRY_MODE=DISABLED`: row 1 rerun with it was correct, 28 s, 0 stderr bytes (R8).
- Chain: dsh session log -> lane receipts read from the local log, telemetry off -> saving not measured.
- Measured benefit: 7 of 7 logs complete (`zstd -dcq LOG | grep -c` by event type). Risk: the log holds tool results, so it is never
  pasted into a public place. Test: event counts per log, then R8.

### P4. Append-only session logs with resume, fork and replay

- Source: `docs/session-format-status.md`, official. Layer: cross-cutting. Decision: **adopted, resume only**.
- Reason: R10 re-opened the R3 session by `--session-id` in 8 s, the log grew from 27 to 37 lines, the first 27 lines are
  byte-identical and the model recalled the commit it had reported. Fork and replay were not exercised and are outside the
  adopted scope.
- Chain: dsh append-only session log -> a follow-up question to a finished lane goes to its session id, not to a fresh run
  -> **8 s against 24 s** for the fresh run that found the same fact (one pair, one run each: a pair, not a distribution).
- Risk: a resumed session carries its old context and its old mistakes.
  Test: `dsh --session-id session-<uuid>`, then `cmp` of the first 27 lines of the log.

### P5. Scheduled-task contract with explicit schedule and expiry

- Source: `docs/user/guide/schedule.md`, official. Layer: cross-cutting. Decision: **rejected**. A fails.
- Reason: the profile a lane can run exposes no schedule tool (0 of 54), and a schedule needs the server profile and writes shared
  state. The contract itself (explicit schedule, explicit expiry) is kept on the T27 side where the scheduler is. Reopen when a
  profile with the tool can run on scratch state.
- Expected benefit: a recurring job states when it runs and when it stops. Measured: 0 schedule tools among 54 in 7 of 7 logs.
  Risk: an unattended schedule keeps spending after its purpose is over. Test: static, the tool list of `request/header`.

### P6. Layered configuration

- Source: `docs/user/develop/basic/config.md`, official. Layer: L1. Decision: **adopted**.
- Reason: measured with `--dump-config`. A flat-key patch on an existing row changes only that key (provider and model: 2 changed
  keys). A patch of `providers` **replaces the whole map** (24 lines vanished) and must restate `apiKeyEnv`, a name only. Rows 1,
  4b and 5b rerun with the routing layer stay correct (R4, R6, R8). Rule: layer 2 carries flat routing keys only and never
  restates providers.
- Chain: dsh layered config (bundle, profile, patches) -> layer 1 is the Phase 0 overlay, layer 2 a routing-only patch, both
  repeatable `--patch` flags -> saving not measured.
- Risk: restating providers silently drops the local model row. Test: `--dump-config` before and after each layer, then the three rows.

### P7. Pinned dsh version, model id, prompt SHA and tool version

- Source: `docs/cli-help.md`, official. Layer: L5. Decision: **adopted**.
- Reason: dsh records none of version, prompt SHA or tool version in the log (0 hits), so the receipts derive them: the clone
  commit and tag, the requested model id from `request/header`, and the prompt SHA from the `system/message` text. The SHA was
  stable per model name: `49193094cc5e` in 4 of 4 runs (glm-5.3-flash), `0fa5ab930ff6` in 3 of 3 (glm-4.5-flash).
- Chain: dsh version flags and request header -> every receipt row names the pin, the requested and served model and the prompt SHA
  -> saving not measured.
- Command: `zstd -dcq LOG | grep '"type":"system/message"' | grep -o '"text":".*"}\],"source"' | shasum -a 256`.
  Risk: the SHA covers the system text only, not the tool schemas.

### P8. Prompts as versioned code with owner, changelog and review

- Source: `docs/subsystems/system-prompt.md`, official. Layer: L2. Decision: **deferred**. B untested.
- Reason: the dsh system prompt is built per model (two SHAs for two model names) and is not a file the T27 side owns, so owner,
  changelog and review have nothing to attach to at the pin. Only the SHA stability was measured; no run used an owner-reviewed
  prompt file.
- Expected benefit: a prompt change is a reviewed change with a recorded cause. Measured: not measured.
  Risk: an unreviewed prompt change moves every result. Test: SHA stability only (P7).

### P9. A fast eval gate per tick and a slower deep eval before a release

- Source: `BENCHMARK.md`, official. Layer: L5. Decision: **adopted**.
- Reason: two tiers were timed. The fast gate is the merged config dump (R1, 0.41 s) plus the spec check (R2, 0.02 s). The deep gate
  is the three task rows rerun (106 s). The rows are not cheap enough to run every tick, the check is.
- Chain: dsh benchmark with fast and deep suites -> per tick `dsh --dump-config` and `t27c check`, before a release the three task
  rows with the receipts of section 6 -> **0.43 s per tick against 106 s for the deep rerun**, a gate 250 times cheaper that
  catches a broken file and cannot see a wrong answer (106 / 0.43 is about 250).
- Risk: the fast gate cannot see a wrong answer, only a broken file.
  Test: wall time of `--dump-config` and of `t27c check`, and of the three layered rows.

### P10. Failure-trace review instead of aggregate scores alone

- Source: `docs/postmortem/README.md`, official. Layer: L2. Decision: **adopted**.
- Reason: 2 of the 7 new runs exit 0 with a bad outcome that only the trace shows: R5 stopped after one sentence with no tool call, and
  R6 called all 27 MCP tools including 7 Telegram tools (all errored) with no approval event. An exit-code or pass-rate gate passes
  both. In Phase 0 the exit code was 0 on 6 of 9 runs and the answer correct on 4 of 9.
- Chain: dsh postmortem practice -> a lane report reads the trace of every failing or odd run before it reports a time ->
  saving not measured.
- Risk: reading traces costs reviewer time and holds tool results. Test: event counts per log and the rc of each run.

### P11. Token, cost and latency accounting per agent step

- Source: `docs/subsystems/token-meter.md`, official. Layer: L1. Decision: **adopted**.
- Reason: usage (input, output, cache read tokens) is on 16 of 16 assistant messages in the 7 new logs and every event has a time.
  Money cost is **not measured**: the ladder carries no price field.
- Chain: dsh token meter -> the receipts table sums inputTokens and outputTokens per run, wall time includes boot -> saving not measured.
- Risk: inputTokens excludes cache reads, so two runs are compared only by the same rule.
  Test: `grep -c` of `inputTokens` on `assistant/message`, per log.

### P12. Model routing with explicit escalation

- Source: `docs/subsystems/subagent.md`, official. Layer: L1. Decision: **deferred**. C fails.
- Reason: the routing layer works (P6) but there is nothing to measure: 4 of 4 runs that asked for glm-5.3-flash were served by
  glm-4.5-flash, so the cheap rung and the default are the same model and the difference between R3 and R4 is run-to-run noise.
  Escalation after a failure was not exercised.
- Expected benefit: a cheap rung does the bounded steps and a stronger one is called only after a failure. Measured: not measured.
  Risk: an escalation rule that never fires costs nothing and proves nothing.
  Test: the same task on two rungs with different served models (`responseModel`).

### P13. Context compaction without loss of task-critical facts

- Source: `docs/subsystems/compaction.md`, official. Layer: L3. Decision: **deferred**. B untested.
- Reason: with a 20000 token window compaction fired (R9: 8 starts, 6 summaries, 4 prunes of tool results) and the final answer was
  correct, but the summary lost the tool results and the model called both tools again: **4 calls for a 2 call task, 142 s**.
  A prune shadows the original result (it stays in the log, `shadowedSeqs`) and writes a replacement. The declared window of the
  ladder is 1000000, so compaction would not fire before a provider overflow. Phase 0 rows were not rerun under a compaction policy.
- Expected benefit: a long lane keeps its goal and its facts after the window is cut. Measured: 4 for 2 calls, correct.
  Risk: a lossy summary is repaired by extra tool calls, which costs time and tokens.
  Test: the window layer, `contextWindow` 20000 and `thresholdRatio` 0.5, on a two-tool task.

### P14. Provider fallback with provenance

- Source: `docs/subsystems/llm-streaming.md`, official. Layer: L4. Decision: **adopted, provenance only**.
- Reason: provenance is native. `request/header` holds the requested provider and model, and `replayState.response.responseModel`
  holds the served model only when it differs. 4 of 4 runs asking for glm-5.3-flash were served by glm-4.5-flash, 0 of 3 runs
  asking for glm-4.5-flash show a different model, and 10 of 10 Phase 0 sessions that asked for glm-5.3-flash were served by another model
  (glm-4.5-flash in 9, glm-4.7-flash in 1). The fallback itself is the ladder of the T27 side; dsh only retries per route. No fork change.
- Chain: dsh provenance fields -> every receipt row carries requested and served model -> saving not measured.
- Risk: `responseModel` is absent when the two are equal, so absence must be read as equal. Test: `grep responseModel` on each log.

### P15. A structured failure taxonomy (arXiv 2607.28802)

- Source: https://arxiv.org/abs/2607.28802, **community** (a paper, not a dsh document). Layer: L2. Decision: **adopted, four sides only**.
- Reason: the Phase 0 failure table classifies 12 of 12 rows by fault side (model, harness, environment, benchmark). One row (F12, a
  model request with no event until the kill) could not be told apart as provider or harness from the events available, so the
  taxonomy needs a fifth value, undetermined. The 41 failure modes of the paper are deferred, none was tested. dsh's own feedback
  categories are a different taxonomy and were not used.
- Chain: paper taxonomy -> lane reports use four fault sides and undetermined -> saving not measured.
- Risk: a coarse side can hide that a harness fault made a model look wrong. Test: the Phase 0 failure rows classified one by one.

## 8. Findings from the runs (not practices, but they bound the adoption)

- **MCP tool calls ran with no approval event** under read-only sandbox and approval policy never (R6: 0 `approval/asked`, 0
  `approval/decided`, 27 calls). The sandbox mode limits file writes, it does not gate MCP tools. Any lane that exposes an MCP server
  exposes all of its tools, including the 7 Telegram ones.
- **Telemetry default is `FEEDBACK_ONLY` to a third-party collector.** Lanes set `DSH_TELEMETRY_MODE=DISABLED` (P3).
- **A `providers` patch replaces the map**, it does not merge (P6).
- **Prune shadows, it does not delete** (P13): a log can be bigger than the context the model saw.
- **dsh records no version, prompt SHA or tool version** (P7).
- **The served model is not the requested one**, in every run asked for glm-5.3-flash (P14), so no routing saving can be claimed (P12).
- The ladder declares a context window of 1000000, which keeps compaction from firing before a provider overflow (P13).

## 9. Not measured, and why

- The Phase 0 cold start over HTTP (200 rows) was not rerun: it needs a polling loop, which would be a script in a language other than
  t27. `--dump-config` timing (R1) is the boot proxy; B for the practices that need a cold start rests on it.
- Money cost: the ladder has no price field (P11).
- Fork and replay of sessions (P4), the 41 taxonomy modes (P15), plugin install, schedule, escalation (P1, P5, P12): not exercised.
- Every row is one run, one machine, one day. A difference of a few seconds between runs is inside the noise seen between R3 and R4.
- Task prompts are reworded copies of the Phase 0 tasks (section 6).

## 10. No JSON published

`specs/tri/dsh_practice_receipts.json` is **not published**. The only generator, `t27c gen-json`, prints one IGLA board event
from a spec and cannot express a table of receipts, so any JSON here would be hand-written or built by a script outside t27, which
the only-t27 rule forbids. The records live in the spec (`practice(k)`), the receipts in section 6; a generator that can render
a record list is a t27c gap, not a file to fake.

## 11. How to check this file against the spec

From the repo root, no build needed. The bracket in each pattern keeps the spec's own header from matching itself.

```bash
grep -c 'status: S_[A]DOPTED, reason' specs/tri/dsh_practice_adoption_plan.t27
```

```bash
grep -c 'status: S_[R]EJECTED, reason' specs/tri/dsh_practice_adoption_plan.t27
```

```bash
grep -c 'status: S_[D]EFERRED, reason' specs/tri/dsh_practice_adoption_plan.t27
```

These print 9, 2 and 4. The spec itself is checked and tested with:

```bash
t27c check specs/tri/dsh_practice_adoption_plan.t27 && t27c test-report specs/tri/dsh_practice_adoption_plan.t27
```

The result at the time of writing: typecheck ok, 10 of 10 tests pass, 0 vacuous passes (every test executed runtime asserts).
