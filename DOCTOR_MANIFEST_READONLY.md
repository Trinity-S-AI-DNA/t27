# T27 Doctor - Read-Only Analysis Manifest
**Timestamp**: 2026-10-29T15:00:16+07:00  
**Status**: BLOCKED_NO_DELETIONS  
**Disk**: 100% used (324MB free) - CRITICAL BUT MANAGEABLE  

---

## 🚫 CLEANUP BLOCKED - PER COMMAND

- ❌ `doctor-cleanup-script.sh` - BLOCKED  
- ❌ Worktree deletion - BLOCKED  
- ❌ Build artifact deletion - BLOCKED  
- ❌ Cache deletion - BLOCKED  

---

## 📊 DSH ANALYSIS

| Path | Size | Type | Active Work Risk | Safe to Delete | Backup Required |
|------|------|------|------------------|---------------|-----------------|
| `/Users/playom/deepseek-harness/node_modules` | 2.0G | Node.js modules | HIGH ❌ | BLOCKED | YES |
| `/Users/playom/deepseek-harness` | MODIFIED | Git workdir | HIGH ❌ | BLOCKED | YES |
| `/Users/playom/deepseek-harness/.agents/worktrees/unpatched-validation` | N/A | Worktree | HIGH ❌ | BLOCKED | YES |

**DSH Status**: 
- Branch: `fix/cordis-preserve-fiber-state` 
- Modified files: 31 tracked + 39 untracked
- Worktrees: 2 active
- Risk: HIGH - active development with modified files

---

## 🛡️ T27 PROTECTED ITEMS (VERIFIED)

| Path | Size | Type | Status | Risk |
|------|------|------|---------|------|
| `/Users/playom/t27/backups` | 97M | Backup files | ✅ PROTECTED | NONE |
| `/Users/playom/t27/.wt/lane-t27b-mdl-01` | 933M | Active worktree | ✅ PROTECTED | NONE |
| Git stashes (6 entries) | N/A | Version control | ✅ PROTECTED | NONE |
| Commit `0c7ea239b` | N/A | Git object | ✅ PROTECTED | NONE |
| `/Users/playom/t27/backups/*.bundle` | 97M+ | Repository bundles | ✅ PROTECTED | NONE |

---

## ⚠️ HIGH RISK CANDIDATES (BLOCKED)

| Path | Size | Type | Active Work Risk | Reason for Block |
|------|------|------|------------------|------------------|
| `/Users/playom/t27/target` | 6.3G | Rust build | ❌ HIGH | May affect active worktree |
| `/Users/playom/t27/.wt/lane-ml-parity/target` | 396M | Build artifact | ❌ MEDIUM | Linked to active branch |
| `/Users/playom/t27/.wt/fpga-petal-home` | 784M | Worktree | ❌ MEDIUM | May have unpushed work |
| `/Users/playom/t27/.wt/lane-ml-parity` | 798M | Worktree | ❌ MEDIUM | Branch status unknown |
| All other worktrees | ~4.3G | Worktrees | ❌ MEDIUM | Ahead/behind status unknown |

---

## 🔍 SPECIFIC RISK ANALYSIS

### FPGA Artifacts (HIGH RISK)
```
Path: /Users/playom/t27/.wt/fpga-petal-home
Size: 784M
Risk: HIGH - Could contain critical unpushed FPGA work
Status: BLOCKED until owner verification
```

### Active Worktree Artifacts (PROTECTED)
```
Path: /Users/playom/t27/.wt/lane-t27b-mdl-01/target  
Size: 587M
Status: PROTECTED - Active development work
Risk: NONE - Explicitly protected
```

### Main Repository Build (HIGH RISK)
```
Path: /Users/playom/t27/target
Size: 6.3G  
Risk: HIGH - May contain ongoing compilation artifacts
Status: BLOCKED - No backup plan available
```

---

## 📋 TRANSPORT UNFUNDED CLASSIFICATION - MINIMAL PATCH PLAN

### Problem Statement
```
HTTP 503 + "deepseek unfunded" -> PROVIDER_UNFUNDED
```

### Required Changes
1. **Transport Layer**: Update error classification
2. **Status Codes**: Map 503 to unfunded status  
3. **Provider Integration**: Handle unfunded state gracefully
4. **Circuit Breaker**: Ensure proper isolation

### Minimal Patch Files
- `packages/llm/llm-deepseek/src/transport.ts` - Transport logic
- `packages/api/remote-access/src/types.ts` - Error types  
- `packages/client/connection/src/rpc.ts` - Client handling

### Backup Requirements
- Current transport state: `/Users/playom/deepseek-harness` 
- Untracked changes: 39 files
- Risk: HIGH - Active development

---

## 🚦 CURRENT STATUS

```text
DOCTOR_CLEANUP: BLOCKED ✅
DISK: CRITICAL_BUT_MANAGEABLE (324MB free)
PRIORITY: TRANSPORT_UNFUNDED_FIX ✅
EXECUTOR_LAUNCH: BLOCKED ✅
LIVE_SMOKE: BLOCKED ✅
MASS_CLEANUP: BLOCKED ✅
```

---

## 💡 RECOMMENDED APPROACH

1. **Immediate**: Apply minimal transport patch (no deletion)
2. **Monitor**: Disk usage after patch
3. **Selective**: Only delete specific paths if space critical
4. **Backup**: Always backup before any deletion
5. **Verify**: Test functionality after each change

---

## 🔄 ALTERNATIVE SPACE STRATEGIES

If transport patch requires more space:

1. **DSH Cleanup**: Target specific `/deepseek-harness/node_modules` (2.0G)
2. **Temporary**: Compress large files
3. **Relocate**: Move non-critical data to external storage
4. **Expand**: Consider disk expansion if critical

---

## 🎯 NEXT ACTIONS

1. ✅ **Complete**: Read-only analysis manifest  
2. ⏳ **Pending**: Minimal transport patch implementation  
3. ⏳ **Conditional**: Selective cleanup (only if absolutely necessary)  

**Priority Order**: Transport Fix > Space Management > Optional Cleanup