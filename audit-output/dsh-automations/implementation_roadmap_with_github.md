# T27OS + DSH Integration Roadmap with GitHub Coordination

## Execution Order Confirmed ✅

### Phase 1: DSH Baseline Verification
```text
STEP 1: DSH smoke test
- create one reminder via Web UI
- verify UI appearance, persistence, delivery
- establish working DSH baseline
```

### Phase 2: T27OS Queen Foundation
```text
STEP 2: T27OS Queen foundation
- persistent task model with status, evidence, owner, retry policy
- GitHub Issue synchronization layer
- atomic claiming mechanism
```

### Phase 3: Bridge Implementation
```text
STEP 3: Bridge 1 — incident to Queen
- Doctor incident creates Queen task
- task stores evidence and status
- creates corresponding GitHub Issue

STEP 4: Bridge 2 — Queen to DSH
- Queen task optionally creates linked DSH reminder
- reminder references Queen task ID
- updates GitHub Issue with progress

STEP 5: Bridge 3 — DSH to Queen
- reminder result appends evidence
- Queen policy alone changes task status
- updates GitHub Issue with execution results
```

### Phase 4: End-to-End Integration
```text
STEP 6: End-to-end integration test
- Doctor incident -> Queen task -> DSH reminder -> UI -> evidence update
- GitHub Issue as single source of truth
- Agent coordination through Issue workflow
```

## GitHub Issue Coordination System

### Queen Task ↔ GitHub Issue Mapping
```text
Every Queen task maps to exactly one GitHub Issue:
- Task ID ↔ Issue Number
- Task Status ↔ Issue Labels/Status  
- Task Evidence ↔ Issue Comments/Body
- Task Owner ↔ Issue Assignee
- Task Timeline ↔ Issue Timeline
```

### Agent Workflow with GitHub
```text
1. ATOMIC CLAIM:
   - Queen task.status = "claimed"
   - POST /issues/:number/comments "TASK_CLAIMED @agent"
   - GitHub issue: set status = "in-progress", assign agent

2. PROGRESS UPDATES:
   - Queen task.evidence.append(execution_result)
   - POST /issues/:number/comments "PROGRESS: {summary}"
   - GitHub issue: add progress comment

3. PROVIDER FAILURE HANDLING:
   - Queen task.status = "blocked"
   - POST /issues/:number/comments "PROVIDER_UNFUNDED: {error}"
   - GitHub issue: add "blocked" label, set status = "blocked"

4. COMPLETION:
   - Queen task.status = "completed"
   - POST /issues/:number/comments "COMPLETED with PR: #{pr_number}"
   - GitHub issue: close, add completion comment

5. FOLLOW-UP CREATION:
   - Queen task.createFollowUp(reason)
   - POST /issues/:number/comments "CREATED_FOLLOW_UP: #{new_issue_number}"
   - GitHub issue: link to new follow-up issue
```

### Issue Labels and Status Management
```text
GitHub Issue Labels:
- status:triage
- status:in-progress  
- status:blocked
- status:completed
- type:incident
- type:follow-up
- priority:high/medium/low

Issue Status Mapping:
- Open → status:triage
- In Progress → status:in-progress
- Blocked → status:blocked
- Closed → status:completed
```

## Agent Coordination Benefits

### Duplicate Prevention
```text
Single Source of Truth:
- Issue ID = Task ID = Claim Authority
- Status updates synchronized across all agents
- Evidence trail visible in issue comments
- No concurrent claiming possible
```

### Transparency
```text
Public Progress Tracking:
- All agents see current status and evidence
- Work history preserved in issue timeline
- Blockages visible to team
- Completion metrics available
```

### Recovery Support
```text
Failure Handling:
- Provider failures clearly documented
- Follow-up issues automatically created
- Status reflects actual work state
- Agent handovers supported through issue assignment
```

## Implementation Details

### Queen Task Data Model
```typescript
interface QueenTask {
  id: TaskId;
  githubIssue: IssueNumber;
  status: 'triage' | 'claimed' | 'in-progress' | 'blocked' | 'completed';
  owner: AgentId | null;
  evidence: EvidenceBundle[];
  retryPolicy: RetryPolicy;
  createdAt: ISO8601;
  updatedAt: ISO8601;
  linkedReminderId: ReminderId | null;
}
```

### GitHub Service Integration
```typescript
interface GitHubService {
  createIssue(task: QueenTask): Promise<IssueNumber>;
  addComment(issue: IssueNumber, comment: string): Promise<void>;
  setLabels(issue: IssueNumber, labels: string[]): Promise<void>;
  assignIssue(issue: IssueNumber, agent: AgentId): Promise<void>;
  closeIssue(issue: IssueNumber, reason: string): Promise<void>;
}
```

## Definition of Done (Final)

### Complete Integration When:
```text
DOCTOR_AUTOMATION_INTEGRATION_COMPLETE WHEN:
1. ✅ DSH reminder smoke test passes.
2. ✅ Doctor incident creates persistent Queen task with GitHub Issue.
3. ✅ Queen task creates linked DSH reminder referencing task ID.
4. ✅ Reminder delivery is visible in DSH UI.
5. ✅ Reminder result updates Queen evidence and GitHub Issue.
6. ✅ PROVIDER_UNFUNDED creates follow-up Queen task and GitHub Issue.
7. ✅ No automatic provider retry or silent fallback.
8. ✅ Agents claim and update work through GitHub Issue workflow.
9. ✅ Status synchronization between Queen task and GitHub Issue.
10. ✅ End-to-end incident automation tested and validated.
```

## Final Architecture
```text
Doctor Incident
  ↓ (creates Queen task + GitHub Issue)
T27OS Queen Task (with GitHub Issue sync)
  ↓ (optionally creates DSH reminder)
DSH Schedule Reminder (with task ID reference)
  ↓ (delivers to session)
DSH Automation Tasks UI (visible reminder)
  ↓ (execution result)
Queen Evidence Update + GitHub Issue Comment
```

This complete roadmap establishes GitHub Issues as the coordination layer for T27OS Queen task management, ensuring agent coordination, preventing duplication, and providing full transparency for the automation workflow.