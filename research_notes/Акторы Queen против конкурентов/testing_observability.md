# Testing and observability for actor and distributed runtimes (2025-2026), and what a small TypeScript/Bun actor runtime could adopt

Scope note: the runtime in question is TypeScript/Bun and BEAM-style. It has pids, bounded mailboxes, supervisors with backoff, per-turn timeouts, slices, thread/process isolation, and multi-node operation over Postgres LISTEN/NOTIFY. It already has a virtual clock and simulated benchmarks. Its production telemetry is mostly log lines, and its decisions come from compiled WebAssembly "cards". Unless a date says otherwise, findings are from 2025-2026. Older material is marked **[older]**.

---

## 1. Deterministic simulation testing (DST): how FoundationDB, TigerBeetle, Antithesis, Resonate, WarpStream/Aiven, madsim/turmoil control time, scheduling and faults, and what they found

### Takeaway
Every serious DST setup makes one seed determine the whole run. Clock, PRNG, scheduling order, network delivery and disk results all come from that seed, so any failure replays exactly. The setups differ in where the line is drawn. FoundationDB and TigerBeetle build the simulator into the runtime itself. Antithesis wraps unmodified containers in a deterministic hypervisor. madsim and turmoil swap the async runtime underneath. Resonate wraps its real TypeScript SDK core in a simulated network.

The evidence (etcd 2025, Kafka 2026, Jepsen on TigerBeetle 2025) shows two things. DST finds real, rare bugs that chaos testing does not. It also misses whatever the fault model is too coarse to express.

### Cited Findings

**FoundationDB [older, still the reference design]**
- Simulation is built on Flow, FoundationDB's C++ coroutine runtime, which runs either in production mode or in deterministic simulated mode. A whole cluster is simulated "within a single-threaded process", which gives perfect repeatability. — [FDB testing docs](https://apple.github.io/foundationdb/testing.html)
- What is simulated: machine count and type, drive performance and drive space (including filling up), and network packet delivery. Failure modes are connection failures, degraded machines, shutdowns and reboots, machines "returning from the dead", and datacenter failures. Tests fail machines at short intervals and delay channels. — [FDB testing docs](https://apple.github.io/foundationdb/testing.html)
- "Swizzle-clogging" came out of an internal competition to design the nastiest failure. It picks a random subset of nodes, clogs their connections one by one over a few seconds, then unclogs them in random order. The page credits it with finding deep, rare bugs. — [FDB testing docs](https://apple.github.io/foundationdb/testing.html)
- Scale: tens of thousands of simulations run nightly, and the page estimates the cumulative total at "roughly one trillion CPU-hours" equivalent. It also cites about a 10:1 real-to-simulated time ratio; the direction of that ratio is as summarized and I did not verify it against the page text. — [FDB testing docs](https://apple.github.io/foundationdb/testing.html)
- The CACM 2023 version of the SIGMOD'21 paper says every new feature is tested under simulated disk, process, network and request-level faults within a single physical process. It calls bugs that need several crashes or restarts in a specific order "deep" bugs. — [CACM 2023](https://cacmb4.acm.org/magazines/2023/6/273229-foundationdb-a-distributed-key-value-store)
- Vendor claim, not independently verified: FoundationDB "only shipped one user-reported bug before Apple acquired FoundationDB in 2015." — [Antithesis blog](https://antithesis.com/blog/is_something_bugging_you/)
- A third-party team (Clever Cloud) wrote Rust workloads in the FDB simulator for their own layer. On the first seed it flagged a `commit_unknown_result` edge case where atomic counters incremented twice, a bug they estimated would show up in production about "once every few months under heavy load". — [Pierre Zemb](https://pierrezemb.fr/posts/writing-rust-fdb-workloads-that-find-bugs)
- The FDB team's own caveat is that testing "can only prove the existence of bugs - never the absence of bugs." — [FDB wiki](https://github.com/apple/foundationdb/wiki/Brainstorming:-Making-FoundationDB-more-resilient-against-bugs)

**TigerBeetle VOPR**
- An entire cluster, "running real code, is subjected to all kinds of network, storage and process faults, at 1000x speed." The VOPR finds "both logical errors in the algorithms and coding bugs." "This simulator is running 24/7 on 1024 cores, fuzzing the latest version of the database." — [TigerBeetle safety docs](https://docs.tigerbeetle.com/concepts/safety/)
- Time compression, per the 2023 blog: "3.3 seconds of VOPR simulation gives you 39 minutes of real-world testing time. An hour gives you a month." All non-deterministic parts (clock, network, disk) are stubbed, and a failing seed plus a git commit hash replays the exact run. — [TigerBeetle blog 2023](https://tigerbeetle.com/blog/2023-07-11-we-put-a-distributed-database-in-the-browser) (seen through a search excerpt)
- **[older]** `--send` posts failing seeds to a "VOPR Hub", which replays them, deduplicates them, and opens GitHub issues automatically. — [TigerBeetle HACKING.md archive](https://github.com/tigerbeetle/tigerbeetle-history-archive/blob/main/docs/HACKING.md)
- Feb 2025 "VOPR green" work did two things. It removed false positives, "cases where our consensus protocol was behaving correctly, but our understanding led to false positives in the simulator". It also fixed the last known liveness bug, which came from a single corrupted block interacting with asynchronous checkpointing and needed a new on-disk CheckpointState format. — [TigerBeetle newsletter, Mar 2025](https://tigerbeetle.com/newsletters/2025-03-05-february-in-tigerland)
- **What Jepsen found that VOPR missed (June 2025, TigerBeetle 0.16.11-0.16.30).** Jepsen reported 12 issues, "only two safety issues", and 7 crashes (2 client-side, 5 server-side).
  - #2681a: "TigerBeetle's internal testing with the VOPR did not discover this bug because it corrupted entire sectors." The VOPR was then changed to inject single-byte errors, which reproduced the bug.
  - #2544 (missing query results): "went undetected by all four TigerBeetle fuzzers" because generated objects happened to be consecutive in each index, so the zig-zag merge path never ran.
  - There were no fuzz tests for cross-version upgrades.
  - Jepsen still attributes TigerBeetle's robustness "in large part to TigerBeetle's extensive simulation, integration, and property-based tests."
  - [Jepsen: TigerBeetle 0.16.11](https://jepsen.io/analyses/tigerbeetle-0.16.11)

**Antithesis (deterministic hypervisor, applied to unmodified systems)**
- Antithesis runs user-provided Docker containers under a deterministic supervisor. "Owning the supervisor allows them to simulate everything inside, including the system clock ticks." No code changes are needed. — [Aiven, Mar 2026](https://aiven.io/blog/deterministic-simulation-testing-in-diskless-apache-kafka)
- **etcd (Sep/Oct 2025).**
  - Setup: 3-node and 1-node clusters. Network faults: latency, congestion, partitions. Container faults: thread pauses, process kills, clock jitter, CPU throttling. 830 wall-clock hours, about 4.5 simulated years.
  - Coverage of known bugs: every known bug tested was found, including 5 historical ones.
  - New bugs: watch on a future revision receiving old events (#20221, fixed in 3.6.2, present in all stable releases); a panic "db page expected to be 5" (#20271, fixed in 3.6.5); and a bug in etcd's own robustness-test model (#19998).
  - It also "revealed a flaw in etcd's linearization checker model."
  - [etcd blog, Oct 2025](https://etcd.io/blog/2025/autonomus_testing_with_antithesis/)
- **Causality analysis (May 2026).** Antithesis rewinds to snapshots along a failing timeline, replays forward many times with different faults, and plots bug probability over time.
  - An etcd stale-read bug appeared about "once in about every 19 days of testing". Runs were 12 hours a day on 48 cores, about 11,000 CPU-hours.
  - The probability graph jumped at a container hang and again at a network delay. Tuning fault injection toward those made the bug appear "twenty times more often".
  - Root cause: a retry reused the same request identifier, so a stale leader took a delayed response as confirmation of a newer request. The fix was a unique ID per retry.
  - [Antithesis blog, May 2026](https://antithesis.com/blog/2026/causality_analysis/)
- **Assertions as both oracle and guidance.** The types are `always`, `alwaysOrUnreachable`, `sometimes` (must be true at least once per run), `reachable` and `unreachable`. In the docs' words, "Assertions also provide hints to our platform about which states should be explored." Failed assertions do not exit the program, and they "can run safely in production with minimal impact." — [Antithesis assertion docs](https://antithesis.com/docs/properties_assertions/assertions/)
- **Aiven Diskless (Inkless) Kafka (Mar 2026).**
  - Setup: 14 Ducktape workers, PostgreSQL 17 as the control-plane coordinator, MinIO for object storage. Invariants: the test never hangs (`reachable`), every written message is read (`always_or_unreachable`), no out-of-order reads or duplicate offsets (`unreachable`).
  - Volume: about 2,200 logical hours on Inkless, with no Inkless bugs found. Broader testing was "~200 wall clock hours which resulted in ~9700 logical hours (1.1 years)".
  - Upstream finding: running upstream Kafka's own system tests surfaced KAFKA-19880, where under the idempotent producer "the first record batch produced may be delivered out of order".
  - [Aiven, Mar 2026](https://aiven.io/blog/deterministic-simulation-testing-in-diskless-apache-kafka)
- **WarpStream [2024].** WarpStream already had a homegrown fault-injection library across its stack. Antithesis added coverage-guided fuzzing of faults and used "280 logical hours in just 6 real-world hours". — [WarpStream blog, Mar 2024](https://warpstream.com/blog/deterministic-simulation-testing-for-our-entire-saas); figure from [Aiven](https://aiven.io/blog/deterministic-simulation-testing-in-diskless-apache-kafka)

**Resonate: DST of a TypeScript SDK (closest analogue to this runtime)**
- Harness and seed: the harness in `sim/` runs every 20 minutes on Ubuntu, Windows and macOS across Node 18, 20 and 22. A seeded linear-congruential PRNG derives the whole run from one integer. It picks which function runs at each step, whether a message is dropped, duplicated or delayed, and whether a worker drops out and returns.
- Run shape: a default run is 10,000 steps across 3 simulated workers and 1 simulated server.
- What is real: "The simulation wraps the SDK's real core logic in a simulated network: the code under test is the code you'd ship." The transport is not simulated; end-to-end suites cover it.
- Determinism check: CI runs each seed twice and diffs the logs, and a mismatch fails the run.
- Failure reporting: a failing run automatically files a GitHub issue with the seed, commit SHA and reproduction command.
- Specification: a Lean 4 executable model is "the normative definition of the protocol". It has 95 decidable properties, with an exhaustive sweep over 1,464 scripts of length up to 3; 32 properties are proved for all reachable traces.
- Differential testing: `differential_random` runs one random operation sequence against an independent in-memory oracle and the SQLite/Postgres/MySQL backends, compares status and body for every operation, and compares full state every 200 steps. It stops when 20 consecutive batches produce no new (operation, status, state-shape) triple, capped at 200,000 steps.
- Captured server traffic is checked against the model, and also by Porcupine for linearizability.
- No bug counts are published.
- [Resonate docs](https://docs.resonatehq.io/evaluate/how-resonate-is-tested)
- Dominik Tornow (Resonate CEO) frames concurrency as "non-deterministic partial order" and distribution as adding "non-deterministic partial failure". — [FOSDEM 2025 talk transcript](https://ftp.fau.de/fosdem/2025/ud6215/fosdem-2025-4279-squashing-the-heisenbug-with-deterministic-simulation-testing.vtt)

**DBOS: durable execution on Postgres, chaos rather than DST (Jul 2025)**
- The approach: tests run against real Postgres with random process crashes and restarts, database disconnects and crashes, and mid-flight version rollouts. The argument: "mocking complex dependencies like Postgres will almost certainly miss some behaviors and obscure critical bugs."
- Invariants checked: every workflow completes, queue concurrency limits hold across restarts, a sent notification is always received, and timeouts are honored across restarts.
- The bug it found: `recv()` kept a map of condition variables. When a DB disconnect threw mid-function, the entry was never removed, so a recovered workflow blocked forever on the stale variable. The fix was a try/finally (dbos-transact-py PR #358).
- [DBOS blog, Jul 2025](https://www.dbos.dev/blog/how-to-test-durable-execution)

**Rust runtime swaps: madsim, turmoil, Shuttle**
- madsim is a drop-in tokio replacement. It works through `[patch]` replacement of tokio and related crates plus libc interception, and is enabled with a `RUSTFLAGS` cfg. It gives deterministic futures per seed, plus APIs to kill a process or disconnect the network. RisingWave uses it for continuous end-to-end simulation. — [madsim docs](https://docs.rs/crate/madsim/latest); [RisingWave blog](https://risingwave.com/blog/deterministic-simulation-a-new-era-of-distributed-system-testing/)
- turmoil runs many hosts on one thread and offers a scriptable or seeded network. s2.dev found turmoil runs were still non-deterministic until they fixed three hidden inputs: timestamps embedded in HTTP packets, use of `std` time instead of tokio time, and tokio's unseeded internal scheduling RNG. — [s2.dev blog](https://s2.dev/blog/dst)
- Polar Signals (Jul 2025) concluded madsim is "likely the right fit" for an existing Rust codebase. — [Polar Signals](https://blog.polarsignals.com/blog/posts/2025/07/08/dst-rust)
- **AWS ShardStore (S3) [2021].** Lightweight formal methods, meaning executable reference models plus property-based and concurrency testing, prevented 16 issues from reaching production, including crash-consistency and concurrency bugs. AWS built Shuttle, a randomized "stateless model checker", for it; even small tests have tens of thousands of atomic steps and the largest exceed a million. — [ShardStore SOSP'21](https://www.cs.utexas.edu/%7Ebornholt/papers/shardstore-sosp21.pdf). Shuttle is randomized and not sound, but scales beyond Loom. — [Shuttle repo](https://github.com/awslabs/shuttle)
- **AWS (CACM, June 2025).** AWS uses DST: a single-threaded simulator controls thread scheduling, timing and message ordering. P is used for state-machine specs, plus TLA+, property-based testing and fault injection. — [CACM: Systems Correctness Practices at AWS](https://cacm.acm.org/practice/systems-correctness-practices-at-amazon-web-services) (seen through search excerpts; the full text returned 403)

### Inferences
- **The runtime is already most of a simulator.** It has a virtual clock and its decisions come from deterministic WASM cards, so it can follow the FoundationDB/TigerBeetle "simulator inside the runtime" pattern instead of Antithesis-style OS interception. Every nondeterminism source should sit behind one seeded interface:
  - the clock and timers;
  - the scheduler's choice of which ready pid runs next;
  - cross-thread and cross-process delivery order;
  - LISTEN/NOTIFY delivery (drop, duplicate, delay, reorder, connection loss);
  - Postgres query failures (throw mid-turn, as in DBOS);
  - worker/thread death;
  - `Math.random`, `Date.now` and `performance.now` inside actors.
- **Copy Resonate's CI hygiene almost verbatim, since it is already TypeScript.** Run each seed twice and diff the event logs, which catches hidden nondeterminism of the kind s2.dev hit. Auto-file an issue carrying seed, commit SHA and repro command.
- **The fault model needs fine grain.** TigerBeetle's whole-sector versus single-byte lesson translates to faults like these: NOTIFY payload truncation, one lost notification while the next arrives, a timeout firing during cleanup, and a supervisor restart racing a late reply from the previous incarnation. That last one is the same shape as the etcd "retry reused request ID" bug, so pid and request IDs should be incarnation-unique.
- **Add `sometimes`-style coverage assertions** for rare runtime states: mailbox full and send rejected, supervisor intensity exhausted, backoff at max, node lost while a call was in flight, slice preempted mid-turn. A green simulation that never reached those states proves little. Antithesis and Jepsen both show coverage gaps, such as "objects consecutive in index", hiding bugs.
- **Keep one real-Postgres chaos suite next to the simulator.** DBOS's warning that mocks of Postgres hide behaviors applies directly to LISTEN/NOTIFY, whose semantics include notifications delivered only at commit, an 8000-byte payload limit, and lost notifications on reconnect. Those are from general knowledge; verify them against the Postgres docs before relying on them.

### Gaps
- I could not retrieve FoundationDB's BUGGIFY primary description or any per-year bug counts from the SIGMOD paper.
- There are no public bug counts for Resonate's TypeScript DST or for WarpStream's Antithesis runs.
- The AWS CACM 2025 article body was blocked (403). P/PObserve usage numbers per service are unverified.
- I found no published DST framework specific to Bun or Node beyond Resonate's in-repo harness and fast-check's scheduler (section 2).

---

## 2. Systematic concurrency testing for actors: Concuerror, QuickCheck/PULSE and PropEr statem, Akka TestKit, Orleans TestCluster, P and Coyote, fast-check, Jepsen

### Takeaway
There are two complementary families. The first is systematic exploration: Concuerror (exhaustive DPOR), Coyote and P (controlled scheduler with prioritized search), and Shuttle (randomized). The second is randomized property and model-based testing with a randomizing scheduler: QuickCheck with PULSE, PropEr statem, and fast-check's `scheduler` and `scheduledModelRun`. Framework testkits (Akka, Orleans) mainly give real-runtime harnesses plus virtual time. They do not explore interleavings.

For a TypeScript runtime, fast-check's scheduler and model-based runners are the ready-made, in-ecosystem tools. Coyote's "actors plus a controlled scheduler" design is the architectural model to imitate.

### Cited Findings
- **Concuerror [tool page last modified Oct 2020; flag as stale].** A "stateless model checking tool for Erlang programs". It schedules processes as if only one scheduler existed, records the operations whose results depend on scheduling (receive, registry, ETS), finds pairs that actually race and reverses them (dynamic partial order reduction). It "systematically explores all 'meaningfully different' schedulings" and so can verify the absence of errors for a bounded test. — [concuerror.com](https://concuerror.com/)
- **[older]** Concuerror was used to test and verify chain-repair methods for Corfu (a Chain Replication variant), checking immutability and linearizability. It integrates with EUnit and replays traces deterministically. — [EUC 2017 tutorial](https://www.erlang-factory.com/euc2017/stavros-aronis)
- **[older, 2017]** Sagonas: stateless model checking "can find concurrency bugs pretty fast" with careful modelling, but exponential blowup "will strike eventually". Hughes: random tools (QuickCheck/PULSE) "sample from a much larger part of the space", so the two are complementary. — [erlang-questions 2017](https://erlang.org/pipermail/erlang-questions/2017-October/093782.html)
- **QuickCheck + PULSE [older, 2009/2011].** The method: QuickCheck generates parallel tests, PULSE randomizes the scheduler, and a sequential-consistency (linearizability-style) check flags atomicity failures. It found 2 races in an industrial prototype. — [ICFP 2009](https://research.chalmers.se/en/publication/125252). Applied to dets (Mnesia's disk storage), it found 5 race conditions "with relatively little effort", 2 of which may explain observed production failures. — [Erlang Workshop 2011](https://research.chalmers.se/en/publication/150540). Hughes's slides say the spec was under 200 lines. — [Hughes guest lecture slides](https://www.cse.chalmers.se/edu/course.2018/TDA567/Lec2018/GuestLectureJH.pdf)
- **fast-check (TypeScript/JavaScript, 2025).**
  - `fc.scheduler()` wraps promises (`schedule`) or whole APIs (`scheduleFunction`) so the test decides resolution order. The docs say any code that can schedule tasks is a potential race source. — [fast-check race conditions](https://fast-check.dev/docs/advanced/race-conditions/)
  - Pitfalls: immediately resolved promises "happen to resolve in the correct order only because they were instantiated in the correct order". Async work started outside the scheduler makes replays harder. — [fast-check tutorial](https://fast-check.dev/docs/tutorials/detect-race-conditions/your-first-race-condition-test)
  - fast-check 4.2.0 (Jul 2025) added `waitIdle`, "simple, efficient and predictable", replacing the gotcha-prone `waitAll`. — [fast-check 4.2.0](https://fast-check.dev/blog/2025/07/18/whats-new-in-fast-check-4-2-0/)
  - Model-based testing: commands implement `check(model)`, `run(model, real)` and `toString()`, with runners `modelRun`, `asyncModelRun` and `scheduledModelRun`. The last combines model-based commands with the race scheduler. — [fast-check MBT](https://fast-check.dev/docs/advanced/model-based-testing/); [scheduledModelRun API](https://fast-check.dev/api-reference/functions/scheduledModelRun.html)
  - The docs warn: "avoid testing the code by comparing it to itself." — [fast-check MBT](https://fast-check.dev/docs/advanced/model-based-testing/)
- **Microsoft Coyote (successor of P#).**
  - Mechanism: binary rewriting hands a scheduler control of execution, which explores interleavings and nondeterminism for safety and liveness bugs, and fully reproduces any bug it finds. It supports both task-based C# and in-memory actors, including a state-machine actor type it "knows how to fully test". — [Coyote project page](https://www.microsoft.com/en-us/research/project/coyote/); [MSR blog 2020](https://microsoft.com/en-us/research/blog/coyote-making-it-easier-for-developers-to-build-reliable-asynchronous-software)
  - Demo: on a 5-server Raft it controlled message ordering and found an implementation bug where two leaders get elected. — [MSR blog 2020](https://microsoft.com/en-us/research/blog/coyote-making-it-easier-for-developers-to-build-reliable-asynchronous-software)
  - Claim: "used by several teams in Azure ... finding hundreds of bugs before they manifest in production". This is the project's own figure. — [NuGet package](https://feed.nuget.org/packages/Microsoft.Coyote.CLI/1.4.3)
  - **Azure Batch [older].** Three core microservices (over 100K LOC) were written as Coyote state machines with models of external services. A low-priority VM feature took 1 dev-month versus 6 in the legacy code. Developers ran failover and concurrency tests on their desktops at every check-in, and features that passed the tester "worked from the start" in production. No bug count is given. — [Coyote Azure Batch case study](https://microsoft.github.io/coyote/case-studies/azure-batch-service/)
  - SoCC 2021: "Coyote Actors impose a principled design pattern that allows writing formal specifications alongside production code," and some services ran live "for months" without issues in Coyote-tested features. — [SoCC'21 paper page](https://www.microsoft.com/en-us/research/publication/building-reliable-cloud-services-using-coyote-actors/)
- **P language.** The model of computation is communicating state machines ("actors"). P-Checker explores interleavings of messages and failures with search-prioritization heuristics and emits reproducible error traces. Other backends: PSym (symbolic), PEx (exhaustive), PVerifier (proofs). PObserve checks production and test logs against the same spec monitors. — [P docs](https://p-org.github.io/P/whatisP/)
- **Akka typed TestKit.**
  - `ActorTestKit` gives a real ActorSystem. `TestProbe` provides `expectMessage` and `expectMessageType`.
  - `ManualTime` is a scheduler you "manually, explicitly advance", with `timePasses` and `expectNoMessageFor`.
  - `LoggingTestKit` asserts log events. `Behaviors.monitor` with a probe spies on traffic to a mocked dependency.
  - The docs admit that with timing-dependent actors "it can be hard to get the timing just right", and that generous timeouts make tests slow.
  - [Akka async testing docs](https://doc.akka.io/libraries/akka-core/current/typed/testing-async.html)
- **Orleans (docs dated Jan 2026).** `InProcessTestCluster` is recommended (default 1 silo; `TestCluster` defaults to 2) and supports `StartSiloAsync`, `StopSiloAsync` and `RestartAsync` mid-test. `UseRealEnvironmentStatistics` defaults to false, so statistics are simulated. Mock-based grain tests have known limits "around scheduling reentrancy and serialization". — [Orleans testing docs](https://learn.microsoft.com/en-us/dotnet/orleans/implementation/testing)
- **Temporal TypeScript SDK.**
  - Time skipping: `TestWorkflowEnvironment` is an in-memory server where "Time is a global property" and timers are fast-forwarded except while activities run.
  - Replay testing: `Worker.runReplayHistory` / `runReplayHistories` replays recorded event histories and throws `DeterminismViolationError` on nondeterminism. The docs recommend failing CI on replay errors.
  - [Temporal TS testing docs](https://docs.temporal.io/develop/typescript/testing-suite)
- **Jepsen 2025-2026.** 2025 analyses covered Amazon RDS for PostgreSQL 17.4 (Apr), TigerBeetle 0.16.11 (Jun), Capela (Aug) and NATS 2.12.1 (Dec). The search summary of the NATS report says JetStream lost data or entered persistent split-brain under file corruption and simulated node failures, partly because the default flushed to disk about every two minutes rather than before acknowledging. Jepsen and Antithesis published a joint reliability glossary in Oct 2025. — [Jepsen analyses index](https://jepsen.io/analyses) (I did not fetch the NATS and RDS reports themselves)

### Inferences
- **Lowest-cost, highest-yield adoption: fast-check `scheduledModelRun` against the supervisor and mailbox.**
  - Model: a pure TypeScript model of the supervisor/mailbox semantics. Ideally it is the same compiled card logic run in a reference interpreter, or an independent oracle as in Resonate.
  - Commands: `send`, `crash`, `timeout`, `restart`, `nodeLoss`, `tick(n)`.
  - The scheduler permutes promise resolution.
  - Heed the fast-check warning: if the model *is* the card, you are testing the card against itself. Write a small independent oracle for the key invariants instead.
- **Port the PULSE/Coyote idea, not the tool.** Because the runtime owns the scheduler, a "test scheduler" mode that picks the next ready pid from the seed (random, or PCT-style priority switching) gives Coyote-like systematic testing without binary rewriting. Coyote's actor support works for exactly this reason: the actor framework owns delivery order.
- **Expose testkit primitives modelled on Akka and Orleans:** a probe pid with `expectMessage` / `expectNoMessageFor(virtualMs)`, manual time, `stopNode` / `startNode` / `restartCluster` on an in-process multi-node cluster, and a log-assertion helper. These are table stakes users of Akka or Orleans will expect.
- **Use replay as a regression gate, as Temporal does.** Record real production decision logs (inputs to the cards plus outputs) and replay them against new card builds in CI. A divergence is a determinism or compatibility break.

### Gaps
- I found no post-2020 public case study listing specific bugs Concuerror found. Its current maintenance status is unclear from the tool page.
- I did not verify PropEr statem's 2025-2026 status or its parallel/linearizability features from primary docs, and found no 2025 PropEr case study.
- Coyote: there are no per-service bug counts beyond "hundreds" (self-reported). The current maintenance status of the GitHub repo was not checked.

---

## 3. Model checking and specs for actor protocols (TLA+, P, Quint, Lean) and how teams connect them to code

### Takeaway
By 2025-2026 the practical bridge between spec and code takes three forms:
1. **Model-based testing**: the spec generates traces, which are replayed against the code (quint-connect, tla-connect/Apalache, Resonate's oracle).
2. **Trace validation**: production or test logs are checked post hoc against the spec (AWS PObserve, TLA+ trace validation in CCF, TraceLink, Resonate's `valid/lean`).
3. **Specs embedded in the programming model** (Coyote and P state machines).

Trace validation has caught serious bugs with very few executions. Manual instrumentation cost is the recurring failure mode.

### Cited Findings
- **AWS PObserve (2023).** "With PObserve, we take structured logs from the execution of distributed systems and validate post hoc that they match behaviors allowed by the formal P specification of the system", in testing and in production. The authors say runtime checking of the same properties "makes the investment in formal specification much more valuable." — [CACM, June 2025](https://cacm.acm.org/practice/systems-correctness-practices-at-amazon-web-services) (via search excerpts). The P docs add that PObserve parses service logs "without requiring extra instrumentation". — [P docs](https://p-org.github.io/P/whatisP/)
- **TLA+ trace validation (2024 paper).** Implementation traces are mapped onto a TLA+ state machine and TLC checks them. This gives no formal guarantee, yet "found serious bugs in code already tested by conventional means". — [arXiv 2404.16075](https://arxiv.org/pdf/2404.16075)
- **CCF (Microsoft).** "We were able to find two significant data-loss issues using TV with just about a dozen system executions." — [TLA+ wiki: trace validation](https://docs.tlapl.us/using:tlc:trace_validation)
- **TraceLink (OOPSLA 2025).** Earlier TLA+ trace validation required that the implementation "be instrumented with sufficient logging statements" and that each event be manually related to the model. TraceLink automates both for PGo-generated systems and found bugs across PGo's components, including the compiler. — [TraceLink paper](https://www.cs.ubc.ca/~bestchai/papers/oopsla25-trace-link.pdf)
- **MongoDB [older].** A trace-checking effort was cancelled because it grew to cost more effort than it was worth. A separate model-based test-generation effort on Realm Sync reached 100% branch coverage of the specified algorithm. — [A. Jesse Jiryu Davis: Extreme modelling in practice](https://emptysqua.re/blog/extreme-modelling-in-practice/); [repl-trace-checker](https://github.com/mongodb-labs/repl-trace-checker)
- **Quint Connect.** Generates traces from Quint specs (random, witnesses or runs) and replays them against Rust through a driver: a state-conversion function plus a `step` per spec action. Its aim is to stop drift between spec and code. — [Quint Connect post](https://quint-lang.org/posts/quint_connect). In the 2025 Emerald case study (Malachite consensus) the spec actions mapped "almost 1:1" to consensus inputs, which made AI-generated glue code easy. — [Quint Connect Emerald](https://quint-lang.org/posts/quint_connect_emerald)
- **A TypeScript Quint Connect port exists.** It runs the Quint CLI MBT mode, parses ITF traces, and can diff spec and implementation state after each step. — [@firfi/quint-connect README](https://cdn.jsdelivr.net/npm/@firfi/quint-connect@2.0.1/README.md)
- **tla-connect (Rust).** It offers three modes: Apalache ITF trace replay, interactive Apalache-driven exploration, and post-hoc NDJSON trace validation against a TLA+ TraceSpec. The docs explain that the first two catch "implementation doesn't handle a case the spec allows," while the third catches "implementation does something the spec doesn't allow." — [tla-connect docs](https://docs.rs/tla-connect)
- **Interactive symbolic testing (Dec 2025).** Implementation outputs are fed back into Apalache's SMT context while testing a TFTP spec. The author found bugs in their own spec and some unexpected implementation behavior. — [TLA+ mailing list](https://discuss.tlapl.us/msg06606.html)
- **Resonate (TypeScript product).** A Lean 4 model is normative, "Where prose and Lean disagree, the Lean model wins". Captured server traffic is checked against the model (`valid/lean`) and by Porcupine for linearizability. Each handler is pure, with "now" as its only time input. — [Resonate docs](https://docs.resonatehq.io/evaluate/how-resonate-is-tested)

### Inferences
- **The runtime is unusually well placed for trace validation.** Its decisions already come from a compiled spec (the WASM cards), so the "relate each logged event to a model action" step that sinks most efforts (MongoDB, pre-TraceLink TLA+) is free when the card *is* the action. Log each card invocation as a structured event (inputs, state hash, chosen action, outputs) and you get a PObserve-style checker almost for free. Replay the event stream through the card in a checker process, and separately through an independent invariant monitor, to catch the gaps "the spec allows but the glue does something else".
- **The risk shifts to the glue between card and runtime:** timers, I/O, NOTIFY handling, thread hand-off. Use model-based testing driven from the spec, quint-connect style, with the TypeScript port or a homegrown ITF reader, to exercise that glue. Use trace validation on production logs to catch it doing what the spec forbids. tla-connect's two-direction distinction is the right framing.
- **Make card handlers pure in time, as Resonate does:** "now" is passed in, never read. Then the same handler runs identically in simulation, in model-based testing and in production replay.

### Gaps
- The P-at-AWS service list (S3, DynamoDB, EBS and so on) and PObserve's log format and deployment are unverified, because the CACM body was blocked.
- I found no public example of trace validation against a WASM-compiled spec, so the inference above is untested by others.

---

## 4. Observability for actors: mailbox depth, message latency, turn duration, restarts, dead letters; OpenTelemetry propagation across actor messages; Akka, Orleans, Cloudflare; sampling

### Takeaway
Mature actor runtimes converge on a common metric set:
- mailbox size, sampled as a distribution;
- time-in-mailbox histogram;
- processing (turn) time histogram;
- long-running-turn counter;
- dead letters, unhandled messages and dropped messages (bounded mailboxes);
- actor failures;
- running-actor counts per class or group (not per pid);
- threshold events for mailbox size or time and processing time.

For tracing, OpenTelemetry's messaging conventions (still marked Development) prescribe a creation context injected into each message, with span links as the default correlation. Discord's 2026 Elixir write-up is the best recent account of doing this for actor messages, including fan-out-aware sampling.

### Cited Findings
**Akka Insights (Lightbend/Akka)**
- Actor metrics with their OpenTelemetry names:
  - mailbox size `akka.actor.mailbox-size` (recorder, messages)
  - mailbox time `akka.actor.mailbox-time` (ns)
  - processing time `akka.message.processing-time` (ns)
  - stash size `akka.actor.stash-size`
  - running actors `akka.actor.running-actors` (per actor class or group)
  - sent and processed message rates
  - dropped messages `akka.actor.dropped-messages` (bounded mailboxes)
  - [Akka Insights metrics and events](https://doc.akka.io/libraries/akka-insights/current/reference/metrics-and-events/metrics-and-events.html)
- Events: dead letter `akka.actor.dead-letter`, unhandled message `akka.actor.unhandled-message`, actor failure `akka.actor.actor-failure`. Threshold events: `mailbox-size-limit`, `mailbox-time-limit`, `processing-time-limit`. Any metric can be disabled, and events can be rate limited. — [Akka Insights metrics and events](https://doc.akka.io/libraries/akka-insights/current/reference/metrics-and-events/metrics-and-events.html)
- Mailbox and stash size "are sampled to avoid skew in the distributions — the sampling period can be configured". Starred metrics such as processing time get an automatic message-type dimension. Prometheus uses `akka_actor_mailbox_time_ns`, and OpenTelemetry durations are converted to seconds by default. — [Akka Insights features 2.21.x](https://doc.akka.io/libraries/akka-insights/2.21.x/introduction/overview/features.html) (via search excerpts)
- **[older, Kamon]** Mailbox time is defined as "from the instant when a message is enqueued in a actor's mailbox to the instant when that message is dequeued for processing". — [Kamon akka docs](https://www.javadoc.io/static/io.kamon/kamon-akka_2.11/0.6.2/kamon/akka/package.html)

**Orleans (meter `Microsoft.Orleans`, docs updated May 2026)**
- Runtime: `orleans-scheduler-long-running-turns` (counter).
- Messaging: `orleans-messaging-expired`, `-rejected`, `-rerouted`, `-sent-dropped`, `-sent-failed`, sent and received size histograms, and `orleans-messaging-processing-ima-enqueued`.
- Gateway: `orleans-gateway-load-shedding` counts messages rejected because the gateway is overloaded.
- Catalog: activation created, destroyed and `failed-to-activate`.
- App requests: `orleans-app-requests-latency` (bucketed 1 ms to 15 s) and `orleans-app-requests-timedout`.
- Streams: `orleans-streams-queue-oldest-message-enqueue-age`.
- Reminders: `orleans-reminders-tardiness` histogram.
- Live view: `dotnet counters monitor -n <proc> --counters Microsoft.Orleans`.
- Tracing: enabled with `AddActivityPropagation()` on silo and client, using ActivitySources for application grain calls, lifecycle, storage and runtime.
- [Orleans observability docs](https://learn.microsoft.com/en-us/dotnet/orleans/host/monitoring/)

**OpenTelemetry messaging conventions**
- Status: the document is marked **Development**. Instrumentations on v1.24.0 or earlier opt in to the new conventions via `OTEL_SEMCONV_STABILITY_OPT_IN=messaging` (or `messaging/dup`). — [OTel messaging spans](https://opentelemetry.io/docs/specs/semconv/messaging/messaging-spans/)
- Operation types: `create` (PRODUCER), `send`, `receive` (CLIENT, pull), `process` (CONSUMER, push or handler) and `settle`.
- "A producer SHOULD attach a message creation context to each message."
- Span links are the default correlation, "the only consistent trace structure that can be guaranteed". Making the creation context the parent of the process span is opt-in and for single messages only.
- When processing happens inside another span, the process span is a child of the ambient span with a link to the creation context.
- Required attributes: `messaging.system`, `messaging.operation.name`. Also listed: `messaging.destination.name`, and `messaging.message.id` for single messages.
- [OTel messaging spans](https://opentelemetry.io/docs/specs/semconv/messaging/messaging-spans/)
- **Rationale (OTEP 0220).** Transport-level tracing by brokers "would not help in correlating producers with consumers". Processing often happens in application code and in a different trace, so a separate creation context is needed. — [OTEP 0220](https://github.com/open-telemetry/oteps/blob/dc619dfc70f174ef31caf90f14e8b00600da4049/text/trace/0220-messaging-semantic-conventions-span-structure.md)

**Actor-message propagation in practice**
- **Discord (InfoQ, Mar 28 2026).**
  - The problem: "Unlike HTTP-based microservices, where trace context travels in headers," Elixir messages have no metadata layer.
  - The solution: a "Transport" library with an `Envelope{message, trace_carrier}` filled by `:otel_propagator_text_map.inject`, and drop-in `call`/`cast` replacements. The receive path accepts both bare and enveloped messages, "enabling gradual migration without service restarts."
  - Fan-out problem: one guild message could spawn "a million child spans", so sampling is dynamic by fan-out size. One recipient keeps the upstream decision 100% of the time, 100 recipients are sampled at 10%, and 10,000 or more at 0.1%.
  - The article gives no overhead numbers despite its title.
  - [InfoQ: Discord Elixir actor tracing](https://infoq.com/news/2026/03/discord-elixir-actor-tracing/)
- **BEAM.** OpenTelemetry context lives in the process dictionary and does not follow `Task.async`. Spans in spawned processes become orphans unless the context is captured and re-attached. `opentelemetry_process_propagator` provides Task wrappers (`_with_span`, `_with_linked_span`) and `fetch_parent_ctx(:"$callers")` for tasks spawned by libraries such as Ecto. — [opentelemetry_process_propagator](https://preview.hex.pm/preview/opentelemetry_process_propagator/0.2.0/show/lib/opentelemetry_process_propagator.ex)

**Cloudflare Workers and Durable Objects**
- Automatic tracing treats Durable Object calls as binding operations, alongside KV and R2. Head-sampling rate is 0..1 and defaults to 1, meaning trace everything. Export is over OTLP to Honeycomb, Grafana, Axiom and others. — [Workers traces docs](https://developers.cloudflare.com/workers/observability/traces)
- Durable Object metrics come from GraphQL datasets `durableObjectsInvocationsAdaptiveGroups`, `durableObjectsPeriodicGroups` (`cpuTime`, `memoryUsageBytes` P50-P999), `durableObjectsStorageGroups` (`storedBytes`) and `durableObjectsSubrequestsAdaptiveGroups`. Charts can be filtered by object ID or name, and logs carry `$workers.durableObjectId`. Memory samples reflect the whole isolate, which may be shared by several objects. — [DO metrics and analytics](https://developers.cloudflare.com/durable-objects/observability/metrics-and-analytics/)
- Cloudflare's blog announces, but has not shipped, OpenTelemetry APIs for Workers and an OTel export of Cloudflare metrics. A preview docs page says metric export is not yet supported. — [Cloudflare blog: one observability platform](https://blog.cloudflare.com/one-observability-platform/)

**Sampling**
- Head sampling is decided early, usually from the trace ID. Consistent probability sampling gives every span in a trace the same decision. Tail sampling, done in the OpenTelemetry Collector's Tail Sampling Processor, can keep all error and slow traces but needs stateful, resource-heavy components. The docs say it is "almost always necessary" at scale, and suggest combining head sampling (to protect the pipeline) with tail sampling. — [OTel sampling concepts](https://opentelemetry.io/docs/concepts/sampling/)

### Inferences
- **Minimum metric set for this runtime**, named after Akka and Orleans so dashboards translate. Label everything by actor *kind* or card, never by pid, to avoid cardinality explosions; Akka groups by class or group.
  - `actor.mailbox.depth`: a sampled distribution, sampled periodically per Akka rather than on every enqueue.
  - `actor.mailbox.time`: enqueue to dequeue, per the Kamon definition.
  - `actor.turn.duration` histogram.
  - `actor.turn.timeouts` and `actor.turn.long_running`, as in Orleans' long-running-turns counter.
  - `actor.mailbox.rejected` (bounded mailbox full).
  - `actor.dead_letters`.
  - `actor.restarts`, labelled by reason and supervisor.
  - `supervisor.intensity_exhausted`.
  - `supervisor.backoff.current`.
  - `node.notify.lag`: NOTIFY publish to delivery.
  - `node.notify.lost` or `reconnects`.
  - `slice.preemptions`.
  - `card.decision` counts, by card and action.
- **Threshold events**, as in Akka's MailboxSizeLimit, MailboxTimeLimit and ProcessingTimeLimit, are cheap and turn today's log lines into structured, alertable events.
- **Tracing design:**
  - Add an optional `traceparent` field to the message envelope (Discord pattern).
  - Treat each turn as an OpenTelemetry `process` span that *links* to the sender's creation context (OpenTelemetry default). Parent it only for request/reply `call`s.
  - Carry the envelope through LISTEN/NOTIFY payloads too.
  - Accept bare messages for rolling upgrades.
  - Sample at ingress with a consistent trace-ID decision.
  - Down-sample on fan-out (Discord's 100% / 10% / 0.1% curve).
  - Always emit a span or event for errors, restarts and timeouts, which is tail-sampling-equivalent at the source.
- **Bun-specific note (unverified):** standard Node OpenTelemetry auto-instrumentation may not apply cleanly, as Cloudflare found with Workers, so manual spans through `@opentelemetry/api` with an OTLP/HTTP exporter is the safe path. This is not a cited finding.

### Gaps
- I found no published overhead numbers (CPU or latency percentage) for Akka Insights, Orleans meters or Discord's envelope, nor for BEAM tracing in production.
- I did not verify how Akka Insights chooses which actors to instrument, or its trace sampling configuration.
- I did not fetch Cloudflare Durable Object trace span attributes (the spans-and-attributes reference).

---

## 5. Live introspection and control: BEAM `sys`, recon, observer and LiveDashboard, and equivalents elsewhere

### Takeaway
The BEAM's standard is that every OTP process answers out-of-band *system messages*: get or replace state, suspend and resume, per-process trace and log ring buffer, and statistics. Operators use rate-limited tracers (recon_trace) and top-N scans (recon `proc_count` / `proc_window`) on live nodes through a remote shell or LiveDashboard.

Other runtimes mostly offer metrics plus tracing rather than per-actor control:
- Orleans: `dotnet-counters` for live views.
- Durable Objects: per-object metric filters and log fields.
- Temporal: replays history instead of inspecting live state.

A TypeScript runtime that owns its mailboxes can offer a BEAM-like system-message channel at low cost.

### Cited Findings
- **Erlang `sys`.**
  - State and status: `get_state/1,2` returns the callback state (`{CurrentState, CurrentData}` for gen_statem). `get_status` returns process dictionary, running or suspended status, parent and debug options. `replace_state` is "intended only to help with debugging" and should not be called from normal code.
  - Control: while suspended, a process "only responds to other system messages". Code change requires suspension.
  - Debug facilities: `trace` prints system events, `log` keeps the last N events (default 10), and `statistics` reports reductions and messages in/out. `install` adds debug functions called on each system event, and a failing debug function is removed automatically.
  - Timeout: the default is 5000 ms, and the call exits `{timeout, ...}` if the process does not answer.
  - [Erlang sys docs](https://www.erlang.org/doc/apps/stdlib/sys.html)
- **recon (v2.5.3).**
  - `info/1,2` wraps `process_info` but excludes the mailbox, because mailboxes "have a tendency to grow and be unsafe when called in production systems".
  - `proc_count(Attr, N)` returns the top N processes by an attribute such as `message_queue_len`, `memory` or `reductions`. `proc_window(Attr, N, Ms)` does the same over a sliding window, for short-lived processes, and "can take a heavy toll on memory" with tens of thousands of processes.
  - `scheduler_usage(Ms)` is used because Erlang CPU as reported by `top` "isn't the most reliable value". `get_state` wraps `sys:get_state` with a timeout. `bin_leak` finds refc binary leaks.
  - [recon docs](https://ferd.github.io/recon/recon.html)
- **recon_trace.**
  - Safety limits: a mandatory limit, either an absolute message count or a rate `{Num, Millisecs}`. The tracer process enforces limits before a formatter and never blocks on a growing mailbox.
  - Lifecycle: the tracer is linked to the shell, so tracing stops when a remote shell disconnects. `recon_trace:clear()` stops everything.
  - Guidance: very broad patterns with lax limits can destabilize the node. "In doubt, start with the most restrictive tracing possible, with low limits, and progressively increase your scope."
  - [recon_trace docs](https://ferd.github.io/recon/recon_trace.html)
- **Phoenix LiveDashboard.** It can "See, filter, and search processes", ports and ETS tables on the current node, charts `:telemetry` events in real time, shows the logs for a given request, and can inspect node B while connected to node A over distributed Erlang. — [LiveDashboard docs](https://phoenix-live-dashboard.hexdocs.pm/Phoenix.LiveDashboard.html)
- **Orleans live view.** `dotnet counters monitor -n <ProcessName> --counters Microsoft.Orleans` streams the meters live without code changes. — [Orleans observability docs](https://learn.microsoft.com/en-us/dotnet/orleans/host/monitoring/)
- **Durable Objects.** Charts can be filtered by Durable Object ID or name, and logs can be filtered on `$workers.durableObjectId`. — [DO metrics and analytics](https://developers.cloudflare.com/durable-objects/observability/metrics-and-analytics/). Cloudflare also lists a "Data Studio" page under DO observability; I did not fetch its contents. — [DO observability index](https://developers.cloudflare.com/durable-objects/observability/)
- **Temporal.** Workflows are inspected after the fact by replaying recorded event history, `Worker.runReplayHistory`, rather than by poking live state. — [Temporal TS testing docs](https://docs.temporal.io/develop/typescript/testing-suite)
- **Antithesis.** Its assertion API is meant to run in production too ("minimal impact"), blurring testing and live observability. — [Antithesis assertion docs](https://antithesis.com/docs/properties_assertions/assertions/)

### Inferences
- **Implement a system-message lane in the mailbox, BEAM-style.** It is processed between turns, ahead of user messages, and still served when the actor is suspended. It would support these operations:
  - `getState(pid, timeoutMs = 5000)`, with redaction hooks;
  - `getStatus` (behavior or card id, supervisor, restart count, backoff state, mailbox depth, current turn age);
  - `suspend` / `resume`;
  - `replaceState`, gated behind a debug flag;
  - `trace(pid, {max, ratePerMs})`, rate-limited and auto-cleared when the operator session drops, as recon_trace does;
  - `log(pid, N)`, a per-actor ring buffer of the last N system events (default 10, as in `sys:log`).
- **Never dump mailbox contents by default** (recon's rule). Report depth and age of the oldest message instead, the Orleans `oldest-message-enqueue-age` pattern.
- **Expose node-level top-N:** pids by mailbox depth, turn time or restart count, over a sliding window (`proc_window`). Expose it over an authenticated admin endpoint or the Postgres control channel, so one node can inspect another (the LiveDashboard node-B-from-node-A pattern).
- **Card decisions add a capability the BEAM lacks.** Because decisions come from deterministic cards, a "why did the supervisor do X" query can be answered by re-running the logged card inputs: replay as introspection, Temporal-style. That is cheaper and safer than live state reads.

### Gaps
- I did not research JVM/Akka live-control equivalents (Akka Management HTTP endpoints, cluster sharding inspection) or Orleans Dashboard (OrleansContrib); not fetched.
- I found no published guidance on the production overhead of `sys` debug options or `recon_trace` beyond the documented warnings.
- I did not verify the specifics of Erlang's `observer` GUI and remote shell (`erl -remsh`) from primary docs in this pass.
