# INCIDENT BRIEF

## INCIDENT DETAILS
- **ID:** `{incident_id}`
- **TYPE:** `{error_class}`
- **SEVERITY:** `{severity}`
- **WORKSPACE:** `{workspace}`
- **REPOSITORY:** `{repository}`
- **BRANCH:** `{branch}`
- **SYMPTOM:** `{symptom}`

## EVIDENCE
{evidence_list}

## HYPOTHESIS
Root cause analysis: `{suspected_root_cause}`

## GOAL
- Fix the identified issue
- Restore normal operation
- Prevent recurrence
- Maintain system integrity

## CONSTRAINTS & FORBIDDEN ACTIONS
- **NEVER** delete files or run destructive cleanup
- **NEVER** push automatically without review
- **NEVER** retry provider 503 more than once
- **NEVER** create tests to force green results
- **NEVER** weaken assertions to pass tests
- **ALWAYS** verify repository root before operations
- **ALWAYS** require explicit evidence for completion

## VERIFICATION GATE
Expected verification steps for `{error_class}`:

```
{verification_commands}
```

## DEFINITION OF DONE
- [ ] Issue resolved according to specifications
- [ ] All verification steps passed
- [ ] No unintended side effects
- [ ] Evidence of fix documented
- [ ] Root cause analysis confirmed
- [ ] Prevention measures in place

## NEXT STEPS
1. Analyze evidence and confirm hypothesis
2. Implement targeted fix
3. Execute verification commands
4. Document results and learnings
5. Close incident if verification passes