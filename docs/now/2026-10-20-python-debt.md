# NOW -- the Python debt census: lines, functions and the share that fits a convertible subset (2026-10-20)

## docs/now/2026-10-20-python-debt.md (Closes #11168, the measure-first step of #8995 and #9006; analysis only)

- Scope: no Python is deleted and no spec is added by this change. It counts the hand-written Python of the five repositories with Python's own `ast` and `tokenize` modules, classifies every function against a written subset, and ranks files by the lines a port could delete. The tables are the same ones posted on #11168. Discipline follows #11149, #11159, #11135 and #11145: measure first, name the method and the commits, state the threshold before writing a rule.
- Sources: t27 is this repository at master 0e00974da. trios main 78671ba794, trinity main 8e535bc9bb, trinity-fpga main c9b180bf78 and 999-multibots-telegraf main fcdbb2f7dc are the commits #11149 and #11159 used. The local checkouts of those four were not used: they sit on stale feature branches (trios golden-bridge-p1-bibliography 483dd9ee, trinity feat/wars-tri-decision-layer 0770abb9, trinity-fpga trinet-fleet-truth 20bf4c41, 999-multibots-telegraf 6e551289). The four commits were fetched into a scratch directory (depth 1, no blobs but the file types listed under "Reproduce").
- Instrument: three throw-away Python scripts (scan, post, report; about 1,700 lines, plus two small scripts that render this file) with `python3 -I`; not committed. No regular expression classifies code. The only regular expression is in the `grep -o` that lists path-like tokens ending in `.py` for the callers search.

## Answer

- Prize, in one sentence: 812 of 331,183 distinct hand-written, non-test Python lines (0.25%) sit in the 105 distinct functions that fit the typed subset today, and 4,090 lines (1.23%) in the 454 that fit once their parameter types are inferred from traces; no file is a whole-file candidate of 150 lines or more, so a Python port deletes functions, not files.
- Scale: 2,126 tracked `.py` files hold 434,986 gate-counted lines (t27 alone 153,503, which equals `git diff --numstat` of deleting every tracked `.py` there, checked). After 23 generated, 22 vendored and 138 test-only files, the census holds 1,943 files and 412,066 lines; 1,906 files parse (12,547 functions, 1,001 classes) and 37 do not. Counting identical contents once (230 of trinity's 306 scanned files are byte-identical files of trinity-fpga) that is 1,618 files, 331,183 lines and 8,501 distinct function bodies.
- Typed subset (T): 230 fns / 1,793 lines raw, 105 distinct bodies / 812 lines (0.25% of 331,183). Shape subset (S, parameter types still to be inferred from traces, #9006): 654 fns / 5,444 lines raw, 454 distinct bodies / 4,090 lines (1.23%). For comparison the Rust measurements were 0.69% (whole files, #11149) and 0.32% (movable fns, #11159).
- The functions that fit are small: the median in-subset function has 6 lines (T) and 6 lines (S), the largest 34 (T) and 89 (S); 4 of the 105 T bodies and 34 of the 454 S bodies have 20 lines or more. The median function of the whole census has 13 lines.
- Ceiling if t27 could say heap values: a further 1,293 distinct fns / 16,577 lines (5.0%) are pure but blocked only by owned strings, dict/set, comprehensions, union or None returns, or a pure standard-library import (re, struct, hashlib, fractions, ...). That is the same wall that bounded the Rust census: what t27 cannot say, not what a converter has not learned.
- Whole-file candidates: T none; S one, `tools/ternary_model.py` (93 lines, 12 fns, no test, imported by `tools/verify_exhaustive.py`, and it already has a spec twin `specs/port/tools/ternary_model.t27`). The first whole-file candidate of 150 lines or more whose functions all fit: none. Of 737 distinct files with 150 lines or more, 0 have every function in T and 1 has every function in S (`archive/experiments/kinetic_sacred_pure.py`, 256 lines, 3 functions) but it also runs 82 module-level statements (a report script made of top-level `print` calls), so it is not whole. The closest real candidate is `tools/gft_train_demo.py` (201 lines): 16 of 18 functions fit the shape subset (130 of 201 lines), but none is typed and `main` (33 lines) reads `random` and prints.
- Oracles: of the 64 distinct files that hold a T function, 3 have a test file that imports them and none has a doctest; 20 contain `assert` statements or inline test functions. 62 of the 64 are scripts with an `if __name__` guard. The original itself, run on sampled inputs, is the only oracle for most of them (#8995).
- Hand check: 30 functions classified by hand against the written rules before looking at the scanner's verdict. First round 29 of 30 agreed (one false positive: a function returning None or bytes; the rule "optional return" was added). Fresh sample of 30 with the final scanner: 30 of 30 agree (15 of 15 in-subset, 15 of 15 out; 95% lower bound for the in-class precision, one-sided, 82%).
- Entry points: no whole-file candidate means a port deletes functions, and `t27c gen-python` lowers no function (it emits consts, enums and structs only; a `fn` block is announced in a comment and no `def` is ever written, see `bootstrap/src/codegen_python.rs`). A script whose functions move to a spec therefore keeps calling them through a hand-written loader such as `scripts/tri_loop/t27b_rules.py` (compiles the generated C and calls it with ctypes), or its callers change. Section "Callers and entry points" has the numbers.

## Scope and exclusions

| rule | files | lines | reason |
|---|---|---|---|
| generated root (`gen/`, `bootstrap/gen/`, `bootstrap/src/memory/generated/`) | 0 | 0 | the own-language gate's generated roots (`specs/policy/own_language.t27`); no tracked `.py` sits there |
| generated header | 23 | 1,699 | a comment line among the first 8 lines says one of: do not edit, auto-generated, autogenerated, this file is generated, generated by, @generated, code generated, automatically generated. 9 VIBEE multi-language generator outputs in each of trinity and trinity-fpga, plus 5 `*_params.py` copies of a spec in trinity-fpga. A docstring that says "generated by `t27c`" is prose and does not count (2 hand-written t27 files had it) |
| vendored | 22 | 8,180 | a path component in node_modules, vendor, vendored, third_party, third-party, site-packages, venv, .venv, __pycache__, zig-pkg, dist, build, target. All 22 are the `zig-pkg/<hash>/` emsdk checkout (11 files, 4,090 lines in each of trinity and trinity-fpga) |
| test-only | 138 | 22,920 | a component `tests`, `test`, `__tests__`, or a file named `test_*.py`, `*_test.py`, `conftest.py`. Excluded from the ranking (a port deletes the tests together with the code or not at all), used as oracles in the "oracle" column |
| parse failures | 37 (+1 test-only) | 8,084 | `ast.parse` raises (Jupyter magics such as `!pip` and `%choose`, a line of JavaScript in a `.py` file, a truncated `try` block, unmatched brackets, an em dash in code, an unescaped brace in an f-string). They stay in the line counts, because the gate counts them, and carry no classified function |
| census | 1,943 | 412,066 | what is left: 1,906 parsed files (403,982 lines) and 37 unparsed (8,084) |

## The subset (what "fits" means)

Rules are applied to every function in source order; the first violation by position is the "first blocker" (signature first; `optional return` is positioned at the last line so that it is first only when nothing else blocks). A function is in the typed subset T when no rule fires and every parameter is typed; in the shape subset S when the same holds but a parameter may be untyped.

- R1 kind. A plain `def`. Out: `async`, `yield`, `await`, `lambda`, a nested `def` or `class`, `global`/`nonlocal`, `del`, the walrus, `match`, `try`, `with`, a decorator other than `staticmethod` (`@property`, `@classmethod`, `@functools.lru_cache`, `@torch.no_grad()` and every framework decorator), `*args`/`**kwargs`. Keyword-only parameters are allowed.
- R2 types. A parameter is typed when its annotation is int, float, bool, bytes, str, list/List/Sequence/tuple/Tuple of those (recursively), or a value class of the same file (R7); or when it has no annotation and a literal int/float/bool/str/bytes default (the type is inferred from the literal), or a default that is a module constant of such a type. An annotation outside that grammar (`Optional`, `Union`, `X | None`, `dict`, `set`, `Any`, `Callable`, `Path`, `Fraction`, a class with state, `np.ndarray`) is out as `param type`; the return annotation follows the same grammar plus `None` (`return type`). A function without a return annotation that returns None on one path and a value on another, or falls off the end after returning a value elsewhere, is out as `optional return`. A parameter that has no annotation and no literal default is untyped: the function is in S only.
- R3 names. A name that is read must be a parameter, a local, a module-level constant, a same-module function or value class (R9), `math.*`, or one of the builtins len abs min max sum range int float bool bytes bytearray round pow divmod ord chr list tuple sorted reversed enumerate zip all any. A module-level constant is a name assigned at module level from literals, lists/tuples of literals, arithmetic and comparisons over them, `math.*` calls, or other constants (tuple targets allowed); a constant that a function mutates (`TABLE.append`, `TABLE[i] = x`) is `module state`. Everything else is out with a named reason: `io builtin` (open, print, input, exit), `dynamic` (eval, exec, compile, getattr, setattr, hasattr, globals, locals, vars, type, id, isinstance, issubclass, callable, dir, super, hash), `dict/set` (dict, set, frozenset, defaultdict, Counter, deque, a dict or set literal or comprehension, a dict or set method), `iterator` (map, filter, iter, next), `module state` (a module-level name that is not a constant, `__file__` included), `unknown name`.
- R4 imports. Only math, typing, dataclasses, enum, collections.abc (and `__future__`). A function that reads a name bound by any other import is out: `io module` when the module is one of os, sys, subprocess, socket, requests, urllib, http, pathlib, shutil, glob, tempfile, io, logging, argparse, getpass, sqlite3, asyncio, threading, multiprocessing, signal, json, csv, pickle, time, datetime, random, secrets and a few more network, archive and config modules; `import use` for every other module (numpy, torch, re, struct, hashlib, fractions, a local module). An import the function never reads is ignored.
- R5 strings are read-only text. Allowed: indexing, slicing, len, comparison, `in`, iteration, ord, chr, startswith, endswith, find, rfind, index, count and the is* tests. Out as `owned string`: an f-string with a field, `str()`, `repr`, `format`, `hex`, `bin`, `oct`, `+`, `*` or `%` on a str, and the methods lower upper strip lstrip rstrip replace join split rsplit splitlines format zfill ljust rjust center title capitalize swapcase hex decode encode removeprefix removesuffix partition and their kin. The message of a `raise` or an `assert` is exempt.
- R6 lists and tuples are allowed: literals, indexing, slicing, element assignment, append/extend/pop/insert/sort/reverse/copy, list comprehensions, tuple unpacking, returning a tuple. Out: generator expressions (`generator expression`), set and dict comprehensions, starred expressions. Use of list growth, tuple values and element writes is recorded as a flag and does not exclude (they are the features the Rust census found t27 cannot say, so they are reported below as a second number).
- R7 methods and attributes. A method is classified like a function only when its class is a value class: `@dataclass(frozen=True)` with annotated fields that fit R2, an `Enum`, a `NamedTuple`, or a namespace class that holds only `staticmethod`s. `self.x` may be read when `x` is a declared field. Any assignment to `self.x`, a `@property`, a `@classmethod`, or a method of any other class is out (`stateful class method`, `self state`, `decorator`). An attribute or method on any other value is out as `attribute` or `method outside table`; the allowed methods are the string and list ones named above plus `bit_length`, `bit_count`, `to_bytes` and `int.from_bytes`.
- R8 exceptions. A plain `raise` of a built-in exception (ValueError, TypeError, IndexError, KeyError, ZeroDivisionError, AssertionError, NotImplementedError, OverflowError, RuntimeError, ArithmeticError, Exception) is allowed. `try`, a re-raise, `raise ... from` and every other exception class are out (`try`, `exceptions`).
- R9 calls. A call to a same-module function or value class is allowed only when the callee is in the same tier (a fixpoint over the module; `calls a function that is out`). A call through a local name or a computed callee is out (`call`).
- R10 whole file. Every function and class of the file is in the tier, the module level holds only a docstring, imports, constants (R3), defs and in-subset classes, and there is no `if __name__ == "__main__"`. Near-whole: the same, with `main` and its nested defs left out.
- Line counts. Physical lines are what `git diff --numstat` reports for a deleted file (newline-terminated lines plus an unterminated last line), i.e. what the own-language gate counts. Code lines carry at least one token other than a comment or layout token (`tokenize`), docstring lines included. A function's lines are its span from the first decorator line to `end_lineno` (blank lines, comments and docstring included). Functions are every `def` (module-level, method, nested); lambdas are not counted as functions.

## Totals per repository

| repo | commit | tracked .py | gate-counted lines | generated / vendored | parse failures | test-only | scanned files | lines / code | fns f+m+n | classes in subset / all | T fns / lines | S fns / lines | whole T / S | near-whole S |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| t27 | 0e00974da7 | 655 | 153,503 | 0 / 0 | 7 | 52 | 596 | 141,796 / 116,879 | 3245+509+289 | 5 / 143 | 29 / 289 | 163 / 1,210 | 0 / 1 | 1 |
| trios | 78671ba794 | 12 | 2,382 | 0 / 0 | 0 | 0 | 12 | 2,382 / 1,975 | 43+0+3 | 0 / 0 | 0 / 0 | 1 / 32 | 0 / 0 | 0 |
| trinity | 8e535bc9bb | 378 | 79,821 | 9 / 11 | 15 | 37 | 306 | 71,005 / 56,402 | 1082+1059+52 | 27 / 346 | 86 / 672 | 133 / 946 | 0 / 0 | 0 |
| trinity-fpga | c9b180bf78 | 1059 | 194,623 | 14 / 11 | 16 | 46 | 972 | 184,498 / 150,269 | 4254+1585+292 | 53 / 503 | 114 / 830 | 352 / 3,220 | 0 / 0 | 4 |
| 999-multibots | fcdbb2f7dc | 22 | 4,657 | 0 / 0 | 0 | 2 | 20 | 4,301 / 3,518 | 107+23+4 | 0 / 9 | 1 / 2 | 5 / 36 | 0 / 0 | 0 |
| **five, raw** | | 2,126 | 434,986 | 23 / 22 | 38 | 137 | 1906 | 403,982 / 329,043 | 8731+3176+640 | 85 / 1001 | 230 / 1,793 | 654 / 5,444 | 0 / 1 | 5 |
| **five, distinct** | | | 348,524 | | | 114 | 1594 | 325,897 / 267,886 | 8501 bodies | | 105 / 812 | 454 / 4,090 | 0 / 1 | 5 |

Columns: `gate-counted lines` is every tracked `.py` including generated, vendored and test files, counted as `git diff --numstat` would; `generated / vendored` and `parse failures` and `test-only` are file counts; `lines / code` are the scanned (parsed, non-test) files; `fns` are module-level functions + methods + nested defs; `classes in subset / all` are module-level classes; `999-multibots` is 999-multibots-telegraf; `whole T / S` are whole-file candidates (R10); `near-whole S` leaves `main` out. The raw row repeats copies (trinity and trinity-fpga share byte-identical files); the distinct row counts identical files once and identical function bodies once, so its `T`/`S` counts are bodies, and its line count adds up only the parsed files (325,897; with 5,286 lines in unparsed files the census is 331,183 lines).

## Ranked rows

### Files that hold a typed (T) function, ranked by the lines of those functions (64 distinct files; all of them)

| # | repo | path | lines / code | fns f+m+n / classes | T fns / lines | S fns / lines | oracle | entry | callers | copies | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | trinity | `archive/experiments/golden_key_proof.py` | 442 / 317 | 12+0+0 / 0 | 8 / 96 | 8 / 96 | none | script | amb 1 | 3 | partial |
| 2 | trinity | `archive/experiments/trinity_benchmarks.py` | 389 / 283 | 11+9+0 / 2 | 6 / 51 | 6 / 51 | none | script | none found | 3 | partial |
| 3 | t27 | `conformance/gen_structural_instances.py` | 190 / 169 | 8+0+0 / 0 | 2 / 47 | 2 / 47 | none | script | doc 3 | 0 | partial |
| 4 | trinity | `deploy/scripts/multi_provider_hybrid.py` | 743 / 563 | 12+12+0 / 5 | 6 / 44 | 6 / 44 | none | script | py 3 | 3 | partial |
| 5 | trinity | `tools/scripts/scripts/multi_provider_hybrid.py` | 743 / 563 | 12+12+0 / 5 | 6 / 44 | 6 / 44 | none | script | py 3 | 1 | partial |
| 6 | t27 | `tools/trinity_tri_api.py` | 2334 / 2004 | 103+4+13 / 3 | 5 / 43 | 9 / 65 | 1 inline test fn, 1 asserts | script | py 3, code 4, doc 3 | 0 | partial |
| 7 | t27 | `tools/queen/propose_boundary.py` | 606 / 507 | 18+0+0 / 0 | 3 / 42 | 3 / 42 | 1 inline test fn | script | py 1, code 1, doc 5 | 0 | partial |
| 8 | trinity-fpga | `conformance/wasm_board_vectors.py` | 261 / 222 | 14+2+0 / 2 | 3 / 27 | 4 / 29 | none | script | py 1 | 0 | partial |
| 9 | t27 | `scripts/cocotb_ref_model.py` | 1616 / 1336 | 51+12+1 / 3 | 4 / 23 | 4 / 23 | none | script | code 1, doc 198 | 0 | partial |
| 10 | t27 | `tools/trinity_manifest.py` | 1048 / 935 | 27+0+3 / 0 | 1 / 22 | 1 / 22 | 3 inline test fn | script | code 53, doc 7 | 1 | partial |
| 11 | trinity | `deploy/trinity-nexus/tools/bindings/python/igla.py` | 293 / 224 | 3+14+0 / 2 | 2 / 22 | 2 / 22 | none | script | amb 4 | 3 | partial |
| 12 | trinity-fpga | `research/verify_lucas_product.py` | 207 / 178 | 6+0+0 / 0 | 4 / 21 | 4 / 21 | none | script | none found | 0 | partial |
| 13 | trinity-fpga | `conformance/gf16_conformance_ax7203.py` | 130 / 99 | 6+0+0 / 0 | 2 / 20 | 2 / 20 | 1 asserts | script | none found | 0 | partial |
| 14 | t27 | `tools/trinity_build_receipt.py` | 586 / 529 | 16+0+1 / 0 | 1 / 19 | 1 / 19 | 1 inline test fn | script | code 1, doc 3 | 0 | partial |
| 15 | trinity | `docs/notebooks/quantum/e8_particles.py` | 487 / 352 | 7+17+1 / 8 | 2 / 19 | 2 / 19 | none | script | none found | 1 | partial |
| 16 | t27 | `tools/dupe_scan.py` | 445 / 385 | 12+0+0 / 0 | 1 / 19 | 1 / 19 | 1 inline test fn | script | ci 1, code 11, doc 16 | 0 | partial |
| 17 | trinity-fpga | `research/spectrum/spectrum.py` | 227 / 185 | 11+0+0 / 0 | 4 / 18 | 4 / 18 | none | script | py 2 | 0 | partial |
| 18 | t27 | `contrib/backend/github/auth.py` | 90 / 70 | 0+3+0 / 1 | 1 / 17 | 1 / 17 | 1 test file | module | py 2, doc 1 | 0 | partial |
| 19 | trinity-fpga | `conformance/decimal_ref.py` | 477 / 352 | 14+10+2 / 2 | 1 / 16 | 1 / 16 | 1 inline test fn, 1 asserts | script | py 5 | 0 | partial |
| 20 | t27 | `scripts/trinity-pellis-pipeline/task1_monte_carlo/monte_carlo_exact_pvalue.py` | 299 / 202 | 7+0+0 / 0 | 1 / 16 | 1 / 16 | none | script | doc 1 | 0 | partial |
| 21 | trinity-fpga | `conformance/trinet_mac32_conformance_ax7203.py` | 431 / 329 | 16+0+1 / 0 | 3 / 14 | 3 / 14 | 1 inline test fn, 2 asserts | script | ci 3, py 7 | 0 | partial |
| 22 | t27 | `tools/trinity_vsa_compat.py` | 749 / 650 | 35+0+2 / 0 | 3 / 12 | 11 / 50 | 1 inline test fn | script | code 1, doc 3 | 0 | partial |
| 23 | trinity-fpga | `research/synth_check.py` | 171 / 144 | 6+0+0 / 0 | 1 / 12 | 1 / 12 | none | script | py 1 | 0 | partial |
| 24 | trinity-fpga | `conformance/fp8_ref.py` | 295 / 238 | 9+13+1 / 2 | 1 / 11 | 1 / 11 | 1 inline test fn, 1 asserts | script | py 22, amb 1 | 1 | partial |
| 25 | trinity | `deploy/trinity-nexus/docs/book/generator/book_generator_v2.py` | 286 / 260 | 5+0+0 / 0 | 2 / 11 | 2 / 11 | none | script | none found | 1 | partial |
| 26 | trinity | `docs/book/book/generator/book_generator_v2.py` | 286 / 260 | 5+0+0 / 0 | 2 / 11 | 2 / 11 | none | script | none found | 1 | partial |
| 27 | trinity | `deploy/trinity-nexus/docs/book/generator/templates.py` | 448 / 397 | 4+0+0 / 0 | 1 / 10 | 1 / 10 | none | script | none found | 1 | partial |
| 28 | trinity | `docs/book/book/generator/templates.py` | 448 / 397 | 4+0+0 / 0 | 1 / 10 | 1 / 10 | none | script | none found | 1 | partial |
| 29 | trinity | `deploy/trinity-nexus/docs/book/generator/book_generator.py` | 420 / 338 | 8+0+0 / 3 | 2 / 10 | 2 / 10 | none | script | none found | 1 | partial |
| 30 | trinity | `docs/book/book/generator/book_generator.py` | 420 / 338 | 8+0+0 / 3 | 2 / 10 | 2 / 10 | none | script | none found | 1 | partial |
| 31 | trinity | `deploy/trinity-nexus/docs/book/generator/llm_book_generator.py` | 275 / 233 | 6+0+0 / 0 | 2 / 10 | 2 / 10 | none | script | none found | 1 | partial |
| 32 | trinity | `docs/book/book/generator/llm_book_generator.py` | 275 / 233 | 6+0+0 / 0 | 2 / 10 | 2 / 10 | none | script | none found | 0 | partial |
| 33 | trinity-fpga | `docs/book/book/generator/llm_book_generator.py` | 275 / 233 | 6+0+0 / 0 | 2 / 10 | 2 / 10 | none | script | none found | 0 | partial |
| 34 | trinity-fpga | `research/witness_narrowing.py` | 142 / 128 | 5+0+0 / 0 | 1 / 9 | 3 / 30 | none | script | none found | 0 | partial |
| 35 | t27 | `tools/trinity_tri_api_session.py` | 1394 / 1200 | 65+0+13 / 0 | 1 / 9 | 3 / 17 | 1 inline test fn, 1 asserts | script | code 1, doc 3 | 0 | partial |
| 36 | trinity-fpga | `research/spectrum/pm2_core.py` | 272 / 220 | 19+0+1 / 0 | 3 / 8 | 5 / 17 | 1 asserts | module | py 3 | 0 | partial |
| 37 | trinity | `deploy/trinity-nexus/docs/book/generator/llm_deep_generator.py` | 463 / 403 | 5+0+0 / 0 | 2 / 8 | 2 / 8 | none | script | none found | 1 | partial |
| 38 | trinity | `docs/book/book/generator/llm_deep_generator.py` | 463 / 403 | 5+0+0 / 0 | 2 / 8 | 2 / 8 | none | script | none found | 0 | partial |
| 39 | trinity-fpga | `docs/book/book/generator/llm_deep_generator.py` | 463 / 403 | 5+0+0 / 0 | 2 / 8 | 2 / 8 | none | script | none found | 0 | partial |
| 40 | trinity | `targets/kaggle/kaggle/eval/runner.py` | 567 / 427 | 12+0+0 / 4 | 1 / 7 | 1 / 7 | none | script | none found | 1 | partial |
| 41 | trinity | `targets/kaggle/kaggle/generators/gen_tefb.py` | 780 / 677 | 8+0+0 / 1 | 1 / 6 | 1 / 6 | none | script | none found | 1 | partial |
| 42 | trinity | `targets/kaggle/kaggle/generators/gen_tagp.py` | 554 / 465 | 8+0+0 / 1 | 1 / 6 | 1 / 6 | none | script | none found | 1 | partial |
| 43 | trinity | `targets/kaggle/kaggle/generators/gen_tscp.py` | 525 / 435 | 8+0+0 / 1 | 1 / 6 | 1 / 6 | none | script | none found | 1 | partial |
| 44 | trinity | `targets/kaggle/kaggle/generators/gen_thlp.py` | 450 / 364 | 9+0+0 / 1 | 1 / 6 | 1 / 6 | none | script | none found | 1 | partial |
| 45 | trinity | `deploy/trinity-nexus/docs/book/generator/literary_templates_v2.py` | 423 / 340 | 4+2+0 / 2 | 2 / 6 | 2 / 6 | none | script | none found | 1 | partial |
| 46 | trinity | `docs/book/book/generator/literary_templates_v2.py` | 423 / 340 | 4+2+0 / 2 | 2 / 6 | 2 / 6 | none | script | none found | 1 | partial |
| 47 | trinity | `targets/kaggle/kaggle/generators/gen_tmp.py` | 275 / 221 | 4+0+0 / 1 | 1 / 6 | 1 / 6 | none | script | none found | 1 | partial |
| 48 | trinity-fpga | `research/verify_lucas_exact.py` | 95 / 78 | 2+0+0 / 0 | 1 / 6 | 1 / 6 | none | script | py 1 | 0 | partial |
| 49 | t27 | `tools/trinity_tri27.py` | 1090 / 987 | 29+8+3 / 1 | 2 / 5 | 2 / 5 | 1 inline test fn | script | code 4, doc 3 | 0 | partial |
| 50 | t27 | `tools/check_documented_commands_exist.py` | 786 / 593 | 19+0+0 / 0 | 1 / 5 | 1 / 5 | 1 inline test fn | script | ci 1, py 3, doc 8 | 0 | partial |
| 51 | t27 | `repro/numerics/nmse_gf16.py` | 406 / 328 | 15+0+0 / 0 | 1 / 5 | 1 / 5 | none | script | ci 1, sh 3, py 1, code 1, doc 2 | 0 | partial |
| 52 | trinity | `experiments/backward/overfit_100/run.py` | 90 / 70 | 2+0+0 / 0 | 1 / 4 | 1 / 4 | none | script | none found | 0 | partial |
| 53 | t27 | `tools/queen/feed_roadmap.py` | 1030 / 863 | 32+0+1 / 2 | 1 / 3 | 1 / 3 | 1 inline test fn | script | ci 1, py 1, doc 2 | 0 | partial |
| 54 | trinity | `deploy/trinity-nexus/docs/book/generator/zhar_ptitsa_generator.py` | 527 / 413 | 2+10+0 / 5 | 1 / 3 | 1 / 3 | none | script | none found | 1 | partial |
| 55 | trinity | `docs/book/book/generator/zhar_ptitsa_generator.py` | 527 / 413 | 2+10+0 / 5 | 1 / 3 | 1 / 3 | none | script | none found | 1 | partial |
| 56 | trinity-fpga | `fpga/witness/gf_decode/gf_decode_ref.py` | 501 / 376 | 18+13+0 / 3 | 1 / 3 | 1 / 3 | 3 asserts | script | py 2 | 0 | partial |
| 57 | trinity-fpga | `conformance/tnf_ladder_versions.py` | 95 / 72 | 3+0+0 / 0 | 1 / 3 | 1 / 3 | 2 asserts | script | ci 1, py 3, amb 3 | 1 | partial |
| 58 | t27 | `tools/queen/criteria_backfill.py` | 1491 / 1285 | 44+12+6 / 3 | 1 / 2 | 1 / 2 | 1 inline test fn | script | py 1, code 1, doc 3 | 0 | partial |
| 59 | 999-multibots | `deployment/voice-clone/server.py` | 452 / 372 | 12+5+0 / 2 | 1 / 2 | 1 / 2 | none | script | py 2, amb 3 | 0 | partial |
| 60 | trinity | `tools/conformance_check.py` | 218 / 182 | 6+0+0 / 0 | 1 / 2 | 1 / 2 | none | script | ci 2 | 0 | partial |
| 61 | trinity | `tools/corona_conformance.py` | 191 / 153 | 4+0+0 / 0 | 1 / 2 | 1 / 2 | none | script | none found | 0 | partial |
| 62 | trinity-fpga | `conformance/tnf16_ref.py` | 132 / 101 | 8+0+0 / 0 | 1 / 2 | 1 / 2 | 1 inline test fn, 3 asserts | script | py 4 | 0 | partial |
| 63 | trinity-fpga | `tools/spec_dialects.py` | 121 / 99 | 4+0+0 / 0 | 1 / 2 | 1 / 2 | 1 test file | script | ci 1, py 1 | 0 | partial |
| 64 | trinity-fpga | `conformance/tnf_ref.py` | 245 / 178 | 12+8+1 / 1 | 1 / 1 | 1 / 1 | 3 test files, 1 inline test fn, 3 asserts | script | ci 1, py 46, amb 8 | 1 | partial |

### Files that hold a shape (S) function, ranked by the lines of those functions (top 25 of 275)

| # | repo | path | lines / code | fns f+m+n / classes | T fns / lines | S fns / lines | oracle | entry | callers | copies | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | trinity-fpga | `fpga/witness/gf_decode/rtl_bit_model.py` | 423 / 317 | 13+0+0 / 0 | 0 / 0 | 6 / 169 | 1 asserts | script | py 1 | 0 | partial |
| 2 | trinity-fpga | `formal/verify_adder_oracle.py` | 329 / 259 | 6+0+0 / 0 | 0 / 0 | 3 / 144 | none | script | ci 1 | 0 | partial |
| 3 | t27 | `tools/gft_train_demo.py` | 201 / 168 | 17+0+1 / 0 | 0 / 0 | 16 / 130 | none | script | py 1, doc 3 | 0 | near-whole (S) |
| 4 | trinity-fpga | `conformance/corona_decode_host_ax7203.py` | 381 / 304 | 16+0+0 / 0 | 0 / 0 | 8 / 129 | 1 inline test fn | script | ci 1, sh 3, py 2 | 0 | partial |
| 5 | t27 | `tools/gft_backprop_microcode.py` | 967 / 684 | 19+0+24 / 0 | 0 / 0 | 13 / 114 | 1 inline test fn, 38 asserts | script | ci 1, py 4, code 1, doc 10 | 0 | partial |
| 6 | trinity | `archive/experiments/golden_key_proof.py` | 442 / 317 | 12+0+0 / 0 | 8 / 96 | 8 / 96 | none | script | amb 1 | 3 | partial |
| 7 | trinity-fpga | `conformance/gf96_bitexact_oracle.py` | 326 / 249 | 7+0+0 / 0 | 0 / 0 | 2 / 69 | none | script | none found | 0 | partial |
| 8 | t27 | `tools/trinity_tri_api.py` | 2334 / 2004 | 103+4+13 / 3 | 5 / 43 | 9 / 65 | 1 inline test fn, 1 asserts | script | py 3, code 4, doc 3 | 0 | partial |
| 9 | trinity-fpga | `conformance/gf128_bitexact_oracle.py` | 273 / 226 | 7+0+0 / 0 | 0 / 0 | 2 / 62 | none | script | none found | 0 | partial |
| 10 | trinity-fpga | `conformance/gf48_bitexact_oracle.py` | 295 / 218 | 9+0+0 / 0 | 0 / 0 | 2 / 59 | none | script | py 1 | 0 | partial |
| 11 | trinity-fpga | `conformance/gf256_bitexact_oracle.py` | 253 / 222 | 7+0+0 / 0 | 0 / 0 | 2 / 59 | none | script | none found | 0 | partial |
| 12 | trinity-fpga | `conformance/gf512_bitexact_oracle.py` | 223 / 196 | 7+0+0 / 0 | 0 / 0 | 2 / 59 | none | script | none found | 0 | partial |
| 13 | trinity-fpga | `conformance/gf1024_bitexact_oracle.py` | 221 / 194 | 7+0+0 / 0 | 0 / 0 | 2 / 59 | none | script | none found | 0 | partial |
| 14 | trinity-fpga | `conformance/top_decode_conformance_ax7203.py` | 134 / 110 | 7+0+0 / 0 | 0 / 0 | 4 / 55 | none | script | py 1 | 0 | partial |
| 15 | trinity | `archive/experiments/trinity_benchmarks.py` | 389 / 283 | 11+9+0 / 2 | 6 / 51 | 6 / 51 | none | script | none found | 3 | partial |
| 16 | trinity-fpga | `formal/prove_gf16_inf_fix.py` | 135 / 114 | 8+0+0 / 0 | 0 / 0 | 7 / 51 | none | script | none found | 0 | partial |
| 17 | t27 | `tools/trinity_vsa_compat.py` | 749 / 650 | 35+0+2 / 0 | 3 / 12 | 11 / 50 | 1 inline test fn | script | code 1, doc 3 | 0 | partial |
| 18 | t27 | `tools/ternary_model.py` | 93 / 67 | 12+0+0 / 0 | 0 / 0 | 12 / 50 | none | module | py 1, code 1, doc 5 | 0 | whole (S) |
| 19 | t27 | `conformance/gen_structural_instances.py` | 190 / 169 | 8+0+0 / 0 | 2 / 47 | 2 / 47 | none | script | doc 3 | 0 | partial |
| 20 | t27 | `conformance/vectors/gen_all_formats.py` | 1366 / 1053 | 33+0+16 / 0 | 0 / 0 | 3 / 46 | 1 asserts | script | ci 1, py 3, doc 3 | 0 | partial |
| 21 | trinity | `deploy/scripts/multi_provider_hybrid.py` | 743 / 563 | 12+12+0 / 5 | 6 / 44 | 6 / 44 | none | script | py 3 | 3 | partial |
| 22 | trinity | `tools/scripts/scripts/multi_provider_hybrid.py` | 743 / 563 | 12+12+0 / 5 | 6 / 44 | 6 / 44 | none | script | py 3 | 1 | partial |
| 23 | trinity-fpga | `conformance/posit16_decode_conformance_ax7203.py` | 129 / 89 | 5+0+0 / 0 | 0 / 0 | 1 / 44 | 1 inline test fn | script | py 2 | 0 | partial |
| 24 | t27 | `tools/queen/propose_boundary.py` | 606 / 507 | 18+0+0 / 0 | 3 / 42 | 3 / 42 | 1 inline test fn | script | py 1, code 1, doc 5 | 0 | partial |
| 25 | trinity-fpga | `research/block/scale_axis_probe.py` | 148 / 116 | 9+0+0 / 0 | 0 / 0 | 5 / 41 | none | script | py 1 | 0 | partial |

### The 20 largest files (the debt), with the share that fits S

| # | repo | path | lines / code | fns f+m+n | T fns / lines | S fns / lines | S lines share |
|---|---|---|---|---|---|---|---|
| 1 | t27 | `tools/bees/reviewer.py` | 3844 / 3228 | 97+36+24 | 0 / 0 | 2 / 14 | 0.4% |
| 2 | t27 | `tools/trinity_tri_api.py` | 2334 / 2004 | 103+4+13 | 5 / 43 | 9 / 65 | 2.8% |
| 3 | t27 | `scripts/tri_loop/t27b.py` | 1864 / 1671 | 55+23+4 | 0 / 0 | 0 / 0 | 0.0% |
| 4 | t27 | `scripts/cocotb_ref_model.py` | 1616 / 1336 | 51+12+1 | 4 / 23 | 4 / 23 | 1.4% |
| 5 | t27 | `tools/trinity_queen_views.py` | 1555 / 1384 | 48+0+4 | 0 / 0 | 3 / 24 | 1.5% |
| 6 | trinity-fpga | `parameter_golf_gf8_ablation/train_gpt_cuda_gf8.py` | 1503 / 1298 | 23+39+6 | 0 / 0 | 0 / 0 | 0.0% |
| 7 | t27 | `tools/queen/criteria_backfill.py` | 1491 / 1285 | 44+12+6 | 1 / 2 | 1 / 2 | 0.1% |
| 8 | trinity-fpga | `parameter_golf_gf8_ablation/train_gpt_cuda_ternary.py` | 1436 / 1232 | 22+39+3 | 0 / 0 | 0 / 0 | 0.0% |
| 9 | t27 | `tools/trinity_tri_api_session.py` | 1394 / 1200 | 65+0+13 | 1 / 9 | 3 / 17 | 1.2% |
| 10 | t27 | `conformance/vectors/gen_all_formats.py` | 1366 / 1053 | 33+0+16 | 0 / 0 | 3 / 46 | 3.4% |
| 11 | t27 | `tools/trinity_tri27.py` | 1090 / 987 | 29+8+3 | 2 / 5 | 2 / 5 | 0.5% |
| 12 | t27 | `tools/trinity_manifest.py` | 1048 / 935 | 27+0+3 | 1 / 22 | 1 / 22 | 2.1% |
| 13 | trinity-fpga | `research/arxiv_tnf/verify_numbers.py` | 1044 / 926 | 5+0+0 | 0 / 0 | 0 / 0 | 0.0% |
| 14 | t27 | `tools/queen/feed_roadmap.py` | 1030 / 863 | 32+0+1 | 1 / 3 | 1 / 3 | 0.3% |
| 15 | t27 | `contrib/railway/t27b-lab/lab.py` | 1006 / 854 | 34+5+10 | 0 / 0 | 1 / 4 | 0.4% |
| 16 | t27 | `tools/gft_backprop_microcode.py` | 967 / 684 | 19+0+24 | 0 / 0 | 13 / 114 | 11.8% |
| 17 | t27 | `tools/trinity_queen_dispatch.py` | 948 / 871 | 20+0+3 | 0 / 0 | 1 / 2 | 0.2% |
| 18 | t27 | `tools/gen_formats_catalog.py` | 908 / 802 | 22+0+0 | 0 / 0 | 0 / 0 | 0.0% |
| 19 | trinity | `deploy/trinity-nexus/deploy/vercel-agent/api/index.py` | 876 / 726 | 0+20+0 | 0 / 0 | 0 / 0 | 0.0% |
| 20 | trinity | `deploy/trinity-nexus/tools/mcp/igla_server.py` | 864 / 761 | 1+24+0 | 0 / 0 | 0 / 0 | 0.0% |

Reading the ranked tables: `lines / code` are physical and code lines; `fns f+m+n` are module-level functions, methods and nested defs; `classes` are module-level classes; `T` and `S` are the distinct-file's own in-subset function counts and spans; `oracle` lists test files that import the module, inline test functions, `assert` statements in the file, doctests; `entry` is `script` when the file has an `if __name__` guard; `callers` counts other files: `ci` workflows and other YAML, `sh` shell, Makefile, Dockerfile, `py` other `.py` files that import it or name it, `code` Rust, t27, Zig, TypeScript, JavaScript, Go and C sources, `doc` Markdown and text, `amb` mentions by a basename that other files of the repository share; `copies` is the number of other paths or repositories with byte-identical content (counted once in the table and in the distinct totals). For trios, trinity, trinity-fpga and 999-multibots-telegraf only the checked-out file types were searched for callers (`.py`, shell, YAML, Makefile, Dockerfile, package.json, pyproject.toml), so `code` and `doc` are undercounts there.

## First blockers (distinct function bodies)

| first blocker | distinct fns | lines | share of fns |
|---|---|---|---|
| stateful class method | 1,195 | 22,315 | 14.1% |
| io module | 973 | 37,553 | 11.4% |
| import use | 968 | 18,421 | 11.4% |
| owned string | 717 | 25,750 | 8.4% |
| param type | 624 | 15,805 | 7.3% |
| nested def | 588 | 9,791 | 6.9% |
| dict/set | 427 | 12,671 | 5.0% |
| return type | 353 | 7,480 | 4.2% |
| shape fits, parameter untyped | 349 | 3,278 | 4.1% |
| io builtin | 343 | 15,601 | 4.0% |
| module state | 333 | 7,684 | 3.9% |
| method outside table | 245 | 2,678 | 2.9% |
| decorator | 191 | 1,791 | 2.2% |
| attribute | 178 | 3,767 | 2.1% |
| try | 148 | 2,502 | 1.7% |
| dynamic | 127 | 2,457 | 1.5% |
| generator expression | 124 | 2,297 | 1.5% |
| in subset (typed) | 105 | 812 | 1.2% |
| async | 96 | 2,890 | 1.1% |
| calls a function that is out | 87 | 599 | 1.0% |
| with | 78 | 2,159 | 0.9% |
| varargs | 45 | 311 | 0.5% |
| lambda | 40 | 1,014 | 0.5% |
| call | 33 | 415 | 0.4% |
| optional return | 25 | 355 | 0.3% |
| class instance | 21 | 826 | 0.2% |
| exceptions | 17 | 294 | 0.2% |
| param default | 16 | 597 | 0.2% |
| attribute assignment | 12 | 1,106 | 0.1% |
| starred | 11 | 179 | 0.1% |
| global/nonlocal | 10 | 182 | 0.1% |
| unknown name | 8 | 85 | 0.1% |
| local type | 6 | 326 | 0.1% |
| expression | 2 | 8 | 0.0% |
| walrus | 2 | 36 | 0.0% |
| generator | 1 | 36 | 0.0% |
| nested class | 1 | 30 | 0.0% |
| iterator | 1 | 72 | 0.0% |
| self state | 1 | 4 | 0.0% |

- Methods of classes with state are the largest wall (1,195 distinct functions, 14.1%), then imports of a module outside the allowed five (1,941, 22.8%; numpy 259, argparse 231, os 183, sys 179, random 105, re 102, serial 95, subprocess 94, torch 77, fractions 65), owned strings (717, 8.4%), annotations outside the grammar (977), nested defs (588) and dict/set (427).
- 349 distinct functions (3,278 lines) pass every structural rule and fail only because a parameter has no annotation and no literal default: that is the whole gap between T and S, and what trace-based typing (#9006) would buy.
- Input and output dominate: imports of other modules, `io builtin` (343), `with` and `try` (226) are the first blocker of 29.5% of the distinct functions.

What would move the number: the functions that fail for exactly one category, so that allowing that category (and nothing else) would admit them.

| the only blocker | distinct fns | lines |
|---|---|---|
| owned string | 166 | 2,075 |
| decorator | 88 | 268 |
| dict/set | 76 | 649 |
| generator expression | 64 | 477 |
| method outside table | 57 | 238 |
| attribute | 40 | 510 |
| module state | 39 | 292 |
| import use (pure stdlib module) | 35 | 617 |
| nested def | 31 | 117 |
| optional return | 25 | 355 |
| stateful class method | 25 | 166 |
| import use | 19 | 133 |

## First whole-file candidate

- Distinct files of 150 lines or more: 737. With every function in T: 0. With every function in S: 1 (`archive/experiments/kinetic_sacred_pure.py`, which runs 82 module-level statements and is therefore not whole). Files of any size with every function in S: 4 (`tools/ternary_model.py` (93 lines), `archive/experiments/kinetic_sacred_pure.py` (256 lines), `parameter_golf_gf8_ablation/gfplus_line/testE_v3.py` (57 lines), `research/arxiv_tnf/gen_figures_prior.py` (129 lines)); only the first has no module-level effect (the others run 82, 21 and 46 module-level statements).
- Largest share of a 150-line-plus file inside S: `tools/gft_train_demo.py` 16 of 18 fns, 130 of 201 lines, `formal/verify_adder_oracle.py` 3 of 6 fns, 144 of 329 lines, `fpga/witness/gf_decode/rtl_bit_model.py` 6 of 13 fns, 169 of 423 lines, `conformance/corona_decode_host_ax7203.py` 8 of 16 fns, 129 of 381 lines.
- Near-whole in S (everything but `main` fits): 5 files, `tools/gft_train_demo.py` (201 lines), `conformance/binary256_decode_conformance_ax7203.py` (91 lines), `conformance/bram_test_conformance_ax7203.py` (43 lines), `research/arxiv_tnf/measurements/gen_topaligned_cost.py` (71 lines), `research/block/verify_phase_theorem.py` (146 lines). Only `tools/gft_train_demo.py` is above 150 lines, and it is imported by `tools/gft_deep_demo.py` (six of its functions) and described in `docs/GFT_TRAINING_DEMO.md`; it has a spec twin `specs/port/tools/gft_train_demo.t27` and no test.
- Already ported: in t27, 57 of 596 scanned `.py` files have a spec of the same path under `specs/port/` (published by the port bees, 93 `docs/now/*-published-port-*-py-python-*` entries, 92 of which name a script; that script is still tracked for 41 of them); 7 of the 76 t27 files that hold S functions have one (204 S lines). Equivalence of those twins was not checked here.

## Hand check (30 functions)

Method. Sample drawn with a seeded generator (seed 20261021) from the functions of the distinct files (byte-identical copies counted once): 15 from the in-subset (T) set, 4 uniformly from the out set, 5 from the shape-fits-but-untyped set, 6 from out functions that have exactly one violation category (the borderline ones, where a scanner bug would flip the verdict). The 30 were shuffled, printed as source with their imports and no verdict, and classified by hand against R1 to R10 (the callee, class and module constant were looked up when the rule needed them); the verdicts were written to a file, then compared with the scanner's.

- Round 1 (seed 20261020, scanner before the fix): 29 of 30 agreed. The disagreement was `extract_field` in `tools/trinity_tri_api.py`, which returns None on some paths and bytes on others: a union that the scanner passed because it checks annotations only. Rule `optional return` (R2) was added; one more rule expected to matter was added in the same edit, not found by the sample (a module list mutated through `TABLE.append` or `TABLE[i] = x` is `module state`). The classification of the 12,547 functions moved by 20 S and 6 T functions.
- Round 2 (seed 20261021, final scanner, the table below): 30 of 30 verdicts agree on both tiers. In-class precision 15 of 15 (95% one-sided lower bound 82%); out-class 15 of 15; first-blocker reason agrees in 14 of 15 out functions (in `accumulate` the scanner found an import I had missed; the verdict agreed).
- What the hand check does not show: the sample is small; the rules and the check are by the same author; recall is measured on 15 out functions (4 uniform, 5 shape-fits-but-untyped, 6 one-category), not over the 8,400 out functions.

| # | stratum | function | hand T / S | scanner T / S | hand reason | scanner first blocker |
|---|---|---|---|---|---|---|
| 1 | out, uniform | `e06` in `t27:tools/trinity_tri_api.py` | OUT / OUT | OUT / OUT | decorator | decorator: @scenario('e06-turn-limit', ... |
| 2 | scanner IN | `toaboutaboutrdandonty` in `trinity:deploy/trinity-nexus/docs/book/generator/book_generator.py` | IN / IN | IN / IN | - | - |
| 3 | scanner IN | `mf_encode` in `t27:conformance/gen_structural_instances.py` | IN / IN | IN / IN | - | - |
| 4 | scanner IN | `quicksort_3way` in `trinity:archive/experiments/trinity_benchmarks.py` | IN / IN | IN / IN | - | - |
| 5 | out, uniform | `test_phi_squared` in `t27:benchmarks/language_tests/python_float64.py` | OUT / OUT | OUT / OUT | return type dict | return type: Dict[str, Any] |
| 6 | scanner IN | `calculate_phi_score` in `trinity:targets/kaggle/kaggle/generators/gen_tmp.py` | IN / IN | IN / IN | - | - |
| 7 | scanner IN | `max_new_tokens` in `999-multibots:deployment/voice-clone/server.py` | IN / IN | IN / IN | - | - |
| 8 | shape fits, untyped | `int_levels` in `trinity-fpga:research/block/bitfair_composition.py` | OUT / IN | OUT / IN | untyped parameter | untyped parameter: nbits |
| 9 | scanner IN | `py_fibonacci` in `trinity:deploy/trinity-nexus/tools/bindings/python/igla.py` | IN / IN | IN / IN | - | - |
| 10 | out, uniform | `accumulate` in `trinity-fpga:conformance/true_width_ladder.py` | OUT / OUT | OUT / OUT | isinstance / Optional return | import use: fractions |
| 11 | out, one category | `indent` in `t27:scripts/gen_w760.py` | OUT / OUT | OUT / OUT | owned string | owned string: str * |
| 12 | scanner IN | `coords` in `trinity:docs/book/book/generator/llm_book_generator.py` | IN / IN | IN / IN | - | - |
| 13 | scanner IN | `proof_golden_key` in `trinity:archive/experiments/golden_key_proof.py` | IN / IN | IN / IN | - | - |
| 14 | scanner IN | `calculate_phi_score` in `trinity:targets/kaggle/kaggle/generators/gen_thlp.py` | IN / IN | IN / IN | - | - |
| 15 | out, uniform | `ModelBoard.read` in `trinity-fpga:conformance/tnf16_board_ax7203.py` | OUT / OUT | OUT / OUT | stateful class method | stateful class method: ModelBoard (stateful) |
| 16 | out, one category | `indent` in `t27:scripts/gen_w715.py` | OUT / OUT | OUT / OUT | owned string | owned string: str * |
| 17 | out, one category | `indent` in `t27:scripts/gen_w783.py` | OUT / OUT | OUT / OUT | owned string | owned string: str * |
| 18 | shape fits, untyped | `decode_e5m2` in `t27:conformance/vectors/gen_fp8_e5m2.py` | OUT / IN | OUT / IN | untyped parameter | untyped parameter: bits |
| 19 | out, one category | `sci` in `trinity-fpga:research/arxiv_tnf/recompute_downstream_table.py` | OUT / OUT | OUT / OUT | owned string | owned string: f-string |
| 20 | scanner IN | `cases` in `trinity-fpga:research/witness_narrowing.py` | IN / IN | IN / IN | - | - |
| 21 | shape fits, untyped | `neuron_table` in `t27:experiments/gfternary-line/gen_sparse.py` | OUT / IN | OUT / IN | untyped parameter | untyped parameter: weights,thr,in_bits |
| 22 | scanner IN | `sacred_formula` in `trinity:docs/book/book/generator/zhar_ptitsa_generator.py` | IN / IN | IN / IN | - | - |
| 23 | shape fits, untyped | `_valid_splits` in `trinity-fpga:parameter_golf_gf8_ablation/gfplus_line/selector_vs_intrapocket.py` | OUT / IN | OUT / IN | untyped parameter | untyped parameter: N |
| 24 | scanner IN | `GFFormat.__post_init__` in `trinity-fpga:fpga/witness/gf_decode/gf_decode_ref.py` | IN / IN | IN / IN | - | - |
| 25 | scanner IN | `has_vibee_name` in `trinity-fpga:tools/spec_dialects.py` | IN / IN | IN / IN | - | - |
| 26 | scanner IN | `verify_phi_identity` in `trinity:deploy/scripts/multi_provider_hybrid.py` | IN / IN | IN / IN | - | - |
| 27 | shape fits, untyped | `qfmt` in `trinity-fpga:research/block/ternary_accumulator.py` | OUT / IN | OUT / IN | untyped parameter | untyped parameter: v,M,nbin |
| 28 | out, one category | `_enumerate_regimes` in `trinity-fpga:conformance/tekum_ref.py` | OUT / OUT | OUT / OUT | property on a value class | attribute: .regime_count |
| 29 | out, one category | `theorem_block` in `t27:scripts/gen_w386_lean.py` | OUT / OUT | OUT / OUT | owned string | owned string: f-string |
| 30 | scanner IN | `toaboutaboutrdandonty` in `trinity:deploy/trinity-nexus/docs/book/generator/book_generator_v2.py` | IN / IN | IN / IN | - | - |

## Callers and entry points

- Of the 64 distinct files that hold a T function: 62 are scripts (an `if __name__` guard), 9 are run or named by a workflow, shell script, Makefile or Dockerfile, 15 are imported by another `.py`, and 29 have no reference found at all. Of the 275 files that hold an S function: 224 are scripts.
- A port has to keep the entry point of a script (`python3 path.py` in a workflow, a shell script or a doc) and the import surface of a module (`from tools.x import f`). `t27c gen-python` cannot give back a function: `bootstrap/src/codegen_python.rs` states that a `fn` is "announced, not stubbed", and its test asserts the output contains no `def `. The only existing way for Python to call spec logic is a hand-written loader: `scripts/tri_loop/t27b_rules.py` (315 lines) compiles `gen/c/tri/t27b/steward.c` and calls it with ctypes. Of the 105 T bodies, 60 (361 lines) are flat numeric code with no list growth, no tuple value and no element write; the others use at least one of those, which a plain scalar C signature does not carry (my reading, not measured).
- What a caller would call after a port: for the module-level pure functions, the same name through a loader shim that is itself hand-written Python and counts against the foreign-line budget (no net-negative diff unless the functions deleted outnumber the shim); or, if the caller is a script that only runs the spec, `tri`/`t27c` directly with the caller changed. With a median in-subset function of 6 lines, a shim of 15 to 40 lines costs more than it deletes unless several functions of one file move together. This is the reason to measure a file before choosing it, and why the whole-file number above is the one that counts.

## What this does not establish

- Not run: no `t27c`, no `cargo`, no `tri` build, no converter. There is no Python front end yet, so "fits the subset" means "passes the written rules", not "converts and verifies". `t27c gen-python` was read, not executed. No oracle replay and no equivalence check of any port twin.
- The subset is ours (#9006 may choose another). T and S, the import list, and the treatment of list growth and tuples are judgment calls; the first-blocker histogram and the "only blocker" table above show how far each choice moves the number.
- Near-duplicates (copies that differ by a few bytes) are collapsed only when their function bodies are identical by `ast.dump`; files are collapsed only when byte-identical.
- Callers in the four sibling repositories were searched in the checked-out file types only; basename matches are labelled `amb` when another file of the same repository shares the name. Dynamic imports (`importlib`, `runpy`) are found only when the path or file name appears as a string token.
- The local checkouts under the user's home were not used (stale feature branches); the numbers are for the pinned commits. A run on a later master will differ.
- The hand check was done by the author of the rules; an independent reviewer would be a stronger check.

## Reproduce

The scripts are throw-away and live outside the tree (a new `.py` file is denied by the own-language gate; the rules R1 to R10 above are the specification). They ran with `python3 -I` on Python 3.14.6. sha256 prefixes: scan 50228b3989d7, post 61c9c9ed582a, report 89f4fcc59bd2, render ad32c485cbda.

1. Fetch each sibling repository at its pinned commit: `git init; git remote add origin <url>; git fetch --depth 1 --filter=blob:none origin <sha>; git sparse-checkout set --no-cone '*.py' '*.sh' '*.yml' '*.yaml' Makefile 'Dockerfile*' package.json pyproject.toml; git checkout FETCH_HEAD`.
2. For each repository list `git ls-tree -r --name-only HEAD`, keep `*.py`, apply the exclusions above, `ast.parse` each file, walk every `def` with the rules R1 to R9, count lines as stated, hash each function with `ast.dump`.
3. Index the path-like tokens ending in `.py` of the shell, YAML, Markdown and source files (`git grep -I -o -E '[A-Za-z0-9_./-]+[.]py'` in t27, `grep -rIo` in the sparse checkouts) and the imports of every `.py` file (`ast`), and match them to the census rows.
4. Draw the hand-check sample with `random.Random(seed).sample` from the stated strata.
