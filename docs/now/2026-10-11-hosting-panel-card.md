# NOW -- specs/hosting/panel.t27: the trios app's hosting panel as a card (2026-10-11)

## specs/hosting/panel.t27 (Closes #8975; gHashTag/trios#1756)

- Every rule the trios macOS app needs to lend a Mac and show what it earned: the pinned agent (`trios-host-v0.1.1`, darwin-arm64, SHA-256 and size through `host.t27 download_kept`), the URLs the app may fetch, the agent's life (off, starting, registered, beating, lapsed, stopping, exited), how its log lines are read, the jobs buckets, CPU and online time, the per-kind ledger tally, today's and the latest signed epoch, the statement verdict and the fixed disclaimer.
- It reaches Swift through `t27c gen-c`, a static library and a Clang module map in the app's Make step; `t27c gen-swift` is #8976.
- `t27c test-report`: 17 tests, 0 FAIL, 0 vacuous, 1 invariant; t27b 0.1.0's pinned binary passes all 17. 8 negative controls, each FAIL by name.
