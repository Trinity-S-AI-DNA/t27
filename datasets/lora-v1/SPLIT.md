# lora-v1 train / test split

Produced by the split pass over `manifest.json` (742 files). Every number below
came from a command that was actually run; the raw leakage-check transcript is
pasted verbatim in section 5.

```
test     72 specs      727799 bytes
train   669 specs     7099028 bytes
dropped   1 specs           0 bytes
total   742           7826827 bytes   (== manifest.json)
```

`split.json` is the authority. It is `{"test":[abs paths], "train":[abs paths],
"dropped":[{path, reason}]}` and the three lists are disjoint and cover the
manifest exactly (check 4 below).

---

## 1. Eligibility

One file was removed from the pool before any stratification:

| path | reason |
|---|---|
| `t27/vscode-trinity-swe/test_highlight.t27` | zero-byte file (0 bytes on disk): nothing to train on and nothing to score against |

741 files remained eligible.

## 2. Stratification

Target size: 60-80. Actual: **72**.

**Dimension 1 - dialect.** Test quotas are proportional to the whole 742-file
corpus, not to whatever happened to be left after filtering, so the test set
mirrors the corpus mix rather than the leftovers:

| dialect | corpus | corpus share | test | test share |
|---|---:|---:|---:|---:|
| `module-code` | 670 | 90.3% | 63 | 87.5% |
| `spec-DSL` | 35 | 4.7% | 4 | 5.6% |
| `markdown-prose` | 19 | 2.6% | 2 | 2.8% |
| `tri27-asm` | 9 | 1.2% | 1 | 1.4% |
| `unknown` | 6 | 0.8% | 1 | 1.4% |
| `algorithm-DSL` | 3 | 0.4% | 1 | 1.4% |

Every dialect that exists in the corpus is present in the test set, including
`algorithm-DSL`, which has only 3 files corpus-wide (1 held out, 2 left in train).
This is deliberate: the model will be asked to write whichever dialect the
instruction implies, and an eval that only contains `module-code` cannot detect
the known failure mode of the model emitting the wrong dialect.

**Dimension 2 - size quartile.** Quartile cuts were computed over the eligible
`module-code` files only (`module-code` is 90.3% of the corpus; the other 72
files are too few to quartile meaningfully):

```
Q1  <    3024 bytes
Q2    3024 -   5803 bytes
Q3    5804 -  13972 bytes
Q4  >=  13973 bytes
```

| module-code stratum | corpus | test | test bytes min / median / max |
|---|---:|---:|---|
| Q1 | 167 | 16 | 148 / 1370 / 2964 |
| Q2 | 168 | 12 | 3036 / 4562 / 5697 |
| Q3 | 167 | 17 | 6252 / 10877 / 13470 |
| Q4 | 168 | 18 | 14351 / 20164 / 48469 |

Within each stratum the candidates were sorted by byte size and cut into N equal
sub-bins (N = that stratum's quota), and exactly one spec was taken from each
sub-bin. That forces the held-out sizes to spread across the whole width of the
quartile instead of clustering at one end.

## 3. Candidate filters (why a spec was eligible to be held out)

Applied as hard filters before the quota was filled. Supply after filtering was
sufficient in every stratum, so no filter had to be relaxed.

| filter | why |
|---|---|
| **has no pair in `pairs/batch-*.jsonl` yet** | The pair-writing pass had already emitted 288 records covering 280 paths when this split was built. Choosing only unpaired specs is what makes leakage check 1 come out at literally zero instead of "zero after you filter". |
| **no near-duplicate anywhere in the corpus** (max 5-gram Jaccard against all 741 other files is below the MinHash candidate floor, i.e. no detectable twin) | The corpus forks heavily. A test spec with a 0.9-similar twin left in train is not held out in any meaningful sense - the model can copy the twin. This filter is what drives the measured max test/train similarity down to **0.4049**. |
| **`stubRatio < 1.0`** | 186 of 742 files have *every* function body empty. They parse clean (94.1% all-clean, measured) precisely because an empty body cannot have a type error, so they are free wins for a generator and a worthless reference to score against. They stay in the training pool; they are not valid eval targets. |

Ranking inside each size sub-bin, first key wins:

1. `sealExact == true` - the file still hashes byte-for-byte to its `.trinity/seals/` record, so the gold reference is known-unmodified since sealing.
2. `stubRatio == 0` - no empty function bodies at all.
3. `blake2b("t27-lora-v1" + path)` - a fixed seed, so the whole selection is reproducible.

Result: 21 of 72 test specs (29.2%) are `sealExact`, against 276 of 742 (37.2%) corpus-wide.
The rate is not higher because `sealExact` is rare among files that also survive
the near-duplicate filter - the seal-exact population is full of the small
template-clone specs the near-duplicate filter is designed to reject.

## 4. The test set is not the easy end of the corpus

`t27c` was run over all 741 eligible files (`parse` for `recovery-events` and
`lexer-discarded-chars`, `typecheck --json` for `errors`). Note the trap from the
earlier investigation still applies - `typecheck --json` alone returns `ok:true`
on garbage, so it is only meaningful alongside the parse counters.

```
                                   n   rec==0   disc==0   tcerr==0   all-three-clean
TEST  all                         72    79.2%    97.2%     77.8%      56.9%
TEST  module-code only            63    85.7%   100.0%     74.6%      60.3%
TEST  module-code, stubRatio<1    63    85.7%   100.0%     74.6%      60.3%
TRAIN all                        669    85.1%    96.4%     84.9%      71.2%
TRAIN module-code only           607    92.4%    99.8%     83.4%      77.1%
TRAIN module-code, stubRatio<1   421    91.7%    99.8%     76.0%      69.6%
```

The test set is **harder** than the training pool on every counter. The
like-for-like comparison is the third row of each block: test `module-code`
is 60.3% all-clean against 69.6% for the comparable training slice. The gap is
the `stubRatio == 1.0` files, which are 94.1% all-clean and which the test set
excludes. Score the model **gold-relative against these numbers**, not against 100%.

## 5. Leakage proof (raw output)

Re-run at the end of the pass, after the last pair batch had landed. Verbatim:

```
======================================================================
CHECK 1 - no TEST spec path appears in any pairs/batch-*.jsonl
pair files: 11   pair records: 288   distinct meta.path: 280
meta.path in TEST set: 0
test path string appearing anywhere in raw pair text: 0 []
test path (repo-relative form) appearing anywhere in raw pair text: 0 []
RESULT: PASS

======================================================================
CHECK 2 - no TEST spec's exact content appears as any pair's output
exact sha256 matches: 0    whitespace-normalised sha256 matches: 0
a TEST file appearing verbatim as a substring of a pair output: 0
RESULT: PASS

======================================================================
CHECK 2b - shingle similarity TEST file vs every pair OUTPUT (5-gram Jaccard + containment)
computed in 1.6s; pair-outputs x test-files compared: 20736
max Jaccard 0.3214   max containment 0.6341
top 8:
  J=0.321 cont=0.486  TEST t27/specs/github/issues.t27  vs pair batch-6.jsonl:12 (from t27/specs/github/prs.t27)
  J=0.306 cont=0.634  TEST t27/specs/tri/agent/faculty_board.t27  vs pair batch-7.jsonl:23 (from t27/specs/tri/utils/error.t27)
  J=0.294 cont=0.471  TEST t27/specs/tri/utils/help.t27  vs pair batch-7.jsonl:23 (from t27/specs/tri/utils/error.t27)
  J=0.263 cont=0.487  TEST t27/chips/phi/specs/fpga/int4.t27  vs pair batch-5.jsonl:23 (from t27/chips/phi/specs/fpga/int8.t27)
  J=0.254 cont=0.464  TEST t27/specs/tri/utils/help.t27  vs pair batch-3.jsonl:21 (from t27/specs/tri/utils/exit_codes.t27)
  J=0.253 cont=0.634  TEST t27/specs/sacred/sacred_governance.t27  vs pair batch-7.jsonl:23 (from t27/specs/tri/utils/error.t27)
  J=0.231 cont=0.445  TEST t27/specs/tri/agent/handoff.t27  vs pair batch-7.jsonl:23 (from t27/specs/tri/utils/error.t27)
  J=0.228 cont=0.477  TEST t27/specs/sandbox/session_timeout.t27  vs pair batch-7.jsonl:9 (from t27/specs/sandbox/orphan_detection.t27)
RESULT: PASS

======================================================================
CHECK 3 - near-duplicate TEST vs TRAIN, exact all-pairs 5-gram Jaccard
exhaustive comparisons: 72 test x 669 train = 48168, in 2.7s
pairs with Jaccard >= 0.85 : 0
pairs with Jaccard >= 0.70 : 0
pairs with Jaccard >= 0.60 : 0
pairs with Jaccard >= 0.50 : 0
MAX test/train Jaccard = 0.4049
top 10 most-similar TEST/TRAIN pairs:
  0.4049  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/tri/utils/string.t27
  0.4049  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/tri/utils/arrow_time.t27
  0.4049  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/tri/agent/memory.t27
  0.4049  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/tri/agent/experience_hooks.t27
  0.4049  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/tri/agent/agents.t27
  0.4049  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/tri/agent/agent_run.t27
  0.4049  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/sacred/quantum_gravity.t27
  0.4049  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/sacred/dark_matter.t27
  0.3883  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/sacred/sacred_constants.t27
  0.3822  TEST t27/specs/tri/utils/help.t27                               TRAIN t27/specs/tri/pipeline/codegen.t27
RESULT: PASS

======================================================================
CHECK 4 - partition integrity
manifest 742 | test 72 + train 669 + dropped 1 = 742
overlap test&train: 0 | test&dropped: 0 | train&dropped: 0
covers manifest exactly: True
duplicate sha256 between test and train (exact content twins):
   0
RESULT: PASS
```

### What each check does

- **Check 1** compares `meta.path` of every pair record against the test list,
  *and* greps the raw JSONL text for each test path in both absolute and
  repo-relative form - so a test path smuggled into an `instruction` string or a
  code comment would still be caught.
- **Check 2** hashes every pair `output` (raw, and again with whitespace
  collapsed) against the sha256 of every test file, then does a verbatim
  substring test in case a test file were embedded inside a larger output.
- **Check 2b** is the fuzzy version of check 2: 5-gram token shingles of each
  pair output against each test file, Jaccard and containment. The top hit is a
  different spec in the same directory family (`github/issues` vs `github/prs`)
  at J=0.321 - that is normal shared house style, not leakage.
- **Check 3** is exhaustive, not sampled: all 72 x 669 = 48168 test/train
  comparisons, exact set Jaccard on 5-gram token shingles, no MinHash
  approximation. **Zero pairs at or above 0.85. Zero above 0.50. Max 0.4049.**
- **Check 4** proves the three lists partition the manifest and that no test
  spec shares a content sha256 with a training spec.

### Similarity metric

Tokens are `[A-Za-z_][A-Za-z0-9_]*` / `\d+` / single non-word chars. Shingle =
5 consecutive tokens joined by a space, hashed with `blake2b(digest_size=8)`.
A document is the *set* of its shingle hashes; similarity is
`|A n B| / |A u B|`. Pure stdlib, no dependencies. The corpus-wide
near-duplicate sweep that drives the section-3 filter used 128-permutation
MinHash to generate candidates at an estimated-Jaccard floor of 0.55, then
recomputed every candidate exactly; the final test/train check in section 5
skips MinHash entirely and compares every pair exactly.

### Known residual risk

The pair-writing pass is a separate agent and may still be running. This split
holds out 72 paths that had no pair at the time of writing, and check 1 above was
re-run after the last batch landed. **If more batches are written later, they
must skip the test list.** Re-verify with:

```
python3 - <<'EOF'
import json,glob
t=set(json.load(open("/Users/playom/t27/datasets/lora-v1/split.json"))["test"])
h=[(f,i) for f in sorted(glob.glob("/Users/playom/t27/datasets/lora-v1/pairs/batch-*.jsonl"))
     for i,l in enumerate(open(f),1) if l.strip() and json.loads(l)["meta"]["path"] in t]
print("test paths leaked into pairs:",len(h)); print(h[:20])
EOF
```

## 6. The held-out set, and why each spec was chosen

`seal` = `sealExact` (gold still matches its `.trinity/seals/` hash).
`maxJ` = highest 5-gram Jaccard to any *other* file in the whole 742-file corpus;
`-` means no twin was detected above the 0.55 MinHash candidate floor, which was
a hard requirement for selection. `rec`/`disc`/`tc` are the `t27c` counters on
the gold file itself.

| # | spec | dialect | stratum | bytes | seal | stub | maxJ | rec | disc | tc |
|---:|---|---|---|---:|:-:|---:|:-:|---:|---:|---:|
| 1 | `trinity/apps/website/public/t27/files/trinity-fpga/t27/compiler/codegen/c/codegen.t27` | module-code | module-code Q1, bin 1/16 | 148 | . | 0.00 | - | 1 | 0 | 0 |
| 2 | `t27/tests/ring0_trivial.t27` | module-code | module-code Q1, bin 2/16 | 201 | . | 0.00 | - | 0 | 0 | 0 |
| 3 | `t27/specs/demos/simple_test.t27` | module-code | module-code Q1, bin 3/16 | 277 | Y | 0.00 | - | 0 | 0 | 0 |
| 4 | `trinity/apps/website/public/t27/files/trinity-fpga/t27/specs/fpga/mac.t27` | module-code | module-code Q1, bin 4/16 | 289 | . | 0.00 | - | 1 | 0 | 0 |
| 5 | `t27/specs/github/comments.t27` | module-code | module-code Q1, bin 5/16 | 398 | Y | 0.00 | - | 0 | 0 | 0 |
| 6 | `t27/specs/github/issues.t27` | module-code | module-code Q1, bin 6/16 | 484 | Y | 0.00 | - | 0 | 0 | 0 |
| 7 | `t27/specs/sandbox/modules.t27` | module-code | module-code Q1, bin 7/16 | 536 | Y | 0.00 | - | 2 | 0 | 0 |
| 8 | `trinity/apps/website/public/t27/files/tri-net/specs/link_statistics.t27` | module-code | module-code Q1, bin 8/16 | 1068 | . | 0.00 | - | 0 | 0 | 0 |
| 9 | `t27/specs/tri/utils/help.t27` | module-code | module-code Q1, bin 9/16 | 1370 | . | 0.00 | - | 0 | 0 | 0 |
| 10 | `trinity/apps/website/public/t27/files/tri-net/specs/frame_buffer.t27` | module-code | module-code Q1, bin 10/16 | 1888 | . | 0.00 | - | 0 | 0 | 0 |
| 11 | `trinity/apps/website/public/t27/files/tri-net/specs/rti_alert.t27` | module-code | module-code Q1, bin 11/16 | 2204 | . | 0.00 | - | 0 | 0 | 0 |
| 12 | `t27/specs/tri/agent/handoff.t27` | module-code | module-code Q1, bin 12/16 | 2325 | . | 0.00 | - | 0 | 0 | 0 |
| 13 | `t27/specs/tri/agent/faculty_board.t27` | module-code | module-code Q1, bin 13/16 | 2412 | . | 0.00 | - | 0 | 0 | 0 |
| 14 | `trinity/apps/website/public/t27/files/tri-net/specs/twr_timestamp.t27` | module-code | module-code Q1, bin 14/16 | 2563 | . | 0.00 | - | 0 | 0 | 0 |
| 15 | `trinity/apps/website/public/t27/files/tri-net/specs/crc16.t27` | module-code | module-code Q1, bin 15/16 | 2731 | . | 0.00 | - | 0 | 0 | 0 |
| 16 | `t27/specs/tutorial/07_tests_invariants_benches.t27` | module-code | module-code Q1, bin 16/16 | 2964 | . | 0.00 | - | 0 | 0 | 0 |
| 17 | `t27/specs/sacred/sacred_governance.t27` | module-code | module-code Q2, bin 1/12 | 3036 | . | 0.00 | - | 0 | 0 | 0 |
| 18 | `trinity/apps/website/public/t27/files/tri-net/specs/tri_fec.t27` | module-code | module-code Q2, bin 2/12 | 3071 | . | 0.00 | - | 0 | 0 | 0 |
| 19 | `trinity/apps/website/public/t27/files/tri-net/specs/account_identity.t27` | module-code | module-code Q2, bin 3/12 | 3441 | . | 0.00 | - | 0 | 0 | 0 |
| 20 | `t27/specs/github/tests/e2e_full_flow.t27` | module-code | module-code Q2, bin 4/12 | 3778 | Y | 0.00 | - | 0 | 0 | 0 |
| 21 | `trinity/apps/website/public/t27/files/tri-net/specs/wire.t27` | module-code | module-code Q2, bin 5/12 | 4189 | . | 0.00 | - | 0 | 0 | 0 |
| 22 | `trinity/apps/website/public/t27/files/tri-net/specs/modem_frame.t27` | module-code | module-code Q2, bin 6/12 | 4409 | . | 0.00 | - | 0 | 0 | 0 |
| 23 | `t27/specs/tutorial/04_control_flow.t27` | module-code | module-code Q2, bin 7/12 | 4562 | . | 0.00 | - | 0 | 0 | 0 |
| 24 | `t27/specs/isa/ternary_tree.t27` | module-code | module-code Q2, bin 8/12 | 4645 | Y | 0.00 | - | 0 | 0 | 1 |
| 25 | `trinity/apps/website/public/t27/files/tri-net/specs/etx.t27` | module-code | module-code Q2, bin 9/12 | 4933 | . | 0.00 | - | 0 | 0 | 0 |
| 26 | `t27/specs/tri/agent/swarm_agents.t27` | module-code | module-code Q2, bin 10/12 | 5212 | . | 0.00 | - | 0 | 0 | 0 |
| 27 | `trinity/t27/compiler/parser/language.t27` | module-code | module-code Q2, bin 11/12 | 5263 | . | 0.00 | - | 0 | 0 | 0 |
| 28 | `trinity/apps/website/public/t27/files/tri-net/specs/performance_benchmarks.t27` | module-code | module-code Q2, bin 12/12 | 5697 | . | 0.00 | - | 0 | 0 | 0 |
| 29 | `t27/specs/sandbox/session_timeout.t27` | module-code | module-code Q3, bin 1/17 | 6252 | Y | 0.00 | - | 1 | 0 | 0 |
| 30 | `t27/specs/git/status.t27` | module-code | module-code Q3, bin 2/17 | 6535 | Y | 0.77 | - | 0 | 0 | 0 |
| 31 | `t27/compiler/codegen/c/codegen.t27` | module-code | module-code Q3, bin 3/17 | 7080 | Y | 0.00 | - | 1 | 0 | 0 |
| 32 | `trinity/apps/website/public/t27/files/tt-trinity-corona/specs/corona/rom_layout.t27` | module-code | module-code Q3, bin 4/17 | 7148 | . | 0.00 | - | 0 | 0 | 0 |
| 33 | `trinity/apps/website/public/t27/files/tri-net/specs/tri_merkle.t27` | module-code | module-code Q3, bin 5/17 | 8139 | . | 0.00 | - | 0 | 0 | 0 |
| 34 | `t27/chips/phi/specs/fpga/int4.t27` | module-code | module-code Q3, bin 6/17 | 8538 | . | 0.00 | - | 0 | 0 | 0 |
| 35 | `trinity/apps/website/public/t27/files/tri-net/specs/congestion_control.t27` | module-code | module-code Q3, bin 7/17 | 9211 | . | 0.00 | - | 0 | 0 | 1 |
| 36 | `trinity/apps/website/public/t27/files/tri-net/specs/adaptive_routing.t27` | module-code | module-code Q3, bin 8/17 | 10004 | . | 0.00 | - | 0 | 0 | 4 |
| 37 | `t27/specs/test_framework/core.t27` | module-code | module-code Q3, bin 9/17 | 10877 | Y | 0.00 | - | 2 | 0 | 0 |
| 38 | `trinity/apps/website/public/t27/files/tri-net/specs/local_processing.t27` | module-code | module-code Q3, bin 10/17 | 11012 | . | 0.00 | - | 0 | 0 | 4 |
| 39 | `trinity/apps/website/public/t27/files/tri-net/specs/trust_manager.t27` | module-code | module-code Q3, bin 11/17 | 11176 | . | 0.00 | - | 0 | 0 | 8 |
| 40 | `t27/compiler/runtime/runtime.t27` | module-code | module-code Q3, bin 12/17 | 11466 | Y | 0.00 | - | 1 | 0 | 0 |
| 41 | `trinity/apps/website/public/t27/files/tri-net/specs/tri_compute_bond.t27` | module-code | module-code Q3, bin 13/17 | 12122 | . | 0.00 | - | 0 | 0 | 0 |
| 42 | `trinity/apps/website/public/t27/files/tri-net/specs/cross_layer_optimizer.t27` | module-code | module-code Q3, bin 14/17 | 12555 | . | 0.00 | - | 0 | 0 | 10 |
| 43 | `t27/specs/math/phi_universal_attractor.t27` | module-code | module-code Q3, bin 15/17 | 12924 | Y | 0.00 | - | 0 | 0 | 0 |
| 44 | `trinity/apps/website/public/t27/files/tri-net/specs/test_framework.t27` | module-code | module-code Q3, bin 16/17 | 13256 | . | 0.00 | - | 0 | 0 | 4 |
| 45 | `trinity/apps/website/public/t27/files/tri-net/specs/tri_a2a_card.t27` | module-code | module-code Q3, bin 17/17 | 13470 | . | 0.00 | - | 0 | 0 | 0 |
| 46 | `trinity/apps/website/public/t27/files/tri-net/specs/resource_scheduler.t27` | module-code | module-code Q4, bin 1/18 | 14351 | . | 0.00 | - | 0 | 0 | 1 |
| 47 | `trinity/apps/website/public/t27/files/tri-net/specs/tri_compute_reputation.t27` | module-code | module-code Q4, bin 2/18 | 14508 | . | 0.00 | - | 0 | 0 | 0 |
| 48 | `t27/chips/phi/specs/fpga/sparse_skip.t27` | module-code | module-code Q4, bin 3/18 | 15117 | . | 0.00 | - | 0 | 0 | 3 |
| 49 | `trinity/apps/website/public/t27/files/tri-net/specs/auto_config.t27` | module-code | module-code Q4, bin 4/18 | 16119 | . | 0.00 | - | 0 | 0 | 9 |
| 50 | `t27/specs/tools/schema.t27` | module-code | module-code Q4, bin 5/18 | 16580 | Y | 0.00 | - | 2 | 0 | 0 |
| 51 | `trinity/apps/website/public/t27/files/tri-net/specs/tri_gft_ladder.t27` | module-code | module-code Q4, bin 6/18 | 17128 | . | 0.00 | - | 0 | 0 | 0 |
| 52 | `t27/specs/fpga/testbench/mac_tb.t27` | module-code | module-code Q4, bin 7/18 | 17571 | Y | 0.00 | - | 0 | 0 | 5 |
| 53 | `trinity/apps/website/public/t27/files/tri-net/specs/integration_framework.t27` | module-code | module-code Q4, bin 8/18 | 18981 | . | 0.00 | - | 0 | 0 | 2 |
| 54 | `t27/specs/runtime/instance.t27` | module-code | module-code Q4, bin 9/18 | 20135 | Y | 0.00 | - | 1 | 0 | 0 |
| 55 | `t27/chips/phi/specs/numeric/gf256.t27` | module-code | module-code Q4, bin 10/18 | 20164 | . | 0.00 | - | 0 | 0 | 0 |
| 56 | `t27/chips/phi/specs/fpga/holo_mux_x4.t27` | module-code | module-code Q4, bin 11/18 | 21272 | . | 0.00 | - | 0 | 0 | 1 |
| 57 | `t27/specs/vsa/sdk.t27` | module-code | module-code Q4, bin 12/18 | 24071 | . | 0.00 | - | 0 | 0 | 0 |
| 58 | `trinity/apps/website/public/t27/files/tri-net/specs/tri_compute_account.t27` | module-code | module-code Q4, bin 13/18 | 24509 | . | 0.00 | - | 0 | 0 | 0 |
| 59 | `t27/specs/vsa/vsa_core.t27` | module-code | module-code Q4, bin 14/18 | 29297 | . | 0.00 | - | 0 | 0 | 17 |
| 60 | `t27/specs/demos/jones_topology_decision_gate.t27` | module-code | module-code Q4, bin 15/18 | 29767 | Y | 0.00 | - | 0 | 0 | 6 |
| 61 | `t27/chips/phi/specs/numeric/tri_net_formats.t27` | module-code | module-code Q4, bin 16/18 | 32170 | . | 0.00 | - | 0 | 0 | 4 |
| 62 | `t27/specs/ternary/bigint.t27` | module-code | module-code Q4, bin 17/18 | 43916 | . | 0.00 | - | 0 | 0 | 0 |
| 63 | `trinity/apps/website/public/t27/files/tri-net/specs/tri_a2a.t27` | module-code | module-code Q4, bin 18/18 | 48469 | . | 0.00 | - | 0 | 0 | 0 |
| 64 | `t27/specs/isa/ternary_encoding.t27` | algorithm-DSL | algorithm-DSL, bin 1/1 | 944 | Y | 0.00 | - | 1 | 0 | 0 |
| 65 | `t27/specs/benchmarks/ternary_vs_binary.t27` | markdown-prose | markdown-prose, bin 1/2 | 1965 | Y | 0.00 | - | 2 | 36 | 0 |
| 66 | `trinity/src/tri27/vsa_cosine.t27` | tri27-asm | tri27-asm, bin 1/1 | 2131 | . | 0.00 | - | 3 | 0 | 0 |
| 67 | `t27/specs/physics/lqg_entropy.t27` | unknown | unknown, bin 1/1 | 2841 | Y | 0.00 | - | 0 | 0 | 0 |
| 68 | `trinity/apps/website/public/t27/files/trinity-fpga/specs/numeric/published_pack_audit.t27` | spec-DSL | spec-DSL, bin 1/4 | 5925 | . | 0.00 | - | 1 | 0 | 0 |
| 69 | `t27/specs/physics/gamma-conflict.t27` | markdown-prose | markdown-prose, bin 2/2 | 6580 | Y | 0.00 | - | 1 | 64 | 0 |
| 70 | `trinity/apps/website/public/t27/files/trinity-fpga/specs/numeric/takum_variant_split.t27` | spec-DSL | spec-DSL, bin 2/4 | 10241 | . | 0.00 | - | 1 | 0 | 0 |
| 71 | `t27/specs/ar/datalog_engine.t27` | spec-DSL | spec-DSL, bin 3/4 | 10935 | . | 0.00 | - | 0 | 0 | 0 |
| 72 | `t27/specs/ar/explainability.t27` | spec-DSL | spec-DSL, bin 4/4 | 16253 | Y | 0.00 | - | 0 | 0 | 0 |

Read a row as: *this spec was taken because it is the representative of
`<stratum>` at that point in the size distribution, it has no detectable
near-duplicate left in training, it is not an all-stub skeleton, and it had no
training pair written for it.* The `seal` column says how much to trust the gold.

## 7. Reproducing

The split is deterministic given `manifest.json` and the contents of `pairs/`
at build time: the only randomness is `blake2b("t27-lora-v1" + path)`, used as
the last tiebreak. Re-running against a *different* `pairs/` state can change
which specs are eligible, so `split.json` - not the procedure - is the record.
