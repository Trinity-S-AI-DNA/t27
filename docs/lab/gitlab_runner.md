# GitLab runner on the lab VPS

Status: installed 2026-11-15 after the owner approval recorded as a comment on
issue #13979. `gitlab-runner verify` reported the runner valid. A trial
pipeline ran on 2026-11-15 13:31 UTC and its job succeeded (see "Trial
pipeline" below). The trial project is empty, so a large clone is not proven.

## Step 1: precondition

An owner approval comment on issue #13979 must exist before anything is
installed. Without it, install nothing.

## Step 2: memory check before install

Run `free -m` on the host. Refuse the install when less than 1536 MB is
available.

Measured before install (2026-11-15): 3320 MB available of 7941 MB total.

## Step 3: limits

The whole runner setup is capped at 1 GiB:

- runner container: 256 MB (`--memory 256m --memory-swap 256m --cpus 0.5`)
- one job container: 768 MB (`memory = "768m"`, `memory_swap = "768m"`,
  `cpus = "1"`)
- `concurrent = 1`, so one job runs at a time

## Step 4: layout

- container name: `gitlab-runner`
- image: `gitlab/gitlab-runner:alpine`
- docker network: `gitlab-net` (the GitLab container is on it)
- restart policy: `unless-stopped`
- config directory: `/opt/gitlab-runner/config`, mounted at `/etc/gitlab-runner`
- docker socket: mounted into the runner container
- executor: `docker`
- default job image: `alpine:3.20`
- job network: `network_mode = "gitlab-net"`
- `url` and `clone_url`: both `http://gitlab:8929`

GitLab listens on localhost:8929 on the host only. The runner reaches it by
container name over `gitlab-net`.

## Step 5: registration

The runner record (instance type, description `vps-docker-1g`) is created
inside the GitLab container with `gitlab-rails runner`. The returned token is
captured into a shell variable and written straight into `config.toml`, which
is kept at mode 600.

The token is never printed, logged or committed. This document names only the
variable: `token` in `config.toml`.

## Step 6: verify

- `docker exec gitlab-runner gitlab-runner verify` must say "is valid".
- `docker stats --no-stream gitlab-runner` must show usage under 256 MiB.
  Measured: 89 MiB at install; 92 MiB after the trial job.
- Every other container on the host must still be up.

## Trial pipeline

Result of the trial run on 2026-11-15:

- time: 13:31 UTC
- job `trial` ran on runner `vps-docker-1g` and succeeded in 4 s
- the clone over `http://gitlab:8929` (container network) worked
- inside the job, `cat /sys/fs/cgroup/memory.max` printed `805306368`
  (768 MiB), so the job container limit is in force
- after the job, the runner container used 92 MiB of its 256 MiB limit
- the host had 3793 MB available

How to repeat it: create a project in GitLab with a one-job CI config
(`.gitlab-ci.yml`) whose `trial` job runs the script `echo runner-ok` and
then `cat /sys/fs/cgroup/memory.max`. Push the file, then check that the job
passes on the `vps-docker-1g` runner and that the printed limit is
805306368.

## Do not touch

- any other unit, container or file on the host
- GitLab itself: it is not restarted or reconfigured by this install

## Rollback

1. `docker rm -f gitlab-runner`
2. Delete the runner record in GitLab (Admin, Runners).
3. Remove `/opt/gitlab-runner/config/config.toml`.

GitLab stays unchanged throughout.

## Open items

- The trial project is empty, so a large clone is not proven.
- No job that reaches the 768 MiB limit is still to be run.
- The GitLab container uses about 2.5 GiB of its 3.4 GiB limit. Re-measure
  memory before raising any runner or job limit.
