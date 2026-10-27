# How to add entry N+1 to epic_progress.t27 (the three-command card)

Measured path of 2026-10-27, entry 2 (PR #12799): three commands, one
test-report, one armed PR. Every number comes from a live gh command, never
from memory; every assert in the card classifies CT-VERIFIED or CHECKED --
never vacuous -- because the reads are bare array indexes (#12777, #12785).

## 1. Measure (three commands)

    gh pr list -R Trinity-S-AI-DNA/t27 --state merged --limit 30 --json number,mergedAt \
      --jq '[.[] | select(.mergedAt >= "YYYY-MM-DDT00:00:00Z")] | length'   # MERGED
    gh pr list -R Trinity-S-AI-DNA/t27 --state open --limit 100 --json number --jq 'length'
    git ls-remote origin 'refs/heads/*' | wc -l                             # BR_END

OPEN_START = the previous entry's OPEN_END; BR_START = its BR_END;
QUEUE_PEAK = the day's highest `gh api repos/.../actions/runs?status=queued`
count (the train-watch card specs/crons/t27-train-watch.t27 holds the
thresholds: WATCH at 100, STORM at 300).

## 2. Write (arrays to [N+1], SHOWN to the newest)

- every column array grows one element; append, never reorder -- the SHOWN
  block must keep naming the newest index and the numbering invariant in the
  tests must keep pinning it;
- SHOWN_CLEARED_PCT_TENTHS = (OPEN_START - OPEN_END) * 1000 / OPEN_START,
  rounded to nearest, sign included: entry 1 recorded an honest -143, entry 2
  the first positive +187 -- a format that cannot print a true negative must
  not be handed a false positive;
- TITLE/SUMMARY/body are rewritten for the new shift; the numbers in the text
  are the same ones the columns hold.

## 3. Verify and land

    t27c test-report specs/blog/epic_progress.t27

Expect: every test CT-VERIFIED or CHECKED, `vacuous passes 0 of N`. Then
commit, push, open the PR, `gh pr merge --squash --auto`. Two past traps this
card encodes: `;`-comments inside a test body are a parse error (module level
only), and [1]T arrays hit the #5980 single-element defect (entry 0 shipped
as scalars; [2]T and [3]T emit cleanly).

## The [N+1] skeleton, ready to fill

Copy this over the arrays, replace every MEASURE with the live command's
number, flip every SHOWN to index N, and the test-report does the rest. The
numbers are placeholders by design: a placeholder that reaches master is a
lie, so the card's own asserts are the gate that stops it.

    pub const DATE: [4]u32 = [20261026, 20261027, 20261027, MEASURE];
    pub const EPIC: [4]u32 = [12660, 12660, 12660, 12660];
    pub const OPEN_START: [4]u32 = [55, 14, 16, MEASURE];   // = OPEN_END[2]
    pub const OPEN_END: [4]u32 = [25, 16, 13, MEASURE];
    pub const MERGED: [4]u32 = [54, 7, 18, MEASURE];
    pub const BR_START: [4]u32 = [6391, 678, 696, MEASURE]; // = BR_END[2]
    pub const BR_END: [4]u32 = [678, 696, 736, MEASURE];
    pub const QUEUE_PEAK: [4]u32 = [502, 139, 139, MEASURE];

    // every SHOWN_*: the newest index becomes 3; the percent sign falls out
    // of the live numbers -- keep it honest either way.
