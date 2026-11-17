# STASH1 EXTRACTION AND CLASSIFICATION REPORT
Generated: 2026-10-29 13:05:00 Asia/Bangkok

## BACKUP VERIFICATION
- ✅ File list extracted: 80 files identified
- ✅ Stat info captured: 133,893 insertions, 951 deletions
- ✅ Full patch backup: `/Users/playom/t27/backups/stash1-extraction/full.patch`

---

## STASH1_CLASSIFICATION_MATRIX

| File | Purpose | T27_LAGGER_RELATED | GENERATED | SAFE_TO_RESTORE | RISK | Notes |
|---|---|---|---|---|---|---|
| **LLM ROUTING** | | | | | | |
| specs/llm/key_pool_rotation.t27 | **CORE** | ✅ HIGH | ❌ NO | ✅ LOW | Pool rotation logic | **PRIORITY 1** |
| specs/llm/restore_sh_fixes.t27 | SUPPORT | ✅ MEDIUM | ❌ NO | ✅ LOW | Shell fixes | |
| **T27OS CORE** | | | | | | |
| specs/t27os/heartbeat_monitor*.t27 | **CORE** | ✅ HIGH | ❌ NO | ✅ LOW | Circuit breaker base | **PRIORITY 1** |
| specs/t27os/trinity_time.t27 | CORE | ✅ HIGH | ❌ NO | ✅ LOW | Time standard | |
| specs/t27os/service_manager.t27 | CORE | ✅ HIGH | ❌ NO | ✅ LOW | Service orchestration | |
| specs/t27os/golden_dashboard.t27 | CORE | ✅ HIGH | ❌ NO | ✅ LOW | Monitoring UI | |
| specs/t27os/acceleration_pareto.t27 | CORE | ✅ HIGH | ❌ NO | ✅ LOW | Performance tracking | |
| **T27OS AEL** | | | | | | |
| specs/t27os/ael_*.t27 | MEDIUM | ✅ MEDIUM | ❌ NO | ✅ LOW | Agent execution loop | |
| **T27F** | | | | | | |
| specs/t27f/mega_map.t27 | MEDIUM | ✅ MEDIUM | ❌ NO | ✅ LOW | T27F specifications | |
| specs/crons/acceleration_loop.t27 | MEDIUM | ✅ MEDIUM | ❌ NO | ✅ LOW | Acceleration logic | |
| specs/crons/competitive_intelligence.t27 | MEDIUM | ✅ MEDIUM | ❌ NO | ✅ LOW | CI specs | |
| specs/crons/pareto_analyzer.t27 | MEDIUM | ✅ MEDIUM | ❌ NO | ✅ LOW | Performance analysis | |
| **GENERATED ARTIFACTS** | | | | | | |
| specs/scratch/w899_bench_module_*.t27 | ❌ NO | ❌ NO | ✅ HIGH | ❌ HIGH | 117K lines generated | **AVOID** |
| specs/spec-ring*.t27 | ❌ NO | ❌ NO | ✅ HIGH | ❌ HIGH | Ring evolution specs | **AVOID** |
| **SKILLS** | | | | | | |
| .agents/skills/*.md | LOW | ✅ LOW | ❌ NO | ✅ LOW | Skill documentation | |
| **SYSTEM** | | | | | | |
| AGENTS.md, README.md, .github/ | LOW | ✅ LOW | ❌ NO | ✅ MEDIUM | System docs | |
| tools/oracle/run.sh | LOW | ✅ LOW | ❌ NO | ✅ LOW | Oracle tool | |

---

## MINIMAL_T27_LAGGER_RESTORE_SET

### CORE ROTATION COMPONENTS (PRIORITY 1)
```
1. specs/llm/key_pool_rotation.t27      # Pool selection logic
2. specs/t27os/heartbeat_monitor*.t27    # Circuit breaker foundation
3. specs/t27os/heartbeat_monitoring*.t27  # Monitoring implementation
4. specs/t27os/trinity_time.t27         # Time standard for operations
```

### SUPPORT COMPONENTS (PRIORITY 2)
```
5. specs/llm/restore_sh_fixes.t27       # Shell fixes for stability
6. specs/t27os/service_manager.t27       # Service orchestration
7. specs/t27os/golden_dashboard.t27      # Monitoring dashboard
8. specs/t27os/acceleration_pareto.t27   # Performance tracking
```

### TOTAL RESTORE SET: 8 files
- **Lines of code**: ~2,000-3,000 (vs 133K total stash)
- **Risk level**: LOW (highly specific, minimal overlap)
- **Generated artifacts excluded**: 117K lines

---

## RESTORE PLAN

### Step 1: Extract Minimal Set (Dry Run)
```bash
cd /Users/playom/t27/.wt/lane-t27b-mdl-01

# Create isolation directory
mkdir -p /Users/playom/t27/stash1-minimal-extract

# Extract only Priority 1 files
git stash show --include-untracked 'stash@{1}' -- specs/llm/key_pool_rotation.t27 \
  > /Users/playom/t27/stash1-minimal-extract/priority1-files.txt

git stash show --include-untracked 'stash@{1}' -- specs/t27os/heartbeat_monitor*.t27 \
  >> /Users/playom/t27/stash1-minimal-extract/priority1-files.txt

git stash show --include-untracked 'stash@{1}' -- specs/t27os/heartbeat_monitoring*.t27 \
  >> /Users/playom/t27/stash1-minimal-extract/priority1-files.txt

git stash show --include-untracked 'stash@{1}' -- specs/t27os/trinity_time.t27 \
  >> /Users/playom/t27/stash1-minimal-extract/priority1-files.txt
```

### Step 2: Apply to Isolated Environment
```bash
# Apply to temporary worktree, not main
git worktree add --detach /tmp/t27-lagger-restore HEAD
cd /tmp/t27-lagger-restore
git stash apply 'stash@{1}' --include-untracked  # Only after careful file selection
```

### Step 3: Test with Mock Transport
```bash
# Test LLM pool rotation with mocked transport
# Verify no 503 errors in isolated environment
```

### Step 4: Incremental Integration
```bash
# Only after successful testing:
# Apply Priority 1 files to main worktree
# Test with deepseek-harness mock
# Enable circuit breaker
```

---

## RISKS ASSESSMENT

### HIGH RISK (AVOID)
- ❌ `specs/scratch/w899_bench_module_*.t27` (117K lines)
- ❌ `specs/spec-ring*.t27` (Ring evolution specs)
- ❌ Full stash application

### MEDIUM RISK (CAUTION)
- ⚠️ Skill files (minimal risk, potential conflicts)
- ⚠️ System docs (low risk, but may overwrite recent changes)

### LOW RISK (SAFE)
- ✅ T27OS core specs (minimal overlap with existing)
- ✅ LLM routing logic (targeted, specific purpose)

---

## CRITICAL INSIGHTS

### Pool Rotation Logic Location
- **Primary**: `specs/llm/key_pool_rotation.t27` (328 lines)
- **Classification**: T27 LAGGER RELATED, NOT GENERATED
- **Purpose**: Provider pool selection and rotation

### Circuit Breaker Foundation
- **Primary**: `specs/t27os/heartbeat_monitor*.t27` series
- **Classification**: T27 LAGGER RELATED, NOT GENERATED  
- **Purpose**: Health monitoring and failure detection

### Generated Artifacts Identified
- **117,291 lines** in single benchmark file
- **Not needed** for LLM rotation restoration
- **Should be excluded** from minimal restore

### Safe Restoration Strategy
- **Target**: 8 core files (~2-3K lines)
- **Exclude**: 72 non-essential files (130K+ lines)
- **Approach**: Incremental, tested integration

---

## NEXT SAFE ACTIONS

1. ✅ **EXTRACT**: File list and backups complete
2. ⚠️ **CLASSIFY**: Matrix analysis complete
3. 🎯 **PLAN**: Minimal restore set identified
4. 🔬 **TEST**: Mock transport testing pending
5. 🚀 **INTEGRATE**: Safe restoration pending

### IMMEDIATE NEXT
Execute Step 1: Extract Priority 1 files to isolated environment for verification.

## STATUS
```
STASH1_EXTRACTION: ✅ COMPLETE
CLASSIFICATION: ✅ COMPLETE  
MINIMAL_SET_IDENTIFIED: ✅ 8_FILES
RESTORE_PLAN: ✅ DESIGNED
NEXT_ACTION: ISOLATED_EXTRACTION
```