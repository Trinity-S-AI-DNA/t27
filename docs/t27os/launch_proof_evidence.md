# Real launch proof: evidence and reproduction (launch_proof.t27)

PROVEN: one real process launch from a `.t27` spec -- admission, real host
binding, a real OS process, observed output, and verification. This file is
the evidence record; the spec is `specs/t27os/launch_proof.t27`. It is a
proof, not production integration: `pty_sys.t27` is untouched, no adapter
added, no commit to stable paths.

## Commands (exact)

Build:
```
t27b build specs/t27os/launch_proof.t27 -o target/t27os/launch_proof.o --macho
```

Link:
```
ld -arch arm64 -platform_version macos 11.0 26.5 \
  -syslibroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk \
  -e _main -lSystem \
  -o target/t27os/launch_proof target/t27os/launch_proof.o
```

Run (allowed):
```
T27OS_TARGET=/bin/echo ./target/t27os/launch_proof
```

Run (denied):
```
T27OS_TARGET=/usr/bin/wc ./target/t27os/launch_proof
T27OS_TARGET=/bin/sh ./target/t27os/launch_proof
```

## Raw transcript (2026-11-10)

Allowed run (fresh repro, PID differs per run):
```
$ T27OS_TARGET=/bin/echo ./target/t27os/launch_proof
T27-REAL-LAUNCH-PROOF      (stdout, from the real /bin/echo)
SPAWNED pid=2671           (stderr)
exit=0
```

Original run of the same proof: `SPAWNED pid=6465`, same stdout token,
exit 0.

Denied runs (fresh repro):
```
$ T27OS_TARGET=/usr/bin/wc ./target/t27os/launch_proof
DENIED                     (stderr)
exit=7
$ T27OS_TARGET=/bin/sh ./target/t27os/launch_proof
DENIED                     (stderr)
exit=7
```

## Evidence table

| Check | Result | Evidence |
|---|---|---|
| Admission | PASS | `/bin/echo` allowed; `/usr/bin/wc` and `/bin/sh` denied |
| Real host binding | PASS | undefined symbols at link: `__exit _getenv _memmove _posix_spawn _strlen _waitpid _write` -> libSystem |
| Real OS process | PASS | `SPAWNED pid=2671` (original: 6465) |
| Observed output | PASS | stdout `T27-REAL-LAUNCH-PROOF` |
| Exit status | PASS | exit 0, waitpid returned the exact spawned pid |
| Pre-launch denial | PASS | `DENIED`, exit 7, no spawn call |
| Simulation separation | PASS | `adapter_spawn_direct.t27` relabelled behavior-model only |
| Production changes | NONE | `pty_sys.t27` untouched; stable commits untouched |

## Exit codes

0 allowed run, pid match; 1 missing/oversized `T27OS_TARGET`; 2 spawn
failure; 3 waitpid failure; 4 pid mismatch; 7 denied before spawn.

## What this does NOT claim

The 8 tests in `specs/t27os/adapter_spawn_direct.t27` are behavior-model
tests with simulated syscalls; they are not runtime evidence. The live PTY
server (`~/.t27os/t27os-pty`) was not disturbed. This proof is a bounded
one-program demonstration, not the production spawn path.
