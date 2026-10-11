# NOW -- golden chain against the GitHub loop, on the last 80 merged PRs (2026-10-11)

## specs/queen/replay/golden_chain_history.t27 (Closes #9088; actors rule see #7851; golden chain see #9027)

- Window: the 80 PRs merged into master from #8776 to #9082 (master a9ad52584016), one link each, all
  golden. Inputs: 2012 edit rows over 1332 changed files (986 derived, 110 whole, 916 hunks), 134 imports.
- gc_land (the gen-c of specs/queen/golden_chain.t27, decided nowhere else), parents from each PR's last
  merge-base: 80 of 80 land in one pass -- FORWARD 39, CARRIED 32, REGEN 9, REJUDGE 0, CONFLICT 0.
- From each PR's first fork point (before GitHub's update merges): 78 land -- CONFLICT 1 (#9009, stacked on
  #8999) and UNKNOWN_PARENT 1 (#9054, built on #9009: a single pass does not requeue), REGEN 14.
- GitHub, the same 80 PRs: 11 merges of master into a PR branch in 9 PRs, 2 force pushes (1 rebase, 1
  amend), so 13 by the literal count; 4 of them started an extra full CI round on a PR that had already run
  CI (24-35 workflow runs a round). 5 of the merges conflicted (git merge-tree), 2 only in derived files.
- Asserted in the replay: the counts, all 80 outcomes, the fork-point run and a negative control (a real
  service.rs hunk of #8857 moved onto #8847's conflicts). t27c test-report: 3 pass, 0 FAIL, 0 vacuous.
- Not modelled: review and queue waits, CI duration, what a REGEN or a frontier REJUDGE costs, dependencies
  that are not a .t27 `use`, line shifts between two links' bases, history before the window.
