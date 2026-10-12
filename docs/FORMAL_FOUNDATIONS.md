# Formal Foundations Documentation

## Proposition 119: Comment Handling in Formal Verification Gates

**Rule**: All formal gates that read Verilog and apply regex patterns MUST strip `//` comments before pattern matching or declare in writing why they do not.

### Rationale

This rule addresses a systematic defect pattern where comments in Verilog code caused formal gates to misbehave. Five separate fixes across four files were required to address this issue:

- **Property 95** (`claims_check`): A comment invented a 29th integration property out of a comment quoting an assertion
- **Property 102c** (`orphan_scan`): Identical defect, identical regex, sibling file — survived a wave because nobody grepped
- **Property 118** (`identity_scan`): A comment explaining why a property is **not** a self-comparison made it read as one
- **Property 118** (`guard_scan`): A note saying a guard "has been removed" reported it as present  
- **Property 118** (`bound_scan`): Rejected the repository's own backtick style in a note it parses

### Declared Exceptions

Four gates read comments *on purpose* and have declared their exceptions:

1. **`width_scan`**: Parses `range [-N, +M]` annotations — stripping them would delete everything its DOCUMENTED check reads
2. **`phantom_scan`**: Matches yosys warning output, where `//` cannot occur
3. **`faith_check`**: Reads `formal/*.py`; the `build/rtl` strings that put it in scope are path literals it compares against docstrings
4. **`encoding_gate`**: Permutes declarations in a copy fed only to yosys, which ignores comments anyway

### Implementation Requirements

- The `comment_scan` gate must detect and report any gate that reads Verilog with regex without stripping comments and without a written declaration
- The `mutate.py` helper `code_mask` must be recognized by `comment_scan` as a valid comment-stripping mechanism
- All formal gates must be documented as compliant with this rule

### Verification

The following must pass for compliance:
- `python -m formal.comment_scan` runs 18 gates, 14 in scope, 0 undeclared, exit 0
- `python -m formal.absence_sweep` runs 42 steps, 37 diagnosed, 0 indeterminate, 5 exempt, exit 0
- `python -m formal.doc_gate` validates 119/119 propositions, exit 0
- `python -m formal.claims_check` validates 9 claims, 0 stale, exit 0

---

*This documentation ensures that the pattern of defects caused by comment handling in formal gates is prevented through systematic declaration and verification.*