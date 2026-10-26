# Cheaper mutation gate: literature survey (2026-10-25)

Method: 25 works opened with WebFetch. "abs" = abstract page only; "docs" = tool documentation. An arXiv id resolves at arxiv.org/abs/<id>. Long pages were read in part (section 4). Our example run (677 mutants, 31 hung at 30 s, 20 min 9 s, 3 jobs): hung waits are about 930 of about 3,630 job-seconds (26%); the other mutants average about 4.2 job-seconds. Derived from the stated figures, not measured.

## 1. Works opened

| # | Work | Idea and reported saving | Risk to verdict |
|---|---|---|---|
| 1 | Petrovic, Ivankovic, Fraser, Just; IEEE TSE 2021 (Google); arXiv:2102.11378 | Mutate only changed, covered lines; drop low-value "arid" mutants; one mutant per line; pick operators by past usefulness. Median mutants per change 820 -> 77 -> 7 (Sec. V-B). No runtime figure. | Yields a few useful survivors, not a full score. |
| 2 | Foster et al.; FSE Companion 2025 (Meta); arXiv:2501.12862 | Issue-targeted LLM mutants; an LLM judge drops equivalents. 31,677 mutants -> 9,095 build and pass -> 4,660 judged non-equivalent (Table 2). No token or cost figure. | Judge precision/recall 0.97/0.44 to 0.79/0.47 by scoring rule; 0.95/0.96 after stripping comments (Table 6). |
| 3 | Mutant-reduction evaluation; arXiv:2102.02978 (abs) | Judge a reduction by whether it keeps the ranking of test suites (OP, EROP). No saving. | A reduced set can match the score yet reorder suites. |
| 4 | Wang, Xiong, Shi, Zhang, Hao; ISSTA 2017 (AccMut); arXiv:1702.06689 (abs) | Group mutants with equal state at a statement; one process per group. 2.56x over split-stream. | Needs process forking. |
| 5 | Gopinath, Alipour, Ahmed, Jensen, Groce; ISSRE 2015; https://experts.nau.edu/en/publications/how-hard-does-mutation-analysis-have-to-be-anyway/ (abs) | A constant-size random sample tracks the full score at any program size. | Estimate only; no survivor list. |
| 6 | Mutation analysis for fuzzers; arXiv:2201.11303 (2022) | Cheap static-suite pass first, costly work only on survivors; supermutants; minimal set computed afterwards. Equivalents cited at 10-40%. No speedup numbers. | Most speed-ups assume a fixed suite. |
| 7 | Tian, Shu, Wang, Cao, Kamei, Chen; ISSTA 2024; arXiv:2408.01760 | Equivalent-mutant detectors on 3,302 Java pairs. Fine-tuned UniXCoder: 0.043 s per pair, F1 81.9. Compiler checks: 1.02 s and 2.35 s per pair, F1 39-51. GPT-4 prompting F1 54-56 (Tables 2-4). | Java, labelled pairs; none exist for t27. |
| 8 | Commit-relevant mutants; arXiv:2112.14566 | Keep mutants relevant to a commit, then only subsuming ones: about 93% fewer mutants (Fig. 12). Quotes 53% and up to 80% time cuts from others (second-hand). | Subsumption depends on the test pool. |
| 9 | Clang mutation optimisations; arXiv:2210.17215 (2022) | Run only tests that reach a mutant: 2.07x-30.52x (Table 5). One-build schemata: up to 5.03x, 0.81x on one subject; compile 7m17s -> 1.23s on another (Table 4). | Schemata can slow runs; split-stream failed; per-test timeouts only proposed. |
| 10 | Contextual predictive mutation testing; arXiv:2309.02389 (2023) | Fine-tuned CodeBERT predicts kills in 34 ms; about 71% time saved if predicted survivors are checked (Table 7). | Cross-project recall 0.37, F1 0.48 (Table 3); Java; needs training data. |
| 11 | Cornejo, Pastore, Briand; IEEE TSE 2022; arXiv:2101.05111 | Space software: confidence-interval sampling, coverage-distance test reduction, coverage and compiler equivalence, cheap rebuild. Sampling saves 65.8-84.7% time (median 75.1%); with a reduced suite 72.1-90.8% (Table XIV); rebuild 0.96 s vs 4.6 s per mutant (Table III). Compiler comparison found 6.36% equivalent, 27.43% duplicate overall, about 21% on pure-math code (Table V, Sec. IV-I). | Random test selection errs above 5 points (Table XIII); coverage-distance equivalence loses 1.88% of killable mutants (RQ6); simulators cannot be forked. |
| 12 | Higher-order mutants; arXiv:2004.02000 (2020) | Finds hard-to-kill mutant pairs. No cost saving measured. | Constituents mask each other; needs all first-order runs first (Sec. 2.3). |
| 13 | MeMu; arXiv:2102.11559 (2021) | Memoize costly unmutated methods: 18.15% faster on average, range -0.66% to 51.77% (Table I); top 20% of methods hold 43.21% of time. | Non-determinism hurts the score; one subject slower. |
| 14 | GateTruth; arXiv:2608.12635 (abs) | Seeded RTL mutants audit testbenches; certify at 95% killed. No cost data. | Mutant count not stated. |
| 15 | YosysHQ mcy (docs); https://yosyshq.readthedocs.io/projects/mcy/en/latest/methodology.html | Single-bit netlist mutants; a formal equivalence check decides if behaviour changed; tests run shortest first, stop at first failure. No numbers. | Assumes tests pass on the correct design. |
| 16 | StrykerJS (docs); https://stryker-mutator.io/docs/stryker-js/configuration/ | Timeout = net time x 1.5 + 5 s + overhead; per-test coverage runs only covering tests; incremental reuse; bail at first failure. No numbers. | Needs order-independent tests. |
| 17 | Stryker issue 3023 (2021); https://github.com/stryker-mutator/stryker-js/issues/3023 | Per-mutant hit counter; fail when hits exceed normal hits x factor (100 suggested) + offset. Aim: avoid worker restarts (about a minute each). Shipped Aug 2021, Karma only. | Heavy but legitimate loops cut; user code may catch the error. |
| 18 | PIT FAQ (docs); https://pitest.org/faq/ | Drop tests that skip the mutated line; run the rest fastest first; timeout from baseline. No numbers. | Spurious timeouts when test order shifts. |
| 19 | FrugalGPT; arXiv:2305.05176 (abs) | Prompt adaptation, approximation, LLM cascade: up to 98% cheaper at GPT-4 accuracy. | Own benchmarks. |
| 20 | RouteLLM; arXiv:2406.18665 (abs) | Router sends easy queries to the weak model: over 2x cheaper in some cases. | Chat data, not code gates. |
| 21 | Agentless; arXiv:2407.01489 (abs) | Fixed localize-repair-validate pipeline: 32.00% of SWE-bench Lite at about $0.70. | One benchmark. |
| 22 | Lindenbauer et al.; arXiv:2508.21433 (abs) | Mask old tool observations: about half the cost at equal solve rate; hybrid 7% and 11% cheaper than masking and summarization. | SWE-bench Verified only. |
| 23 | LLMLingua; arXiv:2310.05736 (abs) | Prompt compression up to 20x with small loss. | Exact identifiers may be lost. |
| 24 | Anthropic, effective context engineering, 2025-09-29; https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents | Clear used tool results; compact; notes outside context; sub-agents return about 1,000-2,000 tokens; load by reference. No measurement. | Compaction can drop detail. |
| 25 | Anthropic prompt caching (docs); https://platform.claude.com/docs/en/build-with-claude/prompt-caching | Cached prefix, 5-minute life refreshed on use, 1-hour option. Write 1.25x (5 min) or 2x (1 h); read 0.1x, 0.05x on Opus 5.5 and Sonnet 5.5. | Exact prefix; any tool or system change invalidates. |

## 2. Fit for our gate

USE NOW, ranked by saving per effort:

1. Baseline-derived timeout [16][18]. One timed unmutated run per spec, then factor x baseline + constant (Stryker: 1.5 and 5 s) instead of 30-45 s. Rerun only the hung set once at the old bound, to catch false hangs under shared-CPU load. The ceiling is the 26% hung share; cutting the bound from 30 s to about 11 s would save about 16% of job-time (my arithmetic).
2. Fail-fast and fastest-first test order, with per-mutant test selection if t27c can map tests to functions [9][15][16][18]. Needs a stop-at-first-failure option in the compiler-test command and an order list from the last run (last killer first). About 90% of mutants die (608 of 677), so most runs can stop early; no opened source gives a percentage for this alone. Tests must be order-independent [16]. Measure compile versus test time first: [9] shows compile can dominate and would cap this gain.
3. Verdict cache and diff scope [1][8][16]. Cache each verdict keyed on the hash of all non-test text plus the test text. After a test-only edit, rerun survivors only (an added test cannot un-kill a mutant); after a code edit, rerun the changed function with `--fn`. The full gate stays mandatory before landing. Google mutates only changed lines [1].

MAYBE:
- Loop-count fuel [17]: kills hung mutants in milliseconds, but needs a step counter in generated code; larger than item 1.
- Schemata and incremental rebuild [9][11]: large only if compile dominates; needs t27c to emit one program with a mutant switch.
- Compiler-output hash prefilter [11][7]: skips identical output; 6.4% equivalent and 27.4% duplicate in the space code, less on pure math; check on a sample.
- Subsumption quick mode [8]: needs a stored kill matrix; tick mode only.
- Sampling with a confidence stop [5][11]: gives a rate, not survivors; tick mode only.
- Cheap-model survivor triage [2][7]: diff only, comments stripped, accepted-file re-check kept.
- Operator order by past survivor yield [1]: tick mode only.
- Formal equivalence prefilter [15]: needs a t27 equivalence tool.

NO:
- [3] method paper only. [4] needs forking; [11] says simulators cannot be forked. [6] fuzzer-specific. [10] needs a training corpus; cross-project recall 0.37.
- [12] batching mutants in pairs fails with about 90% kills: a killed batch says nothing about its members.
- [13] 18% average in Java; our time is compile and hangs. [14] no cost data. [23] drops exact names.

Hardware and analog: I found no paper on mutation operators for analog or mixed-signal models. Nearest are [11] (embedded simulators, no forking) and [14][15] (RTL, early stop, formal filter).

## 3. Token economy of the orchestrator

1. Stable cache prefix with the 1-hour life [25]. A 15-minute tick outlives the 5-minute life, so every tick re-writes the cache. One 1-hour write (2x) plus three reads (0.1x each, or 0.05x on Sonnet 5.5) costs 2.3x per hour against 5.0x for four 5-minute writes (my arithmetic). Put volatile text after the cached part. Whether the harness already sets the 1-hour life is not verified.
2. Read an index plus the delta since the last tick, not the whole ledger [24]: notes outside context, load by reference.
3. Clear raw tool output after use [22][24]: masking old observations halved cost at equal solve rate.
4. Fixed-shape delegate returns [24]: sub-agents read widely and hand back about 1,000-2,000 tokens; ask for class counts plus the survivor list.
5. Cascade with the gate as verifier [19][20]: cheap pane first, stronger model after failures. Their savings (up to 98%, over 2x) come from their own benchmarks; measure ours.
6. Fixed pipeline for bounded jobs [21]: survivor triage as diff, class, gate re-check; Agentless got 32.00% at about $0.70. Skip prompt compression [23] on ledger lines.

## 4. Not verified

- Papadakis et al. 2019 survey, Offutt et al. 1996 (five operators, 99.5%): search snippets only.
- Zhang et al. ISSTA 2013 (test prioritization): ACM 403. Just, Ernst, Fraser ISSTA 2014 (state partitioning, "40% average"): 403, snippet only. Zhang et al. ISSTA 2016 predictive mutation testing ("up to 151.4x"): 403, snippet only.
- Papadakis et al. ICSE 2015 trivial compiler equivalence: PDF unreadable, UCL 403; only the secondary figures in [7] and [11] were read.
- Subsumption savings of Ammann 2014, Alipour 2016, Guimaraes 2020: second-hand via [8].
- PIT default timeout values (snippet only), Stryker.NET PR 3906, mutrim issue: not opened.
- Gopinath composite faults, Wong and Mathur, Jia and Harman and Polo higher-order papers, Google 2018 paper, DATE 2024 Yosys mutation paper, VeriBugBench: not opened. Analog fault-injection patents: snippets only.
- Unread tails: [1] last 31,584 characters, [8] last 67,343, [9] last 46,765, [7] last 4,037, [11] part, caching docs last 62,264.
- Author lists are given only where the page showed them; arXiv PDFs would not render, so HTML versions were used.
