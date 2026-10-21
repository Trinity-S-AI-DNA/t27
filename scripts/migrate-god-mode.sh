#!/bin/bash

# Migration script for GOD Mode Single Source of Truth
# This script helps migrate existing code to use the new unified GOD MODE state system

set -e

echo "🔥 Starting GOD MODE Single Source of Truth Migration"
echo "===================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "CLAUDE.md" ]; then
    print_error "Not in t27 repository root. Please run from /Users/playom/t27"
    exit 1
fi

# Create backup directory
BACKUP_DIR="backup-god-mode-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
print_status "Backup directory created: $BACKUP_DIR"

# Step 1: Find all files that reference GOD MODE
echo ""
echo "Step 1: Finding GOD MODE references..."

find . -name "*.t27" -not -path "./.trinity/*" -not -path "./.claude/worktrees/*" -exec grep -l "C_GOD_ON\|C_GOD_OFF\|god.*mode\|GOD.*MODE" {} \; > god-mode-files.txt

if [ ! -s god-mode-files.txt ]; then
    print_warning "No GOD MODE references found in main codebase"
    rm -f god-mode-files.txt
    exit 0
fi

echo "Found $(wc -l < god-mode-files.txt) files with GOD MODE references"

# Step 2: Backup files
echo ""
echo "Step 2: Creating backups..."

while IFS= read -r file; do
    if [ -f "$file" ]; then
        cp "$file" "$BACKUP_DIR/$(basename "$file").backup"
        print_status "Backed up: $file"
    fi
done < god-mode-files.txt

# Step 3: Count references before migration
echo ""
echo "Step 3: Counting existing references..."

TOTAL_REFERENCES=$(grep -r "C_GOD_ON\|C_GOD_OFF" . --include="*.t27" --include="*.rs" | wc -l)
print_status "Total references to migrate: $TOTAL_REFERENCES"

# Step 4: Migrate queen.t27 first
echo ""
echo "Step 4: Migrating queen.t27..."

QUEEN_FILE="specs/t27os/queen.t27"
if [ -f "$QUEEN_FILE" ]; then
    # Create a migration-specific version
    cp "$QUEEN_FILE" "$QUEEN_FILE.migrating"

    # Add import at the top after existing imports
    if grep -q "use t27os::windows" "$QUEEN_FILE"; then
        # Insert after windows import
        sed -i '/use t27os::windows/a\\nuse t27os::god_mode_single_source::*;' "$QUEEN_FILE.migrating"
    else
        # Add at the top of the import section
        sed -i '/module T27osQueen;/a\\nuse t27os::god_mode_single_source::*;' "$QUEEN_FILE.migrating"
    fi

    # Replace old constants
    sed -i 's/pub const C_GOD_ON : u32 = 11;/pub const GOD_MODE_TOGGLE : u32 = 11;  // Was C_GOD_ON/' "$QUEEN_FILE.migrating"
    sed -i 's/pub const C_GOD_OFF : u32 = 12;/pub const GOD_MODE_FORCE_OFF : u32 = 12;  // Was C_GOD_OFF/' "$QUEEN_FILE.migrating"

    # Replace glow_on function signature
    sed -i 's/pub fn glow_on(god: bool) -> bool {/pub fn glow_on(state: GodModeState) -> bool {/' "$QUEEN_FILE.migrating"

    # Update glow_on function body
    sed -i 's/return god;/return state == GodModeState.ON;/' "$QUEEN_FILE.migrating"

    print_status "Created migration version of queen.t27"
else
    print_warning "queen.t27 not found, skipping"
fi

# Step 5: Update control names string
echo ""
echo "Step 5: Updating control names..."

CONTROL_NAMES_FILE="$QUEEN_FILE.migrating"
if [ -f "$CONTROL_NAMES_FILE" ]; then
    # Update the CONTROL_NAMES string to reflect new constants
    sed -i 's/GOD MODE on,GOD MODE off,/GOD MODE toggle,GOD MODE force off,/' "$CONTROL_NAMES_FILE"
    print_status "Updated control names in queen.t27"
fi

# Step 6: Generate migration report
echo ""
echo "Step 6: Generating migration report..."

cat > god-mode-migration-report.txt << EOF
GOD MODE Single Source of Truth Migration Report
==============================================

Date: $(date)
Branch: $(git branch --show-current)

Files Modified:
---------------

1. queen.t27 - Main T27OS queen panel
   - Added import for god_mode_single_source
   - Updated constants (C_GOD_ON → GOD_MODE_TOGGLE)
   - Updated glow_on function to use GodModeState enum
   - Updated control names string

Summary:
--------

- Total files with GOD MODE references: $(wc -l < god-mode-files.txt)
- Total references to migrate: $TOTAL_REFERENCES
- Backup location: $BACKUP_DIR

Next Steps:
-----------

1. Review the changes in $QUEEN_FILE.migrating
2. Run tests: ./scripts/tri test
3. If tests pass, replace original files
4. Update any other affected files (see list above)
5. Remove old boolean-based GOD MODE logic

Files needing manual review:
--------------------------$(while IFS= read -r file; do
    if [ -f "$file" ] && [ "$file" != "$QUEEN_FILE" ]; then
        echo "  - $file"
    fi
done < god-mode-files.txt)

EOF

print_status "Migration report generated: god-mode-migration-report.txt"

# Step 7: Cleanup
echo ""
echo "Step 7: Cleaning up temporary files..."

rm -f god-mode-files.txt

print_status "Migration preparation complete!"
echo ""
echo "📋 Next steps:"
echo "1. Review queen.t27.migrating"
echo "2. Run tests to verify changes"
echo "3. If tests pass, rename .migrating files to final versions"
echo "4. Manually update other affected files"
echo ""
echo "📖 See god-mode-integration-guide.md for detailed instructions"

# Show git status if in a git repo
if git rev-parse --is-inside-work-tree > /dev/null 2>&1; then
    echo ""
    echo "🔍 Git status:"
    git status --porcelain | grep -E "(queen\.t27|migrated|backup)" || echo "No tracked changes yet"
fi