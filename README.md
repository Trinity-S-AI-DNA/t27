# T27 OS

A minimum viable operating surface written in [t27](https://github.com/gHashTag/t27).
Its home screen is the 27-petal 999 triangle: three realms of nine petals each. Pressing a petal opens that
petal's menu, a grid of cards for existing widgets. The same spec drives the trios macOS app today
and an FPGA board (AX7203) at the end.

- **Epic:** [gHashTag/t27#10782](https://github.com/gHashTag/t27/issues/10782). The milestones live there, not here.
- **The menu spec:** `specs/t27f/petal_menu.t27` in gHashTag/t27 (module `T27fPetalMenu`).
  It covers geometry, realms, fills, fit, hit test, open/back state and the card grid.
- **This repository:** `specs/os/home.t27` names what the OS home is made of and points at the
  specs that own each part. It does not copy them.

## Layers of the home screen

| Layer | What it fixes | Owner |
|---|---|---|
| L1 geometry | 27 quads in milli-units of a 596x526 box | `petal_menu.t27` |
| L2 meaning | RAZUM 0-8 outer, MATERIYA 9-17 middle, DUKH 18-26 inner | `petal_menu.t27` |
| L3 look | idle black 0.22, hover white, open = realm colour | `petal_menu.t27` |
| L4 fit | 0.63 x min(w/596, h/526), centred | `petal_menu.t27` |
| L5 input | integer ray-cast hit test, no division | `petal_menu.t27` |
| L6 state | home, petal open, back | `petal_menu.t27` |
| L7 menu | 3x3 card grid, empty in the MVP | `petal_menu.t27` + widgets gallery |

## Rules

Everything here is t27 or prose. Hand-written code in another language is not accepted. CI gates
come from TRI CLAW (gHashTag/t27#9271), not from hand-written workflow files. See `CLAUDE.md`.

## Where the specs live (SSOT rule, 2026-10-23)

The operating specs of T27 OS -- kernel and agent cap, evolve, domain pulse, devices, panel surfaces -- live in gHashTag/t27 under `specs/t27os/` and are gated there by the compiler, seals and CI. This repo holds the OS shell that mounts them and the build recipe for the three surfaces (trios app, site, FPGA). The epic and its milestones live in gHashTag/t27#10782.

A change to an OS rule belongs in t27; a change to a surface belongs here. Sync check: petal count, the tool-letter registry and the kernel agent cap must match between the two trees -- a mismatch is a defect, not a fork.

## Published numbers

Every number this repository publishes about T27 OS carries the five-line footer that
`specs/t27os/disclosure.t27` in gHashTag/t27 defines (reader, judge, n, cost, repro; t27os#20).
A number without its footer is not published here.

The first lab receipt, `runs/8f28c2d849.048f7f45.json` (2026-10-23):

```
specs/t27f/work_graph.t27|tests=9|pass=9|FAIL=0|vacuous=0|blocked=0
specs/queen/actors.t27|tests=37|pass=37|FAIL=0|vacuous=0|blocked=0
specs/t27f/export_video.t27|tests=6|pass=6|FAIL=0|vacuous=0|blocked=0
reader: t27c 0.5.2 built from t27 8f28c2d849837d5deb43b7d2b37f6180fc5c8640 (the binary reports "build 0 commit unknown"); specs t27f/work_graph, queen/actors, t27f/export_video
judge: t27c test-report; vacuous and blocked are counted, never passed (t27os#13)
n: n=1 run + 1 requester reproduction; no confidence interval
cost: 108 s on one GitHub runner (lab run 39944645613, job 11:14:06Z to 11:15:54Z); five failed dispatches before it, 10:53Z to 11:08Z, cost about 5 runner-minutes more
repro: gh workflow run lab.yml --repo Trinity-S-AI-DNA/t27os -f sha=8f28c2d849837d5deb43b7d2b37f6180fc5c8640 -f challenge=<fresh hex> -f specs="specs/t27f/work_graph.t27 specs/queen/actors.t27 specs/t27f/export_video.t27"; the original challenge is bound by challenge_sha256 048f7f45151ce7b29b6109ab66bff18d325c49a7d8ae451412891b370892f561
```
