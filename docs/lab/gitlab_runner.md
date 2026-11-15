# GitLab runner on the lab VPS

Status: installed 2026-11-15 after the owner approval recorded as a comment on
issue #13979. `gitlab-runner verify` reported the runner valid. No job has been
run yet; the first real pipeline run is still open.

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
  Measured: 89 MiB.
- Every other container on the host must still be up.

## Do not touch

- any other unit, container or file on the host
- GitLab itself: it is not restarted or reconfigured by this install

## Rollback

1. `docker rm -f gitlab-runner`
2. Delete the runner record in GitLab (Admin, Runners).
3. Remove `/opt/gitlab-runner/config/config.toml`.

GitLab stays unchanged throughout.

## Open items

- The first real pipeline run has not been done.
- The GitLab container uses about 2.5 GiB of its 3.4 GiB limit. Re-measure
  memory before raising any runner or job limit.
