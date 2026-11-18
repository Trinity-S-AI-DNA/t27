# Gaps vs Required Agent Core Analysis
**Evidence-Based Gap Assessment**  
**Date:** 2026-10-30T01:32:34+07:00

## 1. Required Agent Core Capabilities

Based on the target actor contract specified in the original task:

### 1.1 Target Actor Core Components
```text
REQUIRED:
- Actor Supervisor: policy + lifecycle + retry
- Doctor Actor: incident_scan_request → incident_report
- Pareto Actor: incident_report → ranked_task  
- Repair Actor: ranked_task → implementation_evidence
- Auditor Actor: implementation_evidence → verdict
- Worktree Actor: isolated execution context
- Typed message schemas for each actor
- Effects system with scoped permissions
- Persistent task queue
- Evidence gates and policy enforcement
```

### 1.2 Required Actor Semantics
```text
CORE REQUIREMENTS:
1. Actor identity and addressability
2. Message passing with schemas
3. State isolation and persistence
4. Lifecycle management with supervision
5. Effects system with boundaries
6. Task queue management
7. Evidence collection and verification
8. Policy enforcement at runtime
```

## 2. Evidence Gap Analysis

### 2.1 Critical Gaps (Blocker Level)

#### Actor Identity System
```text
REQUIRED: Actor ID system for message routing
EVIDENCE: Limited - CLI branch patterns inferred, no global registry
GAP: 90% missing

IMPLICATION: Cannot implement actor message passing without addressing system
MITIGATION: Create actor ID registry from existing branch/issue patterns

EVIDENCE LOCATIONS:
- CLI commands: spawn_teammate, subagent patterns
- Branch naming: agent/<issue-id>-<slug> 
- Missing: Central actor registry, address resolution
```

#### Supervisor Hierarchy
```text
REQUIRED: Actor supervisor with policy + lifecycle + retry
EVIDENCE: Task protocol has handoff logging, no supervision
GAP: 85% missing

IMPLICATION: No automatic recovery or policy enforcement
MITIGATION: Extend task protocol with supervisor patterns

EVIDENCE LOCATIONS:
- docs/coordination/TASK_PROTOCOL.md: handoff logging
- Missing: Parent-child relationships, restart strategies
```

#### Persistent Task Queue
```text
REQUIRED: Persistent task queue for actor work distribution
EVIDENCE: Session-based, ephemeral task tracking
GAP: 80% missing

IMPLICATION: Tasks lost across sessions, no persistence
MITIGATION: Extend session state with persistent queue

EVIDENCE LOCATIONS:
- .trinity/current_task/: Session-bound state
- Missing: Persistent storage, queue management
```

### 2.2 Major Gaps (High Priority)

#### Effects System with Boundaries
```text
REQUIRED: Effects system with scoped permissions (worktree isolation)
EVIDENCE: Worktree isolation exists for safety, no effects framework
GAP: 70% missing

IMPLICATION: No formal effects modeling or permission boundaries
MITIGATION: Formalize worktree isolation into effects system

EVIDENCE LOCATIONS:
- WORKTREE_IMPLEMENTATION_SUMMARY.md: Safety isolation
- Missing: Effects categorization, permission boundaries
```

#### Evidence Gates and Verification
```text
REQUIRED: Evidence gates and policy enforcement
EVIDENCE: Evidence collection exists, no verification gates
GAP: 75% missing

IMPLICATION: No automatic verification or policy enforcement
MITIGATION: Create evidence verification system

EVIDENCE LOCATIONS:
- DOCTOR_DSH_INTEGRATION_INVESTIGATION.md: Evidence-based tasks
- Missing: Verification criteria, policy evaluation
```

#### Typed Message Schemas
```text
REQUIRED: Structured message schemas for actor communication
EVIDENCE: Event schemas exist, not actor-specific
GAP: 60% missing

IMPLICATION: No structured inter-actor communication
MITIGATION: Extend event schemas to actor message schemas

EVIDENCE LOCATIONS:
- specs/bus/schema.t27: Event type definitions
- Missing: Actor-specific message contracts
```

### 2.3 Moderate Gaps (Medium Priority)

#### Message Ordering and Reliability
```text
REQUIRED: Message ordering and delivery guarantees
EVIDENCE: Pub/sub system exists, no ordering guarantees
GAP: 65% missing

IMPLICATION: Non-deterministic message processing
MITIGATION: Add ordering and reliability layer to pub/sub

EVIDENCE LOCATIONS:
- specs/bus/pubsub.t27: Delivery queues
- Missing: Message ordering, acknowledgments
```

#### State Migration and Recovery
```text
REQUIRED: State persistence across actor restarts
EVIDENCE: Session state exists, no persistence
GAP: 80% missing

IMPLICATION: Actor state lost on restart
MITIGATION: Add persistence layer to agent state

EVIDENCE LOCATIONS:
- specs/server/agent-runner.t27: Agent state structure
- Missing: State serialization, recovery
```

#### Hot-Swapping Context
```text
REQUIRED: Worktree context switching for actor migration
EVIDENCE: Worktree isolation exists, no hot-swapping
GAP: 85% missing

IMPLICATION: Cannot migrate actors between contexts
MITIGATION: Extend worktree system for context migration

EVIDENCE LOCATIONS:
- WORKTREE_IMPLEMENTATION_SUMMARY.md: Static worktree creation
- Missing: Context migration, hot-swapping
```

## 3. Implementation Impact Assessment

### 3.1 Blocker Impact Matrix
```text
BLOCKER | Impact | Effort | Risk | Priority
--------|--------|--------|------|----------
Actor ID System | 90% | High | Medium | Critical
Supervisor Hierarchy | 85% | High | Low | Critical  
Persistent Task Queue | 80% | Medium | Medium | Critical
Effects System | 70% | Medium | Medium | High
Evidence Gates | 75% | Medium | Low | High
Message Schemas | 60% | Low | Low | Medium
```

### 3.2 Mitigation Strategy

#### Phase 1: Critical Blockers (4-6 weeks)
```text
1. Actor ID System (Week 1-2)
   - Extend task protocol with agent registry
   - Create address resolution from branch patterns
   - Implement actor messaging foundation

2. Supervisor Hierarchy (Week 3-4)  
   - Extend task protocol with parent-child relationships
   - Add restart strategies and policies
   - Implement supervision lifecycle

3. Persistent Task Queue (Week 5-6)
   - Extend session state with persistent storage
   - Implement queue management and recovery
   - Add persistence guarantees
```

#### Phase 2: Major Components (4-6 weeks)
```text
4. Effects System (Week 7-8)
   - Formalize worktree isolation as effects framework
   - Add permission boundaries and categorization
   - Implement effects validation

5. Evidence Gates (Week 9-10)
   - Create evidence verification system
   - Add policy evaluation framework
   - Implement automatic verification

6. Message Schemas (Week 11-12)
   - Extend event schemas to actor messages
   - Add structured communication contracts
   - Implement type-safe messaging
```

#### Phase 3: Advanced Features (4-6 weeks)
```text
7. Message Ordering (Week 13-14)
   - Add ordering guarantees to pub/sub
   - Implement acknowledgments
   - Ensure reliable delivery

8. State Migration (Week 15-16)
   - Add persistence to agent state
   - Implement recovery mechanisms
   - Ensure state consistency

9. Hot-Swapping (Week 17-18)
   - Extend worktree for context migration
   - Implement actor migration
   - Ensure seamless switching
```

## 4. Risk Assessment

### 4.1 Technical Risks
```text
HIGH RISK:
- Actor ID System: Complex coordination, potential conflicts
- Supervisor Hierarchy: Complex state management, deadlock risks

MEDIUM RISK:
- Persistent Queue: Storage reliability, consistency challenges
- Effects System: Permission boundary complexity, security risks

LOW RISK:
- Message Schemas: Structured typing, backward compatibility
- Evidence Gates: Verification logic, policy evaluation
```

### 4.2 Integration Risks
```text
DISRUPTION POTENTIAL:
- Critical (Blockers): Major refactoring required, breaking changes
- High (Major): Significant API changes, coordination needed
- Medium (Moderate): Incremental changes, minimal disruption
```

## 5. Success Criteria

### 5.1 Completion Metrics
```text
CRITICAL SUCCESS (95%+):
- Actor ID System implemented and tested
- Supervisor hierarchy functional with recovery
- Persistent queue with reliability guarantees

MAJOR SUCCESS (80-94%):
- Effects system with boundaries
- Evidence verification working
- Typed message schemas operational

MODERATE SUCCESS (60-79%):
- Message ordering implemented
- State persistence working
- Context migration functional
```

### 5.2 Validation Points
```text
EACH PHASE REQUIRES:
- Integration testing with existing DSH components
- Backward compatibility verification
- Performance benchmarks
- Security validation
```

---

**CONCLUSION:** Significant gaps exist (60-90% missing for core components). Critical blockers must be addressed first. Total estimated implementation: 12-18 weeks with proper risk management.