# t27o as a service: signed binary + reusable golden-spec Action (#8996, epic #8998)

t27o (Ouroboros, `tri t27o`, #8967) converts a repository's Rust to golden `.t27`
specs with recorded oracle vectors -- zero LLM tokens. This lands the shipping
surface so a foreign repository can consume it entirely inside that repository's
own GitHub Actions: no servers of ours, no secrets of ours, no untrusted code
executed by us.

## What shipped

- `.github/workflows/t27o-release.yml` -- on tag `t27o-v*`: builds `tri` +
  `t27c` release binaries, writes checksums (bare filenames, so `shasum -c`
  works at the download site), signs every artifact cosign-keyless
  (`sign-blob --bundle`, GitHub OIDC id-token, **no owner secret**), attests
  build provenance, uploads everything to the GitHub Release. It signs
  in-workflow on purpose: a release created with GITHUB_TOKEN never fires
  `on: release` workflows (GitHub anti-recursion), so the repo-wide
  `sign-release.yml` cannot cover this path.
- `.github/workflows/t27o-golden.yml` -- reusable (`workflow_call`) workflow a
  foreign repo calls. Downloads `tri`, `t27c`, checksums from the release,
  verifies every blob's sigstore bundle against the exact OIDC identity
  `.../t27o-release.yml@refs/tags/<version>`, then runs
  `tri t27o <file> --check --t27c ./t27c` over the caller's Rust files
  (default pathspec `*.rs`). Modes: `golden` (default) opens a PR containing
  only `specs/t27o_check/` -- specs carry the per-file coverage line and every
  refusal inline; `scan` reports in place; `check` fails on drift between the
  committed specs and regeneration. Zig 0.16.0 (test-report) and rustc (oracle)
  are installed on the caller's runner.
- `specs/tools/t27o/service.t27` -- the LOGIC the plumbing carries: modes,
  defaults, tag family, OIDC issuer + certificate identity, report shape, the
  honest limit ("equivalent on the recorded oracle vectors and translated
  tests, not a proof for all inputs"). Golden *criteria* stay in #8997's lane;
  this file points at them. 4/4 tests green (`t27c test-report`, lab).
- `specs/port/scripts/gen_w374.t27` -- the retired `scripts/gen_w374.py` (and
  its successors' tables, w375/w376) ported to the blessed gen_w392 shape, one
  step further: the invariant-line end, last-block selection, and the splice
  are real, and the chain test (161 runtime asserts) proves the port produces
  the Python script's exact result. 8/8 tests green (lab). The three `.py`
  files are deleted.

## Validation done without secrets

- Both `.t27` specs: `t27c test-report` on the Railway lab, 4/4 and 8/8, exit 0.
- `tri t27o --check` end-to-end on the lab twice: a converting sample (3/3
  oracle tests, coverage 100%, exit 0) and a refusing sample (`async` +
  generics: spec still written with `// t27o: refused ...` comments, exit 0 --
  refusals surface in the PR, they do not fail the job).
- Both workflow YAMLs parsed and structurally checked (jobs, inputs, defaults,
  permissions, PR body) locally; actionlint is not on the lab.
- str `.len` includes the `$` sentinel (probe on the lab) -- the service spec's
  length asserts encode that, with the probe noted in the commit.

## Untested until the first real release run

cosign keyless signing against live OIDC, the release upload, the
download-verify chain from a foreign runner, and create-pull-request in a
caller's repository. The first needs the owner to push `t27o-v0.1.0`; the
checklist is on #8996.

## Hand-written foreign lines

- gained: 40 (t27o-golden.yml) + 25 (t27o-release.yml) = **65**, each file
  under the 40-line per-file cap, 65 under the 80-line diff cap.
- lost: scripts/gen_w374.py + gen_w375.py + gen_w376.py = **123** deleted.
- net **-58** foreign lines; their logic lives in `specs/port/scripts/gen_w374.t27`.
