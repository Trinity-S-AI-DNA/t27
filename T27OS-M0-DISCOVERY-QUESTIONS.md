# T27OS M0 Discovery Questions - Explicit Investigation Scope

## Required M0 Investigation Areas

### 1. QueenTasks.json Location and Functionality

**MISSING COMPONENT**: Core task management system

**Discovery Questions:**
1. Search all `.trinity/` directories for QueenTasks implementation
2. Check alternative storage locations: `.trinity/state/`, `.trinity/seals/`, `queen_*` patterns
3. Verify if task state is managed through QueenLotus instead
4. Document any existing task management mechanisms
5. If completely absent, document the absence, identify consumers and required behavior from existing references, and return a discovery question set for a separate design decision

**Evidence Required:**
- Search command and results
- File paths and contents if found
- Functional testing if implementation exists
- Design specification if replacement needed

### 2. Trigit Verbs Implementation

**MISSING COMPONENT**: Execution mechanism for task operations

**Discovery Questions:**
1. Search all repositories for trigit verb implementations
2. Check existing patterns: `trigit_*.json`, `verb_*.t27`, `operation_*.t27`
3. Identify verb specification format and capabilities
4. Verify if QueenLotus provides verb functionality
5. Document existing patterns; if completely absent, identify consumers and return discovery questions for separate design

**Evidence Required:**
- Search results and file locations
- Verb specification examples
- Capability demonstrations if available
- Design recommendations if missing

### 3. T27OS Runtime Capability Verification

**CAPABILITY_UNVERIFIED**: Runtime exists but operational status unknown

**Discovery Questions:**
1. Verify T27OS runtime functionality beyond log files
2. Test actual task execution capabilities
3. Confirm process management and state tracking
4. Validate integration with existing tools
5. Document operational capabilities and limitations

**Evidence Required:**
- Runtime capability tests and results
- Process monitoring verification
- Integration testing results
- Capability assessment report

### 4. Task Protocol Behavior Validation

**CAPABILITY_UNVERIFIED**: Protocol exists but behavior not tested

**Discovery Questions:**
1. Test agent framework coordination functionality
2. Verify task protocol coordination mechanisms
3. Confirm multi-agent communication capabilities
4. Validate handoff and state transfer mechanisms
5. Document protocol behavior and limitations

**Evidence Required:**
- Protocol functionality tests
- Multi-agent coordination examples
- Handoff mechanism verification
- Behavior assessment results

### 5. Repository Shape Validation

**NOT_ESTABLISHED**: No repository validator identified

**Discovery Questions:**
1. Identify existing repository validation tools
2. Check for epic/M0/spec validators in tooling
3. Verify issue shape compliance mechanisms
4. Document validation requirements and procedures
5. Establish validation process for future issues

**Evidence Required:**
- Validator identification and testing
- Shape compliance verification results
- Validation procedure documentation
- Tool assessment report

## M0 Execution Boundaries

### AUTHORIZED OPERATIONS (Read-Only Only)
- Search `.trinity/` and related specs for QueenTasks references
- Inventory trigit coordination verb references  
- Read logs and existing task protocol documents
- Identify actual consumers and expected contracts
- Produce evidence matrix with commands and outputs
- Return open design questions if components are absent

### FORBIDDEN OPERATIONS
- Creating QueenTasks.json
- Implementing trigit verbs
- Designing a replacement control plane
- Modifying epic, M0, specs, or source files
- Starting executors or model calls
- Creating production artifacts or specifications

## M0 Acceptance Criteria - Updated

### AC-001 Verification (Revised)
```markdown
A component is classified as:
- FILE_PRESENT: Path exists and is readable
- FILE_HASH_VERIFIED: File hash integrity confirmed  
- CAPABILITY_VERIFIED: Functionality observed through direct testing
- CAPABILITY_UNVERIFIED: File exists but functionality not tested
- REFUTED: Prior claims disproven by evidence
- MISSING: Component not found in repository
- NOT_SCHEDULABLE: Hardware constraint acknowledged
- NOT_AVAILABLE: Hardware not available for research

Evidence must be retained and referenced for each classification.
```

### AC-002 Repository Validation (Revised)
```markdown
- Bootstrap build validation completed (exit code 0)
- Repository validation NOT ESTABLISHED until epic/M0 validator identified
- Issue shape validation pending validator identification
- Validation requirements documented for future implementation
```

### AC-003 Discovery Scope
```markdown
M0 discovery scope explicitly includes:
- QueenTasks.json location and functionality
- Trigit verbs implementation patterns
- T27OS runtime capability verification
- Task protocol behavior validation
- Repository shape validation methodology

M0 MUST NOT depend on:
- Secondary Mac availability (FR-016 constraint)
- FPGA hardware (FR-017 research-only)
```

### AC-004 Evidence Chain (Revised)
```markdown
Every claim must have:
- Source reference (file path, command, observation)
- Classification (FILE_PRESENT, CAPABILITY_VERIFIED, etc.)
- Output or log evidence
- Analysis conclusion
- Discovery action for MISSING/UNKNOWN components
```

## Final Discovery Boundaries

### In Scope for M0
- ✅ File existence verification
- ✅ File hash integrity checking
- ✅ Capability testing for existing components
- ✅ Search for missing components
- ✅ Hardware constraint documentation
- ✅ Repository validation methodology investigation

### Out of Scope for M0
- ❌ Implementation of missing components
- ❌ Hardware acquisition or setup
- ❌ Actual distributed execution testing
- ❌ FPGA hardware testing
- ❌ Protocol redesign or enhancement

## Next Phase Trigger - Revised

Completion of M0 enables M1 implementation with:
- ✅ Complete evidence matrix with proper classifications
- ✅ Missing components documented with discovery results
- ✅ Hardware constraints acknowledged and bounded
- ✅ Repository validation methodology identified
- ✅ Clear path from verification to implementation

Ready for review when all material claims are accurately classified and discovery scope is fully documented.