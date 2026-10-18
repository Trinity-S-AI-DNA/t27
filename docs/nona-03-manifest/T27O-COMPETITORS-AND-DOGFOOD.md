# t27o -- dogfood on the roadmap repositories, competitors, weaknesses

Date: 2026-10-18. Epic #10522 (linked to #10486, M2). English, ASCII only.

t27o (Ouroboros) converts foreign code into `.t27` specs. Today it has one front end, Rust
(`specs/tools/t27o/rust_convert.t27`, std table `rust_std.t27`), run as `tri t27o <file.rs>`. A SPICE
front end for the analog work is a separate lane (#10509) and is not measured here.

Tags: [M] measured by a command in this repository, with the commit it ran at; [V] read on the cited
page; [S] seen only in a search-result summary, a lead and not a fact; [I] our inference or proposal.

## 1. Dogfood: t27o on every Rust file of the roadmap repositories [M]

The roadmap at app.t27.ai (key `m` on t27.ai) is the set of issues labelled `roadmap`: stages 1-9
(#4543-#4858) plus the t27o and t27b goals. Its code lives in these repositories; each tracked `.rs`
file outside `gen/` and `target/` was passed to `tri t27o` (the shared release binary, built
2026-10-18 22:18 from 80988a7b5, the branch of #10514), with a 30 s cap per file.

| Repository | Commit | Files | Over 30 s | fns seen | Converted | Share | Tests converted |
|---|---|---|---|---|---|---|---|
| t27 | 7496ed220 | 382 | 5 | 7,108 | 627 | 8.8% | 249 of 3,446 |
| trios | 483dd9ee | 786 | 0 | 8,230 | 884 | 10.7% | 225 of 4,524 |
| trinity | 0770abb9 | 23 | 0 | 182 | 8 | 4.4% | 6 of 35 |
| trinity-fpga | 20bf4c418 | 30 | 0 | 273 | 22 | 8.1% | 13 of 62 |
| 999-multibots-telegraf | 6e5512881 | 167 | 0 | 982 | 32 | 3.3% | 11 of 161 |
| **Total** | | **1,388** | **5** | **16,775** | **1,573** | **9.4%** | **504 of 8,228** |

Rust is a minority of the roadmap's code: the same checkouts hold 3,679 TypeScript, 1,666 Python,
5,974 Zig and 833 shell files that t27o cannot read at all yet.

The five files over 30 s are the largest: `bootstrap/src/compiler.rs` (1.9 MB), `cli/tri/src/fpga.rs`
(420 KB), `cli/tri/src/gates.rs` (385 KB), `cli/t27b/src/lower.rs` (369 KB) and
`bootstrap/src/service.rs` (289 KB). `cli/tri/src/prcheck.rs` (107 KB) took 3.3 s. The run time grows
faster than the file size.

First refused construct per item, 17,015 refusals in total, by kind:

| Kind | Count | Share |
|---|---|---|
| a type outside the subset (`Vec`, `String`, `Path`, `Ok`/`Err` in a value, ...) | 4,768 | 28.0% |
| uses a refused item (the cascade) | 2,248 | 13.2% |
| a method outside the std table | 2,228 | 13.1% |
| a macro (`vec!`, `format!`, `println!`, `env!`, ...) | 1,741 | 10.2% |
| an item kind (`type`, `use` in a block, ...) | 1,127 | 6.6% |
| a tuple | 797 | 4.7% |
| a name from another file | 704 | 4.1% |
| a path call (`Vec::new`, `BTreeSet::new`, ...) | 576 | 3.4% |
| an enum variant with data | 476 | 2.8% |
| a block as a value | 420 | 2.5% |
| a for over an iterator | 401 | 2.4% |
| a literal (`r#"..."#`, a line continuation) | 362 | 2.1% |
| a trait impl | 323 | 1.9% |
| everything else (17 kinds) | 1,044 | 6.1% |

The single largest reasons: type `Vec` 1,419, macro `vec!` 1,363, type `String` 680, type `Err` 612,
tuple 550, type `Ok` 473, `self` of a refused type 412, type `Path` 402, for over `&...` 380.

What the numbers say [I]: owned heap values (`Vec`, `vec!`, `String`, `Path`) are about a quarter of
all first refusals, and the cascade multiplies every root refusal. Five classes need no new type at
all (#10526: `Ok`/`Err` in test asserts, `for x in &[..]`, `&[]` arguments, raw strings, line
continuations, about 2,000 first refusals together).

## 2. The inner loop [M]

| Step | Command | Time on the bench Mac, 2026-10-18 |
|---|---|---|
| test rust_std.t27 (10 tests) | `t27c test-report` | 94 s |
| the same | `t27b test` | 0.89 s, same 10/10 |
| test rust_convert.t27 (66 tests) | `t27b test` | 13.5 s |

`tri t27o --check` used `t27c test-report` for every converted file. #10525 makes it run `t27b test`
first and fall back to the reference only when t27b's exit code is not a verdict (rules in
`specs/tools/t27o/ledger.t27`).

Test coverage of the converter itself (`t27c coverage`, a fn counts when a test names it):
`rust_convert.t27` 13 of 283 fns (5%), `rust_std.t27` 52 of 59 (88%); golden, install, ledger and
service 100%. Most converter fns are reached only through `convert`, so 5% understates what the 66
tests execute, but no test pins them one by one.

## 3. The field

| Tool | Converts | How correctness is established | Refuses or approximates | Licence / maturity |
|---|---|---|---|---|
| [C2Rust](https://github.com/immunant/c2rust) [V] | C to unsafe Rust | none by default; cross-checks on Linux | warns and skips what it cannot translate | BSD-3; output unsafe and unidiomatic by design |
| [DARPA TRACTOR](https://www.ll.mit.edu/tractor) [V], [benchmark](https://arxiv.org/abs/2609.25121) [V] | C to safe Rust | test vectors on stdout, exit code and state; "empirical, not formal" | n/a | program; batteries published per phase |
| [VERT](https://arxiv.org/abs/2404.18852) [V] | C, C++, Go to Rust | LLM candidate checked by property tests and bounded model checking against a Wasm-derived oracle | regenerates until it passes | research; 1,394 programs |
| [Syzygy](https://arxiv.org/abs/2412.14234) [V] | C to safe Rust | LLM translates code and tests; equivalence on test inputs, dependency order | incremental | research; Zopfli |
| [AlphaTrans](https://arxiv.org/abs/2410.24117) [S] | Java to Python | source tests per fragment | per-fragment validation level | research |
| [RustAssure](https://arxiv.org/abs/2510.07604) [V] | C to Rust | differential symbolic testing | - | research |
| [Rustine](https://arxiv.org/abs/2511.20617) [V], [ORBIT](https://arxiv.org/abs/2604.12048) [V], [TRAIL](https://arxiv.org/abs/2609.15381) [V] | C repositories to safe Rust | tests only | humans finish failures (Rustine) | research, 2025-2026 |
| [TransCoder](https://github.com/facebookresearch/TransCoder) [V] | Java, C++, Python | unit-test computational accuracy; neural | none documented | CC BY-NC 4.0; archived |
| [Google LLM migrations](https://arxiv.org/abs/2504.09691) [V] | in-house migrations | LLM plus human review | review | closed |
| [Amazon Q code transformation](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/how-CT-works.html) [V] | Java versions | rebuild plus existing unit tests | reports partial success | closed service |
| [OpenRewrite](https://github.com/openrewrite/rewrite) [V], [GritQL](https://github.com/biomejs/gritql) [V], [Comby](https://github.com/comby-tools/comby) [V], [Coccinelle](https://github.com/coccinelle/coccinelle) [V] | rule-based rewrites | deterministic; no equivalence check | no match, no change | Apache-2.0, MIT, Apache-2.0, GPL-2.0 |
| [Codon](https://github.com/exaloop/codon) [V], [mypyc](https://mypyc.readthedocs.io/en/stable/differences_from_python.html) [V], [Cython](https://github.com/cython/cython) [V], [py2many](https://github.com/py2many/py2many) [V] | Python to native | static types; no equivalence check; mypyc documents silent semantic differences | partial | various |
| [RightTyper](https://github.com/RightTyper/RightTyper) [V], [MonkeyType](https://github.com/Instagram/MonkeyType) [V], [Pyrefly](https://pyrefly.org/blog/v1.0/) [V] | types for Python | runtime traces (only executed paths) or static inference | untyped stays untyped | Apache-2.0, BSD, - |
| [Aeneas + Charon](https://github.com/AeneasVerif/aeneas) [V], [hax](https://github.com/cryspen/hax) [V] | safe Rust (MIR) to Lean, Coq, F* | a faithful model for user proofs | refuse unsafe, some loops, `&mut` returns | Apache-2.0; used on Microsoft SymCrypt |
| [Creusot](https://github.com/creusot-rs/creusot) [V], [Verus](https://github.com/verus-lang/verus) [V], [Kani](https://github.com/model-checking/kani) [V], [Crux-MIR](https://crux.galois.com) [S] | Rust verification | deductive, SMT, bounded model checking, symbolic | subset | LGPL-2.1, MIT, MIT/Apache-2.0, BSD |

## 4. Ten weaknesses of t27o, and the fix each one points to

1. **A text parser with no types.** Generics, traits, closures, iterator adaptors, tuples and data
   enums are refused; a typed front end removes most of that list. Charon extracts MIR as JSON. Fix
   [I]: a spike that reads Charon's LLBC from a t27 reader and counts the fns it types that the text
   parser refuses.
2. **Heap values are refused wholesale.** `Vec`, `vec!`, `String` and `Path` are a quarter of first
   refusals (section 1). Fix [I]: extend the out-buffer form (#9098) from String to `Vec<T>` built by
   push and read by index.
3. **The cascade.** 13.2% of refusals are items refused only because something they use was. Fix
   [I]: report root refusals and cascade separately, so the ranking is by roots.
4. **Run time grows faster than the input.** Five files over 30 s; compiler.rs does not finish.
   Fix [I]: find the quadratic pass (the refusal fixpoint re-reads the whole file per pass) and
   bound it.
5. **Fixed samples, not coverage-guided vectors.** At most five parameters, body literals plus or
   minus one. cargo-fuzz and proptest find inputs that reach branches. Fix [I]: a seeded fuzz corpus
   recorded as vectors, branch coverage printed per golden fn.
6. **Vector quality is never measured.** cargo-mutants shows how. Fix [I]: mutate the converted t27
   body; a golden fn must kill a stated share of mutants.
7. **Panics are dropped, not checked.** The harness skips an input on which Rust panics, so t27 never
   proves it traps too (ledger.t27 marks such a run truncated). Fix [I]: record "panics" as an
   expected outcome.
8. **No proof tier, even for pure integer fns.** Aeneas, hax and SMT validation exist. Fix [I]: a
   bit-vector check for loop-free fns, reported as `proved(bound=K)`.
9. **Refused bodies are dead ends.** Syzygy, VERT and SACTOR loop a model against an oracle; #8969
   (refused bodies become bee issues) is not built. Fix [I]: oracle-gated repair with a held-out
   vector split, so a repaired body cannot overfit the vectors it was shown.
10. **No public benchmark.** TRACTOR publishes batteries; t27o prints "coverage N%" per file. Fix
    [I]: this document's section 1 as a ratcheted number per release (`tri t27o` over a pinned set).

## 5. Where t27o is ahead

- It refuses with a reason and always emits a spec that compiles; it never approximates silently
  (mypyc documents silent semantic differences).
- The default path spends zero model tokens.
- It reuses an existing t27 fn with the same I/O contract instead of writing a copy; none of the
  tools above documents that.
- Provenance: the service binary is cosign-signed and its run identity is checked
  (`specs/tools/t27o/service.t27`, `golden.t27`).
- Claim discipline: golden is stated as equivalence on recorded vectors, not a proof; unknown is
  never counted as ready (`ledger.t27`).

## 6. What to adopt next, by value and effort [I]

1. Root-cause ranking and the heap form for `Vec<T>` (weaknesses 2 and 3): the largest measured block
   of refusals.
2. Vector quality: coverage-guided samples and a mutation score in the golden criteria (5 and 6).
3. A Charon/MIR spike for the Rust front end (1).
