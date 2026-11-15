# GitLab mirrors of the public repositories (read-only)

Status: one-time snapshot run on 2026-11-15 on the lab VPS. The runner that
ran the smoke pipelines is described in [gitlab_runner.md](gitlab_runner.md).

GitHub stays the merge authority (see #13907). Each mirror is a private GitLab
project `root/<name>` whose default branch was fetched once from the public
GitHub URL with no credential (`fetch_as_mirror` with a single-branch refmap,
run through `gitlab-rails runner`). One repository at a time, smallest first.

Stop thresholds between repositories: host available memory under 2000 MB, or
the GitLab container over 3.0 GiB. Neither was reached (lowest available
4169 MB, highest container 2.17 GiB). No runner, container or host limit was
changed; no other service on the host was touched.

## Repositories considered

| Repository | State | Branch | Commit | Fetch time | Smoke pipeline | Smoke result | Duration |
|---|---|---|---|---|---|---|---|
| agent-mail-mcp | mirrored | master | 35c0ad68806877dc2682d2b4bb4b2a180a2f5713 | 1 s | 6 | success | 4 s |
| t27-mcp | mirrored | master | 335e7e517297df2d48910ffca12c1cc5c6086efc | 1 s | 7 | success | 4 s |
| t27os | mirrored | master | ed54560c5b1b4ccdf9557fe0fca4206479d0b633 | 2 s | 8 | success | 5 s |
| trios-trainer-igla | mirrored | main | 61fbdd448b570c1b6da923171bf2d9e7afce131a | 2 s | 9 | success | 4 s |
| trios-railway | mirrored | main | d8f1d1dc7cfdc779162dac656c8612fc18207a10 | 4 s | 10 | skipped (no job ran) | - |
| leela | mirrored | main | fa9eef46eb2e909ceda3ddcd551d4aa516df1ee7 | 19 s | 11 | success | 5 s |
| turbobaby-user-bot | mirrored | main | a717e380859437d08a06bfae69883b2cd4a69581 | 32 s | 12 | success | 5 s |
| ghashtag.github.io | mirrored | main | 401328a15bbf14afbfd851123dac64c304d8d473 | 45 s | 13 | success | 17 s |
| trinity-fpga | mirrored | main | f8499ed8e6f591f269b82304d0257d72e15f80b9 | 47 s | 14 | success | 46 s |
| trinity | mirrored | main | 2f2546e8b937b1a75556284ce5ca4b46c85db96a | 46 s | 15 | success | 30 s |
| trios | mirrored | main | 78671ba794826513c0de3de816e9d5e1c3c12349 | 33 s | 16 | success | 20 s |
| t27 | mirrored earlier (see [gitlab_runner.md](gitlab_runner.md), #13987) | master | - | - | - | - | - |
| private repositories | private, not mirrored | - | - | - | - | - | - |

Note: trios-railway pipeline 10 was skipped by GitLab because the head commit
message carries the `[skip ci]` marker (an hourly snapshot commit). The branch
itself is fetched at the listed commit.

Private repositories are not named here. They stay unmirrored until the owner
enters a read-only credential in GitLab himself.

Totals: 11 fetched, 0 failed, 10 smoke jobs succeeded, 1 pipeline skipped.

## Smoke pipeline

The CI config lives outside every mirrored tree: file `smoke.yml` on branch
`main` of project `root/t27`. Each mirror references it through the project
setting `ci_config_path = smoke.yml@root/t27:main`. No pipeline file was added
to any repository.

The job sets `GIT_DEPTH: "1"` and prints the commit sha, the top-level entry
count and the tree size. It proves the runner can fetch the mirror, nothing
more.

Pipelines were created with `Ci::CreatePipelineService`, because a mirror fetch
raises no push event. Runner: `vps-docker-1g`, one job at a time, so the 11
pipelines ran in sequence.

## Host before and after

| Measure | Before | After |
|---|---|---|
| Available memory | 3799 MB | 5503 MB |
| Running containers | 6 | 6 |
| GitLab container memory | not recorded before the run (2.16 GiB at the first repository) | 2.009 GiB of 3.418 GiB |
| Disk used by `/opt/gitlab` data | not recorded | disk figure printed by the run: 22G |

## Generated pipeline (#13941, #13995)

The pipeline file is rendered from `specs/ci/gitlab_lane.t27` by
`t27c gen-text specs/ci/gitlab_lane.t27 --fn gl_pipeline_yaml`. The first render
was 4062 bytes, md5 55b3dc756ee61dcf51c1de64c889f8dd. It is stored as
`pipeline.yml` on GitLab `root/t27`, branch `main`. It is not in the GitHub
tree: the own-language gate refuses a root `.yml` file.

| Mirror | Head | Result | Pipeline |
|---|---|---|---|
| t27-mcp | 335e7e5172, unchanged, not fetched | success, 12 of 12 jobs, 55 s | 17 |
| t27os | ed54560c5b to 4535f577eb, fetched in 2 s | success, 12 of 12 jobs, 54 s | 19 |

The runner record got the tag `vps-docker-1g` and `run_untagged` true. No limit
was changed. The jobs were print-only in those two runs.

#13995 gives two jobs a real command; every other job stays print-only:

- `prefetch` fails when `.git/HEAD` differs from `$CI_COMMIT_SHA`. The job image
  is plain alpine (busybox, no git binary) and the runner checks out a detached
  HEAD, so that file holds the sha.
- `guard` lists the `.t27` files, counts the bytes above 0x7F in each with
  `tr -d` and `wc -c`, prints `non-ascii: <file>` for each hit and fails at the
  end if there was one.

The render with these commands is 4354 bytes, fingerprint 900205 (the pinned
tests in the spec). Its md5 and its delivery to GitLab, and the recorded
pipelines on two mirrors and the planted non-ASCII run, are to be recorded
after delivery.

## Open items

1. The mirrors are a one-time snapshot. No scheduled update exists. A future
   update run should use one rails process for the whole list and fetch only
   branches whose GitHub head changed.
2. Real project checks come from the generated pipeline file, #13941.
3. Private repositories are pending the owner-entered read-only credential.
4. trios-railway has no smoke result because of the skip marker.
