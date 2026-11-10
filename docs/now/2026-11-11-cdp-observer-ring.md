# CDP observer ring report — 2026-11-11

ORDER: T27_CDP_OBSERVER_READONLY_V7. Read-only observer LANDED in law and
adapter (commit `127bc95330` on `lane/mcp-m0`); live browser run DEFERRED by
the Wave 1 resource guard.

## Wave 1 — resource guard (STOP for the browser)

At 00:36+07: swap 12199/13312 MB (92%, 1.1 GB free), load 13.9. Browser
launch STOPPED per the order. The bounded law/gen/adapter/test work
proceeded; no browser was started at any point.

## Waves 3-4 — observer contract in law (DONE, spec-first)

`mcp/gateway/policy.t27` (commit `127bc95330`):
- `OBS_URL / OBS_TITLE / OBS_A11Y / OBS_DOM_SUMMARY / OBS_SHOT_HASH` with the
  `OBSERVABLE` pins — the only outputs cdp.observe may return; a screenshot
  appears only as its hash, pixels never cross.
- `ACT_NAVIGATE / ACT_CLICK / ACT_INPUT / ACT_FORM_SUBMIT` ->
  `DENY_BROWSER_ACT`; `ACT_COOKIE_EXPORT / ACT_STORAGE_EXPORT` ->
  `DENY_CREDENTIAL`; everything else DENY_UNKNOWN (`observer_action_reason`).
- New test `cdp_observer_contract_is_read_only` pins all of it.
- Loopback-only CDP origin unchanged (127.0.0.1:9222..9332), rate limit 30/60s,
  seven-field audit unchanged.
- Gates: `t27c test` 12/12, `t27b test` 12/12 + 2 invariants, 0 vacuous.
- `gen/rust/tri/gateway_policy.rs` regenerated (`t27c gen-rust`, committed
  with `-f` per CLAUDE.md) and the seal re-saved:
  `gen_hash_rust sha256:d66526fb…`; spec hash `187e6b72…`.

## Adapter (40 added lines, at the per-file cap)

`cdp.observe` on an ALLOWed loopback endpoint performs exactly one read-only
HTTP/1.0 GET of `/json/list` and returns url/title/domSummary per page target
plus `outputs` and `actions:"none"`; any connect failure degrades to
`connected:false` with the error noted. `dsh.status` remains non-live.

## Wave 5 — tests (ALL GREEN)

- `t27c test mcp/gateway/policy.t27`: 12/12.
- `t27b test mcp/gateway/policy.t27`: 12/12 + 2 invariants.
- `cargo test -p tri-mcp`: 1 passed, 0 failed (regression test).
- Live path measured against a throwaway loopback test endpoint (no browser):
  `connected:true`, one page with url/title/domSummary, decision ALLOW,
  actions none. Non-loopback through the INSTALLED server: DENY end-to-end.

## Waves 2/6 — browser profile (DEFERRED) and remaining gaps

1. Dedicated Chrome observer profile (loopback debugging, no creds, no
   extensions) — NOT launched; swap guard. Command when guard lifts:
   Chrome with `--user-data-dir=<fresh tmp profile> --remote-debugging-port=9223
   --no-first-run --disable-extensions` (loopback by default), then
   `cdp.observe` against 127.0.0.1:9223.
2. Accessibility snapshot and screenshot hash need the websocket debugger
   protocol — the law pins them OBSERVABLE, the adapter does not fetch them
   yet; that is the next bounded adapter increment (and will need budget
   headroom in main.rs: the 40-line cap is fully used).
3. The installed plugin may serve the pre-V7 binary until its MCP client
   process reconnects; behavior is safe either way (old = non-live envelope,
   new = read-only live list).
4. Still open: swap normalisation (OPENXC7/Docker), `synth_receipt.t27`
   decision, M1.3 `b693c56d2b` independent verification.
