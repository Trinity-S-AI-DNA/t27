# Complete T27OS + DSH Integration Vision

## Final Architecture Blueprint ✅

### System Layers with Clear Boundaries
```text
DOCTOR LAYER:
- Detect and classify incidents
- Trigger Queen task creation
- No direct provider calls

T27OS QUEEN LAYER:
- Persistent task and state management  
- Evidence collection and policy
- GitHub Issue coordination
- No UI or delivery concerns

DSH SCHEDULE LAYER:
- Reminder and trigger delivery
- Session-based notifications
- UI visibility only

GITHUB LAYER:
- Agent coordination and claiming
- Single source of truth
- Progress tracking and transparency
```

### Complete Data Flow
```text
Doctor Incident
  ↓ (creates Queen task + GitHub Issue)
T27OS Queen Task (persistent state, evidence, retry policy)
  ↓ (optionally creates reminder)
DSH Schedule Reminder (time-based delivery to session)
  ↓ (executes in session)
DSH UI (visible reminder record)
  ↓ (result updates)
Queen Evidence + GitHub Issue Progress
```

### Execution Roadmap

#### Phase 1: Foundation (Manual Verification)
```text
STEP 1: DSH Smoke Test
- Create manual reminder via Web UI
- Verify UI, persistence, delivery
- Establish working baseline ✅
```

#### Phase 2: T27OS Queen Core
```text
STEP 2: T27OS Foundation  
- Task data model (status, evidence, owner, retry policy)
- Persistent storage system
- GitHub Issue synchronization layer
- Atomic claiming mechanism
```

#### Phase 3: Bridge Implementation
```text
STEP 3: Bridge 1 — Incident → Queen
- Doctor incident → Queen task + GitHub Issue
- Evidence storage without provider calls

STEP 4: Bridge 2 — Queen → DSH  
- Queen task → optional DSH reminder
- Reminder references task ID

STEP 5: Bridge 3 — DSH → Queen
- Reminder result → evidence append
- Queen policy controls status changes
```

#### Phase 4: End-to-End Integration
```text
STEP 6: Complete Workflow Test
- Doctor → Queen → DSH → UI → Evidence → GitHub
- Agent coordination through GitHub Issues
- Failure handling and follow-up creation
```

## GitHub Integration: Coordination Layer

### Agent Workflow via GitHub Issues
```text
1. ATOMIC CLAIM:
   - Queen.task.status = "claimed"  
   - POST /issues/:number/comments "TASK_CLAIMED @agent"
   - Issue: status=in-progress, assign agent

2. PROGRESS UPDATES:
   - Evidence append → Issue comment
   - Status changes → Issue labels/assignee

3. FAILURE HANDLING:
   - Provider unfunded → Issue blocked + follow-up created

4. COMPLETION:
   - Task completed → Issue closed with PR link
```

### Benefits of GitHub Coordination
- **No Duplication**: Issue ID = Task ID = Claim Authority
- **Full Transparency**: All progress and evidence visible
- **Recovery Support**: Clear status and handover support  
- **Metrics**: Completion rates and blocking causes trackable

## Definition of Done (Complete)

### Integration Success Criteria
```text
DOCTOR_AUTOMATION_INTEGRATION_COMPLETE WHEN:
1. ✅ DSH reminder smoke test passes (manual verification)
2. ✅ Doctor incident creates persistent Queen task + GitHub Issue
3. ✅ Queen task creates linked DSH reminder with task ID reference  
4. ✅ Reminder delivery visible in DSH UI and executes in session
5. ✅ Reminder result updates Queen evidence and GitHub Issue
6. ✅ PROVIDER_UNFUNDED creates follow-up Queen task + GitHub Issue
7. ✅ No automatic provider retry or silent fallback (explicit handling only)
8. ✅ Agents claim work through atomic GitHub Issue workflow
9. ✅ Status synchronized between Queen task and GitHub Issue at all times
10. ✅ End-to-end incident automation tested with real scenarios
```

## Final State Achieved

### When Complete:
```text
INCIDENT RESPONSE AUTOMATION:
- Doctor detects incident → creates Queen task → links to GitHub Issue
- Queen manages task lifecycle with evidence and policy  
- DSH delivers reminders as triggers/follow-ups
- Agents coordinate through GitHub Issues without duplication
- Full audit trail in GitHub Issue timeline
- Recovery from failures with automatic follow-up creation
```

### System Benefits:
- **Scalable**: No agent duplication or conflict
- **Transparent**: All work visible in GitHub Issues  
- **Recoverable**: Clear status and failure handling
- **Auditible**: Complete evidence trail in Issues
- **Maintainable**: Clear separation of concerns

## Ready for Implementation ✅

The complete vision is documented with clear execution steps, GitHub coordination system, and success criteria. The architecture boundaries are well-defined and the integration path is fully planned.

**Next Step**: Manual DSH smoke test verification to establish working baseline before T27OS Queen implementation.