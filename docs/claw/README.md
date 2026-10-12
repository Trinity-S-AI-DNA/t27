# TRI CLAW

TRI CLAW — TRI-27 — Trinity S3AI: every CI gate of this repository, packaged
behind one front door, with the gate logic in `.t27` specs. Epic
[gHashTag/t27#9271](https://github.com/gHashTag/t27/issues/9271).

## What exists today

| Piece | Where | Status |
|---|---|---|
| Gate inventory (SSOT) | `specs/claw/gates.t27` | T1, this branch |
| `tri claw` CLI | `tri claw status / run / report` | T2, planned |
| Unified reporting workflow | `.github/workflows/claw.yml` | T3, planned |
| GitHub App manifest | `docs/claw/app-manifest.yml` | T4, planned |
| Service skeleton | Railway, thin I/O glue only | T5, planned |
| Positioning / pricing | `docs/claw/competitors.md` | T6, started |

## The inventory spec and how to regenerate it

`specs/claw/gates.t27` holds three parallel `[65]str`-shaped arrays — `NAMES`
(workflow file stems), `CLASSES` (0 blocking / 1 soft / 2 reporting), `EVENTS`
(trigger keys, comma-joined). Rows stay alphabetical and aligned; tests pin
the alignment, the census (60/3/2), pairwise name uniqueness, and the
merge-critical rows.

The class rule is mechanical, so the regeneration is too:

- **soft** = the workflow file contains `continue-on-error`
  (`grep -l 'continue-on-error' .github/workflows/*.yml`)
- **reporting** = the workflow only merges / opens / files work — today
  `auto-merge-ready-prs` and `queen-publish`, the same two files
  `scripts/ci/check_pr_branch_filters.py` records as NOT_MERGE_CRITICAL for
  exactly that reason
- **blocking** = everything else

Events extraction (run from the repository root):

```sh
cd .github/workflows && for f in *.yml; do
  n="${f%.yml}"
  ev=$(awk '/^on:/{p=1;next} p&&/^[A-Za-z_]+:/{p=0}
            p&&/^  [a-z_]+:/{gsub(/:.*/,"");gsub(/^  /,"");printf "%s,",$0}' "$f" | sed 's/,$//')
  printf '%s|%s\n' "$n" "${ev:-NONE}"
done
```

A workflow file that lands in `.github/workflows/` without its row in the
inventory (or vice versa) is an inconsistency by construction: `tri claw
status` (T2) lowers the spec with `t27c` and diffs `NAMES` against the
directory, fail-closed on any unknown name — mirroring the ceiling discipline
of `scripts/ci/check_pr_branch_filters.py`.

## Why this shape (own language first)

The gate logic lives in `.t27` because that is this project's language and the
whole claim of the service: gates as a **tested spec**, not a vendor blacklist
or a foreign DSL. Everything else — the GitHub App wiring, the Railway
service, the CLI plumbing — is thin I/O glue around the spec. See the
`own-language-first` rule in the repository instructions.

S3AI is written "S3AI" here (ASCII, L3) — the wordmark with the superscript
lives where superscripts are safe.
