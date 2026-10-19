| Move gate: whether a Railway service can move to another project, checked before anything is created there (private hosts, doubled outbound loops, state volumes, a target with no plan) | `specs/claw/move_gate.t27` (move kind and five readings in, one `V` line out: go or hold; not wired to a job yet) | #10794 |
# TRI CLAW

TRI CLAW — TRI-27 — Trinity S3AI: every CI gate of this repository, packaged
behind one front door, with the gate logic in `.t27` specs. Epic
[gHashTag/t27#9271](https://github.com/gHashTag/t27/issues/9271).

## What exists today

| Piece | Where | Status |
|---|---|---|
| Gate inventory (SSOT) | `specs/claw/gates.t27` | T1, merged |
| Drift audit law | `claw_status` in `specs/claw/gates.t27` | T2, this branch |
| Drift audit in CI | `gate-topology` workflow, "claw drift" step | T3, this branch |
| GitHub App manifest | release step, see `docs/claw/RELEASE.md` | T4, deferred to App setup |
| T27F check in every `.t27` repository | `specs/claw/t27f_gate.t27`, the `claw-t27f` job of `gate-topology.yml` | #10268, live once the owner steps below are done |
| SARIF code-scanning alerts on a pull request | `specs/claw/sarif_feed.t27` over `specs/claw/sarif.t27`, the `claw-sarif` job of `gate-topology.yml` | #10356, live once the owner steps below are done |
| Suggestion review on a pull request | `specs/claw/suggest_feed.t27` over `specs/claw/suggest.t27` and `specs/tri/fmt_source.t27`, the `claw-suggest` job of `gate-topology.yml` | #10365, live once the owner steps below are done |
| Service skeleton | Railway, thin I/O glue only | T5, merged (spec contract) |
| Queue cost: which workflow holds the runners, and what the queued runs wait for | `specs/claw/queue_cost.t27` (job rows and queued-run rows in, two summary lines out; not wired to a job yet) | #10600 |
| Push gate: whether a lane push helps the PR or only restarts its required checks | `specs/claw/push_gate.t27` (push kind and three flags in, one `V` line out: push, hold or skip; not wired to a job yet) | #10606 |
| Check state: whether a PR's checks are red, pending, none or green, with a queued check read as pending and a commit status read by its state | `specs/claw/check_state.t27` (rollup rows in, a count line and one `V` line out; keeps the PR Dashboard's current per-PR rule as a named function; not wired to a job yet) | #10617 |
| Check latest: a re-run supersedes the run it replaces, so an old failure next to its queued re-run reads pending, not red | `specs/claw/check_latest.t27` (rollup rows with name and run id in; per name the largest run id is live, classed by check_state; three lines out; not wired to a job yet) | #10617 |
| Required red: push_gate's req_red flag from the rollup, newest run per required check, so a red check that is not required never triggers a fix push | `specs/claw/required_red.t27` (check_latest rows in; an R count line and an `F req_red true/false/unknown` line out; not wired to a job yet) | #10606 |
| Merge lane: is the merger starved, and does it merge anything? Counts its runs, the oldest open age, the PRs each run merged, and its share of the whole queue, so an hour-long wait, a zero yield or a tenth of the queue shows up as data | `specs/claw/merge_lane.t27` (rows per merger run or per queued run; `F merger`, `F merger-yield` and `F merger-share low/high/unknown/none` out) | #10644 |
| Shape race: the issue-shape gate judges the event payload, which can be twenty minutes old; replays a gate timeline under the payload reading and a live reading, so an issue closed with a passing body shows up as data | `specs/claw/shape_race.t27` (rows `created TAB ran TAB ok` per gate event; an `R` line and `F shape-race clean/race/unknown/none` out; #10644 is the fixture; the workflow change is the owner's decision) | #10662 |
| Lab receipt: whether a hand-run lab check of a patch really ran on the patch. A `git apply` that changed fewer files than the diff, a blank or BLOCKED test report, zero tests, a failure or a vacuous pass all hold | `specs/claw/lab_receipt.t27` (expected and applied file counts and the t27c `test-report` text in; one `L pass/hold` line with the reason out; not wired to a job yet) | #10810 |
| Lab disk: whether a lab volume has room for a check, read from `df -Pk` before it runs. Under 1 GiB free holds; under 5 GiB or under 10% free warns; a mount df does not list, or a line with no readable size, holds | `specs/claw/lab_disk.t27` (`df -Pk` text and a mount in; one `D go/warn/hold` line with the reason out; not wired to a job yet) | #10813 |
| Red origin: whose red a failing check is. A file the log names that the PR changes, or a green master, makes it the PR's; a red master, or a named file master changed since the PR's base, makes it inherited, and the PR gets no push. Ask required_red first | `specs/claw/red_origin.t27` (the PR's paths, the log's named paths, master's moved paths and master's state in; one `O own/inherited/unknown` line with the reason out; not wired to a job yet) | #10821 |
| Merge judge: which open pull requests TRI CLAW merges, and why it leaves the rest alone. First match wins: the no-auto-merge label, a draft, a base that is not master, a `queen-` bee branch (the reviewer-bee merger's), a conflict, no closing reference, an unread rollup, a pending check, a required check red or missing, any other red that is not inherited from master (claw-sarif is, by definition), a head commit under ten minutes old; then a squash merge pinned to the judged head sha, five per run at most, lowest number first | `specs/claw/merge_judge.t27` (`P`/`K`/`F`/`O`/`B` lines in; `G` merge, `W`/`H`/`N` skip with the reason, `L` failing log wanted and `M` summary out; lowered to `gen/c/claw/merge_judge.c`; run by the `claw-merge` job of `gate-topology.yml`, which also carries `refresh.t27`'s update-branch step; plan-only into the step summary until the repository variable `CLAW_MERGE_LIVE` is `1`) | #10973 |
| Known survivors: whether a mutation sweep holds anything new. A survivor is keyed by its spec, operator and line text, not its line number, so an argued equivalent stays known when lines above it move; a new survivor, a NOAPPLY line or an unreadable line holds; a known entry no line matches asks to be pruned | `specs/claw/mut_known.t27` (the sweep text and the known list in; one `E pass/hold` line with the reason, and one `E new/harness/unreadable` line per holding sweep line, out; applied by hand, not wired to the sweep script yet) | #10835 |
| Model routing: Opus 5.5 orchestrates, Sonnet 5.5 runs sub-tasks | `specs/claw/model_route.t27`, skill `model-routing`, `.claude/settings.json` + `.claude/agents/*` | owner directive 2026-10-18 |
| A T27F visual in every answer that needs one | rule `specs/t27mcp/visual_output.t27`, particle-still layer `specs/claw/visual_output.t27`, procedure in skill `t27f`, "Visual in every output" | owner directive 2026-10-18 |
| `reel_still`: one particle still (PNG at STILL_MS of a token_floor reel) from TRI CLAW's `/mcp` | `specs/claw/reel_still.t27`, rendered on the Railway lab only | follow-up of the row above |
| Positioning / pricing | `docs/claw/competitors.md` | T6, started |

## The T27F check in every `.t27` repository (#10268)

On each run (the cron is `*/10`, but GitHub runs it best effort, see "How often it runs" below) the `claw-t27f` job
of `gate-topology.yml` in gHashTag/t27 lists the repositories the TRI CLAW App
is installed on, and for each open pull request that touches `.t27` posts one `claw/t27f`
check-run on its head commit: the changed specs and their direct importers, each judged by
`t27c seal --verify` from the signed t27o-v0.1.0 release. The law is `specs/claw/t27f_gate.t27`:
unsealed is neutral; a stale, vacuous or unreadable seal is failure; gHashTag/ghashtag.github.io
only reports. No consumer repository carries a workflow file. Until the owner does these four
steps, it posts nothing:

1. Give the App the permission checks: write at https://github.com/settings/apps/tri-claw.
2. Accept the new permission on the App's installation for gHashTag.
3. Set the secret `CLAW_APP_PRIVATE_KEY` in gHashTag/t27 to the App's private key.
4. Set the variable `CLAW_APP_ID` in gHashTag/t27 to `5166765`.

## SARIF code-scanning alerts on a pull request (#10356)

On every pull request in gHashTag/t27 the `claw-sarif` job of `gate-topology.yml` runs the signed t27o-v0.1.0
t27c (`spec-status` and `parse`) over each changed `.t27` file and uploads one SARIF run
(category `claw/sarif`) with the token of the `code-scanning-sarif` row of `specs/claw/github_surface.t27`:
`security_events` write and `contents` read, nothing else. A spec that does not parse is rule T27C001 (error,
at the line:col t27c names); a spec with unwritten functions is T27C004 (warning). The adapter is
`specs/claw/sarif_feed.t27` (lowered to `gen/c/claw/sarif_feed.c`); the document is `specs/claw/sarif.t27`.
Rules T27C002 and T27C003 need the t27c test runner, which needs zig, and are not fed yet. It needs the secret and
variable of steps 3 and 4 above, the App permission `security_events: write`, and code scanning enabled on the
repository; without them the job uploads nothing and says so in its step summary.

## Suggestion reviews on a pull request (#10365)

On every pull request in gHashTag/t27 the `claw-suggest` job of `gate-topology.yml` reads each changed `.t27` file
from the head commit (`git show`, so a symlink is never followed) and gives it, with the `@@` headers of the
GitHub diff, to `specs/claw/suggest_feed.t27` (lowered to `gen/c/claw/suggest_feed.c`). The driver formats the file with
`specs/tri/fmt_source.t27` -- the formatter behind `t27c fmt`, which only normalises whitespace (leading tab to four
spaces, no trailing space or CR, one final newline) and refuses to print anything if a single non-whitespace byte
differs -- compares the two texts line by line, and hands the changed ranges to `specs/claw/suggest.t27`, which
decides placement: event COMMENT only, right side only, lines outside the diff skipped, at most 20 comments, and a
hunk it cannot place poisons its whole file. The job posts the review it prints as one pull request review with the
token of the `pr-suggestions` row of `specs/claw/github_surface.t27`: `pull_requests` write and `contents` read.
An already formatted spec posts nothing. It needs the secret and variable of steps 3 and 4 above and the App
permission `pull_requests: write`; without them the job posts nothing and says so in its step summary.

## The merge lane (#10973)

The `claw-merge` job of `gate-topology.yml` runs on every push to master, on the best-effort `*/10` schedule and on
dispatch (cadence: see "How often it runs" below). It lists the open pull requests once, feeds `gen/c/claw/merge_judge.c` (`specs/claw/merge_judge.t27`),
and applies what the spec prints: `G` lines merge (squash, pinned to the judged head sha, so GitHub refuses it if
the head moved), every other line is a reason in the step summary. After a push it also runs `refresh.t27` as the
former `claw-refresh` job did, skipping a pull request it merged in the same run. Strictness is required + inherited:
the four required checks (validate, check-linked-issue, parse-ratchet, own-language) green, and every other red
check inherited from master by `red_origin.t27` (claw-sarif is, by definition, until #10501 is done).

- The job is plan-only until the repository variable `CLAW_MERGE_LIVE` is `1`: it writes what it would merge to the
  step summary and merges nothing. Set it after reading a dry run; unset it to stop the lane.
- The merge uses a second TRI CLAW token with `contents` and `pull_requests` write (the `merge-pr` row of
  `github_surface.t27`), so master's push workflows start after a merge; without the App secrets it uses
  `github.token`, whose merges start no workflow. The update-branch token is unchanged (`pull_requests` write).
- Bee pull requests (`queen-` branches) are never merged here: `auto-merge-ready-prs.yml` merges them, after the
  reviewer bee. Drafts and pull requests labelled `no-auto-merge` are held; a conflict is listed, never resolved.
- Honest limits are in the header of the spec: the lane reads whether a failing log names a file the PR changes, not
  the failure set; reviews are not read; two pull requests merged in one run were each green alone.

### How often it runs

The cron in `gate-topology.yml` is `*/10 * * * *`, and GitHub runs scheduled workflows best effort. Measured on
master (Actions API, `event=schedule`, #11132): the last 8 scheduled runs, 2026-10-18 12:11Z to 2026-10-19 13:50Z,
were 2.1 to 7.5 hours apart. The dependable triggers are a push to master (any merge starts the lane) and
`workflow_dispatch`. A repository that merges nothing for hours can leave this lane idle for hours; a faster
heartbeat would be an external dispatch and is a separate decision.

## The inventory spec and how to regenerate it

`specs/claw/gates.t27` holds three parallel `[67]str`-shaped arrays — `NAMES`
(workflow file stems), `CLASSES` (0 blocking / 1 soft / 2 reporting), `EVENTS`
(trigger keys, comma-joined). Rows stay alphabetical and aligned; tests pin
the alignment, the census (62/3/2), pairwise name uniqueness, and the
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
inventory (or vice versa) is an inconsistency by construction: `claw_status`
in the spec is lowered by `t27c gen-c` into `gen/c/claw/gates.c`, and the
claw-drift step of the `gate-topology` workflow compiles a four-line harness
around it that diffs `NAMES` against the directory and fails on any unknown
name — mirroring the ceiling discipline of
`scripts/ci/check_pr_branch_filters.py`. The law is the spec; the harness is
machinery (owner rule 2026-10-13: everything new is .t27).

## Why this shape (own language first)

The gate logic lives in `.t27` because that is this project's language and the
whole claim of the service: gates as a **tested spec**, not a vendor blacklist
or a foreign DSL. Everything else — the GitHub App wiring, the Railway
service, the CLI plumbing — is thin I/O glue around the spec. See the
`own-language-first` rule in the repository instructions.

S3AI is written "S3AI" here (ASCII, L3) — the wordmark with the superscript
lives where superscripts are safe.
