# gHashTag/t27#8891 Rust Backend Bugs Verification Report

## Summary
This report documents the verification of 9 reported Rust backend compiler bugs in T27. All test files pass `t27c parse` and `t27c typecheck`, and most pass `t27c test-report` (R3 is BLOCKED on Zig side as expected). The generated Rust code exhibits all the problematic behaviors described in the issue.

## Test Files Created

### R1. `?Enum` return of an enum literal is not wrapped in `Some()`
- **File**: `r1_optional_enum_return.t27`
- **Status**: ✅ parse & typecheck pass, test-report passes
- **Generated Rust Issue**: `return E::Big;` should be `return Some(E::Big);`
- **File**: `r1_optional_enum_return.rs`

### R2. An enum literal outside a return is emitted unqualified
- **File**: `r2_enum_literal_unqualified.t27`
- **Status**: ✅ parse & typecheck pass, test-report passes
- **Generated Rust Issue**: `if (e == AtMax) {` should be `if (e == E::AtMax) {`
- **File**: `r2_enum_literal_unqualified.rs`

### R3. Payload enum variants produce garbage in both backends
- **File**: `r5_payload_enum.t27`
- **Status**: ❌ BLOCKED on Zig side (as expected)
- **Generated Rust Issue**: Enum has member named `u32`, return statement `return (Gap, x);`
- **File**: `r5_payload_enum.rs`

### R4. `[N][]const u8` becomes `Vec<const u8>`
- **File**: `r6_str_array.t27`
- **Status**: ✅ parse & typecheck pass, test-report passes
- **Generated Rust Issue**: `pub const NAMES: Vec<const u8> = ["a", "bb", "ccc"];` invalid syntax
- **File**: `r6_str_array.rs`

### R5. Anonymous struct return type
- **File**: `r7_anon_struct_return.t27`
- **Status**: ✅ parse & typecheck pass, test-report passes
- **Generated Rust Issue**: `pub fn check(x: u32) -> struct { ok : bool , code : u32 } {` invalid syntax
- **File**: `r7_anon_struct_return.rs`

### R6. Returning a `[]u8` slice
- **File**: `r8_return_slice.t27`
- **Status**: ✅ parse & typecheck pass, test-report passes
- **Generated Rust Issue**: Return type `Vec<u8>` but returns slice `&xs[(0) as usize..(n) as usize]`
- **File**: `r8_return_slice.rs`

### R7. `@log2` is emitted verbatim
- **File**: `g2f_log2_float.t27`
- **Status**: ✅ parse & typecheck pass, test-report passes
- **Generated Rust Issue**: `return @log2(v);` invalid syntax, should be proper Rust function call
- **File**: `g2f_log2_float.rs`

### R8. A usize value stored into a u64 struct field is not cast
- **File**: `g4_usize_into_u64_field.t27`
- **Status**: ✅ parse & typecheck pass, test-report passes
- **Generated Rust Issue**: `return R { n: xs.len() };` where `xs.len()` is `usize` but field is `u64`
- **File**: `g4_usize_into_u64_field.rs`

### R9. A parameter named `self` is kept
- **File**: `g8_self_param.t27`
- **Status**: ✅ parse & typecheck pass, test-report passes
- **Generated Rust Issue**: `pub fn counter_bump(self: &mut Counter, by: u64) -> u64 {` invalid outside impl block
- **File**: `g8_self_param.rs`

## Verification Results
- ✅ All 9 test files pass `t27c parse`
- ✅ All 9 test files pass `t27c typecheck`
- ✅ 8 out of 9 test files pass `t27c test-report` (R3 correctly BLOCKED)
- ✅ All generated Rust files exhibit the problematic behaviors described in the issue

## Notes
- Rust compiler (`rustc`) is not available in this environment, so compilation errors could not be directly verified
- However, the generated Rust code matches the expected problematic output described in the issue
- The compiler core is frozen, so these bugs require a decision from gHashTag rather than immediate fixes

## Generated Files
All test files and generated Rust code have been created and are ready for inspection and verification by the gHashTag team.