# TRI CLI - Doctor Integration Commands

## New CLI Commands Added

### Core Doctor Commands
```bash
# Incident detection and classification
tri doctor scan --input fixtures/
tri doctor triage --incident INC-123 --rules docs/triage.rules.md

# Repair task creation with worktree isolation
tri doctor handoff --incident INC-123 --dry-run      # Safe preview
tri doctor handoff --incident INC-123                # DSH task creation (future)
tri doctor verify --incident INC-123 --gate TEST_FAILURE

# System health and worktree checks
tri doctor health                                  # General health check
tri doctor health --verify-worktree                # Worktree isolation check
```

### Pareto Improvement Loop
```bash
# Issue management and prioritization
tri pareto rank                                     # Rank available issues by Pareto score
tri pareto select                                  # Select highest priority issue
tri pareto claim --issue 11038                     # Claim GitHub issue for work

# Worktree management
tri worktree create --issue 11038                 # Create worktree for issue
tri worktree verify                                # Verify worktree isolation
tri worktree cleanup --merged                      # Clean up merged worktrees

# Skills and experience management
tri skills save --topic "worktree_isolation" --insight "Always verify worktree before task creation"
tri skills search --query "worktree isolation"
```

### Loop Management
```bash
# Continuous improvement loop
tri loop health                                    # Check loop health status
tri loop next                                      # Get next recommended task
tri loop report                                    # Generate improvement report

# Quality assurance
tri anomaly scan                                  # Detect potential issues
tri coverage gap                                  # Find uncovered risk areas
tri pr prepare --issue 11038                       # Prepare PR for claimed issue
tri pr merge-guard --pr 123                       # Verify PR meets merge criteria
```

## Worktree-First Integration

### Pre-Flight Checks
```bash
# Automatic worktree verification before handoff
tri doctor handoff --incident INC-123
```

**Output if violations exist:**
```
❌ WORKTREE ISOLATION VIOLATIONS:
  - Not in approved worktree
  - Branch 'main' doesn't match agent/ prefix
  - Unmodified files in primary checkout

Task creation BLOCKED. Verify worktree isolation rules.
```

**Output if clean:**
```
✅ WORKTREE ISOLATION: All checks passed
🔍 DRY-RUN: Generating repair brief for incident INC-123
Task preview generated. No DSH task created.
```

### Health Monitoring
```bash
tri doctor health --verify-worktree
```

**Sample Output:**
```
✅ WORKTREE ISOLATION: All checks passed

🏥 DOCTOR MVP HEALTH CHECK:
  Current directory: /Users/playom/t27/contrib/doctor-mvp
  Python version: 3.9.6
  Worktree isolation: PASS
```

## Claim Workflow Integration

### Issue Comment Format
```text
TASK CLAIMED: Fix Doctor MVP worktree isolation
OWNER: doctor-architect
WORKTREE: ../wt-11038-worktree-fix
BRANCH: agent/11038-worktree-fix
BASE: origin/main
STATUS: IN_PROGRESS
SCOPE: .t27 file, CLI pre-flight checks
ALREADY_DONE:
- Created .t27 worktree isolation rules
- Added verify_worktree_isolation() function
IN_PROGRESS:
- Update CLI handoff command to check isolation
- Add dry-run option
NEXT:
- Test worktree verification
- Document new TRI CLI commands
BLOCKERS:
- None
ESTIMATED_EFFORT: S
PARETO_SCORE: 8.5
```

### Worktree Creation Commands
```bash
# Create worktree for claimed issue
git fetch origin
git switch main
git pull --ff-only
git worktree add ../wt-11038-worktree-fix \
  -b agent/11038-worktree-fix origin/main
cd ../wt-11038-worktree-fix

# Work in the isolated environment
# All changes are isolated from primary checkout
```

## Doctor-Pareto Integration Points

### Task Claim Verification
```bash
tri pareto claim --issue 11038
```

**Automatically:**
1. Creates worktree: `../wt-11038-worktree-fix`
2. Creates branch: `agent/11038-worktree-fix`
3. Updates GitHub issue with claim comment
4. Records worktree location in task metadata

### Pre-Flight for Repair Tasks
```bash
tri doctor handoff --incident INC-123 --dry-run
```

**Checks performed:**
- Worktree isolation verification
- Issue claim validation
- Base branch synchronization
- Unrelated files check
- Branch name validation

## Safety Guarantees

### Primary Repository Protection
- Primary checkout remains read-only
- All modifications in isolated worktrees
- No direct commits to shared branches
- Force-push prevention

### Issue Isolation
- One worktree per GitHub issue
- Clear branch naming convention
- Ownership tracking through claim comments
- No shared uncommitted changes

### Merge Safety
- Conflict-free PR requirement
- Green CI verification
- Diff scope matching issue claim
- Protected path enforcement

## Integration Benefits

### 1. Safety
- Worktree isolation prevents repository corruption
- Issue separation prevents conflicts
- Merge guards ensure clean history

### 2. Visibility
- Claim comments show progress
- Health checks provide status
- Worktree tracking shows agent activity

### 3. Efficiency
- Automated worktree creation
- Pre-flight checks prevent errors
- Pareto scoring optimizes task selection

### 4. Collaboration
- Clear ownership of issues
- Isolated work prevent interference
- PR-based workflow with proper review

---
**Integration Status**: ✅ WORKTREE-FIRST + DOCTOR CLI + PARETO LOOP READY