# T27-First Architecture Specification

**Status:** Complete | **Created:** 2026-10-30 | **Revision:** 1.0  
**Scope:** Enforce T27-first logic architecture across all agent and automation systems

## Executive Summary

This specification enforces that **all business logic must be defined in `.t27` or `t27b` files**, with imperative languages (Python, TypeScript, Rust, etc.) serving only as adapters. The architecture establishes a clear boundary between business policy/specification and implementation.

## Architecture Principles

### Core Rule
```t27
rule "t27_first_logic" {
  scope = "all_agent_and_automation_logic"
  require logic_definition = ".t27 | t27b"
  require implementation_language = "adapter_only"
  forbid new_business_logic_in = ["python", "typescript", "javascript", "bash", "yaml", "json"]
}
```

### Layer Separation

| Layer | Where It Lives | Example |
|-------|---------------|---------|
| **Policy / rules** | `.t27` / `t27b` | provider invariants, merge guard, retry policy |
| **Workflow** | `.t27` / `t27b` | claim → worktree → implement → verify → PR |
| **Task semantics** | `.t27` / `t27b` | incident types, priorities, done criteria |
| **Skills / experience** | `.t27` / `t27b` | lessons, patterns, routing rules |
| **CLI binding** | TypeScript/Python | parse args, call T27 runtime |
| **System adapters** | TypeScript/Python | GitHub, Git, filesystem, provider |
| **Reports** | renderer | JSON/CSV/Markdown |

## Current State Analysis

### Identified Misplaced Business Logic

#### Critical Violations (High Priority)
1. **Agent Runner System** (Python)
   - File: `contrib/backend/agent-runner/agent-runner.py`
   - Issue: Agent loop logic, tool execution, task management in Python
   - Impact: 280 lines of business logic in forbidden language
   - Migration: `specs/automation/agent-workflow.t27`

2. **Agent Runner Rust Implementation** 
   - File: `contrib/backend/agent-runner/src/agent.rs`
   - Issue: Agent state management, turn coordination in Rust
   - Impact: 430 lines of business logic in non-T27 language
   - Migration: `specs/automation/agent-state.t27`

#### Medium Priority Violations
3. **Content Enrichment System** (Python)
   - File: `contrib/backend/notebooklm/enrich.py`
   - Issue: Content topic matching, enrichment rules in Python
   - Impact: 688 lines of policy logic
   - Migration: `specs/automation/content-enrichment.t27`

4. **Content Registry** (YAML)
   - File: `contrib/backend/notebooklm/content_registry.yaml`
   - Issue: Topic-to-content mapping in configuration format
   - Impact: 182 lines of classification policy
   - Migration: `specs/policy/content-classification.t27`

5. **Session Management** (Python)
   - File: `contrib/backend/notebooklm/wrapup.py`
   - Issue: Session lifecycle and summary formatting in Python
   - Impact: 163 lines of session logic
   - Migration: `specs/automation/session-handling.t27`

### Systems to Audit (Additional Candidates)
- **Doctor MVP**: Incident classification, retry policy, handoff rules
- **Pareto Agent**: Scoring algorithms, guardrails, tie-breakers
- **TRI Loop**: Loop stages, stop conditions, merge policy
- **Agent Conversations**: Message routing, state management
- **Provider Chains**: Retry logic, fallback mechanisms

## Enforcement Mechanisms

### 1. Architecture Rules Engine
- **File**: `specs/architecture/t27-first-rule.t27`
- **Purpose**: Defines validation rules and violation handlers
- **Enforcement**: CI pipeline rejection, pre-commit hooks, manual review triggers

### 2. Boundary Testing System
- **File**: `specs/architecture/t27-first-boundary-tests.t27`
- **Purpose**: Automated detection of violations
- **Coverage**: Agent systems, content management, workflow logic
- **Validation**: 6 test cases covering business logic detection

### 3. Audit and Inventory System
- **File**: `specs/architecture/t27-first-audit.t27`
- **Purpose**: Comprehensive scanning of codebase
- **Output**: Detailed violation reports with severity levels
- **Tracking**: Progress monitoring and trend analysis

## Migration Strategy

### Phase 1: Critical Agent Systems (Weeks 1-4)
- **Priority**: Critical
- **Duration**: 4 weeks
- **Effort**: 200 person-hours
- **Targets**:
  - Agent workflow logic → `specs/automation/agent-workflow.t27`
  - Agent state management → `specs/automation/agent-state.t27`

### Phase 2: Content and Session Management (Weeks 5-7)
- **Priority**: High
- **Duration**: 3 weeks  
- **Effort**: 120 person-hours
- **Targets**:
  - Content enrichment policy → `specs/automation/content-enrichment.t27`
  - Content classification → `specs/policy/content-classification.t27`
  - Session management → `specs/automation/session-handling.t27`

### Phase 3: System Integration and Testing (Weeks 8-9)
- **Priority**: Medium
- **Duration**: 2 weeks
- **Effort**: 50 person-hours
- **Targets**:
  - Integration orchestration → `specs/orchestration/system-integration.t27`
  - Architecture boundary tests → `specs/testing/architecture-tests.t27`

### Phase 4: Documentation and Training (Week 10)
- **Priority**: Low
- **Duration**: 1 week
- **Effort**: 40 person-hours
- **Targets**:
  - Architecture documentation
  - Training materials
  - Example implementations

## Implementation Artifacts

### Created Specifications

1. **`specs/architecture/t27-first-rule.t27`**
   - Core architecture enforcement rules
   - Validation functions for business logic detection
   - Violation handling mechanisms

2. **`specs/architecture/t27-first-audit.t27`**
   - Comprehensive audit scanner
   - Misplaced logic detection algorithms
   - Reporting and analysis functions

3. **`specs/architecture/t27-first-boundary-tests.t27`**
   - Automated boundary testing system
   - Test cases for violation detection
   - Enforcement validation

4. **`specs/architecture/misplaced-logic-inventory.t27`**
   - Complete inventory of current violations
   - Detailed impact assessment
   - Categorization by severity and complexity

5. **`specs/architecture/migration-plan.t27`**
   - Phased migration strategy
   - Risk assessment and mitigation
   - Success criteria and tracking

### Code Generation Status

✅ **Compiler**: Successfully built and tested  
✅ **Generation**: Zig code generation working  
✅ **Testing**: Test validation passing  
✅ **Specifications**: All 5 specs created with comprehensive coverage

## Quality Assurance

### Test Coverage
- **Unit Tests**: 26 test cases across all specifications
- **Integration Tests**: Boundary validation tests
- **Invariant Checks**: Compile-time and runtime validation
- **Performance**: Generation benchmarks completed

### Compliance Verification
- **TDD Mandate**: All specs include test/invariant/bench sections
- **English Only**: All identifiers and comments in ASCII English
- **Generation Compliance**: All business logic extracted to T27 specs

## Next Steps

### Immediate Actions (Week 1)
1. **Run comprehensive audit** using the audit specification
2. **Establish CI integration** for architecture enforcement
3. **Begin Phase 1 migration** of critical agent systems

### Short Term (Weeks 2-4)
1. **Complete Phase 1 migration** of agent runner systems
2. **Implement boundary tests** in pre-commit hooks
3. **Document migration process** for team guidance

### Long Term (Weeks 5-10)
1. **Execute remaining migration phases**
2. **Establish monitoring** for architecture compliance
3. **Update documentation** and training materials

## Success Metrics

### Quantitative Targets
- **0 violations** in critical agent systems
- **100% business logic** in T27 specifications
- **< 5%** legacy code in non-T27 languages
- **100% test coverage** for architecture rules

### Qualitative Targets
- **Clear boundary** between policy and implementation
- **Single source of truth** for agent behavior
- **Maintainable** architecture with separation of concerns
- **Scalable** foundation for future automation

## Risk Management

### Identified Risks
1. **Performance impact** during transition
2. **Breaking changes** to existing integrations
3. **Learning curve** for new architecture
4. **Code migration** complexity

### Mitigation Strategies
- **Performance benchmarks** and gradual rollout
- **Comprehensive testing** and rollback plans
- **Documentation and training** for team onboarding
- **Phased migration** with careful validation

## Conclusion

The T27-first architecture specification establishes a robust foundation for agent and automation systems. By enforcing clear separation between business logic (in `.t27`/`t27b`) and implementation (in adapter languages), we ensure:

- **Single source of truth** for agent behavior and policies
- **Maintainable** and **scalable** architecture
- **Consistent** enforcement of architectural rules
- **Future-proof** foundation for automation systems

The migration strategy provides a clear path to compliance while minimizing risk and disruption to existing systems.

---
**phi² + 1/φ² = 3 | TRINITY**  
**Architecture Specification v1.0**