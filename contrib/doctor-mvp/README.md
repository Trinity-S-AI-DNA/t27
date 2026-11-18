# Doctor MVP

**Incident Detection, Classification, and Repair Automation System**

A minimalist automation system that reads problematic chats/sessions, extracts incidents, creates clean repair chats with context, and monitors fixes until verifiable results.

## DOCTOR PIPELINE

```
OBSERVE → TRIAGE → HANDOFF → REPAIR → VERIFY
```

1. **OBSERVE**: Reads active/recent chats and logs, finds errors, blockers, repeated failures
2. **TRIAGE**: Classifies (build, tests, disk, provider, Git, wrong-workspace), determines severity and blockers
3. **HANDOFF**: Creates new isolated repair chat with context and constraints
4. **REPAIR**: Agent executes narrow task with verification gate
5. **VERIFY**: Doctor rechecks result, creates follow-up if verification fails

## MVP Features

### Supported Incident Types
- **BUILD_FAILURE**: Compilation errors, dependency issues
- **TEST_FAILURE**: Failed assertions, test timeouts  
- **DISK_PRESSURE**: Low disk space, storage issues
- **PROVIDER_UNFUNDED**: 503 errors, quota exceeded
- **WRONG_WORKSPACE**: Repository mismatch, wrong branch
- **GIT_PUSH_BLOCKED**: Push rejections, merge conflicts
- **PROTECTED_STATE_RISK**: Unauthorized config changes
- **UNKNOWN**: Novel patterns, unclassified issues

### Core Rules
- **NEVER** delete files or run destructive cleanup
- **NEVER** push automatically without review
- **NEVER** retry provider 503 more than once
- **NEVER** create tests to force green results
- **NEVER** weaken assertions to pass tests
- **ALWAYS** verify repository root before operations
- **ALWAYS** require explicit evidence for completion

## Quick Start

```bash
# Install dependencies
cd contrib/doctor-mvp

# Run CLI demo
python3 examples/cli-demo.py

# Analyze incidents manually
python3 examples/incident-analysis.py

# Use CLI directly
python3 cli/doctor.py scan --input fixtures/
python3 cli/doctor.py triage --input fixtures/sample-chat-export.json
python3 cli/doctor.py handoff --incident INC-2026-10-30-001
```

## Architecture

```
contrib/doctor-mvp/
├── schema/
│   └── incident.schema.json          # JSON schema for incidents
├── cli/
│   ├── doctor.py                    # Main CLI tool
│   ├── requirements.txt             # Python dependencies
│   └── README.md                    # CLI documentation
├── fixtures/
│   ├── sample-chat-export.json      # Sample chat data
│   └── sample-log-errors.txt        # Sample log data
├── examples/
│   ├── cli-demo.py                  # CLI demonstration
│   └── incident-analysis.py          # Analysis workflow example
├── docs/
│   ├── triage.rules.md              # Triage classification rules
│   ├── verification.gates.md        # Verification gate definitions
│   └── repair-brief-template.md      # Brief template for repairs
├── doctor-brief-template.md         # Incident repair brief template
└── README.md                        # This file
```

## CLI Commands

### Scan for Incidents
```bash
python3 cli/doctor.py scan --input fixtures/
python3 cli/doctor.py scan --input fixtures/sample-chat-export.json --output incidents.json
```

### Triage and Classify
```bash
python3 cli/doctor.py triage --input incidents.json
python3 cli/doctor.py triage --input incidents.json --rules docs/custom-rules.md
```

### Create Repair Brief
```bash
python3 cli/doctor.py handoff --incident INC-2026-10-30-001
python3 cli/doctor.py handoff --incident INC-2026-10-30-001 --brief custom-template.md
```

### Verify Repairs
```bash
python3 cli/doctor.py verify --incident INC-2026-10-30-001
python3 cli/doctor.py verify --incident INC-2026-10-30-001 --gate TEST_FAILURE
```

### Generate Reports
```bash
python3 cli/doctor.py report --input incidents.json --format md
python3 cli/doctor.py report --input incidents.json --format json
```

## Incident Schema

```json
{
  "id": "INC-2026-10-30-001",
  "source": "chat-export",
  "workspace": "/Users/playom/deepseek-harness",
  "repository": "deepseek-harness",
  "branch": "clean/transport-unfunded-503-fix",
  "symptom": "5 failing tests",
  "error_class": "TEST_FAILURE",
  "severity": "high",
  "blocked_by": [],
  "evidence": ["vitest output", "git status"],
  "suspected_root_cause": "unverified",
  "repair_chat": null,
  "status": "OPEN"
}
```

## Verification Gates

| Incident Type | Verification Method | Success Criteria |
|---------------|-------------------|------------------|
| BUILD_FAILURE | `npm run build` | Exit code 0, no errors |
| TEST_FAILURE | `npm test` | All tests pass, no assertions weakened |
| DISK_PRESSURE | `df -h` | Free space above threshold |
| PROVIDER_UNFUNDED | Provider API check | Service responsive, no 503 |
| WRONG_WORKSPACE | `git rev-parse --show-toplevel` | Matches expected path |
| GIT_PUSH_BLOCKED | `git log origin/main..HEAD` | Full audit trail exists |
| PROTECTED_STATE_RISK | Protected paths check | Files untouched, configs valid |
| UNKNOWN | Manual review | Resolved to known category |

## Next Steps

1. **Phase 2**: Connect to real chat exports
2. **Phase 3**: Implement handoff to new chat system
3. **Phase 4**: Add automatic repair monitoring
4. **Phase 5**: Integrate with DSH-specific workflows

## Definition of Done

- [x] Incident schema defined
- [x] Repair brief template created
- [x] Triage rules documented
- [x] Verification gates implemented
- [x] CLI tool with basic commands
- [x] Example run on sample fixtures
- [ ] Real chat export integration
- [ ] Handoff to new chat system
- [ ] Automatic repair monitoring
- [ ] DSH-specific workflow integration

## MVP Implementation Plan

1. ✅ **Design Phase**: Schema, rules, gates, templates
2. 🚧 **Development Phase**: CLI implementation, fixture testing
3. ⏳ **Integration Phase**: Real chat exports, handoff system
4. ⏳ **Optimization Phase**: Monitoring, automation improvements
5. ⏳ **Deployment Phase**: Production use, feedback collection

---
**φ² + 1/φ² = 3 | TRINITY**