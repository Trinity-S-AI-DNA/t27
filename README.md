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
