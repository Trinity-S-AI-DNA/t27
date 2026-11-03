Two more instances, measured 2026-11-02 on `lane/t27b-floor-model`:
`specs/tri/shootout/codegen_audit.t27` and `specs/tri/shootout/acceptance_matrix.t27` each
drop 4 tokens on `use tri::shootout::bench::MEASURED_T27B_BASELINE;` -- the shape this issue
describes, with `bench` (`compiler.rs:453`) in place of `test`. `specs/t27os/glass_window.t27`
carries about 20 `use t27os::bench::X;` lines and is among the 48 discarding specs, so the
keyword set that triggers this is at least `test`, `invariant` and `bench`, and the loss is
corpus-wide rather than one module's.

Three things read from source, worth having here before the parser half is attempted.

**No spelling reaches a module whose path contains a keyword.** The `::` loop and the dotted
branch (#5978) both require `Ident` after the separator; the braced list (W630) requires
`full_path.ends_with("::")` and `Ident` inside the braces; `using x: @import("...")` parses
cleanly but `use_resolve::use_path_expr` reads only lines beginning `use `, so it splices
nothing. `use_target` drops a single trailing segment, so a file at `a/b/bench.t27` has
exactly two spellings and both carry the keyword. FR-001 is therefore not a style rule a spec
can satisfy by rewording: for a module at such a path there is no reachable form.

**A dropped `use` line is not a dead one.** t27b splices `use` targets textually from the raw
source before parsing (`cli/t27b/src/front.rs`) and ignores `UseDecl` when lowering
(`cli/t27b/src/lower.rs:755`), so the full path resolves while the parse stays dirty. `t27c
check` prints `Typecheck OK (0 errors, 0 warnings)` for such a spec and its tests read real
values. Deleting the line to clear the discard breaks the asserts that consume the import,
and a clean typecheck does not warn.

**The ledger workaround FR-003 names is not available now.**
`docs/reports/suite_expectations.json` holds 75 entries against `max_entries: 75`, and W699
requires a volume number on a discard-phase row.

The spec-side instance is filed as #13113, which re-homes the measured baseline out of the
keyword path and leaves `bootstrap/` alone. This issue keeps the parser half.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
