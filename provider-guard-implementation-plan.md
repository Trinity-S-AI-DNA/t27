# Provider Guard Implementation Plan

## Overview
Implement and verify a comprehensive provider guard system as a fail-fast preflight invariant gate that blocks violations before network calls, preserves checkpoints, creates follow-up tasks, and prevents silent fallbacks.

## Current State Analysis

### ✅ Already Implemented
1. **Basic Provider Gate**: `specs/system/llm_request_provider_gate.t27`
2. **API Integration**: `contrib/backend/agent-runner/src/api.rs`
3. **Error Classification**: `PROVIDER_UNFUNDED` and other violation types
4. **Reject Function**: Blocks requests with proper error handling

### 🔧 Required Improvements
1. **Comprehensive Testing**: Full test suite covering all guard scenarios
2. **Circuit Breaker**: Prevent repeated violations from hitting providers
3. **Monitoring & Metrics**: Guard overhead and effectiveness tracking
4. **Documentation**: Complete CLI and integration documentation
5. **Integration Testing**: End-to-end verification with provider mocks

## Implementation Tasks

### Phase 1: Core Guard Verification
1. **Verify Existing Implementation**
   - Review current spec compliance
   - Check API integration completeness
   - Validate error handling flow

2. **Enhanced Spec Implementation**
   - Add missing invariant checks
   - Implement circuit breaker state
   - Add comprehensive test coverage

### Phase 2: Testing Infrastructure
1. **Unit Tests**
   - `specs/system/provider_guard_unit_tests.t27`
   - Complete violation scenario coverage
   - Performance benchmarking

2. **Integration Tests**
   - `tests/provider_guard_integration.test.ts`
   - Mock provider responses
   - End-to-end workflow verification

3. **Network Mock Tests**
   - `tests/provider_guard_network_mock.test.ts`
   - HTTP 503 simulation
   - Provider timeout scenarios

### Phase 3: Circuit Breaker Implementation
1. **State Management**
   - Provider health tracking
   - Circuit breaker states (CLOSED, OPEN, HALF_OPEN)
   - Recovery mechanism

2. **Monitoring & Metrics**
   - Guard overhead measurement
   - Violation rate tracking
   - Recovery success metrics

### Phase 4: CLI Integration
1. **Diagnostic CLI**
   - `scripts/provider_guard_doctor.ts`
   - Real-time guard status
   - Configuration validation

2. **Monitoring Commands**
   - Guard health check
   - Violation history
   - Provider status report

### Phase 5: Documentation & Rollout
1. **Complete Documentation**
   - Integration guide
   - Troubleshooting manual
   - Performance benchmarks

2. **Rollback Procedures**
   - Safe rollback script
   - Fallback mechanisms
   - Emergency procedures

## Key Requirements

### Guard Semantics
- **Fail-Fast**: Blocks requests before network I/O
- **Checkpoint Preservation**: No state mutation on violations
- **Follow-Up Tasks**: Create operator tasks for all violations
- **No Silent Fallbacks**: All violations must be explicit
- **Circuit Breaker**: Block repeated violations

### Test Requirements
1. **Full CI Suite Passes**: All guard tests must pass
2. **Preflight Coverage**: Every active session, model, route, credential scope
3. **No Silent Paths**: All violation scenarios covered
4. **Error Classification**: Each violation properly categorized
5. **Retry Prevention**: No automatic retries on violations
6. **Performance**: Guard overhead < 5%

### Specific Tests
- `PROVIDER_ROUTING_MISMATCH` blocks retry
- `PROVIDER_UNFUNDED` blocks retry and preserves checkpoint
- `CREDENTIAL_ERROR` blocks retry
- `MODEL_PROVIDER_COMPATIBILITY` blocks retry
- Explicit fallback requires approved configuration

## PR Scope

### Include
- Provider binding validation
- Model-provider compatibility
- Credential consistency
- Worktree/issue ownership
- Error classification
- Retry budget enforcement
- Doctor preflight CLI
- Network-mock tests
- Documentation and .t27 spec

### Exclude
- Automatic provider switching
- Silent fallback
- Retry on unfunded/config errors
- Credential values in logs

## Success Metrics

### Implementation Status
- **Implementation**: COMPLETE_PENDING_VERIFICATION
- **Deployment**: NOT_READY_YET
- **Guard Semantics**: CORRECT

### Verification Checklist
- [ ] Full CI suite passes
- [ ] Preflight runs for every active session, model, route, credential scope
- [ ] No undeclared silent fallback path remains
- [ ] PROVIDER_ROUTING_MISMATCH blocks retry
- [ ] PROVIDER_UNFUNDED blocks retry and preserves checkpoint
- [ ] CREDENTIAL_ERROR blocks retry
- [ ] MODEL_PROVIDER_COMPATIBILITY blocks retry
- [ ] Explicit fallback requires approved configuration
- [ ] CLI emits actionable JSON + human diagnostics
- [ ] Rollback drill succeeds on disposable environment
- [ ] Measured guard overhead < 5%

## Next Steps

1. **Phase 1**: Verify existing implementation completeness
2. **Phase 2**: Implement comprehensive test suite
3. **Phase 3**: Add circuit breaker and monitoring
4. **Phase 4**: Create CLI tools and documentation
5. **Phase 5**: Deployment and rollback verification