# NOW -- the simulation gate plays keyed actors, two bee dispatchers and the adaptive reviewer pool (2026-10-11)

## specs/queen/simulation.t27 (Closes #8987; gHashTag/trios#1763, ledger gHashTag/trios#1729)

- seed_features gains two bits: the adaptive pool and the lane queue. The 16 gate seeds now hold each of the eight combinations (fixed 4 or adaptive, queue or not, turn stop or not) twice. The store seeds keep the fixed 4.
- Section 7 is the keyed world. Two dispatchers of one deployment run over one lease table. Their faults: redeploys while claims hang, admission and issue-actor crashes, stalled policy answers, silent runner deaths and hangs. Call-chain probes run past slot 63, a turn is deaf to its abort, and every second seed's timers fire up to 3 ms early.
- Six invariants are new:
  - duplicate claim;
  - over lanes;
  - not finished;
  - lane leak;
  - call chain;
  - stop overran.

  Eleven rare states are new. The first runs found two runtime defects, fixed in the trios PR:
  - a lane given back while a claim was out;
  - a call timer that fires early ended its call pending.
- `t27c test-report`: 20 tests, 0 FAIL, 0 vacuous, 22 invariants. 24 negative controls: 19 FAIL by name, 5 blocked by an invariant.
