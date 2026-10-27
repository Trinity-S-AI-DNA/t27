---
name: foreign-budget-discipline
description: The own-language gate enforces TWO independent limits -- at most 40 hand-written foreign lines added per file (80 per PR) AND added <= deleted overall. Load before ANY commit touching .rs/.py/.ts/.js/.yml/.sh in t27, and whenever a commit is refused with "more than 40 ... in one file" or "more hand-written foreign lines added than deleted".
---

# Foreign budget discipline (measured 2026-10-27, PRs #12711/#12748/#12777/#12784)

## The two caps are independent

1. **Added cap**: at most 40 added lines in ONE foreign file, 80 across the PR.
   A net-negative diff with 96 added lines still FAILS.
2. **Shrink cap**: added <= deleted across all foreign files. A 19/8 diff
   fails even though 19 < 40.

`git diff --cached --numstat` per file BEFORE committing; foreign = everything
not `.t27`/`.md`/`.txt`/`.json`/generated roots. No label lifts either cap.

## The honest way under both (worked four times in one session)

- Compress the additions YOU made: docs to fewer lines (keep meaning), branches
  folded to one line, concat pieces merged with identical emitted text.
- Delete blank `///` paragraph separators in the same files (rustdoc-neutral).
- Split a stacked change: the count PR first (#12784 pattern), the verdict
  PR second (#12788) -- each lands net-zero on its own.
- A `;`-comment inside a .t27 test/fn body is a PARSE ERROR: move it to
  module level (caught three times).
- `grep "A\|B"` without -E silently matches nothing on BSD -- re-check any
  "0 matches" verdict with -E (instrument-calibration law).

## Verdict-layer mapping (CT-VERIFIED, #12751)

pass_vacuous = 0 runtime asserts AND 0 folded checks (truly trivial);
pass_ct = 0 runtime asserts, ct > 0 (verified at compile time);
CHECKED otherwise. The law lives in specs/compiler/theory/toolchain.t27
`report_state`; the words in specs/hosting/row.t27 T27B_WORDS; the steward's
is_pass and the rules VERDICTS mirror accept the appended index -- append,
never insert: stored codes must not move.
