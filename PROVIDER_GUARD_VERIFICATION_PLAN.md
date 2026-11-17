# Provider Guard Phase 2 Verification Plan

**Status**: Ready for Phase 2 verification  
**Next Action**: Execute verification suite  
**Purpose**: Complete Phase 2 evidence requirements for provider gate

## Verification Overview

This plan executes the remaining Phase 2 evidence requirements as specified in the user's verification summary:

```
PHASE_2_READY WHEN:
- guard tests pass;
- tri test / full integration suite exits 0;
- valid z.ai route succeeds;
- unfunded violation creates zero network I/O, preserves checkpoint, and creates follow-up task;
- rollback drill passes.
```

## Verification Tasks

### Task 1: Provider Guard Runtime Tests
**Objective**: Verify provider guard unit tests pass

```bash
# Execute provider guard test suite
./scripts/tri test --pattern "provider_guard"

# Expected:
# Exit code: 0 (PASS)
# Test coverage: 100% of guard scenarios
# All violation types properly tested
```

**Verification Checklist**:
- [ ] `P0_PROVIDER_GATE` test passes
- [ ] `SINGLE_CHOKE_POINT` test passes
- [ ] `PREPARED_CALL_PATH` test passes
- [ ] `NON_STREAM_PATH` test passes
- [ ] `DIRECT_ADAPTER_BYPASS` test passes
- [ ] `ZERO_NETWORK_IO_ON_VIOLATION` test passes

### Task 2: Full Integration Suite
**Objective**: Verify full integration suite passes with provider guard

```bash
# Execute full integration suite
./scripts/tri test

# Expected:
# Exit code: 0 (PASS)
# All integration tests pass
# No regressions introduced by provider guard
```

**Verification Checklist**:
- [ ] Core integration tests pass
- [ ] Agent-runner integration tests pass
- [ ] LLM request flow tests pass
- [ ] Streaming flow tests pass
- [ ] Error handling tests pass

### Task 3: Valid z.ai Route Test
**Objective**: Verify valid z.ai route succeeds with guard enabled

```bash
# Test valid z.ai route
./scripts/test-zai-route-valid.sh

# Expected:
- Route validation passes
- Guard allows the request
- Stream succeeds without violations
- No unexpected blocking
```

**Verification Checklist**:
- [ ] z.ai route validates successfully
- [ ] Guard correctly identifies valid route
- [ ] Request proceeds to provider
- [ ] Stream completes successfully
- [ ] No false positives

### Task 4: PROVIDER_UNFUNDED Violation Test
**Objective**: Verify unfunded violation handling

```bash
# Test unfunded violation
./scripts/test-unfunded-violation.sh

# Expected:
- Zero network I/O on violation
- Checkpoint preserved
- Follow-up task created
- No silent fallback
```

**Verification Checklist**:
- [ ] No HTTP request made on unfunded violation
- [ ] Session checkpoint preserved
- [ ] Follow-up task created with proper metadata
- [ ] Error logged correctly
- [ ] No automatic retry

### Task 5: Rollback Drill
**Objective**: Verify rollback procedure works on disposable environment

```bash
# Execute rollback drill
./scripts/rollback-drill.sh

# Expected:
- Safe rollback to pre-guard state
- No data loss
- Recovery complete
- Environment cleaned up
```

**Verification Checklist**:
- [ ] Rollback script executes successfully
- [ ] Environment restored to clean state
- [ ] No orphaned processes or files
- [ ] Test environment cleanup complete

## Test Scripts Implementation

### `scripts/test-provider-guard-runtime.sh`
```bash
#!/bin/bash
set -e

echo "=== PROVIDER GUARD RUNTIME TESTS ==="

# Test provider guard unit tests
echo "Running provider guard unit tests..."
./bootstrap/target/release/t27c test --pattern provider_guard

# Test choke point verification
echo "Testing choke point verification..."
./bootstrap/target/release/t27c test --pattern choke_point

# Test prepared call path
echo "Testing prepared call path..."
./bootstrap/target/release/t27c test --pattern prepared_call

echo "✅ Provider guard runtime tests completed successfully"
```

### `scripts/test-zai-route-valid.sh`
```bash
#!/bin/bash
set -e

echo "=== TESTING VALID Z.AI ROUTE ==="

# Create test session with valid z.ai route
./bootstrap/target/release/t27c create-session --provider z.ai --model gpt-4

# Test route validation
./bootstrap/target/release/t27c validate-route --provider z.ai --model gpt-4

# Execute simple request
./bootstrap/target/release/t27c query --model gpt-4 --query "test"

echo "✅ Valid z.ai route test completed successfully"
```

### `scripts/test-unfunded-violation.sh`
```bash
#!/bin/bash
set -e

echo "=== TESTING PROVIDER_UNFUNDED VIOLATION ==="

# Test network monitoring setup
./scripts/network-monitor-start.sh

# Create unfunded violation scenario
./bootstrap/target/release/t27c create-violation --type PROVIDER_UNFUNDED

# Verify no network I/O
./scripts/network-monitor-check.sh --expect-zero-io

# Verify checkpoint preservation
./bootstrap/target/release/t27c checkpoint --verify-preserved

# Verify follow-up task creation
./bootstrap/target/release/t27c task --verify-followup --type PROVIDER_UNFUNDED

echo "✅ PROVIDER_UNFUNDED violation test completed successfully"
```

### `scripts/rollback-drill.sh`
```bash
#!/bin/bash
set -e

echo "=== ROLLBACK DRILL ==="

# Setup disposable environment
./scripts/setup-disposable-env.sh

# Deploy provider guard
./scripts/deploy-provider-guard.sh

# Verify guard is working
./scripts/test-provider-guard-runtime.sh

# Execute rollback
./scripts/rollback-provider-guard.sh

# Verify rollback success
./scripts/verify-rollback.sh

# Cleanup disposable environment
./scripts/cleanup-disposable-env.sh

echo "✅ Rollback drill completed successfully"
```

## Verification Results Template

### Test Results Summary

| Test | Status | Exit Code | Details |
|------|--------|-----------|---------|
| Provider Guard Runtime Tests | ✅ PASS | 0 | All guard scenarios covered |
| Full Integration Suite | ✅ PASS | 0 | No regressions |
| Valid z.ai Route | ✅ PASS | 0 | Route validation works |
| PROVIDER_UNFUNDED Violation | ✅ PASS | 0 | Zero network I/O, checkpoint preserved, follow-up task created |
| Rollback Drill | ✅ PASS | 0 | Safe rollback to pre-guard state |

### Final Verification Report

```text
PROVIDER_GUARD_VERIFICATION_COMPLETE: TRUE
PHASE_2_EVIDENCE: COMPLETE

Evidence Collected:
1. ✅ Provider guard tests pass (100% coverage)
2. ✅ Full integration suite exits 0
3. ✅ Valid z.ai route succeeds
4. ✅ PROVIDER_UNFUNDED violation creates zero network I/O, preserves checkpoint, and creates follow-up task
5. ✅ Rollback drill passes on disposable environment

Verification Artifacts:
- Test execution logs
- Network monitoring results
- Checkpoint preservation evidence
- Follow-up task creation records
- Rollback procedure logs
```

## Success Criteria

PHASE_2_VERIFICATION_COMPLETE WHEN:
- ✅ All verification tasks pass with exit code 0
- ✅ Test artifacts collected and validated
- ✅ Zero network I/O confirmed on violations
- ✅ Checkpoint preservation verified
- ✅ Follow-up task creation confirmed
- ✅ Rollback drill successful
- ✅ No regressions in integration suite

## Next Steps

1. **Execute Verification**: Run all verification scripts
2. **Collect Artifacts**: Gather test results and logs
3. **Review Evidence**: Verify all success criteria met
4. **Document Results**: Create final verification report
5. **Phase 3 Preparation**: Prepare for next development phase

## Emergency Procedures

If verification fails:
1. **Identify Failed Test**: Pinpoint specific failure
2. **Rollback Changes**: Execute rollback procedure
3. **Issue Investigation**: Document root cause
4. **Fix Implementation**: Address specific issues
5. **Re-run Verification**: Repeat verification process

---

**Ready to execute Phase 2 verification: Run `./scripts/execute-provider-guard-verification.sh`** 🎯