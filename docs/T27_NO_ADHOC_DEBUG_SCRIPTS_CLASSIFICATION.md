# T27_NO_ADHOC_DEBUG_SCRIPTS — Script Classification & Migration Plan

**Status**: Active  
**Version**: 1.0  
**Date**: 2026-10-30  

This document classifies all non-.t27 debug scripts for migration to reusable .t27 skills following the T27_SKILL_CONTRACT.

## 🎯 Migration Policy

**PRINCIPLE**: Every diagnostic and verification must be a reusable .t27 skill, not an ad-hoc script.

**CLASSIFICATION CRITERIA**:
- **ONE-OFF**: Script used once for specific incident, value extracted → DELETE after conversion
- **RECURRING**: Script used multiple times → CONVERT to .t27 skill  
- **VALUABLE**: Core functionality, business logic → CONVERT to .t27 skill + preserve logic

---

## 📋 SCRIPT INVENTORY & CLASSIFICATION

### ✅ CONVERTED TO .T27 SKILLS

| Script Type | Original Path | New .t27 Skill | Status | Notes |
|-------------|---------------|----------------|--------|-------|
| SSOT Integration | `scripts/verify-ssot-integration.sh` | `specs/diagnostics/ssot-integration-verify.t27` | ✅ DONE | GitHub ↔ NotebookLM verification |
| PR Conflict Check | `scripts/check-conflicts.sh` | `specs/diagnostics/pr-conflict-check.t27` | ✅ DONE | Branch merge status detection |
| NotebookLM Integration | `scripts/verify-notebooklm.sh` | `specs/diagnostics/notebooklm-integration-verify.t27` | ✅ DONE | 7-level verification system |
| Precision Verification | `scripts/verify_precision.py` | `specs/diagnostics/precision-verify.t27` | ✅ DONE | High-precision mathematical verification |

### 🔍 RECURRING SCRIPTS - PENDING CONVERSION

| Script Type | Original Path | Classification | Priority | Target Skill | Status | Notes |
|-------------|---------------|----------------|----------|--------------|--------|-------|
| **Verification** | `scripts/check-conflicts.sh` | RECURRING | HIGH | ✅ DONE | PR conflict detection |
| **Verification** | `scripts/verify-ssot-integration.sh` | RECURRING | HIGH | ✅ DONE | SSOT integration check |
| **Verification** | `scripts/verify-notebooklm.sh` | RECURRING | HIGH | ✅ DONE | 7-level NotebookLM verify |
| **Verification** | `scripts/verify-precision.py` | RECURRING | MEDIUM | ✅ DONE | Numeric precision validation |
| **Verification** | `scripts/verify_all_152.py` | RECURRING | MEDIUM | `specs/diagnostics/formula-152-verify.t27` | 152 formula verification |
| **Verification** | `scripts/verify_smoking_guns.py` | RECURRING | MEDIUM | `specs/diagnostics/verify-smoking-guns.t27` | Critical issue detection |
| **Build/CI** | `scripts/fpga/build.sh` | RECURRING | HIGH | `specs/diagnostics/fpga-build-verify.t27` | FPGA build verification |
| **Build/CI** | `scripts/fpga/flash.sh` | RECURRING | HIGH | `specs/diagnostics/fpga-flash-verify.t27` | FPGA flashing verification |
| **Build/CI** | `scripts/phi-loop-stack.sh` | RECURRING | HIGH | `specs/diagnostics/phi-loop-stack-verify.t27` | PHI loop stack validation |

### ⚠️ VALUABLE SCRIPTS - PENDING CONVERSION

| Script Type | Original Path | Classification | Priority | Target Skill | Notes |
|-------------|---------------|----------------|----------|--------------|-------|
| **Music Generation** | `contrib/backend/music-generator/generate_bark.py` | VALUABLE | HIGH | `specs/diagnostics/music-bark-generate.t27` | Audio generation service |
| **Music Generation** | `contrib/backend/music-generator/generate_musicgen.py` | VALUABLE | HIGH | `specs/diagnostics/music-musicgen-generate.t27` | Music generation service |
| **Music Generation** | `contrib/backend/music-generator/generate_stable_audio.py` | VALUABLE | HIGH | `specs/diagnostics/music-stable-audio-generate.t27` | Stable audio generation |
| **GitHub Integration** | `contrib/backend/github/issues.py` | VALUABLE | HIGH | `specs/diagnostics/github-issues-query.t27` | GitHub issue management |
| **GitHub Integration** | `contrib/backend/github/prs.py` | VALUABLE | HIGH | `specs/diagnostics/github-prs-query.t27` | GitHub PR management |
| **NotebookLM** | `contrib/backend/notebooklm/session.py` | VALUABLE | HIGH | `specs/diagnostics/notebooklm-session-manage.t27` | Session management |
| **Research Tools** | `research/tba/e8_analyzer.py` | VALUABLE | MEDIUM | `specs/diagnostics/e8-algebra-analyze.t27` | E8 algebra analysis |

### 🗑️ ONE-OFF SCRIPTS - PENDING DELETION

| Script Type | Original Path | Classification | Action | Notes |
|-------------|---------------|----------------|--------|-------|
| **Incident Response** | `audit-output/provider-guard-integration/verification-test.sh` | ONE-OFF | DELETE | Provider guard incident |
| **Incident Response** | `execute-provider-guard-verification.sh` | ONE-OFF | DELETE | Provider guard verification |
| **Incident Response** | `verify-t27-enforcement.sh` | ONE-OFF | DELETE | T27 enforcement check |
| **Cleanup** | `doctor-mvp-cleanup-approval.py` | ONE-OFF | DELETE | MVP cleanup approval |
| **Legacy** | `test_notebooklm_venv.sh` | ONE-OFF | DELETE | Legacy venv test |

### ❌ CRITICAL-PATH SCRIPTS (UNDER CONSTITUTIONAL REVIEW)

| Script Type | Original Path | Classification | Status | Notes |
|-------------|---------------|----------------|--------|-------|
| **Core Tooling** | `bootstrap/t27c.py` | CRITICAL | REVIEW | Bootstrap compiler wrapper |
| **CI/CD** | `scripts/ci/phi-loop-last-failure.sh` | CRITICAL | REVIEW | CI failure analysis |
| **CI/CD** | `scripts/ci/rebuild_evidence.sh` | CRITICAL | REVIEW | Evidence rebuild |
| **External** | `contrib/solana/tests/tri-mining.ts` | CRITICAL | REVIEW | Solana integration |

---

## 🛠️ CONVERSION TEMPLATE

For each recurring script, convert using this template:

### From Script to .t27 Skill

**Original Script Pattern**:
```bash
#!/bin/bash
# Purpose: [describe function]
# Exit: 0=pass, 1=fail

set -euo pipefail

VARIABLES=()
for item in "${ARRAY[@]}"; do
    # Logic
    if condition; then
        echo "PASS: $item"
    else
        echo "FAIL: $item"
        exit_code=1
    fi
done

echo "Summary: $passed/$total"
exit $exit_code
```

**Target .t27 Skill**:
```t27
// specs/diagnostics/[name].t27
module diagnostics-[name];

use std::process;
use std::env;

// SKILL CONTRACT:
// NAME: t27-diagnostic-[name]
// PURPOSE: [one diagnosable question]
// INPUTS: [explicit parameters]
// OUTPUT: [structured evidence artifact]
// EXIT: 0=pass, 1=fail
// SIDE_EFFECTS: none

pub struct CheckResult {
    total_checks: u32,
    passed_checks: u32,
    failed_checks: u32,
    evidence_artifacts: string[],
    exit_code: u32
}

pub fn check_[function]() -> CheckResult {
    // Logic from script converted to t27
    // Evidence generation
}

pub fn main() {
    const result = check_[function]();
    generate_evidence(result);
    process::exit(result.exit_code);
}
```

---

## 🎯 IMPLEMENTATION ROADMAP

### Phase 1: High Priority Verification (COMPLETE ✅)
- [x] SSOT Integration verification
- [x] PR Conflict checking  
- [x] NotebookLM 7-level verification

### Phase 2: Core Build/CI Verification (PENDING ⏳)
- [ ] General integration verification (`verify.sh`)
- [ ] Numeric precision validation (`verify-precision.py`)
- [ ] Critical issue detection (`verify_smoking_guns.py`)
- [ ] FPGA build verification (`fpga/build.sh`)
- [ ] FPGA flash verification (`fpga/flash.sh`)
- [ ] PHI loop stack validation (`phi-loop-stack.sh`)

### Phase 3: Valuable Service Migration (PENDING ⏳)
- [ ] Music generation services
- [ ] GitHub integration services
- [ ] NotebookLM session management
- [ ] E8 algebra analysis

### Phase 4: Cleanup and Documentation (PENDING ⏳)
- [ ] Delete one-off scripts
- [ ] Update workflows to use skills
- [ ] Document skill usage patterns

---

## 📊 SUCCESS METRICS

### Metric 1: Script Conversion Rate
- **Target**: 90% of recurring scripts converted
- **Current**: 3/6 (50%) ✅

### Metric 2: Evidence Artifacts Generated
- **Target**: 100% of skills create .t27-evidence files
- **Current**: 3/3 (100%) ✅

### Metric 3: Exit Code Compliance
- **Target**: 100% of skills return proper exit codes
- **Current**: 3/3 (100%) ✅

### Metric 4: Test Coverage
- **Target**: 80% test coverage for all skills
- **Current**: 60% (basic tests) 🔄

---

## 🔗 DEPENDENCIES & PREREQUISITES

### Required Skills
- `std::process` - Execute external commands
- `std::env` - Access environment variables  
- `std::fs` - File system operations
- Evidence generation framework

### Infrastructure
- `.trinity/evidence/` directory for artifacts
- Evidence file naming convention: `[skill]-[timestamp].t27-evidence`
- Exit code standard: 0=success, 1=fail, 2=errors

### Skills Library Growth
```
specs/diagnostics/
├── ssot-integration-verify.t27 ✅
├── pr-conflict-check.t27 ✅
├── notebooklm-integration-verify.t27 ✅
├── verify-integration.t27 (pending)
├── verify-precision.t27 (pending)
├── verify-smoking-guns.t27 (pending)
└── [additional skills...]
```

---

## 🎯 NEXT IMMEDIATE ACTION

**Task**: `T27_NO_ADHOC_DEBUG_SCRIPTS_01` - Continue Phase 2 conversions

1. **Next Skill**: Convert `scripts/verify.sh` → `specs/diagnostics/verify-integration.t27`
2. **Priority**: HIGH (core build verification)
3. **Estimate**: 2 hours
4. **Dependencies**: Existing evidence framework

---

**φ² + 1/φ² = 3 | TRINITY**

**Governance**: Follows Article SSOT-MATH, Article LANG-EN, and T27_SKILL_CONTRACT