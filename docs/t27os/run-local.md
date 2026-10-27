# Run T27 OS on a Mac: build, launch, drive a pane

Status: verified 2026-10-20 on an Apple-silicon Mac (macOS 26, Xcode SDK 26.5) from branch
`feat/t27os-petal-menu` at `4c451b9cf` (PR #10783). Epics: gHashTag/t27#11037 (mega), #10782 (product).
Placeholders: `<repo>` is a checkout of gHashTag/t27, `<scratch>` is any scratch directory outside it.

## 1. What gets built

T27 OS is two arm64 Mach-O executables. Both are compiled from `.t27` specs by `t27b`; there is no
generated C and no hand-written bridge for this app (the C bridge in `trios-app-t27-bridge` is a different
product). Every call into the system is an `@extern` import (`specs/t27os/pty_sys.t27` and the head of
`specs/t27os/glass_window.t27`), resolved by `ld`.

| Product | Spec (with its `use` closure) | Links |
|---|---|---|
| `T27OS.app` (window, 960 x 540 pt, title "TRINITY S3AI") | `specs/t27os/glass_window.t27` | libSystem, libobjc, Foundation, AppKit, CoreGraphics, CoreText, CoreFoundation, QuartzCore, Security, WebKit, ImageIO |
| `t27os-pty` (PTY server, one process, owns every agent terminal) | `specs/t27os/pty_server.t27` | libSystem |

The window does not start the server and does not bundle it. They meet on one Unix socket,
`~/.t27os/pty.sock`: the window builds the path from `NSHomeDirectory()` (`term_connect` in
`glass_window.t27`), the server from `$HOME` (`socket_path` in `pty_proto.t27`). With no server the Claude Code
pane in the window says "offline" and echoes keys.

Provider keys (the KEYS tab, up to 8 per provider) live in the macOS Keychain, service `ai.t27.os`, account
`<provider>`, then `<provider>#2` .. `#8`. The window and the PTY server reach them only by running
`/usr/bin/security` (`specs/t27os/keys_host.t27`: posix_spawn, pipes, poll, waitpid; the server reads the
Keychain itself at its start and at a spawn that finds no key for the pane's agent, at most once a minute,
so a pane spawned from a script while the window is closed still has its key; the log line is
`keys read from the Keychain: N`, a count, never a key), never through the SecItem calls, so
`make t27os` links Security.framework with the rest (the list is `APP_FRAMEWORKS` in `install.t27`); the key manager itself
still reaches the Keychain only through `/usr/bin/security`, not SecItem. The reason: the window is signed
ad hoc, every rebuild is a new code identity to macOS, and each SecItem read asked for the login password
again. `/usr/bin/security` is signed by Apple and never changes, so a read of an item it created shows no
dialog across rebuilds. A key is never put on argv (`ps` shows it): an add writes one line to the stdin of
`security -i`, and refuses a key with white space, a quote or a backslash. An add never replaces a key
(the account must be free); a delete removes the newest key only. Items that an older SecItem build made
trust that build, not `security`, so a read of one may still ask once: remove it in Keychain Access and add
the key again in the KEYS tab.

## 2. Prerequisites

- Apple silicon, Xcode installed and selected (`xcode-select -p`): `ld`, the SDK and `codesign`. Command Line
  Tools alone were not tried.
- `cargo`, to build `t27b` (it has one dependency, serde).
- Free disk: the whole `target/` of this build is 57 MB; the repository tree is the rest.
- `t27c` and `tri` are not needed to build the app.

## 3. Build

One command, from the repository root, on Apple silicon with Xcode selected:

```
make t27os            # builds t27b, both objects, the Info.plist, links, signs; 3 s when t27b is built
make -n t27os         # prints the plan
```

It writes `target/t27os/T27OS.app` and `target/t27os/t27os-pty` (the PTY server, a plain file beside the bundle).
What it decides lives in `specs/t27os/install.t27` ("the app target": products, the nine frameworks Foundation
AppKit CoreGraphics CoreText CoreFoundation QuartzCore Security WebKit ImageIO, minimum macOS 11.0, the refusals,
and `plist_text`, the function that writes `Info.plist`); the Makefile only names the same files and flags.

- **Info.plist** is generated: `specs/t27os/plist_main.t27` is built, linked and run by the rule, and prints
  `plist_text` with the version from `specs/t27os/release.t27` and the build number `T27OS_BUILD` (default
  `git rev-list --count HEAD`). `plutil -lint` checks it.
- **Signing** is ad hoc (`-`) unless `T27OS_SIGN_ID="Developer ID Application: ..."` names an identity. Ad hoc is a
  new code identity on every rebuild.
- **Refusals**: not Apple silicon; a build number that is not a plain commit count; an object with zero undefined
  symbols (`nm -u`), which means `t27b` has no `@extern` and the build would be trapping stubs. The `@extern`
  support is two commits of #8946 (`865b4ee3f`, `6113f868b`) that are on this branch's base; with them the
  window object emits in about 13 ms (an earlier quadratic walk made it 148 s; fixed in `specs/tri/t27b/macho.t27`).
- Run inside Claude Code's Bash sandbox, `ld` prints `couldn't create cache file ... xcrun_db (Operation not
  permitted)` and still links. Point `CARGO_TARGET_DIR` at a directory you may write if `target/` is not.

Iterating: the window watches its own executable (`hot_reload.t27`). Re-run `t27b build` and the second `ld`
straight onto `T27OS.app/Contents/MacOS/T27OS`, then `codesign`; the window waits for the file to settle, saves
the open screen and `exec`s itself. Not exercised in this lane.

### The hot loop: watch + the window's own reload

While iterating on the app, the hot loop needs no new build path -- both halves already exist.
In one terminal:

```
target/release/t27b watch specs/t27os/glass_window.t27 -o target/t27os/glass_window.o --macho --then 'make t27os'
```

On every save of a spec in the window's closure, `t27b watch` reruns the ordinary `make t27os`,
and the running window notices its own executable change and `exec`s itself
(specs/t27os/hot_reload.t27). The loop is: save a `.t27` spec, the app restarts with the new
build -- measured 2026-10-27: an edit to specs/t27os/queen_dash.t27 produced a full rebuild
ending `built target/t27os/T27OS.app` (build 7647) while the watch ran, with the open window
hot-swapping. `make t27os` alone stays the one-shot form. (A `t27os-watch` make target was tried
and reverted: the foreign-budget hook rightly refuses Makefile growth for what a documented
command already does.)

## 4. Launch

Run these in a normal Terminal (Claude Code's Bash sandbox blocks `open`, blocks `kill` of other processes and
refuses to connect to the socket).

```
# 1. the server first, detached from the tab. PATH must contain the directory of the `claude` CLI.
cd ~ && PATH=$HOME/.local/bin:$PATH <repo>/target/t27os/t27os-pty < /dev/null > <scratch>/pty-server.log 2>&1 &
# the log then reads: t27os-server: keys read from the Keychain: N  /  t27os-server: listening, restored 0
# 2. the app
open <repo>/target/t27os/T27OS.app
```

What the server creates: `~/.t27os/pty.sock`, `~/.t27os/sessions/` (one `pane-<id>.rec` per pane, read at the
next start to bring panes back), `~/.t27os/hooks.json` (Claude Code hooks that call the server's own binary by
its absolute path, so moving the binary needs a restart). Closing the tab that started the server did not
stop it.

Observed: process `T27OS` (parent launchd), a 960 x 540 black glass window with the 27-petal triangle, a tap
opens the chat screen (sidebar QUEEN / SPACES / AGENTS, footer "OPEN SOURCE T27", "DOCTOR n"). A Claude Code
chat opened in the window showed a live Claude Code TUI; the server listed it as `pane 0 agent 0` and its
process (`claude --session-id <uuid> --settings ~/.t27os/hooks.json`) was a child of the server. The app idles at
about 15 percent of one core (30 fps redraw, `#11025`) and 70 MB.

Stop: quit the app (Cmd+Q) or `kill <app pid>`; `kill <server pid>` stops the server (verified with no pane
open; by the spec its panes then get SIGHUP). Records in `~/.t27os/sessions` bring the panes back at the next
server start; kill a pane through the server (below) to drop its record.

## 5. Drive a pane from outside (the conductor interface)

`t27os-pty` is its own control client. Set `T27OS_CTL` to a verb (only the first letter counts) and run the
same binary; it connects to `~/.t27os/pty.sock`, makes one call and exits. The header comment of
`pty_server.t27` says `keys` for the fifth verb; the code reads the letter `w`, so the verb is `write`.

| Verb | Environment | Prints |
|---|---|---|
| `list` | none | one line per pane: `pane N agent A state S flags F` |
| `spawn` | `T27OS_AGENT=<id>`, `T27OS_CWD=<absolute path>`, optional `T27OS_MODEL` | `spawned pane N` |
| `write` | `T27OS_PANE=N`, `T27OS_KEYS=<text>` | `sent` (the text plus one carriage return) |
| `read` | `T27OS_PANE=N` | the raw bytes of the pane's scrollback |
| `kill` | `T27OS_PANE=N` | `killed` (drops the pane's record) |

Agent ids: 0 Claude Code, 1 Codex, 2 Gemini, 3 OpenCode, 4 Queen, 5 Crush, 6 Cursor, 7 Aider, 8 Copilot,
9 a plain `/bin/bash`. State: 0 stopped, 1 working, 2 blocked, 3 idle, 4 done. Flags: bit 0 live, bit 1 needs
you, bit 2 unseen. At most 16 panes.

Verified transcript (shell pane, from a Terminal tab):

```
$ T27OS_CTL=spawn T27OS_AGENT=9 T27OS_CWD=<repo> <repo>/target/t27os/t27os-pty
spawned pane 0
$ T27OS_CTL=write T27OS_PANE=0 T27OS_KEYS='echo HELLO $((6*7)); uname -sm; pwd' <repo>/target/t27os/t27os-pty
sent
$ T27OS_CTL=read T27OS_PANE=0 <repo>/target/t27os/t27os-pty
...bash-3.2$ echo HELLO $((6*7)); uname -sm; pwd
HELLO 42
Darwin arm64
<repo>
$ T27OS_CTL=kill T27OS_PANE=0 <repo>/target/t27os/t27os-pty
killed
```

A coding task goes in the same way: spawn a shell pane whose cwd is the lane worktree, write one line that runs
the agent CLI with its output redirected to a file, poll `read` for a sentinel, read the file with normal
tools, kill the pane. Verified up to the CLI starting (`claude --version` answered `2.1.235 (Claude Code)` in a
pane); the model call itself failed for the reasons in section 6.

```
T27OS_CTL=write T27OS_PANE=1 T27OS_KEYS="claude -p 'Reply with PONG.' > out.txt 2>&1; echo EXIT=\$?" <bin>
# poll: T27OS_CTL=read T27OS_PANE=1 <bin> | grep 'EXIT=[0-9]'      (the typed echo shows EXIT=$?, no digit)
```

A pane of agent 0 started this way is the one the window shows: `term_open` in `glass_window.t27` picks the k-th
pane of the chat's agent from `list` before it spawns its own. Not exercised for a second pane.

The wire protocol under it (`pty_proto.t27`, `pty_client.t27`), for a client written later in `.t27`: a frame is
`[op u8][pane u8][len u16 LE][payload]`, `len` at most 4096; ops LIST 1, SPAWN 2, ATTACH 3, DETACH 4, KEYS 5,
RESIZE 6, READ 7 (payload: cursor u64 LE; reply: new cursor then bytes), KILL 8, HOOK 9; replies have bit 7 set:
OK 128, ERR 129, DATA 130, LISTING 131.

## 6. Known gaps and traps

1. **No coding agent answers through a pane today (blocker).** A pane inherits the environment of whoever
   started the server. In this Mac's Terminal that holds `ANTHROPIC_BASE_URL`, `ANTHROPIC_API_KEY`,
   `ANTHROPIC_AUTH_TOKEN` and `ANTHROPIC_DEFAULT_{SONNET,OPUS,HAIKU}_MODEL` (names only were read, never the
   values). The Claude Code pane in the window showed `429 [1113] Insufficient balance or no resource package`
   with retries; `claude -p` in a shell pane printed two `unrecognized_model` warnings and no answer in 120 s.
   With a clean environment (`env -i HOME=$HOME PATH=$PATH claude -p ...`) it prints `Not logged in`. Needs an
   owner action on the provider balance or login; the KEYS tab and per-pane profiles (M12/M12b, PR #10977) and the
   child-environment strip (#11029) are not on this branch.
2. **`t27b` needs the extern commits** (section 3). A `t27b` without them builds a binary of trapping stubs with
   no error; `make t27os` now refuses such an object.
3. **The recipe is `make t27os`** (section 3); the Info.plist is generated, not hand-made.
4. **Nothing starts the server.** `T27OS_INSTALL=1 t27os-pty` writes
   `~/Library/LaunchAgents/ai.t27.os-server.plist` (login start, #11035); it is persistent configuration and was
   not used here.
5. **PATH trap.** The Terminal tool's shell had no `~/.local/bin`, so a server started from it cannot find
   `claude` and neither can the window's Claude Code pane. Start the server with PATH as above.
6. **`write` and `spawn` cut at 200 bytes, silently.** `env_text` in `pty_server.t27` caps `T27OS_KEYS` and
   `T27OS_CWD` at 200 bytes. A 238-byte command ran truncated and created a stray file. Keep the line short:
   start the pane in the output directory and use relative file names, or put the prompt in a file.
7. **`read` returns the oldest 4088 bytes of the 64 KiB ring** (cursor 0, no cursor option), never the tail. For
   anything longer than a screen, redirect to a file. `client_read` in `pty_client.t27` takes a cursor; the
   control mode does not expose it. There is also no resize, attach, or raw-control-key verb: `write` adds one CR.
8. **The control client cannot run inside Claude Code's Bash sandbox**: the connect is refused and it prints
   `no server` (exit 10). Run it through the Terminal tool.
9. **The Queen has no model.** Her panel is a fixed rule over the sidebar; cell 4 launches a command named
   `queen`, which is not on PATH here. The conductor is the outside orchestrator using section 5.
10. **The window wires the terminal pane to Claude Code (agent 0)** (`glass_window.t27`, "the terminal pane").
    Other agent cells were not tried.
11. **Load.** The app redraws at 30 fps while idle (about 15 percent of a core) and its DOCTOR scan walks the
    home directory; both are on the M11 list (#11025).

## 7. Releases and the update channel

Rules: `specs/t27os/release.t27` (every one tested). Version: `VERSION_MAJOR/MINOR/PATCH` in that file, the
build number is the commit count. The release is a GitHub release `v<version>` with two assets: `T27OS-<version>.zip`
and `t27os-release.t27`, the **manifest**, a t27 module (not JSON) with six constants: VERSION, BUILD, SHA256 of the
zip, MIN_MACOS, URL, NOTES. Its address is `https://github.com/gHashTag/t27/releases/latest/download/t27os-release.t27`;
every address in it must start with `https://github.com/gHashTag/t27/releases/`.

The update rule the app follows:

1. At start and every 6 hours it fetches the manifest, never while a pane is busy (a due check waits).
2. It offers an update only for a newer version that passes `manifest_check`, is not a version a rollback removed,
   and whose MIN_MACOS the Mac meets.
3. The zip is downloaded to `~/.t27os/updates` by a background curl (spawned, as `keys_host.t27` spawns `security`).
4. `shasum -a 256` of the zip must equal the manifest's SHA256, then the unzipped bundle must pass `codesign --verify`.
   A bad digest deletes the zip and waits for the next check.
5. When no pane is busy the running bundle moves to `~/.t27os/previous` (the one bundle kept) and the new one takes
   its place; `hot_reload.t27` sees the new executable and `exec`s it. The PTY server is a separate process and keeps
   its panes.
6. The swap leaves a `pending` marker; a start that lives 60 s removes it. A start that finds the marker (a crash within
   60 s) puts the previous bundle back, consumes it, and refuses that version from then on.

Status of this lane: steps 1-6 are decided and tested as rules in `release.t27` (21 tests); the window's host calls that
fetch, spawn and move are not wired into `glass_window.t27` yet.

To release (the plan prints, nothing publishes):

```
tri t27os release --plan        # build, zip, sha256, manifest, notes since the last tag, gh release create
tri t27os manifest --build N --sha H     # the manifest text, as step 4 of the plan uses it
```

Publishing needs the owner's yes (`publish_allowed`), and a Developer ID Application certificate for a release other
Macs will open without a Gatekeeper warning; neither exists in this repository.
