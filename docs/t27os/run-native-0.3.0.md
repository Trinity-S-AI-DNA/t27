# How to run T27 OS native 0.3.0

T27 OS native 0.3.0 is the native foundation line: local chat, one agent and a swarm of up to three agents,
talking to LM Studio on this Mac. It is not the T27 OS window. The published window line is 0.2.0
(`specs/t27os/release.t27`), and 0.3.0 is not wired into the window yet (`component.ui.state=text-only`).
So "running 0.3.0" means running its acceptance suite. Version law: `specs/t27os/native_version.t27`;
suite: `specs/t27os/native_accept.t27`; full status: `docs/reports/t27os-native-0.3.0-report.md`
(Trinity-S-AI-DNA/t27#13212).

## 1. What you need

- A Mac with Apple silicon and the t27 repository at a fresh `origin/master`.
- An extern-capable `t27b`. Under `t27c`, every `@extern` is a stub, so no file or socket is ever touched.
- LM Studio with its local server running on `127.0.0.1:1234` and at least one model loaded. Only the
  live tests (B, C live, D one agent, E three agents) need it. Every other test needs only a writable `target/`.

## 2. Build t27b

From the repository root:

```bash
git fetch origin && git switch --detach origin/master
```

```bash
cargo build --release -p t27b
```

The binary lands at `./target/release/t27b`.

## 3. Start LM Studio

1. Open LM Studio and load a model (any chat model).
2. Developer tab: Start Server, port 1234.
3. Check it answers:

```bash
curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:1234/v1/models
```

`200` means it is up.

Optional environment overrides: `T27OS_LMSTUDIO_HOST` (loopback only, default `127.0.0.1`),
`T27OS_LMSTUDIO_PORT` (default `1234`), `T27OS_LMSTUDIO_MODEL` (default: the first model the server lists) and
`T27OS_LMSTUDIO_API_KEY`. These four are the only variables read. The key is read from the environment only
and never reaches a log or a report.

## 4. Run 0.3.0

First, the host glue (3 tests, needs no LM Studio):

```bash
./target/release/t27b test specs/t27os/native_host.t27
```

Then the acceptance suite (17 tests, groups A to G):

```bash
./target/release/t27b test specs/t27os/native_accept.t27
```

Each test prints `PASS` or `FAIL`, then a total line `T27osNativeAccept: N passed, M failed, 17 total`.
0.3.0 counts as created only when all 17 pass (`VERSION_CREATED: YES` in the report).

| Group | Checks | LM Studio |
|---|---|---|
| A | version report: `t27os_version=0.3.0` and the four components | no |
| B | one prompt creates a session, LM Studio answers, the event is saved | yes |
| C | a session continues after a reload; a torn tail is ignored | live test: yes |
| D | one agent runs one bounded task; cancel stops it before the call | live test: yes |
| E | coordinator with at most 3 members, task-graph order, cancel | live test: yes |
| F | tool allowlist, approvals, secret redaction | no |
| G | atomic log save, missing log, cancel file, closed port | no |

Without LM Studio the live tests FAIL. They never pass on a mock and are never skipped.

**Measured 2026-11-17 (master 0839e9de7c): the suite cannot reach 17/17 today, with or without LM Studio.**
`native_host` gives 3 passed, 0 failed. `native_accept` gives 4 passed, 13 failed, with both
`./target/release/t27b` (build 0) and `~/.t27os/bin/t27b` (build 7609). Every failure ends with
`import sys_getenv | sys_mkdir | sys_gettimeofday run outside an object`: `t27b test` interprets the
spec and does not execute `@extern` imports, so every test that touches a file, the clock or a socket
fails, including the offline groups F and G. This is #13228 (host fakes: #13232). Until it is fixed,
report 0.3.0 as `VERSION_CREATED: NO` with these counts, and do not blame LM Studio for the 13 failures.

## 5. Where the files go and how to reset

Session, agent and swarm logs are written to `target/t27os/native-accept/` (gitignored). The fixed-id test
`c_a_session_file_outlives_the_run_that_wrote_it` appends 3 events per run. Running the suite twice is what
proves a real process restart. To start clean:

```bash
rm -rf target/t27os/native-accept
```

## 6. Pure specs, one by one

The six specs that reach no extern also run under `t27c`, one spec per call:

```bash
./target/release/t27c test-report specs/t27os/native_version.t27
```

Do the same for `native_provider`, `native_log`, `native_policy`, `native_agent` and `native_swarm`.

## 7. Known limits

- Replies are not streamed (the final answer only); the reply cap is 4096 bytes.
- Cancel is checked at boundaries: before the provider call and before each swarm member.
- Only the clock tool runs. Command and network tools are held for approval and never executed.
- At the 2026-11-07 report no acceptance test had been executed. CI scored `native_log` with 2 failing tests
  and `native_policy` with 1, still unidentified. Paste the `FAIL` lines from your run into
  Trinity-S-AI-DNA/t27#13212.
