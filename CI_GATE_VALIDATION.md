# T27-First CI Gate Validation Plan

**Status**: Ready for validation  
**Next Action**: Step 1 - CI gate validation  
**Purpose**: Prove BLOCK_NEW mode works before agent-runner migration  

## Validation Overview

Before starting the agent-runner T27 migration, we need to validate that the CI gate correctly:

1. ✅ Allows existing 198-violation baseline
2. ✅ Blocks new business logic violations  
3. ✅ Provides actionable error reporting
4. ✅ Integrates properly with CI pipeline

## Test Cases

### 🔬 Test A: Baseline Pass
```bash
# Run on clean repository
./bootstrap/target/debug/t27c audit --baseline specs/architecture/misplaced-logic-inventory.t27

# Expected:
# Exit code: 0 (PASS)
# Current violations: 198
# New violations: 0
```

### 🔬 Test B: New Violation Block
```bash
# Add temporary violation
cat > temp_violation.py << 'EOF'
# TEMPORARY: expected T27-first violation
def temporary_business_rule(value: str) -> str:
    if value == "blocked":
        return "DENY"
    return "ALLOW"
EOF

# Run audit
./bootstrap/target/debug/t27c audit --baseline specs/architecture/misplaced-logic-inventory.t27

# Expected:
# Exit code: != 0 (FAIL)
# New violations: >= 1
# Error points to temp_violation.py
```

### 🔬 Test C: Baseline Reduction
```bash
# Create branch with removed violation
git checkout -b test-baseline-reduction
# Remove one baselined violation
./bootstrap/target/debug/t27c audit --baseline specs/architecture/misplaced-logic-inventory.t27

# Expected:
# Exit code: 0 (PASS)  
# Current violations: 197
# No automatic baseline update
```

### 🔬 Test D: CI Integration
```bash
# Clean branch
git checkout main
./bootstrap/target/debug/t27c audit --baseline specs/architecture/misplaced-logic-inventory.t27
# Expected: GREEN (pass)

# Violation branch  
git checkout test-violation-branch
./bootstrap/target/debug/t27c audit --baseline specs/architecture/misplaced-logic-inventory.t27  
# Expected: RED (fail with clear message)
```

## Acceptance Criteria

CI_GATE_VALIDATED WHEN:
- ✅ Baseline branch passes (198 violations, exit 0)
- ✅ Intentional new violation fails (exit != 0)
- ✅ Report identifies file, rule, and violation count
- ✅ Baseline violations don't fail the build
- ✅ Baseline can only change through reviewed PR
- ✅ No provider/worktree/scheduler scope involved

## Validation Script

**File**: `validate-ci-gate.sh`  
**Usage**: `chmod +x validate-ci-gate.sh && ./validate-ci-gate.sh`

Automates all test cases and validates CI gate behavior before agent-runner migration.

## Temporary Test Violation

**File**: `temp_violation.py`  
**Purpose**: Trigger known violation during validation  
**Cleanup**: Remove after validation complete

```python
# TEMPORARY: expected T27-first violation
def temporary_business_rule(value: str) -> str:
    if value == "blocked":
        return "DENY"
    return "ALLOW"
```

## Definition of Done

STEP_1_COMPLETE WHEN:
- ✅ Baseline audit passes (198 violations)
- ✅ Intentional violation is blocked
- ✅ CI output is actionable and clear
- ✅ Baseline suppression works correctly
- ✅ Test violation is removed
- ✅ Repository returns to clean state

## Next Steps

After validation:
1. ✅ Remove temporary test files
2. 🚀 Start agent-runner T27 migration
3. 📊 Monitor violation count reduction
4. 🔒 Update baseline through review-only PR

---

**Ready to validate CI gate: Run `./validate-ci-gate.sh`** 🎯