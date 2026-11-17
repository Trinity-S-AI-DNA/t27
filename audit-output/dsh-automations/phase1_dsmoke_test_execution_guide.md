# Phase 1: DSH Smoke Test Execution Guide

## Current Phase Status
```text
PHASE: DSH_BASELINE_SMOKE_TEST
STATUS: PENDING
BLOCKS: T27OS_QUEEN_FOUNDATION
NEXT_ACTION: Manual reminder creation and verification
```

## Objective
Establish working DSH Schedule system baseline by creating and verifying one manual reminder.

## Smoke Test Implementation

### Step 1: Create Reminder via DSH Web UI

#### Instructions
1. **Open DSH Web Interface**
   - URL: http://127.0.0.1:3080
   - Ensure authenticated and in correct workspace

2. **Navigate to Automation Tasks**
   - Find "Automation" or "Schedule" section
   - Click "Automation Tasks" or equivalent

3. **Create New Reminder**
   - Click "Create", "New", "+" or equivalent button
   - Fill form with:
     ```
     Name: T27OS Queen integration smoke test
     Description: Verify DSH automation system functionality
     Trigger: Manual
     Time: 1 minute from now
     Prompt: Check T27OS Queen task "doctor-mvp-duplicate-directory-regression". If incomplete, summarize evidence and create follow-up task. Do not retry providers. Do not modify files.
     ```

4. **Save Reminder**
   - Click "Create", "Save", or "Schedule"
   - Note exact creation time and settings

### Step 2: Verification Checklist

#### UI Appearance Verification
- [ ] Reminder appears in "Automation Tasks" list
- [ ] Name displays correctly
- [ ] Status shows as "active" or equivalent
- [ ] Timing shows next execution time
- [ ] Actions available (Edit, Execute, Delete)

#### Persistence Verification  
- [ ] Reminder persists after page refresh
- [ ] Reminder persists after browser restart
- [ ] Reminder appears in new session

#### Functionality Verification
- [ ] Reminder can be executed manually (if UI supports)
- [ ] Reminder can be edited
- [ ] Reminder can be cancelled/deleted
- [ ] Run history visible after execution

#### Delivery Verification
- [ ] Reminder reaches intended session
- [ ] Prompt content delivered correctly
- [ ] Execution results captured in run history

## Pass Criteria Definition

```text
DSH_BASELINE_PASS WHEN:
1. ✅ Reminder appears in Automation Tasks UI.
2. ✅ Reminder persists after session restart.
3. ✅ Reminder delivers to intended session.
4. ✅ Reminder can be edited or cancelled.
5. ✅ No provider request is required for reminder creation.
```

## Documentation Requirements

### After Successful Creation
1. **Screenshot**: Capture reminder in list view
2. **Timing Log**: Record creation time and next execution time
3. **Behavior Notes**: Document how it behaves (persistence, execution, etc.)
4. **UI Description**: Note exact UI elements and their state

### If Creation Fails
1. **Error Logging**: Document any error messages or failures
2. **Troubleshooting**: Note what was attempted and what failed
3. **Prerequisites**: Check authentication, permissions, system status

## Sequential Gate Logic

### Phase 1 Gate Condition
```text
CAN_PROCEED_TO_PHASE_2 WHEN:
- DSH_BASELINE_PASS criteria all met
- Reminder creation verified working
- Persistence and delivery confirmed
- No blocking issues identified
```

### Blocking Dependencies
```text
BLOCKS_T27OS_FOUNDATION: 
- DSH_BASELINE_PASS not met
- Reminder creation fails
- Persistence not working
- Delivery system broken
```

## Next Steps After Success

1. **Update Status**:
   ```text
   PHASE: DSH_BASELINE_SMOKE_TEST
   STATUS: COMPLETE ✅
   BLOCKS: REMOVED
   NEXT_PHASE: T27OS_QUEEN_FOUNDATION
   ```

2. **Proceed to Phase 2**:
   - Design Queen task data model
   - Implement persistent storage
   - Build GitHub Issue mapping
   - Create atomic claiming mechanism

## Critical Notes

- **Sequential Dependency**: Phase 2 cannot start until Phase 1 passes
- **No Code Required**: This is manual verification only
- **Working Baseline**: Confirms DSH system is functional before building on it
- **Documentation**: Results must be clear and repeatable

## Current Focus
**Create and verify one DSH reminder to establish working system baseline before proceeding with T27OS Queen foundation.**