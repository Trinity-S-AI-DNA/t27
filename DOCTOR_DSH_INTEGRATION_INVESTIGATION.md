# Doctor → DSH Automation Tasks Integration Investigation
**Safe Read-Only Analysis**  
**Timestamp:** 2026-10-30T00:41:16+07:00

## INVESTIGATION SUMMARY

You're absolutely correct. The Doctor MVP is currently a standalone CLI/file generator, not integrated with DSH's automation task system. This explains why the "Задачи автоматизации" tab is empty.

## DSH AUTOMATION TASKS ANALYSIS

### Current State
```text
DOCTOR_MVP: Standalone CLI prototype
DSH_AUTOMATION_TASKS: Separate DSH subsystem  
CONNECTION: NOT_IMPLEMENTED
UI_TABS: Empty (depend on DSH task records)
```

### DSH Task System Investigation

#### Available Tools
From the cordis inspect, I found DSH task-related tools:
- `team_task_create` - Create unowned pending task on shared Team task board
- `team_task_list` - List shared tasks  
- `team_task_get` - Read complete task data
- `team_task_update` - Compare-and-set task actions

#### Current Task Status
```bash
team_task_list result: {"tasks":[]}
```
**CONFIRMED: No tasks exist in DSH system**

## REQUIRED INTEGRATION COMPONENTS

### 1. Doctor → DSH Bridge Architecture

```
DOCTOR WORKFLOW:
1. scan → incidents.json
2. triage → classified_incident.json  
3. handoff --dry-run → task_brief.json
4. handoff --approve → DSH create automation task
5. verify → task status check
```

### 2. DSH Task Schema Investigation

From `team_task_create` parameters:
```json
{
  "subject": "Concise task title",
  "description": "Complete task details and acceptance criteria", 
  "blocked_by": ["Task ids that must complete first"],
  "write_scopes": ["Advisory workspace-relative file/directory prefixes"]
}
```

### 3. Required Fields Mapping

**Doctor Incident → DSH Task**
```json
{
  "doctor_incident": {
    "id": "INC-2026-10-30-001",
    "error_class": "TEST_FAILURE", 
    "severity": "high",
    "evidence": [...],
    "suspected_root_cause": "unverified"
  },
  "dsh_task": {
    "subject": "Fix TEST_FAILURE in transport provider",
    "description": "Evidence-based repair task with verification gates",
    "blocked_by": [], 
    "write_scopes": [] // EMPTY until root cause confirmed
  }
}
```

## INTEGRATION IMPLEMENTATION PLAN

### Phase 1: Schema Discovery (CURRENT)
- ✅ DSH task schema identified
- ✅ Required fields mapped
- ✅ No task creation yet

### Phase 2: Dry-Run Implementation
- Create `doctor handoff --dry-run` command
- Generate task_brief.json preview
- Human approval before DSH creation

### Phase 3: Approval-Gated Creation
- `doctor handoff --approve` calls `team_task_create`
- Task appears in UI with Doctor source
- Status tracking integration

### Phase 4: Verification Integration  
- `doctor verify --task TASK-123` monitors completion
- Evidence attachment to tasks
- Verification gates as task criteria

## CRITICAL GUARDRAILS

### 1. No Write Scopes Until Root Cause Confirmed
```json
{
  "write_scopes": [], // Must be empty initially
  "allowed_actions": ["run tests", "inspect logs", "produce evidence"],
  "forbidden_actions": ["edit files", "push", "destructive operations"]
}
```

### 2. Approval Flow Required
```text
DOCTOR_STEP → USER_APPROVAL → DSH_TASK_CREATION
```

### 3. Evidence-Based Task Creation
- Only after incident classification
- Root cause explicitly documented  
- Verification gates defined

## NEXT STEPS - SAFE INVESTIGATION

### Priority 1: Complete Schema Investigation
- Map all Doctor incident fields to DSH task properties
- Define task status lifecycle
- Identify UI tab structure requirements

### Priority 2: Dry-Run Implementation  
- Create `doctor handoff --dry-run` command
- Generate human-readable task preview
- Include approval workflow

### Priority 3: Bridge Implementation
- `doctor handoff --approve` calls `team_task_create`
- Task status sync between Doctor and DSH
- Evidence attachment mechanism

## UI TAB MAPPING HYPOTHESIS

Based on typical DSH task structures, expected tabs:
```text
Overview - Task summary and status
Evidence - Collected error data and logs
Plan - Repair approach and verification steps  
Repair - Task execution (if applicable)
Verification - Results and gate checks
Logs - Command outputs and audit trail
```

## INTEGRATION BENEFITS

### 1. Visibility
- Tasks appear in DSH UI
- Status tracking visible to team
- Audit trail maintained

### 2. Approval Control  
- Human approval before creation
- Guardrails prevent dangerous operations
- Evidence-based task generation

### 3. Verification Integration
- Task completion verification
- Evidence attachment
- Automatic status updates

---

## CONCLUSION

**ROOT_CAUSE**: Doctor not connected to DSH task system  
**UI_EMPTY**: Expected behavior (standalone CLI)  
**NEXT**: Implement `doctor handoff --dry-run` bridge  
**SAFETY**: No DSH production changes during investigation

The investigation confirms your analysis. Doctor needs a bridge to DSH's `team_task_create` API to populate the automation tasks interface.