# gen-rust slice-capable backend — design note and test matrix

Status: design note (owner order 2026-11-05). No compiler change has landed yet.

## Goal

Extend the `t27c gen-rust` backend (bootstrap/src/compiler.rs, `gen_rust` at
~line 27176, helpers through ~29000) so it can lower the *decision* functions
the retry-suppression activation needs — `should_retry`-style predicates over
slices and callbacks — without a hand-written enum↔bytes adapter. That adapter
is the +19 foreign-line tail that blocks the activation PR under #8236
(shrink-only).

## Current supported subset (measured from the emitted Rust and compiler.rs)

The backend already lowers real function bodies; only tests/invariants and
body-less stubs are "declarations only". Already supported today:

- scalar types: `bool`, `u64`/`i32`/integer widths, `str` (`&'static str`),
  `?T`/`Option<T>`;
- arrays `[T; N]` and `Vec<T>` (read in `rust_type_of`, #8971/#11182);
- byte slices `[]const u8` → `&[u8]` (with string-literal `.as_bytes()` coercion);
- control flow: `while`, `if`/`else`, early `return`, expression statements;
- cross-function call lowering with a return-type fixpoint;
- `Copy`-type and enum-name pre-passes.

## Not yet supported (the capability to add)

| Construct | .t27 | target Rust | today |
|---|---|---|---|
| generic slice param | `xs: []const T` | `xs: &[T]` | only `&[u8]` concrete |
| mutable generic slice | `xs: []T` / out param | `xs: &mut [T]` | no |
| slice-of-bool return | `-> []bool` | `-> Vec<bool>` | no (fixed `[T; N]` only) |
| callback param | `f: fn(i32) -> i32` / closure | `f: impl Fn(i32) -> i32` | no |
| callback mutation | `f: fn(&mut T)` | `f: impl FnMut(&mut T)` | no |

## Lowering rules (target)

1. `[]const T` where `T` is a module type parameter lowers to `&[T]`; the
   function gains the `T` generic (reusing `collect_type_params`, already used
   for `ArrayView(T)`).
2. A return type `[]T` lowers to `Vec<T>`; a `[]bool` to `Vec<bool>`.
3. A bare function-typed parameter lowers to `impl Fn(A) -> R`; one that writes
   its argument lowers to `impl FnMut(A) -> R`. Callbacks are never `dyn` and
   never boxed: the generated Rust stays monomorphic.
4. `.len` on a generic slice keeps the existing `len_is_method` rule; `.len` on
   a `Vec`/`&[T]`/`&mut [T]` stays a method call.
5. Indexing `xs[i]` on a generic slice keeps the existing `ExprIndex` lowering;
   on a `Vec<bool>` it is an element read, not a bit.
6. Anything not lowered keeps today's behavior: an explicit diagnostic names
   the unsupported construct; it never silently emits wrong Rust (the existing
   "NOT LOWERED BY THIS BACKEND" policy for tests/invariants is preserved).

## Test matrix (to write before/with the implementation)

| # | Case | Expected |
|---|---|---|
| 1 | `fn first(xs: []const u8) -> u8` | already lowers `&[u8]`; regression pin |
| 2 | `fn count(xs: []const T) -> usize` (module type param `T`) | `&[T]` + `<T>` generic |
| 3 | `fn fill(xs: []T, v: T)` mutating | `&mut [T]` |
| 4 | `fn flags() -> []bool` | `Vec<bool>` |
| 5 | `fn map(xs: []const u8, f: fn(u8) -> u8) -> []u8` | `impl Fn(u8)->u8` + `Vec<u8>` |
| 6 | `fn for_each(xs: []T, f: fn(&mut T))` | `impl FnMut(&mut T)` |
| 7 | unsupported: a closure capturing a local | explicit diagnostic, no silent wrong code |
| 8 | regression: existing specs regenerate byte-identically | no output drift |

## Acceptance

- `t27c gen-rust` of a fixture spec compiles (`rustc`/`cargo`) and the emitted
  functions behave identically to the Zig/`t27b test` oracle on the same inputs.
- No change to the reference-cache format, `verdict_key`, or `cache_format`.
- Existing generated outputs are byte-identical (regression gate).
- Unsupported constructs stay explicitly rejected.

## Unblocks

Once slices+callbacks lower, the activation glue's adapter can be regenerated
instead of hand-written, the hand-written Rust shrinks, and the activation PR
can be rebased to `net <= 0` without a risky Reference-enum migration.
