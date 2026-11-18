# T27OS + DSH Integration Complete Plan

## Diagnosis Confirmed ✅

**DSH Schedule System**: Functional reminder system, not general automation  
**Empty UI**: Expected behavior (no reminders created)  
**Missing Layer**: T27OS Queen bridge between incidents and reminders

## Manual Smoke Test Plan

### Test Reminder Creation

#### Option 1: DSH Web UI (Recommended)
1. **Navigate**: http://127.0.0.1:3080 → Automation Tasks
2. **Create New Reminder**:
   ```
   Name: T27OS Queen integration smoke test
   Trigger: Manual
   Time: 1 minute from now
   Prompt: Check T27OS Queen task "doctor-mvp-duplicate-directory-regression". If incomplete, summarize evidence and create follow-up task. Do not retry providers. Do not modify files.
   ```
3. **Verify**: Appears in list, persists, can be executed

#### Option 2: Agent Task
```
Create reminder named "T27OS Queen integration smoke test" that triggers in 1 minute to check Queen task status and report findings.
```

#### Option 3: Direct API
```bash
curl -X POST http://localhost:3080/api/schedule/create \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "T27OS Queen integration smoke test",
    "prompt": "Check T27OS Queen task \"doctor-mvp-duplicate-directory-regression\". If incomplete, summarize evidence and create follow-up task. Do not retry providers. Do not modify files.",
    "after_seconds": 60
  }'
```

### Success Criteria

```text
DSH_SCHEDULE_SMOKE_TEST_PASS WHEN:
- ✅ reminder appears in Automation Tasks UI;
- ✅ reminder persists across page refresh/restart;
- ✅ reminder can be triggered manually;
- ✅ run history is visible after execution;
- ✅ reminder can be edited or cancelled.
```

## Bridge Implementation Architecture

### Current State
```
Doctor Incidents
  ↓ (NO BRIDGE)
[???]
  ↓ (NO BRIDGE) 
T27OS Queen Tasks
  ↓ (NO BRIDGE)
[???]
  ↓ (NO BRIDGE)
DSH Schedule Reminders
  ↓ (WORKING)
DSH UI Display
```

### Target Architecture
```
Doctor Incidents
  ↓ (Bridge 1: incident_to_queen_task)
T27OS Queen Tasks  
  ↓ (Bridge 2: queen_task_to_dsh_reminder)
DSH Schedule Reminders
  ↓ (Bridge 3: reminder_result_to_queen_task)
DSH UI Display
```

## Bridge Implementation Details

### Bridge 1: Incident → Queen Task
```typescript
interface IncidentToQueenTaskBridge {
  parseIncident(incident: DoctorIncident): QueenTaskInput;
  createQueenTask(input: QueenTaskInput): QueenTask;
  storeTask(task: QueenTask): Promise<void>;
}
```

**Purpose**: Convert incidents to persistent Queen tasks without provider calls

### Bridge 2: Queen Task → DSH Reminder  
```typescript
interface QueenTaskToReminderBridge {
  shouldCreateReminder(task: QueenTask): boolean;
  createReminder(task: QueenTask): DSHReminderInput;
  linkTaskToReminder(taskId: TaskId, reminderId: ReminderId): void;
}
```

**Purpose**: Optionally create DSH reminders for Queen tasks

### Bridge 3: Reminder Result → Queen Task
```typescript
interface ReminderResultToQueenTaskBridge {
  handleReminderExecution(reminderId: ReminderId, result: ReminderResult): void;
  appendEvidence(taskId: TaskId, evidence: EvidenceBundle): void;
  updateTaskStatus(taskId: TaskId, status: TaskStatus): void;
}
```

**Purpose**: Update Queen tasks from reminder execution results

## Implementation Roadmap

### Phase 1: DSH Verification (Manual)
- [ ] Create manual test reminder
- [ ] Verify UI appearance and persistence
- [ ] Test execution and run history
- [ ] Document working system baseline

### Phase 2: T27OS Queen Foundation
- [ ] Design Queen task data model
- [ ] Implement persistent storage
- [ ] Build task lifecycle management
- [ ] Create task state and evidence system

### Phase 3: Bridge Implementation
- [ ] Bridge 1: Incident → Queen task conversion
- [ ] Bridge 2: Queen task → DSH reminder creation
- [ ] Bridge 3: Result → Queen task updates
- [ ] Handle failures and follow-up tasks

### Phase 4: Integration Testing
- [ ] End-to-end Doctor → Queen → DSH workflow
- [ ] UI visibility and status sync
- [ ] Error handling and recovery
- [ ] Performance optimization

## Definition of Done

### Complete Integration When:
```text
DOCTOR_AUTOMATION_INTEGRATION_COMPLETE WHEN:
1. ✅ DSH reminder smoke test passes.
2. ✅ Doctor incident creates persistent Queen task.
3. ✅ Queen task can create linked DSH reminder.
4. ✅ Reminder delivery is visible in DSH UI.
5. ✅ Reminder result updates Queen evidence.
6. ✅ Provider-unfunded failure creates follow-up Queen task.
7. ✅ No silent fallback or automatic provider retry.
8. ✅ Full end-to-end workflow tested.
```

## System Boundaries Confirmed

```text
T27OS Queen = task/state/evidence system
DSH Schedule = reminder/trigger system  
DSH UI = visibility for schedule records

NO OVERLAP: Clear separation of concerns
BRIDGES: Explicit connections between systems
```

## Next Steps

1. **Immediate**: Manual DSH reminder smoke test
2. **Short-term**: Implement Bridge 1 (Incident → Queen Task)
3. **Medium-term**: Implement Bridges 2 & 3
4. **Long-term**: End-to-end integration and testing

This plan establishes the clear path from working DSH system to complete T27OS + DSH integration. The manual verification step is crucial to confirm the baseline before building integration bridges.