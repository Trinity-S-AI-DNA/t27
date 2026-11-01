# DSH to T27 OS migration: first package (2026-11-01)

Refs #12884 (epic: T27 OS Queen control plane, DSH as temporary execution adapter), #13020
(Matryoshka tracking). Branch `claude/dsh-to-t27os-migration-f903bb`, local HEAD `05b64483d5`
(20 lane commits ahead of and 25 behind `origin/master` `7fb6bc2391`). Nothing in this package
was pushed; push, publication, protected changes and the switch of the working route are
confirmed separately.

The owner's order (2026-11-01, translated): begin moving the working process from DSH
(deepseek-harness) into T27 OS; DSH is the behavioural reference, its logic moves step by step
into executable `.t27` modules; research, audit and implementation run in parallel; do not end
with a catalogue of specs; do not wait for the full migration before using T27 OS; migrate
economically by delegating tasks to DSH. Existing gates (publications, merges, sends,
confirmations) stay; moving the interface does not widen authority.

Three words are kept apart throughout: **spec exists** (a file on a named commit), **model
passes** (a compiler ran its tests, named compiler, FAIL 0) and **feature works** (a scenario in
the running app). A row that reached only the first says so.

Data files of this package (tool-written, home paths rewritten to `~/`):

- [`dsh-to-t27os-audit/audit-inventory-2026-11-01.tsv`](dsh-to-t27os-audit/audit-inventory-2026-11-01.tsv): every `.t27` on `origin/master`, one row each.
- [`dsh-to-t27os-audit/t27os-module-map-2026-11-01.md`](dsh-to-t27os-audit/t27os-module-map-2026-11-01.md): the T27 OS module map, module notes and capability map.
- [`dsh-to-t27os-audit/dsh-behaviour-2026-11-01.md`](dsh-to-t27os-audit/dsh-behaviour-2026-11-01.md): the DSH behaviour contract with evidence and confidence.

## 1. Audit scope: which corpus, how many files

Four corpora, never mixed:

| Corpus | Identity | `.t27` files | In `specs/t27os/` | Notes |
|---|---|---|---|---|
| Published t27 | `gHashTag/t27` `origin/master` `7fb6bc2391` | 3329 | 105 | The audit corpus. Content hash per file = git blob SHA (`git ls-tree -r`), in the TSV. |
| Local tree | this worktree, HEAD `05b64483d5` | 3315 | 97 | 20 unpushed `lane/queen-band-graph` commits, 25 behind master; `--ff-only` refused. Not the audit corpus. |
| t27 MCP view | corpus `1d9c5960d` (t27 MCP manifest) | 2402 | 0 | `specs/t27os` is not indexed. Health ok 2032 / warn 257 / fail 113 over its own list; of the 3329 master files: ok 1522, warn 177, fail 37, not indexed 1593. |
| Related repos | `gHashTag/trinity` local clone; `~/t27os` sibling clone | 1613; 21 | - | File counts only; their commit SHAs were not captured in this round (unverified column). |

Per-file columns in the TSV (origin/master): path, blob SHA, module name (`-` when the file
declares none: 92 files), `test` count, `invariant` count, `@extern` count, seal present
(`.trinity/seals/`), MCP health. Tallies: tests > 0 in 3021 files; invariants > 0 in 730; 22
files declare `@extern`; 2543 sealed / 786 unsealed (3173 seal files exist on master); MCP ok /
warn / fail / not indexed for files under `specs/` only: 1410 / 154 / 10 / 1359.

What the columns are and are not: "seal present" and "MCP ok" are ledgers written by earlier
runs on other machines; no compiler sweep ran on this Mac for this audit (owner rule: no builds
or sweeps on the Mac). "Executable" therefore means "a compiler once sealed it or the MCP once
checked it", not "it passed today". The compilers on this Mac are the main clone's
`target/release/t27c` (built 2026-11-01 17:40) and `t27b` (2026-11-01 20:19); this worktree has
none.

## 2. Critical modules and their host dependencies

The full module map (122 rows on HEAD plus the 10 DSH-related specs that exist only on
`origin/master`) is in the module-map file, sections 1, 1b and 2. "Wired" there means a direct
`use` from one of `glass_window`, `kernel`, `frame_main`, `router_serve` or `queen_chats`. The
chat path today, and what each module needs from the host:

| Module (`specs/t27os/` unless noted) | What it is | Wired | Host dependency | Conversation model |
|---|---|---|---|---|
| `glass_window.t27` | the macOS window (`@extern`, built by `t27b`, `make t27os`) | root | the whole host | draws the Queen pane as the transcript |
| `queen_chats.t27` | right dock: one 200-byte input line, sent as bytes + CR into the Queen's CLI pane | glass_window | pty_client keys and resize | none: no message, turn, role, id; hard-wired to `AGENT_QUEEN` |
| `session_store.t27` | one record per pane (`agent slot state sid model cwd`), restore plan, resume argv | glass_window, kernel, router_serve | file I/O under `~/.t27os/sessions/`, the SessionStart hook | session record only; history is the CLI's jsonl |
| `pty_proto.t27` + `pty_server.t27` + `pty_client.t27` | frame protocol over `~/.t27os/pty.sock`, bounded scrollback ring, ops LIST/SPAWN/KEYS/READ/... | through the client | unix socket, pty, 2 externs in pty_server | byte stream; a stale cursor is clamped, old output is lost past the ring |
| `keys.t27` + `keys_host.t27` | providers, rotation, circuit breaker, per-pane profile at spawn | glass_window | Keychain via `/usr/bin/security`, probe HTTP, child env | none; a client proxy to spend a stored key is marked MISSING in the spec |
| `agents.t27` | 9 CLI cells, 5 session states, event machine | glass_window, queen_chats | the terminal owner raising events | none |
| `kernel.t27` | supervision tree as data, resource formula, mailbox caps | none | sysctl / load / memory reads, not called | none; a lane is a process slot |
| `fanout.t27` | spawn admit by depth, Boundary decompose, result pack, join and land | none | none declared | result pack of a lane, no chat |
| `live_tap.t27`, `queen_graph_plane.t27`, `lanes_view.t27` | lane-table transitions become work-graph events | glass_window (tap) | none | no chat kind |
| `specs/t27f/work_graph.t27`, `specs/t27f/live_stream.t27`, `specs/t27f/live_chat.t27` | the graph, the one live seq, a 12-message chat panel (CK_USER / CK_TOKEN / CK_DONE, MAX_MSG 512) | live_chat by nothing | none | a streaming panel, not a durable log |
| `dsh_mcp_bridge.t27`, `dsh_scope.t27`, `agent_report.t27`, `dsh_head_election.t27` (master only, the first three) | 6 session tools mapped to ACP methods; a write gate; a report row; a rung flip law | none | an MCP host, the dsh ACP profile, a PreToolUse host, a probe sender: none exist | no prompt text, no call payload, no result |
| `router.t27`, `router_serve.t27`, `ladder.t27`, `specs/provider/stream.t27` | per-request routing, HTTP surface, rung law, structured stream parse | none (router_serve unsealed) | a socket accept loop | opaque model-call bytes |

Residual logic outside `.t27` on the chat path: the Claude Code CLI itself (the transcript, the
model loop, the tools) and the DSH process. T27 OS today drives CLIs through a pty; it does not
own a conversation.

## 3. DSH behaviour against T27 OS capabilities

Sources: the DSH behaviour file (clone `~/deepseek-harness` at `ce7073d512`, 2026-10-30, docs
and `.ts` sources read, nothing run; confidence high = read in source, medium = prose only) and
the module map section 3. "Verification" names the check that settles the T27 OS side; a check
that needs the window waits on the blocker in section 5.

| # | DSH behaviour (confidence) | User need | Existing T27 OS module | Missing | Verification method |
|---|---|---|---|---|---|
| 1 | Session = append-only event log with a stable id; blank-session reuse; resume; fork; list (medium) | start a chat, leave, come back with history, branch, no phantom chats, no lost draft | `session_store` (session record), `queen_chats` (input line), `live_chat` (12-message panel) | message, turn, role and event id; fork; blank reuse | #13069: `t27c test-report` FAIL 0, replay-equals-live test; window: reopen shows the same n events (after #13074) |
| 2 | Streaming reply: transient stream, durable settlement with `interrupted` prefix (high) | see text as produced; stop keeps the text shown | `pty_proto` OP_READ ring (raw, lossy), `provider/stream.t27` (parse, unwired), `live_chat` CK_TOKEN | settlement event with the interrupted flag; chunks never logged | #13069 `assistant_delta` / `cancel` tests; stream.t27 tests exist (not re-run here) |
| 3 | Five roles on one message base; source kind tells a human line from an injected note (high) | one transcript that says who produced each line | `live_chat` three kinds only | roles system / developer / user / assistant / tool; source kind | #13069 `role_known` closed-set test |
| 4 | `tool/call` with raw arguments; `tool/result` with error name, code, user-facing reason, `meta` card (high) | a tool card with name, args, error and reason that reappears after reload | `fanout` result pack; `dsh_mcp_bridge` table of 6 session tools | call and result payload; error kept outside model content; read-only allowlist | #13070: allowlist and result tests; integration: one read verb of the real tri CLI run from a pane |
| 5 | Prompt queue and steer; `requestId` idempotency; cancel keeps the inbox (high) | type while the agent works; stop without losing the next line; no double send | `actors` stop signal, `kernel` K_STOP, `dsh_mcp_bridge` T_CANCEL (no payload) | queue model, idempotent send, steer | #13069 idempotent-send test (0 events on the same request id) and cancel test; #13071 queue length in the snapshot |
| 6 | Subagent spawn / fork; depth and active caps; over capacity rejects, never queues (medium) | hand a bounded job to a helper, cap fan-out, talk to it later | `fanout` spawn_admit / decompose, `queen/actors` call_admit, `kernel` lane tree (all unwired) | stable child id, spawn vs fork seed, limit refusal as an event | #13072 tests through the existing fanout decisions |
| 7 | Running and finished executors shown; catalog fact in the parent; lifecycle pair (medium) | a helper panel that survives reload | `agents` states, `lanes_view`, `work_graph`, `pty_live_graph` (master) | the chat side of the catalog; child node and edge in the graph | #13073 fold test (same seq, one edge); window after #13074 |
| 8 | Result to parent: `parent` notice or `caller` promise; stop reasons; no durable mailbox, not exactly-once (medium) | see the helper's conclusion once, without polling | `fanout` result_delivery, join_decide, fanin_land | durable mailbox as events; exactly-once; accept / rework in chat (DSH lacks it too) | #13072 mailbox test: delivered once when the parent turn ends |
| 9 | Recovery: `follow` snapshot + gap-free frames; `observeEvent`; paged `events`; contiguous seq; orphan turns closed as interrupted (high) | reopen and see exactly the missed events | `session_store` restore_plan (session only), `pty_proto` cursor (volatile) | event store, cursor resume, snapshot, orphan close | #13071 resume-from-cursor test (0 duplicates); window reopen after #13074 |
| 10 | Approvals: asked / decided audit pair; outcomes allowed-once, rejected, cancelled, unavailable; no answerer = deny (high) | a prompt per risky action that cannot widen; audit trail | `dsh_scope` write gate, `dsh_mcp_bridge` needs_go, `kernel` request default | approval events in the log; the closed outcome set | #13069 EV_APPROVAL_* kinds; #13070 HOLD verdict for unknown tools; existing gates untouched |
| 11 | Compaction: pressure and overflow triggers, pruner, summary with shadowed range, lock events (medium) | long chats keep working; raw log intact | none (context belongs to the CLI) | compaction events, pruner, orphan-lock detection | Wave C, no issue filed (economy); later a shadowed-range replay test |
| 12 | Search, history, lineage, export (medium) | find an old conversation, jump to an event, see parent and children | `session_store` transcript_path (the CLI's jsonl), `graph_k_hop` (master) | session list and search over the chat store; lineage from delegation | Wave C after #13071 and #13072: a k-hop query from a parent returns its child |
| 13 | Goal with revision compare-and-set; plan mode as soft guidance; todo list replaced whole (medium) | a visible objective with a stop reason; a checklist | `queen/tasks.t27` (kinds, 7 states), `queen/dashboard`, `work_graph` | goal revision CAS; todo events | Wave B later; tasks.t27 tests exist; a chat_graph fold of task state |
| 14 | ACP server surface: v1 plus session list / resume / close; one prompt in flight (medium) | an external controller drives sessions | `dsh_mcp_bridge` (table only), the pane server `~/.t27os/t27os-pty` (list, spawn, write, read, kill) | the MCP host of the bridge, and on this Mac the dsh `acp` profile itself | blocker: `dsh --profile acp` answers `profile "acp" does not exist`; owner installs the plugin; then #13028 / #13047 |
| 15 | Event log and sequence model: `{type, seq, time, data, ...}`, contiguous seq, one replayable truth (high) | one truth for chat, tree and graph | `live_stream` seq (t27f), `work_graph` events, `live_tap` | chat events on the same log; the contiguity invariant | #13069 invariant `next_seq == count + 1`; #13073 same-seq fold, equal digests on replay |

DSH defects the owner met and the docs do not name: a double send, an unsent paste, old
scrollback shown as current. Each gets a named test in #13069 (idempotent send) and #13071
(draft survives reopen; resume from cursor yields no old frames as new).

## 4. Prioritised migration backlog, with executors

Executor rule (`specs/claw/model_route.t27`, TIER_PANE; owner 2026-10-20): a sub-task starts in
a free-tier T27 OS pane, goes to a Sonnet subagent after 2 failed attempts, to Opus after 4 or
on the owner's word. Delegation to DSH = one four-heading issue per thread (Matryoshka: one
thread = one issue), picked up by a DSH thread or the Queen when DSH boots again. Labels
(`P0` / `P1`) are priority = authority, so none were set; the owner labels.

| Wave | Spec (Boundary) | Issue | What it gives | Executor | State 2026-11-01 |
|---|---|---|---|---|---|
| A1 | `specs/t27os/chat_log.t27` | #13069 | durable conversation event log: session id, contiguous seq, 5 roles, turn end reasons, idempotent send, cancel, replay | pane 13 (Claude Code, free tier, this worktree), brief `target/BRIEF-chat-log.md` | brief sent; gate = `t27c check` + `t27c test-report` FAIL 0, pane runs it; result not yet read |
| A2 | `specs/t27os/tool_call.t27` | #13070 | tool call and structured result; read-only allowlist of real tri / MCP verbs; ALLOW / REFUSE / HOLD | pane or DSH thread | filed |
| A3 | `specs/t27os/chat_history.t27` | #13071 | lossless record line, resume from cursor, snapshot, draft survives reopen, bad line counted | pane or DSH thread, after A1 | filed |
| B1 | `specs/t27os/delegation.t27` | #13072 | subtask with stable child id, attempts, durable parent mailbox, accept / rework / abandon | pane or DSH thread, after A1 | filed |
| B2 | `specs/t27os/chat_graph.t27` | #13073 | chat and delegation events folded into `work_graph` with the same seq: one model, three views | pane or DSH thread, after A1 and B1 | filed |
| blocker | launch receipt | #13074 | a runnable post-#13016 `T27OS.app`, rescue copy, 10-minute receipt | owner (MAC_HEAVY_OK) or a machine that is not the owner's Mac | filed |
| C | compaction, search and lineage, confirmable external actions, SHA-bound evidence, resource-limited checks | none yet | the self-pipeline | later; one issue each when A and B land | backlog only (economy) |

Tracks and who edits shared files: Repo audit and DSH research are done for this round (this
package). Chat-core = A1, A3; Agent-runtime = B1; T27 OS UI = the window after #13074; tri /
MCP / TriGit = A2 allowlist from the real verbs, then the bridge host; Verification = the gate
in every issue (FAIL 0, 0 vacuous passes, `exit 0`). Shared files (`glass_window.t27`,
`work_graph.t27`, `live_tap.t27`) are not in any Boundary above: one integrator wires them in a
later issue once the pure specs pass.

Cutover ladder per feature (owner's order): external route -> T27 OS shadow -> verified work in
T27 OS -> main route T27 OS -> external only for recovery. Today every chat feature is on rung
1 (DSH). A1 moves "message log" to rung 2 the day its spec passes and the window shows it.

Main criterion: the owner returns to DSH less often because real tasks run in T27 OS. Baseline
today: 0 real tasks have run through a T27 OS chat (the window does not start); 1 real task
(A1) is running in a T27 OS pane without a window. Count both per day from here.

## 5. First chat scenario: what ran, what blocks it

The first mandatory vertical: owner writes a line in chat -> agent accepts -> calls one allowed
read-only tool -> result in chat -> saved in history -> restored after reopen. Then: parent
creates a subtask -> child runs -> result returns -> parent accepts -> the same events in chat
and graph.

| Step | State | Evidence |
|---|---|---|
| owner line in a T27 OS chat | blocked | no T27 OS window process runs (pgrep 0); `/Applications/T27OS.app` binary dated 2026-10-27 crash-loops (25 reports to 2026-11-01); the fix #13016 merged 2026-11-01 but no newer binary exists on the disk; builds forbidden on the Mac -> #13074 |
| owner line to an agent through the pane server | works | `T27OS_CTL=spawn` gave pane 13; `T27OS_CTL=write` + `T27OS_RAW=1 T27OS_KEYS='\r'` delivered the brief; `T27OS_CTL=list` shows it (13 panes: 7 idle Claude, 4 idle shell cells, 2 blocked) |
| agent accepts and calls a read-only tool | spec missing | no tool-call or allowlist model in `.t27` -> #13070 |
| result saved and restored after reopen | spec missing | no message store; `session_store` restores the session, not its messages -> #13069, #13071 |
| subtask -> child -> result -> accept | spec partial | `fanout` decisions exist, unwired; no accept step -> #13072, #13073 |

DSH as the delegation target, probed on this Mac (installed `@deepseek-ai/dsh` 0.1.0-rc.8):

- Headless boot fails: plugin `dsh-credentials-local` wants `~/.dsh/.credentials.yaml` as a flat
  mapping ref -> string; the owner's file (rewritten by launchd `ai.t27.dsh-zai-rotate`) has
  nested `records`, `refs`, `version`; error `the value for "records" ... must be a string`.
  Key names only were read, no value.
- ACP profile absent: `dsh --profile acp` answers `profile "acp" does not exist; create it with
  'dsh plugin --profile acp add <package>'`. The bridge spec `dsh_mcp_bridge.t27` therefore has
  no server to talk to on this Mac.
- DSH Web (127.0.0.1:3080, launchd `ai.t27.dsh-web`) is a UI, not an MCP server; its access
  token is not reused (#12973 FR-003); the service was not restarted.

So the economical route today is the one taken: issues for DSH threads, one pane for A1.

## 6. Own diff, integration SHA, checks

Files in this change (all prose or tool-written data; no code in any language):

- `docs/coordination/dsh-to-t27os-migration-2026-11-01.md` (this file)
- `docs/coordination/dsh-to-t27os-audit/audit-inventory-2026-11-01.tsv` (3329 rows + header)
- `docs/coordination/dsh-to-t27os-audit/t27os-module-map-2026-11-01.md`
- `docs/coordination/dsh-to-t27os-audit/dsh-behaviour-2026-11-01.md`
- `docs/README.md` (one row in the coordination table, Article DOCS-TREE)

Integration: the audit corpus and the PR base are `origin/master` `7fb6bc2391`. This branch
carries 20 commits of `lane/queen-band-graph` that are not this work, so the package needs a
fresh branch cut from master before any PR (not done; push is confirmed separately). The commit
of this package is local, made with the standard hooks on (lefthook: gitleaks, own-language,
foreign budget); its SHA is in `git log` of the branch.

Checks that actually ran for this package, and on what:

| Check | Where | Result |
|---|---|---|
| issue-shape gate on #13069 .. #13074 | GitHub workflow `needs-boundary.yml` | read back with `gh issue view --json state,labels` after filing; see the commit message footer for the states at commit time |
| home paths in the package | `grep -c` for the absolute home prefix on the three data files | 0, 0, 0 |
| `t27c check` + `t27c test-report` on `chat_log.t27` | pane 13 on this Mac, single spec, as the brief orders | pending; the pane's report is `target/REPORT-chat-log.md`; the diff is reviewed before any commit of the spec |
| any compiler sweep, seal re-save, build | none | not run (owner rule); seal and MCP columns are earlier ledgers |

Not verified in this round: the trinity and t27os clone SHAs; whether the 2543 sealed files
still pass on today's `t27c`; every "medium" row of the DSH table (prose only, not source).

## Requests out

| To | What | Options and cost | Default and when | Meanwhile |
|---|---|---|---|---|
| owner | `~/.dsh/.credentials.yaml` schema: the rotation writes nested keys, rc.8 wants flat ref -> string | (a) teach the rotation the flat schema, (b) upgrade dsh to a build that reads `records`, (c) leave headless off | (c) from 2026-11-02 06:03Z | panes carry the lanes; DSH threads pick up #13069 .. #13073 when they boot |
| owner | install the dsh `acp` profile plugin (`dsh plugin --profile acp add <package>`) or retire the ACP target of `dsh_mcp_bridge.t27` | one command, or one spec header edit | retire-not-yet: #13028 / #13047 stay pending | no bridge host is written |
| owner | a post-#13016 `T27OS.app` build and launch receipt (#13074): MAC_HEAVY_OK on this Mac, or a lab machine | one `make t27os` plus a 10-minute launch | compiler-level verification only until then | A1 .. B2 land as pure specs |
| owner | priority labels on #13069 .. #13074 | `P1` on A1 .. A3 would make the Queen pick them first | unlabelled (picked last) | - |
| owner | push of this package from a fresh branch off `7fb6bc2391` and a PR | one branch, one PR, no code | local commit only | the package is readable in this worktree |
