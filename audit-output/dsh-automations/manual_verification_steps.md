# Manual DSH Reminder Verification Steps

## Objective
Create a manual reminder in DSH to verify the automation system works.

## Method 1: DSH Web GUI (Recommended)

### Steps
1. **Open DSH Web Interface**
   - URL: http://127.0.0.1:3080
   - Ensure you're authenticated

2. **Navigate to Automation Tasks**
   - Look for "Automation" or "Schedule" section
   - Click on "Automation Tasks" or similar

3. **Create New Reminder**
   - Click "Create", "New", or "+" button
   - Fill in the form:
     ```
     Name: Doctor MVP Test
     Description: Verify T27OS Queen integration capability
     Trigger: Manual
     Time: 1 minute from now
     Prompt: Check T27OS Queen task "doctor-mvp-test" status. If not COMPLETED, summarize evidence and create follow-up task.
     ```

4. **Save the Reminder**
   - Click "Create", "Save", or "Schedule"
   - Verify it appears in the list

5. **Verify Functionality**
   - Check if reminder appears in the list
   - Verify it shows correct timing
   - Test manual execution if possible
   - Check for run history after execution

## Method 2: Direct API Call

### Using curl (if API available)
```bash
curl -X POST http://localhost:3080/api/schedule/create \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Doctor MVP Test",
    "prompt": "Check T27OS Queen task status and report findings",
    "after_seconds": 60
  }'
```

## Expected Results

### Success Indicators
- ✅ Reminder appears in Automation Tasks list
- ✅ Reminder shows correct timing (1 minute)
- ✅ Reminder persists across page refresh
- ✅ Can be executed manually (if UI supports)
- ✅ Run history visible after execution

### UI Elements to Look For
- **Empty State** → **Populated List** transition
- **Status**: Should show as "active" 
- **Timing**: Shows next scheduled time
- **Actions**: Execute, Edit, Delete buttons
- **Run History**: Shows execution results

## Troubleshooting

### If Reminder Doesn't Appear
1. **Check authentication** - Ensure logged into DSH
2. **Check permissions** - Verify create/schedule permissions
3. **Check network** - Ensure DSH backend is running
4. **Check filters** - Verify not filtered out by status/type
5. **Check browser console** - Look for JavaScript errors

### If Creation Fails
1. **Check form validation** - Ensure all required fields filled
2. **Check time format** - Ensure time is valid future time
3. **Check character limits** - Title/prompt length restrictions
4. **Check API response** - Look for error messages in console

## Documentation

### After Successful Creation
1. **Screenshot**: Take screenshot of reminder in list
2. **Timing Note**: Record exact creation time and next execution time  
3. **Behavior Log**: Document how it behaves (persistence, execution, etc.)
4. **UI Description**: Note exact UI elements and their state

### System Confirmation
This verification will confirm:
- ✅ DSH Schedule system is functional
- ✅ UI can display automation tasks
- ✅ Persistence works across sessions
- ✅ Ready for T27OS Queen integration

## Next Steps After Verification

1. **Document exact working system**
2. **Design T27OS Queen task model**
3. **Build incident-to-task bridge**
4. **Implement task-to-reminder adapter**
5. **Create status synchronization**