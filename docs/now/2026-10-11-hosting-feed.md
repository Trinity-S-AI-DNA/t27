# NOW -- specs/hosting/feed.t27: the network feeds itself, re-running only what changed (2026-10-11)

## specs/hosting/feed.t27 (Closes #8973; gHashTag/trios#1756, release plan gHashTag/trios#1763)

- Which master heads are fed (poll at most every 300 s; a head once it has been the head for 600 s; one open feed; 2 an hour), which specs of a commit need shards (per half: new, another toolchain, voided by a poisoned audit, changed closure, base half no vote, 1-in-128 audit), how the corpus receipt is assembled with each half's provenance, when it is complete, and how it is compared with the t27b lab's receipt (equal / different / missing / extra / excused; agrees only when complete and on the pinned toolchain).
- Measured on 21 consecutive pairs of t27b-lab master runs: where a half's toolchain sources held, 0 of 16556 reference and 0 of 14742 t27b carried halves differed from the lab's later row; carrying by closure alone would have carried 233 halves the lab saw change. `retried_after_timeout` flips with the lab's load and is never compared.
- `t27c test-report`: 10 tests, 0 FAIL, 0 vacuous, 91 runtime asserts, 2 invariants; `t27b test`: 10 pass. 26 negative controls, each failing a test by name. Card: gen-c + zig wasm32-wasi, 967 B, two builds one hash.
