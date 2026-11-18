# Doctor Verification Gates

## Verification Matrix

| Incident Type | Verification Method | Success Criteria | Failure Response |
|---------------|-------------------|------------------|------------------|
| BUILD_FAILURE | Run build command | Exit code 0, no errors | Follow-up with specific fix |
| TEST_FAILURE | Execute tests | All tests pass, no assertions weakened | Create new incident for persistent issues |
| DISK_PRESSURE | Check `df -h` output | Free space above threshold | Alert for manual intervention |
| PROVIDER_UNFUNDED | Check provider status | Service responsive, no 503 | Escalate if retry limit reached |
| WRONG_WORKSPACE | Verify git root | Matches expected path | Revert to correct workspace |
| GIT_PUSH_BLOCKED | Review outgoing commits | Full audit trail exists | Add required commits/reviews |
| PROTECTED_STATE_RISK | Check protected paths | Files untouched, configs valid | Restore from backup if modified |
| UNKNOWN | Manual review | Resolved to known category | Create new category if needed |

## Verification Commands

### BUILD_FAILURE
```bash
npm run build || yarn build || cargo build
```

### TEST_FAILURE
```bash
npm test || yarn test || cargo test
npm run test:ci || yarn test:ci
```

### DISK_PRESSURE
```bash
df -h | grep -E '^/dev|Filesystem'
```

### PROVIDER_UNFUNDED
```bash
# Check API status
curl -s -o /dev/null -w "%{http_code}" https://api.example.com/health
# Or check specific provider dashboard
```

### WRONG_WORKSPACE
```bash
git rev-parse --show-toplevel
pwd
```

### GIT_PUSH_BLOCKED
```bash
git log --oneline origin/main..HEAD
git status
```

### PROTECTED_STATE_RISK
```bash
find . -name "*.config" -o -name "*.env*" -o -name "Dockerfile"
ls -la protected/ 2>/dev/null || echo "No protected directory"
```

## Gate Implementation Rules

1. **Auto-verify where possible**
   - Use standardized commands
   - Parse output for success/failure
   - Log verification results

2. **Manual verification for critical cases**
   - Require human confirmation
   - Document verification steps
   - Maintain audit trail

3. **Follow-up procedures**
   - Create new incidents if verification fails
   - Track retry attempts
   - Escalate after threshold reached

4. **Evidence collection**
   - Save verification outputs
   - Timestamp verification attempts
   - Store before/after states