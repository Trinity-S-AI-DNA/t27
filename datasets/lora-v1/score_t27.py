#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
# /Users/playom/t27/datasets/lora-v1/score_t27.py
#
# Mechanical scorer for generated .t27 source, graded against the measured
# gold corpus at /Users/playom/t27/specs/.
#
# Python 3 standard library ONLY. No third-party imports. Nothing installed.
#
# WHY THIS EXISTS
# ---------------
# `t27c typecheck --json` is NOT a usable gate. Measured 2026-09-09 on this
# machine, it returns {"ok": true, "errors": 0} for:
#   - 635 words of English prose
#   - a valid Python 3 module
#   - a zero-byte file
#   - a .t27 file whose every function body is a `// TODO: Implement` stub
# So the scorer never trusts typecheck alone. The load-bearing signals are the
# parser's recovery counters, a declaration-count floor that excludes imports,
# and a per-function AST-node census from `t27c metrics` that detects empty
# bodies exactly.
#
# USAGE
#   python3 score_t27.py FILE.t27                # score one file
#   cat gen.t27 | python3 score_t27.py -         # score stdin
#   python3 score_t27.py FILE.t27 --verbose      # include raw tool output tails
#   python3 score_t27.py A.t27 B.t27 ... --tsv   # batch, one TSV row per file
#   python3 score_t27.py --selftest              # run the built-in proof suite

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile

T27C = "/Users/playom/t27/target/release/t27c"
GOLD_ROOT = "/Users/playom/t27/specs"

# ===========================================================================
# CALIBRATION CONSTANTS
# ===========================================================================
# Measured 2026-09-09 on this machine by running t27c (parse / size / count /
# typecheck --json / lint --json / metrics) over every *.t27 under
# /Users/playom/t27/specs/ with any path segment named "scratch" excluded.
# That walk found 506 files. t27c build: /Users/playom/t27/target/release/t27c,
# 7,827,680 bytes, mtime 2026-09-08 14:59. Full sweep: 506 files x 5 t27c
# invocations in 15.8 s wall.
#
# The 506 gold files are NOT one population. Measured dialect split:
#     module_semi   235   `module X;`   -- 92.1% of its fn-bearing files have
#                                          EVERY function body empty
#     module_brace  187   `module X {`  -- 12.7% all-empty
#     other          58   (mostly `module X {` variants)   4.0% all-empty
#     markdown       15   prose, 0% parse-clean
#     specDSL         8   `spec X { }`  -- 87.5% all-empty
#     algorithm       3   `algorithm X { }`
# 206 of the 428 fn-bearing gold files (48.1%) have every single function body
# empty. Calibrating the body-substance term against *that* would reward the
# exact failure mode this scorer must reject.
#
# So calibration uses a NAMED SUBSET, not the raw 506:
#
#   GOLD-IMPL cohort  (n = 281)
#     = files under specs/ with
#         decl_total >= 1                  (has at least one declaration)
#         AND dialect != markdown          (is not a prose document)
#         AND NOT every function body empty (stub_fraction < 1.0, or 0 fns)
#     dialects in cohort: module_brace 164, module_semi 59, other 54,
#                         algorithm 3, specDSL 1
#
# GOLD-IMPL measured percentiles (the numbers below are copied from that run):
#   DECLS             min 1   p05 1     p10 1     p25 11    p50 30    p75 52
#   nodes_per_decl    min 1.75 p05 3.5  p10 4.544 p25 6     p50 7.357 p75 10.14
#   stub_fraction     p50 0    p75 0    p90 0     max 0.9
#   recovery_events   p90 0    max 4          (91.8% of cohort == 0)
#   lexer_discarded   max 0                   (100.0% == 0)
#   decls_swallowed   max 0                   (100.0% == 0)
#   tc_errors         p75 0    p90 4    max 23 (78.6% == 0)
#   tc_err_per_decl   p75 0    p90 0.0781     max 1.294
#   lint_per_decl     p05 0.08 p25 0.1667 p50 0.2581 p75 0.3714 p90 1.0
#   testcov_per_decl  p05 0.1111 p25 0.3043 p50 0.4167 p75 0.55  p90 1.0
#   per-fn AST nodes  (2521 functions) p05 3  p10 4  p25 7  p50 14  p75 27
#   all four parse/type signals clean: 199/281 = 70.8%
#
# For contrast, the SAME percentiles over the raw 506 (used nowhere in the
# score, recorded so the gap is visible):
#   DECLS p25 7 / p50 17 ; nodes_per_decl p25 3.111 / p50 5.13 ;
#   stub_fraction p50 0.0625 / p75 1.0 ; recovery==0 90.3% ; tc_errors==0 88.1%
#   all four clean 398/506 = 78.7%
#
# NOTE ON "GOLD-RELATIVE": full credit is pegged at a gold percentile, never at
# perfection. lint_per_decl full credit is p25 = 0.1667, not 0 -- almost every
# gold function trips lint's "has no test or invariant" rule, so demanding 0
# lint issues would score the gold corpus below the model. Likewise tc_errors:
# 21.4% of the cohort has a nonzero typecheck error count, so the typecheck
# term is a soft decay, not a gate.
CAL = {
    "measured_on": "2026-09-09",
    "cohort": "GOLD-IMPL n=281 of 506 under /Users/playom/t27/specs (scratch excluded)",
    # declaration floor: full credit at cohort p25
    "DECL_FULL": 11.0,          # GOLD-IMPL DECLS p25
    "DECL_EXP": 0.5,            # sub-linear ramp so small-but-real modules are not crushed
    # body substance
    "NPD_FLOOR": 1.75,          # GOLD-IMPL nodes_per_decl min  -> zero credit at/below
    "NPD_FULL": 6.0,            # GOLD-IMPL nodes_per_decl p25  -> full credit at/above
    "FN_NODES_EMPTY": 1,        # metrics `nodes` == 1 means the parser built no body
    "FN_NODES_IMPL": 4,         # GOLD-IMPL per-fn nodes p10 -> "substantively implemented"
    "BODY_FLOOR": 0.08,         # residual credit for correct-but-hollow syntax
    # parse cleanliness: gold tolerates a few recovery events, zero of the other two
    "RECOVERY_HALF": 2.0,       # recovery_events giving half credit (cohort max is 4)
    "SWALLOW_HALF": 0.5,        # decls_swallowed: cohort max is 0, so punish hard
    "LEXDISC_HALF": 4.0,        # lexer_discarded_chars: cohort max is 0
    # typecheck: soft, gold itself is 21.4% dirty
    "TCERR_HALF": 0.12,         # tc_errors per decl giving half credit (cohort p90 = 0.078)
    # lint: full credit at cohort p25, zero at cohort p90
    "LINT_GOOD": 0.1667,
    "LINT_BAD": 1.0,
    # tests+invariants per decl: full credit at cohort p50
    "TEST_FULL": 0.4167,
    # copy-paste padding guard. `uniformity` = fraction of a file's functions
    # sharing the single most common per-function AST node count. Measured over
    # the 165 GOLD-IMPL files with >= 5 functions:
    #   min 0.053  p10 0.143  p25 0.167  p50 0.231  p75 0.333  p90 0.467  max 0.900
    # Only 1 of 165 gold files exceeds 0.90. A generated file of 50 identical
    # `fn fN(x: u32) -> u32 { return x + N; }` bodies measures 1.000. The
    # penalty therefore activates only ABOVE the gold maximum, so it costs zero
    # gold files anything.
    "UNIFORMITY_MIN_FNS": 5,    # below this, uniformity is not meaningful
    "UNIFORMITY_OK": 0.90,      # == GOLD-IMPL max; no penalty at or below
    "UNIFORMITY_FLOOR": 0.25,   # multiplier applied at uniformity == 1.0
    # quality mixture weights (sum to 1.0)
    "W_PARSE": 0.40,
    "W_TYPE": 0.25,
    "W_LINT": 0.15,
    "W_TESTS": 0.20,
}

# ===========================================================================
# t27c drivers
# ===========================================================================

_PARSE_COUNTER_RE = re.compile(
    r"^(recovery-events|declarations-swallowed|lexer-discarded-chars)\s*:\s*(-?\d+)\s*$"
)
_SIZE_RE = re.compile(
    r"^\s+(Bytes|Lines|Nodes|Functions|Structs|Enums|Constants|Tests|Invariants|Benchmarks|Imports)\s*:\s*(\d+)\s*$"
)
# metrics table:  name  params  nodes  ret  CC  ratio
_METRICS_RE = re.compile(r"^(\S.*?)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+([\d.]+)\s*$")
# count table:  "  NodeKind    N"  and a trailing "  ... and N more"
_COUNT_ROW_RE = re.compile(r"^\s{2}(\w+)\s+(\d+)\s*$")
_COUNT_MORE_RE = re.compile(r"^\s*\.\.\.\s+and\s+(\d+)\s+more\s*$")
_COUNT_HEAD_RE = re.compile(r"^(.*?):\s*(\d+)\s+node types\s*$")


def _stream(args, keep_re, timeout=120):
    """Run t27c and keep only lines matching keep_re.

    `t27c parse` dumps the entire AST to stdout; the largest file in this repo
    is 38.5 MB of source. Streaming line-by-line keeps memory bounded instead of
    buffering the whole dump.
    """
    kept = []
    total_lines = 0
    try:
        p = subprocess.Popen(
            [T27C] + args,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            errors="replace",
            bufsize=1,
        )
    except OSError as e:
        return kept, 0, "spawn-failed: %s" % e
    try:
        for line in p.stdout:
            total_lines += 1
            if keep_re.search(line):
                kept.append(line.rstrip("\n"))
        p.wait(timeout=timeout)
    except subprocess.TimeoutExpired:
        p.kill()
        return kept, total_lines, "timeout"
    except Exception as e:  # pragma: no cover
        p.kill()
        return kept, total_lines, "error: %s" % e
    return kept, total_lines, None


def _capture(args, timeout=120):
    try:
        r = subprocess.run(
            [T27C] + args, capture_output=True, text=True, errors="replace", timeout=timeout
        )
        return r.stdout, r.stderr, r.returncode, None
    except subprocess.TimeoutExpired:
        return "", "", None, "timeout"
    except OSError as e:
        return "", "", None, "spawn-failed: %s" % e


def run_t27c(path):
    """Run every t27c probe on `path` and return a dict of raw tool output."""
    raw = {"tool_errors": []}

    # --- t27c parse <INPUT>  (no --json; counters are plain trailing lines) ---
    kept, nlines, err = _stream(["parse", path], _PARSE_COUNTER_RE)
    if err:
        raw["tool_errors"].append("parse: " + err)
    raw["parse_recovery_events"] = None
    raw["parse_declarations_swallowed"] = None
    raw["parse_lexer_discarded_chars"] = None
    for line in kept:
        m = _PARSE_COUNTER_RE.match(line)
        if m:
            raw["parse_" + m.group(1).replace("-", "_")] = int(m.group(2))
    raw["parse_ast_dump_lines"] = nlines

    # --- t27c size <INPUT>  (structured declaration census) ---
    kept, _, err = _stream(["size", path], _SIZE_RE)
    if err:
        raw["tool_errors"].append("size: " + err)
    for key in ("bytes", "lines", "nodes", "functions", "structs", "enums",
                "constants", "tests", "invariants", "benchmarks", "imports"):
        raw["size_" + key] = None
    for line in kept:
        m = _SIZE_RE.match(line)
        if m:
            raw["size_" + m.group(1).lower()] = int(m.group(2))

    # --- t27c count <INPUT>  (node-type histogram; cross-check only) ---
    out, _, _, err = _capture(["count", path])
    if err:
        raw["tool_errors"].append("count: " + err)
    hist = {}
    truncated = 0
    ntypes = None
    for line in out.splitlines():
        m = _COUNT_HEAD_RE.match(line)
        if m:
            ntypes = int(m.group(2))
            continue
        m = _COUNT_MORE_RE.match(line)
        if m:
            truncated = int(m.group(1))
            continue
        m = _COUNT_ROW_RE.match(line)
        if m:
            hist[m.group(1)] = int(m.group(2))
    raw["count_histogram"] = hist
    raw["count_node_types"] = ntypes
    # `count` prints only the top 15 node types then "... and N more", so this
    # cross-check is a LOWER BOUND whenever count_truncated_types > 0.
    raw["count_truncated_types"] = truncated
    raw["count_decl_nodes"] = sum(
        v for k, v in hist.items()
        if k.endswith("Decl") or k.endswith("Block")
    )

    # --- t27c typecheck --json <INPUT> ---
    out, _, _, err = _capture(["typecheck", "--json", path])
    if err:
        raw["tool_errors"].append("typecheck: " + err)
    raw["tc_errors"] = None
    raw["tc_warnings"] = None
    raw["tc_ok"] = None
    try:
        j = json.loads(out)
        raw["tc_errors"] = j.get("errors")
        raw["tc_warnings"] = j.get("warnings")
        raw["tc_ok"] = j.get("ok")
        raw["tc_messages"] = j.get("messages") or []
    except Exception:
        raw["tool_errors"].append("typecheck: non-JSON output")
        raw["tc_messages"] = []

    # --- t27c lint --json <INPUT> ---
    out, _, _, err = _capture(["lint", "--json", path])
    if err:
        raw["tool_errors"].append("lint: " + err)
    raw["lint_issues"] = None
    raw["lint_warnings"] = []
    try:
        j = json.loads(out)
        raw["lint_issues"] = j.get("issues")
        raw["lint_functions"] = j.get("functions")
        raw["lint_tests"] = j.get("tests")
        raw["lint_invariants"] = j.get("invariants")
        raw["lint_warnings"] = j.get("warnings") or []
    except Exception:
        raw["tool_errors"].append("lint: non-JSON output")

    # --- t27c metrics <INPUT>  (per-function AST node census) ---
    # This is the exact empty-body detector. A `// TODO: Implement` body, or any
    # body the parser could not consume, yields nodes == 1 for that function.
    out, _, _, err = _capture(["metrics", path])
    if err:
        raw["tool_errors"].append("metrics: " + err)
    fn_nodes = []
    fn_names = []
    for line in out.splitlines():
        s = line.strip()
        if not s or s.startswith("===") or s.startswith("---") or s.startswith("function "):
            continue
        m = _METRICS_RE.match(line)
        if m:
            fn_names.append(m.group(1).strip())
            fn_nodes.append(int(m.group(3)))
    raw["fn_nodes"] = fn_nodes
    raw["fn_names"] = fn_names
    return raw


# ===========================================================================
# scoring
# ===========================================================================

def _clamp01(x):
    if x != x:  # NaN
        return 0.0
    return 0.0 if x < 0.0 else (1.0 if x > 1.0 else x)


def _half_life_decay(x, half):
    """1.0 at x==0, 0.5 at x==half, asymptotic to 0. Monotone decreasing."""
    if x is None or x <= 0:
        return 1.0
    if half <= 0:
        return 0.0
    return 0.5 ** (float(x) / float(half))


def _ramp(x, lo, hi):
    """0 at x<=lo, 1 at x>=hi, linear between."""
    if hi <= lo:
        return 1.0 if x >= hi else 0.0
    return _clamp01((float(x) - lo) / (hi - lo))


def _inv_ramp(x, good, bad):
    """1 at x<=good, 0 at x>=bad, linear between. For 'lower is better'."""
    if bad <= good:
        return 1.0 if x <= good else 0.0
    return _clamp01((float(bad) - float(x)) / (bad - good))


def score_raw(raw):
    """Turn raw t27c output into components and one 0..1 score."""
    g = lambda k, d=0: (raw.get(k) if raw.get(k) is not None else d)

    functions = g("size_functions")
    structs = g("size_structs")
    enums = g("size_enums")
    constants = g("size_constants")
    tests = g("size_tests")
    invariants = g("size_invariants")
    benchmarks = g("size_benchmarks")
    imports = g("size_imports")
    nodes = g("size_nodes")
    nbytes = g("size_bytes")

    # DECLARATION FLOOR.
    # `imports` is deliberately EXCLUDED. Measured: a valid Python file scores
    # size_imports == 5, because t27c's recovery turns `import re` /
    # `from x import y` lines into UseDecl nodes. Counting imports would give
    # Python a nonzero declaration floor.
    decl_total = functions + structs + enums + constants + tests + invariants + benchmarks

    fn_nodes = raw.get("fn_nodes") or []
    n_fn_rows = len(fn_nodes)
    empty_fns = sum(1 for n in fn_nodes if n <= CAL["FN_NODES_EMPTY"])
    impl_fns = sum(1 for n in fn_nodes if n >= CAL["FN_NODES_IMPL"])
    stub_fraction = (empty_fns / n_fn_rows) if n_fn_rows else None
    impl_fraction = (impl_fns / n_fn_rows) if n_fn_rows else None
    # Per-function body credit, graded rather than thresholded: 0.0 at nodes==1
    # (parser built no body at all), 1.0 at nodes>=4 (GOLD-IMPL per-fn p10),
    # linear between. fnfill is the mean over declared functions.
    fnfill = (
        sum(_ramp(n, CAL["FN_NODES_EMPTY"], CAL["FN_NODES_IMPL"]) for n in fn_nodes) / n_fn_rows
        if n_fn_rows else None
    )

    nodes_per_decl = nodes / decl_total if decl_total > 0 else 0.0
    tc_err_per_decl = g("tc_errors") / decl_total if decl_total > 0 else float(g("tc_errors"))
    lint_per_decl = g("lint_issues") / decl_total if decl_total > 0 else float(g("lint_issues"))
    testcov_per_decl = (tests + invariants) / decl_total if decl_total > 0 else 0.0

    # --- component: declaration floor (multiplicative gate) ---
    if decl_total <= 0:
        s_decl = 0.0
    else:
        s_decl = min(1.0, (decl_total / CAL["DECL_FULL"]) ** CAL["DECL_EXP"])

    # --- component: body substance (multiplicative gate) ---
    shape = _ramp(nodes_per_decl, CAL["NPD_FLOOR"], CAL["NPD_FULL"])
    if n_fn_rows > 0:
        # fnfill MULTIPLIES rather than averages with shape. Measured reason:
        # 206 gold files have every function body empty, and the richest of them
        # (specs/storage/kv.t27: 8 empty fns, 11 test blocks, nodes_per_decl
        # 10.63) reached shape==1.0 purely from its test blocks. Averaging let
        # test-block richness rescue a file with zero implemented functions.
        # Multiplying makes an all-hollow file score 0 on body no matter how
        # much other AST it carries.
        s_body = fnfill * (0.70 + 0.30 * shape)
    else:
        # No functions declared. A struct/const-only module is legitimate gold,
        # so judge it purely on AST richness per declaration.
        s_body = shape
    s_body = _clamp01(s_body)

    # --- component: parse cleanliness ---
    s_parse = (
        _half_life_decay(g("parse_recovery_events"), CAL["RECOVERY_HALF"])
        * _half_life_decay(g("parse_declarations_swallowed"), CAL["SWALLOW_HALF"])
        * _half_life_decay(g("parse_lexer_discarded_chars"), CAL["LEXDISC_HALF"])
    )

    # --- component: typecheck ---
    s_type = _half_life_decay(tc_err_per_decl, CAL["TCERR_HALF"])

    # --- component: lint density ---
    s_lint = _inv_ramp(lint_per_decl, CAL["LINT_GOOD"], CAL["LINT_BAD"])

    # --- component: test / invariant coverage ---
    s_tests = _ramp(testcov_per_decl, 0.0, CAL["TEST_FULL"])

    quality = (
        CAL["W_PARSE"] * s_parse
        + CAL["W_TYPE"] * s_type
        + CAL["W_LINT"] * s_lint
        + CAL["W_TESTS"] * s_tests
    )

    # --- copy-paste padding guard (far tail only; see CAL comment) ---
    uniformity = None
    s_variety = 1.0
    if n_fn_rows >= CAL["UNIFORMITY_MIN_FNS"]:
        counts = {}
        for n in fn_nodes:
            counts[n] = counts.get(n, 0) + 1
        uniformity = max(counts.values()) / n_fn_rows
        if uniformity > CAL["UNIFORMITY_OK"]:
            frac = (uniformity - CAL["UNIFORMITY_OK"]) / (1.0 - CAL["UNIFORMITY_OK"])
            s_variety = 1.0 - (1.0 - CAL["UNIFORMITY_FLOOR"]) * frac

    body_term = CAL["BODY_FLOOR"] + (1.0 - CAL["BODY_FLOOR"]) * s_body
    score = _clamp01(s_decl * body_term * quality * s_variety)

    return {
        "score": round(score, 4),
        "components": {
            "s_decl": round(s_decl, 4),
            "s_body": round(s_body, 4),
            "s_parse": round(s_parse, 4),
            "s_type": round(s_type, 4),
            "s_lint": round(s_lint, 4),
            "s_tests": round(s_tests, 4),
            "s_variety": round(s_variety, 4),
            "quality_mix": round(quality, 4),
            "body_term": round(body_term, 4),
        },
        "derived": {
            "decl_total": decl_total,
            "decl_breakdown": {
                "functions": functions, "structs": structs, "enums": enums,
                "constants": constants, "tests": tests,
                "invariants": invariants, "benchmarks": benchmarks,
            },
            "imports_excluded_from_decl_total": imports,
            "nodes_per_decl": round(nodes_per_decl, 4),
            "stub_fraction": (round(stub_fraction, 4) if stub_fraction is not None else None),
            "impl_fraction": (round(impl_fraction, 4) if impl_fraction is not None else None),
            "fn_body_fill": (round(fnfill, 4) if fnfill is not None else None),
            "body_shape_nodes_per_decl": round(shape, 4),
            "fn_shape_uniformity": (round(uniformity, 4) if uniformity is not None else None),
            "empty_body_functions": empty_fns,
            "implemented_functions": impl_fns,
            "tc_err_per_decl": round(tc_err_per_decl, 4),
            "lint_per_decl": round(lint_per_decl, 4),
            "testcov_per_decl": round(testcov_per_decl, 4),
        },
        "signals": {
            "parse_recovery_events": raw.get("parse_recovery_events"),
            "parse_declarations_swallowed": raw.get("parse_declarations_swallowed"),
            "parse_lexer_discarded_chars": raw.get("parse_lexer_discarded_chars"),
            "declaration_count": decl_total,
            "count_decl_nodes_crosscheck": raw.get("count_decl_nodes"),
            "count_crosscheck_truncated_types": raw.get("count_truncated_types"),
            "typecheck_errors": raw.get("tc_errors"),
            "typecheck_warnings": raw.get("tc_warnings"),
            "typecheck_ok_UNTRUSTWORTHY": raw.get("tc_ok"),
            "lint_issues": raw.get("lint_issues"),
            "size_nodes": nodes,
            "size_bytes": nbytes,
            "per_function_ast_nodes": fn_nodes,
        },
        "tool_errors": raw.get("tool_errors") or [],
    }


def score_path(path):
    if not os.path.exists(T27C):
        return {"error": "t27c not found at %s" % T27C, "score": 0.0}
    r = score_raw(run_t27c(path))
    r["path"] = os.path.abspath(path)
    return r


def score_text(text, suffix=".t27"):
    fd, tmp = tempfile.mkstemp(suffix=suffix, prefix="score_t27_")
    try:
        with os.fdopen(fd, "w", encoding="utf-8", errors="replace") as fh:
            fh.write(text)
        r = score_path(tmp)
        r["path"] = "<stdin>"
        return r
    finally:
        try:
            os.unlink(tmp)
        except OSError:
            pass


# ===========================================================================
# built-in proof suite
# ===========================================================================

# 635 words of ordinary English prose. Contains no t27 syntax whatsoever.
_PROSE = """The history of programming languages is a long and winding road that begins well
before the first electronic computer was ever switched on. Ada Lovelace wrote what
many consider the first algorithm intended to be carried out by a machine, and she
did so for an engine that was never actually built. Her notes on the Analytical
Engine describe a sequence of operations for computing Bernoulli numbers, and in
describing them she anticipated ideas about looping and about the manipulation of
symbols that would not become common currency for another hundred years.

When the first machines did arrive they were programmed by rewiring, by plugging
cables into panels, and by setting banks of switches. The idea that a program could
be stored in the same memory as the data it operated on was a genuine conceptual
leap. Once that leap was made, the question of how to express a program became a
question about language rather than a question about wiring. Assembly languages
were the first step, giving mnemonic names to the numeric opcodes of a particular
machine, and they made programs readable in a way that raw octal never could.

The next step was to abstract away from the machine entirely. Fortran was designed
so that scientists could write formulas that looked like the formulas they already
wrote on paper, and the compiler would take responsibility for turning them into
efficient machine code. There was considerable skepticism at the time about whether
a compiler could ever produce code as good as a careful human programmer, and the
Fortran team spent enormous effort on optimization precisely because they knew the
project would be judged on the speed of the code it produced.

Cobol took a different approach, aiming at readability by people who were not
programmers at all. Its designers believed that business logic should be expressible
in something close to English, and the resulting language was verbose in a way that
later generations found tiresome, but it processed an extraordinary share of the
world's financial transactions for decades and in many places it still does.

Lisp came from a different tradition altogether. John McCarthy was interested in
symbolic computation and in artificial intelligence, and the language he designed
treated code and data as the same kind of thing. That single decision gave Lisp a
malleability that no other language of the era possessed, and it made macros, which
are programs that write programs, a natural part of the language rather than an
awkward bolt-on. Many ideas that eventually spread everywhere, including garbage
collection, first-class functions, and the read-eval-print loop, appeared in Lisp
first and only slowly made their way into the mainstream.

Algol was the great language of the committees, and although it was never widely
used in industry it became the language in which algorithms were published, and its
syntax influenced almost everything that came afterward. Block structure, lexical
scoping, and the careful distinction between a declaration and a statement all come
down to us from the Algol reports. The line of descent runs through Pascal and C and
onward into the languages that most people write today.

The story of the last few decades has been the gradual absorption of ideas that were
once considered exotic. Type inference, pattern matching, algebraic data types, and
closures all came from the functional tradition and now appear in languages that
would once have rejected them as academic. Memory safety without garbage collection,
long considered impossible to achieve without an unacceptable burden on the
programmer, turned out to be achievable through careful attention to ownership and
lifetimes. Every generation of language designers believes it has finally found the
right set of tradeoffs, and every generation is proved partly wrong by the one that
follows it, which is perhaps the most reliable pattern in the whole history.
"""

# A real Python 3 module: `python3 -c "import ast; ast.parse(_PY)"` succeeds.
_PY = '''#!/usr/bin/env python3
"""A small but real Python module: an LRU cache and a tokenizer."""

import re
import json
from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Iterator, Optional

TOKEN_RE = re.compile(r"[A-Za-z_][A-Za-z_0-9]*|\\d+|\\S")


@dataclass
class Token:
    kind: str
    text: str
    pos: int = 0
    meta: dict = field(default_factory=dict)

    def is_ident(self) -> bool:
        return self.kind == "ident"


class LRUCache:
    def __init__(self, capacity: int = 128):
        if capacity <= 0:
            raise ValueError("capacity must be positive")
        self.capacity = capacity
        self._data = OrderedDict()
        self.hits = 0
        self.misses = 0

    def get(self, key: str, default=None):
        if key in self._data:
            self._data.move_to_end(key)
            self.hits += 1
            return self._data[key]
        self.misses += 1
        return default

    def put(self, key: str, value) -> None:
        if key in self._data:
            self._data.move_to_end(key)
        self._data[key] = value
        while len(self._data) > self.capacity:
            self._data.popitem(last=False)

    def stats(self) -> dict:
        total = self.hits + self.misses
        return {"hits": self.hits, "misses": self.misses,
                "ratio": (self.hits / total) if total else 0.0}


def classify(text: str) -> str:
    if text.isdigit():
        return "number"
    if text[0].isalpha() or text[0] == "_":
        return "ident"
    return "punct"


def tokenize(source: str) -> Iterator[Token]:
    for m in TOKEN_RE.finditer(source):
        yield Token(kind=classify(m.group(0)), text=m.group(0), pos=m.start())


def main(argv: Optional[list] = None) -> int:
    src = "x = foo(12) + bar_baz * 3"
    toks = list(tokenize(src))
    cache = LRUCache(4)
    for t in toks:
        cache.put(t.text, t.pos)
    print(json.dumps({"tokens": len(toks), "cache": cache.stats()}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
'''

_STUBS = """// SPDX-License-Identifier: Apache-2.0
module RingBuffer;
    use base::types;
    pub const CAPACITY = 256;
    pub const SENTINEL = 0;
    pub const Ring = struct { head : u32, tail : u32, len : u32, cap : u32, };
    fn init(cap: u32) -> Ring {
        // TODO: Implement from .tri spec
    }
    fn push(r: Ring, v: u8) -> bool {
        // TODO: Implement from .tri spec
    }
    fn pop(r: Ring) -> u8 {
        // TODO: Implement from .tri spec
    }
    fn is_empty(r: Ring) -> bool {
        // TODO: Implement from .tri spec
    }
    fn is_full(r: Ring) -> bool {
        // TODO: Implement from .tri spec
    }
    fn len(r: Ring) -> u32 {
        // TODO: Implement from .tri spec
    }
    fn clear(r: Ring) -> Ring {
        // TODO: Implement from .tri spec
    }
    fn capacity(r: Ring) -> u32 {
        // TODO: Implement from .tri spec
    }
"""


# Five gold specs, chosen by a stated deterministic rule rather than by hand:
# score every *.t27 under specs/ (scratch excluded) with this scorer, keep the
# 222 files that have at least one non-empty function body, sort by score, and
# take the p10 / p30 / p50 / p70 / p90 members. Recorded 2026-09-09. This spans
# the implemented-gold range instead of showcasing only its top.
GOLD_SAMPLE = [
    "/Users/playom/t27/specs/nn/hslm.t27",                      # p10
    "/Users/playom/t27/specs/fpga/testbench/mac_tb.t27",        # p30
    "/Users/playom/t27/specs/numeric/gf24.t27",                 # p50
    "/Users/playom/t27/specs/fpga/testbench/ternary_isa_tb.t27",# p70
    "/Users/playom/t27/specs/fpga/apb_bridge.t27",              # p90
]


def _selftest(gold_samples=None):
    """Prove the scorer separates garbage from gold. Returns (ok, rows)."""
    rows = []
    for name, text in (("(a) english_prose", _PROSE), ("(b) valid_python", _PY),
                       ("(c) empty_file", ""), ("(d) todo_stubs", _STUBS)):
        r = score_text(text)
        rows.append((name, "NEGATIVE", r))
    if not gold_samples:
        gold_samples = list(GOLD_SAMPLE)
    for p in gold_samples:
        rows.append((os.path.relpath(p, GOLD_ROOT), "GOLD", score_path(p)))

    neg = [r["score"] for n, k, r in rows if k == "NEGATIVE"]
    pos = [r["score"] for n, k, r in rows if k == "GOLD"]
    # Pass criteria, stated up front so the result is auditable:
    #   every negative near zero      max(NEGATIVE) < 0.10
    #   every gold clearly high       min(GOLD)    >= 0.40
    #   and the two bands do not touch, with margin
    #                                 min(GOLD)    >  5 x max(NEGATIVE)
    chk = [
        ("max(NEGATIVE) < 0.10", max(neg) < 0.10),
        ("min(GOLD) >= 0.40", bool(pos) and min(pos) >= 0.40),
        ("min(GOLD) > 5 x max(NEGATIVE)", bool(pos) and min(pos) > 5.0 * max(neg)),
    ]
    ok = all(v for _, v in chk)

    w = max(len(n) for n, _, _ in rows) + 2
    print("%-*s %-9s %6s | %6s %6s %6s %6s %6s %6s | %5s %5s %5s %5s %5s %5s"
          % (w, "case", "kind", "SCORE", "decl", "body", "parse", "type", "lint", "tests",
             "ndec", "rec", "lex", "swal", "tcE", "stub"))
    print("-" * (w + 100))
    for name, kind, r in rows:
        c, d, s = r["components"], r["derived"], r["signals"]
        sf = d["stub_fraction"]
        print("%-*s %-9s %6.4f | %6.3f %6.3f %6.3f %6.3f %6.3f %6.3f | %5d %5s %5s %5s %5s %5s"
              % (w, name, kind, r["score"],
                 c["s_decl"], c["s_body"], c["s_parse"], c["s_type"], c["s_lint"], c["s_tests"],
                 d["decl_total"], s["parse_recovery_events"], s["parse_lexer_discarded_chars"],
                 s["parse_declarations_swallowed"], s["typecheck_errors"],
                 ("%.2f" % sf) if sf is not None else "-"))
    print("-" * (w + 100))
    print("max(NEGATIVE) = %.4f    min(GOLD) = %.4f    separation ratio = %.1fx"
          % (max(neg), min(pos) if pos else float("nan"),
             (min(pos) / max(neg)) if (pos and max(neg) > 0) else float("inf")))
    for label, v in chk:
        print("  [%s] %s" % ("PASS" if v else "FAIL", label))
    print("SELFTEST: %s" % ("PASS" if ok else "FAIL"))
    return ok, rows


# ===========================================================================
# CLI
# ===========================================================================

def main(argv=None):
    ap = argparse.ArgumentParser(
        description="Mechanically score generated .t27 against the measured gold corpus.")
    ap.add_argument("files", nargs="*", help="paths to .t27 files, or '-' for stdin")
    ap.add_argument("--tsv", action="store_true", help="one TSV row per file instead of JSON")
    ap.add_argument("--verbose", action="store_true", help="include lint/typecheck message text")
    ap.add_argument("--selftest", action="store_true", help="run the built-in proof suite")
    ap.add_argument("--gold", nargs="*", default=None,
                    help="gold files to use in --selftest (default: all of specs/)")
    ap.add_argument("--calibration", action="store_true", help="print calibration constants and exit")
    a = ap.parse_args(argv)

    if a.calibration:
        print(json.dumps(CAL, indent=2))
        return 0
    if a.selftest:
        ok, _ = _selftest(a.gold)
        return 0 if ok else 1
    if not a.files:
        ap.error("give at least one FILE, '-' for stdin, or --selftest")

    results = []
    for f in a.files:
        r = score_text(sys.stdin.read()) if f == "-" else score_path(f)
        if not a.verbose:
            r["signals"].pop("per_function_ast_nodes", None)
        results.append(r)

    if a.tsv:
        print("\t".join(["path", "score", "s_decl", "s_body", "s_parse", "s_type",
                         "s_lint", "s_tests", "decls", "recovery", "lexdisc",
                         "swallowed", "tc_errors", "lint_issues", "stub_fraction"]))
        for r in results:
            c, d, s = r["components"], r["derived"], r["signals"]
            print("\t".join(str(x) for x in [
                r.get("path"), r["score"], c["s_decl"], c["s_body"], c["s_parse"],
                c["s_type"], c["s_lint"], c["s_tests"], d["decl_total"],
                s["parse_recovery_events"], s["parse_lexer_discarded_chars"],
                s["parse_declarations_swallowed"], s["typecheck_errors"],
                s["lint_issues"], d["stub_fraction"]]))
    else:
        print(json.dumps(results[0] if len(results) == 1 else results, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
