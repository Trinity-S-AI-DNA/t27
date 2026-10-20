# Our actor runtime (the Queen, t27 / trios): feature inventory, gaps and measured numbers

Source baseline (read 2026-10-09). The scratchpad copies named in the assignment had been wiped, so the same sources were re-read at pinned commits. Spec of record: gHashTag/t27 `origin/master` f21a0b39c (actors.t27 last changed by facad0d9a, #8137; 1648 lines, 37 tests, 21 invariants). Runtime: gHashTag/trios branch `queen` head 274db7adc (2026-10-09 06:14 +0700, #1707), path `trios/agent-server/apps/server`; this is the branch Railway deploys trios-agent-server from ([memory queen-moved-to-trios L22-26](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md)). Measured numbers come from the memory notes plus the PR and issue bodies where the benchmark tables were posted: trios#1698, #1702, #1704, #1707 and t27#7851. "Inferred" marks anything I reasoned from code rather than read as a stated fact.

## Q1. Which BEAM/OTP concepts exist (spec vs runtime), and which are absent?

### Takeaway
The spec (actors.t27) models most of OTP: generational pids, bounded mailbox, a control lane, selective receive, `after`, links/trap_exit/kill, monitors with flush, calls with aliases and cycle detection, three restart types and three strategies, intensity, backoff with jitter, shutdown timeouts, auto_shutdown, max_children, hung-turn kill, nodes, and a file-owner actor. The TypeScript runtime implements a **subset**: pids with generations, a bounded FIFO mailbox, the control lane, one-way monitors, the turn-timeout kill, supervisors (3 strategies, intensity, backoff and jitter), slices, thread/process isolates, and nodes with remote spawn and monitor. **Links, trap_exit, exit-signal propagation, selective receive, `receive ... after`, call/reply/alias, demonitor, graceful shutdown timeouts, max_children, auto_shutdown, the Queen tree and the file owner are specified but absent from the runtime.** By design there is no hot code swap, no global name registry and no process groups.

### Cited Findings
**Design stance (spec):**
- The spec is explicitly "taken from the BEAM (Erlang/OTP)". It lists what it adopts: identity, async non-blocking unacknowledged send, per-pair order, selective receive with `after`, links, untrappable kill, monitors, and supervisors with MaxR/MaxT — [actors.t27 L2-26](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L2-L26).
- Section 7 lists the deliberate departures as boolean constants: `MAILBOX_BOUNDED=true`, `HOT_CODE_SWAP=false`, `TRANSPARENT_DISTRIBUTION=true`, `PREEMPTIVE_REDUCTIONS=false`, `PREEMPTIVE_SLICES=true`, `GLOBAL_NAME_REGISTRY=false`. They are pinned by the invariant `section_7_departs_from_the_beam` — [actors.t27 L932-954](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L932-L954).
- How decisions are made: every runtime decision is a call into `queen/actors.wasm`, the compiled card of actors.t27 (`loadCardWasm`; `c64`/`c` wrappers). `queen-actors.ts` "holds the state, runs the turns, and keeps the clock it is given" — [queen-actors.ts L6-20, L56-65](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L6-L65). The constants come from `t27c gen-js`, which "lowers declarations, not bodies" — [queen-actors-card.gen.js L1-20](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-card.gen.js#L1-L20).

**Concept-by-concept matrix:**

| Concept | Spec | Runtime | Status |
|---|---|---|---|
| Pid = slot<<32 \| generation; restart reuses the slot with gen+1; an old pid reaches nobody | [L37-69](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L37-L69) | `spawn` uses `next_gen`/`pid_of`; `procOf` checks `reaches` ([L202-224, L297-329](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L202-L329)) | Implemented. Generations live only in an in-memory `Map` (L202) |
| Async send that never blocks and is never acked; dropped silently to a dead pid | [L73-91](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L73-L91) | `send` returns a delivery code; the drop is counted in `stats.deadLetters` ([L337-363](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L337-L363)) | Implemented |
| Bounded mailbox, MAILBOX_CAP 256 ("Chosen, not measured") | [L82-91](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L82-L91) | `deliver(found, box.length, MAILBOX_CAP)` ([L354](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L354)) | Implemented |
| Order only per sender-receiver pair | [L93-97](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L93-L97) | FIFO array, `box.push`/`box.shift` ([L359, L437](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L359-L437)) | Implemented (FIFO) |
| Selective receive (oldest match; the rest stay queued) | [L99-155](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L99-L155) (8-slot u64 model) | Not present: `receive(msg)` is one callback per message, taken in FIFO order ([L436-437](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L436-L437)) | **Spec only** |
| `receive ... after T`, `after 0` poll | [L157-169](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L157-L169) | Not present | **Spec only** |
| Control lane: 1 bit per kind, coalescing, taken before data (Akka system-message rule) | [L171-243](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L171-L243) | `post()` + `ctl_*` + `next_lane` ([L365-374, L419-435](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L365-L435)) | Implemented |
| Dead letters "counted where the reconciler reads it" | [L191-197](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L191-L197) | In-memory `stats.deadLetters`. The messages left in a dead process's box are added on exit ([L412](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L412)) | Partial. No reader of the counter was found (inferred) |
| Call = request + alias + monitor + `after`; late reply to a dead alias; call-cycle refusal, CALL_MAX_DEPTH 8 | [L245-311](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L245-L311) | `call_admit`, `call_outcome` and `live_alias` appear in the generated card only, as "not emitted"; nothing in `src` calls them (grep) | **Spec only** |
| Links (bidirectional), exit signals, trap_exit, kill->killed | [L313-359](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L313-L359) | No link API and no trap flag. `exit()` only deletes the process and sends DOWN to monitors ([L403-417](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L403-L417)). `on_exit_signal` is never called (grep) | **Spec only** |
| Link/monitor to a dead pid answers noproc at once | [L353-359](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L353-L359) | `monitor` sends DOWN with X_NOPROC ([L393-396](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L393-L396)) | Implemented for monitors |
| Monitors: one-way, one DOWN per monitor call, demonitor with flush | [L361-377](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L361-L377) | `watchers: Map<Pid, Set<Pid>>` ([L205, L397-399](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L397-L399)). No demonitor | Partial. Inferred: a watcher that monitors one target twice gets one DOWN (a Set), not two as the spec says |
| Restart types permanent/transient/temporary | [L381-393](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L381-L393) | `on_child_exit(rtype, ...)` ([L639-655](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L639-L655)) | Implemented |
| Strategies one_for_one / one_for_all / rest_for_one | [L395-422](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L395-L422) | `stopped_by_restart` and `restarted` from the card ([L664-695](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L664-L695)) | Implemented |
| Restart intensity MaxR in MaxT -> give up with `shutdown` to the parent | [L430-450](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L430-L450) | sliding window `in_period`, `SUP_GIVE_UP` -> `stopAll` + `onExit(GIVE_UP_REASON)` ([L633-661](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L633-L661)) | Implemented |
| Exponential backoff (base 10 s, cap 300 s, stable 600 s, give up after an unstable streak > 6) | [L452-486](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L452-L486) | `unstable_streak`, `with_backoff`, `backoff_seconds` ([L640-655, L672-678](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L640-L678)) | Implemented (beyond OTP; Akka/K8s-style) |
| Jitter pulled down by up to 20%, phi-hash of the slot | [L488-506](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L488-L506) | `jittered_seconds(backoff, slot)` ([L672-678](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L672-L678)) | Implemented |
| Shutdown: brutal_kill / timeout then kill / infinity for supervisors | [L508-520](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L508-L520) | `actorChild.stop` is `exit(pid, X_SHUTDOWN, quiet)` at once ([L584](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L584)). `stop_signal` is never called | **Spec only** (no graceful window) |
| Stop in reverse start order | [L424-428](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L424-L428) | `stopAll` iterates from the end ([L625-629](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L625-L629)) | Implemented |
| Significant children / auto_shutdown (OTP 24) | [L522-559](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L522-L559) | never called | **Spec only** (the Queen uses AUTO_NEVER anyway, [L610](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L610)) |
| max_children (DynamicSupervisor) | [L561-588](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L561-L588) | never called | **Spec only** |
| Supervisor as a process | OTP supervisors are processes | The runtime `supervisor()` is a closure with no pid or mailbox; restarts are timers on `clock.after` ([L601-708](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L601-L708)) | Divergence (inferred from code) |
| Hung-turn kill (TURN_MAX_SECONDS 3600, per-domain bound) | [L670-690](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L670-L690) | kill timer per data turn ([L438-452](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L438-L452)) | Implemented (see Q2) |
| Queen tree: root one_for_one over intake, reconciler and 4 domain sups x 4 agents (slots 1..23) | [L590-668](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L590-L668), [L968-970](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L968-L970) | `root_child_kind` and `queen_restart_type` are never called. Only a reviewer tree exists (Q4) | **Spec only** |
| Timers / send_after | not specified | No `send_after` API. Actors call `sys.clock.after` themselves, e.g. the intake heartbeat ([queen-review-actors.ts L117-120](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L117-L120)) | Absent as a primitive |
| Name registry | `GLOBAL_NAME_REGISTRY=false`: "the store is the only directory" ([L941](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L941)) | Only `registerKind(name, factory)`, a per-node registry of spawnable **kinds** for remote spawn ([L520-523](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L520-L523)) | No process-name registry |
| Process groups (pg) | not specified | none | Absent |
| Hot code upgrade | `HOT_CODE_SWAP=false`: "a new build is a retired runtime and a fresh one" ([L935](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L935)) | none | Absent by design |
| File owner actor (one owner per path hash, edit regions, admit/queue/refuse) | section 10, [L832-930](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L832-L930) (merged #8137) | Not in the vendored card (no FILE_SLOT_BASE; `__DECL_ORDER__` ends at section 7's flags, [card L234](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-card.gen.js#L234)); `file_admit`/`edits_meet` absent from `src` | **Spec only** |

**Other observations:**
- The spec's own header is stale. It still says the supervisor is TypeScript in gHashTag/BrowserOS with "no mailbox, link, monitor or restart", and that "every rule below is a contract with tests, not a measured runtime behaviour" — [actors.t27 L28-32](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L28-L32). The runtime has existed since trios#1698 (merged 2026-10-08T18:25Z) — [trios#1698](https://github.com/gHashTag/trios/pull/1698).
- Spec-quality history: the first version had 16 tests, 6 invariants and 62/62 mutants killed. Later whole-file mutation runs gave 255/261 killed and then 296 killed of 319, with 6 survivors that are known loop bounds and 17 hangs — [memory queen-actors-beam L20, L39, L54](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-actors-beam.md).

### Inferences
- In OTP terms the runtime is "gen_server + monitor + supervisor + backoff". It has no link graph. Failure travels only through DOWN messages to explicit monitors and through supervisor `onExit` callbacks, so the spec's links section (crash cascades, trap_exit) has no runtime counterpart.
- There is no call primitive, so request/reply between actors is hand-rolled: the reviewer workers send `done` back to the intake by pid ([queen-review-actors.ts L196-203](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L196-L203)).
- Local slots come from a monotonic `nextLocal++` ([L203, L303-305](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L303-L305)) under a 20-bit LOCAL_SLOT_MASK (1,048,575; [actors.t27 L777-789](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L777-L789)). So a process lifetime gets about 1M fresh spawns before `node_pid` returns NO_PID. Restarts reuse slots and are not affected.

### Gaps
- No reader of `stats.deadLetters` and no export of it to a route was located. Only the services and tests directories were inspected for this.
- No evidence of a dead-letter reconciler for actor mail (the spec assigns repair to control.t27's reconciler).

## Q2. How do turn timeouts, slices and isolates work, and what do they guarantee?

### Takeaway
A data turn gets a kill timer: `turnMaxSeconds`, or TURN_MAX_SECONDS 3600. On expiry the pid dies with `killed` and its supervisor restarts it. **On the event loop, and in a worker thread, a kill only abandons the work; only an OS-process turn is really stopped (SIGKILL).** Slices preempt between turns only: at most 64 turns or 5 ms, then a `setImmediate` yield. They cannot interrupt a running synchronous turn. That needs isolation, which is decided per actor at spawn.

### Cited Findings
**Turn timeouts:**
- Spec: the domain supervisor kills a runtime whose turn has run TURN_MAX_SECONDS = 3600 ("chosen, not measured: twice the reviewer bee's 1800 s"). `turn_signal(age, bound)` uses the per-domain bound, and 0 means the default — [actors.t27 L670-690](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L670-L690). The reviewer's per-turn bound is REVIEW_ROW_SECONDS = 300 — [reviewer.t27 L35-37](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/reviewer.t27#L35-L37).
- Runtime: each data turn arms `clock.after((bound || TURN_MAX_SECONDS)*1000)`. On fire, if `turn_signal` says X_KILL, `stats.killed++`, then `stopWork()` if `kill_effect == KILL_STOPS`, then `exit(pid, death_reason(X_KILL))` (= X_KILLED) — [queen-actors.ts L438-452](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L438-L452). Control-lane turns get no kill timer: they are synchronous `control()` calls ([L423-435](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L423-L435)).
- An abandoned turn keeps running. Its late completion is ignored because `!current(p)` ([L461-475](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L461-L475)), and its sends with `from` are dropped as dead letters ([L338-341](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L338-L341)). Its external effects are fenced by control.t27's lease — [queen-actors.ts header L22-29](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L22-L29).
- The reviewer workaround: a killed review keeps its row and z.ai key lane until the review really ends (map `reviewing`), so no row is reviewed twice at once — [queen-review-actors.ts L90-96, L186-195](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L90-L195). The fix was commit 1f738b9: without it "the intake would have handed the same row to a second worker" — [t27#7851 comment 2026-10-08T18:26Z](https://github.com/gHashTag/t27/issues/7851).
- A known side effect: "During a stall, more than REVIEWER_CONCURRENCY reviews can run at once: 4 workers plus each abandoned review, until it ends" — [trios#1698](https://github.com/gHashTag/trios/pull/1698).

**Slices:**
- Spec: SLICE_TURNS = 64 and SLICE_MICROS = 5000. `slice_spent` is true when either is reached. Invariant: `SLICE_MICROS <= 20000` — [actors.t27 L706-729, L1546](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L706-L729). The background measurement: "one message chain ran 200 000 turns without yielding once" — [L712-714](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L712-L714).
- Runtime: one ready queue. A slice opens on the first turn and closes on the next `setImmediate`. Once spent, the next drain is deferred with `setImmediate` (`yieldThenDrain`, counted in `stats.yields`). `slices:false` falls back to one `queueMicrotask` per step — [queen-actors.ts L229-295](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L229-L295).
- What slices do not preempt: "A JavaScript host runs one thread and cannot stop a function midway". A turn on the host thread "cannot be stopped"; the BEAM's reduction count can — [actors.t27 L709-711, L731-733, L939-940](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L709-L733).

**Isolation:**
- Spec `turn_isolation(cpu_bound, runs_foreign_code, must_stop)`: foreign code -> ISO_PROCESS; must_stop -> ISO_PROCESS; cpu_bound -> ISO_THREAD; otherwise ISO_LOOP (I/O waits stay on the loop) — [actors.t27 L734-746](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L734-L746).
- `kill_effect`: PROCESS -> KILL_STOPS; LOOP and THREAD -> KILL_ABANDONS. `holds_after_kill`: an abandoned turn holds its row and lane until its work ends — [actors.t27 L748-762](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L748-L762).
- Runtime: isolation is fixed per actor at `spawn` from `spec.isolated` ([L309-315](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L309-L315)). The isolated work runs first and its result is passed to `receive` ([L453-460](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L453-L460)).
- `threadWork` spawns a fresh Bun `Worker` per turn and terminates it on result; its `stop` = `worker.terminate()` — [queen-actors-isolate.ts L21-54](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-isolate.ts#L21-L54). The thread entry dynamically imports `module` and calls `fn(arg)` — [queen-actor-thread.ts](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actor-thread.ts).
- `processWork` = `Bun.spawn(argv)`: stdout is the result on exit 0; `stop` = `proc.kill(9)` — [queen-actors-isolate.ts L60-72](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-isolate.ts#L60-L72).
- Measured on Bun 1.3.11: "`worker.terminate()` left a spinning worker thread counting, still running 3 s later" — [actors.t27 L716-719](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L716-L719); [trios#1704](https://github.com/gHashTag/trios/pull/1704); [memory queen-moved-to-trios L43](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md). The test asserts the counter keeps rising after `stop()` and that `stats.stopped` stays 0 — [queen-actors-dist.test.ts L147-188](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/tests/api/queen-actors-dist.test.ts#L147-L188). The process test asserts SIGKILL with `stats.stopped == 1` — [L189-215](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/tests/api/queen-actors-dist.test.ts#L189-L215).
- The isolate file's header contradicts this. It says "The OS preempts both, and `stop` really ends the work" — [queen-actors-isolate.ts L6-10](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-isolate.ts#L6-L10).

### Inferences
- `proc.kill(9)` signals only the direct child pid. No process group is created, so grandchildren of a spawned shell or compiler may survive a kill (inferred; macOS/Linux semantics, not tested).
- Isolation is per actor, not per message. An actor that only sometimes runs CPU-heavy or foreign work must be a separate actor kind (inferred from the `spawn` code).
- Nothing in production uses `isolated` turns today: the reviewer workers are plain loop actors ([queen-review-actors.ts L178-205](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L178-L205)). So the production kill is always "abandon".

### Gaps
- No measurement of thread or process spawn overhead per turn. A new Worker per turn and a new process per turn are both unmeasured.
- No measurement of slice latency beyond the "0 ms timer ran after 128 turns" figure (Q5).

## Q3. Distribution: discovery, node-down, in-flight mail, delivery semantics, persistence, placement, split-brain

### Takeaway
Section 9 is distribution "over the store, not over sockets": the pid carries a 12-bit node id, a node holds a lease in Postgres (heartbeat 5 s, TTL 20 s), and remote mail is a row plus `pg_notify`. **Delivery is at-most-once.** There is no retry, no ack and no redelivery after a crash. Actor state, mailboxes and pid generations are in process memory and do not survive a deploy or restart. Placement picks the roomiest live node only when a child (re)starts, and nothing rebalances running actors. There is no split-brain protection in the actor layer; safety is delegated to control.t27's fenced task leases and its effects journal. **None of the cross-machine path is wired in production.**

### Cited Findings
**Spec model:**
- NODE_BITS 12 (MAX_NODE 4095) and LOCAL_SLOT_BITS 20 (LOCAL_SLOT_MASK 1,048,575). Node 0 = section 1 unchanged. `node_pid`, `node_of`, `local_of`, `is_remote` — [actors.t27 L764-802](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L764-L802).
- NODE_HEARTBEAT_SECONDS 5 and NODE_TTL_SECONDS 20. `node_up(age) = age < 20`. Invariant: TTL >= 3 x heartbeat — [L804-809, L1545](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L804-L809).
- `send_remote(down)` gives D_DROPPED_DEAD, a dead letter. `remote_down`: node down -> X_NOCONNECTION "whatever the pid did"; node up and pid ended -> its reason — [L811-823](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L811-L823).
- `place_before`: more free room wins, a tie goes to the lower node id "so that two placers reading the same counts choose the same node" — [L825-830](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L825-L830).
- "No code moves. Every node runs the same build, so a remote start names a kind, not a function" — [L775](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L775).
- X_NOCONNECTION "proves nothing -- the holder may be alive across a partition". The task waits out its lease TTL (`reclaim_wait_seconds`). An early reclaim is still safe because "the claim bumps the fence and the old holder writes nothing". External effects are covered by control.t27 section 7's effects journal and `look_wait_seconds` — [L692-704](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L692-L704).
- Task lease parameters: TASK_LEASE_TTL_SECONDS 180, TASK_HEARTBEAT_SECONDS 60, fenced `INSERT ... ON CONFLICT` claim — [control.t27 L112-125](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/control.t27#L112-L125). EFFECT_RUN_LIMIT 3 — [control.t27 L542](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/control.t27#L542).

**Runtime, transport-agnostic part:**
- `NodeLink {node, up, carry, onMail, onNodeDown}`. The Mail kinds that cross are send, monitor, exit, spawn and spawned, and they must survive `structuredClone` — [queen-actors.ts L135-160](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L135-L160).
- A remote send checks `link.up(to)` through `send_remote`, then `link.carry`; otherwise it counts a dead letter ([L342-351](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L342-L351)). On arrival the receiving node calls `send(mail.to, mail.msg)` and does not pass `from` ([L480-481](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L480-L481)).
- A remote monitor to a node that is not up gets an immediate DOWN with X_NOCONNECTION ([L377-386](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L377-L386)). When a node goes down, every registered remote watch on it gets DOWN X_NOCONNECTION ([L506-513](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L506-L513)).
- On any node-down, **every** pending `spawnOn` wait resolves to null, whichever node it targeted — [L514-517](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L514-L517).
- `spawnOn(node, kind, arg)` sends a `spawn` mail and resolves with the remote pid or null; the receiving node starts `kinds.get(kind)(arg)` — [L496-503, L525-541](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L496-L541).
- `remoteChild`: each (re)start picks the roomiest up node by `place_before`, spawns a local watcher actor that monitors the remote pid, and maps the DOWN reason to the supervisor's `onExit`. No node available -> X_NOCONNECTION — [L710-786](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L710-L786).

**Transports:**
- `createMemoryNet`: in-process nodes on one clock. `carry` = `structuredClone` + optional latency; mail to or from a crashed node is silently lost. Peers see a crashed node up until the TTL, then `onNodeDown` fires — [queen-actors-net.ts L1-75](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-net.ts#L1-L75).
- `createPgLink` (store as the wire):
  - Tables: `queen_actor_node(node int PK, host, heartbeat_at)` and `queen_actor_mail(id bigserial, node, body text, created_at)` with index `(node, id)` — [pg-migrate.ts L169-186](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/lib/db/pg-migrate.ts#L169-L186).
  - Lease: each heartbeat upserts the node's own row, reads **all** rows' ages using DB `now()`, and fires `onNodeDown` only on an up->down transition as this node saw it — [queen-actors-pg.ts L63-81](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-pg.ts#L63-L81).
  - Send: `INSERT` + `pg_notify` in one statement. Sends are serialized through one promise chain so ids keep per-sender order. A failed insert is only logged — [L116-135](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-pg.ts#L116-L135).
  - Receive: LISTEN on `queen_actor_mail_<node>` plus a poll every 1000 ms by default. Each drain is `DELETE ... RETURNING` of up to 500 rows `FOR UPDATE SKIP LOCKED`, sorted by id, **then** handled — [L83-114, L137-160](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-pg.ts#L83-L160).
  - The header: "Nothing here decides anything. Every decision is the card's" — [L9-18](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-pg.ts#L9-L18).
- Ack semantics: the spec sets `SEND_ACKNOWLEDGED=false` and `SEND_BLOCKS=false` — [actors.t27 L79-80](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L79-L80).

**Tests:**
- The two-nodes-over-PostgreSQL suite has 3 cases: a pid survives the wire exactly; sends cross in order and a monitor hears the real reason; a node that stops renewing is down and its monitors hear noconnection — [pglive/queen-actors-live.test.ts L86-169](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/tests/pglive/queen-actors-live.test.ts#L86-L169).
- pglive tests can only run in CI, because this Mac has no local Postgres — [memory queen-event-bus-slice3 L19](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-event-bus-slice3.md).
- The in-memory distribution tests cover a pid naming its node, ordered sends, the real reason while the node is up, noconnection after the TTL, and placement plus moving on node death — [queen-actors-dist.test.ts L216-330](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/tests/api/queen-actors-dist.test.ts#L216-L330).

**Persistence:**
- All actor state lives in closures and Maps inside `createActorSystem`: `gens`, `live`, `watchers`, `remoteWatches`, `kinds`, `spawnWaits`, and each process's `box` and `lane` — [queen-actors.ts L195-219](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L195-L219).
- The reviewer rebuilds its queue from the DB on every wake (`deps.waiting()`) — [queen-review-actors.ts L152-166](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L152-L166).
- A new build is "a retired runtime and a fresh one" (no hot swap) — [actors.t27 L935](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L935).

### Inferences
- **At-most-once across nodes.** Rows are deleted before the handlers run, so a crash or a decode error between DELETE and dispatch loses the whole batch, and a failed INSERT is logged, not retried. Inside one node, sends to a dead pid or a full box are dropped. This matches the spec's "never acknowledged" model.
- **Mail in flight to a node that dies stays in `queen_actor_mail`.** No code expires rows for dead nodes; only the addressed node deletes them. If a node id comes back after a restart, it drains the stale rows.
- **Pid reuse after a restart.** `gens` and `nextLocal` restart from empty, so a restarted node re-issues the same (node, slot, gen) pids. Stale rows can then reach the new incarnation's processes, which defeats the generation guarantee across process restarts.
- **Split-brain.** A node that peers consider down (lease older than 20 s) keeps running its actors, and nothing tells it to stop. `remoteChild` restarts the lost children elsewhere, so duplicates can run concurrently during a partition; correctness rests entirely on control.t27 fencing. The heartbeat `setInterval` runs on the same event loop as the actors ([queen-actors-pg.ts L152-159](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-pg.ts#L152-L159)), so a long synchronous loop turn can delay renewal and cause a false node-down.
- **No rebalancing or migration.** Placement happens only at start or restart. `room()` is supplied by the caller, and the only placer is test code.
- **Node identity is manual.** `createPgLink(pool, node)` takes the id as a parameter, and no env var or allocator assigns it (see Q4).

### Gaps
- There is no stated policy for node-id assignment, node row cleanup, or mail-row retention or TTL.
- No measurement of cross-node latency over PgLink (LISTEN path vs the 1 s poll) or of throughput.
- No documented handling of Postgres failover or partition between a node and the DB itself. The node's own view of "down" is not covered.

## Q4. What is wired into production today vs only specified or tested?

### Takeaway
In production exactly one domain runs on actors: **the reviewer**, behind `TRIOS_QUEEN_REVIEWER=actors` on trios-agent-server (live since 2026-10-08 ~19:00Z). It is a single-node, in-process system: node 0, no NodeLink, no isolates. The round, bee dispatch, runners and pollers are still timer loops. Sections 6 (Queen tree), 9-over-Postgres (PgLink) and 10 (file owner), plus the reviewer's adaptive concurrency, are specified and partly tested but not wired.

### Cited Findings
**Wiring and flags:**
- `queen-tick.ts` chooses the reviewer at startup. `TRIOS_QUEEN_REVIEWER=actors` -> `startReviewerActors`; anything else -> `startReviewer` (the loop). `off` hands reviewing back to the round — [queen-tick.ts L5004-5013](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-tick.ts#L5004-L5013); [queen-review-loop.ts L286-295](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-loop.ts#L286-L295).
- The flag was set on trios-agent-server with the owner's OK. "The rollback is to unset the flag; the loop is still the default code path" — [t27#7851 comment 2026-10-08T18:26Z](https://github.com/gHashTag/t27/issues/7851). The memory note says "Actors live in production since 2026-10-08 19:00Z (TRIOS_QUEEN_REVIEWER=actors, owner OK; trios#1698)" — [memory queen-moved-to-trios L36](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md).

**The production tree:**
- `queen-root` (one_for_one, ROOT_MAX_RESTARTS 2 in ROOT_PERIOD_SECONDS 900) -> `reviewer-domain` (rest_for_one, DOMAIN_MAX_RESTARTS 3 in DOMAIN_PERIOD_SECONDS 300) -> intake + REVIEWER_CONCURRENCY (=4) workers. Each worker turn is bounded at REVIEW_ROW_SECONDS 300 — [queen-review-actors.ts L6-29, L207-262](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L6-L262); constants at [actors.t27 L631-640](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L631-L640).
- rest_for_one is used instead of the card's one_for_one "because workers report to the intake by pid" — [queen-review-actors.ts L19-21](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L19-L21).
- The intake wakes on the bus event EV_TASK_ENDED and on a 10 s heartbeat posted to its control lane. A worker's DOWN puts its row back at the front of the queue unless a killed review still runs — [L117-151, L263-267](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L117-L267).
- If the root gives up, the system falls back: "reviewing goes back to the round" (`setReviewerRunning(false)`) — [L256-262](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L256-L262).
- `createActorSystem()` is called with no options, i.e. node 0, no link and slices on — [L242](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-actors.ts#L242). Grep of `src`: `queen-review-actors.ts` is the only production caller of `createActorSystem`.

**Wait backoff (both runtimes):**
- `withWaitBackoff` applies reviewer.t27 `due_again` to the loop and to the actors: 30 s doubling up to 900 s — [queen-review-loop.ts L55-56, L175-190](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-review-loop.ts#L55-L190); [reviewer.t27 L110-138](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/reviewer.t27#L110-L138); [trios#1702](https://github.com/gHashTag/trios/pull/1702).

**Not wired:**
- **PgLink / multi-node.** `createPgLink` has no caller in `src`, and `TRIOS_ACTOR_NODE` occurs nowhere in `src` (grep). trios#1704 itself says "The Postgres link that joins Railway services is the next PR" — [trios#1704](https://github.com/gHashTag/trios/pull/1704). `QUEEN_ACTORS_SQL` is still embedded in the boot `MIGRATION_SQL`, so the tables are created at every boot — [pg-migrate.ts L339](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/lib/db/pg-migrate.ts#L339).
- **Queen tree (section 6), links, calls, file owner (section 10).** See the Q1 matrix.
- **Adaptive reviewer concurrency.** reviewer.t27 now has `reviewer_concurrency(keys, lanes_per_key, free_mb, mb_per_review, waiting)` and REVIEWER_CEILING 16 (t27#8212, merged 2026-10-09T13:54Z) — [reviewer.t27 L22-62](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/reviewer.t27#L22-L62). The vendored card in trios still has only `REVIEWER_CONCURRENCY = 4` — [queen-reviewer-card.gen.js L6-13](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-reviewer-card.gen.js#L6-L13).
- **Everything else is still loops.** Epic t27#7851: "The Queen still works through timers and loops: the round on one clock, the reviewer on another (#7840), and the runners counting lanes" — [t27#7851](https://github.com/gHashTag/t27/issues/7851). The round is a `setInterval` gate — [queen-tick.ts L4995-5030](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-tick.ts#L4995-L5030).
- **The swap rule.** No loop may be replaced before an MVP, its tests, a same-input benchmark and the numbers posted on #7851 — [memory queen-moved-to-trios L57](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md).

**Hosting:**
- Railway services trios-agent-server (branch `queen`) and trios-bee-runner-1/2 (branch `queen-runners`) build from gHashTag/trios since 2026-10-09. rootDirectory `trios/agent-server` — [memory queen-moved-to-trios L22-26](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md).
- The bee runners deploy from `queen-runners` so that Queen merges do not restart them — [memory queen-deploy-does-not-stop-swarm L16](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-deploy-does-not-stop-swarm.md).

**Test inventory:**

| Suite | Cases | Notes |
|---|---|---|
| `queen-actors.test.ts` | 13 | includes the vendored card PIN check |
| `queen-actors-dist.test.ts` | 9 | slices, isolates, nodes |
| `queen-actors-bench.test.ts` | 2 | |
| `pglive/queen-actors-live.test.ts` | 3 | |
| actors.t27 | 37 tests, 21 invariants | |
| reviewer.t27 | 7 tests, 2 invariants | |

Runtime test counts are from grep of `it(` in the files at 274db7a. trios#1704 reports "Tests: 46/46 (actors, distribution, reviewer, holds, bench)" — [trios#1704](https://github.com/gHashTag/trios/pull/1704).

### Inferences
- In production the "actor runtime" is today a single-process supervision tree of 6 actors: root and domain supervisors (closures), intake, and 4 workers. Distribution, isolation and the broader Queen tree are capability in code and tests, not operational.
- `TRIOS_ACTOR_NODE`, named in the assignment as a flag, does not exist at the deployed head. A node id would have to be added when PgLink is wired.

### Gaps
- Whether `stopReviewer` is invoked on SIGTERM during the 30-minute drain (so the old container's reviewer stops) was not traced.
- Whether a production one-hour or 24-hour comparison (loop vs actors) was posted after 2026-10-08T19:57Z was not checked. The last #7851 comment read promises it.

## Q5. Every benchmark and measurement number available

### Takeaway
On the same seeded input with a virtual clock, the actor reviewer ties the loop without faults and wins with faults:

| Scenario | Measure | Loop | Actors |
|---|---|---|---|
| Overload | throughput | baseline | **+18.7%** |
| Faults | done p95 | 553 s | **384 s** |
| Faults | worst recovery | 1181 s | **27 s** |
| Burst | median wait | 2416 s | **1153 s** |

The runtime costs 3.7 us per message, 5.5 us with slices. In production the reviewer used 23% of its worker-seconds after the wait backoff. A day later the spec recorded about 11 reviewed/h against about 58 finished/h, and those two production readings conflict.

### Cited Findings
**Reviewer benchmark (trios#1698, final table after fix 1f738b9; seconds; horizon 7 h)** — [trios#1698](https://github.com/gHashTag/trios/pull/1698), [t27#7851 comment 2026-10-08T18:26Z](https://github.com/gHashTag/t27/issues/7851):
- Setup: arrivals Poisson from a fixed seed. Per-attempt fate: crash 5% (at 30% of its length) or stall 2% (1800 s, no own timeout). Review length is lognormal around the median, capped at 0.9 x 300 s. The loop runs one pass every 10 s. `actors-hb-only` separates the runtime from the bus — [queen-actors-bench.test.ts L1-125, L280-317](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/tests/api/queen-actors-bench.test.ts#L1-L317).
- Steady 45/h, 60 s reviews, faults (263 rows):

  | runtime | waitP50 | waitP95 | doneP50 | doneP95 | recoveryMax | killed |
  |---|---|---|---|---|---|---|
  | loop | 6 | 45 | 72 | 220 | 9 | |
  | actors | 0 | 0 | 62 | 215 | 8 | 8 |
  | hb-only | 5 | 10 | 66 | 220 | 20 | |

- Steady 45/h, 180 s, faults (278 rows):

  | runtime | waitP95 | doneP50 | doneP95 | recoveryMax |
  |---|---|---|---|---|
  | loop | 308 | 270 | 553 | 1181 |
  | actors | 152 | 210 | 384 | 27 |
  | hb-only | 160 | 215 | 386 | 10 |

- Steady 45/h, 180 s, no faults (262 rows): loop doneP95 319, actors 309, hb-only 315. Throughput is 37.4/h for all three.
- Overload 120/h, 180 s, faults (660 rows):

  | runtime | done | open | perHour | waitP50 | doneP95 | recoveryP50 |
  |---|---|---|---|---|---|---|
  | loop | 476 | 184 | 68 | 4669 | 8389 | 12259 |
  | actors | 565 | 95 | 80.7 | 2824 | 6363 | 10 |
  | hb-only | 569 | 91 | 81.3 | | | |

- Burst of 120 in 10 min + 45/h (365 rows): waitP50 loop 2416, actors 1153, hb-only 1115. doneP95 loop 6551, actors 5122.
- The first, pre-fix run reported smaller wins: overload +12.6% (536 vs 476), faults doneP95 453 vs 553, recoveryMax 294 vs 1181, burst waitP50 1607 vs 2416, overhead 3.9 us per message — [t27#7851 comment 2026-10-08T17:51Z](https://github.com/gHashTag/t27/issues/7851).
- Stated limits: "It is a simulation" (arrival rate matches production at ~45/h; lengths and fault rates are assumptions), and the serialized steps (importRunnerBranch, criteria worktree) are not modelled — [trios#1698](https://github.com/gHashTag/trios/pull/1698).

**Runtime micro-benchmarks:**
- Per-message cost: a ring of 1000 actors passing 100,000 messages, every decision a wasm card call — [queen-actors-bench.test.ts L360-391](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/tests/api/queen-actors-bench.test.ts#L360-L391).
  - 3.7 us per message without slices (3.9 us in the first run) — [trios#1698](https://github.com/gHashTag/trios/pull/1698).
  - 5.5 us with slices, "the price of fairness" — [trios#1704](https://github.com/gHashTag/trios/pull/1704). The memory note calls this "about 30%" — [memory queen-moved-to-trios L44](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md). Arithmetic: +49% per-message time, or -33% messages/s.
- Slice fairness: a 0 ms timer during a 200,000-turn message chain ran after 200,000 turns before slices, and after **128** turns (3125 yields) with them — [trios#1704](https://github.com/gHashTag/trios/pull/1704); [queen-actors-dist.test.ts L41-80](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/tests/api/queen-actors-dist.test.ts#L41-L80).
- Thread vs loop: 8 turns of 120 ms CPU on 4 actors took 962 ms wall on the loop and **251 ms** in threads. The longest event-loop hold was 952 ms on the loop and **1 ms** in threads — [trios#1704](https://github.com/gHashTag/trios/pull/1704).
- Node loss: 3 of 6 workers on a dying node were all restarted on the other node "within the 20 s lease TTL + backoff (<= 10 s)" — [trios#1704](https://github.com/gHashTag/trios/pull/1704); [queen-actors-dist.test.ts L285-330](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/tests/api/queen-actors-dist.test.ts#L285-L330).
- A terminated worker thread was still counting 3 s after `terminate()` (Bun 1.3.11) — [actors.t27 L716-719](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L716-L719).

**Production measurements:**
- Baseline at the flip (loop), from `/queen/status`: reviewed/h 25, finished/h 62, unreviewed 100, active bees 27/70. Over the previous 8 h, reviewed/h was 23-31 — [t27#7851 comment 2026-10-08T18:26Z](https://github.com/gHashTag/t27/issues/7851).
- Actors live at 19:00Z without wait backoff — [t27#7851 comment 2026-10-08T19:57Z](https://github.com/gHashTag/t27/issues/7851):
  - about 184 visits/min to the 22 unreadable rows, against about 20/min for the loop;
  - the round's tail stretched to about 6 min (normally about 30 s);
  - active bees fell to 22-27 of 70;
  - 13 real decisions in 6.5 min.
- Over 44 minutes of logs: 941 reviewer visits, 888 of them `wait` (94%), from 22 rows. One row was visited 54 times, and only 53 visits decided anything — [trios#1702](https://github.com/gHashTag/trios/pull/1702); [reviewer.t27 L110-114](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/reviewer.t27#L110-L114). The memory note adds that criteria measurement ran on only 6 of 56 judged rows — [memory queen-moved-to-trios L39](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md).
- After the backoff went live at 19:44Z, first 10.5 min — [t27#7851 comment 2026-10-08T19:57Z](https://github.com/gHashTag/t27/issues/7851):

  | verdict | rows | p50 | p95 | worker-s |
  |---|---|---|---|---|
  | sendBack | 8 | 49 s | 77 s | 329 |
  | escalate | 3 | 94 s | 99 s | 250 |
  | empty | 7 | 0.1 s | | |
  | wait | 117 | 0.0 s | | |

  - Rounds were back to one every ~31 s, and wait visits fell to about 11/min.
  - Utilization was 584 of 2520 worker-seconds (**23%**). The ceiling is about 240 reviews/h at ~60 s per review, against an inflow of about 60/h.
  - Criteria measurement took 4-7 s each.
- Reviewer spec measurements:
  - Before the separate reviewer (2026-10-08 afternoon): bees finished 47/h and the Queen reviewed 23/h, a backlog of 91. The in-round sweep did at most 3 reviews in at most 45 s (0.75 of a 60 s tick), with model calls of up to 2 min — [reviewer.t27 L3-9](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/reviewer.t27#L3-L9).
  - 2026-10-09: about 58 finished/h against about 11 reviewed, with 111 rows waiting. The fixed 4 slots at 300 s top out at 48/h — [reviewer.t27 L39-42](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/reviewer.t27#L39-L42).
- Queen lanes on 2026-10-09T06:38Z (`/queen/status`): 70 lanes, 23 active, 47 idle. The last round skipped fileConflict 236, missingBoundary 72 and claimed 61 — [actors.t27 L833-835](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L833-L835).

**Deploy and ops numbers:**
- Drain is 1800 s on all three services (RAILWAY_DEPLOYMENT_DRAINING_SECONDS=1830). Before the fix every merge stopped all rounds for about 30 min, and one morning had 5 such blackouts. A 120 s drain was tried for 5 min and reverted — [memory queen-deploy-does-not-stop-swarm L9-25](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-deploy-does-not-stop-swarm.md).
- "Each BrowserOS deploy freezes Queen rounds for about 30 min" — [memory queen-scheduler-control-6657 L57](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-scheduler-control-6657.md).
- Twice on 2026-10-08, rounds came 6 min apart instead of ~30 s (19:27-19:40Z and 20:18-20:24Z) — [trios#1707](https://github.com/gHashTag/trios/pull/1707).
- The first swarm release job ran 92 min, of which about 90 were spent waiting on the Actions queue (695 runs queued at peak) — [memory queen-moved-to-trios L50-55](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md).
- The event-bus long poll waits at most 25 s (WAIT_MAX_SECONDS), with a snapshot every 300 s — [events.t27 L121, L162](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/events.t27#L121).

### Inferences
- The benchmark measures the reviewer workload only, on a virtual clock. There is no wall-clock throughput, memory-per-actor or max-actor-count benchmark for the runtime.
- The 3.7-5.5 us per message is dominated by wasm card calls per decision. Several card calls happen per send and per step: `slot_of`, `reaches`, `deliver`, `next_lane`, `slice_spent`, `turn_signal`. This is inferred from the code at [queen-actors.ts L221-260, L337-363, L419-452](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors.ts#L221-L452).

### Gaps
- The 2026-10-08 19:57Z reading ("the reviewer is no longer the bottleneck", 23% utilization, ceiling ~240/h) conflicts with the 2026-10-09 spec reading (~11 reviewed/h vs ~58 finished/h, 111 waiting). No source reconciles them. A plausible cause is that most waiting rows were unreadable (trios#1706) rather than slot-bound; this is inferred, not confirmed.
- No memory footprint per actor or per worker thread. No max-actors-per-node figure, and no latency figure for PgLink cross-node mail.
- Dates mix UTC and +07:00. Example: t27#7851 says "The owner said yes in chat on 2026-10-09" in a comment stamped 2026-10-08T18:26Z, and trios#1702 says "44 minutes on 2026-10-09" in a PR merged 2026-10-08T19:07Z.

## Q6. Known pain points recorded in notes and code

### Takeaway
The recurring pains are:
- deploy drains that froze the swarm for ~30 min per merge;
- a hot loop when the actor reviewer went live without backoff;
- JS turns that can only be abandoned, not stopped (Bun `worker.terminate` included), so stalled work keeps holding rows and lanes;
- the fairness cost of slices;
- runtime-vs-spec drift: a stale spec header, an un-vendored adaptive concurrency spec and file owner, and an isolate comment that overclaims.

No OOM incident is recorded in the supplied notes.

### Cited Findings
- **Deploy drain blackouts.** Every Queen merge stopped all rounds for about 30 min (drain 1800 s; /workspace volumes prevent overlapping containers), with 5 blackouts in one morning. Fixed by moving the runners to the `queen-runners` branch and running rounds through the Queen's own drain (BrowserOS#546). The rule now is to batch merges, never shorten the drain, and expect up to 30 min to go live per merge — [memory queen-deploy-does-not-stop-swarm L9-25](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-deploy-does-not-stop-swarm.md).
- **Hot loop without backoff.** At go-live the actor reviewer hammered 22 unreadable rows about 184 times a minute, against about 20 for the loop. That stretched rounds to ~6 min and cut active bees to 22-27/70 — [t27#7851 comment 2026-10-08T19:57Z](https://github.com/gHashTag/t27/issues/7851); [memory queen-moved-to-trios L37-39](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md). The fix is `due_again` (30 s -> 900 s): without it "each one burned about 430 visits of the reviewer's slots" — [reviewer.t27 L110-138](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/reviewer.t27#L110-L138).
- **Unstoppable turns.** "Bun `worker.terminate()` does NOT stop a spinning worker thread. Only a process kill stops work" — [memory queen-moved-to-trios L43](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md). A killed review "still holds its z.ai lane until its 120 s timeout" — [t27#7851 comment 2026-10-08T17:51Z](https://github.com/gHashTag/t27/issues/7851). Concurrency can exceed 4 during stalls — [trios#1698](https://github.com/gHashTag/trios/pull/1698).
- **Slice cost.** 3.7 -> 5.5 us per message — [trios#1704](https://github.com/gHashTag/trios/pull/1704).
- **Round stalls of 6 min**, root cause pending phase timing — [trios#1707](https://github.com/gHashTag/trios/pull/1707).
- **Codegen and card constraints.** `t27c gen-js` emits only constants, so the host must mirror functions or call the wasm card — [memory queen-actors-beam L23](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-actors-beam.md). The vendored compiler wasm refuses byte-walk functions, and `loadControlSpec` has no fallback, so a bad card would stop all dispatch — [memory queen-scheduler-control-6657 L62](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-scheduler-control-6657.md). The gen-c bit-63 defect is behind the typed-local workaround — [actors.t27 L208-217](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L208-L217).
- **Process friction around the runtime:**
  - t27#7901 (sections 8/9 spec) was merged into a side branch by mistake and re-landed as #7907 — [memory queen-moved-to-trios L41](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md).
  - Until trios#1700, PRs into `queen` ran no checks — [memory L33](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md); #1700 merged 2026-10-08T20:17Z ([trios PR list](https://github.com/gHashTag/trios/pull/1700)).
  - gHashTag/trios `main` needs 1 approving review and has auto-merge off, so Queen code lives on branches — [memory L45](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-moved-to-trios.md).
- **Earlier dispatch-level failures** (pre-actor, the loop scheduler): 33 of 51 finished dispatches "ended unexpectedly (cause undetermined)", and "40 of 70 bees" idle was a lack of eligible issues, not a worker cap — [memory queen-scheduler-control-6657 L25-26](file:///Users/playom/.claude/projects/-Users-playom-t27/memory/queen-scheduler-control-6657.md).
- **Doc drift in code:** the spec header still describes a pre-runtime world ([actors.t27 L28-32](https://github.com/gHashTag/t27/blob/f21a0b39c3ea154142fa688dfec5d967e9183147/specs/queen/actors.t27#L28-L32)), and the isolate header claims `stop` "really ends the work" for threads ([queen-actors-isolate.ts L6-10](https://github.com/gHashTag/trios/blob/274db7adced39ad634966280d9f348a576b2dac3/trios/agent-server/apps/server/src/api/services/queen-actors-isolate.ts#L6-L10)), contrary to the spec and test.

### Inferences
- The actor runtime has not removed the dominant operational costs: the deploy drain, the Actions queue and unreadable rows. Those sit outside the runtime. The runtime's measurable production win so far is fault isolation of stalled reviews and bus-driven wakeups.
- Because state is in-process, every deploy (one per merge, up to 30 min apart) restarts the reviewer tree from scratch. That is harmless for the reviewer, which re-reads its queue from the DB, but it would matter for any future actor that holds non-reconstructible state.

### Gaps
- **OOM.** No out-of-memory incident, heap figure or memory limit is recorded in the five supplied memory notes or in the read code. The spec's `reviewer_concurrency` takes `free_mb` and `mb_per_review` as inputs, but no measured value for them was found.
- The cause of the 6-minute round gaps (trios#1707) was not found in the supplied sources; only the instrumentation was added.
