# CDP observer live report — 2026-11-11

ORDER: T27_CDP_OBSERVER_LIVE_V8. LIVE read-only observation EXECUTED against
a real, isolated headless browser — then torn down cleanly.

## Wave 1 — resource guard (PASS, 00:49+07)

Swap 5867/7168 MB, load 4.35/8.50/18.65, disk 29 GiB free. Calm machine;
browser launch allowed.

## Wave 2 — observer profile (CREATED, then removed)

- Binary: `/Applications/BrowserOS.app/Contents/MacOS/BrowserOS` (Chromium
  155.0.8309.26). A completely FRESH `--user-data-dir` means zero credentials,
  zero extensions, no default profile — the user's signed-in profile was never
  touched.
- Flags: `--headless=new --user-data-dir=/Users/playra/t27/.claude/
  cdp-observer-profile --remote-debugging-port=9223 --no-first-run
  --no-default-browser-check --disable-extensions about:blank`.
  CDP binds loopback by default; `/json/version` confirmed Chrome/155 on
  127.0.0.1:9223 only.
- Teardown: browser process killed, profile directory deleted, endpoint down
  (verified). No litter.

## Wave 3 — observer verification (PASS)

Two defects found and fixed live (commit `409e57fd75`, +5/-2, hooks green):
1. Chrome 155 answers HTTP/1.1 only — the HTTP/1.0 probe got an empty reply.
2. Chrome keeps the socket alive past `Connection: close` — the observer hung
   waiting for EOF; now bounded by a 3 s read timeout.

Final measurement against the real browser (stdio, exit 0):

    connected:true, decision ALLOW,
    observed: 1 page {url: "about:blank", title: "about:blank",
                      domSummary: {targetType: "page"}},
    outputs: [url, title, domSummary], actions: "none: navigation, click,
    input, form submit and credential or storage export are DENY rows in
    policy.t27"

Non-loopback DENY re-verified through the installed server (10.0.0.9 and a
bare-args call to the separate pre-existing `t27-gateway` mount both DENIED —
deny-by-default on an unspecified endpoint works as pinned).

## Wave 4 — a11y and screenshot (NOT IMPLEMENTED — budget verdict pending)

Owner decision requested: the 40-added-line per-file cap on
`cli/tri-mcp/src/main.rs` is effectively exhausted (39 + 40 + 5 across three
commits, offset by deletions). The websocket-based a11y snapshot and the
screenshot-hash path are pinned OBSERVABLE in law but not fetched. Options:
(a) owner headroom verdict for the adapter file, (b) refactor to free lines,
(c) a new generated-path or exception-approved module. Law needs no change.

## Remaining gaps

1. Wave 4 a11y/screenshot (budget verdict above).
2. Adapter does not yet emit the law's seven-field audit event per
   observation; the harness logs every call, and the audit schema is tested
   in the law — wiring it into the adapter belongs with the Wave 4 increment.
3. Swap now 5.9/7.2 GB: OPENXC7/Docker still HOLD; M1.3 `b693c56d2b`
   independent verification and `synth_receipt.t27` decision remain open.
