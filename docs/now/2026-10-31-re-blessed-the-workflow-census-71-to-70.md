# NOW -- Re-blessed the workflow census, 71 to 70 (2026-10-31)

## Re-blessed the workflow census, 71 to 70 (Closes #12968)

- workflow files read 71 -> 70 in tools/census/quiet.txt and tools/census/shell.txt; census explain traced it to four workflow files: t27os.yml and t27os-release.yml removed, stale-arm-repair.yml added, seal-coverage.yml changed
- Also moved with them: quiet 'named a path but not quiet' 277 -> 285; shell 'run: steps' 342 -> 341 and 'the runner does' 321 -> 320; no gate changed its verdict
