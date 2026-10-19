# CLAUDE.md -- t27os

The rules are the t27 repository's rules. They are pointed at here, never copied:

- Language policy: `specs/policy/own_language.t27` in gHashTag/t27. Only `.t27`, files t27c
  generates, prose and tool-written data. No hand-written code in other languages, scratch included.
- Secrets: none in the tree, ever. The canonical gitleaks rules are `.gitleaks.toml` in gHashTag/t27.
- English only on GitHub: issues, PRs, commits, code comments.
- Builds and tests run on the lab, not on a laptop.
- CI and release gates: TRI CLAW, `docs/claw/README.md` in gHashTag/t27 (epic #9271).
  Do not hand-write workflow YAML. The TRI CLAW App proposes it on install (`specs/tools/t27o/install.t27`).
- Every PR references an issue. Milestones are in gHashTag/t27#10782.

## What lives here

- `specs/os/home.t27`: the T27 OS home, composed from the T27F petal menu.
- The petal menu itself lives in gHashTag/t27 `specs/t27f/petal_menu.t27`. Change it there.
