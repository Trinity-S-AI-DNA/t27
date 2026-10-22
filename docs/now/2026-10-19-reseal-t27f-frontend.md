# NOW -- re-seal specs/t27f/frontend.t27 (2026-10-19)

## .trinity/seals/t27f_T27fFrontend.json (Closes #10811)

- Why: 2e561c612 (#10782 M4) changed `specs/t27f/frontend.t27` without re-sealing it, so `tools/check_seal_coverage.py` failed on every pull request ("1 seal(s) newly do not hold: t27f_T27fFrontend.json [stale]").
- `t27c seal specs/t27f/frontend.t27 --save` with t27c built from master at this branch's base (it includes the gen-rust change 0131f3196), zig 0.16.0. The spec itself is not touched.
- After: `t27c seal --verify specs/t27f/frontend.t27` prints all hashes MATCH.
- `check_seal_coverage.py` with that t27c: 2883 seals, 2770 hold, 113 known-broken listed in seal_baseline.txt, none newly broken.
