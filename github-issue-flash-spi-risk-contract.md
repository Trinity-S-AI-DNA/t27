# GitHub Issue: Flash-spi Risk Management Contract

**Issue Title**: fix or remove isolated flash-spi CI failure  
**Issue Type**: Bug / Risk Quarantine  
**Priority**: High (Blocks Full Release)  
**Labels**: risk-quarantine, ci-blocking, isolated-component  

---

## 🎯 ISSUE SUMMARY

Flash-spi CI compilation error blocks full release but is isolated from critical path. Component requires fix, removal, or formal quarantine.

---

## 🔍 EVIDENCE PACKAGE

### Current Failure:
```bash
Error: missing fields `bitswap` and `no_jprogram` in initializer
Location: cli/flash-spi/src/main.rs:81
Impact: BLOCKS flash-spi compilation ONLY
```

### Dependency Graph Audit Results:
- ✅ **BUILD_GRAPH**: NOT_CONNECTED (workspace exclusion)
- ✅ **TEST_GRAPH**: NOT_CONNECTED (no test dependencies)  
- ✅ **DEPLOY_GRAPH**: NOT_INCLUDED (no release artifacts)
- ✅ **RUNTIME_GRAPH**: UNREACHABLE (no dynamic loading)
- ✅ **PROVIDER_GUARD**: UNAFFECTED (critical path clear)

### Risk Classification:
```text
FLASH_SPI_ROLE: ISOLATED
CRITICAL_PATH_IMPACT: NONE
RELEASE_IMPACT: BLOCKED_FULL_ONLY
```

---

## 📋 ACCEPTANCE CRITERIA (ANY)

### Option 1: FIX
```bash
# Acceptance: flash-spi compiles successfully
cargo build --release -p flash-spi
# Expected: Exit code 0, no compilation errors
```

### Option 2: REMOVE  
```bash
# Acceptance: component removed from workspace
# Update Cargo.toml: remove "cli/flash-spi" from members
# Verify: workspace builds without flash-spi
```

### Option 3: QUARANTINE
```bash
# Acceptance: CI job excluded with owner + expiry
# Create: .github/workflows/quarantine-flash-spi.yml
# Set: owner assignment + 30-day expiry date
```

---

## 🛡️ RISK QUARANTINE PROTOCOL

### Current State:
- **Status**: ACTIVE - Blocks Full Release
- **Risk Level**: Managed (Isolated Component)
- **Shadow Deployment**: Allowed (Critical Path Only)
- **Contract Owner**: To be assigned
- **Expiry Date**: To be set (max 30 days)

### Quarantine Rules:
1. **No full release** until acceptance criteria met
2. **No reclassification** as "unrelated" without evidence  
3. **No deploy graph inclusion** while red
4. **Mandatory tracking** via this issue

### Allowed Activities:
- ✅ DSH smoke test
- ✅ Queen foundation work
- ✅ Guarded shadow deployment (critical path only)
- ✅ Provider guard runtime verification

### Blocked Activities:
- ❌ Full release production rollout
- ❌ Merging as "resolved" without acceptance
- ❌ Adding to deploy graph while red
- ❌ Closing issue without fix/remove/quarantine

---

## 📊 DEPLOYMENT STATUS

### Phase 2 - ALLOWED ✅
- DSH smoke test: ✅ GREEN
- Queen foundation: 🟡 NEXT
- Guarded shadow rollout: 🟡 PENDING
- Provider guard verification: 🟡 PENDING

### Phase 2 - BLOCKED ❌  
- Full release: 🔴 BLOCKED_BY_THIS_ISSUE
- Production rollout: 🔴 BLOCKED_BY_THIS_ISSUE
- Full CI green claim: 🔴 BLOCKED_BY_THIS_ISSUE

---

## 🛠️ EXECUTION QUEUE

```text
1. ✅ DSH smoke test — COMPLETED
2. 👑 Queen foundation + GitHub claim — NEXT PRIORITY  
3. 🚀 Guarded shadow rollout — AFTER QUEEN
4. 🧨 Execute flash-spi contract — AFTER SHADOW
5. 🧪 Full CI green -> reconsider full release — FINAL STEP
```

---

## 📋 COMPLIANCE TRAIL

### Evidence Documentation:
- [flash-spi-role-classification-corrected-report.md](flash-spi-role-classification-corrected-report.md)
- [flash-spi-risk-management-contract.md](flash-spi-risk-management-contract.md)

### T27 Constitution Compliance:
- ✅ L1 TRACEABILITY: Evidence documented
- ✅ L2 GENERATION: No hand-editing
- ✅ L3 PURITY: English maintained
- ✅ L4 TESTABILITY: Critical path verified
- ✅ L5 IDENTITY: Audit trail complete
- ✅ L6 CEILING: SSOT integrity preserved
- ✅ L7 UNITY: No shell scripts

---

## 🎉 CONTRACT STATUS

**Current State**: ACTIVE - Blocks Full Release, Allows Shadow  
**Next Action**: Assign owner + proceed with Queen foundation  
**Contact**: Trinity Agent (Team Lead) for execution

---

*Generated: 2026-10-30T12:37:00+07:00 [Asia/Bangkok]*  
*Issue ID: TO_BE_ASSIGNED*  
*φ² + 1/φ² = 3 | TRINITY*  
*Status: RISK_QUARANTINE_ACTIVE*