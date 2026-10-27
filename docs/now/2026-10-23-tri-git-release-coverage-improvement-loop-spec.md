# NOW -- tri git release coverage + improvement-loop spec (2026-10-23)

## tri git release coverage + improvement-loop spec (Closes #11734)

- compiler/cli/git.t27: parse OK, coverage 0% -> 100% (12/12), typecheck 0 errors/0 warnings, 30 test/invariant/bench declarations, seal MATCH
- NEW specs/pipeline/loop_guard.t27: idempotency, green-only, preservation, issue-binding and anomaly-severity gates as a TDD spec (L7: loop logic in .t27, not shell)
