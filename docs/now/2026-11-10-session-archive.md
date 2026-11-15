# Session archive, 2026-11-10 Asia/Bangkok

Cloud agent `bc-3d9fc7fe-32a7-5628-9d4b-2916ec43c2d9`.
Chat: https://cursor.com/agents/bc-3d9fc7fe-32a7-5628-9d4b-2916ec43c2d9

Saved because the owner asked to keep the work and archive the chat. No merge. No mail. No X post.

## Pushed branches

| Branch | Head | Pull request |
|---|---|---|
| `cursor/reseal-queen-dash-c2d9` | `4837bf4e7` | https://github.com/Trinity-S-AI-DNA/t27/pull/13506 |
| `cursor/sieve-kleene-dedupe-c2d9` | `b82ff5367` | https://github.com/Trinity-S-AI-DNA/t27/pull/13505 |
| `cursor/promotion-claim-gate-c2d9` | `503abca7a` | https://github.com/Trinity-S-AI-DNA/t27/pull/13507 |
| `cursor/promotion-baseline-c2d9` | `e333cca22` | https://github.com/Trinity-S-AI-DNA/t27/pull/13529 |

Working tree at save time was clean. Those four branches were already on `origin`.

## What each pull request holds

- #13506 reseals `specs/t27os/queen_dash.t27`. Finished checks were green. Merge stayed blocked because the required `check-linked-issue` workflow ended `startup_failure`. Auto-merge was armed at 2026-11-09 18:14 UTC.
- #13505 shares Kleene `k_and` in `specs/logic/kleene3.t27` and records `cells_with 2` in the duplicate-body ledger. `duplicate-bodies` passed. `coverage`, `spec-guards`, `emit-bitexact`, and `t27b-native-ratchet` were still red.
- #13507 adds `specs/automation/promotion_kernel.t27`. It refuses "84 formats" and unchecked ml_dtypes ratios. Base is `cursor/sieve-kleene-dedupe-c2d9`. The cycle report comment is on this pull request. Issue comments on #13473 returned HTTP 403.
- #13529 adds `docs/promotion/baseline.md`, `baseline.json`, and `README.md`. Capture 2026-11-10 01:14 Asia/Bangkok: stars 5, forks 4, subscribers 0, `open_issues_count` 2107, open PRs 66, contributors 31. https://t27.ai printed 2402 sources, 1861 issues, 16 repositories, snapshot `10/8/2026`. X was not re-read.

## Not written into git before this note

- Grok and Zoho were not opened. BrowserOS neo was not connected. No login wall was reached. `specs/automation/grok_crm.t27` was not created. No issues were filed for that order.
- The Composer 2.5 / Grok 4.7 routing rule was accepted in chat and not copied into a spec. Token cost for that turn was not measured (`getUsage()` was not available).
- `specs/t27os/graph_transition.t27` was not changed. This machine is x86_64, so `t27b test` would not run the JIT.

## Issues filed earlier

- #13503 queen_dash seal
- #13504 t27b ratchet (jit trap site 45 vs interpreter site 46)
- #13474 is the baseline issue named by #13529
