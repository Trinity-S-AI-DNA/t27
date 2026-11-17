# Pi-AI Timeout Fix Verification Plan

## 🎯 Status Overview

```text
PI_AI_TIMEOUT_FIX:
CONFIG_APPLIED ✅
DSH_RUNNING ✅ 
VERIFICATION_TOOLS_READY ✅
RUNTIME_TEST_PENDING ⏳
EVIDENCE_PENDING ⏳
TASK_COMPLETE: NO ⛔
```

## 🔧 Applied Configuration

**File:** `/Users/playom/.dsh/profiles/web/cordis.patch.yml`  
**Target Provider:** `t27-ladder`  
**Changes:**
```yaml
streamIdleTimeoutMs: 3600000  # 1 hour (was 300000ms)
timeoutMs: 3600000           # 1 hour HTTP timeout
```

## 📋 Verification Tools Created

1. **`/Users/playom/t27/quick-verify.sh`** - Quick status check
2. **`/Users/playom/t27/verify-pi-ai-timeout-fix.sh`** - Full verification test
3. **`/Users/playom/t27/pi-ai-timeout-fix.md`** - Documentation

## 🚀 Required Next Steps

### Step 1: Restart DSH (Critical)
```bash
# Current DSH PID: Multiple processes detected
# Stop DSH completely:
kill -9 <all-dsh-pids>
pnpm run dev:web
```

### Step 2: Verify Configuration Reload
```bash
/Users/playom/t27/quick-verify.sh
# Should show new configuration loaded
```

### Step 3: Execute Runtime Test
```bash
/Users/playom/t27/verify-pi-ai-timeout-fix.sh
# Will run 1-hour test and generate evidence
```

### Step 4: Collect Evidence
Expected artifacts:
- `verification-report.json` - Machine-readable results
- `task-output.log` - Raw execution log  
- Exit code verification
- Absence of "pi-ai stream idle timeout after 300000ms"

## ✅ Acceptance Criteria

| Criteria | Status | Evidence Required |
|---|---|---|
| Config applied | ✅ | cordis.patch.yml verified |
| DSH restarted | ⏳ | Process logs |
| Long stream test | ⏳ | Execution artifacts |
| No 300000ms timeout | ⏳ | Log analysis |
| Evidence saved | ⏳ | JSON + log files |

## 🎯 Task Completion Conditions

**TASK WILL BE MARKED COMPLETE WHEN:**
1. ✅ Configuration applied to active profile
2. ✅ DSH restarted and configuration reloaded  
3. ✅ Long-stream test executed (1+ hour)
4. ✅ Machine-verifiable evidence collected
5. ✅ No timeout errors detected in logs
6. ✅ Exit code successful

**TASK WILL NOT BE MARKED COMPLETE WITHOUT:**
- Runtime evidence
- Successful long-stream completion
- Verification artifacts

## 🔬 Evidence Requirements

**Evidence must include:**
- Command executed
- Raw output (complete)
- Start/end timestamps
- Exit code
- Log analysis showing no timeout errors
- JSON verification report

## ⚠️ Important Notes

- **Configuration changes require DSH restart** to take effect
- **DSH retains old configuration in memory** while running
- **Timeout fix only applies to t27-ladder provider**
- **Guard functionality remains intact** (still blocks unfunded routes)
- **Evidence must be machine-verifiable** not just anecdotal

## 📞 Contact

Questions about verification process or evidence requirements?