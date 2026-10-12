# TRI CLAW release checklist

TRI CLAW -- TRI-27 / Trinity S3AI continuous gates. One place that says what a
release of it contains and what must be true before it ships. The version of
record is the line under this paragraph; a release PR changes it and ticks the
boxes in the same commit.

Current: **v0.1 (unreleased)**

## What v0.1 is

Gates as a spec, running as CI, sold as a service later:

- `specs/claw/gates.t27` -- the SSOT inventory: every CI gate as data (name,
  class, trigger events), with invariants and tests. 66 gates at v0.1.
- `claw_status` in the same spec, lowered by `t27c gen-c` into
  `gen/c/claw/gates.c` and run by the claw-drift step of the gate-topology
  workflow: CI fails when a workflow exists that the spec does not know, or
  the spec names one that does not exist. No hand-written reader -- the law
  is the spec, the four-line harness is machinery (owner rule 2026-10-13).
- `docs/claw/README.md`, `docs/claw/competitors.md` -- the regeneration recipe
  and the competitor landscape with the pricing hypothesis.

## Before tagging v0.1

- [ ] PRs #9277 (drift audit in .t27 + the gate-topology step) and #9279
      (policy entries) merged.
- [ ] Claw-drift step green on master (the drift gate watching itself).
- [ ] Epic #9271 tasks T1-T4 done; T5 (service contract spec) merged; the
      Railway skeleton explicitly deferred to v0.2 in the release notes.
- [ ] T7 verified: auto-merge-when-green fires on a claw PR end to end.
- [ ] T8 coverage report: every gate class in `gates.t27` is exercised by at
      least one spec test (generate from the spec and attach it).

## Release steps

1. Update the version line above and tick the boxes; land that PR.
2. Tag `claw-v0.1` on master. The t27o cosign signing lane signs whatever the
   tag carries; TRI CLAW's artifact is the repo state, so the tag is the
   release.
3. Owner creates the GitHub App when the service ships (the manifest is a
   v0.2 step, written then, not carried dead now -- a manifest with no App
   behind it is a false claim).
4. Owner decides pricing (hypothesis in `competitors.md`: Solo 15 / Team 25 /
   self-host 500 USD per month) and whether v0.1 is announced at all.

## Out of scope for v0.1 (v0.2 candidates)

- The App acting as the CI identity (today gates run as GitHub Actions; the
  App reads and reports, it does not replace Actions).
- The t27c-lowering reader (v0.1 parses the spec's arrays textually; the
  compiler-backed reader ships with the App service).
- Marketplace listing and any public pricing page.
