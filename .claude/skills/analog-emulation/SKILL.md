---
id: analog-emulation
name: analog-emulation
description: Drive epic #10486, t27 analog emulation v0.1 with Symbiotic EDA -- an analog netlist with IHP SG13G2 models becomes one .t27 spec that is the reference model and the FPGA design, run per corner and judged by ngspice and Symbiotic EDA's evaluator. Use when working a milestone (M1-M6), when updating the status dashboard, or when writing a progress report to the Symbiotic EDA contact on X.
argument-hint: "[milestone, e.g. M1] | report | dashboard"
---

# /analog-emulation -- netlist to t27 spec to FPGA, per corner

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## Where the truth lives

| What | Where |
|---|---|
| Plan, milestones, acceptance, needs | epic gHashTag/t27#10486 (edit the epic, not a copy) |
| Current milestone | its sub-issue, opened when the milestone starts (M1 is #10487) |
| M0, the pattern every cell follows | `specs/fpga/rc_filter_ax7203.t27`, #10276, PR #10278 |
| Status dashboard | https://claude.ai/artifact/75G2ZjZ5CihcWPNLgFDxwD (private artifact; the owner shares it) |
| External references | mirekez/cpphdl `analog/` (examples, IHP setup in `analog/pdk`), ckdur/asicone_sg13g2, regymm/pcie_7x |
| The conversation | the owner's X chat with the Symbiotic EDA contact; context in the owner's memory, never in this repo |

## The milestone loop

Each milestone follows M0's shape. Do not mark one done until all five hold:

1. **Spec.** One `.t27` module that is both the reference and the hardware. The header comment states the circuit, the step, the number format, the run, the receipt line, *what a PASS says* and *what it does not say*.
2. **Tests on the lab.** Error bound against a float64 run of the same method, named in the spec. 0 vacuous. ngspice + IHP runs (M3 on) happen on the Railway lab, never on the Mac.
3. **Negative control.** A mutant (truncation for rounding, a mis-stamped element, the wrong corner's table) that fails the tests, is refused by `t27c seal`, and on the board prints the predicted FAIL line.
4. **Board receipt.** Bitstream from `t27b verilog` (no hand HDL, no XDC), run on the AX7203 bench (`board-eye`, `bench-agent` skills). Record LUT, FF, DSP, BRAM, Fmax and the UART line.
5. **Oracle.** From M3 on, ngspice with the named PDK revision and corner. From M5 on, Symbiotic EDA's self-checking evaluator decides.

Then: close the sub-issue with `Closes #N`, tick it in the epic, open the next milestone's sub-issue in the same Goal / Boundary / Scenarios / Requirements shape, update the dashboard, and draft a report.

### Technical choices that are already decided

- **Fixed step, compile-time matrices.** Linear MNA `(G + C/dt) x' = (C/dt) x + B u` folds to `x' = M x + N u` with constants; no solve and no division in fabric. M0 is the 1x1 case (`g = 1/21`). The stability invariant (quantized `M`, spectral radius < 1) belongs in the spec's tests.
- **Nonlinear devices as per-corner tables first.** `Id(Vgs, Vds)` from the PDK's PSP103 in ngspice, one table per corner (tt ff ss fs sf), one bitstream per corner. Compact-model (Verilog-A) ingest is post-v0.1 research; never promise it for v0.1.
- **Designers keep their netlist.** The entry point is a SPICE subset read by a t27 program. Unsupported input is refused with the line and the reason, never approximated silently.

## Claim guard

Every number that leaves the repo -- in a report, the dashboard, a post or a partner summary -- comes from a test or a receipt in gHashTag/t27. Before sending, check each sentence against the receipt:

- Never claim power (nothing measures it), accuracy outside measured circuits, real-time or "faster than SPICE" unless measured on the same circuit, or PCIe throughput before it runs.
- Say "table model of PSP103", not "the IHP transistor model", for M3-M4.
- If a previous message overstated something, the next report corrects it explicitly.

## Progress reports to the Symbiotic EDA contact

The owner wants a report on every breakthrough and every milestone. The owner sends, or confirms each send; Claude drafts.

- **When:** a milestone closes; a board receipt or oracle result lands for the first time; a blocker appears that needs something from them; a claim made earlier turns out wrong.
- **Language and form:** English, plain, written. Lead with what was measured, then what it does not show, then the next step and what is needed from them. Link the PR or issue. No more than about 12 lines.
- **Template:**
  ```
  <Milestone>: <one-line result>.

  What we measured:
  - <number> (<source: test / receipt>)
  - ...
  Not covered yet: <boundary>.
  Next: <next milestone> by <date>. From you we would need: <item, if any>.
  <link to PR or issue>
  ```
- **Privacy:** the chat is private. Nothing from it (names, budgets, proposals, payments, accounts) goes into the public repo, issues or PRs. The epic says "Symbiotic EDA" and nothing more personal.
- **Sending on X:** in BrowserOS neo, open the chat. `act type` drops characters in X's composer; use `act fill` with `clear=false` per paragraph and Shift+Enter between paragraphs, verify with a screenshot, then press the Send button by its ref. Read the chat first: X keeps message text in a shadow root and a virtualised list, so read `[role=log]` through the shadow roots and scroll it with mouse-wheel events.
- After sending, add a line to the dashboard's report log and republish it.

## Updating the dashboard

The page is a single HTML file whose data sits in four arrays at the top of its script: `MS` (milestones, status `done|active|planned|blocked`), `PATH_STATE` (the seven stages of the signal path), `NEEDS`, and `LOG`, plus `UPDATED`. Read the live artifact, change only those values, and republish to the same URL. A status changes only when the epic or a receipt says so.

## Traps

- `gh` fails TLS inside the Claude sandbox (x509 OSStatus -26276); `curl` with `$(gh auth token)` against api.github.com works.
- The asicone sub-blocks (`ro_reliability`, `saradc_auto`) are submodules on gitlab.symbioticeda.com; a shallow clone of asicone_sg13g2 leaves them empty. Ask for the netlist instead of guessing it.
- `t27b` bitstream specs are skipped by the fpga-build job and have no gen-verilog rows (#10300); follow M0's ledger handling.
