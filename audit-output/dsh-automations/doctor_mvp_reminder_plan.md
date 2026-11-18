# Doctor MVP Reminder Creation Plan

## Objective
Create a manual DSH reminder to verify the automation UI works and establish baseline for T27OS Queen integration.

## Manual Reminder Creation

### Option 1: Via DSH Web UI
1. Navigate to DSH Web GUI: http://127.0.0.1:3080
2. Find "Automation Tasks" section  
3. Click "New automation task" or "+"
4. Fill in:
   ```
   Name: Doctor MVP duplicate directory regression
   Trigger: manual
   Task: Return JSON status report
   ```
5. Set timing: "Once" at current time + 1 minute
6. Save and verify appears in list

### Option 2: Via Agent Task
Create a task that schedules the reminder:
```
Create a reminder named "Doctor MVP duplicate directory regression" 
that triggers in 1 minute to check T27OS Queen task status.
```

### Option 3: Direct API Call
```bash
curl -X POST http://localhost:3080/api/schedule/create \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Doctor MVP duplicate directory regression",
    "prompt": "Check T27OS Queen task \"doctor-mvp-duplicate-directory-regression\". If its status is not COMPLETED, summarize current evidence and create a follow-up task. Do not retry providers. Do not modify files.",
    "after_seconds": 60
  }'
```

## Expected Results

### Success Indicators
- ✅ Reminder appears in "Automation Tasks" UI
- ✅ Reminder persists across page refresh
- ✅ Reminder can be executed manually
- ✅ Run history visible after execution

### UI Verification
1. **Empty State → Populated**: Should see the reminder
2. **Status**: Shows as "active" 
3. **Timing**: Shows next execution time
4. **Execution**: Manual trigger works
5. **History**: Run results visible

## T27OS Queen Integration Bridge

### Current Architecture
```
Doctor Incident → ? → T27OS Queen Task → ? → DSH Reminder → Session Delivery
```

### Missing Components
1. **Incident → Task Bridge**: Convert incidents to Queen tasks
2. **Task → Reminder Bridge**: Link Queen tasks to DSH reminders  
3. **Delivery → Status Bridge**: Update Queen tasks from reminder results

### Implementation Steps
1. **Verify DSH works** (this manual reminder)
2. **Build incident parser**: Identify incidents needing Queen tasks
3. **Create Queen task adapter**: Map incidents to task records
4. **Build reminder adapter**: Link Queen tasks to DSH reminders
5. **Create status sync**: Update Queen tasks from reminder results

## Success Metrics

### DSH Schedule System
- ✅ Reminder creation works
- ✅ Reminder persistence works  
- ✅ Reminder delivery works
- ✅ UI display works

### T27OS Queen Integration  
- 🔄 Incident → Queen task mapping
- 🔄 Queen task → DSH reminder linking
- 🔄 Status synchronization
- 🔄 Evidence collection

## Next Steps After Verification

1. **Document working DSH reminder**
2. **Design Queen task data model**
3. **Build incident detection service**  
4. **Implement task-reminder bridge**
5. **Create status update mechanisms**

This manual test will confirm DSH Schedule works and provide the foundation for building the T27OS Queen automation bridge.