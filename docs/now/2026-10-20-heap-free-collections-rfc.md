# NOW -- RFC: heap-free collections, owned strings and inlined closures in t27 (2026-10-20)

## docs/now/2026-10-20-heap-free-collections-rfc.md (Closes #11177; a document, no compiler, spec, ledger, seal or Rust is changed)

- Question. Five measurements of 2026-10-19/20 (#11135, #11145, #11149, #11159, #11168) say the share of foreign code that can
  be converted is bounded by what t27 cannot say, not by what the converter has not learned. This RFC asks which of those things
  t27 should learn (lists with a stated capacity, owned strings, closures), at what cost, for what measured demand, and whether
  the answer is "none". It is the document asked for by #11177 and follows its template: measured demand, at least three
  options each with the t27 source, the four back ends, the demand it unlocks and its cost, a recommendation, one experiment
  that needs no compiler change, and the open questions with the default taken.
- Base. master a26cafbb6 (t27). The sweep numbers are lane A4's saved per-file outputs (1,388 tracked `.rs` files of the five
  repositories, 19,392 fns seen), the same sweep #11135, #11145, #11149 and #11159 used. The experiment ran with `t27c 0.5.2`
  (built 2026-10-19 13:04 local), `t27b` (2026-10-19 16:23) and the `tri` of master 16c7dcf13; none of them was rebuilt for this
  document (no `cargo build` on the shared Mac).
- Reading order. Summary, then section 3 (the recommendation and its arithmetic), then section 4 (the experiment). Section 1
  is the evidence, section 2 the options, section 5 the open questions with defaults, section 6 what was found on the way and what
  was not run.

## Summary

1. Demand, measured. Of the 16,057 Rust fns that stop at a first refusal, the families the options below address are lists
   (2,168 direct, 2,813 once a cascade is followed to its root), owned strings (922 / 1,655) and iterator chains with closures
   (419 / 588). Three more families that no option here addresses are of the same order: tuples (854 / 879), paths and I/O
   (1,021 / 1,091) and maps and sets (252 / 1,269). Python has no front end; its heap-blocked pure code is 6,004 distinct lines
   (1.8% of 331,183), 2,075 of them blocked by owned strings alone.
2. What t27 says today. A list with a stated capacity, a caller-sized buffer and an owned string as buffer plus count already
   work in the Zig back end and in `t27b`, as hand-written library code (experiment X1: 9 tests, 9 pass under `t27c` and `t27b`,
   0 vacuous, 9 of 9 mutants killed). Generics and closures do not exist. The real gap is the other back ends: `gen-c` writes
   a slice parameter as a bare pointer and loses its length, `gen-rust` writes a slice struct field as `Vec<T>`, `gen-verilog`
   leaves pointer parameters unbound. Those are defects of the existing out-buffer forms (#9098, #11109), independent of any new
   construct.
3. Options. (a) `Buf(T)`, a caller-sized buffer as a language form; (b) `Vec(T, N)`, fixed capacity in the type; (c) an arena
   passed by the caller; (d) closure literals inlined at their single call site; (e) owned strings as buffer plus count with a
   small std; (f) none. Each is costed against two measured precedents of a backend-visible change (#10812: 49 files, 42 seals;
   #9135: 290 files, 286 seals) and the owner's rule that a pull request may add no more hand-written foreign lines than it
   deletes (40 per file, 80 per diff, no label lifts it).
4. Recommendation: none of (a)-(e) as a language construct now; option (f) with three steps that change no language: fix the
   back-end defects X1 found, keep the buffer forms as one shared library (extend, do not copy), and run the sized pilot X2
   before any construct is built. The arithmetic: at the end-to-end rates measured so far (0.6% to 1.3%) the three-family
   package (lists, strings, iterator chains) converts 30 to 64 more Rust fns (0.15% to 0.33% of those seen) and deletes 15 to 139
   hand lines (0.003% to 0.03% of the swept lines); at four times the best measured rate, 253 fns and at most 550 lines. The golden
   track has already found 4,381 lines (1.0%). Deleting 1,000 lines needs 9.1% of the package to convert, 7 times the best rate
   measured. The bar for building anything is stated: a pilot rate of at least 15 in 100.
5. Experiment X1 (no compiler change). Four functions taken from the censuses (three Rust, one Python, with its sibling
   `decode_wbstar`) written by hand in forms (a), (b), (c), (d) and (e): `t27c test-report` 9/9, vacuous 0 of 9; `t27b test`
   9/9 in under 20 ms; generated Rust
   compiles and agrees with the original on every vector; generated C does not compile (12 errors, 7 from slice parameters);
   generated Verilog does not elaborate (20 errors). The module is pasted in section 4.
6. Defaults taken, not asked (section 5): C is gated on slices becoming (pointer, length) pairs; Verilog refuses these forms;
   an element past the capacity is counted and not written (the rule of #11073); no generics, one library copy per element type
   and capacity; Python is conditional demand and is not in the arithmetic.

## 1. Problem and measured demand

### 1.0 The demand in one table

Rust: fns whose first refusal is in the family, by repository (direct; Tables 1 and 2 of section 1.2 have the root-attributed
counts, the tokens and the percentages). Whole-file bound: files that would convert whole if only that family vanished (Table 3).
Python: distinct lines of pure functions blocked only by that feature, all five repositories together (section 1.3; the split by
repository of this decomposition was lost, see there).

| missing feature | Rust t27 | trios | trinity | trinity-fpga | 999-multibots | Rust all: direct / root | Rust whole-file bound | Python lines, only this |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| lists (`Vec`, `vec!`, `push`) | 762 | 1,341 | 18 | 24 | 23 | 2,168 / 2,813 | 5 files / 802 lines | none counted (580 lines of S append) |
| owned strings | 701 | 191 | 0 | 4 | 26 | 922 / 1,655 | 4 / 628 | 2,075 |
| closures over iterators | 257 | 131 | 8 | 8 | 15 | 419 / 588 | 3 / 299 | 477 |
| maps and sets | 61 | 176 | 4 | 4 | 7 | 252 / 1,269 | 0 / 0 | 649 |
| tuples | 695 | 154 | 0 | 0 | 5 | 854 / 879 | 6 / 688 | none counted (1,327 lines of S pass tuples) |
| paths and I/O | 842 | 158 | 2 | 19 | 0 | 1,021 / 1,091 | not computed | not a heap feature |
| all first refusals (fns) | 8,694 | 6,568 | 164 | 238 | 393 | 16,057 | | 1,293 fns / 16,577 lines pure but blocked (5.0%); 495 fns / 6,004 lines (1.8%) by the four heap categories alone |

Fns seen / converted: t27 9,735 / 796, trios 8,220 / 969, trinity 182 / 3, trinity-fpga 273 / 18, 999-multibots-telegraf 982 / 38,
all 19,392 / 1,824 (9.41%). The table has no cascade column: 4,468 of the 16,057 are `uses refused` and are counted under their
roots in the root column.

### 1.1 What t27 can say today (verified, not assumed)

Each row was run on the binaries named above; "X1" is the module of section 4, "stack" is `specs/tri/collections/stack.t27`.

| thing | Zig (`t27c test-report`) and `t27b` | `gen-rust` | `gen-c` | `gen-verilog` |
| --- | --- | --- | --- | --- |
| fixed array `[N]T`, repeat `[0] ** N` | works | `[T; N]` | `T x[N]`; a non-zero repeat came out as `{0}` with this binary (D6) | not measured alone; a struct holding one is a packed vector (544 bits for 8 x u64 and a count) |
| slice parameter `[]T`, `[]const T`, `.len` | works | `&[T]`, `&mut [T]`; `.len` is left unconverted when the module declares a struct field named `len` (D1) | `T*` and the length is gone; `x.len` is invalid C (D4) | length unbound |
| slice as a struct field | works (`stack` 11/11 in both) | `Vec<T>`, the struct takes the caller's storage by value (D2; 2 rustc type errors on `stack`) | `T*`, 7 `cc` errors on `stack` | not measured |
| struct with an array field and a count, by value, and `*Struct` parameters | works (X1) | works (X1) | works for the struct; slice parameters of the same module do not | the pointer parameter's fields are unbound (D7) |
| `?T`, unit enums, `[N]str` constants | works | works; a `[N]str` element passed as `[]const u8` mismatches (D3) | works (a duplicate `const` warning on the `str` table) | not measured |
| generics `Name(T)` | no: `Ring(T)` in `specs/tri/collections/ring_buffer.t27` stops `t27c` with "use of undeclared identifier 'T'"; `t27b` reports an unsupported construct | - | - | - |
| closure literal `\|y\| y + 1` | no: parse error "Unexpected token in expression: Pipe" in `t27c parse` and `t27b` | - | - | - |
| heap | none. The converter says so in its header: "t27 holds no growing list" (rust_convert.t27) and "t27 has no heap text" (#9098). 77 specs name `alloc`, `create_buffer` or `ArrayList`; the two checked are stubs that `t27b` rejects (`allocator.alloc` in `specs/tri/collections/interval.t27`, `create_buffer` in `ring_buffer.t27`) | - | - | - |

The existing building blocks, in the converter's own words (header of `specs/tools/t27o/rust_convert.t27`):

- the String out-buffer form (#9098): a fn whose result is an owned String, built only by appends, takes a last `out: []u8`
  and returns the byte count;
- the Vec out-buffer form (#11073, merged as #11109): a fn returning `Vec<T>` (T integer, float or bool) built by `push` and
  `extend_from_slice` takes a last `out: []T` and returns the element count; "the capacity is the caller's and is never
  guessed: an element past `out.len` is counted and not written".

The corpus has already grown the same idea by hand, without a name. `specs/tri/collections/` holds 28 specs (4,689 lines);
`t27b test` passes 14 of them (178 tests) and fails 14 ("undeclared fn", "rejected fn", `allocator.alloc`). 13 of the 14 that
pass keep the caller's storage in a slice field of a struct and are typed for one element (`items : []i64` in `stack`, `data :
[]i64` in `deque` and `priority_queue`, `bits : []u64` in `bitmap`, `items : []const u8` in `array`, a node struct in
`linked_list`): the shape of option (a), one copy per element type, and the shape that breaks `gen-rust` and `gen-c` (D2, D4).

### 1.2 Rust demand

Sweep. The converter of `gen/rust/tools/t27o/rust_convert.rs` (lane A4's build, the one that measured #11109; lanes A5 and A6
re-ran the sweep with master ca90be110 and got the same totals), built with `rustc` into a driver that makes the calls
`cmd_t27o` makes, one process per file, 60 s cap; lane A4's `sweep_all.zsh` over its pinned copy of the five
repositories (t27 7496ed220, trios 483dd9ee, trinity 0770abb9, trinity-fpga 20bf4c418, 999-multibots-telegraf 6e5512881;
`gen/` and `target/` left out; the five files over 300 KB included). Totals: 1,388 files, 19,392 fns seen, 1,824 converted
(9.41%), 16,057 first refusals, 85 files convert whole. These are the totals of #11135, #11145, #11149 and #11159.

The refusal lines were read back with a scratch reader (not committed): the line `// t27o: refused <item> <name> (line N):
<reason>` gives the kind (`type`, `method`, `macro`, `tuple`, `path`, `closure`, `uses refused` ...) and its token; a cascade
(`uses refused X`) is followed to the first refusal line of the item named X in the same file, at most 64 hops
(`root_of` of `specs/tools/t27o/ledger.t27`, #11127). The reader reproduces the published counts: `type Vec` 1,289,
`Path` 435, `String` 232, `PathBuf` 230, `std` 212, `anyhow` 144 and root-attributed `type` 6,180 exactly, root-attributed `method`
as 3,325 against the 3,323 that the pull request of #11127 prints.

A caveat about the headline share. "`type` 38.5%" is 6,180 divided by 16,057, where 6,180 counts every refused item (fn, struct,
enum, const) and 16,057 counts fns. Counting fns alone, `type` is 3,049 direct first refusals (19.0%) and 4,949 once cascades
are followed to their roots (30.8%). `method` is exact: 3,325 root-attributed, 20.7%. The table below counts fns only.

Table 1. Rust fns by the family of their first refusal, fn level. Direct = the fn's own first refusal; root = a cascade
(`uses refused`) followed to its root; 696 cascades cannot be followed (656 of them are `self` or `Self`; #11130 names the type).

| family | direct | share of 16,057 | root | share | largest direct tokens |
| --- | ---: | ---: | ---: | ---: | --- |
| lists (`Vec`) | 2,168 | 13.5% | 2,813 | 17.5% | type `Vec` 1,289; macro `vec` 552; method `push` 164; path `Vec::new` 85; `insert` 34; `remove` 16 |
| owned strings | 922 | 5.7% | 1,655 | 10.3% | method `to_string` 235; type `String` 232; "an owned String outside the out-buffer form" 125; macro `format` 89; `lines` 67; path `String::new` 66 |
| iterator chains and closures | 419 | 2.6% | 588 | 3.7% | method `iter` 246; closure 45; `for` over an iterator 30; `iter_mut` 27; 71 others (`map`, `windows`, `find` ..) |
| maps and sets | 252 | 1.6% | 1,269 | 7.9% | type `BTreeSet` 127, path `BTreeSet::new` 43; `HashMap::new` 20; `BTreeMap` 17; `HashSet` 16; `HashMap` 13 |
| tuples | 854 | 5.3% | 879 | 5.5% | `(` 594; `String` 65; `&` 39; `usize` 19; `Vec` 15 |
| paths and I/O | 1,021 | 6.4% | 1,091 | 6.8% | `Path` 435; `PathBuf` 230; `std` 212; `anyhow` 144 |
| everything else | 5,953 | 37.1% | 7,066 | 44.0% | `join` and `into` (ambiguous), `unwrap`, `contains`, `new`, enum variants with data, macros .. |
| cascade, not followed | 4,468 | 27.8% | 696 | 4.3% | the 4,468 direct cascades are counted under their roots in the other rows |
| total | 16,057 | | 16,057 | | |

Notes on the families: the brief for this RFC lists "tuples 650"; no cited source has that number, the saved sweep has 854
direct (594 at the token `(`), so the table uses the measured figure. `join` (426 root-attributed) is not placed in any family
because it is `Path::join` as often as it is `[String]::join`; `push` is placed with lists although a `String` has one too.

Table 2. The same by repository, direct first refusals (the fns column is fns seen).

| repository | files | fns | converted | refused | lists | strings | iter/closure | maps/sets | tuples | paths/IO | else | cascade | whole files |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| t27 | 382 | 9,735 | 796 | 8,694 | 762 | 701 | 257 | 61 | 695 | 842 | 3,432 | 1,944 | 1 |
| trios | 786 | 8,220 | 969 | 6,568 | 1,341 | 191 | 131 | 176 | 154 | 158 | 2,043 | 2,374 | 83 |
| trinity | 23 | 182 | 3 | 164 | 18 | 0 | 8 | 4 | 0 | 2 | 103 | 29 | 0 |
| trinity-fpga | 30 | 273 | 18 | 238 | 24 | 4 | 8 | 4 | 0 | 19 | 140 | 39 | 0 |
| 999-multibots-telegraf | 167 | 982 | 38 | 393 | 23 | 26 | 15 | 7 | 5 | 0 | 235 | 82 | 1 |
| all | 1,388 | 19,392 | 1,824 | 16,057 | 2,168 | 922 | 419 | 252 | 854 | 1,021 | 5,953 | 4,468 | 85 |

(In t27 paths and I/O (842) and lists (762) lead, then strings (701) and tuples (695); in trios, 786 files of small crates, lists (1,341) dominate.) The
real command on this checkout, `tri t27o-sweep --max-bytes 100000000 --cache <file> --top 14` at master 133dac338, prints for
t27 alone 390 files, 9,671 fns seen, 753 converted, ranked kinds `type` 3,217, `method` 1,994, `tuple` 732, and 0 files that
convert whole (the golden port #11157 took the one that did).

Where the refused `Vec` is (#11135, lane A5), of the 1,289 `type Vec` first refusals: a parameter 87 (6.7%), a return type 674
(52.3%), a local 528 (41.0%). Of the 528 locals, 496 are `let x: Vec<T> = ..collect()` (435 with a closure in the chain, 451
inside test fns) and 27 start empty. Of the 87 parameters, 71 are `&mut Vec<T>` and 0 of the 221 fns with a Vec parameter convert;
the read-only form already converts to a slice (#8999). The Vec out-buffer form (#11109) reaches 4 fns: of 626 fns returning a bare
`Vec<T>` the tail is a `vec!` literal in 251, a `collect` in 151 (135 + 16), push-only in 91 (element: 19 scalar, 32 struct or
enum, 21 tuple, 15 text), and 4 convert (`from_bits`, `to_bytes`, `uniform_lengths`, `regex_lite_global`).

Where iterator chains stop (#11145, lane A6): 2,697 fns hold a chain; 676 are lowerable in place; 250 were lowered to a counted
loop; **17 convert end to end** (7 `any`/`all`, 10 `collect` into a returned `Vec` of scalars), 23 with `find` and `position`,
26 and 32 counting the fns that call them. 64% of the closures (1,870 of 2,928 chains; 1,516 of them over the std table alone) are one pure
expression; that is not the wall. The wall is what follows: of the 233 lowered fns that did not convert, the next refusal is `uses refused`
70, a method outside the std table 48, an enum variant or fn of another module 25, `String`/`Path`/a collection/a tuple 17,
`vec!` 14. The largest group left, 353 fns (286 of them tests), collect a lowerable chain into a LOCAL list: 212 over a
range (82 with literal bounds), 141 over a slice of run-time length. No `sum`, `count` or `fold` fn converted.

Table 3. What would convert whole. For each set of families, the files whose every refusal (fn, struct, enum, const, each
cascade followed to its root) is in the set: the files that would convert whole if those first refusals vanished and nothing
else were hidden behind them. An upper bound: the converter stops at the first refusal of a fn, so a second refusal is
invisible. Lines are physical lines of the file, tests and scaffolds included.

| set of families | files | refused fns in them | physical lines |
| --- | ---: | ---: | ---: |
| lists | 5 | 33 | 802 |
| iterator chains and closures | 3 | 13 | 299 |
| owned strings | 4 | 5 | 628 |
| tuples | 6 | 21 | 688 |
| lists + iterators | 16 | 141 | 2,802 |
| lists + iterators + strings | 23 | 162 | 3,993 |
| + maps and sets | 41 | 384 | 7,808 |
| + tuples | 54 | 449 | 10,098 |
| for comparison: the 85 files that convert whole today | 85 | | 3,006 (80 ranked rows, 0.69% of 435,491) |

The families are complements, not substitutes: lists alone 5 files, iterators alone 3, together 16.

What a converted fn is worth in deleted hand lines (#11149, #11159): 223 partly converting files hold 817 converted fns with
6,555 hand lines outside `#[cfg(test)]` (8.0 per fn, 44,864 more in `cfg(test)`); 146 fns / 1,375 lines (21.0%) move with no
caller change, 54 fns / 406 lines (6.2%) with one; whole files add 3,006 lines. Together 4,381 lines, about 1.0% of the 435,491
swept. An out-buffer or struct-returning form changes the signature of every fn it touches, so it lands in the second class at
best.

### 1.3 Python demand

Source: #11168 and `docs/now/2026-10-20-python-debt.md` (lane A9; `ast`, no regex; t27 0e00974da, trios 78671ba794, trinity
8e535bc9bb, trinity-fpga c9b180bf78, 999-multibots-telegraf fcdbb2f7dc). 2,126 tracked `.py` files hold 434,986 gate-counted
lines; after 23 generated, 22 vendored and 138 test-only files the census holds 1,943 files, 12,547 functions; counting
identical contents once, 331,183 lines and 8,501 distinct function bodies.

| subset | distinct fns | lines | share of 331,183 |
| --- | ---: | ---: | ---: |
| fits the typed subset today (T) | 105 | 812 | 0.25% |
| fits once parameter types are inferred (S, #9006) | 454 | 4,090 | 1.23% |
| of S: uses list growth, tuple values or an index write (features t27 cannot say, or says only as in section 1.1) | 179 | 1,643 | 0.50% |
| of S: uses none of those | 275 | 2,447 | 0.74% |
| pure, blocked only by owned strings, dict/set, generator expressions, optional returns, pure stdlib imports, module state, methods outside the table .. (the "5.0%" of #11168) | 1,293 | 16,577 | 5.0% |
| of those: blocked ONLY by the four heap categories below | 495 | 6,004 | 1.81% |
| of those: also need something that is not a heap feature (methods outside the table 429, pure stdlib import 297, module state 274, return/param type 172/152, lambda 55) | 798 | 10,573 | 3.19% |

The four heap categories, fns and lines whose only blockers are in the set (derived here from lane A9's per-function
classification, `cats`, not in the repository; the single-category rows are also in the "only blocker" table of the document):

| only blocked by | fns | lines | share |
| --- | ---: | ---: | ---: |
| owned strings | 166 | 2,075 | 0.63% |
| dict / set | 76 | 649 | 0.20% |
| generator expressions (closures over an iterator) | 64 | 477 | 0.14% |
| optional return (`?T` exists; not a heap feature) | 25 | 355 | 0.11% |
| owned strings + dict/set | 71 | 1,034 | 0.31% |
| owned strings + generator expressions | 49 | 674 | 0.20% |
| dict/set + generator expressions | 14 | 171 | 0.05% |
| owned strings + dict/set + generator expressions | 12 | 356 | 0.11% |
| every combination that includes optional return | 18 | 213 | 0.06% |
| total | 495 | 6,004 | 1.81% |

So: strings alone 2,075 lines; strings and closures 3,226; adding dict/set 5,436; adding optional return 6,004. By repository
the document gives T and S only, raw and not de-duplicated (T / S fns: t27 29 / 163, trinity 86 / 133, trinity-fpga 114 / 352,
trios 0 / 1, 999-multibots-telegraf 1 / 5); the split of the heap decomposition by repository was not possible here because lane A9's scratch was removed while this
lane ran (the aggregate above was computed before that).

Two cautions. First, the Python lines are conditional demand: the verdict on #8995 and #9006 is "no Python front end now", and
`t27c gen-python` lowers no function (it announces a `fn` in a comment), so a ported function is called through a hand-written
loader. Second, even the 1.23% presumes list growth and tuples: 77 S fns (580 lines) append to a list and 129 (1,327 lines) pass
tuples.

### 1.4 Demand inside t27 itself

New t27 code already pays for the missing forms. At base 133dac338, by `grep` over 2,481 `.t27` files (approximate, patterns in
section 6): 96 files declare a parameter `out : []u8`; 206 have a struct field of array type; 248 have a slice-typed field; 883
have a slice anywhere. The byte-appending helper is written again and again: `t27c dupes --name` finds `put` in 18 places,
`put_dec` 5, `put_byte` 5, `put_u64` 4, `put_num` 3: 35 definitions under five names, 26 of which take a byte buffer (`out : []u8`,
`dst`, `buf` or `*Out`), in policy, widgets, the t27b emitter and t27o; the other nine are unrelated (the `put` of an LRU cache ..). The
28 collection specs of section 1.1 are one copy per element type, and the generic ones (11 files with `Name(T)`) do not compile.
This demand needs no foreign code and no converter: it is the cost of writing in t27 today.

### 1.5 Reproducing the numbers

```
# Rust, per checkout (counts, coverage, ranked first-refusal kinds with root attribution, files that convert whole)
tri t27o-sweep --repo <checkout> --max-bytes 100000000 --cache <scratch-file> --top 14
# Rust, five repositories: lane A4's saved per-file outputs (`<converter> convert <file>` per tracked .rs, 60 s cap) read back
# by the reader described in 1.2; the rules of the reader are root_of / quoted_refusal in specs/tools/t27o/ledger.t27
# Python: the scratch scripts of #11168 (ast + tokenize, python3 -I); single-category rows are in its "only blocker" table
# Corpus (section 1.1, 1.4):
t27b test specs/tri/collections/<name>.t27 --quiet          # 28 files: 14 pass, 178 tests
t27c test-report specs/tri/collections/ring_buffer.t27       # BLOCKED: use of undeclared identifier 'T'
t27c dupes --repo-root . --name put                          # defined in 18 places
```

## 2. Options

Letters follow #11177. Every option is described the same way: the t27 source it adds, what each back end emits (Zig, Rust,
C, Verilog; "measured" means X1 or a corpus spec was run on it, "expected" means it is a design that does not exist), the
demand it unlocks (section 1 numbers), and its cost.

How cost is counted. Four things move when the compiler changes, and the repository has measured examples of each:

- `bootstrap/src/compiler.rs` (46,515 hand lines; the back ends are `gen_zig`, `RustCodegen`, `CCodegen`, `VerilogCodegen`)
  and `cli/t27b` (14,156 hand lines; every PR under it is net <= 0, #6198 and `.claude/skills/t27b-loop/SKILL.md`). The own-language budget holds for every pull request: at most
  40 hand-written foreign lines added per file and 80 per diff, never more added than deleted, and no label lifts it
  (`specs/policy/own_language.t27`). New logic therefore goes into `specs/compiler/<rule>.t27` and is mounted with `#[path]`
  (10 modules in `bootstrap/gen/rust/compiler/`), and the glue is paid for by deletions in the same pull request: the recent
  `compiler.rs` edits are +1/-1, +26/-54, +31/-48, +39/-75, +40/-47.
- The seal. Editing `compiler.rs` moves `bootstrap/stage0/FROZEN_HASH` (one line, CONTRIBUTING.md). A spec whose generated
  output changes needs `seal --save`: 2,939 seal files exist.
- `gen/`: 74 Rust, 26 C, 1 Verilog and 1 JSON file under `gen/` and 46 under `bootstrap/gen/`, regenerated only where output
  changes.
- The ledger: `docs/reports/t27b_expectations.json` rows (the steward's) and, for the converter, the t27o sweep ledger
  (`specs/tools/t27o/ledger.t27`, unaffected unless a refusal kind appears or goes).

Three sizes, each with a precedent. S: the converter or a library only, no `compiler.rs`: #11109 (the Vec out-buffer form) is
one spec, its generated Rust and one seal. M: one cross-back-end rule: #10812 (a string escape lowered by every back end) is 49
files, 42 seals, `compiler.rs` +31/-48, a new 87-line rule spec, `FROZEN_HASH`, one `t27b` ledger row, 529 insertions and 318
deletions. L: several constructs across the back ends: #9135 (eight gen-rust constructs) is 290 files, 286 seals, `compiler.rs`
+39/-75, 1,216 insertions and 980 deletions.

### Option (a): `Buf(T)`, a caller-sized buffer as a language form

Source added (a built-in generic type; the language has none today):

```
pub const Buf(T) = struct { room : []T, count : usize };    ; count = how many the callee wanted

pub fn sample_words(n: usize, out: *Buf(u64)) -> void {
    var i : usize = 0;
    while (i < n) { buf_push(out, (i as u64) * 0x0100010001000101); i = i + 1; }  ; past room: counted, not written
}
; caller:  var store : [64]u64 = [0] ** 64;  var b = buf_over(&store);  sample_words(65, &b);
;          b.count == 65;  buf_fits(b) == false;  store holds the first 64
```

Today the same thing is `out: []u64` plus a returned `usize` (X1 `sample_words_out`; the form #11109 emits). The type adds the
pairing of room and count and a name for "did it fit".

| back end | what it emits | status |
| --- | --- | --- |
| Zig, `t27b` | `struct { room: []u64, count: usize }` | works now (`stack`: 11/11 in both) |
| Rust | `struct BufU64<'a> { room: &'a mut [u64], count: usize }`, a lifetime on the struct and on every fn that takes one | expected; today `gen-rust` writes `Vec<u64>` by value (D2), the largest back-end change in this RFC |
| C | `struct { uint64_t* room; size_t cap; size_t count; }` and every slice parameter becomes a (pointer, length) pair | expected; today the length is lost (D4); the pair changes the C signature of the slice fns in 18 of the 26 `gen/c` files and the ctypes loader `scripts/tri_loop/t27b_rules.py` that calls generated C |
| Verilog | cannot: a slice into the caller's memory is a pointer; RTL has memories with ports | refuse, with a message naming the form |

Unlocks. Rust: the lists that are returns or parameters, 761 of the 1,289 `type Vec` first refusals (674 + 87); 91 of 626
bare-`Vec` returns are push-only. Measured end to end: 4 fns (#11109) and a ceiling of 1 for `&mut Vec` parameters (#11135).
Whole files: lists alone 5 files, 802 lines (upper bound, Table 3). Python: none counted.

Cost: M to L. The first built-in generic; parser, type checker and four back ends. The Rust lifetime plumbing and the C pair are
each an M by the precedent of #10812. `FROZEN_HASH` moves; seals move for every spec with a slice field (248 by grep) or a slice
parameter in a C-bound spec; ledger rows. Converter: `rust_convert.t27` maps its existing out-buffer forms onto the type
(spec only).

### Option (b): `Vec(T, N)`, fixed capacity written in the source

Source added:

```
pub fn sample_words(n: usize) -> Vec(u64, 64) {
    var v : Vec(u64, 64) = vec_new();
    var i : usize = 0;
    while (i < n) { vec_push(&v, (i as u64) * 0x0100010001000101); i = i + 1; }
    return v;
}
```

A value that holds N elements of storage and a count; `vec_push` past N counts and does not store (the rule of #11073);
`vec_over(v)` says it did not fit. Nothing traps and nothing is undefined, so there is no hidden bound. The converter takes N
from the source: `with_capacity(K)` with a literal or a const, a range `(0..K)` with a literal or const K, an array length, a
`[T; K]` parameter; if it cannot read a bound it refuses with "no stated bound" (#11073: "a list that grows without a bound
the caller states has no t27 form"). Today, by hand (X1): `pub const ListU64 = struct { data : [64]u64, count : usize };` with
`list_new`, `list_push`, `list_fits`: 10 lines per element type and capacity, because there are no generics.

| back end | what it emits | status |
| --- | --- | --- |
| Zig, `t27b` | a struct with an array field | measured (X1) |
| Rust | `#[derive(Debug, Clone, Copy)] pub struct ListU64 { pub data: [u64; 64], pub count: usize }` | measured: compiles, the generated functions agree with the originals on every vector; 512 bytes are copied on each return, so a large N wants `&mut` |
| C | `struct ListU64 { uint64_t data[64]; size_t count; }`, by value | measured: compiles; a return by value copies N elements (`specs/compiler/c_fixed_array_return.t27`); a helper that takes a slice still needs D4 |
| Verilog | a packed vector (544 bits for 8 elements and a count) | measured for the declaration; `vec_push` through `*Vec` is unbound (D7); a value-in, value-out form might lower (not measured) |

Unlocks. Rust: the 528 locals (41% of `type Vec`; 496 are `collect`), and the 353 fns of #11145 that collect a lowerable chain
into a local list: 212 over a range (82 with literal bounds, the rest constants or variables) and 141 over a slice of run-time
length, which have no bound and stay refused; 286 of the 353 are tests. Whole files: lists alone 5 files / 802 lines, with
iterators 16 files / 2,802 lines (Table 3). Python: none counted; it protects the 77 S fns (580 lines) that append.

Cost: S as a library (X1, no compiler change). As a construct the feature is not the list but instantiation: a type
constructor with a value parameter is const generics in all but name, so it is L. Back-end emission is already there for the
struct-with-array shape; the only new emission is the C by-value return. No existing seal moves for specs that do not use it.
Converter: emit the instance for a bounded local (spec only).

### Option (c): an arena (region) passed by the caller

Source (X1, library form): `pub const Arena = struct { mem : [256]u8, used : usize };` with `arena_alloc(&a, n)` returning an
offset or `ARENA_FULL`, and `arena_set32`, `arena_get32` as byte arithmetic. An allocation is an offset, never a pointer. A
language form would add typed views (`arena.slice(u32, n)`, a slice of the region) and so needs a cast from bytes to `T`.

| back end | what it emits | status |
| --- | --- | --- |
| Zig, `t27b` | an array and offsets | measured (X1: 10 asserts, incl. exhaustion at the boundary) |
| Rust | the same, safe; a typed view needs `unsafe` or a byte-cast crate | library measured; view expected to be refused |
| C | an array and offsets compile (measured); a typed view is a cast with alignment caveats | library measured |
| Verilog | a memory with ports; the offset arithmetic is unbound through `*Arena` (D7) | refuse |

Unlocks: nothing measurable. No census separates the fns that need two growing locals of different element types (A5 counted 27
empty-start locals; the Vec form refuses a second Vec local). Python: none.

Cost: S as a library; the typed-view construct is L (a cast and an aliasing rule in four back ends).

### Option (d): closure literals inlined at their single call site

Source added:

```
pub fn path_delay(arcs: []const TimingArc) -> u32 { return sum(arcs, |a| a.delay_ps); }
pub fn count_nonzero(v: []const Ternary) -> usize { return count(v, |t| t != Ternary.Zero); }
pub fn sample_words(n: usize) -> ListU64 {
    var out = list_new();
    fill(&out, 0..n, |i| (i as u64) * 0x0100010001000101);
    return out;
}
```

`|p| expr` is legal only as the last argument of a fusing built-in (`sum`, `count`, `any`, `all`, `find`, `min_by`, `max_by`,
`fill`). The body is one pure expression over the parameter and read-only names in scope: no assignment, no `return`, no call
except a pure fn of the module. It has no value of its own: the front end substitutes the body for the parameter inside the
loop of the built-in (the loops of X1), and every back end sees `while`. Passing, storing or returning a closure is an error.
Today it is a parse error (section 1.1). The same effect with no language change: the converter lowers the Rust chain to the
loop, which is what lane A6 did by hand and what the converter already converts.

| back end | what it emits | status |
| --- | --- | --- |
| Zig, Rust, C | the loop, as without the sugar | expected unchanged; X1 shows the loops (9 lines for a 3-line original) and they agree |
| Verilog | the same loop; a constant trip count unrolls, a run-time bound does not synthesize, as today | unchanged |
| `t27b` | nothing if the expansion is in the shared front end; otherwise `lower.rs` repeats it (net <= 0) | to decide |

Unlocks. Rust, measured by lane A6 with the converter as judge: 17 fns end to end (23 with `find` and `position`; 26 and 32
counting callers) of the 2,546 refused fns that hold a chain (0.7% to 1.3%), 0.17% of all fns seen, below the issue's own
threshold of 100. Whole files: iterators
alone 3 files / 299 lines, with lists 16 files / 2,802 lines. Python: generator expressions alone 64 fns / 477 lines, with
strings 3,226 lines.

Cost: S in the converter (measured, rejected at the threshold). As a language construct M: the lexer already has `|` for `for (x)
|c|`, one desugar pass before code generation, a check of the pure-body rule; no back end. Precedent of this size: #10812.
`FROZEN_HASH` moves; no existing seal moves, because nothing existing uses the syntax.

### Option (e): owned strings as buffer plus count, with a small std

Source (X1, library form): `pub const StrBuf = struct { data : [160]u8, count : usize };` with `str_new`, `str_put`, `str_text`,
`str_uint`, `str_hex8`, `str_is`: 45 lines for the first nine functions of the module. `decode_cor1` of the Python census is 14
lines against 8. Sugar (`s ++ "abc"`, a format string) is not proposed: the std is the feature.

| back end | what it emits | status |
| --- | --- | --- |
| Zig, `t27b` | a struct with a byte array and calls | measured (X1) |
| Rust | the same; `[N]str` elements do not pass as `[]const u8` (D3), a struct field named `len` breaks `.len` (D1) | measured: 4 of 4 `decode_cor1` vectors agree once both are avoided |
| C | text parameters are `[]const u8`: need D4 | expected after D4 |
| Verilog | no: strings are not hardware | refuse |

Unlocks. Rust: strings 922 direct / 1,655 root (the form of #9098 already takes the fns that only append); strings alone as a
whole-file upper bound 4 files / 628 lines. Python: 2,075 lines alone, 3,226 with closures, 5,436 with dict/set (section 1.3).
Inside t27: the 26 buffer-taking copies of `put`, `put_dec`, `put_byte`, `put_u64`, `put_num`.

Cost: S for the library, as one shared module: extend `specs/tri/utils/text.t27` (`put_byte` already lives there) instead of
adding a spec (DRY). A construct (literals, concatenation, formatting, in four back ends, plus the choice between Rust `String`
and `&[u8]`) is L.

### Option (f): none; stop at about 1% and write new code in t27

Cost: nothing in the compiler, no new seal, no `FROZEN_HASH` move; the budget is not an obstacle. What it costs the user: new
t27 code that wants a list or a string pays the library price of X1 (three lines per original line for a one-line Rust fn, 1.75
for the Python string function, a one-time 45-line library) and one copy per element type and capacity.

What the debt does. The hand-written Rust (435,491 swept lines) and Python (331,183 distinct lines) stay except the 1% found:
4,381 Rust lines (3,006 in whole files, 1,375 in movable fns) and, for Python, 812 to 4,090 lines in functions and no whole
file. The budget is shrink-only, so the debt falls only by deletions, and every converted line needs an oracle. New work does not
add to it (the rule of 2026-10-05), so the `.t27` share of the stack (L0) rises with new code, not with conversion.

### The options side by side

| | (a) `Buf(T)` | (b) `Vec(T, N)` | (c) arena | (d) inlined closures | (e) owned strings | (f) none |
| --- | --- | --- | --- | --- | --- | --- |
| Rust first refusals in scope (direct / root) | 761 of the `type Vec` | lists 2,168 / 2,813 | not separable | 419 / 588 | 922 / 1,655 | - |
| measured end to end | 4 fns (1 for parameters) | none (needs the bound) | none | 17 to 32 fns | the #9098 form, count not published | - |
| whole files if only this vanished (upper bound) | 5 files / 802 lines | 5 / 802 | - | 3 / 299 | 4 / 628 | - |
| Python lines, only this blocked | none | none (protects 580) | none | 477 | 2,075 | - |
| Zig, `t27b` | works | works | works | loops | works | - |
| Rust | lifetime plumbing | works | library works | loops | works (D1, D3) | - |
| C | needs slice pair | struct works, slices need D4 | library works | loops | needs D4 | - |
| Verilog | cannot | declaration only | cannot | loops | cannot | - |
| size as a construct (S / M / L) | M to L | L | L | M | L | none |
| size as a library or in the converter | S | S | S | S | S | none |
| `FROZEN_HASH` | moves | moves | moves | moves | moves | no |
| seals moved | slice-field and slice-parameter specs | none existing | none existing | none existing | none existing | none |

## 3. Recommendation and its arithmetic

### 3.1 Recommendation

None of (a)-(e) as a language construct now. Take option (f) with three steps that change no language:

1. Fix the back-end defects that X1 and the corpus exposed (section 6, D1-D4 and D7 for the record). They are defects of forms
   that are already merged (#9098, #11109) and of the 96 specs that declare an `out : []u8` parameter, not of anything new,
   and they are the whole cost of options (a) and (e) on the back ends other than Zig. D4 changes the C signature of slice fns, so
   it ships with its loader (`scripts/tri_loop/t27b_rules.py`).
2. Keep the list, string and arena helpers as one shared library: extend `specs/tri/utils/text.t27` and the collection specs
   that pass, after `t27c dupes --name`, and delete the copies as callers move (26 buffer-taking copies of five names today).
3. Run the sized pilot X2 (section 4.5) before any construct is built; the bar is stated there.

If the owner says "start language features" anyway, the order is (d) first, because it is the only construct that adds no back-end
work, then (b) and (e) as the library the converter targets; (a) is not a new type, (c) is not worth a construct.

### 3.2 The arithmetic

The population the options address is 5,056 fns (root-attributed: lists 2,813, strings 1,655, iterator chains 588). The rates
at which a fix of the nearest kind has turned first refusals into converted fns are measured, each against the population it
addressed: 0.6% (#11109: 4 of the 674 return-`Vec` first refusals), 0.9% (#11145: 23 of the 2,546 refused fns that hold an
iterator chain, with `find` and `position`) and 1.3% (the same, 32, counting the fns that call them). A converted fn carries 8.0
hand lines (#11160: 6,555 / 817), and 6.2% to 27.2% of those lines can be deleted: 6.2% (406 of 6,555) move only with a caller
change, which is the best a form that changes signatures can do (the out-buffer and struct forms do), 27.2% (1,781) move with or
without one. The last row is a stress case, not a measurement.

| rate | from | converted fns of 5,056 | share of 19,392 | hand lines in them | deletable | share of 435,491 swept lines |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 0.6% | #11109 | 30 | 0.15% | 240 | 15 to 65 | 0.003% to 0.015% |
| 0.9% | #11145 | 46 | 0.24% | 368 | 23 to 100 | 0.005% to 0.023% |
| 1.3% | #11145 with callers | 64 | 0.33% | 512 | 32 to 139 | 0.007% to 0.032% |
| 5.0% | four times the best | 253 | 1.30% | 2,024 | 125 to 550 | 0.029% to 0.126% |

What stands behind the iterator wall confirms that the lever is not the heap alone. Lane A6 lowered 250 chains and 17 fns
converted; of the 233 that did not, at most 47 (20%) stopped at a heap-like wall: a type such as `String`, `Path` or a collection (17),
`vec!` or another macro (14), a tuple (10), a `Vec` local (6); and 186 (80%) at something no option here changes: a cascade from a refused callee or `self` type 70, a method outside the
std table 48, an enum variant or fn of another module 25, `uses external` 11, syntax 9, a `!` of an operand of unknown type 7,
others 16. Removing every heap wall moves at most a fifth of that population one step, and then it meets the next refusal.

Against it: the golden track has already found 4,381 lines (3,006 + 1,375), and the first port deleted 170 lines for one spec,
one generated file and one seal (#11157). One construct is an M to L change (49 to 290 files, a `FROZEN_HASH` move, a net-negative
`compiler.rs` edit): in the stress row, 550 lines for 49 to 290 files is 2 to 11 lines per file touched. To delete 1,000 hand
lines (about the 1,375 of all movable fns) the package must convert 459 fns, 9.1% of its 5,056, seven times the best measured rate.

The whole-file route is bounded by Table 3 and not by a measurement: lists, iterators and strings together would make 23 more
files (3,993 lines, 0.9% of the swept lines) convert whole if every second refusal were absent; with maps, sets and tuples 54
files and 10,098 lines (2.3%). The measured survival of the nearest forms (0.6% to 1.3% of fns) says the true figure is a small
part of that; no file-level survival has been measured.

Python offers 3,226 lines of function bodies (strings and closures) up to 6,004 (all four heap categories), not files, behind a
front end that does not exist, and `gen-python` cannot hand a function back. It is conditional demand and is not used above.

The unit that is not in the table is new t27 code: 26 buffer-taking copies of five helper names and 14 passing collection specs
typed for one element each are what it costs today. Option (f)'s second step is aimed at that, and it needs no compiler.

## 4. Experiment X1: the forms, written by hand, on functions from the censuses

### 4.1 Method

The experiment needs no compiler change and commits nothing. One t27 module, written by hand with what t27 says today, holds
one census function per form:

| form | census function | where it comes from | first refusal or blocker |
| --- | --- | --- | --- |
| (a) caller-sized buffer | `sample_words_out` | `sample_words`, Rust, `bootstrap/src/host/dma.rs:230` | `type Vec` |
| (b) fixed capacity | `sample_words` | the same, in the list form | `type Vec` (a local collect over a range, A6's largest group) |
| (d) inlined closure | `count_nonzero` | Rust, `trios/crates/trios-tri/src/arith.rs:142`, `vec.iter().filter(\|&&t\| t != Ternary::Zero).count()` | `method iter` |
| (d) inlined closure | `path_delay` | Rust, `bootstrap/src/compiler.rs:32381`, `arcs.iter().map(\|a\| a.delay_ps).sum()` | `method iter` |
| (e) owned string | `decode_cor1`, `decode_wbstar` | Python, `scripts/dump_bit_config.py:134`, an f-string with `{v:08X}`, a dict and `dict.get`, `list.append`, `"\n".join` | `owned string` (and dict, list growth) |
| (c) arena | `arena_*` | no census function; the library form only | - |

Oracles. The three Rust functions are copied verbatim into a `rustc` program that prints their results (`sample_words` for n =
0, 1, 3, 8, 64; `count_nonzero` on four arrays; `path_delay` on three slices); the Python functions are called by
`python3 -I` on seven values of `decode_cor1` and three of `decode_wbstar`, importing the script by path. Every expected value in
the tests is one of those printed values. The module is checked with `t27c test-report` (Zig, one fresh `ZIG_LOCAL_CACHE_DIR`
deleted after the run), `t27b test`, then pushed through `gen-rust`, `gen-c` and `gen-verilog` and judged by `rustc`, `cc
-fsyntax-only` and `iverilog`. Nine mutants of the module check that the tests are not vacuous.

### 4.2 The module

Hand-written, ASCII, 302 lines. Not committed as a spec; it is the whole experiment.

```
// SPDX-License-Identifier: Apache-2.0
; heap-free collections probe (RFC #11177, experiment X1). Scratch: hand-written with what t27 says
; today, checked, pasted into the RFC, not committed. One census function per form:
;   (a) caller-sized buffer   sample_words_out   the out-buffer form of #11073 (a slice and a count)
;   (b) fixed capacity        sample_words       ListU64 { data : [64]u64, count }
;   (c) arena                 arena_*            Arena { mem : [256]u8, used } and offsets
;   (d) inlined closure       count_nonzero, path_delay: the closure body sits where the loop is
;   (e) owned string          decode_cor1        StrBuf { data : [160]u8, count }
; The rule for (a), (b), (e): an element past the capacity is counted and not written, so count >
; capacity says "it did not fit" and the count is the same whatever room there is.
; ASCII only (L3)
; phi^2 + 1/phi^2 = 3 | TRINITY

module HeapFreeProbe;

pub const ListU64 = struct {
    data : [64]u64,
    count : usize,
};

pub const StrBuf = struct {
    data : [160]u8,
    count : usize,
};

pub const Arena = struct {
    mem : [256]u8,
    used : usize,
};

pub const ARENA_FULL : usize = 65535;

; --- (b) the list: one copy per element type and capacity, because t27 has no generics ----------

pub fn list_new() -> ListU64 {
    return ListU64{ .data = [0] ** 64, .count = 0 };
}

pub fn list_push(l: *ListU64, x: u64) -> void {
    if (l.count < 64) { l.data[l.count] = x; }
    l.count = l.count + 1;
}

pub fn list_fits(l: ListU64) -> bool {
    return l.count <= 64;
}

; --- (e) the string -----------------------------------------------------------------------------

pub fn str_new() -> StrBuf {
    return StrBuf{ .data = [0] ** 160, .count = 0 };
}

pub fn str_put(s: *StrBuf, c: u8) -> void {
    if (s.count < 160) { s.data[s.count] = c; }
    s.count = s.count + 1;
}

pub fn str_text(s: *StrBuf, t: []const u8) -> void {
    var i : usize = 0;
    while (i < t.len) {
        str_put(s, t[i]);
        i = i + 1;
    }
}

; The decimal digits of v (Python: str(v), Rust: v.to_string() appended).
pub fn str_uint(s: *StrBuf, v: u64) -> void {
    if (v >= 10) { str_uint(s, v / 10); }
    str_put(s, 48 + (v % 10) as u8);
}

; Eight upper-case hex digits (Python: f"{v:08X}").
pub fn str_hex8(s: *StrBuf, v: u32) -> void {
    var k : u32 = 8;
    while (k > 0) {
        k = k - 1;
        const d : u32 = (v >> (k * 4)) & 15;
        if (d < 10) { str_put(s, 48 + d as u8); } else { str_put(s, 55 + d as u8); }
    }
}

pub fn str_is(s: StrBuf, t: []const u8) -> bool {
    if (s.count != t.len) { return false; }
    var i : usize = 0;
    while (i < t.len) {
        if (s.data[i] != t[i]) { return false; }
        i = i + 1;
    }
    return true;
}

; --- (c) the arena: allocation is an offset, never a pointer; typed access is byte arithmetic ---

pub fn arena_new() -> Arena {
    return Arena{ .mem = [0] ** 256, .used = 0 };
}

pub fn arena_alloc(a: *Arena, n: usize) -> usize {
    if (a.used + n > 256) { return ARENA_FULL; }
    const off : usize = a.used;
    a.used = a.used + n;
    return off;
}

pub fn arena_set32(a: *Arena, off: usize, v: u32) -> void {
    a.mem[off] = (v & 255) as u8;
    a.mem[off + 1] = ((v >> 8) & 255) as u8;
    a.mem[off + 2] = ((v >> 16) & 255) as u8;
    a.mem[off + 3] = ((v >> 24) & 255) as u8;
}

pub fn arena_get32(a: Arena, off: usize) -> u32 {
    return (a.mem[off] as u32) | ((a.mem[off + 1] as u32) << 8) | ((a.mem[off + 2] as u32) << 16) | ((a.mem[off + 3] as u32) << 24);
}

; --- census function 1: Rust, bootstrap/src/host/dma.rs:230, first refusal `type Vec`
;       (0..n).map(|i| (i as u64) * 0x0100_0100_0100_0101).collect()
;     a closure over a range into a list whose length is a run-time value (A6's largest group).
pub fn sample_words(n: usize) -> ListU64 {
    var out = list_new();
    var i : usize = 0;
    while (i < n) {
        list_push(&out, (i as u64) * 0x0100010001000101);
        i = i + 1;
    }
    return out;
}

; (a) the same function in the caller-sized form: the caller's slice, the count returned.
pub fn sample_words_out(n: usize, out: []u64) -> usize {
    var i : usize = 0;
    while (i < n) {
        if (i < out.len) { out[i] = (i as u64) * 0x0100010001000101; }
        i = i + 1;
    }
    return n;
}

; --- census function 2: Rust, trios-tri/src/arith.rs:142, first refusal `method iter`
;       vec.iter().filter(|&&t| t != Ternary::Zero).count()
pub const Ternary = enum { NegOne, Zero, PosOne };

pub fn count_nonzero(v: []const Ternary) -> usize {
    var n : usize = 0;
    var i : usize = 0;
    while (i < v.len) {
        if (v[i] != Ternary.Zero) { n = n + 1; }
        i = i + 1;
    }
    return n;
}

; --- census function 3: Rust, bootstrap/src/compiler.rs:32381, first refusal `method iter`
;       arcs.iter().map(|a| a.delay_ps).sum()
pub const TimingArc = struct {
    delay_ps : u32,
};

pub fn path_delay(arcs: []const TimingArc) -> u32 {
    var total : u32 = 0;
    var i : usize = 0;
    while (i < arcs.len) {
        total = total + arcs[i].delay_ps;
        i = i + 1;
    }
    return total;
}

; --- census function 4: Python, scripts/dump_bit_config.py:134, first blocker `owned string`;
;     it also holds a dict literal, dict.get, list.append and "\n".join. The dict becomes an if-chain.
;       lines.append(f"  raw  : 0x{v:08X}") ... return "\n".join(lines)
pub fn field(v: u32, hi: u32, lo: u32) -> u32 {
    return (v >> lo) & ((1 << (hi - lo + 1)) - 1);
}

pub fn decode_cor1(v: u32, s: *StrBuf) -> void {
    str_text(s, "  raw                 : 0x");
    str_hex8(s, v);
    str_put(s, 10);
    const w : u32 = field(v, 8, 7);
    str_text(s, "  spi_buswidth      [8:7]   : ");
    str_uint(s, w as u64);
    str_text(s, " (");
    if (w == 0) { str_text(s, "x1"); } else if (w == 1) { str_text(s, "x2"); } else if (w == 2) { str_text(s, "x4"); } else { str_text(s, "reserved"); }
    str_put(s, 41);
    str_put(s, 10);
    str_text(s, "  bpi_page_size     [3:0]   : ");
    str_uint(s, field(v, 3, 0) as u64);
}

pub fn decode_wbstar(v: u32, s: *StrBuf) -> void {
    str_text(s, "  raw                 : 0x");
    str_hex8(s, v);
    str_text(s, "  start_address=");
    str_uint(s, (v & 8388607) as u64);
}

; --- tests: every expected value was printed by the original (rustc, python3) -------------------

test sample_words_agrees_with_the_rust_original {
    const a = sample_words(0);
    assert(a.count == 0);
    const b = sample_words(3);
    assert(b.count == 3);
    assert(b.data[0] == 0);
    assert(b.data[2] == 144117387132666370);
    const c = sample_words(8);
    assert(c.count == 8);
    assert(c.data[7] == 504410854964332295);
    const d = sample_words(64);
    assert(d.count == 64);
    assert(list_fits(d));
    assert(d.data[63] == 4539697694678990655);
}

test negative_control_a_list_past_its_capacity_is_counted_not_written {
    const e = sample_words(65);
    assert(e.count == 65);
    assert(list_fits(e) == false);
    assert(e.data[63] == 4539697694678990655);
}

test the_caller_sized_form_counts_the_same_whatever_room_there_is {
    var room : [4]u64 = [0] ** 4;
    assert(sample_words_out(8, &room) == 8);
    assert(room[3] == 216176080698999555);
    var none : [1]u64 = [7] ** 1;
    assert(sample_words_out(8, none[0..0]) == 8);
    assert(none[0] == 7);
}

test count_nonzero_agrees_with_the_rust_original {
    const a = [Ternary.PosOne, Ternary.Zero, Ternary.NegOne, Ternary.Zero];
    assert(count_nonzero(&a) == 2);
    const b = [Ternary.Zero, Ternary.Zero, Ternary.Zero];
    assert(count_nonzero(&b) == 0);
    const d = [Ternary.NegOne, Ternary.NegOne, Ternary.PosOne, Ternary.PosOne, Ternary.PosOne];
    assert(count_nonzero(&d) == 5);
}

test path_delay_agrees_with_the_rust_original {
    const arcs = [TimingArc{ .delay_ps = 120 }, TimingArc{ .delay_ps = 80 }, TimingArc{ .delay_ps = 4000 }];
    assert(path_delay(&arcs) == 4200);
}

test decode_cor1_agrees_with_the_python_original {
    var s0 = str_new();
    decode_cor1(0, &s0);
    assert(str_is(s0, "  raw                 : 0x00000000\n  spi_buswidth      [8:7]   : 0 (x1)\n  bpi_page_size     [3:0]   : 0"));
    var s1 = str_new();
    decode_cor1(128, &s1);
    assert(str_is(s1, "  raw                 : 0x00000080\n  spi_buswidth      [8:7]   : 1 (x2)\n  bpi_page_size     [3:0]   : 0"));
    var s2 = str_new();
    decode_cor1(256, &s2);
    assert(str_is(s2, "  raw                 : 0x00000100\n  spi_buswidth      [8:7]   : 2 (x4)\n  bpi_page_size     [3:0]   : 0"));
    var s3 = str_new();
    decode_cor1(384, &s3);
    assert(str_is(s3, "  raw                 : 0x00000180\n  spi_buswidth      [8:7]   : 3 (reserved)\n  bpi_page_size     [3:0]   : 0"));
    var s4 = str_new();
    decode_cor1(305419896, &s4);
    assert(str_is(s4, "  raw                 : 0x12345678\n  spi_buswidth      [8:7]   : 0 (x1)\n  bpi_page_size     [3:0]   : 8"));
    var s5 = str_new();
    decode_cor1(4294967295, &s5);
    assert(str_is(s5, "  raw                 : 0xFFFFFFFF\n  spi_buswidth      [8:7]   : 3 (reserved)\n  bpi_page_size     [3:0]   : 15"));
}

test decode_wbstar_agrees_with_the_python_original {
    var a = str_new();
    decode_wbstar(8912895, &a);
    assert(str_is(a, "  raw                 : 0x0087FFFF  start_address=524287"));
    var b = str_new();
    decode_wbstar(4294967295, &b);
    assert(str_is(b, "  raw                 : 0xFFFFFFFF  start_address=8388607"));
}

test negative_control_a_string_past_its_capacity_is_counted_not_written {
    var s = str_new();
    var i : usize = 0;
    while (i < 200) {
        str_put(&s, 65);
        i = i + 1;
    }
    assert(s.count == 200);
    assert(s.data[159] == 65);
    assert(str_is(s, "A") == false);
}

test the_arena_hands_out_offsets_and_says_when_it_is_full {
    var a = arena_new();
    assert(arena_alloc(&a, 8) == 0);
    assert(arena_alloc(&a, 4) == 8);
    assert(arena_alloc(&a, 300) == ARENA_FULL);
    assert(a.used == 12);
    assert(arena_alloc(&a, 244) == 12);
    assert(a.used == 256);
    assert(arena_alloc(&a, 1) == ARENA_FULL);
    arena_set32(&a, 8, 3735928559);
    assert(arena_get32(a, 8) == 3735928559);
    assert(a.mem[8] == 239);
    assert(a.mem[11] == 222);
}
```

### 4.3 Results

| check | result |
| --- | --- |
| `t27b test` | 9 passed, 0 failed; 4.6, 6.5 and 17.7 ms in all in three runs on the loaded Mac (parse, typecheck, lower, codegen, run); a 1,551-word image |
| `t27c test-report` (Zig) | tests 9, pass 9, FAIL 0, rate 100%; 41 runtime asserts executed (9, 3, 4, 3, 1, 6, 2, 3, 10); vacuous passes 0 of 9; 3.0 s wall |
| mutants | 9 of 9 killed (each fails at least one test): lower-case hex digits (2 fail), unguarded list write, a count that stops at the capacity, the wrong enum variant, a mask one bit short, an unguarded string write, an out-buffer write that ignores the room, the arena bound off by one (`> 257`), a swapped byte order. A first draft of the arena test let the off-by-one mutant survive; the boundary asserts (`alloc 244` to exactly 256, then `alloc 1` is full) were added and the mutant dies |
| `gen-rust`, `rustc --crate-type lib`, a driver | compiles; the generated `decode_cor1` agrees with Python on 4 of 4 vectors; `sample_words(65)` has count 65 and `data[63] = 4539697694678990655`; `sample_words_out(8, room)` returns 8 and fills `room[3] = 216176080698999555`; `count_nonzero` is 2; the arena hands out 0, 8 and `ARENA_FULL`, and `get32` reads back `0xdeadbeef`. `gen-rust` does not lower tests (its header says so), so this is a cross-check by driver, not a test run |
| `gen-c`, `cc -std=c11 -fsyntax-only` | 12 errors: 7 are a slice parameter lowered to a bare pointer (`t.len`, `v.len`, `arcs.len` on a pointer), 2 a range slice `none[0..0]` emitted as `none[(0 .. 0)]`, 3 an untyped array-literal local emitted as `__auto_type a = {..}`. The signature `sample_words_out(size_t n, uint64_t* out)` has lost the room, so the out-buffer contract is gone even where it compiles. The struct and arena functions compile |
| `gen-verilog`, `iverilog -g2012` | 20 elaboration errors: a function taking `*ListU64`, `*StrBuf`, `*Arena` or a slice leaves its fields unbound (`l_count`, `a_used`, `v_len`, `arcs_len`) |

Cost per function (hand lowering, non-blank non-comment lines): the one-line Rust functions are 3 lines in the original and 9 in
t27 (three times); `decode_cor1` is 8 lines of Python and 14 in t27 (1.75 times), with a one-time library of 45 lines for the
list and string helpers (`list_*` 10, `str_*` 35). The loops are exactly what an inlining front end would have to emit; the
longer form is the price of having no closure syntax, not of the heap-free storage.

What X1 does not show. It is four functions chosen by hand for being small and pure; it shows that the forms can say them and on
which back ends, not how many functions in the censuses they can say. The Python original works on unbounded integers; the t27
`decode_cor1` is a `u32` function, so a port has to prove the range or refuse. Rust's `sum()` panics on overflow in debug where
t27 traps; both abort, and the spec of a port must say so.

### 4.4 Exit criteria for X1 (what justifies building anything)

| # | criterion | threshold | X1 |
| --- | --- | --- | --- |
| E1 | each census function passes `t27c test-report` and `t27b test` | all pass, 0 vacuous | met: 9/9, 0 of 9 vacuous |
| E2 | tests pin the forms | every mutant killed | met: 9/9 |
| E3 | generated Rust compiles and agrees with the original on every vector | all vectors | met, with D1 and D3 avoided |
| E4 | generated C compiles and agrees | all vectors | **not met**: 12 errors, 7 from slice parameters. This gates (a), (e) and every text parameter |
| E5 | cost of the hand-lowered function | at most 3 lines per original line | met: 3.0 and 1.75 |
| E6 | generality | E1-E5 hold on 30 more functions, 10 per family, drawn by a fixed seed from the sweep | not run; this is the second half of X1 |

A construct is built only if E1-E6 hold, D1-D4 are fixed, X2 passes, and the pull request is net-negative in hand lines.

### 4.5 X2: the pilot that measures the package, sized and not run

X1 measures feasibility; the demand needs a rate. X2 needs no compiler change either.

- Population: the 5,056 fns above (lists 2,813, strings 1,655, iterator chains 588) in the saved sweep.
- Sample: 100, a fixed seed, stratified 34 / 33 / 33 across the three families, tests included (they are most of the local
  collects and 44,864 `cfg(test)` lines).
- Method, as lane A6 did for chains: rewrite each sampled fn at the Rust level into the heap-free Rust form (a `Vec` with a
  readable bound becomes `[T; N]` and a count; a closure chain becomes a counted loop; a `String` becomes `[u8; N]` and a count
  with appends). The unmodified converter, `--check` oracle included, judges the rewritten text. No compiler or spec changes.
  Count separately the fns for which no bound could be read (the share of the population the forms can apply to at all).
- Threshold: at least 15 of 100 convert end to end (one-sided 95% lower bound 9.5%, against the 9.1% that deletes 1,000 hand
  lines at 8.0 lines per fn and 27.2% deletable). Fewer: stop at (f) and record the rate.
- Cost: one lane, the size of #11145.

## 5. Risks and open questions, each with the default taken

| # | question or risk | default taken |
| --- | --- | --- |
| 1 | Which back ends must carry a heap-free form? | Zig and `t27b` now; Rust after D1-D3; C after D4; Verilog refuses with a message that names the form |
| 2 | Overflow: count and drop, or trap? | count and drop for every capacity-bounded form (the rule of #11073 and #9098: the count is the same whatever the room); `t27b --overflow trap` stays for arithmetic |
| 3 | Generics, or one copy per element type and capacity? | library copies in one module; no generics; revisit only if X2 clears its bar, because a type constructor with a value parameter is the L-sized item of this RFC |
| 4 | A bound that cannot be read (141 slice-length collects)? | refuse, naming "no stated bound"; never guess a capacity |
| 5 | Large capacities copied by value? | library functions take `*T`; a by-value return only when N times the element size is at most 512 bytes (what X1 does) |
| 6 | Does Python count? | no: conditional on a front end the owner has not started; its lines are reported and not used in the recommendation |
| 7 | A struct field named `len` breaks `.len` in `gen-rust` (D1) | name the count `count` in library structs until D1 is fixed |
| 8 | Where do the shared helpers live? | extend `specs/tri/utils/text.t27` and the 14 collection specs that pass; run `t27c dupes --name` first; add no new spec (DRY) |
| 9 | An owner label for foreign growth? | none requested; any construct PR must be net-negative in hand lines (the precedents are +31/-48 and +39/-75) |
| 10 | Are tests part of the prize? | yes for the gate's line count, no for the movable-fn tables (#11160 excludes `cfg(test)`); X2 includes them |
| 11 | Second refusals: the converter stops at the first, and #11135 and #11145 each found the next wall in a neighbouring family | measure the package (X2), not each feature alone |
| 12 | The oracle records scalars only (#11109: struct, enum and `&str` elements convert 8 more fns but cannot be recorded element by element) | out of scope here; noted as a cap on any element type other than integers, floats and bools |
| 13 | Integer width and overflow: unbounded Python ints, panicking Rust `sum()` | a port states the width in the spec and refuses what it cannot prove |
| 14 | Who files the follow-up issues for D1-D7? | this lane files none (its brief is one document); each defect has a reproduction in section 6 |

## 6. Found on the way, what was not run, sources

### 6.1 Defects the experiment found (binaries named in the header; none was fixed here)

- D1. `gen-rust` leaves `t.len` as it is (not `t.len()`) on a slice parameter when the module declares a struct field named `len`.
  X1 with the field called `len` fails `rustc` (E0615); renamed `count` it compiles.
- D2. `gen-rust` writes a slice-typed struct field as `Vec<T>` and the constructor takes it by value: `specs/tri/collections/stack.t27`
  becomes `items: Vec<i64>` and `empty(storage: Vec<i64>)`; `rustc` reports 2 type errors. The caller's storage is moved, not borrowed.
- D3. `gen-rust` types a `[N]str` constant element as `&str`, which does not pass where `[]const u8` is expected (E0308).
- D4. `gen-c` lowers a `[]T` parameter and a slice field to `T*`: the length is lost and `x.len` is invalid C (7 errors in X1, 7
  in `stack`). The specs that are already C-bound (`gen/c/policy/json_int53.c`) pass the length as a second parameter by hand.
- D5. `gen-c` emits a range slice `a[0..0]` as `a[(0 .. 0)]` and an untyped array-literal local as `__auto_type a = {..}`.
- D6. `gen-c` emitted `var a : [4]u64 = [7] ** 4;` as `/* repeat: { 7 } ** 4 */ {0}`: compiled and run, `a[2]` is 0 where `t27b`
  and Zig give 7. The master source has a branch for this (#6050: "any other single value takes the GNU range"), so a fresh
  build of master may differ; not re-checked.
- D7. `gen-verilog` leaves the fields of a `*Struct` or slice parameter unbound (`iverilog`: `l_count`, `a_used`, `v_len`).

### 6.2 Commands

```
# hf.t27 is the module of section 4.2 saved to a scratch file
t27b test hf.t27
t27c test-report hf.t27     # ZIG_GLOBAL_CACHE_DIR shared; a fresh ZIG_LOCAL_CACHE_DIR per run, deleted after
t27c gen-rust hf.t27 > hf.rs && rustc --crate-type lib --edition 2021 hf.rs
t27c gen-c hf.t27 > hf.c && cc -std=c11 -fsyntax-only hf.c
t27c gen-verilog hf.t27 > hf.v && iverilog -g2012 -o hf.vvp hf.v
# corpus counts of 1.4 (approximate, grep over specs/):
grep -rlE --include='*.t27' '\bout *: *\[\](const )?u8\b' specs                                         # 96
grep -rlE --include='*.t27' '^\s+[a-z_0-9]+ *: *\[[A-Za-z0-9_]+\][A-Za-z0-9_]+,' specs                 # 206 array field
grep -rlE --include='*.t27' '^\s+[a-z_0-9]+ *: *\[\](const )?[A-Za-z0-9_]+,' specs                     # 248 slice field
grep -rlE --include='*.t27' ': *\[\](const )?[A-Za-z]' specs                                           # 883 slice anywhere
grep -rlE --include='*.t27' 'const [A-Z][A-Za-z]*\(T\)' specs                                          # 11 generic form
grep -rlE --include='*.t27' 'Allocator|\balloc\(|create_buffer|ArrayList|std\.heap' specs              # 77
```

### 6.3 Not run

- A fresh `cargo build` of master: `t27c` (built 2026-10-19 13:04) and `t27b` (2026-10-19 16:23) are the shared binaries; the
  `gen-c` and `gen-verilog` findings are theirs. `gen/` was not regenerated and no seal was touched.
- `tri t27o-sweep` over the five repositories: it needs git checkouts; lane A4's saved per-file outputs were read instead, with
  a reader that reproduces the published counts (section 1.2). The real command was run on this checkout (t27 only).
- The split of the Python heap decomposition by repository: lane A9's scratch was removed while this lane ran.
- X2, and the second half of X1 (E6): sized above, not run.
- A Verilog simulation of any form; the only Verilog check is elaboration.
- Whether any option changes the converted counts: nothing in the converter was touched.

### 6.4 Sources

Issues and pull requests: #11177 (this RFC), #11135 (Vec parameters, lane A5), #11145 (iterator chains, lane A6), #11149 and
`docs/now/2026-10-20-golden-candidates.md`, #11159 and `docs/now/2026-10-20-golden-functions.md`, #11168 and
`docs/now/2026-10-20-python-debt.md`, #11127 (root attribution), #11109 (Vec out-buffer, closes #11073), #9098 (String out-buffer),
#10546 (the Vec epic), #10602 (ranked refusals), #10812 and #9135 (cost precedents), #11157 (the first golden port), #8995 and
#9006 (the Python front end), the epic ledger on #10522 (rows 33-37).
Files: `specs/tools/t27o/rust_convert.t27` (header), `specs/tools/t27o/ledger.t27` (`root_of`), `specs/policy/own_language.t27`
(budget), `docs/T27-CONSTITUTION.md` (L0 PURPOSE, L2 GENERATION, L4 TESTABILITY), `CONTRIBUTING.md` (seal discipline,
`FROZEN_HASH`), `docs/BACKEND_CONTRACT.md`, `specs/tri/collections/`, `specs/compiler/c_fixed_array_return.t27`,
`scripts/dump_bit_config.py`, `bootstrap/src/host/dma.rs`, `bootstrap/src/compiler.rs`, `trios/crates/trios-tri/src/arith.rs`.
