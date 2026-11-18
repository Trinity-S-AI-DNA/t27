# Report index — 2026-11-17 (VF + browser)

Ledger for the read-only coordinator pass (VF PR #14083, browser 0.3.2 host, issue #14080 draft).

**Git:** Every path below under `docs/reports/` from this batch is **untracked** until the owner commits them (no agent commit in this pass).

| File | Purpose (one line) | Required owner action |
|------|--------------------|------------------------|
| [`2026-11-17-vf-14083-review-package.md`](2026-11-17-vf-14083-review-package.md) | VF three-file bundle manifest, WT vs HEAD, CI poll, and post-push checklist for PR #14083. | Say **`commit push lefthook #14083`** so the local lefthook literal + `effective_role` clamp lands on the PR; then run **POST_PUSH_CHECKLIST** in that doc. |
| [`2026-11-17-browser-032-tick.md`](2026-11-17-browser-032-tick.md) | BROWSER_032_TICK: running T27OS pid, `~/.t27os/browser/` artifacts, loopback 47131 plan, SPEC_GREEN vs host FAIL. | **Relaunch** rebuilt dev app (`target/t27os-local/T27OS.app`, optional `T27OS_BROWSER=1`), open **BROWSER** tab; run **OWNER_POST_RELAUNCH_COMMANDS** there to confirm listener/`native.png`. |
| [`2026-11-17-dual-lane-handoff.md`](2026-11-17-dual-lane-handoff.md) | Side-by-side VF vs browser status, blockers, and split after owner acts. | Execute **both** lanes independently: VF push phrase above **and** browser relaunch above (VF push does not relaunch T27OS). |
| [`2026-11-17-14080-issue-update-draft.md`](2026-11-17-14080-issue-update-draft.md) | Paste-ready #14080 update: startup_failure window, tail greens, #14083 linked-issue pass. | **Paste** on gHashTag/t27#14080 (or close/retitle per recommendation); **do not** auto-post from agents. |

Also indexed: this file — [`INDEX-2026-11-17-vf-browser.md`](INDEX-2026-11-17-vf-browser.md) (navigation only; commit with the batch when retaining the ledger).

`2026-11-17 13:49 +0700` UNCHANGED — T27OS pid **31586**; `lsof -iTCP:47131 -sTCP:LISTEN` empty; `~/.t27os/browser/native.png` absent.