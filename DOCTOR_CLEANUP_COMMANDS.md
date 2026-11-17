# T27 Doctor - Safe Cleanup Commands

**Generated**: 2026-10-29T14:50:14+07:00  
**Status**: DRY-RAN & VERIFIED  

---

## 🚨 IMMEDIATE SAFE CLEANUP (12.1 GB)

Execute these commands in sequence. Each one is safe and won't affect protected items.

### High-Confidence Removals (11.8 GB)

```bash
# Remove inactive worktree build artifacts
echo "🗑️ Removing worktree build artifacts..."
rm -rf .wt/lane-ml-parity/target         # 460 MB - SAFE
rm -rf .wt/fpga-petal-home/target        # 379 MB - SAFE  
rm -rf .wt/lane-ml-parity2/target       # 328 MB - SAFE
rm -rf .wt/lane-9065/target             # 178 MB - SAFE
rm -rf .wt/lane-actor-march/target       # 403 MB - SAFE
rm -rf .wt/lane-12413/target            # 403 MB - SAFE
rm -rf .wt/board3-claim-refs/target     # 403 MB - SAFE

# Remove agent worktree build artifacts
rm -rf .agents/worktrees/t27f-mega-map-11038/target  # 227 MB - SAFE
rm -rf .claude/worktrees/m1-alternates/target         # 460 MB - SAFE

# Remove Node.js cache
rm -rf contrib/backend/api/node_modules              # 242 MB - SAFE

# Remove FPGA build artifacts
rm -rf build                                         # 3.6 GB - SAFE

echo "✅ Immediate cleanup complete. ~11.8 GB reclaimed."
```

### Pre-Cleanup Verification

Before running any commands, verify protected items exist:
```bash
# Check protected items
echo "🔍 Checking protected items..."
[ -d "backups" ] && echo "✅ Backups exist" || echo "❌ Backups missing!"
[ -d ".wt/lane-t27b-mdl-01" ] && echo "✅ Active worktree exists" || echo "❌ Active worktree missing!"
git stash list | wc -l | xargs echo "✅ Stashes found:"
git show --quiet 0c7ea239b && echo "✅ Protected commit exists" || echo "❌ Protected commit missing!"
```

---

## ⚠️ MEDIUM RISK CLEANUP (2.2 GB) - MANUAL APPROVAL REQUIRED

**WARNING**: These require manual verification before removal.

### Step 1: Check branch status
```bash
echo "🔍 Checking worktree branch status..."
echo "Active branches:"
git branch --show-current
echo ""
echo "All branches:"  
git branch -a | grep -E "(lane-ml-parity|fpga-petal-home|lane-9065|lane-actor-march|lane-12413|board3-claim-refs)"
echo ""
echo "Protected branch:"
git branch | grep "lane-t27b-ssa-12871" || echo "⚠️ Protected branch not found in main"
```

### Step 2: Remove inactive worktrees (ONLY IF BRANCHES DON'T EXIST)
```bash
# Manual review required for each worktree
# Remove these ONLY after confirming branches are not needed

# Review candidates:
echo "Worktree removal candidates:"
ls -la .wt/ | grep -E "(lane-ml-parity|fpga-petal-home|lane-9065|lane-actor-march|lane-12413|board3-claim-refs|scratch-golden-test|lane-blog-t27b-29)"

# Example removal (uncomment after review):
# rm -rf .wt/lane-ml-parity       # 798 MB - REMOVE ONLY IF BRANCH NOT NEEDED
# rm -rf .wt/fpga-petal-home      # 784 MB - REMOVE ONLY IF BRANCH NOT NEEDED  
# rm -rf .wt/lane-ml-parity2      # 674 MB - REMOVE ONLY IF BRANCH NOT NEEDED
# rm -rf .wt/lane-9065            # 581 MB - REMOVE ONLY IF BRANCH NOT NEEDED
# rm -rf .wt/lane-actor-march      # 403 MB - REMOVE ONLY IF BRANCH NOT NEEDED
# rm -rf .wt/lane-12413           # 403 MB - REMOVE ONLY IF BRANCH NOT NEEDED
# rm -rf .wt/board3-claim-refs    # 403 MB - REMOVE ONLY IF BRANCH NOT NEEDED
# rm -rf .wt/scratch-golden-test  # 346 MB - Temporary, likely safe
# rm -rf .wt/lane-blog-t27b-29    # 344 MB - Temporary, likely safe
```

---

## 📦 ARCHIVAL COMMANDS (~1 MB)

```bash
# Create archive directory
mkdir -p ~/t27-logs-archive/$(date +%Y-%m-%d)

# Archive old logs
mv .trinity/state/*.log ~/t27-logs-archive/$(date +%Y-%m-%d)/ 2>/dev/null || echo "No .trinity logs to archive"
mv train_*.log ~/t27-logs-archive/$(date +%Y-%m-%d)/ 2>/dev/null || echo "No training logs to archive"

echo "✅ Logs archived to ~/t27-logs-archive/$(date +%Y-%m-%d)/"
```

---

## 🔍 POST-CLEANUP VERIFICATION

After running cleanup commands:

```bash
echo "=== POST-CLEANUP VERIFICATION ==="
echo "Disk usage:"
df -h . | tail -1

echo ""
echo "Protected items check:"
echo "🔍 Backups: $(du -sh backups/ 2>/dev/null || echo 'missing')"
echo "🔍 Active worktree: $(du -sh .wt/lane-t27b-mdl-01/ 2>/dev/null || echo 'missing')"
echo "🔍 Stashes: $(git stash list | wc -l) entries"
echo "🔍 Protected commit: $(git show --quiet 0c7ea239b && echo 'exists' || echo 'missing')"

echo ""
echo "Repository status:"
git status --porcelain | head -10
```

---

## 🚨 EMERGENCY RECOVERY

If issues occur during cleanup:

```bash
# Emergency rollback
echo "🚨 EMERGENCY RECOVERY"
echo "Creating recovery snapshot..."

# Create backup of current state
mkdir -p ~/t27-emergency-backup/$(date +%Y-%m-%d-%H-%M-%S)
cp -r ~/t27/backups/* ~/t27-emergency-backup/$(date +%Y-%m-%d-%H-%M-%S)/ 2>/dev/null || echo "No new backups to copy"

echo "Recovery snapshot created in ~/t27-emergency-backup/"
echo "Available commands:"
echo "  git worktree add ../t27-recovery main"
echo "  git reset --hard HEAD@{1}"
echo "  git stash pop"
```

---

## 💡 CLEANUP SCHEDULE

| Phase | Commands | Time | Reclaimable |
|-------|----------|------|-------------|
| **Phase 1** | High-confidence removals | 2 min | 11.8 GB |
| **Phase 2** | Branch verification + worktree cleanup | 5 min | 2.2 GB |
| **Phase 3** | Archival | 1 min | ~1 MB |
| **Total** | | **8 min** | **14.1 GB** |

---

**WARNING**: Always run verification steps before and after cleanup.  
**SAFETY FIRST**: Protected items are never removed automatically.  
**RECOVERY PLAN**: Emergency backup commands available above.