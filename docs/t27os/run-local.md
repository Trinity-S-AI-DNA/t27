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
| `T27OS.app` (window, 960 x 540 pt, title "TRINITY S3AI") | `specs/t27os/glass_window.t27` | libSystem, libobjc, Foundation, AppKit, CoreGraphics, CoreText, CoreFoundation, QuartzCore |
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
**Security.framework is not linked** and the link line in section 3 is complete (measured 2026-10-20: the
SecItem build did not link without `-framework Security`; this one does). The reason: the window is signed
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

`t27b` on master and on the petal branch does NOT implement `@extern` in `--macho` objects. Without it the
build still "succeeds" and writes an object with no undefined symbols whose every import is a trapping stub
(`otool -tv` shows `brk` in `_sys_close`; `nm -u` is empty). The feature is two commits on
`origin/feat/t27b-host-boundary` (#8946), not on master:
`865b4ee3f` (extern boundary, Mach-O undefined symbols and BRANCH26) and `6113f868b` (regenerated Rust).

```
cd <repo>
git fetch origin feat/t27os-petal-menu feat/t27b-host-boundary
git checkout --detach origin/feat/t27os-petal-menu
git cherry-pick --no-commit 865b4ee3f 6113f868b        # applied, never committed on this branch
cargo build --release -p t27b                          # about 2 min on this Mac, target/release/t27b

T27B=<repo>/target/release/t27b ; OUT=<repo>/target/t27os ; mkdir -p $OUT
$T27B build specs/t27os/pty_server.t27   -o $OUT/pty_server.o   --macho    # 0.9 s, 64 KB object
$T27B build specs/t27os/glass_window.t27 -o $OUT/glass_window.o --macho    # 16 s, 183 KB object
nm -u $OUT/pty_server.o      # must list _forkpty, _write, ... ; an empty list means the extern commits are missing

SDK=$(xcrun --show-sdk-path) ; SDKV=$(xcrun --show-sdk-version)
ld -arch arm64 -platform_version macos 11.0 $SDKV -syslibroot $SDK -e _main -lSystem \
   -o $OUT/t27os-pty $OUT/pty_server.o
ld -arch arm64 -platform_version macos 11.0 $SDKV -syslibroot $SDK -e _main -lSystem -lobjc \
   -framework Foundation -framework AppKit -framework CoreGraphics -framework CoreText \
   -framework CoreFoundation -framework ImageIO -framework QuartzCore -o $OUT/T27OS $OUT/glass_window.o

mkdir -p $OUT/T27OS.app/Contents/MacOS && cp $OUT/T27OS $OUT/T27OS.app/Contents/MacOS/T27OS
# $OUT/T27OS.app/Contents/Info.plist needs: CFBundleExecutable T27OS, CFBundleIdentifier ai.t27.os,
# CFBundleName T27OS, CFBundlePackageType APPL, CFBundleVersion 1, LSMinimumSystemVersion 11.0,
# NSHighResolutionCapable true
codesign --force --sign - $OUT/T27OS.app ; codesign --force --sign - $OUT/t27os-pty
otool -L $OUT/T27OS.app/Contents/MacOS/T27OS            # the 7 libraries above
```

Run inside Claude Code's Bash sandbox, `ld` prints `couldn't create cache file ... xcrun_db (Operation not
permitted)` twice and still links; the products exist and `otool -L` shows the right libraries.

Iterating: the window watches its own executable (`hot_reload.t27`). Re-run `t27b build` and the second `ld`
straight onto `T27OS.app/Contents/MacOS/T27OS`, then `codesign`; the window waits for the file to settle, saves
the open screen and `exec`s itself. Not exercised in this lane.

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
2. **`t27b` needs the extern commits** (section 3). Master's `t27b` builds a binary of trapping stubs with no
   error.
3. **No recipe in the repository.** The build is a scratch script around `t27b` and `ld`; `Info.plist` is a
   hand-made bundle manifest. A `tri` target or a spec that generates the bundle does not exist.
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
