# Browser pump latency, static page (Refs #14037)

Order BROWSER_PUMP_STATUS_PROBE_V1. Measured 2026-11-16 11:06-11:09 UTC on the bench Mac, dev app
`ai.t27.os.dev` build 8358 (target/t27os-local), one pump process (pid 98673), Chromium over CDP.

## What is measured

`latency_ms` = time the pump wrote the first screencast frame after an input line, minus the
modification time of `in.txt` when the pump read that line (ms resolution, APFS). It covers inbox
pickup, the CDP call, Chromium render, JPEG encode, the socket and the frame write. It does NOT
cover the app noticing the new `seq.txt` and drawing the frame in the window.

The probe is in `BR_PUMP_SCRIPT` (specs/t27os/glass_window.t27): it runs only while
`$D/trace.on` exists, and appends one JSON line per input line to `$D/status.jsonl`. A line with no frame
within 3 s, or replaced by the next input, is written with `frame_seq -1` and `latency_ms -1`
(dropped). The pump's `status.txt` keeps its old format because the app parses it. CDP, input parsing,
screencast and restart logic are unchanged.

## Page and drive

`static-test.html`: no script, no animation, no autoplay. It has a CSS checkbox toggle (click), a
textarea (keys), nine `:target` cells (go/back/forward) and 24 bands of 400 px (scroll). Idle check:
0 frames in 5 s once the textarea had no focus. A focused caret blinks at about 2 frames/s; typing
restarts the blink, so the frame after a key comes from the key.

The inputs were appended to `in.txt` by an inline zsh loop: 0.6 s apart for click, keys and scroll,
1.0 s apart for go, back, forward and reload, and 1.2 s apart for fit. Fit alternated between
843x683 and 883x723. The CSV and statistics come from `jq` over `status-samples/status.jsonl`, using
nearest-rank percentiles.

## Results (ms)

| action | n | min | avg | p50 | p90 | p95 | max |
|---|---|---|---|---|---|---|---|
| click | 23 | 28 | 40.1 | 37 | 53 | 58 | 66 |
| scroll | 22 | 0 | 12.1 | 12 | 20 | 22 | 30 |
| keys | 22 | 18 | 28.9 | 25 | 46 | 49 | 53 |
| go | 22 | 37 | 45.6 | 45 | 54 | 54 | 55 |
| back | 22 | 35 | 44.1 | 44 | 52 | 53 | 54 |
| forward | 22 | 34 | 45.0 | 45 | 52 | 52 | 53 |
| reload | 22 | 36 | 45.9 | 45 | 53 | 55 | 58 |
| fit | 22 | 32 | 68.6 | 43 | 57 | 63 | 585 |
| **all** | **177** | **0** | **41.3** | **41** | **53** | **55** | **585** |

Dropped frames: 0. Restarts: 0 (one pump pid for all samples). Viewports: 883x723, 843x683 (fit).
Click includes 1 extra sample: the click that focused the textarea before the keys block.

## Reading

- Click p50 is 37 ms and scroll p50 is 12 ms. Both are well under the 100 ms target on the pump leg.
- One fit took 585 ms, at 11:08:17.276. A fit resizes the window and restarts the screencast, so
  an occasional slow first frame is expected. It is the only sample over 66 ms.
- Scroll min 0 ms and p50 12 ms are suspiciously fast. A wheel input starts a smooth scroll of about
  7 frames. Inputs were 0.6 s apart and each sample's `frame_seq` is new, but a late frame from the
  previous animation cannot be ruled out. Read scroll as a floor, not a proof.

## Do frames match inputs?

Screenshots were taken after each block, in `screenshots/`. Their md5 values show the expected state
round-trips:

- `01-keys` differs from `00-static-loaded`: the typed text is visible.
- `02-click` equals `01-keys`: 22 toggles is an even count, so the checkbox is back where it started.
- `05-back` equals `03-scroll`: 22 back steps return to the page as it was before the go block.
- `06-forward` and `07-reload` equal `04-go`: forward returns to the last go, and reload changes nothing.
- `08-fit` equals `04-go` at 1766x1446: the 22nd fit restores 883x723.

There are no screenshots per sample. Matching is shown per block and by `frame_seq` advancing on every
sample.

## Reproduce

1. Build the dev app with `make t27os T27OS_OUT=target/t27os-local` and `T27OS_CHANNEL=dev`, then open
   the BROWSER page.
2. Send `go file://<abs>/static-test.html`.
3. Run `touch $D/trace.on` and drive the same lines.
4. Remove `trace.on` and copy `$D/status.jsonl`.

## Limits

- `t27b test` is refused on this Mac by the load hook (MAC LOAD RULE). The spec tests were not re-run
  after the probe; the hook was not worked around.
- The driver loop and the `jq` statistics are inline shell plumbing, with no script files.
