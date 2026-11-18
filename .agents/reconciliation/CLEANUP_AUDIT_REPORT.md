# DSH Cleanup Audit and Safe Disk-Release Plan

## 1. DSH Cleanup Audit Results

### Paths Cleared:
- **DSH sessions:** `/Users/playom/.dsh/sessions/` - contained worktrees but NO active checkpoints required for #12871
- **DSH cache:** No cache directory found (already clean)
- **Estimated space reclaimed:** Minimal (worktrees were already present)

### Protected Artifacts Verification:
- ✅ **Backups verified:** Both SSA backups match SHA256 checksums
- ✅ **Lane worktree intact:** `/Users/playom/t27/.wt/lane-t27b-mdl-01` (933M) preserved
- ✅ **Stash list intact:** 6 staves preserved in lane worktree
- ✅ **No required sessions deleted:** DSH sessions were worktree directories, not critical checkpoints
- ✅ **Git history intact:** All commits and branches preserved

### Current Free Space:
- **Root filesystem:** 2.9Gi available (80% capacity)
- **Target for live smoke test:** Need 10-20Gi buffer

## 2. Cleanup Candidates (Dry-Run Analysis)

| PATH | SIZE | SAFE_TO_REMOVE | RECLAIMED_SPACE | RISK |
|------|------|----------------|-----------------|------|
| `/Users/playom/deepseek-harness/node_modules` | 2.0G | ❌ PROTECTED | N/A | High - needed for circuit-breaker |
| `/Users/playom/t27/.wt/lane-t27b-mdl-01` | 933M | ❌ PROTECTED | N/A | High - #12871 preserved work |
| `/Users/playom/t27/.wt/fpga-petal-home` | 784M | ⚠️ UNUSED | 784M | Medium - old FPGA work |
| `/Users/playom/t27/.wt/lane-ml-parity` | 798M | ⚠️ UNUSED | 798M | Medium - parity testing |
| `/Users/playom/t27/.wt/lane-9065` | 581M | ⚠️ UNUSED | 581M | Medium - completed work |
| `/Users/playom/Library/Caches/Comet` | 580M | ✅ SAFE | 580M | Low - system cache |
| `/Users/playom/Library/Caches/@opencodedesktop-updater` | 557M | ✅ SAFE | 557M | Low - system cache |
| `/Users/playom/Library/Caches/BrowserClaw` | 477M | ✅ SAFE | 477M | Low - browser cache |
| `/Users/playom/Library/Caches/pnpm` | 409M | ✅ SAFE | 409M | Low - package cache |

## 3. Minimum Space Requirements

- **Current free:** 2.9Gi
- **Target free for smoke test:** 10-20Gi
- **Additional needed:** 7.1-17.1Gi

## 4. Safe Cleanup Plan (Low Risk)

### Immediate Safe Removals:
```bash
rm -rf ~/Library/Caches/Comet              # 580M - safe
rm -rf ~/Library/Caches/@opencodedesktop-updater # 557M - safe  
rm -rf ~/Library/Caches/BrowserClaw        # 477M - safe
rm -rf ~/Library/Caches/pnpm               # 409M - safe
```
**Total safe cleanup:** 2.023Gi

### Conditional Removals (Need approval):
```bash
rm -rf /Users/playom/t27/.wt/fpga-petal-home    # 784M - old FPGA work
rm -rf /Users/playom/t27/.wt/lane-ml-parity    # 798M - parity testing
rm -rf /Users/playom/t27/.wt/lane-9065         # 581M - completed work
```
**Total conditional cleanup:** 2.163Gi

## 5. Live Smoke Test Command

```bash
# After cleanup, minimal smoke test:
curl -s -X POST http://127.0.0.1:3080/api/remote.mux \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"session/create","params":{"model":"test-model","reason":"smoke-test"}}' | jq .

# Expected acceptance:
# SUCCESS: Clean response OR FALLBACK_EXHAUSTED with preserved checkpoint
# FAILURE: Infinite retries, repeated 503, lost checkpoint
```

## 6. Acceptance Criteria

After cleanup and smoke test:
- ✅ Free space >= 10Gi
- ✅ Protected artifacts verified
- ✅ One bounded model turn completed
- ✅ Either success on funded pool OR clean FALLBACK_EXHAUSTED
- ✅ No repeated 503 unfunded errors

## 7. Current Status

```
CLEANUP_AUDIT: ✅ COMPLETED
PROTECTED_ARTIFACTS: ✅ VERIFIED
SAFE_CLEANUP_AVAILABLE: ✅ 2.023Gi IMMEDIATE
CONDITIONAL_CLEANUP_AVAILABLE: ⚠️ 2.163Gi NEEDS_APPROVAL
LIVE_SMOKE_PREPARED: ✅ COMMAND_DEFINED
MODEL_ROUTE: PENDING_LIVE_VERIFICATION
EXECUTOR_LAUNCH: BLOCKED
```

**Recommendation:** Proceed with immediate safe cleanup (2.023Gi), then evaluate if conditional cleanup is needed for full smoke test buffer.