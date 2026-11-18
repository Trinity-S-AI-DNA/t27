# Doctor Triage Rules

## Incident Classification

| Incident Type | Detection Patterns | Severity Criteria |
|---------------|-------------------|-------------------|
| BUILD_FAILURE | Exit codes != 0, compilation errors, dependency issues | high: build blocked; medium: warnings present |
| TEST_FAILURE | Failed assertions, test timeouts, flaky tests | high: critical tests fail; medium: non-critical fail |
| DISK_PRESSURE | Low disk space, disk full errors, slow I/O | high: <10% free; medium: <20% free |
| PROVIDER_UNFUNDED | 503 errors, quota exceeded, payment required | high: critical service down; medium: degraded service |
| WRONG_WORKSPACE | Wrong git root, unexpected repository, mixed branches | high: data loss risk; medium: confusion possible |
| GIT_PUSH_BLOCKED | Push rejected, merge conflicts, network issues | high: work lost risk; medium: delayed integration |
| PROTECTED_STATE_RISK | Protected files modified, config changes, dangerous ops | high: security risk; medium: compliance risk |
| UNKNOWN | Novel patterns, unclassified issues | medium: needs investigation |

## Triage Decision Flow

1. **ANALYZE ERROR PATTERNS**
   - Scan logs for error codes
   - Check command exit codes
   - Identify recurring failures
   - Look for blocked operations

2. **DETERMINE SEVERITY**
   - Check impact on development
   - Assess data loss risk
   - Evaluate time-to-resolution
   - Consider business impact

3. **IDENTify BLOCKERS**
   - Dependencies that prevent progress
   - Required resources unavailable
   - External system failures
   - Prerequisites not met

4. **COLLECT EVIDENCE**
   - Command outputs
   - Error traces
   - File states
   - System metrics

## Prevention Rules

**NEVER:**
- Delete files or run destructive cleanup
- Push automatically without review
- Retry provider 503 more than once
- Create tests to force green results
- Weaken assertions to pass tests
- Ignore repository root verification

**ALWAYS:**
- Verify workspace before operations
- Require explicit evidence for completion
- Document root causes and fixes
- Test changes in isolated environments
- Maintain audit trails for critical operations