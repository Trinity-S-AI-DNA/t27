# T27OS Epic Corrections - Required Edits

## Exact Edits for T27OS-EPIC-CORRECTED.md

### Change 1: QueenLotus Capability Description

**FROM:**
```
### Verified Components:
- ✅ QueenLotus: Code generation system (SHA verified)
```

**TO:**
```
### Verified Components:
- ✅ QueenLotus file integrity: SHA 81df76808de6fc8d26e648a22f915d92b0f2420515b8001bd4d3cf46ba4019f2 verified
- ⚠️ QueenLotus capability: Functionality exists but not verified as operational
```

### Change 2: T27OS Runtime Description

**FROM:**
```
- ✅ T27OS Runtime: Active monitoring with 3 live processes
```

**TO:**
```
- ❌ T27OS Runtime: 3 live processes claim REFUTED (no t27b processes found)
- ✅ T27OS Runtime monitoring: Log file present but operational status UNVERIFIED
```

### Change 3: Dependency Graph Description

**FROM:**
```
- ✅ Dependency Graph: 2163-node machine-readable structure
```

**TO:**
```
- ❌ Dependency Graph: 2163-node claim REFUTED (actual count: 55 nodes via jq '.nodes | length')
- ✅ Dependency Graph: 55-node machine-readable structure exists
```

### Change 4: Agent Framework Description

**FROM:**
```
- ✅ Agent Framework: Constitutional mandates and specifications
```

**TO:**
```
- ✅ Agent Framework file: Constitutional mandates and specifications present
- ⚠️ Agent Framework completeness: Full capability verification pending
```

### Change 5: Task Protocol Description

**FROM:**
```
- ✅ Task Protocol: Coordination and handoff mechanisms
```

**TO:**
```
- ✅ Task Protocol file: Coordination and handoff mechanisms present
- ⚠️ Task Protocol behavior: Operational verification pending
```

### Change 6: Missing Components Section

**ADD:**
```
### Missing Components (Discovery Required):
- ❌ QueenTasks.json: Core task management system absent
- ❌ Trigit verbs: Execution mechanism not found
```

### Change 7: FR-016 Distributed Worker Description

**FROM:**
```
### FR-016 — Distributed worker execution
T27OS MUST support capability-based task placement across registered
execution nodes.

The initial deployment MUST support:
- a primary interactive control node;
- a secondary Mac worker, after hardware and transport verification.
```

**TO:**
```
### FR-016 — Distributed worker execution (FUTURE CAPABILITY)
T27OS MUST support capability-based task placement across registered
execution nodes.

The initial deployment SHALL support:
- a primary interactive control node;
- secondary Mac worker support AFTER hardware and transport verification.

MUST_NOT block M0 completion on secondary Mac availability.
```

### Change 8: FR-017 FPGA Description

**FROM:**
```
### FR-017 — Optional FPGA acceleration
FPGA support MUST be an optional capability-specific execution backend,
not a general-purpose replacement for agent processes.

Hardware discovery MUST precede implementation.

An acceleration candidate MUST provide:
- a CPU reference;
- input/output and correctness contracts;
- host and device requirements;
- end-to-end measurements including transfer overhead;
- a safe failure path.

FPGA availability MUST NOT block the first native engineering workflow.
```

**TO:**
```
### FR-017 — Optional FPGA acceleration (NON_BLOCKING_RESEARCH)
FPGA support SHALL be an optional research-only capability,
not a general-purpose replacement for agent processes.

Hardware discovery SHALL be conducted as research.

An acceleration candidate SHALL provide:
- a CPU reference;
- input/output and correctness contracts;
- host and device requirements;
- end-to-end measurements including transfer overhead;
- a safe failure path.

FPGA SHALL NOT block M0 completion or first native engineering workflow.
MUST appear in acceptance criteria only as NON_BLOCKING_RESEARCH marker.
```

## Exact Edits for T27OS-M0-CORRECTED.md

### Change 1: Status Section

**FROM:**
```
### Verification (AC-001)
A component is verified only when:
- Its actual path exists and is readable
- Its revision/timestamp is recorded
- Its capability is observed through direct testing
- Evidence is retained and referenced
```

**TO:**
```
### Verification (AC-001)
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

### Change 2: Component Analysis Matrix

**ADD new columns:**
```
| Component | Verified path | Status | Classification | Evidence |
|-----------|---------------|--------|----------------|----------|
| QueenTasks | Not found | MISSING | Critical discovery required | Search all .trinity/ directories |
| Trigit verbs | Not found | MISSING | Implementation needed | Search existing verb patterns |
| T27OS Runtime | runtime/logs/ | LOG_FILE_PRESENT | CAPABILITY_UNVERIFIED | Test actual functionality |
```

### Change 3: Repository Validation

**FROM:**
```
### Repository Validation
- Execute: `cargo build --release` in bootstrap/
- Capture exit code and complete output
- Validate against expected shape conventions
- Document any validation failures
```

**TO:**
```
### Bootstrap Build Validation
- Execute: `cd bootstrap && cargo build --release`
- Capture exit code and complete output
- Document: Bootstrap compilation success (exit code 0)
- Note: Repository validation NOT ESTABLISHED until epic/M0 validator identified
```

### Change 4: Discovery Limits

**ADD:**
```
### M0 Scope Boundaries
- M0 MUST NOT depend on secondary Mac availability (FR-016 constraint)
- M0 MUST NOT depend on FPGA hardware (FR-017 research-only)
- Missing components (QueenTasks, tr verbs) are explicit M0 discovery scope
```

## Final Status After Corrections

**EVIDENCE MATRIX: READY_FOR_REVIEW**
**EPIC: BLOCKED_WITH_GAPS**
**M0: BLOCKED_WITH_GAPS**
**PUBLICATION: NOT_AUTHORIZED**
**BOOTSTRAP_BUILD: PASSED**
**REPOSITORY_VALIDATION: NOT_ESTABLISHED**

Ready for review because all claims are now accurately classified with proper evidence categories and missing components are explicitly documented as discovery scope.