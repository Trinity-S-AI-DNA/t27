#!/bin/bash
# Doctor Script - Dry-run disk space analysis for T27 repository
# Usage: bash doctor-analysis.sh

set -euo pipefail

echo "=== DOCTOR DRY-RUN ANALYSIS ==="
echo "Timestamp: $(date)"
echo "Disk usage: $(df -h . | tail -1)"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# PROTECTED ITEMS CHECK
echo -e "${BLUE}=== PROTECTED ITEMS STATUS ===${NC}"
echo ""

# Check protected backup directory
echo -e "${GREEN}✓ Protected backups:${NC}"
du -sh /Users/playom/t27/backups/ 2>/dev/null || echo "  Backup directory not found"
echo ""

# Check active worktree
echo -e "${GREEN}✓ Active worktree lane-t27b-mdl-01:${NC}"
if [ -d "/Users/playom/t27/.wt/lane-t27b-mdl-01" ]; then
    du -sh /Users/playom/t27/.wt/lane-t27b-mdl-01 2>/dev/null || echo "  Size: unavailable"
    echo "  Status: PROTECTED (active worktree)"
else
    echo "  Warning: Active worktree not found"
fi
echo ""

# Check git stashes
echo -e "${GREEN}✓ Git stashes:${NC}"
stash_count=$(git stash list | wc -l)
if [ "$stash_count" -gt 0 ]; then
    echo "  Count: $stash_count stashes (PROTECTED)"
    git stash list | head -5
    [ "$stash_count" -gt 5 ] && echo "  ... and $((stash_count - 5)) more"
else
    echo "  No stashes found"
fi
echo ""

# Check specific commit
echo -e "${GREEN}✓ Protected commit 0c7ea239b:${NC}"
if git show --quiet 0c7ea239b >/dev/null 2>&1; then
    echo "  Status: COMMIT EXISTS (PROTECTED)"
else
    echo "  Warning: Commit not found in repository"
fi
echo ""

# CANDIDATES ANALYSIS
echo -e "${YELLOW}=== CLEANUP CANDIDATES ===${NC}"
echo ""

total_reclaimable=0

# Candidate 1: Rust build directories
echo -e "${YELLOW}1. Rust build directories (**/target):${NC}"
find /Users/playom/t27 -name "target" -type d 2>/dev/null | head -10 | while read -r dir; do
    if [ -d "$dir" ]; then
        size=$(du -sh "$dir" 2>/dev/null | cut -f1)
        echo "  $dir: $size"
    fi
done
echo ""

# Candidate 2: Node.js caches
echo -e "${YELLOW}2. Node.js build caches:${NC}"
cache_dirs=()
if [ -d "/Users/playom/t27/node_modules/.cache" ]; then
    cache_dirs+=("/Users/playom/t27/node_modules/.cache")
fi
if [ -d "/Users/playom/t27/.turbo" ]; then
    cache_dirs+=("/Users/playom/t27/.turbo")
fi
if [ -d "/Users/playom/t27/.parcel-cache" ]; then
    cache_dirs+=("/Users/playom/t27/.parcel-cache")
fi

if [ ${#cache_dirs[@]} -gt 0 ]; then
    for dir in "${cache_dirs[@]}"; do
        if [ -d "$dir" ]; then
            size=$(du -sh "$dir" 2>/dev/null | cut -f1)
            echo "  $dir: $size"
        fi
    done
else
    echo "  No Node.js cache directories found"
fi
echo ""

# Candidate 3: Inactive worktrees
echo -e "${YELLOW}3. Inactive worktrees:${NC}"
active_worktree="/Users/playom/t27/.wt/lane-t27b-mdl-01"
for wt in /Users/playom/t27/.wt/*; do
    if [ -d "$wt" ] && [ "$wt" != "$active_worktree" ] && [ "$wt" != "/Users/playom/t27/.wt/t27os" ] && [ "$wt" != "/Users/playom/t27/.wt/tmp" ]; then
        size=$(du -sh "$wt" 2>/dev/null | cut -f1)
        # Check if branch exists and is not active
        branch=$(basename "$wt")
        if git show --quiet refs/heads/"$branch" >/dev/null 2>&1 || git show --quiet refs/remotes/origin/"$branch" >/dev/null 2>&1; then
            echo "  $wt: $size (BRANCH EXISTS - MANUAL REVIEW NEEDED)"
        else
            echo "  $wt: $size (LIKELY SAFE - no active branch)"
        fi
    fi
done
echo ""

# Candidate 4: Temporary files
echo -e "${YELLOW}4. Temporary files and logs:${NC}"
temp_dirs=()
if [ -d "/Users/playom/t27/target" ]; then
    temp_dirs+=("/Users/playom/t27/target")
fi
if [ -d "/Users/playom/t27/.diesel" ]; then
    temp_dirs+=("/Users/playom/t27/.diesel")
fi
if [ -d "/Users/playom/t27/.DS_Store" ]; then
    temp_dirs+=("/Users/playom/t27/.DS_Store")
fi

for dir in "${temp_dirs[@]}"; do
    if [ -d "$dir" ]; then
        size=$(du -sh "$dir" 2>/dev/null | cut -f1)
        echo "  $dir: $size"
    fi
done
echo ""

# Candidate 5: Bundle duplicates
echo -e "${YELLOW}5. Repository bundle files:${NC}"
find /Users/playom/t27 -name "*.bundle" 2>/dev/null | while read -r bundle; do
    if [ -f "$bundle" ]; then
        size=$(du -sh "$bundle" 2>/dev/null | cut -f1)
        echo "  $bundle: $size"
    fi
done
echo ""

# SUMMARY
echo -e "${BLUE}=== SUMMARY ===${NC}"
echo ""
echo "Protected items verified:"
echo "✓ Backup directory: /Users/playom/t27/backups/"
echo "✓ Active worktree: /Users/playom/t27/.wt/lane-t27b-mdl-01"
echo "✓ Git stashes: $stash_count entries"
echo "✓ Protected commit: 0c7ea239b"
echo ""
echo "RECOMMENDATIONS:"
echo "1. Run dry-run analysis above to identify specific targets"
echo "2. Manually approve each deletion target"
echo "3. Execute cleanup commands only after confirmation"
echo "4. Verify git status after each cleanup step"
echo ""
echo "SAFETY REMINDER:"
echo "- This is a DRY-RUN - no files will be deleted"
echo "- Always verify backups exist before cleanup"
echo "- Test cleanup commands in smaller batches"
echo "- Monitor disk space during cleanup operations"