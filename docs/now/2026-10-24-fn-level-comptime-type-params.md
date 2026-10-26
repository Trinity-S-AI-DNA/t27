# NOW -- fn-level comptime type params: parser + Zig emitter (2026-10-24)

## fn-level comptime type params, sub-slice 1 of #11940 (Refs #11940 #11840)

- `bootstrap/src/compiler.rs`: `fn name(comptime T: type, ...)` now parses. Before,
  `comptime` lexed as a plain Ident and the sequence became three parameters --
  `("comptime", "T"), ("type", "")` -- and every backend emitted an invalid
  signature (`fn identity(@"comptime": T, @"type": , value: T) T`).
- The Zig emitter renders Zig's generic-function idiom: a parameter whose type
  text is `type` emits as `comptime T: type`; call sites pass the type
  explicitly (`identity(u8, 42)`), per the slice's scope decision -- call-site
  type inference is the recorded follow-up, not this slice.
- A struct literal attached to a call (`Box(T){ .kept = true, .item = value }`,
  the shape `fn pure(comptime T: type, value: T) Maybe(T)` needs) was captured
  by `parse_expr_postfix` as token-joined TEXT on the call's `value` and then
  DROPPED by the Zig emitter -- the body emitted `return Box(T);`, a bare type.
  The emitter now writes the captured literal; the unused-parameter scan reads
  the literal text, so a parameter read only inside it is no longer discarded.
- A comptime VALUE parameter (`comptime x: i32`) is refused with a parse error
  naming the line, not silently parsed as a type parameter.
- Verification: repros in /tmp/opencode/repros/ (`fn_comptime_type.t27`,
  `fn_comptime_compose.t27`) pass `zig test` and native + wasm32
  `zig build-obj`; 5 new unit tests in `tests_11940_fn_comptime_type_params`;
  `cargo test -p t27c --bin t27c` 1802 passed (one pre-existing timing-flaky
  harness test, `a_looping_test_is_stopped_and_counted_as_failed`, flakes under
  full-suite parallel load; green in isolation, and this diff cannot reach its
  probe -- no comptime tokens, no call-attached literals).
- M5 re-stamp: `bootstrap/stage0/FROZEN_HASH` follows the live hash (ceremony
  re-run after every build in this slice).
- The 14 specs blocked on #11940 stay blocked at their OWN next defect
  (maybe.t27's fn-type aliases = sub-slice 2; option/tree/queue need their
  sources rewritten to the new grammar), recorded per spec, no blanket claim.
