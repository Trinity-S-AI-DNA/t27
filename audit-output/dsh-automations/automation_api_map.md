# DSH Automation API Map

## API Endpoints Investigation

### Step 1: Identify Automation API Routes
Search for automation-related API endpoints in the codebase:

```bash
# Find automation API routes
find packages -name "*.ts" -o -name "*.js" | xargs grep -l -i "api.*automation\|automation.*api" | head -20

# Find route definitions
find packages -name "*.ts" -o -name "*.js" | xargs grep -l -i "route\|endpoint\|app\.get\|app\.post" | head -20
```

### Step 2: Map Request/Response Patterns
Identify how automation data is structured:

```bash
# Find automation type definitions
find packages -name "*.ts" -o -name "*.tsx" | xargs grep -l -i "interface.*Automation\|type.*Automation" | head -10

# Find API response patterns
find packages -name "*.ts" -o -name "*.tsx" | xargs grep -l -i "response.*automation\|automation.*response" | head -10
```

### Step 3: Identify Authentication/Authorization
Check if authentication affects automation visibility:

```bash
# Find auth-related automation code
find packages -name "*.ts" -o -name "*.js" | xargs grep -l -i "auth.*automation\|automation.*auth" | head -10
```

## Expected API Structure

### List Automations Endpoint
```typescript
// Expected GET /api/automations
{
  "automations": [
    {
      "id": "string",
      "name": "string", 
      "description": "string",
      "status": "new|enabled|inactive",
      "trigger": "manual|schedule|event",
      "createdAt": "ISO8601",
      "updatedAt": "ISO8601",
      "runs": []
    }
  ],
  "pagination": {
    "total": number,
    "page": number,
    "limit": number
  }
}
```

### Create Automation Endpoint
```typescript
// Expected POST /api/automations
{
  "name": "string",
  "description": "string", 
  "trigger": "manual",
  "task": {
    "command": "string",
    "mode": "read-only|write",
    "approval": "required|not-required"
  }
}
```

## API Call Scenarios to Test

### Scenario 1: Direct API Call
```bash
# Test if automations endpoint returns data
curl -X GET http://localhost:3080/api/automations \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json"
```

### Scenario 2: Create Automation
```bash
# Test creating an automation
curl -X POST http://localhost:3080/api/automations \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "T27OS Queen Smoke Test",
    "description": "Manual smoke automation test",
    "trigger": "manual",
    "task": {
      "command": "Return JSON status",
      "mode": "read-only",
      "approval": "not-required"
    }
  }'
```

### Scenario 3: Test Different Scopes
- Different workspaces
- Different users
- Different sessions
- Different statuses

## UI Component Integration

### Expected Frontend Code Pattern
```typescript
// Component fetching automations
useEffect(() => {
  fetch('/api/automations')
    .then(res => res.json())
    .then(data => setAutomations(data.automations))
    .catch(error => console.error('Failed to fetch automations:', error));
}, []);
```

### Data Flow Analysis
1. UI calls API endpoint
2. Backend queries storage
3. Storage returns data (or empty array)
4. Backend processes and returns response
5. UI displays data or "Заданий пока нет" message

## Common Failure Patterns

### Pattern 1: Empty Response
- API returns 200 OK but empty array
- Authentication token issues
- Database query fails silently

### Pattern 2: Authentication Error
- API returns 401/403
- Token missing or expired
- Permission issues

### Pattern 3: Network Error
- API returns 500/502
- Backend service down
- CORS issues

## Investigation Questions

1. **Which exact endpoint** serves the automation list?
2. **What authentication** is required?
3. **How are permissions** determined?
4. **What storage query** is executed?
5. **How are filters** applied?

## Next Steps

1. **Find the actual API endpoints** by searching the codebase
2. **Test the endpoints** manually using curl
3. **Verify the data format** expected by the UI
4. **Check for authentication issues**
5. **Map the data flow** from storage to UI