# Analog reference chain: PSP103 model hashes at the IHP-Open-PDK pin, dev and main

Measured 2026-10-24 for issue #12027 (requirement 3), epic #10486. Read-only: the GitHub tree API, no
clone, no build, no simulator run. Requirements 1 and 2 of #12027 (OpenVAF commit and OSDI version in
each receipt, one ngspice-47 load test) are not covered here; they need a T27 OS pane.

## What was compared

| Ref | Commit | Commit date | Note |
|-----|--------|-------------|------|
| pin | `21133f219b1a6da10d803e345761e77415ceff40` | 2026-09-08 | the commit every analog receipt names |
| dev | `cb743f75` (head on 2026-10-24) | 2026-10-23 | the moving branch |
| main | `5e6d592e` (head on 2026-10-24) | 2026-09-01 | GitHub compare main...pin: diverged, the pin is 410 commits ahead and 2 behind |

Repository: IHP-GmbH/IHP-Open-PDK. The pin is not a descendant of main: main carries two
commits the pin does not have.

## Command

Run once per ref, sort, then diff the lists (replace REF by the commit or branch name):

    gh api "repos/IHP-GmbH/IHP-Open-PDK/git/trees/REF?recursive=1" --jq '.tree[] | select(.path | test("^ihp-sg13g2/libs.tech/verilog-a/psp103/")) | "\(.path) \(.sha)"' | sort

The corner library uses the same call with the path filter `sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib$`.

## Result

Directory `ihp-sg13g2/libs.tech/verilog-a/psp103/`, 17 files. The hash is the git blob hash. Dev equals the
pin on all 17 files. Main equals the pin on 15 files and differs on 2.

| File | Blob hash at pin (= dev) | At main |
|------|--------------------------|---------|
| `Changelog` | `fc97d792701070796b9339b58890d4c9c82d0733` | `b0c7dc05018b810e3772370f3cc9edc3cc90e771` (differs) |
| `Common103_macrodefs.include` | `a88dbccddd1f901eee474ec54020bd1dcb1d0101` | same |
| `JUNCAP200_InitModel.include` | `46d39877a3997aa0699931806f174f45c384777e` | same |
| `JUNCAP200_macrodefs.include` | `0723a00e1e804adc2f783c18be38e980fa6315b7` | same |
| `JUNCAP200_parlist.include` | `bf0006d67c6d3aa35db7882d3e49431e81c34cff` | same |
| `JUNCAP200_varlist.include` | `5733d9e03324362c203c84e212871234b216b46e` | same |
| `PSP103_macrodefs.include` | `da0e4d94ff78e407b772f84c0e069149a0cd7aba` | same |
| `PSP103_module.include` | `2d839ee791815f4c9e85774ef435c02a6466f3f1` | `39bcc1b7d076b3f6c84a4bdb6fb1e25517e2e08d` (differs) |
| `PSP103_nqs_macrodefs.include` | `03c369ab1e531339a3409cf1b49f832ae11fa27d` | same |
| `PSP103_parlist.include` | `be936530b47feae1b31b2306d2bf65a928173e95` | same |
| `PSP103_scaling.include` | `924080665756131e12fddd363a8a621b731146a7` | same |
| `README.md` | `1fc48f774ddbdd17bcb95515f6254a2e20d83220` | same |
| `juncap200.va` | `7d8a7a3412f79f49c614dfba04c4b141ed18b91a` | same |
| `psp103.va` | `f069f3621c0d98ce64b8b181daa14ef0d1087bbb` | same |
| `psp103_nqs.va` | `bf90def1ac48e08a783c355cab8570d656f78d73` | same |
| `psp103t.va` | `1a618e62c08a1e5de0461fe5dfa63351866bf98b` | same |
| `releasenotesPSP103.8.2.txt` | `99c90afce48ba18939d3721cc1667cd91fe64d88` | same |

`ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib`: blob `f86c3efa204a27a4bcadd3cf3fcd67da8793b293` at the
pin, at dev and at main.

## Reading

- The pin is safe against the moving dev branch for the PSP103 model files and for `cornerMOSlv.lib` as of
  2026-10-24. A later dev commit that touches either file shows up as a changed hash when the command is
  rerun.
- Checking out `main` instead of the pin gives a different `PSP103_module.include` (and a different
  `Changelog`). Receipts therefore keep naming the commit, never a branch.
- Not measured: whether the two main-only commits change any number in the analog tables. Nobody uses main
  today, so this stays a question until someone does; it then gets its own issue.

## Rerun rule

A difference between the pin and dev on any of the 18 files above is a new issue, not a silent edit of this
page or of a table.
