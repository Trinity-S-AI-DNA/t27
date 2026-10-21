# lora-v1 corpus manifest

Authoritative inventory of hand-authored `.t27` source across
`/Users/playom/t27`, `/Users/playom/trinity`, `/Users/playom/trios`.

Built by `build.py` (see "Reproducing" below). Every number here came from an
actual run; none are estimates unless labelled `~`.

Outputs:
- `manifest.json` — JSON array, one object per kept file:
  `{path, repo, bytes, chars, dialect, sealed, sealExact, stubRatio, sha256}`
- `stats.json` — every count below, machine-readable, plus
  `logical_collision_detail` and `skipped_symlink_paths`.

---

## 1. Enumeration

`os.walk` over the three roots, `*.t27`, regular files only.

| | count |
|---|---|
| t27 | 2299 |
| trinity | 1551 |
| trios | 3 |
| **total** | **3853** |

3 additional paths were skipped as **symlinks / non-regular files**. All three
are the same broken self-referential link (`bootstrap/bootstrap/specs/physics/
formula_registry.t27 -> ../../specs/physics/formula_registry.t27`, which resolves
into a loop and raises `OSError: Too many levels of symbolic links`), once in the
main tree and twice inside worktrees. The link target itself
(`t27/specs/physics/formula_registry.t27`) IS in the manifest, so no content is lost.

## 2. Exclusion rule

Applied per file, **first match wins**, against the path relative to its repo root:

1. `worktrees` — any path with both a `.claude` and a `worktrees` path segment.
2. `specs_scratch` — any path containing consecutive segments `specs/scratch`.
3. `build_output` — path starts with `var/trinity/output/`, **or** any directory
   segment (excluding the filename) is one of:
   `target dist build node_modules generated out .next .cache coverage zig-out zig-cache .zig-cache`
4. `website_dist_mirror` — path starts with `apps/website/dist/`.
5. `website_public_mirror` — path starts with `public/` or contains `/public/`.

Removals:

| rule | removed |
|---|---|
| `worktrees` | 1623 |
| `website_dist_mirror` | 760 |
| `website_public_mirror` | 760 |
| `specs_scratch` | **0** |
| `build_output` | **0** |
| **total excluded** | **3143** |
| **strict survivors** | **710** |

`specs_scratch` shows 0 **not** because there is no scratch — there are 455
`specs/scratch/*.t27` files — but because every one of them is inside
`.claude/worktrees/` and was already removed by rule 1. Verified:
`grep '/specs/scratch/' all.txt | grep -vc '/.claude/worktrees/'` → `0`.
`build_output` shows 0 for the same reason: no `.t27` lives under a build dir
outside the excluded trees.

### Mirror rescue (deviation from a naive "delete the mirror" rule)

`trinity/apps/website/dist/` and `trinity/apps/website/public/` are two
byte-identical mirrors of a snapshot of the t27 repo root, 760 files each.
Deleting both outright loses real work: **185 of the 760 mirrored relative paths
exist nowhere else in the three roots**, and 154 of the 760 have content found
nowhere else. They come from sibling projects that were never checked out here:

| mirrored source tree | website-only paths | exists on disk? |
|---|---|---|
| `tri-net/specs` | 113 | no |
| `trinity-fpga/specs` + `/t27` + `/src` | 64 | yes, at `/Users/playom/trinity-fpga` (outside the 3 task roots) |
| `tt-trinity-corona/specs` | 5 | no |
| `trinity/src`, `trinity/t27` | 3 | yes |

So: `dist/` is dropped outright (exact duplicate of `public/`), and the 760
`public/t27/files/**` entries are **re-admitted into the dedup pool as
lowest-priority variants**. A mirror copy survives only if nothing else claims
its content hash or its logical path. **154 survived**; they carry
`repo: "trinity-website-mirror"` in the manifest so they can be filtered out in
one line if you want mirror-free training data.

## 3. Deduplication

Pool = 710 strict survivors + 760 mirror variants = **1470 variants**.

**Priority (lowest tuple wins):**
`(0 if sealExact else 1, is_mirror, -bytes, -mtime, repo_rank, path)`
with `repo_rank = t27:0, trinity:1, trios:2`.

**Stage A — exact content (sha256 of raw bytes):**

| | |
|---|---|
| distinct content hashes | 763 |
| hashes with >1 copy | 606 |
| removed | **707** |

**Stage B — logical path.** Logical path = path relative to repo root, then
strip a leading `apps/website/public/t27/files/`, then strip a leading `t27/`
(this is what folds the `trinity/t27/...` fork onto `t27/...`).

| | |
|---|---|
| distinct logical paths | 742 |
| logical paths with >1 surviving variant | **21** |
| removed | **21** |

All 21 collisions are the `trinity/t27/` fork vs. the canonical `t27/` tree
(20 kept from `t27`; `specs/numeric/tf3.t27` kept from `trinity/t27` because
that copy is larger). Full list in `stats.json → logical_collision_detail`.
No mirror copy ever won a logical collision — every mirrored file that had a
source-tree counterpart was byte-identical to it and collapsed in stage A.

**Final kept: 742 files, 7,826,827 bytes, 7,585,434 chars.**
By repo: `t27` 574, `trinity-website-mirror` 154, `trinity` 11, `trios` 3.

## 4. Dialect classifier

Regexes are multiline, applied to the whole decoded file, **in this order**;
first match wins.

| dialect | rule |
|---|---|
| `module-code` | `^[ \t]*(pub[ \t]+)?module[ \t]+[^\n;{]{1,120}?[ \t]*[;{]` |
| `tri27-asm` | `^[ \t]*\.(const\|data\|code\|text\|word\|dword\|tryte\|section\|org\|global)\b` |
| `spec-DSL` | `^[ \t]*spec[ \t]+[A-Za-z_][\w.]*[ \t]*(version[ \t]+\S+)?[ \t]*\{?[ \t]*$` |
| `algorithm-DSL` | `^[ \t]*algorithm[ \t]+[A-Za-z_]\w*[ \t]*\{` |
| `markdown-prose` | `^#{1,6}[ \t]\S` |
| `unknown` | none of the above |

The module-name pattern is deliberately loose (`[^\n;{]{1,120}?`, not an
identifier) because real files contain `module avs-reconf;` (hyphenated) and
`module []const u8;` (a corrupted name where a Zig type was substituted for the
module name). A strict identifier regex misclassified 96 module files as
`unknown`.

| dialect | all 742 | of the 276 sealExact |
|---|---|---|
| `module-code` | 670 | 252 |
| `spec-DSL` | 35 | 3 |
| `markdown-prose` | 19 | 15 |
| `tri27-asm` | 9 | 0 |
| `unknown` | 6 | 3 |
| `algorithm-DSL` | 3 | 3 |

### Independent check of the classifier

Ran `t27c parse` (`/Users/playom/t27/target/release/t27c`) on all 742 files and
grouped its `recovery-events` / `lexer-discarded-chars` counters by dialect
label. The labels separate almost perfectly on parse health:

| dialect | n | recovery-events == 0 | lexer-discarded == 0 |
|---|---|---|---|
| `module-code` | 670 | 615 (91.8%) | 669 (99.9%) |
| `spec-DSL` | 35 | 7 (20.0%) | 30 (85.7%) |
| `markdown-prose` | 19 | 0 (0.0%) | 1 (5.3%) |
| `tri27-asm` | 9 | 0 (0.0%) | 7 (77.8%) |
| `unknown` | 6 | 5 (83.3%) | 6 (100.0%) |
| `algorithm-DSL` | 3 | 0 (0.0%) | 3 (100.0%) |

`module-code` is the only dialect the compiler actually accepts. The other 72
files are, from the validator's point of view, malformed input.

The 6 `unknown` files, individually inspected: 2 copies of `formula_registry.t27`
and `specs/physics/lqg_entropy.t27` (const-only, no module header),
`contrib/backend/zig/legacy/main_zig_handwritten.t27` (real Zig source under a
`.t27` extension), `trinity/src/tri27/cache_w/tiny_lfu.t27` (assembly with no
section directives), and `vscode-trinity-swe/test_highlight.t27` which is
**0 bytes** — drop that one before training.

## 5. Seal cross-reference

`/Users/playom/t27/.trinity/seals/*.json` — 512 records, each
`{spec_path, spec_hash, gen_hash_{c,rust,verilog,zig}, module, ring, sealed_at}`.

**Hash algorithm reproduced.** `spec_hash` is literally
`"sha256:" + sha256(raw file bytes).hexdigest()`, no normalization. Verified by
recomputing all 512 against their `spec_path` under `/Users/playom/t27`:
**276 match, 236 mismatch, 0 missing files** — which reproduces the known
276 figure exactly, so the algorithm is confirmed, not assumed.

- `sealed` = the file's logical path appears in the seal index, **or** its
  content hash is a sealed hash. → **512 of 742**.
- `sealExact` = `"sha256:"+sha256` of the file's current bytes is one of the 512
  `spec_hash` values, i.e. the file is unchanged since sealing. → **276 of 742**.

## 6. stubRatio

For each file: find every `^[ \t]*(pub[ \t]+)?fn[ \t]+NAME[ \t]*(` , locate the
first `{` after it (skipping the declaration if a `;` appears before that `{`,
which means a bodyless signature), brace-match to the closing `}`, and take the
body. Strip `/* */` and `//` comments from the body. **A declaration is a stub
if the remaining text is empty.** `stubRatio = stubs / declarations-with-bodies`;
a file with zero such declarations gets `stubRatio = 0.0`.

| | |
|---|---|
| function declarations with bodies, total | 6494 |
| of those, stubs (empty after comment-strip) | 854 |
| of those stubs, carrying an explicit `TODO`/`FIXME`/`unimplemented` comment | 573 |
| files with `stubRatio > 0` | 198 |
| files with `stubRatio == 1.0` (every function stubbed) | 186 |
| files with **no** function declarations at all (`stubRatio` is 0.0 by convention, not by merit) | 139 |

The 139 zero-declaration files are the main caveat: `stubRatio == 0` alone is
not a quality filter, because a markdown file or a const-only spec also scores 0.
Filter on `dialect == "module-code" AND stubRatio == 0` to mean something.

**The seal is a real quality signal.** Files with at least one stub:
- sealExact (276 files): **20 → 7.2%**
- everything else (466 files): **178 → 38.2%**

## 7. Candidate training slices

| slice | files | bytes | chars | ~tokens (chars/4) |
|---|---|---|---|---|
| everything kept | 742 | 7,826,827 | 7,585,434 | ~1,896,358 |
| `module-code` | 670 | 6,666,892 | 6,489,863 | ~1,622,465 |
| `module-code` & `stubRatio==0` | 472 | 5,661,837 | 5,637,391 | ~1,409,347 |
| `module-code` & `sealExact` | 252 | 2,310,571 | 2,297,875 | ~574,468 |
| `module-code` & `sealExact` & `stubRatio==0` | 232 | 2,124,397 | 2,114,349 | ~528,587 |
| `sealExact`, any dialect | 276 | 2,470,701 | 2,457,919 | ~614,479 |

`~tokens` is a crude `chars/4` proxy, not a real tokenizer count. Treat it as
an order of magnitude only.

## Reproducing

`build.py` is pure stdlib python3, reads only, and writes only into this
directory. It re-derives every number above:

```
python3 build.py        # rewrites manifest.json and stats.json, prints stats
```

The `t27c parse` cross-check in section 4 is a separate one-off; it shells out to
`/Users/playom/t27/target/release/t27c parse <file>` per manifest entry and
greps `recovery-events:` / `lexer-discarded-chars:` from the output.
