Target repo: gHashTag/t27 (draft - NOT filed; publish on owner confirmation)
Title: t27b DX: @floatFromInt applied to an f64 operand reports an unsupported construct instead of suggesting @intFromFloat
Type: compiler DX limitation. RETRACTED as a correctness defect: the observed repro misused the builtin (f64 -> int needs @intFromInt); t27b's refusal matches Zig semantics. The genuine gap is the diagnostic not naming the conversion that would apply.

## Boundary
- bootstrap/src/compiler.rs:7944 (is_float_expr, ExprCall arm: resolves only via declared_fn_returns of the same spec)
- bootstrap/src/compiler.rs:11205 (@floatFromInt emission for (true,false))
- cli/t27b/src/lower.rs:165 (unsupported-construct reporting)

## Repro
`pub fn widen(x: f64) -> u64 { return @as(u64, @floatFromInt(x)); }`
Command: `t27b test c3.t27`
Actual: `unsupported construct ExprCall(@floatFromInt) at line 2 (result type u64)`
Zig semantics: @floatFromInt requires an integer operand; f64 -> u64 is @intFromFloat.

## User Scenarios
**Given** an explicit @floatFromInt cast whose operand is statically f64, **when** t27b lowers the cast, **then** it either lowers @intFromFloat (the semantically implied conversion) or reports the construct with a hint naming @intFromFloat.

## Requirements
- FR-001: an invalid input MUST be refused; the refusal message MUST name the offending builtin (@floatFromInt) and state the type mismatch (operand is f64).
- FR-002: an alternative-builtin hint (@intFromFloat) is allowed only as an explicit hint, not as silent reinterpretation of the cast.
- FR-003: no automatic lowering of an invalid cast just to make the string check pass.

## Success Criteria
- the refusal message names @floatFromInt and states the type mismatch (machine-checkable: the text contains both `@floatFromInt` and `f64`);
- the repro still exits non-zero under `t27b test`.
