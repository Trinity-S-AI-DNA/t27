# Boilerplate: how a spec imports a family of names (epic #12444, row H, issue #12452)

Date: 2026-10-25. Design report with measurements. No compiler change is made or proposed as a precondition.

Baseline: every count below is read from the committed tree `d9f093ff2d` with `git grep ... d9f093ff2d`, not from
the working tree. Other sessions were editing `specs/widgets/*.t27` in this worktree while I measured (the working
tree total moved from 10007 to 10023 mid-run), and HEAD has since moved to `745e734cf9`, where the total is 10028
(16 widgets now say `use std::mem::bytes_eq;`). The commands are in section 8 so the figures can be re-run at any commit.

## 0. The answer in short

- 10007 `use` lines start in column 0 across 594 of 2810 specs. 9475 of them are item or brace lines whose module
  file exists, and they are 897 distinct (module, importing spec) pairs. Naming each module once per spec instead of
  once per name leaves 1429 lines: **8578 lines (85.7 percent) and 308,012 of 342,416 bytes (90.0 percent) saved.**
- The spelling that does it, `use a::b;` (whole module), **already exists**: 473 column-0 lines in 293 specs plus
  661 indented ones. So a glob exists in another spelling and no compiler change is needed. `use m::*;` has 0 uses.
- Three designs compared (section 3): whole-module `use m;` saves 8578 lines; block form `use m::{a, b};` saves 8578
  lines only if lines may run to 2466 characters (7637 if wrapped at 100 columns) and 189,276 bytes; a facade module
  saves a further 50 lines net, and only for the widgets kit.
- All of them leave the flat-rename rule (R3 of `specs/compiler/use_splice.t27`) unchanged, because the closure of
  imports is the same. A facade that **re-exports** (`pub use`, not in the language) is the one design that would
  have to change it.
- Recommendation: whole-module `use m;`, applied in family batches. Cheapest first step: the five widgets kit modules
  plus `t27f::viz_contract` in 43 specs: **3360 lines become 108 (3252 saved, 32.5 percent of all `use` lines).**
- Decision owner: the language owner (C-Compiler for the language, B-Builder for `bootstrap/src/use_resolve.rs`),
  not epic #12444 and not the widgets epic #10937.

## 1. Measurements (FR-001)

### 1.1 The issue's numbers against what reproduces

| Claim in the issue | At `d9f093ff2d` | Verdict |
|--------------------|-----------------|---------|
| 9950 `use` lines | **10007** | Does not reproduce at this commit. 9950 is the value at `4c7dbe4cd9` (HEAD~10); the tree grew 57 lines in 10 commits. |
| 98 specs with 20 or more `use` lines | 98 | Reproduces. |
| `widgets::js_num` 811 | 811 | Reproduces (lines that start `use widgets::js_num`). |
| `t27f::viz_contract` 722 | 722 | Reproduces. |
| `widgets::json_scan` 661 | 661 | Reproduces. |
| `widgets::html_out` 607 | 607 | Reproduces. |

The issue counts lines that start in column 0. Indented lines (`module X { use base::types; }`) are a second population
of 737 lines in 398 specs that the resolver also reads (`line.trim().strip_prefix("use ")`, use_resolve.rs line 67):
661 of them are already whole-module, 76 are item or brace, none passes 9 per spec. With them there are 10744 `use`
lines. They are not a boilerplate problem and are left out of every figure below.

### 1.2 Shapes of the 10007 column-0 lines

| Shape | Lines | Notes |
|-------|-------|-------|
| Total | 10007 | in 594 specs (21 percent of 2810 `.t27` files) |
| Whole module `use a::b;` (file `a/b.t27` exists) | 473 | 141 modules, 293 specs |
| Item `use a::b::Item;` | 9399 | one name per line |
| Brace `use a::b::{X, Y};` | 133 | longest line 731 characters |
| Glob `use a::b::*;`, `pub use`, `use ... as ...` | 0, 0, 0 | none of the three exists in the corpus |
| Item or brace lines whose module file exists | 9475 | the convertible set; 897 (module, spec) pairs |
| Item or brace lines whose module file does not exist | 57 | 52 `use agents::<Letter>;`, 3 `use base::math::*`, 2 `use std::*` (see 1.5) |
| Lines outside the pattern | 2 | `use sandbox.session_timeout;` (dotted path), `use t27os::petal_centre_unused` (no semicolon) |
| Exact duplicate `use` line inside one spec | 0 | |

Names imported by the 9475 lines: 10425 (the 109 brace lines that list two or more names carry 1059 names).

### 1.3 Repeated and large imports

| Measure | Pairs | Names or lines |
|---------|-------|----------------|
| (module, spec) pairs | 897 | 10425 names, 9475 lines |
| Same module imported on 2 or more lines of one spec | 571 | 9149 lines |
| Same module, 2 or more names (brace names counted) | 678 | 10206 names |
| 5 or more names from one module in one spec | 467 | 9633 names (92.4 percent of all names) |
| 10 or more names | 293 | |
| 50 or more names | 31 (30 by lines) | |
| Pairs with exactly one name | 219 | already minimal |

### 1.4 Specs with many `use` lines, before and after whole-module form

| Measure | Before | After (design A) |
|---------|--------|------------------|
| Specs with 20 or more `use` lines | 98 (hold 8213 lines, 82.1 percent) | 3 |
| Specs with 50 or more | 47 | 0 |
| Specs with 100 or more | 27 | 0 |
| Largest | `t27os/glass_window.t27` 1818 | 48 (49 with the one line that has no semicolon) |
| Next | `t27os/pty_server.t27` 272, `queen/actors_hw.t27` 174, `widgets/bit_autopsy.t27` 167 | all below 20 |

The 3 that remain are `glass_window` (48 modules) and two specs, `t27os/identity.t27` (28) and
`t27os/domain_pulse.t27` (27), that each hold 26 unresolvable `use agents::<Letter>;` lines (1.5). Without those lines
one spec is left at 20 or more.

Top modules by lines (all item or brace lines of one module): `widgets::js_num` 811, `t27f::viz_contract` 722,
`widgets::json_scan` 661, `widgets::html_out` 607, `widgets::js_val` 403, `queen::actors` 324, `t27os::keys` 213,
`t27os::dash` 194, `t27f::spec_graph` 171, `analog::mos_table` 159, `widgets::canvas_out` 156,
`analog::mos_psp103_data` 143. By importing specs (item form): `widgets::html_out` 29, `port::scripts::gen_w_flat` 29,
`widgets::json_scan` 23, `widgets::js_num` 22, `t27f::scene_raster` 22, `analog::linear` 21.

### 1.5 Things found on the way

- **57 lines name a module with no file.** `use agents::A;` ... `use agents::Z;`, `use agents::ti;` look for
  `agents.t27`, which does not exist; the files are `specs/agents/a.t27` and so on (lower case). On a case-insensitive
  file system `specs/agents/A.t27` finds `a.t27`, so these lines may resolve on this Mac and not on Linux CI. Not
  verified (I did not run t27c). `missing_use_notes` in use_resolve.rs already warns about such lines. If they are
  dead, deleting them is 59 more lines for free and is the whole reason two specs stay above 20.
- **No spec mixes the two spellings for one module.** The 473 whole-module pairs and the 897 item pairs share no
  (module, spec) pair, so a conversion never has to merge with an existing `use m;` line.
- **Facades have little to gather.** The most common exact set of modules imported together is the four widgets kit
  modules (10 specs); the next is 7 specs (`t27f::scene_bg`, `scene_lex`, `scene_raster`). Across the importing specs,
  242 (spec, directory) groups import 2 or more modules of one directory, worth at most 568 lines over design A
  (section 3.3).

## 2. What the splice does with a `use` line (read, not run)

Read: `bootstrap/src/use_resolve.rs` (`use_target`, `consumed_use`, `identifiers`, `resolve`),
`specs/compiler/use_splice.t27` (rules R1 to R5), and for a glob the whole of `use_path`. I did not run t27c.

1. **Whole module exists already.** `use_target` (line 113): `use a::b;` names `<specs>/a/b.t27` and is "whole". The
   item forms `use a::b::Item;` and `use a::b::{X, Y};` name the module that holds the items and are "not whole". One
   level only. So `use a::b::c;` already resolves a whole file when `a/b/c.t27` exists. That is a glob in another
   spelling, and it is used 473 times at column 0.
2. **A glob is not needed, and `use a::b::*;` would likely already be read.** By `use_target`, `use a::b::*;` pushes
   `*` onto the path, finds no `a/b/*.t27`, and falls to the item branch: module `a/b.t27`, not whole, item `*`. Nothing
   declares `*`, so `consumed_use` keeps the line and no qualifier is rewritten. Whether the parser accepts `*` in a
   `use` path I did not read. Untested, 0 uses in the corpus.
3. **The closure does not depend on the spelling.** The doc comment above `use_target` says it: "A module named through
   an item is spliced like any other -- the referenced names it declares are pulled, not only the listed ones". The
   frontier starts from `identifiers()` of the whole importer text. That counts names written on `use` lines, so a
   listed name is a mention and pulls its declaration. The fixpoint then follows names inside pulled declarations. The
   imports of an imported module are followed (transitive), including through a dependency that fails to parse.
   **Item form, block form and whole-module form pull the same declarations**, except for a listed name the importer
   never writes anywhere else (a dead name): item form pulls it, whole-module form does not.
4. **What does differ is the line left behind.** `consumed_use` (line 583) turns item and brace lines into
   `// use ... -- X spliced below by t27c (#7281)`, one line in and one line out. A whole-module line stays as a `use`
   line, and a whole import also makes the module's basename a qualifier (`html_out::put` is rewritten to `put`).
   So every converted spec resolves to different text (comment lines become kept `use` lines), and the 1134 whole
   lines that exist today show the backends cope. This is the thing to byte-compare, not assume.
5. **The rules R1 to R5** (`specs/compiler/use_splice.t27`): R1 a name inside a pulled declaration of module O is read
   in O's scope, "the declaration O makes, else the one declaration its direct imports make"; R2 the scope is only
   consulted when the whole closure cannot answer, so "a spec that resolved yesterday generates the same bytes today";
   R3 a scoped copy is named `<module path>__<name>` (`flat_name`, `rewrite`); R4 a bare name the importer writes that
   two imported modules declare differently stays UNRESOLVED; R5 paths in a pulled text lose their qualifier. The
   splice keys declarations by bare name over the closure, as the issue says.

## 3. Designs compared (FR-002)

Figures are over the 10007 baseline lines and 342,416 bytes. "Convertible" is the 9475 item or brace lines whose module
exists; the 57 + 2 other lines stay in every design.

| # | Design | Spelling | Compiler change | `use` lines after | Saved lines | Saved bytes | R1 to R5 and flat names unchanged |
|---|--------|----------|-----------------|-------------------|-------------|-------------|-----------------------------------|
| A | Whole module | `use widgets::js_num;` | none, exists (473 + 661 lines) | 1429 | **8578** (85.7 percent) | 308,012 (90.0 percent) | **yes** |
| B | Block form | `use widgets::js_num::{fmt_u32, put_u32};` | none, exists (133 lines) | 1429 as one line each; 1838 if wrapped at 100 columns | 8578 unwrapped; 7637 wrapped | 189,276 (55.3 percent) | yes |
| C | Module glob | `use widgets::js_num::*;` | parser check; the splice reads it as item form (2 above) | 1429 | 8578 | 305,321 | yes, but it is A with a longer spelling, a kept line and no qualifier |
| D | Facade, transitive | `use widgets::kit;` over a module that only holds `use` lines | none | 1379 (kit only) | 8628 (8578 + 50) | not measured | R3 and R4 yes; R1 loses sight of origin modules (3.3) |
| E | Facade that re-exports | `pub use` in the facade | **yes**, new syntax | as D | as D | as D | **no**: R1 and R3 must say which module a re-exported name belongs to |

### 3.1 Design A, whole module

Saves the most because it names a module once per spec: 897 pairs from 9475 lines. It also drops listed names the
importer never writes (they stop pulling their declarations; section 7). It needs no syntax work, no splice change
and no new module. Lines per spec stay below 50 everywhere (largest `glass_window` 48).

### 3.2 Design B, block form, and design C, glob

B has the same line count only on paper. 359 of the 897 block lines exceed 100 characters, 196 exceed 200, 37 exceed
500, and the longest is 2466 (the longest existing brace line is 731); wrapped at 100 columns the 897 lines become 1838.
It keeps every name as a mention, so nothing about the pull set changes, and it is the only design where the file still
says which names a spec uses. It saves 189,276 bytes against A's 308,012. Single-name pairs (219) stay in item form.
C costs a parser question and gains nothing over A.

### 3.3 Design D and E, a facade

A facade is a module made only of `use` lines plus one test (L4), for example `widgets::kit`:
`use widgets::html_out; use widgets::js_num; use widgets::js_val; use widgets::json_scan; use widgets::canvas_out;`.
An importer writes `use widgets::kit;`. It works with no compiler change because the closure is transitive.

- Widgets kit (those five modules): 31 specs import at least one, 25 import two or more. Over design A it saves
  **65 lines gross** (sum of count minus 1 over the 25 specs), against about 15 lines for the facade file (SPDX line,
  two prose lines, `module`, five `use` lines, one test), so **about 50 net**. Four modules only: 59 gross.
- The five kit modules declare 154 names with **0 duplicates among them** (measured on the working tree files, which
  other sessions were not editing), so R4 cannot bite an importer that now sees all five.
- Upper bound for facades everywhere, one per directory: 242 (spec, directory) groups in 21 directories, 568 lines
  gross over A. The facades would list 294 modules and cost about 21 x 9 + 294 lines, so the net is about 85. That is
  1 percent of what design A saves and widens each importer's closure. Not worth doing outside the widgets kit.
- A facade hides origin modules from R1: the scope of module O is "the declaration O makes, else the one declaration its
  direct imports make", and a facade has no declarations of its own. This only matters for a name two modules declare
  differently (R2), which the kit does not have today, but it makes future ambiguity harder to resolve. This is my
  reading of the rule text, not a test.
- Qualified references (`html_out::put`) written in an importer are rewritten only for modules the importer imports
  whole; through a facade the importer would need unqualified names. The widgets specs write unqualified names today.
- E (`pub use`) is the only design that touches R1 and R3 and the language, and it buys nothing D does not.

### 3.4 Effect on the duplicate-bodies ratchet

None. `tools/dupe_scan.py` counts function bodies that are byte-identical after comments and whitespace are normalised
away (its docstring), kept as a ledger that only shrinks. A `use`
line is not a function body, so converting import lines changes no group, and a facade module adds no function. The
ratchet does not need `--bless` for this work.

## 4. How Chisel, Zig and Rust import a family

The sibling report `docs/reports/boilerplate_competitors_2026-10-25.md` records what each ships, not how a caller
spells the import: Chisel `chisel3.util` as a package (its section 1), Zig `std.mem` (section 6), Rust std docs with a
Prelude page (section 7), Veryl a `$std` namespace "usable without a dependency entry" (section 5), Dafny `import opened`
after one flag (section 8). The spellings below are **general knowledge, not read from that page and not checked
against a source in this task**:

| Language | One line brings a family | Per-name lines | Equivalent here |
|----------|--------------------------|----------------|-----------------|
| Chisel (Scala) | `import chisel3.util._` (wildcard of a package) | `import chisel3.util.{Queue, Valid}` | A or C |
| Zig | `const mem = @import("std").mem;` binds the namespace, members are reached as `mem.eql(...)` | none; there is no brace list or glob | A with qualifiers (`html_out::put`) |
| Rust | `use m::*;` or a prelude module that does `pub use` | `use m::{a, b};` | A or C, B, and E (a prelude re-exports) |

Each reaches a family in one line. None lists a family one name per line; Rust is the only one with all of our
designs, and its prelude is the facade-with-re-export (E). t27 already has the Zig shape in the whole-module form.

## 5. Overlap with the widgets work and who decides (FR-003)

- Widgets epic: gHashTag/t27#10937, "Epic: widget factory - retire 22,833 hand-written widget lines into .t27"
  (OPEN). It touches the same files: all `widgets::` modules together are 2971 item lines in 112 (module, spec)
  pairs, of which design A alone saves 2859 (section 6).
- 27 widget specs carry a hand function `imports_anchor` whose comment (for example `specs/widgets/race_the_bee.t27`
  line 596) says "The splicer follows only the names a module itself mentions, not the names the imported functions
  mention". My reading of `resolve` is the opposite (the fixpoint follows names inside pulled declarations). One of the
  two is wrong, or the anchors guard something I did not read. If the anchors are dead they are further lines to
  delete, so the widgets epic should be told before its generator emits imports.
- Whoever generates widget specs should emit one `use widgets::js_num;` per module, not one line per name, from now on.
- **Decision owner:** the language owner. `specs/OWNERS.md` row `base/`, `compiler/` is **C-Compiler** ("Core
  language"); `bootstrap/OWNERS.md` names B-Builder for the Rust bootstrap compiler t27c, which holds `use_resolve.rs`. Epic #12444 (boilerplate) only
  measures and recommends; the widgets epic #10937 owns the widgets rewrite, not the import syntax. `t27f/` is
  T-Queen's (that is `t27f::viz_contract`, 722 lines). No compiler change is made here.

## 6. Recommendation (FR-004)

**One design: whole-module `use m;`, the form the language already has (design A).** Do not add a glob or `pub use`.
Keep block form only where a spec wants to document that it needs two or three names of a big module. Add a facade
only for the widgets kit (about 50 net lines), as a second step if the first batch lands.

**Cheapest first step, no compiler change:** fold the per-name lines of six modules into one line each, in the 43 specs
that import them (31 of them under `specs/widgets/`):

| Batch | Modules | Item lines now | Pairs (lines after) | Saved |
|-------|---------|----------------|---------------------|-------|
| 1 | `widgets::js_num`, `json_scan`, `html_out`, `js_val`, `canvas_out`, `t27f::viz_contract` | 3360 | 108 | **3252** (32.5 percent of 10007) |
| of which the issue's three | `widgets::js_num`, `json_scan`, `html_out` | 2079 | 74 | 2005 |
| of which | `t27f::viz_contract` | 722 | 12 | 710 |

Then: all of `widgets::` (2971 lines in 112 pairs, 2859 saved), then the 30 pairs with 50 or more lines (2583 lines),
then `t27os::glass_window` (1818 to 48, an `@extern` host built by `t27b` with `make t27os`; keep it last).

How to land a batch (the parent runs this; I ran none of it): for each spec replace the lines of module `m` by one
`use m;` at the position of the first; compare the resolved text and the generated output before and after (the only
expected difference is a comment line becoming a kept `use` line, plus dropped dead names); reseal with a master-built
`t27c seal <spec> --save`; run `./scripts/tri test`. The rewrite itself must not be a Python, awk or shell program (rule
only-t27); a small `.t27` tool in `specs/tools/`, in the style of `dupes_key.t27`, or an edit by hand per spec is the
way. New std callers should write `use std::mem;` and `use std::text;`, one line however many helpers the module grows
to (16 specs say `use std::mem::bytes_eq;` at `745e734cf9`).

## 7. Not verified, and where it could bite

1. **Generated bytes.** I did not run t27c. Item lines become comments in the resolved text and whole lines stay; the
   backends' treatment of the kept line in a converted spec is untested (it is exercised by the 1134 whole lines that
   exist). Compare before sealing.
2. **Dead listed names.** A name listed on a `use` line and written nowhere else pulls its declaration today; after
   conversion it does not. An `@extern` host or an exported symbol that relies on that would lose it (`glass_window`
   has `use t27os::petal_centre_unused`, which suggests such names exist). Not counted.
3. **`imports_anchor`** comment against the code (section 5).
4. **Qualifier rewrite** for whole imports inside string literals: `use_splice.t27` says `rewrite` leaves string and
   character literals alone; I did not read the separate qualifier stripping for whole imports.
5. **Seals.** Every edited spec needs its seal re-saved from a master-built `t27c`; batch 1 is 43 reseals.
6. **t27b parity.** t27b compiles the same `use_resolve.rs`; not rerun.
7. **Verilog.** The sibling report says the Verilog backend drops functions that arrive through `use` (its compiler
   list, item 1). `specs/tri/t27b/conformance/verilog_use.t27` (#8796) says `t27b verilog` carries the use closure
   while t27c gen-verilog wrote a call with no definition. Which of the two a given spec goes through decides whether
   a hardware spec sees a shared module at all; the spelling of the import does not change that, so it neither helps
   nor blocks this work. Not run.
8. **The `agents::<Letter>` lines** (1.5): case-insensitive file system effect, not verified.

## 8. Commands used (all read-only; shell one-liners with grep, sed, sort, join, uniq, wc, paste and bc)

Stream definitions, run in zsh from the worktree root. Use `${R}:` before a colon, never `$R:`.

```
R=d9f093ff2d; export LC_ALL=C
M=$(git ls-tree -r --name-only $R specs | grep '\.t27$' | sed -E 's#^specs/##; s#\.t27$##; s#/#::#g' | sort)
S=$(git grep -E '^use ' $R -- 'specs/*.t27' | sed -E 's/^[^:]*:([^:]*):use +([^;]*);.*$/\2|\1/' | grep -E '^[A-Za-z0-9_:{}, -]+\|specs/[^|]*\.t27$' | sort -t'|' -k1,1 -k2,2)
W=$(join -t'|' <(echo "$M") <(echo "$S"))
N=$(join -t'|' -v 2 <(echo "$M") <(echo "$S") | sed -E -e 's/^(([^|{]*)::\{([^}]*)\})\|(.*)$/\2|\4|\3|\1/; t' -e 's/^(([^|{]*)::([^:|{]*))\|(.*)$/\2|\4|\3|\1/' | sort -t'|' -k1,1 -k2,2 -k3,3)
N2=$(join -t'|' <(echo "$M") <(echo "$N"))
```

`M` is the module list, `S` the parsed lines as `X|spec`, `W` the whole-module lines, `N` the item and brace lines as
`module|spec|names|X`, `N2` those whose module file exists. The sort must be `-t'|' -k1,1 -k2,2`: a whole-line sort
orders `base::types|` after `base::types_x`, `join` then misses 213 `t27os::keys` lines, and whole-module came out as 260
instead of 473 (my first run).

| Figure | Command |
|--------|---------|
| Total, specs, modules | `git grep -hE '^use ' $R -- 'specs/*.t27' \| wc -l` (10007); `git grep -cE '^use ' ...\| wc -l` (594); `git ls-tree -r --name-only $R specs \| grep -c '\.t27$'` (2810) |
| 20 or more, 50 or more, 100 or more | `git grep -cE '^use ' $R -- 'specs/*.t27' \| grep -cE ':([2-9][0-9]\|[1-9][0-9]{2,})$'` (98), `':([5-9][0-9]\|[1-9][0-9]{2,})$'` (47), `':[1-9][0-9]{2,}$'` (27); the 8213 is the same grep, `sed 's/.*://' \| paste -sd+ - \| bc` |
| Brace, glob, `pub use`, alias | `grep -hE '^use .*\{'` (133), `'^use .*\*'` (0), `'^pub use '` (0), `'^use .* as '` (0), all with `git grep -h ... $R` |
| Indented | `git grep -hE '^[[:space:]]+use ' $R -- 'specs/*.t27' \| wc -l` (737); classified with `join` as above (661 whole, 76 item) |
| Whole, item, convertible | `echo "$W" \| wc -l` (473), `join -v 2 ... \| wc -l` (9532), `echo "$N2" \| wc -l` (9475) |
| Pairs | `echo "$N2" \| cut -d'\|' -f1,2 \| sort \| uniq -c` (897 lines); thresholds with `grep -cE '^ *([2-9]\|[0-9]{2,}) '` and similar |
| Names | brace names split with `sed -E -e 's/^([^\|]*\|[^\|]*\|)([^,\|]*), (.*)$/\1\2\<newline>\1\3/' -e 'P;D'` (10425) |
| Block form text | the same names grouped with BSD `sed -e ':a' -e '$!N' -e 's/^\(\([^\|]*\|[^\|]*\|\)[^\|]*\)\n\2/\1, /' -e 'ta' -e 'P;D'`, then `wc -c`, `fold -w 100 \| wc -l` (1838) |
| Bytes | `echo "$N2" \| cut -d'\|' -f4 \| sed 's/^/use /; s/$/;/' \| wc -c` (328,518 current); module-level `use m;` lines 20,506; block 139,242; glob 23,197; all `use` bytes 342,416 |
| Top modules | `echo "$E" \| cut -d'\|' -f1 \| sort \| uniq -c \| sort -rn \| head` (`E` is the expanded name list) |
| Issue's four modules | `git grep -hE "^use widgets::js_num(::\|;)" $R -- 'specs/*.t27' \| wc -l` and the same for the others |
| Total at older commits | `git grep -hE '^use ' <rev> -- 'specs/*.t27' \| wc -l` for `HEAD~5` (9955), `HEAD~10 = 4c7dbe4cd9` (9950), `HEAD~20` (9735), `HEAD~30` (9726) |
| Sets of modules per spec | `PM` = distinct `module\|spec` from `W` and `N2`; group by spec with the same grouping `sed`, then `cut -d'\|' -f2 \| sort \| uniq -c \| sort -rn` |
| Widgets kit | `echo "$PM" \| grep -E '^widgets::(html_out\|js_num\|js_val\|json_scan\|canvas_out)\|' \| cut -d'\|' -f2 \| sort \| uniq -c`, then `sed 's/^ *\([0-9]*\) .*/\1-1/' \| paste -sd+ - \| bc` over counts of 2 or more (65) |
| Duplicate names in the kit | `grep -hE '^(pub )?(fn\|const) ' specs/widgets/{html_out,js_num,js_val,json_scan,canvas_out}.t27 \| sed -E 's/^(pub )?(fn\|const) +([A-Za-z0-9_]+).*/\3/' \| sort \| uniq -d` (empty) |
| `imports_anchor` | `git grep -c 'fn imports_anchor' $R -- 'specs/*.t27' \| wc -l` (27) |
