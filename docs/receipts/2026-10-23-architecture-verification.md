# Architecture verification receipt -- independent agent run

Date: 2026-10-23 (UTC 2026-10-23T10:00Z). Subject: `docs/ARCHITECTURE.md` (assembled 2026-10-23).
Method: every claim re-run from a clean worktree of gHashTag/t27 (master + one spec,
HEAD `b6ee2f11e`, clean tree) and from live GitHub/lab state. Commands, exit codes and
observations recorded verbatim. The verifier did not edit ARCHITECTURE.md.

## Verdict table

| # | Claim (from ARCHITECTURE.md) | Command | Exit | Observed | Verdict |
|---|---|---|---|---|---|
| 1 | actors.t27: 37/37 tests pass, 0 vacuous | `t27c test-report specs/queen/actors.t27` (in t27 worktree) | 0 | `tests 37 / pass 37 / FAIL 0`; `vacuous passes 0 of 37` | CONFIRMED |
| 2 | hive.t27: 5/5 tests pass | `t27c test-report specs/queen/hive.t27` | 0 | `tests 5 / pass 5 / FAIL 0`; `vacuous passes 0 of 5` | CONFIRMED |
| 3 | dispatch.t27: 13/13 tests pass | `t27c test-report specs/queen/dispatch.t27` | 0 | `tests 13 / pass 13 / FAIL 0`; `vacuous passes 0 of 13` | CONFIRMED |
| 4 | "55 tests green across actors + hive + dispatch" | sum of rows 1-3 | 0 | 37 + 5 + 13 = 55 | CONFIRMED |
| 5 | Gap #1: lesson_reel.t27 landed BLOCKED with `module_of` undeclared | `t27c test-report specs/t27f/lesson_reel.t27` | 2 | `BLOCKED  does not compile: spec.zig:2002:14: error: use of undeclared identifier 'module_of'` | CONFIRMED |
| 6 | Gap #2: branches `queen` and `queen-runners` "neither is on the remote today" | `gh api "repos/Trinity-S-AI-DNA/trios/branches?per_page=100&page=N"` for N=1..6 (exhausted: page 6 returned 47) | 0 | 547 unique branches; `queen` -> `929307c1c` and `queen-runners` -> `b8c0e8318` BOTH EXIST (verified via `gh api .../branches/<name>`) | REFUTED |
| 7 | Gap #2 (second half): `dev`, `bee/queen-doctor-autoloop`, `actors-next` exist | same paged listing + per-branch lookup | 0 | `dev` -> `67174491f`, `bee/queen-doctor-autoloop` -> `d33dc72c8`, `actors-next` -> `952fb6cc7`, also `actors-next-2`, `ci/queen-workflows` | CONFIRMED |
| 8 | t27os home spec parses; repo master at `facb844` | `t27c parse specs/os/home.t27` (in /Users/playra/t27os); `git log --oneline` | 0 | parse exit 0 (full AST printed); HEAD = `facb844 init: T27 OS -- home spec...`, 1 commit at verification time; this receipt is commit #2 | CONFIRMED |
| 9 | PR #11753 (M6 / export_video) is an open PR | `gh pr view 11753 -R gHashTag/t27 --json state,headRefName` | 0 | `{"headRefName":"lane/t27f-export-video-10265","state":"OPEN"}` | CONFIRMED |
| 10 | PR #11754 (lesson-reel fix) is an open PR | `gh pr view 11754 -R gHashTag/t27 --json state,headRefName` | 0 | `{"headRefName":"lane/t27f-lesson-reel-factory-11678","state":"OPEN"}` | CONFIRMED |
| 11 | "the lab answers HTTP 200 since 2026-10-23" | `curl -s -o /dev/null -w '%{http_code}' --max-time 10 https://t27b-lab-production.up.railway.app/` | 0 | `200` | CONFIRMED |

## Refuted claim: exact difference from ARCHITECTURE.md

ARCHITECTURE.md gap #2 states: "AGENTS.md points at branches that no longer exist:
'Queen server on branches `queen` and `queen-runners`' in Trinity-S-AI-DNA/trios --
**neither is on the remote today**."

Observed: both branches exist on the remote today, with live heads
`queen` -> `929307c1c` and `queen-runners` -> `b8c0e8318`. The trios repo has 547
branches (6 pages via the API). The likely source of the false absence: a single
`branches?per_page=100` query returns only the first 100 names alphabetically
(`actors-next` .. `feat/laws-v2`), and both `queen*` names sort after that page.
Any absence claim about a large remote must paginate to exhaustion before it is
evidence.

What survives of gap #2: the doc-repointing work itself may still be worth doing
(AGENTS.md's branch pointer may be stale in other ways), and `dev` + `bee/*` +
`actors-next` do exist as the map says. But the load-bearing sentence "neither is
on the remote today" is false and must not be repeated.

## Ancillary checks

- Worktree HEAD `b6ee2f11e` ("spec(t27f): export_video ... (Closes #10265) [sealed]")
  sits directly on top of `4e96f3a43` ("spec(t27f): hud foundation for M3 (#10262)
  (#11752)"), and `git merge-base --is-ancestor 4e96f3a43 HEAD` exits 0: the map's
  citation of t27 master `4e96f3a43` is a real ancestor of the tree the law specs
  were tested in. Consistent with the map.
- t27os commit count: 1 commit (`facb844`) at verification time; this receipt is the
  second, matching the task's "exactly 2 commits after your work".

## Total

10 of 11 lines CONFIRMED, 1 REFUTED (gap #2's branch-absence claim). UTC: 2026-10-23T10:00Z.

phi^2 + 1/phi^2 = 3 | TRINITY
