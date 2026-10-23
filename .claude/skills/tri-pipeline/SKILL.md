---
id: tri-pipeline
name: TRI Pipeline
description: The exact commands of the t27 toolchain (tri, t27c, t27b, tri t27o) for spec-first work -- check, test with per-test counts and vacuity, seal, lower to Verilog, convert foreign code, gate a PR. Every command named here exists; checked against --help on 2026-10-18.
---

# TRI Pipeline Skill

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## Three things called `tri`

| Name | What it is | Note |
|---|---|---|
| `scripts/tri` | bash front door; runs `scripts/tri_loop/*.py` helpers first (`pr-state`, `priority`, `triage`, `tick`, `t27b`, `toolbelt`, `damage`, `swarm`; list: `tri loop-help`), else forwards to `t27c` | `scripts/tri test` is the WHOLE-REPO `t27c suite`, not one spec | <!-- not built as a t27c subcommand -->
| `target/release/tri` | Rust CLI, `cargo build --release -p tri`, ~55 subcommands; `--help` is the authority | `tri test <spec>` is the one-spec runner |
| `t27c` | the compiler, `cargo build --release -p t27c` | lands at `./target/release/t27c` (workspace root) |

Use `./target/release/tri` and `./target/release/t27c` by path when the difference matters.

## One spec, end to end -- fast path first

The native JIT for the inner loop, the seal to skip what did not change, the Zig reference once at the end. Measured on the bench Mac, 2026-10-18, under load:

| Step | Command | Cost |
|---|---|---|
| parse + typecheck | `./target/release/t27c check <spec>` | under 1 s |
| inner loop, after every edit | `./target/release/t27b test <spec>` | rust_std.t27 0.89 s; rust_convert.t27 (66 tests) 13.5 s |
| specs you did not change | `./target/release/t27c frontier --only <specs> --list` | about 5 s for 3 specs; REUSE = seal, closure and toolchain still match, so the sealed verdict stands: do not re-run its tests |
| final gate, once | `./target/release/t27c test-report <spec>` and `tri test <spec>` | rust_std.t27 94 s; per-test assert counts and `vacuous passes N of M` |
| seal | `./target/release/t27c seal <spec> --save`, then `--verify` | writes `.trinity/seals/<dir>_<Module>.json`; must print "all hashes MATCH"; the seal is also the REUSE key |

```bash
export ZIG_GLOBAL_CACHE_DIR=$TMPDIR/zig-cache ZIG_LOCAL_CACHE_DIR=$TMPDIR/zig-local   # every sandboxed shell, every time
```

- **Do not build what already exists.** If another worktree of this checkout has release binaries, symlink them: `mkdir -p target/release && ln -sf <that worktree>/target/release/{t27c,t27b,tri} target/release/`. Build only the crate you changed, into one shared `CARGO_TARGET_DIR` (cargo's lock serializes builds), and copy the binary out right after.
- **t27b's exit code** (`cli/t27b/README.md`): 0 pass, 1 a test failed, 2 unsupported construct, 3 front end, 4 JIT and interpreter disagree (`--check`), 5 codegen limit. Only 0 and 1 are a verdict; on any other code run `test-report` (the rule is `t27b_outcome` in `specs/tools/t27o/ledger.t27`, #10525).
- Without the two `ZIG_*` variables, `test-report` says BLOCKED and `seal --save` refuses with "does not compile: failed to create output directory ... PermissionDenied". That is the sandbox, not the spec.
- `t27c test <spec>` only LISTS the test blocks; it runs nothing. Never quote it as a test result.
- A pass with 0 runtime asserts is vacuous; `tri test` fails on it since #10507. t27b prints only the total assert count, so the vacuity number comes from `test-report`.
- Negative control: copy the spec to `$TMPDIR` (or beside it when it has a `use`), break one constant, run `t27b test`, and see a FAIL before you claim the tests mean anything.
- `t27c test-report` exits 1 on FAIL and 2 on BLOCKED (did not compile).
- `gen/` is in `.gitignore`: a generated file a crate includes by `#[path]` is committed with `git add -f`.

## Generate and lower

- `t27c gen-verilog | gen-c | gen-rust | gen <spec>` print to stdout (`gen` is Zig). `tri gen <spec>` runs gen-verilog, gen-c and gen-rust to stdout and writes nothing.
- `t27b verilog <spec>` (native backend, `cargo build --release -p t27b`) is the path for FPGA board tops: it can instantiate primitives and other modules, takes pin attributes `<port>_LOC|IOSTANDARD|SLEW|IN_TERM` (no XDC), and writes a testbench under `-DT27B_TESTBENCH`. It **refuses signed types** (exit 2): fixed-point hardware specs keep a sign flag with unsigned magnitudes.
- `t27b test <spec> [--check]` is the AArch64 JIT runner (fast; prints total runtime asserts, not per test). Exit 4 means the JIT and the interpreter disagree.
- AX7203 caveat (#10498): `t27c fpga-build` defaults to the Wukong fgg676 part and `t27c fpga-flash` has no ax7203 board; the AX7203 is `xc7a200tfbg484-2` (`fpga/HARDWARE_SSOT.md`). Follow M0 (`specs/fpga/rc_filter_ax7203.t27`, #10276) for the board flow.

## Foreign code in

- `tri t27o <file.rs> [--name N] [--oracle] [--check]` turns a Rust file into a `.t27` skeleton; refused bodies keep their signature and a comment naming the refused construct. Rules: `specs/tools/t27o/rust_convert.t27`. Rust is the only front end today; a new language is a new `specs/tools/t27o/<lang>_convert.t27` beside it (the SPICE front end for #10486 goes there).
- The last line of every conversion is the file's count (`t27o: N fn seen, M converted ...`). Coverage on the roadmap repositories, the refusal kinds and the ten weaknesses: `docs/nona-03-manifest/T27O-COMPETITORS-AND-DOGFOOD.md`. How to change the converter and measure it: the `t27o` skill.

## Is someone already on it?

- `tri topic <keywords>` -- open issues, PRs and recent commits that match. Run it before opening an issue or a lane.
- `tri loop claim <name>` -- take a named claim so two loop ticks or sessions cannot pick the same task.

## Pull requests and CI

- The required check `own-language` fails a PR whose hand-written foreign lines (Rust, Python, shell, ...; not specs, not `gen/`) gain more than they lose over the whole diff, beside the 40-per-file / 80-per-PR cap: glue is paid for by deleting as much hand code in the same diff. Check with `git diff --numstat origin/master...HEAD` before the first push (the `t27o` skill has the one-liner).
- `tri pr ready <N> [--wait --required-only]` prints one `VERDICT:` line (safe to merge, DO NOT MERGE, WAIT, CANNOT TELL, NEW REASON) after classifying every failing check.
- `tri pr landed <N> --probe S --file P` -- did the content actually reach the default branch.
- `tri night`, `tri red now|why`, `tri ci baseline`, `tri gates ...`, `tri merging`, `tri lanes --repo t27 --base master`.
- `tri seals fresh|drift|twins|hollow` -- seal health across the tree.
- `tri hooks pre-commit|commit-msg|pre-push|l1-check|status` -- the commit gates, runnable by hand.
- Commit in a worktree with `git -c core.hooksPath=.githooks commit`: the configured hooks path is the main checkout's.

## Experience

- `tri experience save` (no flags) is the only experience command. There is no `tri experience query` and no `tri verify`. <!-- not built as a t27c subcommand -->

## MCP servers

- **t27 MCP** (gHashTag/t27-mcp, local stdio): `t27_check` (compiler front end in wasm: parse + typecheck only, no tests), `t27_spec`, `t27_specs`, `t27_grep`, `t27_find`, `t27_examples`, `t27f_outline`, `t27f_graph`, `t27_dashboard` and more. Use `t27_check` for a fast syntax check without a build.
- **TRI CLAW** (`/mcp` of the vibee-render service): about 170 tools; `t27_specs` and `hdl_run` (verilator lint, iverilog sim, yosys synth_xilinx) are the ones spec work uses. The t27 tools are being mounted there (#10483, gHashTag/t27-mcp#26).
- `cli/tri-mcp` is not buildable (not a workspace member, #5395) and its `tri_test` only lists tests (#8140, fix in #8149). Do not rely on it.
