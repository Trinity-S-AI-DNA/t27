#!/usr/bin/env bash
# t27 correctness oracle.
#
#   spec -> t27c gen -> mirrored .zig tree -> zig test
#
# The mirror matters (imports resolve only on the real tree shape) and the shim
# at the root makes that root the Zig module root. Before this tool existed
# nothing executed; its first full run found 575 of 906 specs non-compiling.
# Written in bash on purpose: it establishes whether .t27 output runs at all
# and cannot depend on the thing it measures.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
T27C="${T27C_BIN:-$ROOT/target/release/t27c}"
ZIG="${ZIG_BIN:-$(command -v zig)}"
OUT="${ORACLE_OUT:-${TMPDIR:-/tmp}/t27-oracle}"
JOBS="${ORACLE_JOBS:-8}"
LEDGER="$ROOT/tools/oracle/baseline.tsv"
# Seconds one spec may run before it is called hung (2026-09-17: with no limit
# fpga/testbench/gf16_accel_tb held xargs for 24 minutes).
ORACLE_TIMEOUT="${ORACLE_TIMEOUT:-120}"

[ -x "$T27C" ] || { echo "oracle: no t27c at $T27C (set T27C_BIN)" >&2; exit 2; }
[ -n "$ZIG" ]  || { echo "oracle: no zig on PATH (set ZIG_BIN)" >&2; exit 2; }

rm -rf "$OUT"; mkdir -p "$OUT"
cd "$ROOT"
gen=0
# A failed gen must leave no fragment behind: a truncated valid-Zig fragment
# once scored PASS (2026-09-17: 89 of 278 "passing" specs had a failed gen).
# ORACLE_SPEC=<relative specs path>: measure ONE spec instead of the corpus.
# Its sealed use-closure generates with it, so cross-file imports resolve;
# without a seal the outside imports make the honest NOCOMPILE verdict. The
# seal is <parent-dir>_<Module>.json -- the compiler's own naming, not a path
# flattening.
spec_set() {
  if [ -z "${ORACLE_SPEC:-}" ]; then
    find specs -name '*.t27' | sort
  else
    local parent modn seal
    parent=$(printf '%s' "$ORACLE_SPEC" | sed 's|/[^/]*\.t27$||; s|^.*/||')
    [ "$parent" = "$ORACLE_SPEC" ] && parent=""
    modn=$(grep -m1 '^[[:space:]]*module[[:space:]]' "$ORACLE_SPEC" \
      | sed 's/^[[:space:]]*module[[:space:]]*//; s/;.*$//')
    if [ -n "$parent" ] && [ -n "$modn" ]; then
      seal=".trinity/seals/${parent}_${modn}.json"
    else
      seal=".trinity/seals/${modn}.json"
    fi
    # No module line names no seal: fall back to the one seal whose spec_path
    # names it; still nothing, and the outside imports fail as NOCOMPILE.
    if [ ! -f "$seal" ]; then
      seal=$(grep -rl "\"${ORACLE_SPEC}\"" .trinity/seals --include='*.json' 2>/dev/null | head -1)
    fi
    if [ -f "$seal" ]; then
      perl -ne 'while (/"(specs\/[a-zA-Z0-9_\/.-]+\.t27)"/g) { print "$1\n"; }' "$seal" | sort -u | grep -v "^${ORACLE_SPEC}$"
    else
      echo "oracle: no seal for $ORACLE_SPEC (at $seal); outside imports are not generated" >&2
    fi
    printf '%s\n' "$ORACLE_SPEC"
  fi
}
nogen="$OUT/.nogen"; : > "$nogen"
while read -r f; do
  rel="${f#specs/}"; dst="$OUT/${rel%.t27}.zig"
  mkdir -p "$(dirname "$dst")"
  if "$T27C" gen "$f" > "$dst" 2>/dev/null && [ -s "$dst" ]; then
    gen=$((gen+1))
  else
    rm -f "$dst"
    printf '%s\n' "${rel%.t27}.zig" >> "$nogen"
  fi
done < <(spec_set)
echo "oracle: generated $gen specs"
# Zero is not a result (measured 2026-09-17: a misplaced copy reported
# "0 / 0 pass" and exited 0); the ratchet would read it as no regression.
if [ "$gen" -eq 0 ]; then
  echo "oracle: measured nothing -- no spec generated under $ROOT/specs" >&2
  exit 2
fi

run_one() {
  local f="$1" OUT="$2" ZIG="$3" LIMIT="$4"
  local shim="$OUT/_s_$(echo "$f" | tr '/' '_')"
  printf 'test { _ = @import("%s"); }\n' "$f" > "$shim.zig"
  local log rc
  # Bounded on the whole PROCESS GROUP, not zig alone: the hang is usually the
  # grandchild test binary, which outlives zig, keeps the pipe open and holds
  # the command substitution forever (2026-09-17: a 20s limit held 7 minutes).
  # Fork, own group, kill GROUP on the alarm; perl, because it is on every
  # CI runner and on macOS, where `timeout` is not -- one verdict semantics
  # everywhere.
  local logf; logf="$(mktemp)"
  perl -e '
    my $limit = shift;
    my $pid = fork;
    if (!defined $pid) { exit 125 }
    if ($pid == 0) { setpgrp(0, 0); exec @ARGV or exit 127 }
    local $SIG{ALRM} = sub { kill "TERM", -$pid; sleep 2; kill "KILL", -$pid; exit 124 };
    alarm $limit;
    waitpid($pid, 0);
    my $st = $?;
    exit(($st & 127) ? 128 + ($st & 127) : ($st >> 8));
  ' "$LIMIT" "$ZIG" test "$shim.zig" > "$logf" 2>&1
  rc=$?
  log="$(cat "$logf")"
  rm -f "$logf"
  rm -f "$shim.zig"
  if [ $rc -eq 0 ]; then
    printf '%s\tPASS\t\n' "$f"
  else
    # Name the file the error is IN: an inherited import failure is a
    # different defect from a local one.
    local where first
    where=$(printf '%s' "$log" | grep -m1 -oE '^[^ :]+\.zig:[0-9]+' | cut -d: -f1)
    first=$(printf '%s' "$log" | grep -m1 'error:' | sed 's/.*error: //' | cut -c1-90)
    # Hung, not broken (124/142; 2026-09-17: gf16_accel_tb): a test that never
    # returns COMPILED -- calling it NOCOMPILE misfiles a runtime defect.
    if [ "$rc" = 124 ] || [ "$rc" = 142 ] \
       || printf '%s' "$log" | grep -q 'signal TERM\|signal KILL\|signal ALRM'; then
      printf '%s\tTIMEOUT\thung past %ss\n' "$f" "$LIMIT"
    elif printf '%s' "$log" | grep -q 'signal ABRT\|test failure'; then
      printf '%s\tTESTFAIL\t%s\n' "$f" "$first"
    else
      printf '%s\tNOCOMPILE\t%s [in %s]\n' "$f" "$first" "${where:-?}"
    fi
  fi
}
export -f run_one

cd "$OUT"
results="$OUT/results.tsv"
find . -name '*.zig' ! -name '_s_*' | sed 's|^\./||' | sort \
  | xargs -P "$JOBS" -I{} bash -c 'run_one "$@"' _ {} "$OUT" "$ZIG" "$ORACLE_TIMEOUT" > "$results"

# A spec that did not generate has no Zig to test, so it never reached the test
# loop. Record it anyway: dropping it shrinks the denominator and makes the pass
# rate look better than the corpus is.
while read -r z; do
  [ -n "$z" ] && printf '%s\tNOGEN\tt27c gen failed\n' "$z" >> "$results"
done < "$OUT/.nogen"

pass=$(awk -F'\t' '$2=="PASS"' "$results" | wc -l | tr -d ' ')
total=$(wc -l < "$results" | tr -d ' ')
echo "oracle: $pass / $total pass"
awk -F'\t' '$2!="PASS"{print $2}' "$results" | sort | uniq -c | sort -rn | sed 's/^/  /'

# Ratchet: the pass count may rise, never fall. A filtered run (ORACLE_SPEC)
# is a lane verdict, not a corpus measurement -- the ratchet does not read it.
if [ -f "$LEDGER" ] && [ -z "${ORACLE_SPEC:-}" ]; then
  base=$(awk -F'\t' '$1=="pass"{print $2}' "$LEDGER")
  if [ -n "$base" ] && [ "$pass" -lt "$base" ]; then
    echo "oracle: REGRESSION - $pass passing, baseline is $base" >&2
    comm -13 <(awk -F'\t' '$2=="PASS"{print $1}' "$results" | sort) \
             <(awk -F'\t' '$1!="pass"{print $1}' "$LEDGER" | sort) \
      | sed 's/^/  no longer passing: /' >&2
    exit 1
  fi
  echo "oracle: baseline $base, now $pass"
fi

if [ "${ORACLE_WRITE_BASELINE:-0}" = "1" ]; then
  { printf 'pass\t%s\n' "$pass"
    awk -F'\t' '$2=="PASS"{print $1}' "$results" | sort; } > "$LEDGER"
  echo "oracle: baseline written ($pass)"
fi
