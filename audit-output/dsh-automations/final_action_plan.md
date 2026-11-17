# Final Action Plan: T27OS + DSH Integration

## Diagnosis Complete ✅

**DSH Schedule System**: Functional reminder/scheduler (NOT general automation)  
**Empty UI**: Expected behavior (no reminders created)  
**Ready for**: T27OS Queen integration with clear architectural boundaries

## Correct Architecture Established

### System Responsibilities
```text
Doctor: detect/classify incident
T27OS Queen: own task lifecycle and evidence  
DSH Schedule: deliver reminders/triggers
DSH UI: show schedule records
```

### Target Flow
```
Doctor Incident
  -> T27OS Queen Task
       persistent status, evidence, owner, retry policy
  -> optional DSH Schedule Reminder  
       time-based trigger / follow-up notification
  -> DSH Automation Tasks UI
       visible reminder record
```

## Immediate Action: Manual Smoke Test

### Test Reminder Creation
```text
TITLE: T27OS Queen integration smoke test
SCHEDULE: once (1 minute from now)
PROMPT:
Check T27OS Queen task "doctor-mvp-duplicate-directory-regression".  
If incomplete, summarize evidence and create follow-up task.
Do not retry providers. Do not modify files.
```

### Pass Criteria
```text
DSH_SCHEDULE_SMOKE_PASS WHEN:
- ✅ reminder appears in Automation Tasks UI;
- ✅ reminder persists across session restart;  
- ✅ reminder delivers to intended session;
- ✅ reminder can be edited or cancelled.
```

## Bridge Implementation Specifications

### Bridge 1: Incident → Queen Task
```typescript
interface IncidentToQueenTaskBridge {
  // Doctor incident creates persistent Queen task
  createTaskFromIncident(incident: DoctorIncident): QueenTask;
  // Store evidence, status, owner without provider calls
  storeTask(task: QueenTask): Promise<void>;
}
```

### Bridge 2: Queen Task → DSH Reminder  
```typescript
interface QueenTaskToReminderBridge {
  // Queen task optionally creates linked reminder
  createReminderForTask(task: QueenTask): DSHReminderInput | null;
  // Reminder stores Queen task ID reference
  linkTaskToReminder(taskId: TaskId, reminderId: ReminderId): void;
}
```

### Bridge 3: Reminder Result → Queen Task
```typescript
interface ReminderResultToQueenTaskBridge {
  // Reminder execution appends evidence
  appendEvidence(taskId: TaskId, evidence: EvidenceBundle): void;
  // Only Queen policy changes task status
  updateTaskStatus(taskId: TaskId, status: TaskStatus): void;
}
```

## Definition of Done

### Complete Integration When:
```text
DOCTOR_AUTOMATION_INTEGRATION_COMPLETE WHEN:
1. ✅ DSH reminder smoke test passes.
2. ✅ Doctor incident creates persistent Queen task.
3. ✅ Queen task creates linked DSH reminder.
4. ✅ Reminder delivery is visible in DSH UI.
5. ✅ Reminder result updates Queen evidence.
6. ✅ PROVIDER_UNFUNDED creates follow-up Queen task.
7. ✅ No automatic provider retry or silent fallback.
8. ✅ End-to-end workflow tested and validated.
```

## Implementation Sequence

### Phase 1: Verification (Manual)
- [ ] Create manual DSH reminder via Web UI
- [ ] Verify UI appearance and persistence
- [ ] Test execution and run history
- [ ] Confirm system baseline works

### Phase 2: T27OS Foundation
- [ ] Design Queen task data model
- [ ] Implement persistent storage
- [ ] Build task lifecycle management
- [ ] Create evidence collection system

### Phase 3: Bridge Implementation
- [ ] Bridge 1: Incident → Queen task
- [ ] Bridge 2: Queen task → DSH reminder
- [ ] Bridge 3: Reminder → Queen task updates
- [ ] Handle provider failures gracefully

### Phase 4: Integration
- [ ] End-to-end Doctor → Queen → DSH workflow
- [ ] UI visibility and status synchronization
- [ ] Error handling and recovery mechanisms
- [ ] Performance optimization and testing

## Summary

**Current State**: DSH Schedule system confirmed functional  
**Missing Layer**: T27OS Queen task/state bridge  
**Next Step**: Manual smoke test verification  
**Final Goal**: Complete incident automation integration

The architecture is clear, the plan is solid, and we're ready to proceed with T27OS Queen integration.