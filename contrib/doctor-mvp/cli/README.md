# Doctor MVP CLI

## Installation and Setup

```bash
# Make the doctor CLI executable
chmod +x doctor.py

# Or create a symlink for easier access
ln -s $(pwd)/doctor.py /usr/local/bin/doctor
```

## Usage Examples

### 1. Scan for incidents

```bash
# Scan fixtures directory
python doctor.py scan --input fixtures/

# Scan specific file
python doctor.py scan --input fixtures/sample-chat-export.json

# Save results to file
python doctor.py scan --input fixtures/ --output incidents.json
```

### 2. Classify incidents

```bash
# Triage incidents from file
python doctor.py triage --input incidents.json

# Use custom rules file
python doctor.py triage --input incidents.json --rules docs/custom-rules.md
```

### 3. Create repair brief

```bash
# Generate brief for specific incident
python doctor.py handoff --incident INC-2026-10-30-001

# Use custom template
python doctor.py handoff --incident INC-2026-10-30-001 --brief custom-template.md
```

### 4. Verify repairs

```bash
# Verify incident resolution
python doctor.py verify --incident INC-2026-10-30-001

# Use specific verification gate
python doctor.py verify --incident INC-2026-10-30-001 --gate TEST_FAILURE
```

### 5. Generate reports

```bash
# Generate markdown report
python doctor.py report --input incidents.json --format md

# Generate JSON report  
python doctor.py report --input incidents.json --format json
```

## Incident Types Supported

- **BUILD_FAILURE**: Compilation errors, dependency issues
- **TEST_FAILURE**: Failed assertions, test timeouts
- **DISK_PRESSURE**: Low disk space, storage issues
- **PROVIDER_UNFUNDED**: 503 errors, quota exceeded
- **WRONG_WORKSPACE**: Repository mismatch, wrong branch
- **GIT_PUSH_BLOCKED**: Push rejections, merge conflicts
- **PROTECTED_STATE_RISK**: Unauthorized config changes
- **UNKNOWN**: Novel patterns, unclassified issues

## Sample Output

```json
{
  "id": "INC-2026-10-30-001",
  "source": "chat-export", 
  "workspace": "/Users/playom/deepseek-harness",
  "repository": "deepseek-harness",
  "branch": "clean/transport-unfunded-503-fix",
  "symptom": "5 tests failed after 503 classification change",
  "error_class": "TEST_FAILURE",
  "severity": "high",
  "blocked_by": [],
  "evidence": ["vitest output", "git status"],
  "suspected_root_cause": "unverified",
  "repair_chat": null,
  "status": "OPEN"
}
```