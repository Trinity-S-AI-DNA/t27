---
id: dev-monitor
name: Dev monitor
description: On dev-mode entry, find the EXISTING DeepSeek Harness (dsh) web instance, open or focus its authorized UI once, verify the workspace and report healthy / degraded / blocked -- never a second instance, never the credential-bearing URL in a transcript, never a kill. Load when the owner or a session says "dev mode", "open dsh", "open the harness", "dev monitor", "is dsh running", "dsh web authentication required", or before any command that would start, restart or re-port dsh web. First milestone only (dev entry -> existing authorized dsh -> workspace confirmed); T27 OS panes, a notification inbox, host resources and the TECH TREE are later lanes.
argument-hint: "[enter | status | sign-in]"
---

# Dev monitor: enter dev mode into the existing dsh web (lane #12976 of the Mother epic #12945)

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

The law is `specs/t27os/dev_monitor.t27` (module `T27osDevMonitor`): the state machine, the actions, the verdict and the once rule, one `test` per row of the acceptance table. This page is the order of moves around it. Every shell line here is a one-line pipeline of tools (`launchctl`, `lsof`, `grep`, `curl`, `open`), not code in another language; keep each under 110 characters (skill `short-terminal-commands`).

## What dsh web is, as found on 2026-10-31

- One launchd job, label `ai.t27.dsh-web` (plist in `~/Library/LaunchAgents`, WorkingDirectory `~/deepseek-harness`), runs `dsh web --no-open` from the pinned checkout and listens on `127.0.0.1:3080`. Its stdout and stderr append to `~/.dsh/web-service.log`.
- Each start mints ONE random launch token, kept in memory only. One GET of the ready URL (the `?token` query) mints a signed cookie; every other cookie-less GET answers `401` with `dsh web authentication required; reopen the URL printed by dsh web.`
- The ready URL is printed once per start as a `dsh web: http...` line. The LAST such line in the log belongs to the running process. It is a credential: it is handed to `open`, never printed, quoted, screenshotted, pasted or saved.
- The hint "terminal tab c42" in older notes is stale: that tab belonged to the hand restart of 2026-10-31 12:32 and is gone. The launchd job is the only home of dsh web now; `launchctl` is the discovery, not a terminal tab.

## The verified path (2026-10-31)

The open move of step 3 below was run against the running instance (launchd job `ai.t27.dsh-web`, pid 97706) and the owner confirmed by eye at 13:45 local that the dsh workspace renders in the opened tab. From the shell side the same run showed the log growing by the `model listing` line within the window and the listener pid unchanged. So: the path is proven end to end for the running instance; the shell-side render check stays a heuristic, and the owner looking at the tab is the only full verification.

## The moves (one dev entry)

1. **Discover.** Three reads, no writes:
   - `launchctl print gui/$(id -u)/ai.t27.dsh-web | /usr/bin/grep -E 'state =|pid ='` -> `state = running` and the job pid, or an error when the job is not loaded.
   - `lsof -nP -iTCP:3080 -sTCP:LISTEN | tail -n +2 | awk '{print $1, $2}'` -> the listener's command and pid, or nothing when the port is free.
   - `curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:3080/` -> `401` (up, authentication on), `200` (a cookie was present), `000` (no answer).
   - Use `/usr/bin/grep`: zsh aliases `grep` to ugrep on this Mac.
2. **Decide** with `state()` and `action()` of the law:

   | job pid | listener pid | GET / | state | action |
   |---|---|---|---|---|
   | any | set, not the job pid | any | `S_PORT_HELD` | `A_STOP`: stop and report; no kill, no restart, no other port |
   | 0 | 0 | any | `S_NOT_RUNNING` | `A_REPORT`; `A_START` only under an explicit ALLOW (below) |
   | set | the job pid | `000` | `S_NO_ANSWER` | `A_REPORT` |
   | set | the job pid | `401` or `200` | `S_RUNNING` | marker absent: `A_OPEN`; marker present: `A_FOCUS` |
   | set | the job pid | the browser itself asks for authentication | `S_AUTH_REQUIRED` | `A_SIGN_IN` |

3. **Open once (`A_OPEN`).** Count the log, hand the last ready line to `open`, write the marker:
   - `wc -l < ~/.dsh/web-service.log`
   - `open "$(/usr/bin/grep -o 'dsh web: http[^ ]*' ~/.dsh/web-service.log | tail -1 | cut -c10-)"`
   - `mkdir -p target && date -u +%FT%TZ > target/dev-monitor.opened`
   The marker lives under the gitignored `target/` tree of the worktree and is never committed. One marker per dev entry; a new dev entry (a new session, or the owner's word) removes it first. Do not open the URL in the in-app browser, Chrome, BrowserOS or any MCP browser: that puts the credential into the transcript.
4. **Repeat entry (`A_FOCUS`).** Marker present: bring the default browser forward without a URL (`open -a` with the browser's name) or just report the state. No new tab, no new process, no second `open` of the ready URL.
5. **Verify the workspace (heuristic).** Within 20 s of the open the UI fetches its model list and the log grows by a `model listing` line:
   - `sleep 15; wc -l < ~/.dsh/web-service.log`
   - `tail -n 40 ~/.dsh/web-service.log | /usr/bin/grep -v -i -E 'token[=:]|api.?key|secret|passw|sk-[a-z]|bearer' | /usr/bin/grep -c 'model listing'`
   A count of 1 or more is `UI_PASS_LOG`; the owner saying the workspace is on screen is `UI_PASS_EYE`; nothing is `UI_FAIL`. The server keeps no HTTP access log, so the line is the only machine-readable evidence; the report says "heuristic" unless the owner confirmed by eye. Never launch a model call to test that the page opens.
6. **Prove no second instance.** The `lsof` line of step 1 again: the same listener pid as before, equal to the job pid (`pid_unchanged()` of the law).
7. **Verdict** (`verdict()` and `reason()`): `healthy` = `S_RUNNING` and the check passed; `degraded` = the UI opened, or asked to sign in, but the check did not pass; `blocked` with its reason otherwise: `not running`, `port held`, `no answer`, `start denied`.

## Authentication required (`A_SIGN_IN`)

The signal is the browser's own message on screen (the owner sees it, or the page text reads `dsh web authentication required`). A `401` from `curl` is NOT that signal: a cookie-less GET always answers 401. The move is the `open` line of step 3 again; the running process still holds its token, so its last ready line still signs in. Never:
- print, echo, `cat`, `tail` or screenshot the ready line (every log excerpt passes the `grep -v` filter of step 5; `grep -c 'dsh web: http'` is safe, it counts and stops before the token);
- navigate any browser tool to the URL;
- open `~/.dsh/.credentials.yaml` or `~/.dsh/render-local.env`, disable authentication, or persist the URL anywhere (not chat, memory, notes, GitHub, receipts, screenshots, skills).
When `open` is unavailable (no desktop session, an SSH shell), ask the owner to open the printed URL themselves and report `degraded`.

## Not running (`A_REPORT`, `A_START`)

The default is DENY: report `blocked: start denied` with the `launchctl` state line and name who decides (the owner). Start only when the dev-mode startup policy says ALLOW in the owner's own words for this entry, and then only:
- in the pinned checkout `~/deepseek-harness`, which stays read-only for the agent (nothing committed there, no second clone anywhere);
- with the verified command `cd ~/deepseek-harness && pnpm dsh web --no-open`, in a terminal tab the owner can see;
- never `pnpm install`, never an upgrade, never another port.
After a permitted start the first open is step 3 as usual: the new process prints a new ready line, and the old lines in the log are dead.

## Port held by another process (`A_STOP`)

Listener pid not equal to the job pid, or a listener with no job: stop. Report both pids and the listener's command name from the `lsof` line, nothing more. No `kill`, no `pkill`, no `launchctl kickstart -k`, no restart of any of the owner's launchd jobs, no other port. The decision is the owner's.

## The report: verified first, then what is not available

One block, in this order:
- `job`: state and pid from `launchctl`; `listener`: command and pid; `GET /`: the code.
- `action` taken, `verdict` and its reason.
- `pid before / after`: equal or not.
- `workspace check`: `UI_PASS_LOG` (heuristic: the `model listing` line within 20 s), `UI_PASS_EYE`, or `UI_FAIL`.
- `not visible`: external lanes and T27 OS actors are not connected to dsh; nothing in this window shows them. Say it every time until an integration is implemented and verified (`EXTERNAL_LANES_VISIBLE` and `T27OS_ACTORS_VISIBLE` in the law are `false`; `may_claim_visible()` refuses the claim).
The window shows the actual dsh sessions and tool activity; the monitor invents no activity.

## Traps

- `t27c check specs/t27os/dev_monitor.t27` typechecks the law and `t27b test specs/t27os/dev_monitor.t27` runs its tests; `t27c test` only lists them. Both run from the terminal tool on this Mac (the heavy-run hook denies `t27b test` in Bash, and it denies any Bash command whose text merely contains the word).
- The owner's launchd job (pid 97706 on 2026-10-31) and every other launchd job are the owner's: never kill or restart them.
- 403 lines on `/auth-api/v0/users/current` in the log are the DeepSeek-account plugin talking to its own backend, not a dsh web authentication failure.
- `grep -o 'dsh web: http[^ ]*'` prints the token; only the `open "$(...)"` form of step 3 may carry it. Everything else that reads the log counts (`grep -c`) or filters (`grep -v`).
- Measured 2026-10-31: a second `open` four minutes after a first one (same running process, same browser) grew the log by 10 lines within 15 s, all of them the plugin's 403 blocks, and no `model listing` line; one such line arrived about two minutes later, outside the window, and this run cannot tell the open from a periodic refresh. The first open had produced two such lines. The heuristic then says `degraded`, and only the owner's eye upgrades it.
- Count the NEW lines (`sed -n 'N,$p'` from the `wc -l` taken before the open), not `tail -n 40`: a `tail` count of `model listing` can be nonzero from an earlier open and pass the check for you.
- A clone of dsh anywhere but `~/deepseek-harness` is litter (CLAUDE.md, "Where t27 work lives"); do not make one to look at the UI.
