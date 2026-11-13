# Golden Master

Clean production repository — системные файлы запуска .t27 + golden specs + сгенерированные артефакты, без experimental/work-in-progress кода.

## Promotion Pipeline

master (development + experiments) → candidate (specs passing local GOLDEN CHAIN) → golden-master (verified gold) → release (tagged deployable state)

## GOLDEN CHAIN Gate

Specs enter golden-master only if ALL pass:
1. t27b build PASS
2. tri test PASS
3. golden test PASS
4. regression PASS
5. seals/gates PASS
6. coverage delta recorded
7. no shell violations
8. no parallel-edit conflicts

## CI Fast Path

Minimal required checks for golden-master only:
- Fast compile check
- Fast test suite
- Optional heavy Coq proofs (nightly/optional track)

Not blocking release for heavy proof jobs if not required for runtime.

## CI Fast Path

For `golden-master` only minimal required checks run:

- Fast compile check
- Fast test suite
- Optional heavy Coq proofs (nightly/optional track)
- Not blocking release for heavy proof jobs if not required for runtime

## GOLDEN CHAIN Gate — Detailed

Specs enter `golden-master` only if ALL of the following pass:

1. **t27b build PASS** — `cargo build --release -p t27b` succeeds on target platform
2. **tri test PASS** — `./target/release/t27c suite --repo-root .` or `tri t27b test <spec>` passes
3. **golden test PASS** — Golden-specific validation (seals, coverage, invariants)
4. **regression PASS** — No regressions vs baseline (run `tri diff` or equivalent)
5. **seals/gates PASS** — All seals and gates pass (own-language, conformance, dupe-ratchet, etc.)
6. **coverage delta recorded** — Coverage change documented in `docs/reports/coverage/`
7. **no shell violations** — No new `.sh` files on critical path; existing `.sh` budget respected (at most 40 added lines per file, 80 per PR per owner rule 2026-10-07)
8. **no parallel-edit conflicts** — No concurrent edit conflicts with other working branches

## Promotion Pipeline

```
master (development + integration + experiments)
    │
    ↓
candidate (specs passing local GOLDEN CHAIN — run `tri t27b ready`)
    │
    ↓
golden-master (verified gold — promotion automatic after green)
    │
    ↓
release (tagged deployable state — `tri golden land` or equivalent)
```

## Branches Relationship

| Branch | Role | Content |
|---|---|---|
| `master` | Development + integration | Full code, experiments, WIP, new specs |
| `candidate` | Pre-golden | Specs after local GOLDEN CHAIN verification |
| `golden-master` | Production gold | Only GOLDEN CHAIN-verified content — production baseline |
| `release` | Deployable | Tagged golden state — what's actually deployed |

## Adding New Specs

1. Write spec in `specs/` following LANG-EN (Law L3) and TRI invariants
2. Run `tri t27b ready` locally — must pass ALL 8 GOLDEN CHAIN gates
3. Push to `candidate` branch
4. Promotion to `golden-master` is automatic after all checks green
5. Tag release on `golden-master` when ready for deploy

## CI Acceleration for golden-master

The golden-master CI fast path includes:

- **Caching**: opam package cache, compiler artifacts, test results
- **Path filters**: Only touched specs rebuilt (changed since last golden commit)
- **Fail-fast**: Early exit on first gate failure
- **Matrix splitting**: Fast compile/test vs heavy Coq proofs (separate nightly job)
- **Artifact reuse**: Between dependent jobs (build → test → seal)
- **Required vs advisory**: Only required gates block promotion; advisory gates inform but don't block
- **Nightly Coq proofs**: Heavy Coq proof compilation runs nightly, not on every PR
