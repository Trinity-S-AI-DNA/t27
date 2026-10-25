# Boilerplate loops in specs: a sample of 326 hand `while` loops, by shape

Date: 2026-10-25. Epic gHashTag/t27#12444, row G (issue #12451). Written for the owner and for the
agents that pick the next std batch.

Scope: this is a measurement and a proposal. It changes no spec, no compiler and no language rule. The one
new spec that comes with it, `specs/tools/tri/loopshape.t27`, is an advisory rule (section 9): it is not a
gate, and no `tri loopshape` command exists.

## 1. Summary

At commit `d9f093ff2d` the `.t27` files under `specs/` hold **14671 lines that open a `while (` loop**, in
1146 of 2810 files, and **2059 lines that open a `for` loop**. A systematic sample of 326 of the `while`
loops was read by eye and put in one of six shapes:

| Shape            | Loops | Share  | +/- (95%) | Covered today by                                                              |
|------------------|------:|-------:|----------:|-------------------------------------------------------------------------------|
| Find first       |    60 |  18.4% |      4.2  | bytes: `std::mem` (14 of 60). Predicate scans: no function (46 of 60).         |
| All equal        |    56 |  17.2% |      4.1  | two byte runs: `std::mem` (24 of 56). Forall over a table: no function (32).   |
| Count            |    18 |   5.5% |      2.5  | one fixed byte: `std::mem` (4 of 18). Predicate or mismatch count: none (14).  |
| Fold             |    28 |   8.6% |      3.0  | none                                                                          |
| Copy             |    56 |  17.2% |      4.1  | none                                                                          |
| Other            |   108 |  33.1% |      5.1  | not a boilerplate shape                                                        |
| **Total**        |   326 | 100.0% |           |                                                                               |

Reading it:

- At most **12.9% (42 of 326)** of the sampled loops have the shape of a `std::mem` function (find a byte or
  substring, equality of two runs, count of one byte). It is an upper bound: it assumes the operands are
  byte slices, and some of the 42 loop over int arrays (section 7).
- **Copy has no library function at all** and is the largest named shape without one (17.2%). **Fold** has
  none either (8.6%).
- Find, all-equal and count have a function only for the case "bytes against one byte or one run". The
  same shapes with a predicate (a byte class, a table lookup, a custom condition) are 46 + 32 + 14 = 92
  loops, **28.2%**, and `std::text` gives them only the one-byte class test, not a scan, a forall or a count.
- A third (33.1%) is not boilerplate by this reading: state machines, tokenisers, nested drivers, binary
  search, heap sift, bignum, drivers of tests.

## 2. Totals (counted)

All commands run from the repo root, pinned to commit `d9f093ff2d` with `git grep` so that uncommitted edits
of other agents in the working tree cannot move a number. (HEAD later moved to `599e145a04`, which makes 16
widgets call `std::mem::bytes_eq`; at that commit the `while (` total is 14654, 17 fewer. The sample below was
drawn before it and stays pinned to `d9f093ff2d`.)

| What                                                      | Command (after `git grep ... d9f093ff2d -- 'specs/*.t27'`)       | Result |
|-----------------------------------------------------------|-------------------------------------------------------------------|-------:|
| `.t27` files under specs                                  | `git ls-tree -r --name-only d9f093ff2d -- specs \| grep -c '\.t27$'` | 2810 |
| lines that open a `while (` loop                          | `git grep -h -E '^[[:space:]]*while \(' ... \| wc -l`               | 14671  |
| files with at least one such line                         | `git grep -l -E '^[[:space:]]*while \(' ... \| wc -l`               | 1146   |
| lines that open a `for` loop (either form)                | `git grep -h -E '^[[:space:]]*for ' ... \| wc -l`                   | 2059   |
| ... of them `for (...) \|x\|` (Zig form)                  | `git grep -h -E '^[[:space:]]*for \(' ... \| wc -l`                 | 786    |
| ... of them `for X in ...` (range form)                   | `git grep -h -E '^[[:space:]]*for [^(]' ... \| wc -l`               | 1273   |
| ... of them with a range that starts at 0                 | `git grep -h -E '^[[:space:]]*for .* in 0\.\.' ... \| wc -l`        | 1146   |
| ... of the Zig form, a fixed `for (0..1000) \|_\| {`      | `git grep -h -F 'for (0..1000) \|_\| {' ... \| wc -l`              | 290    |
| `while (i < ` loops (loop variable named `i`)             | `git grep -h -E '^[[:space:]]*while \(i < ' ... \| wc -l`           | 4564   |
| ... of them exactly `while (i < n)`                       | `git grep -h -E '^[[:space:]]*while \(i < n\)' ... \| wc -l`        | 372    |

The full form of one row, as an example:

```
git grep -h -E '^[[:space:]]*while \(' d9f093ff2d -- 'specs/*.t27' | wc -l
```

Two things the counts do not say. First, a line that starts with `while (` is one loop; a `while` that sits
after another token on the same line (`} else { while (...`) is not counted. Second, 14671 counts
every loop, including the ones in `test` blocks and in tool-card test bodies, which is how the specs are
written and not a defect of the count.

The issue text quotes 4584 for the `while` loops over an index and 2059 for the `for` loops. The closest
literal readings of those at this commit are 4564 (`while (i < `) and 2059 (every line that opens a `for`),
so the second matches and the first is off by 20; it may have been taken at another commit or with another
pattern. This report uses its own numbers and its own commands.

## 3. How the sample was drawn (counted)

A systematic sample: every 45th line of the sorted list of all 14671 `while (` lines, so lines 45, 90, ...,
14670, which is `floor(14671 / 45) = 326` loops. The list is sorted by `path:line` in the C locale, so the
sample is spread over all directories in proportion to how many loops each holds.

```
git grep -n -E '^[[:space:]]*while \(' d9f093ff2d -- 'specs/*.t27' \
  | gsed -E 's/^d9f093ff2d://' | LC_ALL=C sort | gsed -n '0~45p'
```

`gsed` is GNU sed from Homebrew: BSD `sed` on macOS has no `first~step` address. The command prints 326
lines. To read the bodies of samples A to B (sample number = line number of this output), append:

```
  | gsed -n 'A,Bp' | gsed -E 's#^([^:]+):([0-9]+):.*#\2{=};\2,+6p\n\1#' \
  | xargs -n2 sh -c 'echo "@@ $1"; git show d9f093ff2d:$1 | gsed -n "$0"' | cut -c1-112
```

That prints, for each sample, the file name and the loop line plus the six lines after it.

Where the 326 samples fall: 259 distinct files; by top directory widgets 68, t27f 35, tri 33, t27os 27,
tools 20, port 20, analog 20, ml 13, fpga 10, claw 10, compiler 9, igla 8, the rest in smaller directories.

## 4. How a loop was labelled (judged by eye)

Each sample was read once, with six lines of body, and put in one of nine labels, then rolled up to the six
shapes. This is the part that is a judgement and not a count.

| Label | Shape      | Meaning                                                                                              |
|-------|------------|------------------------------------------------------------------------------------------------------|
| Fb    | Find first | first byte or substring equal to something; skip-while a byte equals b; trim                        |
| Fp    | Find first | first element that meets a predicate or a custom condition; table membership; reverse or last-occurrence search; nth delimiter |
| Ee    | All equal  | element-wise equality of two runs, or of a run and a literal; prefix, suffix, and the assert form `assert_eq(a[i], b[i])` |
| Ep    | All equal  | one forall: every element of a table meets a predicate, or is asserted to                           |
| Cb    | Count      | how many elements equal one value                                                                    |
| Cp    | Count      | how many meet a predicate; a mismatch count between two runs                                         |
| D     | Fold       | sum, parse a number, hash, maximum, any single accumulator                                           |
| P     | Copy       | copy, map, fill, zero, emit or append into an output buffer                                          |
| O     | Other      | state machines, tokenisers, binary search, heap sift, bignum, drivers, nested exhaustive test drivers |

Rules that decided the close calls:

- A loop that **writes an output** is Copy even when it also compares (a filter-copy).
- A loop that **leaves on the first hit** is Find; one that **leaves on the first failure** is All equal.
- A loop with a **nested inner loop** is Other. This rule was fixed late: samples 1 to 200 were revised to
  follow it (samples 72 and 132 moved to Other), but samples 201 to 326 were labelled before it was written
  down, so the boundary between All equal (forall) and Other is a little fuzzy there. Nested exhaustive
  loops (`for each a, for each b, assert ...`) in tool cards are the usual case.
- A loop that **steps by other than one**, or keeps more state than an index and one accumulator, is Other.
- A loop whose body is mostly side effects on a struct or a world (a simulation step, a scheduler) is Other.

The decision rule above, reduced to eight yes/no facts, is what `specs/tools/tri/loopshape.t27` writes down
(section 9). It was written after the labelling, to state the rule; it did not label the sample.

## 5. The table, with its sub-shapes

| Label | Loops | Share of 326 | Shape total (share)   |
|-------|------:|-------------:|-----------------------|
| Fb    |    14 |         4.3% | Find first: 60 (18.4%) |
| Fp    |    46 |        14.1% |                        |
| Ee    |    24 |         7.4% | All equal: 56 (17.2%)  |
| Ep    |    32 |         9.8% |                        |
| Cb    |     4 |         1.2% | Count: 18 (5.5%)       |
| Cp    |    14 |         4.3% |                        |
| D     |    28 |         8.6% | Fold: 28 (8.6%)        |
| P     |    56 |        17.2% | Copy: 56 (17.2%)       |
| O     |   108 |        33.1% | Other: 108 (33.1%)     |
| Sum   |   326 |       100.0% |                        |

The tally is recomputed from the label record in Appendix B with:

```
grep -E '^[0-9]+-[0-9]+:' docs/reports/boilerplate_loops_2026-10-25.md \
  | sed -E 's/^[^:]+: //' | tr ' ' '\n' | sort | uniq -c | sort -rn
```

Projected to all 14671 loops, with the point estimates only (the intervals in section 8 are wide): Find
about 2700, All equal about 2520, Copy about 2520, Fold about 1260, Count about 810, Other about 4860.
These are estimates from a sample, not counts.

## 6. What `std::mem` and `std::text` already cover

`specs/std/mem.t27` (row B) has: `bytes_eq`, `bytes_eq_prefix`, `bytes_starts_with`, `bytes_ends_with`,
`bytes_index_of` (answers -1 when absent), `bytes_contains`, `bytes_index_of_byte`, `bytes_count`,
`bytes_trim_start`, `bytes_trim_end`. `specs/std/text.t27` (row A) has the byte classes and folds:
`is_digit`, `is_upper`, `is_lower`, `is_alpha`, `is_alnum`, `is_word`, `is_space`, `is_hex`, `digit_value`,
`hex_value`, `to_lower`, `to_upper`. It has no scan, count or forall function. `specs/std/` holds nothing
else. No copy or fill helper exists anywhere in the tree: a grep for `bytes_copy`, `copy_bytes`, `memcpy`,
`fill`, `bytes_fill` and `mem_copy` found no function with those names.

| Shape      | Sub-shape (label)                      | Function today                                                  | Gap |
|------------|-----------------------------------------|-----------------------------------------------------------------|-----|
| Find first | byte or substring (Fb)                  | `bytes_index_of_byte`, `bytes_index_of`, `bytes_contains`, `bytes_trim_start`, `bytes_trim_end` | none, for byte slices |
| Find first | predicate, custom condition, last, nth (Fp) | `std::text` classes only                                    | no scan with a predicate; no last-index; no nth-delimiter |
| All equal  | two byte runs, prefix, suffix (Ee)      | `bytes_eq`, `bytes_eq_prefix`, `bytes_starts_with`, `bytes_ends_with` | none, for byte slices; the assert form over int tables is not a byte run |
| All equal  | forall predicate over a table (Ep)      | `std::text` classes only                                        | no forall |
| Count      | one fixed byte (Cb)                     | `bytes_count`                                                   | none, for byte slices |
| Count      | predicate, mismatch count (Cp)          | `std::text` classes only                                        | no count with a predicate; no mismatch count |
| Fold       | sum, parse, hash, maximum (D)           | none                                                            | **no function** |
| Copy       | copy, map, fill, zero, append (P)       | none                                                            | **no function** |
| Other      | (O)                                     | not boilerplate                                                 | none to fill |

**The shape with no library function: Copy (17.2%), then Fold (8.6%).** Together 25.8% of the loops, 84 of
326. Among the shapes `std::mem` does cover, the part it does not reach (predicate, forall and count with
a predicate) is 92 loops, 28.2%, and is the larger half: 92 loops against the 42 that are covered.

Not checked: whether t27 can pass a function or a predicate as an argument. If it cannot, the 92
predicate loops cannot be written as one library function taking a predicate; they would need one function
per class (`bytes_index_of_class`, and so on), and that is a design question for the batch that takes them.

## 7. Examples (sample number, file:line at `d9f093ff2d`, label)

Sample N is line N of the output of the command in section 3.

| Sample | Where                                       | Loop (first line)                                          | Label |
|-------:|---------------------------------------------|------------------------------------------------------------|-------|
|      5 | `specs/analog/mos_psp103_data.t27:8916`     | `while (i < 200) { assert_eq(r2k_n(c)[i], r2_n(c)[i]); ... }` | Ee: two runs, assert form, over number tables; `bytes_eq` does not cover it (not bytes) |
|      9 | `specs/analog/raw_writer.t27:649`           | `while (i < to and buf[i] >= 48 and buf[i] <= 57) { ... }` | Fp: skip-while a byte class; `std::text` has `is_digit`, there is no skip function |
|     17 | `specs/analog/sar_logic.t27:1248`           | `while (n < gt[GT_NNET]) { if (ms[SL_VAL + n] == SL_X) { unknown = unknown + 1; } ... }` | Cb: count of one value, over an int array; `bytes_count` is for byte slices, so not covered |
|     33 | `specs/claw/token_shape.t27:186`            | `while (text[je] != ':') { je += 1; }`                     | Fb: find the first `:`; `bytes_index_of_byte(text, 58)` is this loop with a bound check |

Two of the four examples are labelled with a `std::mem` shape and are not covered because the operands are
not byte slices. So the 12.9% in section 1 is a ceiling and the real covered share is lower. How much
lower was not counted.

## 8. Sampling error and what is counted versus judged

Counted (a command gives the same number every time): the totals in section 2, the 326 sample lines, the
label tally of section 5, and the percentages computed from it.

Judged by eye (another reader will differ): every label. Close calls were Ep against Other (a forall inside
a larger driver), Fp against Other (a scan that also updates state), and Cp against D (a counter used as an
accumulator). I did not label the sample twice and did not have a second reader, so I cannot give an
agreement figure.

Sampling error, 95% interval from the normal approximation for a proportion with n = 326, in percentage
points: Other +/- 5.1, Find +/- 4.2, All equal +/- 4.1, Copy +/- 4.1, Fold +/- 3.0, Count +/- 2.5. Treat
these as a **lower bound** on the uncertainty, for four reasons:

- The sample is systematic over a sorted list, not simple random. Loops in one file are written alike, and
  259 files hold the 326 loops; clustering widens the true interval.
- Tool-card files and widget files contain many near-duplicate loops (tool cards share one test template;
  68 of the 326 samples are in `specs/widgets/`). They push Other and Ep up, and they would move if a
  neighbouring file were drawn. Commit `599e145a04` already moved 16 widgets to `std::mem::bytes_eq`, so
  the Ee share is a "before" figure for widgets.
- The nesting rule was applied unevenly (section 4).
- The labels are one reader's judgement.

The conclusions that survive all of this: Copy and Fold have no function and together are about a quarter of
the loops; the predicate versions of find, forall and count are as large as everything `std::mem` covers or
larger; a third of the loops are not boilerplate. The ordering of Find, All equal and Copy against each other
(each 17% to 18%) does not survive: their intervals overlap.

Not covered by this sample: the 2059 `for` loops, which were counted and not classified; `while` loops that
do not open their line with `while (`; any spec outside `specs/` (the Rust and Zig that `t27c` emits).

## 9. The advisory rule: `specs/tools/tri/loopshape.t27`

The file states the decision rule of section 4 as a pure t27 function, and the second table (which std
function covers a shape) as another. It is advisory: it refuses nothing, it is not a gate, and nothing runs
it yet. It is a rule a reviewer can read or a later scanner can load.

- `loop_shape(steps_by_one, plain_body, writes_output, leaves_on_first_hit, leaves_on_first_mismatch,
  compares_two_runs, bumps_a_counter, folds_a_value)` answers one of six shape codes; the first rule that
  matches wins, and a loop that is nested, steps by other than one, or shows no feature is Other.
- `std_cover(shape, element_test)` answers which family covers the loop (`std::mem` equality, find, count,
  the `std::text` class only, or none); `suggests_rewrite(cover)` is true only when a whole function covers it.
- Its `test` blocks carry negative controls: a find is not an all-equal check, a mismatch count between two
  runs is a count and not an equality, a nested loop is never called a copy.

**Not verified:** I was told not to run `t27c`, so the spec has not been compiled or tested by me. The
success criterion is `t27c test-report specs/tools/tri/loopshape.t27` with no vacuous pass; it is for the
session that commits to run. It lives in `specs/tools/tri/` because the brief names that path; that directory
otherwise holds one clap tool card per `tri` command, and I did not verify that no catalog or website
generator enumerates every `.t27` file there and expects a card (a grep for code that globs `tools/tri` found
nothing, which is not proof).

## 10. A separate issue, not made here: range-loop ergonomics

Proposal only; no language change is made by this report, and I have not checked what the compiler accepts.

Of the 4564 `while (i < ` loops, the plain counted ones (start at 0, step by 1, bound known) are the
candidates for the range form `for i in 0..n`, which the specs already use 1146 times, and for the
Zig form `for (a..b) |x|`, used 786 times. The report cannot say how many of the 4564 are plain counted
loops without reading each body, and the sample was not drawn for that question. A separate issue could:

1. Measure it: of the loops that `loop_shape` calls steps_by_one and plain, how many have a loop variable
   the body does not otherwise write.
2. Check whether the range form works in every place the `while` form is used here: early `return` inside
   the loop, nested loops, `test` blocks, and the specs that lower to Verilog.
3. If a gap shows (a form that cannot return early, say), file that as a language issue with a measured
   count, and only then ask the owner for a change.

Moving a loop to a range form removes the counter declaration and the increment line. It does not remove a
copy of a shape: the loops that are `std::mem` shapes (at most 12.9%) are better served by a call than by a
shorter loop.

## Appendix A. Reproduce

```
git grep -n -E '^[[:space:]]*while \(' d9f093ff2d -- 'specs/*.t27' \
  | gsed -E 's/^d9f093ff2d://' | LC_ALL=C sort | gsed -n '0~45p'
```

must print 326 lines. Line N is sample N in Appendix B. The line numbers inside the files are those of
`d9f093ff2d`; at a later commit use `git show d9f093ff2d:<path>` to read the same file.

## Appendix B. Label record (sample index in sorted order : label)

1-28: Cp D O O Ee D P O Fp O P D O O O Ep Cb O O O Ep Ep O P O Cp O Ee
29-56: D P Cp P Fb Fp O Fp O O O O Ep Ep Cp Ep Ee Ee Ee O O O P O O O O D
57-84: D Ep Fp Ee O Fp P O D Ee O O Ep Ep Cp O Fp Ep O Fp O O P O P D P O
85-112: D Ep O P Fp D P D Ep O O O O Ep D D D O P O O Cp Fp Cb Fp Fb P P
113-140: Ee Fp P Ee Ee Fp Ep Ep O Fb Fp Fb P Ep P D P O Ep O P Cb Fp P P O P O
141-168: Ep O Ee O O D Ep O O O Ep O O O Ep Ep P D Cp Cp Cp O Cb Fp D O Cp Ep
169-200: O Ep Ep Fp Ep O O Fp O D Ee O Fp Fp Cp Fb Fb P O P O P Fp P P Fp Ee O Fb D P D
201-225: O O Fp O Ep Fp Ee Fb Fp Fp O P O Fp O Ep O Ep O Ep O Fp O P Ee
226-250: P Ee O Fp Fb Fb P Cp Ep O O Cp O O D O Fb O O Ep O P Fp P Ee
251-258: Fb O O Fb D Ee D Cp
259-284: O P O P Fp O Ee O O Fp O Fp O Ee O P P D Fp O Fb P P P O Fp
285-310: Fp D Fp Fp O P P O Fp O Fp P P P O Fp O Fp D P P O P O P Ee
311-326: D O Fp Fp P O P Fp Ee Fp Ee O P Fp Ee P
