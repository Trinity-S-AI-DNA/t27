# Doctor MVP - Incident Repair Automation System

A system to automatically detect, classify, and repair incidents from chat/session exports and logs.

## Architecture Overview

The Doctor system follows the PHI LOOP:
1. **Issue** - Detect incidents from logs
2. **Spec** - Classify incident types
3. **TDD** - Validate incident detection
4. **Code** - Generate repair briefs
5. **Gen** - Create isolated repair environments
6. **Seal** - Verify repair attempts
7. **Verify** - Monitor results
8. **Land** - Generate final reports
9. **Learn** - Extract patterns

## Key Components

- **Incident Schema** - JSON structure for detected incidents
- **Repair Brief Template** - Standard format for repair tasks
- **Triage Rules** - Classification logic for incident types
- **Verification Gates** - Validation criteria per incident type
- **CLI Commands** - Interface for scanning, triaging, and reporting

## Constraints

- Never delete files or run destructive cleanup
- Never push automatically
- Never retry provider 503 more than once
- Never create tests to make failures pass
- Always verify repository root before repair
- Always require explicit evidence for completion