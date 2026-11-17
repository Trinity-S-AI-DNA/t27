#!/bin/bash
# Doctor Cleanup Script - SAFE disk space reclamation for T27 repository
# USAGE: Run this script INTERACTIVELY to approve each cleanup step

set -euo pipefail

echo "=== DOCTOR CLEANUP SCRIPT ==="
echo "Timestamp: $(date)"
echo "Disk usage: $(df -h . | tail -1)"
echo ""
echo "⚠️  READ CAREFULLY: This script will PERMANENTLY DELETE files!"
echo "⚠️  PROTECTED items are SAFE: backups, stashes, active worktree, specific commit"
echo ""

# Track total reclaimed space
TOTAL_RECLAIMED=0

# Function to get confirmation
confirm_deletion() {
    local path="$1"
    local size="$2"
    local reason="$3"
    
    echo ""
    echo "=== DELETE CANDIDATE ==="
    echo "Path: $path"
    echo "Size: $size"
    echo "Reason: $reason"
    echo ""
    
    while true; do
        read -p "Delete this item? [y/N/a(ll)/s(kip)]: " choice
        case $choice in
            [yY]) return 0 ;;
            [nN]) return 1 ;;
            [aA]) echo "Auto-approving remaining deletions"; return 0 ;;
            [sS]) echo "Skipping this item"; return 1 ;;
            *) echo "Please choose y, n, a, or s" ;;
        esac
    done
}

# Cleanup 1: Rust build directories (excluding active worktree)
echo -e "\n=== CLEANUP STEP 1: Rust build directories ==="
rust_targets=()
rust_sizes=()
rust_reasons=()

# Main project target (6.3G)
rust_targets+=("/Users/playom/t27/target")
rust_sizes+=("6.3G")
rust_reasons+=("Main project build artifacts - can be rebuilt")

# Agent worktree target (227M) 
rust_targets+=("/Users/playom/t27/.agents/worktrees/t27f-mega-map-11038/target")
rust_sizes+=("227M")
rust_reasons+=("Completed agent worktree build artifacts")

# Claude worktree target (460M)
rust_targets+=("/Users/playom/t27/.claude/worktrees/m1-alternates/target")
rust_sizes+=("460M")
rust_reasons+=("Completed worktree build artifacts")

# Backend target (143M)
rust_targets+=("/Users/playom/t27/backend/trinity-core/target")
rust_sizes+=("143M")
rust_reasons+=("Backend build artifacts")

# Solana mining target (362M)
rust_targets+=("/Users/playom/t27/contrib/solana/programs/tri-mining/target")
rust_sizes+=("362M")
rust_reasons+=("Solana program build artifacts")

# Active worktree target (587M) - KEEP THIS
echo "🛡️  SKIPPING: /Users/playom/t27/.wt/lane-t27b-mdl-01/target (587M)"
echo "   This is part of your ACTIVE worktree - PROTECTED"

for i in "${!rust_targets[@]}"; do
    if confirm_deletion "${rust_targets[$i]}" "${rust_sizes[$i]}" "${rust_reasons[$i]}"; then
        echo "🗑️  DELETING: ${rust_targets[$i]}"
        rm -rf "${rust_targets[$i]}"
        echo "✅ DELETED: ${rust_targets[$i]}"
        TOTAL_RECLAIMED=$((TOTAL_RECLAIMED + $(numfmt --from=iec ${rust_sizes[$i]})))
    else
        echo "⏭️  SKIPPED: ${rust_targets[$i]}"
    fi
done

# Cleanup 2: Large inactive worktrees
echo -e "\n=== CLEANUP STEP 2: Inactive worktrees ==="
worktrees=(
    "/Users/playom/t27/.wt/fpga-petal-home:784M:Completed FPGA worktree, branch may exist"
    "/Users/playom/t27/.wt/lane-ml-parity:798M:ML parity worktree, large but completed"
    "/Users/playom/t27/.wt/lane-ml-parity2:674M:Second ML parity worktree"
    "/Users/playom/t27/.wt/lane-9065:581M:Legacy lane worktree"
    "/Users/playom/t27/.wt/board3-claim-refs:403M:Board claim refs worktree"
    "/Users/playom/t27/.wt/lane-actor-march:403M:Actor march worktree"
    "/Users/playom/t27/.wt/lane-blog-t27b-29:344M:Blog worktree"
    "/Users/playom/t27/.wt/lane-12413:403M:Legacy worktree"
    "/Users/playom/t27/.wt/scratch-golden-test:346M:Scratch/test worktree"
)

for wt_entry in "${worktrees[@]}"; do
    wt_path=$(echo "$wt_entry" | cut -d: -f1)
    wt_size=$(echo "$wt_entry" | cut -d: -f2)
    wt_reason=$(echo "$wt_entry" | cut -d: -f3-)
    
    # Double-check if branch still exists
    branch_name=$(basename "$wt_path")
    if git show --quiet refs/heads/"$branch_name" >/dev/null 2>&1; then
        echo "⚠️  BRANCH STILL EXISTS: $branch_name - MANUAL REVIEW REQUIRED"
        continue
    fi
    
    if confirm_deletion "$wt_path" "$wt_size" "$wt_reason"; then
        echo "🗑️  DELETING worktree: $wt_path"
        cd "$wt_path"
        git worktree prune
        cd - >/dev/null
        rm -rf "$wt_path"
        echo "✅ DELETED: $wt_path"
        TOTAL_RECLAIMED=$((TOTAL_RECLAIMED + $(numfmt --from=iec ${wt_size})))
    else
        echo "⏭️  SKIPPED: $wt_path"
    fi
done

# Cleanup 3: Old bundle files (keep the backup)
echo -e "\n=== CLEANUP STEP 3: Old bundle files ==="
bundle_candidates=(
    "/Users/playom/t27/t27-preservation.bundle:183M:Old preservation bundle, backup exists"
)

for bundle_entry in "${bundle_candidates[@]}"; do
    bundle_path=$(echo "$bundle_entry" | cut -d: -f1)
    bundle_size=$(echo "$bundle_entry" | cut -d: -f2)
    bundle_reason=$(echo "$bundle_entry" | cut -d: -f3-)
    
    if confirm_deletion "$bundle_path" "$bundle_size" "$bundle_reason"; then
        echo "🗑️  DELETING bundle: $bundle_path"
        rm -f "$bundle_path"
        echo "✅ DELETED: $bundle_path"
        TOTAL_RECLAIMED=$((TOTAL_RECLAIMED + $(numfmt --from=iec ${bundle_size})))
    else
        echo "⏭️  SKIPPED: $bundle_path"
    fi
done

# Final verification
echo -e "\n=== FINAL VERIFICATION ==="
echo "Total reclaimed space: $(numfmt --to=iec $TOTAL_RECLAIMED)"
echo ""
echo "Checking protected items:"

# Verify backups exist
if [ -d "/Users/playom/t27/backups" ]; then
    echo "✅ Backups directory exists"
else
    echo "❌ ERROR: Backups directory missing!"
fi

# Verify active worktree
if [ -d "/Users/playom/t27/.wt/lane-t27b-mdl-01" ]; then
    echo "✅ Active worktree exists"
else
    echo "❌ ERROR: Active worktree missing!"
fi

# Verify stashes
stash_count=$(git stash list | wc -l)
if [ "$stash_count" -gt 0 ]; then
    echo "✅ $stash_count stashes preserved"
else
    echo "❌ ERROR: No stashes found!"
fi

# Verify protected commit
if git show --quiet 0c7ea239b >/dev/null 2>&1; then
    echo "✅ Protected commit 0c7ea239b exists"
else
    echo "❌ ERROR: Protected commit missing!"
fi

echo ""
echo "=== CLEANUP COMPLETE ==="
echo "Final disk usage: $(df -h . | tail -1)"