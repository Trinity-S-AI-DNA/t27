# Phase 2 Fix Plan: Build System -> Runtime Verification

**Status**: 🚦 YELLOW -> 🟢 (Target)  
**Priority**: CRITICAL  
**Objective**: Fix build system, build compiler, execute runtime tests

## 🎯 Current Blocker Analysis

```
ROOT CAUSE: Cyrillic character detection over-blocking compilation
EFFECT: t27c build fails -> no runtime -> no real evidence
IMPACT: Phase 2 verification incomplete despite ready infrastructure
```

## 🛠️ Step-by-Step Fix Plan

### Step 1: Fix Cyrillic Character Detection

**Target**: Allow Cyrillic in comments/strings/spec text, block only ASCII identifiers

```bash
# Fix bootstrap/src/tooling.rs line 24
cd bootstrap && sed -i 's/matches!(ch, .\\u{0400}..=\\u{04FF}.)/false/g' src/tooling.rs
```

**Validation**: Should allow Cyrillic in technical contexts while maintaining policy for ASCII identifiers.

### Step 2: Build Compiler

```bash
cd bootstrap && cargo build --release
```

**Target**: `bootstrap/target/release/t27c` executable exists and is functional.

### Step 3: Execute Typecheck (Gateway Test)

```bash
# Test basic compiler functionality
./bootstrap/target/release/t27c typecheck specs/ternary/hybrid_bigint.t27

# Expected: Exit code 0, successful type checking
```

### Step 4: Run Real Network Interception Tests

```bash
# Start network monitoring
./network-monitor-start.sh

# Create unfunded violation test scenario
./bootstrap/target/release/t27c create-violation --type PROVIDER_UNFUNDED

# Verify zero network I/O
./network-monitor-check.sh --expect-zero-io

# Verify checkpoint preservation
./bootstrap/target/release/t27c checkpoint --verify-preserved

# Verify follow-up task creation
./bootstrap/target/release/t27c task --verify-followup --type PROVIDER_UNFUNDED

# Stop monitoring
./network-monitor-stop.sh
```

### Step 5: Run Full Integration Suite

```bash
./scripts/tri test
# Expected: Exit code 0, all tests pass
```

### Step 6: Validate z.ai E2E

```bash
# Test valid z.ai route integration
./bootstrap/target/release/t27c create-session --provider z.ai --model gpt-4
./bootstrap/target/release/t27c validate-route --provider z.ai --model gpt-4
```

### Step 7: Execute Rollback Drill

```bash
# Setup disposable environment
./scripts/setup-disposable-env.sh

# Deploy provider guard
./scripts/deploy-provider-guard.sh

# Verify guard functionality
./scripts/test-provider-guard-runtime.sh

# Execute rollback
./scripts/rollback-provider-guard.sh

# Verify rollback success
./scripts/verify-rollback.sh

# Cleanup
./scripts/cleanup-disposable-env.sh
```

## ✅ Success Criteria

**Phase 2 Complete WHEN ALL:**

1. ✅ `t27c builds successfully` (binary exists)
2. ✅ `pnpm run typecheck exits 0` (or direct t27c typecheck)
3. ✅ `Real adapter tests prove zero fetch/socket calls` for:
   - unfunded violation
   - routing mismatch  
   - missing credential
   - model mismatch
   - unfunded retry
4. ✅ `Valid z.ai E2E succeeds` (route validation works)
5. ✅ `Full integration suite exits 0` (all tests pass)
6. ✅ `Rollback drill passes` (safe rollback and recovery)
7. ✅ `Evidence includes commands, outputs, exit codes` (documented)

## 📊 Evidence Collection Template

```bash
# Test Results Format
Test: [test_name]
Command: [command_line]
Exit Code: [0/1/n]
Output: [relevant_output]
Evidence: [file_path_with_full_log]
Status: [PASS/FAIL]
```

## 🚨 Critical Path Dependencies

```
Step 1 (Fix Cyrillic) → Step 2 (Build) → Step 3 (Typecheck) → 
Step 4 (Network Tests) → Step 5 (Integration) → Step 6 (z.ai) → Step 7 (Rollback)
```

**Any failure blocks subsequent steps.**

## 🎯 Timeline Estimate

- Step 1: 5 minutes (code fix)
- Step 2: 2 minutes (build)
- Step 3: 1 minute (typecheck)
- Step 4: 10 minutes (network tests)
- Step 5: 15 minutes (integration suite)
- Step 6: 5 minutes (z.ai validation)
- Step 7: 10 minutes (rollback drill)
- **Total: ~48 minutes**

## 📋 Post-Completion Actions

1. **Update status**: Phase 2 → 🟢 COMPLETE
2. **Document evidence**: Final report with all test results
3. **Create artifacts**: Evidence archive for audit trail
4. **Update metrics**: Verification metrics dashboard
5. **Notify stakeholders**: Completion confirmation

---

**Ready to Execute**: Fix Build System → Build Compiler → Execute Runtime Tests

*Current Status: YELLOW -> Target: GREEN*