# gen_w ports: what `build_tree` really is (row J of epic #12444)

Measured 2026-10-25 on branch `claude/t27-boilerplate-refactor-1f93fd`. Sources: the first
`fn build_tree*` signature of each `specs/port/scripts/gen_w*.t27` (59 files carry one), the
duplicate-bodies ledger `tools/duplicate_bodies_baseline.txt`, and per-spec `t27c test-report`
counts taken before and after the change.

## The premise of #12454 was wrong, and what we did instead

The issue assumed "about 51 ports carry a `build_tree` compatible with
`port::scripts::gen_w_tree::build_tree`". The signature table says otherwise:

| Signature (first `fn build_tree*` in the file) | Files | Same function as gen_w_tree? |
| --- | --- | --- |
| `build_tree(n: u32, inner_points: u32, base: u32) -> u32` | 30 (gen_w631-657, 662, 680, 681) | No: it counts points, it builds no tree |
| `build_tree_levels(remaining: u32) -> u32` | 16 (gen_w861-876) | No: a level counter |
| `build_tree(shape: []const i64, base: i64, offset: i64) -> TreeBuild` | 4 (gen_w706, 718, 726, and gen_w_tree itself) | Yes in signature; bodies differ, so not exact copies |
| `build_tree(shape: [7]u32, depth: u32, base: u32) -> u32` | 3 (gen_w660, 672, 692) | No: fixed rank 7, counts points |
| one-offs | gen_w669, 670, 682, 683, 731, 734 | No: six different shapes |

Only 3 of 59 share the signature, and none of those three is an exact body copy. Wiring the
30 counting copies to `gen_w_tree` would have changed what the ports compute. So row J adds a
second shared module for the function the 30 really are.

## What changed

- `specs/port/scripts/gen_w_flat.t27` (new, module `port::scripts::gen_w_flat`): one
  `pub fn build_tree(n, inner_points, base)`, 6 tests, 0 vacuous, sealed. Same name as the local
  copies, so no caller edit is needed. The local copies add `inner_points` n times in a loop;
  the home uses the closed form `base + n * inner_points`, which overflows `u32` at exactly the
  same inputs. Its body keeps a named intermediate on purpose: gen_w681 already holds the
  one-line form, and an identical body would have joined that group as a clone.
- 29 ports (gen_w631-657, 662, 680) drop their local `fn build_tree` and its comment, and gain
  `use port::scripts::gen_w_flat::build_tree;` placed before `module gen_wNNN {`.
  Diff: 29 files, +29 / -349 lines. Tests are untouched.
- Ledger: `build_tree 29,3` becomes `build_tree 3` (1851 duplicate bodies down to 1822,
  692 groups down to 691). The remaining 3 are gen_w660, 672, 692.

## Findings

1. A `use` line before a brace-form `module gen_wNNN { ... }` splices correctly. Verified on
   all 29: per-spec test counts are identical before and after (6 tests, 6 pass, 0 failed,
   0 vacuous each), and the same holds for the new home.
2. gen_w681 is BLOCKED at baseline (`spec.zig:24:26: error: expected type 'u32', found 'i32'`)
   and is not migrated. It was a different body anyway (the closed form). When it compiles
   again its copy goes the way of the other 29.
3. gen_w680 had no seal at baseline. It was left unsealed so the diff matches master.
4. Same-name helpers in the same ports show further copy groups: `flat_index` (gen_w706,
   gen_w726, gen_w_tree), `is_ascii` (gen_w706 and more) and the `indent` / coordinate helpers.
   They belong to `gen_w_tree` and are the next candidates for this row.

## Not done, by design

| Item | Why | Next step |
| --- | --- | --- |
| 16 `build_tree_levels` copies (gen_w861-876) | one body, but a different function; needs its own home | `gen_w_levels` module, same recipe, batches of at most 10 |
| 3 `[7]u32` copies (gen_w660, 672, 692) | fixed-rank variant | fold into `gen_w_flat` only after a signature check |
| gen_w706, 718, 726, 734 | same signature, different bodies | read the bodies; migrate only if a test-count match is shown |
| gen_w669, 670, 682, 683, 731 | one-off shapes | leave alone |

## Recipe (so the next loop does not re-derive it)

1. List the first `fn build_tree*` signature per file and group by signature, not by name.
2. Write the home with a body that is not text-equal to any existing body.
3. Record per-spec test counts before; delete the local copy and add one `use` line; record
   counts after; stop on any difference.
4. Seal the home and the ports; leave specs that had no seal unsealed.
5. Run `tools/dupe_scan.py`; bless only when the ledger diff shrinks a group and adds no NEW one.
