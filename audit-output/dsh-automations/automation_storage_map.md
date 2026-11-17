# DSH Automation Storage Map

## Overview
This document maps the DSH automation storage paths and APIs to understand where automation tasks are persisted and how the UI accesses them.

## Investigation Plan

### Step 1: Backend Endpoints
Search for automation-related backend code:
- `/api/*` endpoints that serve automation lists
- Database/storage access patterns
- API route definitions

### Step 2: Storage Location
Identify where automation tasks are stored:
- File system paths (localStorage, IndexedDB, SQLite)
- Database schemas
- Configuration files

### Step 3: UI Query Patterns
Trace how the Automation UI queries data:
- Which components fetch automation lists
- What filters are applied
- How pagination works

### Step 4: Persistence Model
Understand task lifecycle:
- Creation time and cross-session persistence
- Update and deletion mechanisms
- State management (New/Enabled/Inactive)

## Expected Areas to Investigate

### Backend
- `packages/*/src/api/*` - REST endpoints
- `packages/*/src/services/*` - Business logic
- `packages/*/src/store/*` - Data persistence

### Frontend
- `packages/*/src/components/*` - UI components
- `packages/*/src/hooks/*` - Data fetching hooks
- `packages/*/src/types/*` - TypeScript definitions

### Configuration
- `packages/*/config/*` - Storage configuration
- `packages/*/db/*` - Database schemas
- `packages/*/migrations/*` - Migration scripts

## Files to Examine

### Priority 1 (Backend API)
```bash
# Find automation-related API endpoints
find packages -name "*.ts" -o -name "*.js" | xargs grep -l -i "automation" | head -20

# Find storage-related code  
find packages -name "*.ts" -o -name "*.js" | xargs grep -l -i "storage\|database\|persist" | head -20
```

### Priority 2 (Frontend Components)
```bash
# Find UI components that display automations
find packages -name "*.tsx" -o -name "*.ts" | xargs grep -l -i "Заданий пока нет\|automation.*list" | head -10

# Find data fetching hooks
find packages -name "*.ts" -o -name "*.tsx" | xargs grep -l -i "useEffect.*automation\|fetch.*automation" | head -10
```

### Priority 3 (Storage Configuration)
```bash
# Find storage configuration files
find packages -name "*.json" | xargs grep -l -i "storage\|database" | head -10

# Find database schemas
find packages -name "*.sql" -o -name "*.db" | head -10
```

## Hypotheses to Test

### Hypothesis 1: Missing Storage
- Automation tasks are created but not persisted
- Storage implementation is incomplete
- Database connection fails silently

### Hypothesis 2: UI Filter Issue  
- UI filters out all automations
- Wrong workspace/session/user context
- Status filter excludes all tasks

### Hypothesis 3: API/Storage Mismatch
- Backend returns empty results
- Storage format incompatible with UI expectations
- Authentication/authorization blocks access

## Next Steps

1. **Manual Creation**: Create a smoke automation task through the UI
2. **Storage Trace**: After creation, search for where it's stored
3. **API Verification**: Check if the task appears in API responses
4. **UI Verification**: Confirm task appears in the automation list