# SCORING.md - mechanical scorer for generated `.t27`

`score_t27.py` grades a chunk of generated `.t27` source against the measured
gold corpus at `/Users/playom/t27/specs/`. It emits one 0..1 score plus every
raw component, as JSON.

Everything below is a number produced by a command actually run on this machine
on **2026-09-09**. Nothing is estimated. Anything unverified is labelled
`UNVERIFIED`.

- Scorer: `/Users/playom/t27/datasets/lora-v1/score_t27.py` (817 lines, Python 3
  stdlib only - `argparse json os re subprocess sys tempfile`, verified by AST
  walk against `sys.stdlib_module_names`; zero non-ASCII bytes, so it satisfies
  the repo's L3 PURITY law).
- Validator: `/Users/playom/t27/target/release/t27c`, 7,827,680 bytes,
  mtime 2026-09-08 14:59, `t27c version` -> `t27c 0.1.0`, compiler LOC 29538.
- Cost: **44-49 ms per file** (6 `t27c` invocations each). Scoring the whole
  506-file gold corpus takes 22.3 s.

```
python3 score_t27.py FILE.t27                # JSON for one file
cat gen.t27 | python3 score_t27.py -         # stdin
python3 score_t27.py A.t27 B.t27 ... --tsv   # batch, one row per file
python3 score_t27.py --selftest              # the proof suite below; exit 0 = PASS
python3 score_t27.py --calibration           # dump the calibration constants
```

---

## 1. The problem this exists to solve

`t27c typecheck --json` is not a gate. Real output, run just now:

```
$ for f in fixtures/prose.t27 fixtures/python_file.t27 fixtures/empty.t27 fixtures/stubs.t27; do
    printf "%-28s " "$f"; t27c typecheck --json $f | tr -d '\n '; echo; done

fixtures/prose.t27           {"errors":0,"messages":[],"ok":true,"warnings":0}
fixtures/python_file.t27     {"errors":0,"messages":[],"ok":true,"warnings":0}
fixtures/empty.t27           {"errors":0,"messages":[],"ok":true,"warnings":0}
fixtures/stubs.t27           {"errors":0,"messages":[],"ok":true,"warnings":0}
```

635 words of English prose, a valid Python 3 module, a zero-byte file, and a
`.t27` file whose every function body is `// TODO: Implement` all come back
`ok: true, errors: 0`. Four adversarial files added later - keyword salad, 80
bare consts, 50 copy-pasted trivial functions, and a real gold spec with its
bodies deleted - also all come back `ok: true, errors: 0`.

So `typecheck` contributes only 25% of the quality mixture in this scorer, and
it can never on its own rescue a file. The load-bearing signals are elsewhere.

---

## 2. Signals, and the exact `t27c` command each comes from

Flags were discovered by running `--help` on each subcommand. Note what is
*not* there: `parse` and `count` and `metrics` have **no `--json`**; only
`typecheck` and `lint` do.

| Signal | Command | How it is read |
|---|---|---|
| `parse_recovery_events` | `t27c parse <IN>` | trailing line `recovery-events: N` |
| `parse_declarations_swallowed` | `t27c parse <IN>` | trailing line `declarations-swallowed: N` |
| `parse_lexer_discarded_chars` | `t27c parse <IN>` | trailing line `lexer-discarded-chars: N` |
| declaration census | `t27c size <IN>` | `Functions/Structs/Enums/Constants/Tests/Invariants/Benchmarks/Imports/Nodes/Bytes/Lines` |
| declaration cross-check | `t27c count <IN>` | node-type histogram, `*Decl` + `*Block` rows |
| `tc_errors`, `tc_warnings` | `t27c typecheck --json <IN>` | JSON `errors`, `warnings` |
| `lint_issues` | `t27c lint --json <IN>` | JSON `issues` |
| per-function AST nodes | `t27c metrics <IN>` | `nodes` column of the metrics table |

Two implementation notes that matter:

- **`parse` dumps the entire AST to stdout.** The largest file in this repo is
  38.5 MB of source. The scorer streams `parse` output line by line and keeps
  only the three counter lines, so memory stays bounded.
- **`count` truncates.** It prints only the top 15 node types then
  `... and N more`. Its declaration total is therefore a *lower bound* whenever
  `count_truncated_types > 0`, and it is reported as
  `count_decl_nodes_crosscheck` for auditing only. The authoritative
  declaration census comes from `t27c size`, which is complete and structured.

### `t27c metrics` is the empty-body detector

This is the signal that makes stub rejection exact rather than heuristic. The
`nodes` column is the AST node count of a function's body. A `// TODO:
Implement` body - or any body the parser could not consume - yields exactly 1.

```
$ t27c metrics specs/tri/crypto/hex.t27        $ t27c metrics specs/fpga/spi.t27
function          params nodes ret CC ratio    function              params nodes ret CC ratio
lower_case             0     1   0  1   1.0    spi_set_prescaler          1    14   2  2  14.0
upper_case             0     1   0  1   1.0    spi_get_prescaler_div      0     6   0  1   6.0
encode                 3     1   0  1   0.3    spi_get_sck_freq           0     5   1  1   5.0
decode                 2     1   0  1   0.5    spi_transfer               1    33   2  2  33.0
                                               spi_transfer_bit           0    15   0  1  15.0
```

Across all 2,521 functions in the calibration cohort the per-function `nodes`
percentiles are p05 3, p10 4, p25 7, p50 14, p75 27. So `nodes == 1` means
"no body", and `nodes >= 4` (cohort p10) means "substantively implemented".

`t27c complexity` was evaluated for this job and **rejected**: its `stmts`
column is 0 for legitimate gold functions whose body is a single `match` or
`return` expression (e.g. `spi_get_prescaler_div`, which has a real 8-arm
`match` body). It would have produced false stub positives.

---

## 3. Calibration: what "gold-relative" actually meant here

The naive plan - take the 506 files under `specs/` and normalize against their
distribution - is wrong, and measurement is what showed it. `specs/` is not one
population:

```
DIALECT SPLIT (506 *.t27 under specs/, any "scratch" path segment excluded)
  dialect         n  fn-bearing  all-bodies-empty  %empty  recov==0  tcerr==0
  module_semi   235         189               174   92.1%     96.2%     97.9%   `module X;`
  module_brace  187         181                23   12.7%     95.2%     75.4%   `module X { }`
  other          58          50                 2    4.0%     79.3%     84.5%
  markdown       15           0                 0    0.0%      0.0%    100.0%   prose documents
  specDSL         8           8                 7   87.5%     87.5%    100.0%   `spec X { }`
  algorithm       3           0                 0    0.0%      0.0%    100.0%   `algorithm X { }`
```

**206 of the 428 fn-bearing gold files (48.1%) have every single function body
empty.** For comparison, `grep -rl "TODO: Implement" specs | grep -c '\.t27$'`
returns 172 - the AST-based detector finds 34 more that are hollow without
saying so, mostly `spec X { }`-dialect files whose bodies `t27c` cannot parse.

Normalizing the body term against that population would have made "emit a
plausible signature and leave the body empty" score at the corpus median. That
is the exact failure mode this scorer must reject.

So calibration uses a **named subset**, stated in the code:

> **GOLD-IMPL** (n = **281** of 506) = files under `specs/` with
> `decl_total >= 1`, dialect != markdown, and NOT every function body empty.
> Dialects in cohort: module_brace 164, module_semi 59, other 54, algorithm 3,
> specDSL 1.

Measured GOLD-IMPL percentiles - these are the constants:

```
DECLS             min 1    p05 1     p10 1     p25 11    p50 30    p75 52
nodes_per_decl    min 1.75 p05 3.5   p10 4.544 p25 6     p50 7.357 p75 10.14
per-fn AST nodes  (2521 fns)         p05 3     p10 4     p25 7     p50 14
recovery_events   p90 0    max 4     (91.8% of the cohort == 0)
lexer_discarded   max 0              (100.0% == 0)
decls_swallowed   max 0              (100.0% == 0)
tc_errors         p75 0    p90 4     max 23    (78.6% == 0)
tc_err_per_decl   p75 0    p90 0.0781          max 1.294
lint_per_decl     p05 0.08 p25 0.1667 p50 0.2581 p75 0.3714 p90 1.0
testcov_per_decl  p05 0.1111 p25 0.3043 p50 0.4167 p75 0.55  p90 1.0
all four parse/type signals clean: 199/281 = 70.8%
```

Raw-506 percentiles, recorded so the gap is visible but used nowhere in the
score: `DECLS` p25 7 / p50 17; `nodes_per_decl` p25 3.111 / p50 5.13;
`stub_fraction` p50 0.0625 / p75 1.0; `recovery==0` 90.3%; `tc_errors==0` 88.1%;
all four clean 398/506 = 78.7%.

**"Gold-relative" concretely means:** full credit is pegged at a gold
percentile, never at zero defects.

- `lint_per_decl` earns full credit at cohort **p25 = 0.1667**, not at 0.
  Nearly every gold function trips lint's `fn 'X' has no test or invariant`
  rule - `specs/fpga/spi.t27` has 12 functions, 17 tests, 12 invariants, and
  still reports 12 lint issues, because the rule matches test *names* against
  function names. Demanding 0 lint issues would score the gold corpus below
  the model.
- `tc_errors` is a soft half-life decay, not a gate, because 21.4% of the
  cohort has a nonzero typecheck error count.
- `recovery_events` gets a half-life of 2 because the cohort tolerates up to 4.
  `lexer_discarded_chars` and `declarations_swallowed` are punished harder
  because the cohort max for both is exactly 0.

---

## 4. The formula

```
decl_total = functions + structs + enums + constants + tests + invariants + benchmarks
             # imports DELIBERATELY EXCLUDED - see below

s_decl   = min(1, (decl_total / 11)**0.5)                  # 11 = GOLD-IMPL DECLS p25
fnfill   = mean over functions of ramp(fn_nodes, 1, 4)     # 0 at nodes==1, 1 at nodes>=4
shape    = ramp(nodes_per_decl, 1.75, 6.0)                 # cohort min .. cohort p25
s_body   = fnfill * (0.70 + 0.30*shape)     if any functions declared
         = shape                            if none (struct/const-only module)
s_parse  = 0.5**(recovery/2) * 0.5**(swallowed/0.5) * 0.5**(lexdisc/4)
s_type   = 0.5**((tc_errors/decl_total) / 0.12)
s_lint   = inv_ramp(lint_issues/decl_total, 0.1667, 1.0)   # cohort p25 .. p90
s_tests  = ramp((tests+invariants)/decl_total, 0, 0.4167)  # cohort p50
s_variety= 1.0 unless fn-shape uniformity > 0.90 (see below)

quality  = 0.40*s_parse + 0.25*s_type + 0.15*s_lint + 0.20*s_tests
score    = s_decl * (0.08 + 0.92*s_body) * quality * s_variety
```

Three design choices are load-bearing and each was forced by a measurement:

**(1) `imports` is excluded from `decl_total`.** Measured: a valid Python file
scores `size_imports == 5`, because `t27c`'s recovery turns `import re` and
`from collections import OrderedDict` into UseDecl nodes. Counting imports
would have given Python a nonzero declaration floor and a nonzero score.

**(2) `s_decl` and the body term are MULTIPLICATIVE gates, not summands.** A
weighted sum lets a file that parses cleanly and typechecks cleanly bank 65% of
the score while containing nothing. The empty file demonstrates this: it scores
`s_parse 1.000, s_type 1.000` - perfect on both - and still ends at 0.0000
because `decl_total == 0`.

**(3) `fnfill` multiplies `shape` rather than averaging with it.** In the first
version they were averaged, and `specs/storage/kv.t27` - 8 functions all empty,
but 11 test blocks with real bodies, `nodes_per_decl` 10.63 - reached
`shape == 1.0` from its test blocks alone and scored **0.3397** despite having
zero implemented functions. Under multiplication it scores 0.0776, and the
all-hollow gold population's ceiling dropped from 0.3397 to **0.0800**, which
is what finally separated it from the implemented population.

### The copy-paste guard

`uniformity` = the fraction of a file's functions sharing the single most
common per-function AST node count. Measured over the 165 GOLD-IMPL files with
at least 5 functions:

```
min 0.053  p10 0.143  p25 0.167  p50 0.231  p75 0.333  p90 0.467  max 0.900
only 1 of 165 gold files exceeds 0.90
```

A generated file of 50 identical `fn fN(x: u32) -> u32 { return x + N; }`
bodies measures **1.000**. The penalty therefore activates only *above* the
observed gold maximum. Verified across the full corpus: the guard touched
61 of 506 gold files, and **0 of them have any implemented function** - the
implemented-gold distribution is bit-identical with and without it.

---

## 5. THE PROOF

`python3 score_t27.py --selftest`, run just now, verbatim. All four negatives
are built into the file (the prose is 635 words; `_PY` is checked with
`ast.parse`). The five gold specs were chosen by a stated deterministic rule,
not by hand: score all 506, keep the 222 with at least one non-empty function
body, sort by score, take the p10 / p30 / p50 / p70 / p90 members - so the
sample spans the implemented-gold range instead of showcasing its top.

```
case                                kind       SCORE |   decl   body  parse   type   lint  tests |  ndec   rec   lex  swal   tcE  stub
---------------------------------------------------------------------------------------------------------------------------------------
(a) english_prose                   NEGATIVE  0.0000 |  0.000  0.000  0.707  1.000  0.000  0.000 |     0     1     0     0     0     -
(b) valid_python                    NEGATIVE  0.0000 |  0.000  0.000  0.177  1.000  0.000  0.000 |     0     5     0     0     0     -
(c) empty_file                      NEGATIVE  0.0000 |  0.000  0.000  1.000  1.000  0.000  0.000 |     0     0     0     0     0     -
(d) todo_stubs                      NEGATIVE  0.0137 |  1.000  0.000  1.000  1.000  0.218  0.000 |    11     0     0     0     0  1.00
nn/hslm.t27                         GOLD      0.5985 |  1.000  0.729  1.000  0.206  0.972  1.000 |    84     0     0     0    23  0.25
fpga/testbench/mac_tb.t27           GOLD      0.8301 |  1.000  1.000  1.000  0.637  0.806  0.750 |    64     0     0     0     5  0.00
numeric/gf24.t27                    GOLD      0.9026 |  1.000  1.000  1.000  0.637  0.956  1.000 |    64     0     0     0     5  0.00
fpga/testbench/ternary_isa_tb.t27   GOLD      0.9367 |  1.000  0.984  1.000  1.000  1.000  0.754 |    35     0     0     0     0  0.00
fpga/apb_bridge.t27                 GOLD      0.9700 |  1.000  1.000  1.000  1.000  0.800  1.000 |    48     0     0     0     0  0.00
---------------------------------------------------------------------------------------------------------------------------------------
max(NEGATIVE) = 0.0137    min(GOLD) = 0.5985    separation ratio = 43.7x
  [PASS] max(NEGATIVE) < 0.10
  [PASS] min(GOLD) >= 0.40
  [PASS] min(GOLD) > 5 x max(NEGATIVE)
SELFTEST: PASS
```

Read the negative rows against the `typecheck` output in section 1. Prose,
Python and the empty file are all `typecheck ok:true errors:0` and all score
**0.0000**, because `decl_total == 0` gates them. The stub file is `ok:true
errors:0` *and* parse-perfect (`rec 0, lex 0, swal 0`) *and* has 11 real
declarations - `s_decl` is a full 1.000 - and it still lands at **0.0137**,
because `s_body` is 0.000.

The same run against the on-disk fixtures, to show it is not an artifact of
inlined strings:

```
path                  score   s_decl  s_body  s_parse  s_type  s_lint  s_tests  decls  stub_fraction
prose.t27             0.0     0.0     0.0     0.7071   1.0     0.0     0.0      0      None
python_file.t27       0.0     0.0     0.0     0.1768   1.0     0.0     0.0      0      None
empty.t27             0.0     0.0     0.0     1.0      1.0     0.0     0.0      0      None
stubs.t27             0.0137  1.0     0.0     1.0      1.0     0.2182  0.0      11     1.0
```

Fixtures live in `/Users/playom/t27/datasets/lora-v1/fixtures/`.

### Population-level separation (not 5 hand-picked files)

Scoring all 506 gold files, split by whether any function has a real body:

```
GOLD every fn body EMPTY       n= 206  min 0.0137  p05 0.0161  p25 0.0198  med 0.0552  p75 0.0725  max 0.0800
GOLD >=1 fn implemented        n= 222  min 0.1675  p05 0.4571  p25 0.8058  med 0.9026  p75 0.9484  max 1.0000
GOLD zero functions declared   n=  78  min 0.0000  p05 0.0000  p25 0.2277  med 0.2563  p75 0.3805  max 1.0000
```

The hollow ceiling (0.0800) sits below the implemented floor (0.1675). The two
populations do not overlap. That is a 206-vs-222 file separation, not a demo.

The 5 gold files scoring exactly 0.0000 are `specs/benchmarks/bench_main.t27`,
`specs/tri/encoding/html.t27`, `specs/tri/encoding/xml.t27`,
`specs/tri/search/match.t27`, `specs/tri/utils/args.t27` - all have
`decl_total == 0`. They are prose or unparseable, and the scorer is right to
zero them. **The gold corpus contains files this scorer rejects. That is
intended.** Do not expect a "gold scores 1.0" invariant; the honest headline is
`GOLD >=1 fn implemented, median 0.9026`.

### Adversarial cases

Every one of these also returns `typecheck ok:true errors:0`.

```
path                      score   s_decl  s_body  s_parse  s_lint  s_tests  decls  stub_fraction
adv_keyword_salad.t27     0.0000  0.000   0.000   0.0221   0.0     0.0      0      None
adv_const_spam.t27        0.1095  1.000   0.062   1.0000   1.0     0.0      80     None
adv_trivial_fns.t27       0.2350  1.000   1.000   1.0000   0.6     1.0      100    0.00
adv_gold_hollowed.t27     0.0194  1.000   0.000   1.0000   0.8     1.0      48     1.00
GOLD:fpga/apb_bridge.t27  0.9700  1.000   1.000   1.0000   0.8     1.0      48     0.00
```

- `adv_keyword_salad` - 900 real t27 tokens in random order -> 0.0000.
- `adv_const_spam` - 80 bare `pub const K_n = n;` and nothing else, a direct
  attempt to inflate the declaration floor -> 0.1095.
- `adv_gold_hollowed` - `specs/fpga/apb_bridge.t27` with every function body
  mechanically replaced by `// TODO: Implement`, nothing else changed ->
  **0.9700 drops to 0.0194, a 50x fall from one edit.**
- `adv_trivial_fns` - 50 copy-pasted `return x + N` functions plus 50 matching
  tests -> 0.2350, held down only by the uniformity guard. See the next
  section: this is the scorer's real ceiling.

---

## 6. What this scorer CANNOT measure

Stated plainly, because using it outside these limits will produce a wrong
conclusion about the LoRA.

**It cannot measure semantic correctness. There is no t27 interpreter.**
Measured under `specs/`: 4,278 `test` blocks, 1,910 `invariant` blocks and 756
`bench` blocks. `t27c test` lists them (`specs/numeric/gf24.t27` -> `Tests: 25,
Invariants: 12, Benchmarks: 6`) but nothing executes them. Nothing here checks
that a function returns the right value, that an invariant holds, or that the
code does what its name says. A function that computes the wrong answer with
correct types scores exactly as well as one that computes the right answer -
and `s_tests` rewards the mere *presence* of test blocks, not their content.

**"The generated code compiles" is not a gate, and is not used.** `t27c`'s Zig
backend emits bare `match;` placeholders, and only ~67.5% of sampled *gold*
specs produce Zig that passes `zig ast-check` (measured in the prior
investigation; not re-verified here - treat as UNVERIFIED in this document).
Gating on codegen would reject a third of the gold corpus.

**It cannot detect plagiarism or memorization.** A model that reproduces
`specs/fpga/apb_bridge.t27` verbatim scores 0.9700. Score this alongside an
n-gram overlap check against the training set, or the number is meaningless.

**It cannot tell "well-written" from "structurally well-formed."** This is the
demonstrated ceiling. `adv_trivial_fns.t27` - 50 near-identical one-line
functions with 50 matching tests - has perfect parse, perfect typecheck, zero
stubs, and full test coverage. It scores 0.2350 *only* because the uniformity
guard fires at 1.000. Vary the bodies slightly to bring uniformity under 0.90
and the same padding would score above 0.90. **Do not use this score as a
standalone RL reward signal.** It is a filter and a comparator, not an
objective.

**Its lint term is weak by construction.** `t27c lint` mostly emits
`fn 'X' has no test or invariant`, matched by name, and it fires on nearly all
gold. It carries only 15% weight for that reason. A file can be genuinely bad
and clean on lint.

**`t27c count` is a truncated cross-check, not a source of truth.** It prints
the top 15 node types then `... and N more`. When `count_truncated_types > 0`
the reported `count_decl_nodes_crosscheck` is a lower bound. The declaration
census comes from `t27c size`.

**It is dialect-blind on purpose, and this cuts both ways.** The scorer does
not care whether output is `module X;` or `module X { }`. But it does
implicitly punish the `spec X { }` dialect: `t27c` parses those files' function
signatures and not their bodies, so they read as 100% stubs
(`specs/ar/asp_solver.t27` scores **0.0455** with `stub_fraction 1.0`, despite
having visible `return Program { clauses: [], constraints: [] }` bodies in the
source). If the LoRA is ever meant to emit that dialect, this scorer will grade
it as garbage. Train on one dialect.

**The calibration is a snapshot.** Constants were measured against this
`t27c` binary (mtime 2026-09-08 14:59) and this state of `specs/`. Rebuild the
compiler or land a batch of specs and the constants need re-measuring. The
measurement is cheap - 22.3 s for the full corpus.

---

## 7. Output shape

```
$ python3 score_t27.py /Users/playom/t27/specs/numeric/gf24.t27
{
  "score": 0.9026,
  "components": {
    "s_decl": 1.0, "s_body": 1.0, "s_parse": 1.0, "s_type": 0.6368,
    "s_lint": 0.9563, "s_tests": 1.0, "s_variety": 1.0,
    "quality_mix": 0.9026, "body_term": 1.0
  },
  "derived": {
    "decl_total": 64,
    "decl_breakdown": {"functions": 13, "structs": 1, "enums": 0,
                       "constants": 7, "tests": 25, "invariants": 12,
                       "benchmarks": 6},
    "imports_excluded_from_decl_total": 2,
    "nodes_per_decl": 10.0,
    "stub_fraction": 0.0, "impl_fraction": 1.0, "fn_body_fill": 1.0,
    "body_shape_nodes_per_decl": 1.0, "fn_shape_uniformity": 0.1538,
    "empty_body_functions": 0, "implemented_functions": 13,
    "tc_err_per_decl": 0.0781, "lint_per_decl": 0.2031,
    "testcov_per_decl": 0.5781
  },
  "signals": {
    "parse_recovery_events": 0, "parse_declarations_swallowed": 0,
    "parse_lexer_discarded_chars": 0, "declaration_count": 64,
    "count_decl_nodes_crosscheck": 57, "count_crosscheck_truncated_types": 7,
    "typecheck_errors": 5, "typecheck_warnings": 7,
    "typecheck_ok_UNTRUSTWORTHY": false,
    "lint_issues": 13, "size_nodes": 640, "size_bytes": 13626
  },
  "tool_errors": [],
  "path": "/Users/playom/t27/specs/numeric/gf24.t27"
}
```

That file is worth reading closely: it has **5 typecheck errors** and
`typecheck_ok: false`, and it still scores 0.9026. That is gold-relative
scoring working as intended - 21.4% of the calibration cohort has nonzero
typecheck errors, so a handful of them costs `s_type` (0.637) but does not
disqualify a file that is otherwise dense, tested and parse-clean. Note also
`count_crosscheck_truncated_types: 7`, meaning `t27c count` hid 7 node types
and its 57 is a lower bound against `size`'s authoritative 64.

`typecheck_ok_UNTRUSTWORTHY` is named that way on purpose. It is reported for
completeness and is never allowed to raise a score on its own.

Robustness, all verified: runs from any cwd; a nonexistent path returns
`score 0.0` with the failure listed in `tool_errors`; 4 KB of raw
`/dev/urandom` returns `score 0.0`; `parse` output is streamed so a 38 MB input
does not blow memory; `--selftest` exits 0 on PASS and 1 on FAIL, so it can be
wired into a gate.
