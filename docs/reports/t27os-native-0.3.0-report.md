# T27 OS native 0.3.0: completion report

Date: 2026-11-07. Branch: `lane/t27os-native-0.1.0-alpha.1` (the branch name keeps the first, wrong version
until the owner allows a rename; every file below says 0.3.0). Tracking issue: Trinity-S-AI-DNA/t27#13212.
Status words: EXECUTED means the test ran and its output was read. WAITING_RESOURCE means it was written,
parsed and typechecked, but no run was possible from this session. Nothing below is EXECUTED.

## Completion report (machine-readable)

```
VERSION_CREATED: NO
LOCAL_CHAT_SLICE: WAITING_RESOURCE
SESSION_RECOVERY: WAITING_RESOURCE
AGENT_ONESHOT: WAITING_RESOURCE
SWARM_ONESHOT: WAITING_RESOURCE
SAFETY_CHECK: WAITING_RESOURCE
DSH_CRITICAL_PATH_REQUIRED: NO
TEST_RESULTS: A=WAITING_RESOURCE B=WAITING_RESOURCE C=WAITING_RESOURCE D=WAITING_RESOURCE E=WAITING_RESOURCE F=WAITING_RESOURCE
REMAINING_GAPS: see the list below
```

`VERSION_CREATED` is NO because the brief makes all six acceptance tests the gate for declaring the version
created, and none has been run. The branch, the nine specs and the suite exist; they typecheck. The flip to YES
is one run of the two commands under "What the owner must run".

`DSH_CRITICAL_PATH_REQUIRED: NO` is by construction, not by measurement: no native spec imports a DSH module,
and the chat slice reaches LM Studio only through `native_host` (libSystem sockets on 127.0.0.1).

## What exists (11 commits, specs only, all `.t27`)

| Spec | Role |
|------|------|
| `native_version.t27` | T27OS_VERSION 0.3.0, four component versions, `version_report` rows |
| `native_provider.t27` | provider contract: list models, chat request, stream event, timeout, cancellation, token usage, redacted error classes; LM Studio adapter bytes |
| `native_log.t27` | append-only event log, six event kinds, sequence numbers, torn-tail recovery, cursor `Cur`, `continuable` |
| `native_policy.t27` | tool allowlist, approval gate, spawn refusal, audit lines, redaction, secret-only-from-environment |
| `native_agent.t27` | ONESHOT agent contract (9 fields), status machine, one optional read-only tool, one final result, cancel |
| `native_swarm.t27` | coordinator, at most 3 members, explicit task graph, deterministic collection, aggregate, cancel |
| `native_host.t27` | thin libSystem glue: env, loopback exchange, atomic log save, cancel file, `host_turn`, `host_run_agent`, `host_run_swarm` |
| `native_accept.t27` | the acceptance suite, 13 tests in groups A-F |

No Rust, JS, Python or shell was written. `git diff --stat origin/master` shows only these eight specs and this
report. The seven staged `.t27` files that belong to another actor, `parser_fixtures.t27`, `glass_window.t27`,
the LM Studio and NVIDIA settings, the DSH service and the stable chain were not touched.

## Tests (13 in `native_accept.t27`)

| Group | Test | Needs LM Studio | Status |
|-------|------|-----------------|--------|
| A | a_version_report_names_the_version_and_the_four_components | no | WAITING_RESOURCE |
| B | b_one_prompt_creates_a_session_lmstudio_answers_and_the_event_is_persisted | yes | WAITING_RESOURCE |
| C | c_live_session_continues_after_a_reload | yes | WAITING_RESOURCE |
| C | c_a_session_file_outlives_the_run_that_wrote_it | no | WAITING_RESOURCE |
| C | c_a_torn_tail_is_ignored_and_the_whole_events_remain | no | WAITING_RESOURCE |
| D | d_one_agent_runs_one_bounded_task_and_its_result_is_persisted | yes | WAITING_RESOURCE |
| D | d_cancellation_ends_the_agent_before_any_provider_call | no | WAITING_RESOURCE |
| E | e_the_coordinator_takes_at_most_three_members_and_only_under_a_spawn_policy | no | WAITING_RESOURCE |
| E | e_three_agents_run_in_task_graph_order_and_the_coordinator_aggregates | yes | WAITING_RESOURCE |
| E | e_cancellation_stops_every_pending_member_before_any_provider_call | no | WAITING_RESOURCE |
| F | f_an_unlisted_tool_is_rejected_and_command_and_network_tools_need_approval | no | WAITING_RESOURCE |
| F | f_a_secret_in_the_endpoint_never_reaches_a_log_or_the_report | no | WAITING_RESOURCE |
| F | f_only_the_api_key_is_a_secret_and_only_from_the_environment | no | WAITING_RESOURCE |

What was actually done: `t27_check` (parse and typecheck, no import resolution) returned ok, 0 errors, 0
warnings on `native_accept.t27`; imported names and signatures were checked by reading the source specs. The
module-level tests in the pure specs carry their own `test` blocks; they have not been run either. The live
tests assert the endpoint loads first, so without a server they fail; a mock cannot satisfy them.

One fact observed from this session: a status-code-only `GET /v1/models` on 127.0.0.1:1234 returned HTTP 200
at 13:28 local on 2026-11-07, so LM Studio was listening. No body was read and no chat request was made.

## What the owner must run (Mac load rule: the hook denies t27b and t27c without `MAC_HEAVY_OK=1`, which is the owner's alone)

From the worktree root, with an extern-capable `t27b` built from this branch (the binary in `~/t27/target/release`
dates from 2026-11-04; a binary without `@extern` support emits trapping stubs):

```bash
MAC_HEAVY_OK=1 ./target/release/t27b test specs/t27os/native_host.t27
```

```bash
MAC_HEAVY_OK=1 ./target/release/t27b test specs/t27os/native_accept.t27
```

The six pure specs run under `t27c test-report` (one spec per call). A T27 OS shell-cell pane on the owner's word
is the other sanctioned route. Delete `target/t27os/native-accept` to reset the one fixed-id log
(`c_a_session_file_outlives_the_run_that_wrote_it` appends 3 events per run; about 200 reruns fit in one log).

## REMAINING_GAPS

1. **No test has run.** All thirteen acceptance tests and the existing spec tests are WAITING_RESOURCE.
2. **Version baseline corrected.** The first commits named 0.1.0-alpha.1, below the published T27 OS line
   (0.2.0, `release.t27`, tag `t27os-v0.2.0-b7645`, master build 7896). The native line is now 0.3.0, the next
   minor, as proposed in `target/lane/VERSION_DECISION.md`. Open: the branch name and issue #13212 still say
   0.1.0-alpha.1 (a rename needs the owner); when the published line reaches 0.3.0 two artifacts will share
   the number (the `line=native` row keeps them apart). Nothing tagged, published, or edited in the release card.
3. **Process restart is simulated.** Acceptance C drops every in-memory copy and rebuilds from the file bytes;
   a true cross-process restart is covered only by running the suite twice (the fixed-id test reads what the
   first run left).
4. **Cancellation is checked at boundaries.** The host is single-threaded .t27, so cancel is a file
   (`<dir>/s<id>.cancel`) seen before the provider call or before each swarm member. A request already on the
   wire is not interrupted mid-flight; its deadline bounds it.
5. **No SSE streaming.** The slice returns the final response (stream events exist in the contract, not on the wire).
6. **Tool coverage.** The host runs `TOOL_CLOCK` only. `TOOL_READ_TEXT` has no host path allowlist yet, so it is
   not runnable. Command and network tools are classified, held for approval and never executed.
7. **Truncation limits.** Reply content is capped at 4096 bytes, agent result at 512, task 128, input 256,
   tool output 256, agent budget 4096 tokens, one tool call.
8. **Not checked:** file mode 0600 on logs, and `str` module constants passed as `[]const u8` at runtime in
   `t27b` (typechecks; first run will show).
9. **Wiring.** No seals saved (new specs stay unsealed), `native_host` is not wired into a UI window or the
   kernel, and the UI component is text-only by declaration (`component.ui.state=text-only`).
10. **Duplicate-bodies.** No function body was copied; the dupe ratchet has not been run on this branch.
11. **`host_run_swarm` takes 11 parameters** (three fixed agent slots), a consequence of no heap in .t27.
12. **#5980 was not needed.** The atomic write, env read, loopback socket and poll-with-deadline all have
    working .t27 paths through `pty_sys`, so nothing is parked and no capability issue is filed.

## Safety defaults in force

Secrets come from the environment only (`T27OS_LMSTUDIO_API_KEY`) and live in a zeroable `Endpoint.key`;
`endpoint_clear` wipes it. Logs and reports carry no key; the audit and error lines are redacted classes. A
tool outside the allowlist is rejected; command and network tools return "hold for approval"; an agent cannot
hold or call the spawn tool, and a swarm refuses members without an explicit spawn policy.

Requests out: owner -- run the two `t27b test` commands above (or say "pane" for a shell-cell pane); sent
2026-11-07; default: statuses stay WAITING_RESOURCE and `VERSION_CREATED` stays NO; due: before any merge.
