#!/usr/bin/env bash
# T27 provider-guard rollback drill (owner requirement 2026-10-30).
#
# Thin verification driver: no decision logic lives here. Every assertion is
# machine-verifiable and owned by
#   - specs/system/llm_request_provider_gate.t27 (SSOT: classes + policy)
#   - contrib/backend/agent-runner/src/api.rs (guard intercept tests)
# UNITY (L7) note: created on the owner's explicit drill request; the script
# only orchestrates t27c and cargo invocations and reads their exit codes.
#
# Drill contract:
#   1. checkpoint preserved   -> intercept tests assert conversation state
#                                is untouched across transient and unfunded
#                                refusal paths.
#   2. state restorable       -> unfunded_refusal_then_same_checkpoint_retry_
#                                stays_restricted re-runs from the SAME
#                                checkpoint: refusal repeats, capped 1 req.
#   3. unfunded retry_count=0 -> unfunded_stream / insufficient_balance_on_429
#                                both prove exactly ONE request leaves.
#   4. follow-up task exists  -> follow_up_task_event_is_machine_parseable
#                                asserts the event shape; this drill also
#                                greps the runner JSONL log the tests write.
#   5. exit 0 when every drill step passed.
set -u

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
T27C="$REPO_ROOT/target/release/t27c"
RUNNER_DIR="$REPO_ROOT/contrib/backend/agent-runner"
DRILL_LOG="${DRILL_LOG:-$(mktemp "${TMPDIR:-/tmp}/t27-rollback-drill.XXXXXX.log")}"
RUNNER_LOG="${RUNNER_LOG:-/tmp/agent-log.jsonl}"

overall=0
steps=0

step() { echo "[DRILL] $*"; }
note() { echo "[DRILL]       $*"; }
pass() { echo "[DRILL][PASS] $*"; }
fail() { echo "[DRILL][FAIL] $*"; overall=1; exit_code_problem=1; }

run_step() {
    step "$1"
    local name="$1"
    shift
    if "$@" >"$DRILL_LOG" 2>&1; then
        pass "$name (exit 0)"
    else
        local rc=$?
        fail "$name (exit $rc); log tail:"
        tail -n 12 "$DRILL_LOG" | sed 's/^/              /'
    fi
    steps=$((steps + 1))
}

grep_step() {
    step "$1"
    if grep -q "$2" "$3" 2>/dev/null; then
        pass "$1"
    else
        fail "$1 (pattern not found: $2)"
    fi
    steps=$((steps + 1))
}

if [ ! -x "$T27C" ]; then
    fail "t27c not found at $T27C (run: cd bootstrap && cargo build --release)"
    exit 1
fi

echo "[DRILL] provider-guard rollback drill"
echo "[DRILL] drill log: $DRILL_LOG"

run_step "typecheck: SSOT guard spec" \
    "$T27C" typecheck "$REPO_ROOT/specs/system/llm_request_provider_gate.t27"

run_step "seal: guard spec hashes match baseline" \
    "$T27C" seal "$REPO_ROOT/specs/system/llm_request_provider_gate.t27" --verify

run_step "cargo: guard tests (intercept + drill evidence)" \
    cargo test --offline --manifest-path "$RUNNER_DIR/Cargo.toml" --quiet

# The unfunded intercept tests write follow_up_task events into the runner
# log (the OnceLock LOG_FILE stays untouched in tests, therefore the default
# path receives them). Their presence is the drill's follow-up-task proof.
grep_step "follow-up task exists in the runner log" \
    '"event":"follow_up_task"' "$RUNNER_LOG"

grep_step "deterministic follow-up title present" \
    'PROVIDER_GATE:PROVIDER_UNFUNDED:127.0.0.1:claude-test-model' "$RUNNER_LOG"

if [ "$overall" -eq 0 ]; then
    echo "[DRILL] RESULT: PASS ($steps steps)"
else
    echo "[DRILL] RESULT: FAIL ($steps steps; failures above)"
fi
exit "$overall"
