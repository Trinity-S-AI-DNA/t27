# T27 Doctor Analysis Report - DRY-RUN COMPLETE

**Timestamp**: 2026-10-29T14:50:14+07:00  
**Analysis Agent**: doctor-agent  
**Status**: DRY-RUN_AUTHORIZED  

---

## 📊 Executive Summary

- **Total repository size**: ~21 GB
- **Total potential reclaimable space**: **~14.5 GB** (69% of repository)
- **Current disk usage**: 99% (3.5GB free)
- **Protection level**: MAXIMUM

---

## 🛡️ PROTECTED ITEMS (SAFETY VERIFIED)

| Item | Path | Size | Status |
|------|------|------|---------|
| Backups | `/Users/playom/t27/backups/**` | 97M | ✅ PROTECTED |
| Active Worktree | `/Users/playom/t27/.wt/lane-t27b-mdl-01` | 933M | ✅ PROTECTED |
| Git Stashes | 6 entries | N/A | ✅ PROTECTED |
| Protected Commit | `0c7ea239b` | N/A | ✅ PROTECTED |
| DSH Checkpoints | System-managed | N/A | ✅ PROTECTED |

---

## 🗑️ CLEANUP CANDIDATES ANALYSIS

### Phase 1: IMMEDIATE SAFE REMOVAL (12.1 GB)

| Category | Path | Size | Safety Level | Action |
|----------|------|------|-------------|---------|
| **Rust Build Artifacts** | `./target` | 6.3 GB | ⚠️ HIGH RISK | Verify before removal |
| **Rust Build Artifacts** | `.wt/lane-ml-parity/target` | 460 MB | ✅ SAFE | Remove immediately |
| **Rust Build Artifacts** | `.wt/fpga-petal-home/target` | 379 MB | ✅ SAFE | Remove immediately |
| **Rust Build Artifacts** | `.wt/lane-ml-parity2/target` | 328 MB | ✅ SAFE | Remove immediately |
| **Rust Build Artifacts** | `.wt/lane-9065/target` | 178 MB | ✅ SAFE | Remove immediately |
| **Agent Worktree** | `.agents/worktrees/t27f-mega-map-11038/target` | 227 MB | ✅ SAFE | Remove immediately |
| **Claude Worktree** | `.claude/worktrees/m1-alternates/target` | 460 MB | ✅ SAFE | Remove immediately |
| **Node.js Cache** | `contrib/backend/api/node_modules` | 242 MB | ✅ SAFE | Remove immediately |
| **FPGA Build** | `./build` | 3.6 GB | ✅ SAFE | Remove immediately |

### Phase 2: VERIFY BEFORE REMOVAL (2.2 GB)

| Category | Path | Size | Risk Level | Verification |
|----------|------|------|------------|-------------|
| **Inactive Worktrees** | `.wt/lane-ml-parity` | 798 MB | ⚠️ MEDIUM | Check branch status |
| **Inactive Worktrees** | `.wt/fpga-petal-home` | 784 MB | ⚠️ MEDIUM | Check branch status |
| **Inactive Worktrees** | `.wt/lane-ml-parity2` | 674 MB | ⚠️ MEDIUM | Check branch status |
| **Inactive Worktrees** | `.wt/lane-9065` | 581 MB | ⚠️ MEDIUM | Check branch status |
| **Inactive Worktrees** | `.wt/lane-actor-march` | 403 MB | ⚠️ MEDIUM | Check branch status |
| **Inactive Worktrees** | `.wt/lane-12413` | 403 MB | ⚠️ MEDIUM | Check branch status |
| **Inactive Worktrees** | `.wt/board3-claim-refs` | 403 MB | ⚠️ MEDIUM | Check branch status |
| **Temporary Worktrees** | `.wt/scratch-golden-test` | 346 MB | ⚠️ LOW | Verify not needed |
| **Temporary Worktrees** | `.wt/lane-blog-t27b-29` | 344 MB | ⚠️ LOW | Verify not needed |

### Phase 3: ARCHIVE BEFORE REMOVAL (~1 MB)

| Category | Path | Size | Action |
|----------|------|------|---------|
| **Old Logs** | `.trinity/state/*.log` | ~1 MB | Archive first |
| **Training Logs** | `train_*.log` | ~10 KB | Archive first |

---

## 🎯 RECOMMENDED CLEANUP SEQUENCE

### STEP 1: IMMEDIATE SAFE CLEANUP (12.1 GB)
```bash
# High-confidence removals
rm -rf .wt/lane-ml-parity/target
rm -rf .wt/fpga-petal-home/target
rm -rf .wt/lane-ml-parity2/target
rm -rf .wt/lane-9065/target
rm -rf .wt/lane-actor-march/target
rm -rf .wt/lane-12413/target
rm -rf .wt/board3-claim-refs/target
rm -rf .agents/worktrees/t27f-mega-map-11038/target
rm -rf .claude/worktrees/m1-alternates/target
rm -rf contrib/backend/api/node_modules
rm -rf build
```

### STEP 2: VERIFICATION AND MANUALLY APPROVED CLEANUP
```bash
# Check branch status before worktree removal
git branch -a | grep -E "(lane-ml-parity|fpga-petal-home|lane-9065|lane-actor-march|lane-12413|board3-claim-refs)"

# Remove inactive worktrees (only if branches don't exist)
# Manual approval required for each worktree
```

### STEP 3: ARCHIVAL
```bash
# Archive old logs
mkdir -p ~/t27-logs-archive/$(date +%Y-%m-%d)
mv .trinity/state/*.log ~/t27-logs-archive/$(date +%Y-%m-%d)/
mv train_*.log ~/t27-logs-archive/$(date +%Y-%m-%d)/
```

---

## 🚨 RISK ASSESSMENT

| Risk Level | Items | Action Required |
|------------|-------|----------------|
| **LOW RISK** | Build artifacts, caches, inactive worktrees | Safe to remove immediately |
| **MEDIUM RISK** | Worktrees with possible branch references | Verify before removal |
| **HIGH RISK** | Main project build artifacts (`./target`) | Extra verification needed |
| **PROTECTED** | All backup files, stashes, active worktree | Never remove |

---

## 🔧 INTERACTIVE CLEANUP TOOL

**File**: `doctor-cleanup-script.sh`  
**Usage**: `bash doctor-cleanup-script.sh`  
**Features**:
- Interactive approval for each deletion
- Automatic backup verification
- Branch status checking
- Step-by-step execution
- Total space tracking

---

## 📋 NEXT STEPS

1. **Run Analysis**: Verify current state with `bash doctor-analysis.sh`
2. **Interactive Cleanup**: Execute `bash doctor-cleanup-script.sh`
3. **Post-Cleanup Verification**: Check all protected items
4. **T27 Lagger Patch**: Continue with original development task

---

## 💡 RECOVERY PLAN

If issues occur during cleanup:
```bash
# Emergency rollback commands
git worktree add ../t27-recovery main
git reset --hard HEAD@{1}
git stash pop
```

**Emergency contact**: Immediate restoration from backups

---

**DOCTOR STATUS**: READY FOR CLEANUP  
**CLEANUP AUTHORIZATION**: PENDING_USER_APPROVAL  
**SAFETY LEVEL**: MAXIMUM