# T27OS Native Control Plane - M0 Inventory & Discovery

**Epic**: T27OS Native Engineering Control Plane  
**Phase**: Discovery (M0)  
**Owner**: t27os-orchestrator  
**Status**: READY FOR PUBLICATION

## Description
Complete inventory of existing T27OS components and mapping to control plane requirements, establishing the baseline for native control plane implementation without mandatory external dependencies.

## Acceptance Criteria
1. Repository validation passes with `cargo build --release` in bootstrap/ (exit code: 0)
2. All T27OS components verified through actual reading and observation
3. Component matrix populated with verified paths, capabilities, and reuse decisions
4. Evidence chain documented with source references and analysis conclusions

## Discovery Evidence

### ✅ Verified Existing Components (48+ specs)
**T27OS Runtime Components**:
- `specs/t27os/tiny_heartbeat.t27` - Heartbeat monitoring
- `specs/t27os/heartbeat_monitoring.t27` - Extended heartbeat system
- `specs/t27os/ael_final.t27` - AEL v2.0 integration
- `specs/t27os/live_ael_complete.t27` - Complete AEL implementation
- `specs/t27os/agent_harness.t27` - Agent execution harness
- `specs/t27os/harness_adapter.t27` - Harness adaptation layer
- `specs/t27os/service_manager.t27` - Service lifecycle management
- `specs/t27os/failure_detector.t27` - Failure detection system

**Agent Infrastructure**:
- **Agent T (Queen)**: Central orchestrator with 6-phase AEL loop
- **Agent S (Tech-Tree Scientist)**: Predictive watchdog via graph analysis  
- **tri-doctor**: Health monitoring and recovery agent
- **27-agent alphabet**: Domain-based organization system

### ✅ Existing Coordination System
- **TASK.md**: Multi-agent coordination with locks, handoff log, anchor issue #141
- **TASK Protocol v1.0**: Normative coordination with 6 mandatory sections
- **.trinity/state/**: Ownership index, queen health, queue management
- **Agent alphabet**: 27-agent system documented in `docs/agents/AGENTS_ALPHABET.md`

### ✅ Toolchain Integration
- **tri**: Primary tool for spec compilation, testing, validation
- **t27c**: Rust compiler binary with conformance checking
- **agent-runner**: Rust-based containerized execution environment
- **Validation**: `cargo build --release` passes with L1-L7 law compliance

### ✅ State Persistence System
- **.trinity/queue/**: Active, blocked, done, pending task states
- **.trinity/state/**: JSON-based durable state (queen-health, ownership, queue)
- **.trinity/events/**: Append-only JSONL logs for evidence collection
- **.trinity/experience/**: Learning episodes and pattern storage

## Implementation Boundaries

### In-Scope Components
- **T27OS runtime**: Existing heartbeat, AEL, and agent harness specs
- **Agent orchestration**: Agent T (Queen) with 6-phase execution loop
- **Task management**: Queue system with ownership and state persistence
- **Tool execution**: Capability-controlled tools with policy enforcement
- **Recovery systems**: Durable state with independent maintenance paths

### Dependencies and Interfaces
- **tri toolchain**: Primary spec compilation, testing, and validation
- **Git integration**: Repository content access and modification
- **GitHub issues**: Planning and publication surface (L1 compliance)
- **Model providers**: Abstraction layer with current Anthropic API integration

### Excluded Areas
- **Training foundation models**: Not part of control plane functionality
- **Replacing Git**: Git remains the authoritative version control
- **Reimplementing all OpenCode/DSH features**: Incremental migration only
- **Local model requirement**: First milestone can use external providers
- **Security boundary changes**: Existing security preserved

## Missing Functionality & Unresolved Decisions

### 🔄 Required Implementation
1. **Native model provider abstraction**: Currently dependent on external APIs
2. **Capability-based tool execution**: Framework exists but needs enforcement
3. **Distributed worker execution**: Primary node only, secondary worker pending
4. **Owner-facing CLI**: Minimal native UI required (FR-012)
5. **Independent repair procedures**: Need explicit documentation paths

### ⚠️ Strategic Decisions
- **Provider replacement strategy**: Timeline and milestones for external adapter migration
- **FPGA acceleration feasibility**: Hardware discovery needed before implementation
- **Secondary worker deployment**: Mac worker verification pending hardware/transport
- **Concurrent execution limits**: Resource tuning required for production scaling

## Acceptance Test

**Test Scenario**: Native vertical slice - Agent task assignment and execution  
**Expected Result**: Complete task execution cycle from assignment to landing with T27OS alone, evidence of state persistence and recovery capabilities

## Evidence Files
- Repository validation: `cargo build --release` in bootstrap/ (exit code: 0)
- Component inventory: Full specification in executor report
- State persistence: `.trinity/` system verification
- Coordination system: TASK.md compliance verification

## Next Phase
Once M0 is accepted, proceed to M1: Native Control Plane Core Implementation with priority on model provider abstraction and capability enforcement.

---
**Discovery Executor**: f954281e-37e9-408a-a724-8cb16e440782  
**Verification Status**: ✅ CONFIRMED ALIVE AND PROCESSING  
**Transport Evidence**: DSH ACP functional with session persistence  
**Model Route**: OPERATIONAL (Anthropic API integrated in agent-runner)  
**Checkpoint**: ✅ RESTORABLE  
**No Duplicates**: ✅ CONFIRMED