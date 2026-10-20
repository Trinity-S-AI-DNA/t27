# BEAM / OTP (Erlang, Elixir, Gleam) as an actor runtime, state of the art 2025-2026: what a small BEAM-inspired TypeScript runtime is most likely missing

Scope note: research done 2026-10-09. Primary docs are the OTP 29.1.1 documentation set on erlang.org (current at time of fetch), hexdocs for Elixir 1.20.x / Broadway 1.3.0 / libcluster 3.5.0 / recon 2.5.6 / gleam_otp 1.3.0. Older sources (Discord 2017, Partisan 2018, Ghaffari 2015, erlang-questions 2016) are marked as such.

Comparison baseline (our runtime, as given by the caller): pids with generations, bounded mailboxes that drop when full, a separate control lane, monitors, supervisors (strategies, restart intensity, exponential backoff with jitter, unstable-streak detection), per-turn time limit with abandonment of stuck turns, reduction-like slices (yield after 64 turns or 5 ms), optional Worker/process isolation, multi-node over Postgres LISTEN/NOTIFY with 5 s heartbeats and 20 s TTL, remote monitors that fire `noconnection`, simple placement.

---

## 1. Links vs monitors, trap_exit, exit-signal propagation; why OTP supervision is built on links

### Takeaway
Links are bidirectional "fate-sharing" edges whose exit signals kill the peer unless it traps exits; monitors are unidirectional, stackable observation edges that only deliver a `'DOWN'` message. OTP supervision is built on links (child start functions must create and link the child; supervisors exit with the same reason to linked processes), which gives two-way failure propagation: a child dies if its supervisor dies, so no orphans survive a subtree restart. Our runtime has monitors and supervisors but (as described) no links / trap_exit, no "die-together" groups, and likely no ordered, timed shutdown protocol.

### Cited Findings
- Links: "Links are bidirectional and there can only be one link between two processes"; repeated `link()` calls have no effect; either side can create or remove the link. On termination a linked process sends an exit signal carrying its exit reason; the reason is `noproc` if the link target did not exist and `noconnection` if a distributed connection was lost or could not be established -- [Erlang Reference Manual: Processes (OTP 29.1.1)](https://www.erlang.org/doc/system/ref_man_processes.html)
- Exit-signal reception rules (same source): a signal with reason `normal` is silently dropped if the receiver is not trapping exits; a non-trapping receiver that gets any other reason terminates with that same reason; a non-link exit signal with reason `kill` terminates the receiver with reason `killed` and cannot be trapped; a trapping receiver gets the signal converted to a message `{'EXIT', From, Reason}`. Irregularity: a `kill` reason received via a link is NOT converted to `killed` and CAN be trapped, while `exit(Pid, kill)` cannot -- [Erlang Reference Manual: Processes](https://www.erlang.org/doc/system/ref_man_processes.html)
- Monitors: unidirectional; "Repeated calls to `erlang:monitor(process, Pid)` create several independent monitors, and each one sends a 'DOWN' message"; message shape `{'DOWN', Ref, process, Pid2, Reason}`; if the target does not exist the DOWN is sent immediately with reason `noproc`; monitors can target registered names, including on other nodes -- [Erlang Reference Manual: Processes](https://www.erlang.org/doc/system/ref_man_processes.html)
- `monitor/3` options type: `{alias, explicit_unalias | demonitor | reply_demonitor} | {tag, term()} | priority` -- [erlang module docs](https://www.erlang.org/doc/apps/erts/erlang.html)
- Aliases (OTP 24+): created with `alias/0,1` or implicitly by `monitor/3` with `{alias, _}`; "When the alias has been deactivated, messages sent using the alias will be dropped before entering the message queue"; already-queued messages are not removed; only the creator can deactivate its alias -- [Erlang Reference Manual: Processes](https://www.erlang.org/doc/system/ref_man_processes.html)
- Signal ordering: "if an entity sends multiple signals to the same destination entity, the order is preserved" -- guaranteed only per sender/receiver pair; delay is "unspecified but positive"; signals from different runtime services (timers, name service, spawn) to one process have no mutual order guarantee -- [Erlang Reference Manual: Processes](https://www.erlang.org/doc/system/ref_man_processes.html)
- BEAM has the same "cannot kill a stuck native computation" problem we have with JS promises: a process in a dirty NIF that receives an exit signal enters an exiting state, but "the runtime system cannot force the native code to stop executing" -- [Erlang Reference Manual: Processes](https://www.erlang.org/doc/system/ref_man_processes.html)
- Supervisor defaults: `intensity` defaults to 1 and `period` to 5 (seconds); if more than MaxR restarts occur in MaxT seconds the supervisor terminates all children and then itself with reason `shutdown` (escalation to the parent) -- [supervisor (stdlib, OTP 29.1.1)](https://www.erlang.org/doc/apps/stdlib/supervisor.html)
- Strategies: `one_for_one` (default), `one_for_all`, `rest_for_one`, `simple_one_for_one`. Restart types: `permanent` (default), `transient` (restart only if exit reason is not `normal`, `shutdown`, or `{shutdown, Term}`), `temporary` (never restarted) -- [supervisor docs](https://www.erlang.org/doc/apps/stdlib/supervisor.html)
- Shutdown protocol: `brutal_kill` = unconditional `exit_signal(Child, kill)`; integer = send `shutdown` exit signal, then kill if no exit within N ms; `infinity` expected for supervisor children. Default 5000 ms for workers, `infinity` for supervisors. Children start left to right and terminate in reverse order (right to left); `simple_one_for_one` children stop asynchronously in undefined order -- [supervisor docs](https://www.erlang.org/doc/apps/stdlib/supervisor.html)
- Significant children / `auto_shutdown` (OTP 24+): `never` (default), `any_significant`, `all_significant` -- lets a supervisor shut itself down when a significant transient/temporary child ends (useful for "job" subtrees) -- [supervisor docs](https://www.erlang.org/doc/apps/stdlib/supervisor.html)
- Why links: the child start function "must create and link to" the child, `start_link` links the supervisor to its caller, and a supervisor stopping with a non-normal reason sends an exit signal with the same reason to linked processes -- [supervisor docs](https://www.erlang.org/doc/apps/stdlib/supervisor.html)
- OTP 28: `erlang:link/2` and `erlang:monitor/3` accept a `priority` option so that DOWN/EXIT signals are delivered as priority messages (ahead of the ordinary backlog) -- [Erlang/OTP 28 Highlights](https://www.erlang.org/blog/highlights-otp-28/)

### Inferences
- Gap 1 for us: links + trap_exit. Without bidirectional links, a supervisor crash (or a parent actor crash) leaves children running as orphans; monitors alone only notify one direction. A minimal port: `link(a,b)` stored on both sides, exit-signal delivery with the 3-way rule (normal/drop, other/kill-unless-trapping, kill/untrappable), and `trapExit` flag that turns signals into control-lane messages.
- Gap 2: the shutdown protocol (shutdown signal -> wait N ms -> brutal kill; reverse start order; supervisors wait `infinity` for subtrees). This is what lets a BEAM node stop cleanly and is the basis of graceful drain (see Q6). Our "abandon stuck turn" is the analogue of brutal_kill; we should make the per-child shutdown budget explicit and ordered.
- Gap 3: `transient` vs `temporary` vs `permanent` semantics and `{shutdown, Term}` as "expected exit" -- our backoff/unstable-streak detection is richer than OTP's (OTP has no backoff; it escalates after intensity is exceeded), but the restart-type taxonomy is cheap to add and important for job-like actors.
- Our separate control lane already matches OTP 28's design decision to let monitor/link signals bypass a long data backlog; worth confirming that our DOWN/EXIT notifications travel on the control lane.
- The dirty-NIF note shows BEAM itself accepts "zombie native work"; our Worker/process isolation is the right escape hatch (BEAM's equivalent is running foreign code in a port/OS process).

### Gaps
- Elixir `Supervisor` defaults (commonly `max_restarts: 3`, `max_seconds: 5`) were not verified against current Elixir docs in this pass.
- No primary source found that states the design rationale ("why links, not monitors") in prose; the rationale above is inferred from documented mechanics.

---

## 2. Process registry and naming: local registry, global, pg groups, Elixir Registry, gproc; pub/sub on top of pg

### Takeaway
BEAM layers naming: local atom registration (per node), `global` (cluster-wide, strongly consistent, lock-based, replicated on every node, with pluggable conflict resolution after netsplits), `pg` (OTP 23+, scoped, strongly eventually consistent process groups without a leader or locks, with subscribe-able membership since OTP 25.1), and Elixir's `Registry` (local, partitioned, ETS-backed, supports unique names via `{:via, ...}` and duplicate keys for local pub/sub). Phoenix.PubSub uses distribution to broadcast cluster-wide. A small runtime most likely lacks: scoped groups with join/leave notifications, via-style pluggable naming, and an explicit name-conflict resolution policy after a partition.

### Cited Findings
- `global`: "The registered names are stored in replica global name tables on every node"; lookups are local; registration "is completely synchronous, that is, when this function returns, the name is either registered on all nodes or none"; registration and lock services are atomic -- [global (kernel, OTP 29.1.1)](https://www.erlang.org/doc/apps/kernel/global.html)
- `global` name clash after netsplit: "If any name clashes are discovered, function `Resolve` is called"; built-ins `random_exit_name/3` (keep one at random, kill the other), `random_notify_name/3` (keep one, send `{global_name_conflict, Name}` to the other), `notify_all_name/3` (unregister both, notify both with `{global_name_conflict, Name, OtherPid}`); if the resolver crashes or returns a non-pid the name is unregistered -- [global docs](https://www.erlang.org/doc/apps/kernel/global.html)
- `global` locks: locks are released when the holder dies or its node goes down; the docs warn that taking two or more locks can deadlock and "This function does not address the problem of a deadlock" -- [global docs](https://www.erlang.org/doc/apps/kernel/global.html)
- `pg` (functions tagged "since OTP 23.0"): groups organised into independent scopes; default scope `pg` started if kernel `start_pg` is set; "Process Groups implement strong eventual consistency"; views "may temporarily diverge"; "Membership view is not transitive" (nodes not directly connected do not see each other's groups) -- [pg (kernel, OTP 29.1.1)](https://www.erlang.org/doc/apps/kernel/pg.html)
- `pg` semantics: a process may join a group several times and must leave the same number of times; "It is only allowed to join processes running on local node"; members are removed automatically when they terminate; "Local membership is not preserved if scope process exits and restarts"; no leader/lock is documented -- [pg docs](https://www.erlang.org/doc/apps/kernel/pg.html)
- `pg:monitor_scope/1` and `pg:monitor/2` (since OTP 25.1) return current membership plus a ref and then stream `{Ref, join, Group, Pids}` / `{Ref, leave, Group, Pids}` -- [pg docs](https://www.erlang.org/doc/apps/kernel/pg.html)
- Elixir `Registry`: "A local, decentralized and scalable key-value process storage"; `:unique` or `:duplicate` keys; `partitions` default 1, recommended `System.schedulers_online()` under heavy concurrency; `{:via, Registry, {registry, key}}` naming for unique registries, returning `{:error, {:already_started, pid}}` on conflict; `dispatch/3` runs in the caller and is described as "local, non-distributed, scalable PubSub"; keys of a crashed process are removed automatically, but "Looking up, dispatching and registering are efficient and immediate at the cost of delayed unsubscription" (lookups may return dead pids); `:listeners` get register/unregister events but must monitor to see crashes -- [Elixir Registry (v1.20.4)](https://elixir.hexdocs.pm/Registry.html)
- Phoenix.PubSub: default adapter `Phoenix.PubSub.PG2` "uses Distributed Elixir, directly exchanging notifications between servers"; `pool_size` = number of pubsub partitions, default one per 4 cores, and "must be the same throughout the cluster"; "Duplicate subscriptions for a Pid/topic pair are allowed and will cause duplicate events to be sent"; no formal delivery guarantee stated -- [Phoenix.PubSub docs](https://phoenix-pubsub.hexdocs.pm/Phoenix.PubSub.html)
- Horde.Registry (distributed, delta-CRDT) "terminates duplicate processes as soon as they are discovered with a special exit code" -- [Horde README](https://horde.hexdocs.pm/readme.html)
- Discord (2017, older) fan-out lesson: a single `send/2` cost 30-70 us of wall-clock time because the caller was descheduled; publishing from a 30,000-member guild took 900 ms to 2.1 s at peak; fix (Manifold) groups target pids by remote node and calls `send/2` at most once per node, with a per-node partitioner doing local fan-out -- [Discord blog, 2017-07-06](https://discord.com/blog/how-discord-scaled-elixir-to-5-000-000-concurrent-users)

### Inferences
- What we probably lack: (a) a local name registry with unique and duplicate keys and automatic cleanup on death (we have pids with generations, which protects against stale-pid reuse but not stable names); (b) `pg`-style scoped groups with join/leave subscription (`monitor_scope`) -- this is the BEAM primitive for service discovery and pub/sub; (c) a documented conflict-resolution policy for cluster-unique names after a partition (global's resolve hooks, Horde's kill-duplicate).
- Our Postgres LISTEN/NOTIFY transport is effectively a hub, which naturally gives Manifold's "one message per node, fan out locally" shape if broadcast payloads are addressed to a group and each node fans out to local members. Avoid per-recipient NOTIFYs.
- `pg` deliberately avoids locks and a leader; a Postgres table can give us `global`-style strong consistency cheaply (unique constraint), but we must then decide what happens to the losing actor on reconnect (kill vs notify), as global does.
- Registry's "delayed unsubscription" trade-off is a useful explicit design note: fast lookup may return a dead pid; our generation check on send already handles that case.

### Gaps
- gproc (Ulf Wiger's extended registry, with properties/counters/aggregated counters) was not researched in this pass; no current-status source fetched.
- The exact mechanism by which Phoenix.PubSub.PG2 uses `:pg` internally was not confirmed from its docs (page does not mention `:pg`).
- pg's netsplit behaviour is not described in its docs; by its eventual-consistency model each side keeps its own view and merges on reconnect, but this is inference.

---

## 3. Timers, selective receive and its costs, hibernation, off-heap message queues, per-process memory limits

### Takeaway
BEAM timers are plain messages (`send_after`, `start_timer` -> `{timeout, Ref, Msg}`, `receive ... after`); selective receive scans the queue from the start and is O(queue) unless the compiler can use the ref-tagged request/reply optimisation; idle processes can hibernate (OTP 28 added a stack-preserving `erlang:hibernate/0` that cut memory ~69% for 1M idle processes in the official benchmark); processes can opt into off-heap message queues and a per-process `max_heap_size` kill switch. BEAM has no bounded mailbox -- mailboxes are unbounded by design, and overload is handled by monitoring and policies, not by the VM.

### Cited Findings
- Process footprint: "A newly spawned Erlang process uses 327 words of memory" (2616 bytes on 64-bit), including 233 words of heap+stack; `min_heap_size` per process via `spawn_opt` can avoid GCs for short-lived workers, "not to be attempted without proper measurements" -- [Efficiency Guide: Processes](https://www.erlang.org/doc/system/eff_guide_processes.html)
- Selective receive cost: "the entire message queue must be searched until it finds a matching message"; the `monitor/2`/`make_ref/0` request-reply pattern lets the compiler "search only the messages that arrived after the call to monitor/2"; check with `+recv_opt_info` -- [Efficiency Guide: Processes](https://www.erlang.org/doc/system/eff_guide_processes.html)
- Message copying: "All data in messages sent between Erlang processes is copied," except refc binaries and literals on the same node -- [Efficiency Guide: Processes](https://www.erlang.org/doc/system/eff_guide_processes.html)
- Timers: `start_timer` sends `{timeout, TimerRef, Msg}` on expiry; `send_after` works exactly like it except for the message format; as of OTP 28.0 a suspended process's receive-timeout timer is also suspended, and as of OTP 29.0 BIF timers created via the suspendee's pid are also suspended -- [erlang module docs](https://www.erlang.org/doc/apps/erts/erlang.html)
- `gen_server:call/2` default timeout is 5000 ms; on timeout the caller exits with `timeout`; "Starting with OTP 24, gen_server:call uses process aliases, so late replies will not be received" (before OTP 24 callers had to flush garbage replies) -- [gen_server (stdlib, OTP 29.1.1)](https://www.erlang.org/doc/apps/stdlib/gen_server.html)
- Async request API: `send_request/2` (OTP 23), `receive_response/2` that abandons the request on timeout (OTP 24), request-id collections `reqids_new/0`, `reqids_add/3` etc. (OTP 25) for many outstanding requests -- [gen_server docs](https://www.erlang.org/doc/apps/stdlib/gen_server.html)
- Hibernation: `hibernate_after` start option hibernates automatically after an idle timeout; returning `hibernate` from a callback hibernates until the next message; docs warn hibernation "implies at least two garbage collections" -- [gen_server docs](https://www.erlang.org/doc/apps/stdlib/gen_server.html)
- OTP 28 `erlang:hibernate/0`: minimises memory without discarding the call stack (unlike `hibernate/3`); official benchmark memory (MB, hibernated vs not): 10,000 procs 55.5 vs 73.4; 100,000 procs 130.3 vs 307.1; 1,000,000 procs 827.9 vs 2687.1. The blog text says "more than 75%" reduction; the table itself gives about 69% -- [Erlang/OTP 28 Highlights](https://www.erlang.org/blog/highlights-otp-28/)
- `max_heap_size` accepts a map `#{size, kill, error_logger, include_shared_binaries}` -- [erlang module docs](https://www.erlang.org/doc/apps/erts/erlang.html). Older context (2016, OTP team's Lukas Larsson): size is in words; with `kill` the process exits with reason `killed` without a costly GC; by default queued messages count toward the limit, and `off_heap` message queues stop counting them; recommendation is to treat it as a runaway guard (run with `kill=false` first, observe, then set high), and to use `message_queue_data = off_heap` for processes that may build huge queues -- [erlang-questions, April 2016](https://erlang.org/pipermail/erlang-questions/2016-April/088988.html)
- `process_flag(message_queue_data, off_heap | on_heap)` is the per-process queue placement knob -- [erlang module docs](https://www.erlang.org/doc/apps/erts/erlang.html)

### Inferences
- Our bounded mailbox with drop-on-full is stricter than BEAM (which never bounds mailboxes). That is a good default for a JS runtime without per-process GC, but we probably lack BEAM's complementary tools: a per-actor memory/size guard that kills (like `max_heap_size`), and an idle "hibernate" that releases actor state caches.
- Request/reply: copy OTP 24's alias trick -- each call gets a one-shot reply address that is deactivated on timeout, so late replies are dropped before enqueue instead of polluting the mailbox. Add a multi-request collection API (`reqids`) for fan-out calls with one deadline.
- Timers: expose `sendAfter(pid, msg, ms) -> ref` and `cancelTimer(ref)` that deliver as ordinary messages through the mailbox (so they obey ordering and backpressure), plus a receive-timeout per actor turn. Note OTP 28/29's decision that suspended processes also suspend their timers.
- Selective receive is a cost we may not want; if we have a single-handler-per-message model we avoid the O(n) scan entirely; the only selective pattern worth supporting is "await reply with ref", which is exactly what the BEAM compiler optimises.

### Gaps
- Current docs text for `message_queue_data` default (believed `on_heap`) and the OTP version that introduced it (believed OTP 19) were not retrieved from the current erlang module page (truncated fetch); flagged as unverified.
- No current measurement found for selective-receive cost per queued message.

---

## 4. Backpressure in BEAM land: GenStage, Broadway, demand-driven flow, mailbox overload protection

### Takeaway
BEAM's core primitives have no backpressure (send never blocks locally, mailboxes are unbounded); backpressure is built in libraries: synchronous calls (bounded by caller count), demand-driven pull (GenStage/Broadway: consumers ask for N events, producers never emit more), fail-fast in-flight counters (Discord's Semaphore), and tiered overload modes in OTP's own `logger` (async -> sync -> drop -> flush, plus optional kill-and-restart). OTP 27/28 added runtime support: a `long_message_queue` system monitor and priority messages so an overloaded process can still receive control signals.

### Cited Findings
- GenStage: "Once demand arrives, the producer will emit events, never emitting more events than the consumer asked for"; "The consumer will never ask the producer for more than `:max_demand` events at a time" and re-asks at the `:min_demand` low-water mark; producers "must buffer the demand until data arrives"; producer `:buffer_size` default 10_000 (`:infinity` for producer_consumer); `:buffer_keep` default `:last`; on overflow "an error is logged" and discarded items are reported through `format_discarded/2`; default dispatcher `DemandDispatcher`, also `BroadcastDispatcher`; events in a consumer's `handle_events/3` when it crashes "will be lost" -- [GenStage docs](https://gen-stage.hexdocs.pm/GenStage.html)
- Broadway (v1.3.0): "we only get the amount of events necessary from upstream sources, never flooding the pipeline"; processors default concurrency `System.schedulers_online() * 2`, `max_demand` 10, `min_demand` 5; batchers `batch_size` 100, `batch_timeout` 1000 ms; producer-level `:rate_limiting` (`allowed_messages` per `interval` ms, shared by all producers on a node, updatable at runtime); `:shutdown` default 30000 ms to "gracefully shutdown without discarding events"; automatic ack at end of pipeline, failed messages acked as failed immediately, no built-in retry (producer's job); `partition_by` guarantees in-order processing within a partition but a slow partition slows the whole pipeline -- [Broadway docs](https://broadway.hexdocs.pm/Broadway.html)
- OTP logger overload protection defaults: `sync_mode_qlen` 10 (below this, logging is async; above, callers wait for the handler), `drop_mode_qlen` 200 (new events dropped), `flush_qlen` 1000 (queue drained without logging), `burst_limit_enable` true with 500 events per 1000 ms, `overload_kill_enable` false, `overload_kill_qlen` 20000, `overload_kill_mem_size` 3,000,000 bytes, `overload_kill_restart_after` 5000 ms -- [Logging chapter, kernel](https://www.erlang.org/doc/apps/kernel/logger_chapter.html)
- OTP 27 added a `long_message_queue` system monitor (`{Disable, Enable}` thresholds) but its notification arrives as a message at the end of the monitoring process's own long queue; OTP 28 priority messages (EEP 76) fix this: receiver opts in with `alias([priority])`, sender uses `erlang:send(Alias, Msg, [priority])`; priority messages are inserted ahead of ordinary ones but keep arrival order among themselves; "no performance penalty" claimed; `unalias/1` disables -- [Erlang/OTP 28 Highlights](https://www.erlang.org/blog/highlights-otp-28/); system_monitor option list incl. `long_message_queue`, `long_schedule`, `large_heap`, `long_gc`, `busy_port`, `busy_dist_port` -- [erlang module docs](https://www.erlang.org/doc/apps/erts/erlang.html)
- The reference manual cautions: "Priority messages are intended to solve very specific problems"; large volumes are not optimised; with priority messages, messages from one sender can be reordered relative to each other -- [Erlang Reference Manual: Processes](https://www.erlang.org/doc/system/ref_man_processes.html)
- Distribution backpressure: a sender can block when the distribution buffer reaches its busy limit; `process_flag(async_dist, true)` makes signalling fully asynchronous but then flow control is the caller's responsibility -- [Erlang Reference Manual: Processes](https://www.erlang.org/doc/system/ref_man_processes.html)
- Discord (2017, older): ~5 million session processes calling ~10 guild-registry processes; timed-out requests stayed queued, retries piled up, queues grew, VMs ran out of memory (cascading outages). They rejected a circuit breaker and used a Semaphore: an ETS atomic counter (`:ets.update_counter/4`) of in-flight requests; above the limit the session fails fast without calling -- [Discord blog, 2017-07-06](https://discord.com/blog/how-discord-scaled-elixir-to-5-000-000-concurrent-users)

### Inferences
- Our drop-when-full mailbox corresponds roughly to logger's `drop_mode`; what we likely lack is the graduated policy: (1) a soft threshold at which `send` switches to a blocking/awaitable send for callers that can wait (logger's `sync_mode`), (2) a hard drop threshold, (3) a flush threshold, (4) optional "kill and restart after N ms" when queue length or memory exceeds a bound -- with hysteresis (OTP's `{Disable, Enable}` pair).
- A demand-driven stage abstraction (producer/consumer with `maxDemand`/`minDemand`, buffer with keep-first/keep-last, partitioned dispatch, rate limit, graceful drain) is the single most-used backpressure tool in Elixir production and is absent from most small runtimes.
- Discord's semaphore is trivially portable: an atomic in-flight counter per target, fail fast above N; it prevents the retry-storm failure mode that bounded mailboxes alone do not.
- Our control lane already plays the role of OTP 28 priority messages; OTP's experience says keep it low-volume and make DOWN/EXIT/system signals optionally use it.
- Expose a "mailbox long" event (threshold + hysteresis) on the control lane / telemetry rather than requiring polling.

### Gaps
- No primary source fetched on WhatsApp's current (2025-2026) overload practices.
- No 2025-2026 Dashbit post on backpressure was found in this pass.

---

## 5. Distribution: failure detection (net_ticktime), partitions, global/pg on netsplit, libcluster, Horde, Partisan, limits of full mesh

### Takeaway
Distributed Erlang is a transitive full mesh over one TCP connection per node pair, with tick-based failure detection that by default declares a peer down after 45-75 s (net_ticktime 60, intensity 4). `global` enforces a fully connected network and, since OTP 25, actively prevents overlapping partitions; `pg` is eventually consistent and non-transitive. Practical full-mesh limits are commonly cited at ~60-200 nodes (older sources). Ecosystem tools fill gaps: libcluster (discovery only), Horde (delta-CRDT supervisor and registry, AP, duplicates possible, optional quorum shutdown), Partisan (alternative distribution with pluggable topologies and multiple channels).

### Cited Findings
- `net_ticktime` default 60 s, "the approximate time a connected node may be unresponsive until it is considered down"; `net_tickintensity` default 4 (range 4..1000); a node is down "if no ticks or payload packages have been received during the last NetTickIntensity number of TickInterval seconds intervals" with TickInterval = NetTickTime / NetTickIntensity; detection time between NetTickTime - NetTickTime/N and NetTickTime + NetTickTime/N, i.e. 45 < T < 75 s with defaults -- [kernel app docs (OTP 29.1.1)](https://www.erlang.org/doc/apps/kernel/kernel_app.html)
- `net_setuptime` default 7 s (max 120); `connect_all` default true (global actively connects to every node it learns of -> transitive mesh); `dist_auto_connect` unset = always auto-connect, alternatives `never` / `once` -- [kernel app docs](https://www.erlang.org/doc/apps/kernel/kernel_app.html)
- `global` "maintains a fully connected network"; "As of OTP 25, `global` will by default prevent overlapping partitions due to network issues"; "you are _strongly_ advised _not_ to disable this fix"; must be enabled on all nodes -- [global docs](https://www.erlang.org/doc/apps/kernel/global.html)
- Monitors/links over distribution fire with reason `noconnection` when the connection is lost -- [Erlang Reference Manual: Processes](https://www.erlang.org/doc/system/ref_man_processes.html)
- `pg`: strong eventual consistency, non-transitive membership views (see Q2) -- [pg docs](https://www.erlang.org/doc/apps/kernel/pg.html)
- libcluster v3.5.0: "Automatic cluster formation/healing" with pluggable strategies -- Epmd, LocalEpmd, ErlangHosts, Gossip (multicast UDP heartbeat), Kubernetes (metadata API by label selector), Kubernetes.DNS (headless service), Rancher; uses Distributed Erlang by default, but connect/disconnect/list_nodes callbacks allow alternatives such as Partisan -- [libcluster README](https://libcluster.hexdocs.pm/readme.html)
- Horde: `Horde.DynamicSupervisor` and `Horde.Registry` built on DeltaCrdt; "eventually (as opposed to immediately) consistent"; "Horde cannot guarantee consistency. This means you may end up with duplicate processes in your cluster"; `UniformDistribution` (default, hash-based, "Both sides of the partition will continue to operate") vs `UniformQuorumDistribution` ("will shut down if less than half of the cluster is reachable"); membership via `Horde.Cluster.set_members/2`; on `stop/3` or `:init.stop()` local processes are redistributed; recommends Highlander/HighlanderPG for a single global process -- [Horde README](https://horde.hexdocs.pm/readme.html)
- Partisan (2018 paper, older): distributing traffic over multiple connections performs "up to 18x better under normal conditions, and up to 30x better" under congestion; supports full-mesh, peer-to-peer, client-server and pub/sub overlays; the README cites a practical limit of 60-200 connected nodes for distributed Erlang; Erlang "conflates control plane messages with application messages on the same TCP/IP connection", causing head-of-line blocking -- [Partisan paper, arXiv 1802.02652](https://arxiv.org/pdf/1802.02652) (README figures relayed via search summary; not peer-reviewed)
- Ghaffari (2015 thesis, older): distributed Erlang scales linearly up to ~150 nodes / 1200 cores without global commands; with 0.01% global commands scalability limited to ~60 nodes -- [Ghaffari PhD thesis, Glasgow 2015](https://theses.gla.ac.uk/6789/1/2015ghaffariphd.pdf)
- Full mesh implies O(n^2) connections and per-VM bookkeeping quadratic in node count (Chechina et al., via search summary) -- [Partisan paper](https://arxiv.org/pdf/1802.02652)

### Inferences
- Our 5 s heartbeat / 20 s TTL detects failures 2-4x faster than BEAM's defaults (45-75 s). BEAM's tick also counts any payload as liveness, which avoids false positives under load; worth copying ("any NOTIFY from node X refreshes X's lease").
- Our Postgres hub is not a full mesh, so we avoid the O(n^2) problem but inherit a SPOF and a single control/data channel (Partisan's head-of-line-blocking critique applies to a single LISTEN connection carrying both heartbeats and payload). Consider separating heartbeat/control from data channels.
- Missing vs BEAM land: (a) a partition policy for singletons/names -- BEAM's menu is global's resolve functions (kill/notify one/notify all) and Horde's quorum mode (minority side shuts down); (b) distributed supervision with handoff on graceful node stop (Horde); (c) pluggable discovery (libcluster-style strategies) if we ever go beyond Postgres membership.
- Because Postgres can provide linearizable leases, we can do better than `global` on singletons (a row lock / advisory lock with TTL fencing) -- but we must define what the old owner does when it reconnects (BEAM's `{global_name_conflict, Name}` message is a good shape).

### Gaps
- Exact OTP release that introduced `prevent_overlapping_partitions` as default is stated as "As of OTP 25" in global docs; kernel docs did not give the version.
- No 2025-2026 production report found on maximum distributed-Erlang cluster sizes; the 60-200 figure is old/secondhand.
- Partisan's current maintenance status (5.0.0 betas on hex) was not verified.
- Distribution buffer busy limit default size (`+zdbbl`) not retrieved.

---

## 6. Hot code loading / release upgrades vs blue-green; deploying without dropping in-flight work

### Takeaway
BEAM can hold two versions of a module (current and old); fully qualified calls jump to the new version; loading a third version purges the old one and kills processes still running it. OTP releases (appup/relup + `code_change/3`) automate suspend -> load -> migrate state -> resume, but the community treats them as powerful and fragile; most teams do rolling or blue-green deploys of immutable releases and rely on supervisor shutdown ordering and pipeline drain (e.g. Broadway's 30 s graceful shutdown). A 2025 trend is narrow hot-upgrade tooling (Chris McCord's FlyDeploy, Nov 2025) with cold-deploy fallback.

### Cited Findings
- "The code of a module can exist in two variants in a system: current and old"; "If a third instance of the module is loaded, the code server removes (purges) the old code and any processes lingering in it are terminated"; "Fully qualified function calls always refer to current code" and a process must make a fully qualified call to switch -- [Erlang System Docs: Compilation and Code Loading](https://www.erlang.org/doc/system/code_loading.html)
- `code_change/3` is called during release upgrade/downgrade per the appup; returning `{error, Reason}` makes the ongoing upgrade fail and roll back -- [gen_server docs](https://www.erlang.org/doc/apps/stdlib/gen_server.html)
- FlyDeploy (Chris McCord, Fly.io community, 2025-11-06): `mix fly_deploy.hot`; builds a new image, uploads BEAM files, then running machines run `FlyDeploy.hot_upgrade/2`, which "detects changed code, suspends affected processes, reloads their code, runs their `code_change` callbacks, and resumes them"; re-applies upgrades after cold restarts; "fallback to `fly deploy` whenever you change deps, supervision tree structure, or elixir versions"; described as "limited compared to true OTP releases" -- [Fly.io community post](https://community.fly.io/t/hot-code-upgrades-for-elixir-and-phoenix-apps/26389)
- Failure modes reported by practitioners (older forum threads): brutal purge killing workers still in old code; bare Tasks and hand-rolled loops are not safe to hot-reload because they are not OTP behaviours; `release_handler` install timeouts restarting the whole supervision tree -- [Elixir Forum: exrm upgrade kills worker](https://elixirforum.com/t/exrm-upgrade-kills-worker/1553); [Elixir Forum: release handler crashed during hot reload](https://elixirforum.com/t/release-handler-crashed-during-hot-reload/42536)
- Graceful drain primitives: Broadway `:shutdown` default 30000 ms so the pipeline can stop "without discarding events" and, under a supervisor, "guarantee all events are flushed once the VM shuts down" -- [Broadway docs](https://broadway.hexdocs.pm/Broadway.html); supervisor reverse-order termination with per-child shutdown timeouts -- [supervisor docs](https://www.erlang.org/doc/apps/stdlib/supervisor.html); Horde redistributes a node's children on `stop/3` / `:init.stop()` -- [Horde README](https://horde.hexdocs.pm/readme.html)

### Inferences
- For a TS/Bun runtime, true hot code loading is not worth copying; the transferable parts are (a) ordered, timed shutdown of the supervision tree, (b) explicit drain of in-flight work (stop accepting, finish or hand back, ack), (c) state handoff to another node on graceful stop (Horde-style), and (d) a `codeChange(oldState, fromVersion)` hook used when an actor is re-created on a new deploy and state is persisted.
- "Supervision tree structure changes require a cold deploy" is a good rule to adopt verbatim for any rolling deploy that migrates actors between versions.

### Gaps
- No recent (2025-2026) large-scale production write-up comparing relups vs blue-green was found; the "most teams use rolling/blue-green" claim rests on course material and forum evidence, not a survey.

---

## 7. Observability: observer, recon, sys tracing, telemetry, OpenTelemetry; process_info

### Takeaway
BEAM exposes per-process introspection (`process_info`: message_queue_len, reductions, memory, current function, label) and system monitors, and the ecosystem wraps it in production-safe tools (recon: top-N by attribute, sliding windows, binary-leak detection, scheduler utilisation). Since OTP 27, multiple isolated trace sessions and process labels exist. `telemetry` is the de-facto event bus (synchronous handlers, `span/3` convention) and OpenTelemetry Erlang traces are stable while metrics/logs are still in development.

### Cited Findings
- `process_info` result items include `{message_queue_len, N}`, `{reductions, N}`, `{label, Term}`, `{trap_exit, Bool}` -- [erlang module docs](https://www.erlang.org/doc/apps/erts/erlang.html)
- OTP 27: `proc_lib:set_label/1` / `get_label/1` attach an arbitrary label to non-registered processes, shown in shell `i/0`, observer, and crash dumps; SSH and SSL label their processes; multiple trace sessions via the new `trace` module (`trace:session_create/3`, `trace:process/4`, `trace:function/4`, `trace:session_destroy/1`), each with its own tracer, while old `erlang:trace/3` users share one global session; `tprof` with `call_count`, `call_time`, `call_memory`; native JIT coverage (`+JPcover`) -- [Erlang/OTP 27 Highlights](https://www.erlang.org/blog/highlights-otp-27/)
- recon (v2.5.6): `info/1` is production-safe and "excludes fields such as the mailbox, which have a tendency to grow and be unsafe"; `proc_count/2` returns the biggest N consumers of an attribute (memory, reductions, ...); `proc_window/3` samples deltas over a time window (catches short-lived hot processes; warns of memory cost with tens of thousands of processes); `bin_leak/1` GCs each process and reports those that freed the most refc binaries; `scheduler_usage/1` uses scheduler wall time (more reliable than `top`); `get_state/1,2` default 5000 ms -- [recon docs](https://recon.hexdocs.pm/recon.html)
- `gen_server:format_status/1` (OTP 25) can redact state/message/reason/log for `sys:get_status` and crash logs -- [gen_server docs](https://www.erlang.org/doc/apps/stdlib/gen_server.html)
- telemetry: "a lightweight library for dynamic dispatching of events, with a focus on metrics and instrumentation"; handlers are "invoked synchronously on each `telemetry:execute` call", so "it is extremely important to avoid blocking operations"; `span/3` emits start and stop/exception events with `system_time`, `duration`, and `monotonic_time` in native units; tooling-agnostic, bridged to OpenTelemetry via opentelemetry_telemetry -- [telemetry README](https://telemetry.hexdocs.pm/readme.html)
- Broadway's telemetry event names as an example of the convention: `[:broadway, :processor, :message, :start | :stop | :exception]`, `[:broadway, :batcher, :start]`, etc. -- [Broadway docs](https://broadway.hexdocs.pm/Broadway.html)
- OpenTelemetry Erlang/Elixir status: Traces Stable, Metrics Development, Logs Development; instrumentation in opentelemetry-erlang-contrib (Phoenix, Ecto, ...) -- [OpenTelemetry Erlang/Elixir docs](https://opentelemetry.io/docs/languages/erlang/)
- `sys` / OTP debug protocol is what lets observer and debugging APIs talk to behaviours; Gleam's actor "handles OTP's system messages automatically", though not all system messages are supported yet -- [gleam_otp docs](https://gleam-otp.hexdocs.pm/)

### Inferences
- Minimum observability surface our runtime should expose per actor (BEAM parity): queue length, turns/"reductions" counted, memory estimate, current state of the turn (running/waiting/abandoned), label, parent/links/monitors, restart count. Plus node-level: run-queue lengths and slice utilisation (scheduler_usage analogue).
- Add "top-N by attribute" and "top-N by delta over a window" queries (recon's `proc_count` / `proc_window`) -- these are the first commands BEAM operators run in an incident.
- Add a status hook with redaction (`format_status`) and a safe `getState(pid, timeout)` over the control lane.
- Telemetry events should be synchronous-cheap with a `span` convention (start/stop/exception, monotonic durations); OTel bridging can wait. Process labels (OTP 27) are a cheap win for unnamed actors.
- Tracing: per-session isolation (OTP 27) matters once more than one tool traces at a time; a rate-limited trace (recon_trace style) prevents tracing from becoming the outage.

### Gaps
- recon_trace rate-limit specifics were not retrieved (the recon page did not cover them).
- observer / observer_cli current features were not researched.
- OTel context propagation across process boundaries (must be passed explicitly) was not confirmed from the fetched page.

---

## 8. Notable 2025-2026 changes (OTP 28/29, JIT, Erlang typing, Gleam OTP v1, Elixir 1.19/1.20)

### Takeaway
2025-2026 BEAM changes relevant to actor runtimes are incremental: OTP 28 (May 2025) added priority messages (a VM-level "control lane") and a stack-preserving `hibernate/0`; OTP 27 (2024) added process labels and trace sessions; OTP 29 (May 13, 2026) added native records, `-unsafe` function marking, JIT and secure-by-default SSH. On the typing side, Elixir became gradually typed in v1.20 (June 3, 2026) and Gleam's typed OTP reached v1 (June 12, 2025) with a factory supervisor in v1.2 (Oct 2025). Erlang typing remains third-party (eqWAlizer moved into ELP; Etylizer research).

### Cited Findings
- OTP 28.0 released May 2025 -- [erlang-announce, May 2025](https://erlang.org/pipermail/erlang-announce/2025-May/000314.html); priority messages (EEP 76) and `erlang:hibernate/0` -- [Erlang/OTP 28 Highlights](https://www.erlang.org/blog/highlights-otp-28/); OTP 28 also implemented EEP-69 nominal types (can encode opaque types) per search summary of the OTP 28.0 notes -- [OTP-28.0 release notes](https://www.erlang.org/patches/OTP-28.0)
- OTP 29.0 released 2026-05-13: native records (EEP-79); `-unsafe` attribute and compiler warnings on calls to always-unsafe OTP functions; xref finds unsafe/undocumented calls; JIT generates better code for multi-segment little-endian binaries; SSH daemon shell/exec and SFTP disabled by default; x25519mlkem768 post-quantum hybrid key exchange preferred by default (RC3 notes); `ct_doctest`; `io_ansi` -- [OTP 29.0 announcement](https://erlang.org/pipermail/erlang-announce/2026-May/000353.html); [OTP-29.0 notes](https://www.erlang.org/patches/otp-29.0) (some items reported from RC notes by the search summary, not confirmed against final notes)
- OTP 29.0 also: `pg` internal protocol gained versioning (inferred from type signatures in the pg docs, not documented behaviour) -- [pg docs](https://www.erlang.org/doc/apps/kernel/pg.html); suspended processes' BIF timers are suspended -- [erlang module docs](https://www.erlang.org/doc/apps/erts/erlang.html)
- OTP 27 (2024, slightly older than window): process labels, trace sessions, `tprof`, `json` module, JIT native coverage -- [Erlang/OTP 27 Highlights](https://www.erlang.org/blog/highlights-otp-27/)
- Elixir 1.19 (2025-10-16): type checking of protocol dispatch, type inference/checking of anonymous functions, captures carry types, up to 4x faster compilation on large codebases -- [Elixir v1.19 release post](https://elixir-lang.org/blog/2025/10/16/elixir-v1-19-0-released/)
- Elixir 1.20 (2026-06-03): announced as the point where Elixir became gradually typed -- infers and checks every program without annotations, including guards; reports dead code and "verified bugs"; requires OTP 27+, compatible with OTP 29 -- [Elixir v1.20 release post](https://elixir-lang.org/blog/2026/06/03/elixir-v1-20-0-released); current docs at v1.20.4 -- [Elixir Registry docs](https://elixir.hexdocs.pm/Registry.html)
- gleam_otp: v1.0.0 2025-06-12 (rc1 2025-05-16 replaced `Spec` with an actor `Builder`, added `Initialised`, removed old `supervisor`/`task` modules, added `supervision` and `static_supervisor`, generated child ids); v1.1.0 2025-08-05; v1.2.0 2025-10-03 added `factory_supervisor` (dynamic children of one type); v1.3.0 2026-08-09 added `count_children`; unreleased: actors stop rebuilding their receive selector per message -- [gleam_otp CHANGELOG](https://raw.githubusercontent.com/gleam-lang/otp/main/CHANGELOG.md)
- gleam_otp goals and limits: "Full type safety of actors and messages"; compatible with Erlang's OTP actor framework; actors do not yet support all OTP system messages; not all OTP functionality is included because some cannot be represented type-safely; further supervision strategies still in development -- [gleam_otp docs](https://gleam-otp.hexdocs.pm/)
- Gleam compiler: v1.19.0 (2026-10-05) "Gleam doesn't compile to Erlang source anymore" (post title only; details not read) -- [Gleam news](https://gleam.run/news/)
- Erlang static typing: `dynamic()` type (EEP 61) in OTP 26; eqWAlizer (WhatsApp) development reportedly moved into ELP and the standalone repo archived 2026-06-26 (from a GitHub mirror, unconfirmed); Etylizer (set-theoretic types) is research-grade (hex 0.0.1, 2025); an ICFP 2026 Erlang-workshop study ran it on ~2,700 student projects -- [EEP 61](https://www.erlang.org/eeps/eep-0061); [eqWAlizer mirror](https://gitblind.noratr.app/WhatsApp/eqwalizer); [Etylizer on hex](https://hex.pm/packages/etylizer); [ICFP 2026 Erlang workshop paper](https://icfp26.sigplan.org/details/erlang-2026-papers/6/Set-Theoretic-Type-Checking-of-Beginner-Erlang-Code-A-Retrospective-Study)

### Inferences
- The strongest 2025-2026 signal for our design: the BEAM team itself added a priority lane (OTP 28) for exactly the problem our control lane solves, and kept it opt-in, low-volume, with per-sender reordering explicitly allowed. That validates our control lane and suggests letting DOWN/EXIT/system signals opt into it.
- Typed actors (Gleam's `Subject(Msg)`, Elixir 1.20 inference) are the ecosystem trend; for a TypeScript runtime the analogue is typed pids / typed reply channels (`Pid<Msg>`, `ReplyTo<T>`), which TypeScript can express natively.
- Gleam's choice of factory_supervisor (dynamic children of one type) vs static_supervisor mirrors Elixir's DynamicSupervisor/Supervisor split; a small runtime should offer both shapes rather than one generic supervisor.
- Nothing in OTP 28/29 changes the fundamentals (links, monitors, supervisors, unbounded mailboxes, mesh distribution); the gaps listed in Q1-Q7 remain the priority list.

### Gaps
- No primary source on OTP 28/29 JIT work beyond the binary-matching item; no data on scheduler changes in 2025-2026.
- Did not confirm whether OTP 29 changed any distribution, pg, or global defaults (only the pg protocol versioning hint).
- Gleam v1.19 post content (what replaced Erlang source output) was not read.
- Whether `gleam_erlang`'s `process.Name` (named processes) shipped with gleam_otp v1 was not confirmed from the fetched pages.

---

### Cross-cutting priority list (inference, for the report writer)

Ranked by production impact in BEAM shops versus likely absence in our runtime:
1. Links + trap_exit + untrappable kill, and an ordered, timed supervisor shutdown (Q1, Q6).
2. Request/reply with one-shot reply aliases (late replies dropped at the door) and multi-request collections with a shared deadline (Q3).
3. Graduated mailbox overload policy (async -> blocking send -> drop -> flush -> kill/restart) with hysteresis events, plus a per-actor memory/size kill guard (Q3, Q4).
4. Demand-driven stage abstraction (GenStage/Broadway shape: max/min demand, buffer keep policy, partitioning, rate limit, graceful 30 s-style drain) and a fail-fast in-flight semaphore (Q4).
5. Naming: local unique/duplicate registry with auto-cleanup; pg-style scoped groups with membership subscription; explicit name-conflict policy after partitions; quorum mode for singletons (Q2, Q5).
6. Observability: per-actor counters + labels, top-N and windowed top-N, safe get_state with redaction, telemetry spans, isolated rate-limited tracing (Q7).
7. Distribution hygiene: count any payload as liveness; separate control/heartbeat from data channels; per-node batched fan-out (Manifold pattern) (Q2, Q5).
