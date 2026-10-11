# Formal Foundations

## Prop. 112: Inverse-Check Rule for Claims Verification

### Statement

For a quantity the tree already knows, **no *other* numeric claim about it may appear unregistered**. The registered spelling is blanked out of the text and anything numeric left over is a finding.

### Formal Specification

The inverse-check rule operates as follows:

1. **Blanking**: The registered claim pattern's matched text is removed from the document before scanning
2. **Qualification**: Detection scopes to the same document and subject using explicit qualifiers (CI, checking, swept, sweep)
3. **Detection**: Any remaining numeric claim referencing a known quantity is reported as drift
4. **Precision**: The check permits rephrasing while forbidding drift and requiring no second pattern

### Implementation Requirements

- **FR-001**: The claims_check gate MUST blank out the registered claim pattern's matched text before scanning for residual numeric claims
- **FR-002**: The claims_check gate MUST treat any remaining numeric claim referencing a known quantity as a finding
- **FR-003**: Detection MUST scope to the same document and subject using explicit qualifiers to avoid false positives
- **FR-004**: Self-test suite MUST include a third case with injected synonym producing exactly one finding
- **FR-005**: Formal specification MUST be encoded in a machine-checkable .t27 spec file

### Verification Properties

The inverse-check rule satisfies the following properties:

1. **Completeness**: All unregistered numeric synonyms about known quantities are detected
2. **Soundness**: No false positives for unrelated numeric phrases in the same document
3. **Precision**: Exactly one finding produced for the injected synonym test case
4. **Consistency**: Maintains correctness through rephrasing while preventing drift

### Test Cases

The formal specification includes three self-test cases:

1. **Basic detection**: Verify unregistered synonyms are found
2. **Qualification scope**: Ensure unrelated numeric phrases are ignored  
3. **Injected synonym**: Confirm exactly one finding for "all forty CI steps" referencing "37 checking steps"

### Relationship to Previous Work

This rule addresses the limitation identified in Prop. 73 where "a gate that matches one phrasing sees only that phrasing." By inverting the check - detecting what remains after blanking the registered pattern - the system becomes robust against rephrasing while maintaining strict consistency.

The inverse-check rule was predicted by Prop. 110 to "meet shape 7 first" and has now held in three consecutive waves across three different checks, confirming its reliability and correctness.