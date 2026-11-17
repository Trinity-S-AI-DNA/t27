# DSH Automation Smoke Test Plan

## Test Objectives
Create a minimal automation task to verify the DSH automation subsystem is functioning correctly.

## Test Scenario 1: Manual Creation via UI

### Prerequisites
1. DSH Web GUI accessible at http://127.0.0.1:3080
2. User authentication completed
3. Workspace context set

### Test Steps

#### Step 1: Locate Automation Interface
1. Navigate to DSH Web GUI
2. Find the Automation/Task section
3. Identify the "Create Automation" or "New Task" button

#### Step 2: Create Smoke Automation
Fill in the following minimal automation:

```json
{
  "name": "T27OS Queen Smoke Automation",
  "description": "Minimal automation for testing subsystem",
  "trigger": "manual",
  "task": {
    "command": "Return JSON status report",
    "prompt": "Return JSON:\n{\n\"status\":\"ok\",\n\"source\":\"t27osqueen-smoke\",\n\"timestamp\":\"<now>\"\n}",
    "mode": "read-only",
    "approval": "not-required"
  }
}
```

#### Step 3: Verify Creation
1. Check if task appears in automation list
2. Note the task ID and timestamp
3. Verify status is "New" or "Enabled"

#### Step 4: Test Task Execution
1. Locate the task in the list
2. Click "Run" or "Execute"
3. Verify task starts successfully
4. Check for run history entry
5. Verify output matches expected JSON

### Expected Results
- **PASS**: Task appears in list immediately after creation
- **PASS**: Task executes successfully
- **PASS**: Run history is visible
- **PASS**: Output matches expected format
- **FAIL**: Task does not appear in list
- **FAIL**: Task fails to execute
- **FAIL**: No run history appears

## Test Scenario 2: API Direct Test

### Prerequisites
1. API endpoint identified
2. Authentication token available
3. Correct base URL

### Test Steps

#### Step 1: List Existing Automations
```bash
curl -X GET http://localhost:3080/api/automations \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" | jq .
```

#### Step 2: Create Automation via API
```bash
curl -X POST http://localhost:3080/api/automations \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "API Smoke Test",
    "description": "Created via API",
    "trigger": "manual",
    "task": {
      "command": "Return JSON status",
      "prompt": "Return JSON: {\"api_test\": \"success\"}",
      "mode": "read-only",
      "approval": "not-required"
    }
  }' | jq .
```

#### Step 3: Verify Creation
```bash
curl -X GET http://localhost:3080/api/automations \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" | jq '.[] | select(.name | contains("API Smoke Test"))'
```

### Expected Results
- **PASS**: API returns 201 Created on creation
- **PASS**: Task appears in API response
- **PASS**: Task appears in UI after refresh
- **FAIL**: API returns 400/500 error
- **FAIL**: Task not in API response
- **FAIL**: Task not visible in UI

## Test Scenario 3: Persistence Test

### Test Steps
1. Create smoke automation via UI
2. Refresh the browser page
3. Verify task still appears in list
4. Close and reopen browser
5. Verify task still appears in list
6. Restart DSH service
7. Verify task still appears in list

### Expected Results
- **PASS**: Task persists across page refresh
- **PASS**: Task persists across browser restart
- **PASS**: Task persists across service restart
- **FAIL**: Task disappears after page refresh
- **FAIL**: Task disappears after browser restart
- **FAIL**: Task disappears after service restart

## Test Scenario 4: Filter Test

### Test Steps
1. Create multiple automation tasks
2. Test different filters:
   - "New" filter
   - "Enabled" filter  
   - "Inactive" filter
3. Verify correct tasks appear in each filter
4. Test search functionality

### Expected Results
- **PASS**: All tasks visible when no filter applied
- **PASS**: Correct tasks visible for each filter
- **PASS**: Search returns matching tasks
- **FAIL**: Tasks missing from filters
- **FAIL**: Search not working
- **FAIL**: Incorrect tasks in filter results

## Test Scenario 5: Error Handling

### Test Steps
1. Try to create automation with missing fields
2. Try to create automation with invalid data
3. Try to execute non-existent automation
4. Try to execute automation without permission

### Expected Results
- **PASS**: Appropriate error messages displayed
- **PASS**: Validation prevents invalid creation
- **PASS**: Execution errors are handled gracefully
- **FAIL**: No error messages for invalid actions
- **FAIL**: System crashes on invalid input
- **FAIL**: Silent failures without user feedback

## Success Criteria

### Automation Visibility Fixed When:
- ✅ A manually created automation appears in UI
- ✅ It persists after page reload and session restart
- ✅ Its run history is visible
- ✅ Its storage location and API are documented
- ✅ Agent-created automations use the same persistent store
- ✅ No task is silently scoped to a dead session

## Test Deliverables

### Output Files
1. `./automation_test_results.md` - Detailed test results
2. `./automation_test_screenshots/` - Screenshots of key steps
3. `./automation_test_logs/` - Browser console logs and API responses

### Documentation
1. API endpoint documentation
2. Storage location mapping
3. UI component identification
4. Error handling patterns

## Next Steps After Testing

### If Tests PASS
1. Document the working subsystem
2. Proceed with T27OS Queen automation development
3. Implement persistent task storage for T27OS

### If Tests FAIL
1. Identify specific failure points
2. Fix storage/API issues
3. Fix UI display issues
4. Re-run smoke tests
5. Only then proceed with T27OS Queen

## Timeline

- **Phase 1**: Manual UI test (15 minutes)
- **Phase 2**: API direct test (10 minutes)
- **Phase 3**: Persistence test (5 minutes)
- **Phase 4**: Filter and error handling (10 minutes)
- **Phase 5**: Documentation (10 minutes)
- **Total**: 50 minutes