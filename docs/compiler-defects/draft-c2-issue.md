Target repo: gHashTag/t27 (draft - NOT filed; publish on owner confirmation)
Title: t27b: a module-qualified call to an imported fn is rejected instead of resolved (DX)
Type: compiler DX limitation (not a correctness defect - the unqualified form works)
Family: same resolution gap as #13055 (import handling in the standalone/t27b path)

## Boundary
- cli/t27b/src/lower.rs (call resolution: sigs keyed by bare name)
- specs/t27os/research_card.t27 (repro spec, already on PR #13062 stack)

## Repro
`use t27os::dash::pack_cols;` then a call written qualified: `dash::pack_cols(w, 100.0, 8.0)`.
Command: `t27b test c2.t27`
Actual: `t27b: unsupported construct ExprCall(undeclared fn) at line 3 (call to 'dash::pack_cols')`
Expected: resolves through the import and compiles (the unqualified call of the same import compiles).

## User Scenarios
**Given** an import of a pub fn from another module, **when** the call site uses the module-qualified spelling, **then** t27b resolves and lowers it like the unqualified spelling.

## Requirements
- FR-001: qualified calls to imported symbols MUST resolve through the `use` table before the undeclared-fn rejection.
- FR-002: if resolution fails, the diagnostic MUST name the import that was not resolved.

## Success Criteria
- the repro spec compiles under `t27b test` with `exit 0`;
- the generated module contains no `undeclared fn` rejection for a correctly imported symbol.
