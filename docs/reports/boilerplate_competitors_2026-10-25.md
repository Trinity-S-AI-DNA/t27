# How comparable ecosystems cut boilerplate, and what t27 can copy (2026-10-25)

Scope: 10 ecosystems, read from their public docs on 2026-10-25. Marked unverified where a page
did not say it or only a search summary did. t27 baseline: 4584 hand `while (i < n)` loops, 1851
identical bodies in 692 groups, `(ptr, len)` parameter pairs, no macros/generics/derive, and a
Verilog backend that drops functions arriving through `use`. `specs/base/` holds 8 small modules and
none is a byte/char helper by name. `tools/dupe_scan.py` already ratchets duplicate bodies and has
`--name` / `--like` lookups, so t27 has a gate; what it lacks is a library to point at.

## 1. Chisel (chisel3.util)
- Mechanism: Scala embedded DSL; reuse is ordinary Scala functions, classes and factory objects (general knowledge, not on the page).
- Ships: `Queue` (FIFO with a factory), `Decoupled`/`Valid` (ready/valid bundles), `Arbiter`/`RRArbiter`, `Counter`, `MuxLookup`, `PriorityMux`, `Cat`/`Fill`, `ShiftRegister`, `Pipe`, `log2Ceil`.
- Discovery: Scaladoc API tree grouped by package, with search, per-item pages and a source link; deprecated items sit in their own section.
- Lesson: bundle the recurring signal group into one named type (`Decoupled`) and give every helper a factory call, so the user writes one line instead of the wiring. This is the cure for t27's `(ptr, len)` pairs.
- Source: https://www.chisel-lang.org/api/latest/chisel3/util/index.html

## 2. SpinalHDL (spinal.lib)
- Mechanism: Scala library like Chisel; standard conventions (Stream = valid/ready, Flow = valid only, Fragment = last-flagged payload) that every component accepts.
- Ships: `StreamFifo`/`StreamFifoCC` (there is no plain `Fifo` entry), `StreamArbiter`/`Fork`/`Join`/`Mux`, `BusSlaveFactory`, `RegIf`, state-machine and pipeline frameworks.
- `BusSlaveFactory` (sibling page of the same docs): one API (`read`, `write`, `readAndWrite`, `createReadWrite`) with one implementation per bus (APB3, AXI4, Wishbone, ...), so register maps do not change when the bus does.
- Discovery: library index grouped by purpose (Utils, Stream/Flow, Bus, Com, IO, Misc); the page itself says the package is under construction.
- Lesson: define a small shared convention first, then many components over it; one API with per-target implementations maps to t27's seven backends.
- Source: https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Libraries/index.html

## 3. Amaranth HDL (lib.wiring, lib.fifo, lib.data)
- Mechanism: Python library; declarative classes (`class RGB565(data.Struct): red: 5; green: 6; blue: 5`) replace hand slices like `pix[5:11]`. Details from the sibling pages stdlib/data.html and stdlib/wiring.html.
- Ships: `data.Struct`/`ArrayLayout`/`View` (offsets computed, never written), `wiring.Signature` + `connect()` (one port declaration shared by both ends), `fifo.SyncFIFO`/`AsyncFIFO`, `cdc.FFSynchronizer`, `crc`.
- Discovery: one stdlib chapter. Modules are sorted into three stability tiers (used by all idiomatic code; platform-dependent; one correct implementation), with a compatibility guarantee and an RFC process for changes.
- Lesson: name the fields and let the library compute offsets, and publish stability tiers so authors know what they may build on.
- Source: https://amaranth-lang.org/docs/amaranth/latest/stdlib.html

## 4. Bluespec (typeclasses, Prelude, Libraries)
- Mechanism: Haskell-style typeclasses with `deriving`; `deriving(Bits, Eq, Bounded, FShow)` generates `pack`/`unpack`, `==`, bounds and `$display` support from the type declaration (via search summaries of the reference guide; unverified on the primary PDF).
- Ships: `Prelude`, `Vector`, `FIFO`/`FIFOF`, `GetPut`, `Connectable` (`mkConnection` joins any two matching interfaces), `RegFile`, `Counter`; 43 files visible in `src/Libraries/Base1` (listing may be partial).
- Discovery: BSC Libraries Reference Guide (PDF built from the repo `doc` directory); Prelude documented in an appendix of the language guide.
- Lesson: each library file owns one interface, and generated instances (deriving) mean a new type is usable everywhere with zero hand code.
- Source: https://github.com/B-Lang-org/bsc

## 5. Veryl
- Mechanism: transpiler to SystemVerilog; the std library is Veryl source in a built-in `$std` namespace usable without a dependency entry (search summary of the book; unverified on the page). Parameters such as WIDTH, DEPTH, TYPE; `async_fifo::<S>` takes a generic module argument. Language-book feature list unverified (page returned 404).
- Ships (22 modules): `fifo`, `async_fifo`, `gray_encoder`/`gray_decoder`/`gray_counter`, `edge_detector`, `counter`, `delay`, `ecc_encoder`/`ecc_decoder`, `lfsr_galois`, `onehot`, `slicer`, `synchronizer_basic`.
- Discovery: generated site with one page per module (`std.veryl-lang.org/<module>.html`), a module list, and the namespace itself; the API may change until 1.0.
- Lesson: a std of 22 tiny parameterized modules, always in scope, is enough to retire most copy-paste; closest in spirit to what t27 can build.
- Source: https://std.veryl-lang.org/modules.html

## 6. Zig (std.mem, comptime generics)
- Mechanism: types are compile-time values, so one function serves all element types: `pub fn eql(comptime T: type, a: []const T, b: []const T) bool`.
- Ships in `std.mem`: `eql`, `findScalar`/`find`/`findLast` (older name `indexOf`), `count`, `trim`/`trimStart`/`trimEnd`, `allEqual`, `order`, `tokenizeScalar`/`splitScalar`; `std.ArrayList`.
- Discovery: generated std docs with keyboard search, versioned per compiler release; the source file is readable top to bottom. Names churn between versions (`indexOf` became `find` on master).
- Lesson: one verb per loop shape (equal, find, count, trim, tokenize) removes the hand loops; t27 lacks `comptime T`, so it must name the family per element type (`bytes_eq`, `u32s_eq`) and generate it.
- Source: https://raw.githubusercontent.com/ziglang/zig/master/lib/std/mem.zig

## 7. Rust (derive and macro_rules)
- Mechanism: `macro_rules!` matches syntax fragments (`$i:ident`, repetitions `$(...),*`) with hygiene; procedural macros come in function-like, derive and attribute kinds, and must live in a separate crate.
- Ships: `#[derive(Debug, Clone, PartialEq, Default, Hash)]` (the exact derivable list is general knowledge), `vec!`, `matches!`, `assert_eq!`, `format!`.
- Discovery: std docs with sidebar, search, an All Items index, the Prelude page, and Deprecated/Experimental labels; Clippy lints point at idioms (`match_same_arms`, `if_same_then_else`; via search).
- Lesson: derive means a type declaration alone triggers generated code, and a lint that names the better idiom teaches reuse at the moment of writing.
- Source: https://doc.rust-lang.org/reference/procedural-macros.html (macro_rules from the sibling page macros-by-example)

## 8. Dafny (Std library)
- Mechanism: modules with `import opened`, generics via type parameters, and a library shipped inside the compiler and switched on with `--standard-libraries`; it is pre-verified, so it adds no proof burden.
- Ships: `Std.Wrappers` (Option and failure-carrying values), `Std.Collections`, `Std.Math`, `Std.BoundedInts`, `Std.Strings`, `Std.JSON`, `Std.Parsers`, `Std.Unicode`, `Std.FileIO`.
- Discovery: directory in the repo, one flag, then one `import` line; no `include` path wrangling.
- Lesson: ship the library with the compiler, with its own tests/proofs already passing. Also note `--translate-standard-library`: library code is emitted into the translated output, which is exactly what t27's Verilog backend does not do for `use`.
- Source: https://github.com/dafny-lang/dafny/tree/master/Source/DafnyStandardLibraries

## 9. Lean 4 (Std/Batteries, tactics and macros)
- Mechanism: hygienic macros map new syntax onto old, elaborators for harder cases, notations in one declaration, and deriving handlers (BEq, Repr, Hashable, Ord, Inhabited, DecidableEq) registered by libraries (reference manual sections Notations and Macros, Deriving Instances).
- Ships: Batteries data structures and tactics; core `omega`, `exact?`/`apply?`, `bv_decide` (release notes via search); `Std.HashMap`.
- Discovery: `exact?` searches the environment from inside the proof; Loogle searches by type signature (Hoogle style); semantic engines (Moogle, LeanSearch); `lake lint` runs the environment linter.
- Duplicates: the one Mathlib linter file I read has `dupNamespace` only (repeated name parts); no duplicate-body linter found (unverified elsewhere).
- Lesson: discovery by type signature, available where you type, is what makes people reuse; linters ship with the library.
- Source: https://github.com/leanprover-community/batteries

## 10. Clash (and Filament)
- Mechanism: Haskell compiled to VHDL/Verilog/SystemVerilog; polymorphism, higher-order functions and type classes (`BitPack`, `Resize`, `Counter`, `Bundle`) do the sharing. Self-description: "datatypes and functions for circuit design" (clash-prelude 1.10.3).
- Ships in `Clash.Prelude`: `register`, `regEn`, `mealy`, `moore` (state machine from one pure function), `blockRam`, `rom`, `window`, `isRising`, sized `Vec` operations.
- Discovery: Hackage module pages with sectioned contents; a separate tutorial and user guide site.
- Filament (homepage also read): parametric modules (`Shift[W, N]`) are checked for all valid parameter values; its page describes no standard library (unverified).
- Lesson: a few state-machine combinators that take the transition as an argument replace every hand-written loop-plus-state skeleton; t27 would need first-class function arguments, or generation, to copy this.
- Source: https://hackage.haskell.org/package/clash-prelude

## Comparison table

| Ecosystem | Mechanism | Discovery | Duplicate-body gate? | Lesson |
|---|---|---|---|---|
| Chisel | Scala functions, factory objects | Scaladoc tree, search | none found | Bundle signal groups; one-line factories |
| SpinalHDL | Scala lib, Stream/Flow conventions | Index by purpose | none found | Shared convention, many components; one API per target |
| Amaranth | Python classes, annotations | Stdlib chapter, 3 stability tiers | none found | Name fields, compute offsets; publish tiers |
| Bluespec | Typeclasses, deriving | Reference guide PDF | none found | One interface per library file; derive instances |
| Veryl | Transpiler, parameterized std modules | Always-in-scope `$std`, page per module | none found | 22 small modules cover the common cases |
| Zig | comptime generics, `std.mem` | Versioned generated docs, search | none found | One verb per loop shape; names churn |
| Rust | macro_rules, derive, proc macros | std docs, prelude, Clippy hints | intra-function Clippy lints only; jscpd/PMD CPD external (via search) | Derive from declaration; lint names the idiom |
| Dafny | Modules, generics, bundled lib | One flag + one import | none found | Ship pre-verified; emit library in output |
| Lean 4 | Macros, deriving, tactics | `exact?`, Loogle, `lake lint` | linter framework exists; no duplicate-body linter found | Search by signature at write time |
| Clash | Haskell classes, higher-order | Hackage modules, tutorial | none found | Combinators taking the step function |

Honest reading: in the sources read, none of the ten has a cross-file duplicate-body ratchet like
`tools/dupe_scan.py`. They prevent duplication by making the shared version easy to find and cheaper
to use than copying. t27's gate is ahead; its library and lookup are behind.

## What t27 can copy without a compiler change
Constraint: until the Verilog `use` drop is fixed (compiler list, item 1), shared modules help the
Zig/Rust/C/JS backends only; hardware specs keep local copies or use item 6.
1. `std::mem` module (Zig std.mem): `bytes_eq`, `bytes_find`, `bytes_count`, `bytes_starts_with`, `bytes_trim`, plus `u32s_*` twins. Effect: retires the 22x `bytes_eq` and the index loops that implement find/eql/count.
2. `std::text` module (Veryl-style small modules): `is_digit`, `is_alpha`, `is_space`, `digit_value`. Effect: retires 36 hand-written `is_digit` definitions in 12 text variants (measured 2026-10-25 with `grep -rn -A3 -E '^(pub )?fn is_digit' specs`; the ratchet sees no group because parameter names differ); cheapest single win.
3. `std::signmag` module: one sign-magnitude add/sub/mul per word size (`sadd`, `magadd`, ...). Effect: collapses the 33x adder group and its `magsub`/`smul` siblings.
4. `tri find <name|signature>` (Loogle/Hoogle): extend `tools/dupe_scan.py --name/--like` to match parameter and return types and print the file:line to reuse; call it from the pre-write hook. Effect: new copies stop at write time.
5. `tri lint loop-shape` (Clippy style): recognize `while (i < n)` bodies that are find/eql/count/fold and print the `std::mem` call to use; ratchet the 4584 count down. Effect: loops stop regrowing.
6. `tri family` template expander (Zig comptime / macro_rules by text): a `.t27` template with `$T`/`$W` holes expands into checked-in sibling specs, with a freshness check like `check:onboarding`. Effect: gf4..gf64 and per-width helpers come from one source; also gives hardware specs a flattened copy that sidesteps the `use` drop (unverified).
7. `std::slice` module (Amaranth `Struct`, Chisel `Valid`): a `Bytes { base, len }` struct with accessors, replacing `(s, n)` pairs. Effect: halves argument lists; unverified that t27 structs can hold slice fields.
8. `tri derive` (Rust derive, Bluespec deriving, Lean deriving): from a struct/enum declaration generate `eq`, `default`, `is_<variant>`, pack/unpack into a generated `.t27` sibling. Effect: no hand-written comparison/pack code per type. Caveat: new `tri` logic must itself be `.t27` under the only-t27 rule (unverified that t27c can write files).

## What needs a compiler change
1. Emit `pub fn` declarations that arrive through `use` in the Verilog backend (Dafny's translate-the-library flag is the model). Unblocks every shared module for hardware.
2. Compile-time parameters (a width or element type as a parameter, monomorphized at use): Zig `comptime T`, Veryl `::<S>`, Filament `Shift[W, N]`. Replaces the family expander of item 6.
3. A first-class slice type carrying its length, plus a range loop (`for i in 0..n`) that lowers to the same code. Removes the `(ptr, len)` pair and the loop skeleton at the syntax level.
4. A declarative macro or `derive` attribute in the parser (Rust `macro_rules`/derive, Lean macros), only if the text-template tools of items 6 and 8 prove too clumsy.
5. First-class function arguments (named functions passed to `fold`/`find`/`any`) so Clash-style combinators and `std::iter` become possible. Unverified whether t27 already has them.
