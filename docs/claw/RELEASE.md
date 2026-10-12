# TRI CLAW release checklist

TRI CLAW -- TRI-27 / Trinity S3AI continuous gates. One place that says what a
release of it contains and what must be true before it ships. The version of
record is the line under this paragraph; a release PR changes it and ticks the
boxes in the same commit.

Current: **v0.1 (unreleased)**

## What v0.1 is

Gates as a spec, running as CI, sold as a service later:

- `specs/claw/gates.t27` -- the SSOT inventory: every CI gate as data (name,
  class, trigger events), with invariants and tests. 67 gates at v0.1.
- `scripts/tri_loop/claw.py` + `.github/workflows/claw.yml` -- the reader and
  the drift gate: CI fails when a workflow exists that the spec does not know,
  or the spec names one that does not exist.
- `docs/claw/app-manifest.yml` + `docs/claw/APP-SETUP.md` -- the GitHub App
  one-click manifest and the owner-only setup guide.
- `docs/claw/README.md`, `docs/claw/competitors.md` -- the regeneration recipe
  and the competitor landscape with the pricing hypothesis.

## Before tagging v0.1

- [ ] PRs #9277 (reader, drift gate, manifest) and #9279 (policy entries) merged.
- [ ] `matrix-drift` green on master (the drift gate watching itself).
- [ ] Documented-commands gate green (the `tri claw` mention resolves).
- [ ] Epic #9271 tasks T1-T4 done; T5 (Railway service skeleton) deployed or
      explicitly deferred to v0.2 in the release notes.
- [ ] T7 verified: auto-merge-when-green fires on a claw PR end to end.
- [ ] T8 coverage report: every gate class in `gates.t27` is exercised by at
      least one spec test (run `tri claw report` and attach it).

## Release steps

1. Update the version line above and tick the boxes; land that PR.
2. Tag `claw-v0.1` on master. The t27o cosign signing lane signs whatever the
   tag carries; TRI CLAW's artifact is the repo state, so the tag is the
   release.
3. Owner creates the GitHub App from the manifest (two minutes,
   `docs/claw/APP-SETUP.md`) -- one click, nothing else blocked by it.
4. Owner decides pricing (hypothesis in `competitors.md`: Solo 15 / Team 25 /
   self-host 500 USD per month) and whether v0.1 is announced at all.

## Out of scope for v0.1 (v0.2 candidates)

- The App acting as the CI identity (today gates run as GitHub Actions; the
  App reads and reports, it does not replace Actions).
- The t27c-lowering reader (v0.1 parses the spec's arrays textually; the
  compiler-backed reader ships with the App service).
- Marketplace listing and any public pricing page.
