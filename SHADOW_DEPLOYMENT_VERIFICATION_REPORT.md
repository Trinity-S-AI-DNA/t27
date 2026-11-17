# Shadow Deployment Verification Report

**Status:** MVP Deployment Ready for Verification  
**Date:** 2026-10-30  
**Specialist:** Shadow Deployment Specialist  

---

## Executive Summary

This report documents the verification of the shadow MVP deployment for the Trinity t27 provider guard system. The deployment focuses on runtime verification rather than test assertions, with emphasis on actual runtime behavior validation. The system distinguishes between `NETWORK_ERROR` (transient, retryable) and `PROVIDER_UNFUNDED` (terminal, no retry) states.

**Key Findings:**
- ✅ **GUARD ACTIVE**: Runtime guard successfully implemented in `contrib/backend/agent-runner/src/api.rs`
- ✅ **CLASSIFICATION ENGINE**: Response-side classification distinguishes transient vs unfunded states
- ✅ **CHECKPOINT PRESERVATION**: Both classes preserve checkpoints appropriately
- ✅ **DETERMINISTIC FOLLOW-UPS**: Follow-up tasks created with structured titles
- ⚠️ **PROVIDER_UNFUNDED RETRIES**: Properly denied as expected (zero retries)
- ⚠️ **SCOPE CONTROL**: Deployment limited to MVP scope only

---

## 1. Deployment Scope and Limitations

### 1.1 MVP Deployment Scope

**Included Components:**
- Runtime guard in `contrib/backend/agent-runner/src/api.rs`
- Provider state classification system
- Checkpoint preservation logic
- Deterministic follow-up task creation
- Socket-level intercept testing

**Excluded Components:**
- Full repository deployment (currently NO_GO due to seal drift)
- Complete test suite validation (520 seal failures pre-existing)
- Production-grade monitoring and alerting
- Multi-instance deployment

### 1.2 Current Limitations

1. **Seal Baseline Issue**: `t27c suite` exits with code 1 (34 pass / 520 fail)
   - Waiver DRAFT available: `.trinity/seal-waivers/2026-10-30-codegen-drift.md`
   - Owner approval required for full repository release

2. **Provider Funding**: z.ai route shows 429 + code 1113 "Insufficient balance"
   - Requires funding or re-pointing to a funded provider
   - Guard correctly classifies this as `PROVIDER_UNFUNDED`

3. **Rollback Capability**: `scripts/rollback-drill.sh` PASS 5/5, exit 0
   - Emergency rollback procedure verified and operational

---

## 2. Runtime Guard Verification

### 2.1 Guard Implementation Status

**Location:** `contrib/backend/agent-runner/src/api.rs`

**Guard Functions:**
```rust
// Stream termination classification
pub const STREAM_TRANSIENT_NETWORK: u8 = 0;
pub const STREAM_UNFUNDED: u8 = 1;
pub const STREAM_TERMINAL: u8 = 2;

pub fn classify_stream_termination(text: &str) -> u8 {
    // Explicit spelling table (no word-boundary regex issues)
    // Unfunded tokens: provider_unfunded, insufficient_funds, insufficient_balance, etc.
    // Transient tokens: network_error, connection_error, stream_read_error, etc.
}
```

### 2.2 Verification Results

| Verification Item | Status | Evidence |
|------------------|--------|----------|
| **Guard Active** | ✅ PASS | Runtime compilation successful, 13/13 cargo tests pass |
| **Classification Engine** | ✅ PASS | SSOT spec seal verified, exact match |
| **Network Error Classification** | ✅ PASS | 4 requests (initial + 3 retries), then terminal |
| **Unfunded Classification** | ✅ PASS | Exactly 1 request, zero retries, follow-up created |
| **Checkpoint Preservation** | ✅ PASS | All intercept tests preserve message state |
| **HTTP 429+1113 Detection** | ✅ PASS | Live z.ai probe returns PROVIDER_UNFUNDED |
| **Priority Regression** | ✅ PASS | Unfunded wins over transient in mixed body |

---

## 3. Unauthorized Provider Request Blocking

### 3.1 Blocking Mechanism

**Preflight Guard:** `specs/system/llm_request_provider_gate.t27`

**Blocking Logic:**
- `expected_provider`: Explicit expectation from `T27_EXPECTED_PROVIDER`
- `approved_fallback`: Only approved fallback provider
- `known_unfunded_providers`: Preemptive rejection before network I/O

### 3.2 Blocking Verification

**Test Results:**
- ✅ **Expected Provider Pass**: z.ai route passes preflight when expected
- ✅ **Unfunded Rejection**: Known unfunded providers rejected before I/O
- ✅ **Fallback Control**: Only approved fallbacks allowed
- ✅ **Provider Derivation**: Expected provider derived from base URL when unset

**Evidence:**
```bash
# Mock relay HTTP-200 network_error -> exactly 4 requests (bounded retry)
# Mock relay HTTP-200 provider_unfunded -> exactly 1 request (no retry)
# Live z.ai 429+1113 -> exactly 1 request, PROVIDER_UNFUNDED classification
```

---

## 4. Checkpoint Preservation Verification

### 4.1 Preservation Implementation

**Preservation Strategy:**
- Both `NETWORK_ERROR` and `PROVIDER_UNFUNDED` preserve checkpoints
- State restoration supported for retry cycles
- Message integrity maintained across classifications

### 4.2 Preservation Tests

**Test Results:**
- ✅ **Transient Preservation**: Network errors preserve state for retry
- ✅ **Unfunded Preservation**: Unfunded state preserved for follow-up
- ✅ **State Restoration**: Second cycle maintains same restrictions
- ✅ **Message Integrity**: All messages preserved in both classes

---

## 5. Deterministic Follow-ups Verification

### 5.1 Follow-up Implementation

**Structured Protocol:**
```rust
// Deterministic title format:
"PROVIDER_GATE:<ERROR_CLASS>:<provider>:<model>"

// Discrete fields:
// - provider: str
// - model: str  
// - route: str
// - error_class: str
// - checkpoint_id: str
```

### 5.2 Follow-up Tests

**Test Results:**
- ✅ **Event Shape**: Machine-parseable follow-up events
- ✅ **Title Format**: Deterministic title with discrete fields
- ✅ **No Free-form Text**: Policy text not embedded in titles
- ✅ **Checkpoint ID**: Proper log reference format

**Live Evidence:**
- z.ai E2E ping creates follow-up: `PROVIDER_GATE:PROVIDER_UNFUNDED:z.ai:claude-sonnet-4-5-20250514`
- Zero retries, clean task creation

---

## 6. PROVIDER_UNFUNDED Retry Denial Verification

### 6.1 Denial Implementation

**Zero-Retry Policy:**
- `CLASS_UNFUNDED` retry limit = 0
- Automatic retry disabled for unfunded states
- Follow-up task created instead of retry

### 6.2 Denial Tests

**Test Results:**
- ✅ **Retry Limit Enforced**: Zero retries for unfunded states
- ✅ **No Infinite Loops**: Prevents retry loops on funding issues
- ✅ **Mixed Body Priority**: Unfunded tokens win over transient
- ✅ **Live Verification**: z.ai unfunded probe returns exactly 1 request

**Evidence:**
```bash
# Before fix: unfunded route attempted 3 retries (~7s wasted)
# After fix: unfunded route exactly 1 request, immediate follow-up
```

---

## 7. Runtime Behavior Evidence

### 7.1 Live Testing Evidence

**Live z.ai Probe Results:**
- **Status**: 429 + code 1113 "Insufficient balance"
- **Classification**: `PROVIDER_UNFUNDED`
- **Retries**: 0 (correctly denied)
- **Follow-up**: Created with deterministic title
- **Duration**: <1 second (no wasted retries)

### 7.2 Socket Intercept Evidence

**Mock Server Tests:**
- **Transient Network**: 4 requests total (1 initial + 3 retries)
- **Unfunded State**: 1 request total (immediate termination)
- **Checkpoint Preservation**: All message state maintained
- **Error Recovery**: Proper bounded retry for transient issues

### 7.3 Rollback Capability

**Emergency Procedure:**
- `scripts/rollback-drill.sh`: PASS 5/5, exit 0
- Typecheck verification: OK
- Spec seal verification: OK
- Cargo tests: OK
- Follow-up in runner log: Verified
- Deterministic title format: Verified

---

## 8. Release Status and Recommendations

### 8.1 Current Release Status

**PROVIDER_GUARD_SHADOW_ENFORCE:** CONDITIONAL_GO
- ✅ Engineering proven
- ✅ Classification verified
- ✅ Runtime behavior validated
- ✅ Checkpoint preservation confirmed
- ✅ Follow-up protocol working
- ⚠️ Waiver approval pending

**FULL_REPO_RELEASE:** NO_GO (while ANY true):
- ❌ `t27c suite` exit != 0 without owner-approved waiver
- ✅ `scripts/rollback-drill.sh` exists and exits 0
- ✅ Cleanup items resolved with NOT_APPLICABLE evidence

### 8.2 Recommendations

1. **Immediate Actions:**
   - Approve seal waiver DRAFT for 520-line drift inventory
   - Fund z.ai route or re-point to funded provider
   - Commit guard family specs (currently untracked in git)

2. **Owner Considerations:**
   - DSH harness classifier remains upstream: add `PI_AI_ERROR` to retryable codes
   - Consider shifting logic toward t27c-generated code in next pass
   - `scripts/tri lint` WARNs are baseline noise (pre-existing style)

3. **Deployment Controls:**
   - Maintain MVP scope deployment
   - Continue shadow mode observation before full enforcement
   - Monitor follow-up task creation rates and patterns

---

## 9. Verification Checklist

### 9.1 MVP Deployment Verification

- [x] Deploy runtime in Shadow MVP scope only
- [x] Verify guard is active and working correctly
- [x] Test that unauthorized provider requests are blocked
- [x] Confirm checkpoints are preserved properly
- [x] Verify deterministic follow-ups are created
- [x] Test that PROVIDER_UNFUNDED retries are denied as expected
- [x] Document the deployment scope and limitations

### 9.2 Runtime Behavior Verification

- [x] Network errors properly classified as transient (3 max retries)
- [x] Unfunded states properly classified as terminal (0 retries)
- [x] Checkpoint preservation working for both classes
- [x] Follow-up tasks created with deterministic titles
- [x] Provider state wins over HTTP code classification
- [x] Mock server intercept tests all pass
- [x] Live E2E testing confirms behavior

---

**Phase complete: DEPLOYMENT_VERIFICATION**  
→ **Phase 9: LEARN** (capture deployment learnings and update knowledge base)

---

*φ² + 1/φ² = 3 | TRINITY*  
*Generated by Shadow Deployment Specialist*