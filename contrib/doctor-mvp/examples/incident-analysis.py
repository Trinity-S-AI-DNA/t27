#!/usr/bin/env python3
"""
Doctor MVP Example: Incident Analysis Workflow
Demonstrates the complete incident analysis pipeline
"""

import json
import re
from datetime import datetime
from pathlib import Path

class IncidentAnalyzer:
    def __init__(self):
        self.incidents = []
        self.rules = self._load_rules()
    
    def _load_rules(self):
        """Load triage rules and verification gates"""
        return {
            'error_classes': [
                'BUILD_FAILURE', 'TEST_FAILURE', 'DISK_PRESSURE', 
                'PROVIDER_UNFUNDED', 'WRONG_WORKSPACE', 'GIT_PUSH_BLOCKED',
                'PROTECTED_STATE_RISK', 'UNKNOWN'
            ],
            'severity_levels': ['low', 'medium', 'high'],
            'verification_gates': {
                'BUILD_FAILURE': 'npm run build || cargo build',
                'TEST_FAILURE': 'npm test || cargo test',
                'DISK_PRESSURE': 'df -h | grep -E "^/dev"',
                'PROVIDER_UNFUNDED': 'curl -s -o /dev/null -w "%{http_code}" https://api.example.com/health',
                'WRONG_WORKSPACE': 'git rev-parse --show-toplevel',
                'GIT_PUSH_BLOCKED': 'git log --oneline origin/main..HEAD',
                'PROTECTED_STATE_RISK': 'find . -name "*.config" -o -name "*.env*"',
                'UNKNOWN': 'manual review required'
            }
        }
    
    def analyze_chat_export(self, file_path):
        """Analyze chat export file for incidents"""
        with open(file_path, 'r') as f:
            data = json.load(f)
        
        incidents = []
        
        # Extract from messages
        for message in data.get('messages', []):
            if 'system' in message.get('role', ''):
                content = message.get('content', '')
                
                # Check for command failures
                if 'exit code:' in content:
                    exit_code_match = re.search(r'Exit code:\s*(\d+)', content)
                    if exit_code_match and int(exit_code_match.group(1)) != 0:
                        incident = self._create_incident(
                            source='chat-export',
                            symptom='Command failed with non-zero exit code',
                            content=content,
                            metadata=data.get('metadata', {})
                        )
                        incidents.append(incident)
                
                # Check for test failures
                if 'FAIL' in content and 'tests' in content:
                    incident = self._create_incident(
                        source='chat-export',
                        symptom='Tests failed',
                        content=content,
                        metadata=data.get('metadata', {}),
                        error_class='TEST_FAILURE'
                    )
                    incidents.append(incident)
        
        return incidents
    
    def analyze_log_file(self, file_path):
        """Analyze log file for incidents"""
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        incidents = []
        
        for line_num, line in enumerate(lines, 1):
            line = line.strip()
            
            # Check error patterns
            if 'error' in line.lower() or 'failed' in line.lower():
                incident = self._create_incident(
                    source=f'log-{Path(file_path).name}',
                    symptom=line,
                    content=f"Line {line_num}: {line}",
                    error_class=self._classify_error(line)
                )
                incidents.append(incident)
        
        return incidents
    
    def _create_incident(self, source, symptom, content, metadata=None, error_class='UNKNOWN'):
        """Create incident object"""
        incident = {
            'id': f"INC-{datetime.now().strftime('%Y-%m-%d-%H%M%S')}",
            'source': source,
            'workspace': metadata.get('workspace', '') if metadata else '',
            'repository': metadata.get('repository', '') if metadata else '',
            'branch': metadata.get('branch', '') if metadata else '',
            'symptom': symptom,
            'error_class': error_class,
            'severity': 'medium',
            'evidence': [content],
            'suspected_root_cause': 'unverified',
            'repair_chat': None,
            'status': 'OPEN',
            'verification_command': self.rules['verification_gates'].get(error_class, 'manual review')
        }
        
        return incident
    
    def _classify_error(self, line):
        """Classify error based on content"""
        line_lower = line.lower()
        
        if 'build' in line_lower or 'compile' in line_lower:
            return 'BUILD_FAILURE'
        elif 'test' in line_lower and 'fail' in line_lower:
            return 'TEST_FAILURE'
        elif 'disk' in line_lower or 'space' in line_lower:
            return 'DISK_PRESSURE'
        elif '503' in line_lower:
            return 'PROVIDER_UNFUNDED'
        elif 'wrong' in line_lower or 'repository' in line_lower:
            return 'WRONG_WORKSPACE'
        elif 'push' in line_lower or 'rejected' in line_lower:
            return 'GIT_PUSH_BLOCKED'
        elif 'protected' in line_lower or 'config' in line_lower:
            return 'PROTECTED_STATE_RISK'
        else:
            return 'UNKNOWN'
    
    def generate_brief(self, incident):
        """Generate repair brief for incident"""
        brief = f"""# INCIDENT BRIEF

## INCIDENT DETAILS
- **ID:** {incident['id']}
- **TYPE:** {incident['error_class']}
- **SEVERITY:** {incident['severity']}
- **WORKSPACE:** {incident['workspace']}
- **REPOSITORY:** {incident['repository']}
- **BRANCH:** {incident['branch']}
- **SYMPTOM:** {incident['symptom']}

## EVIDENCE
{chr(10).join(f"- {evidence}" for evidence in incident['evidence'])}

## HYPOTHESIS
Root cause analysis: {incident['suspected_root_cause']}

## GOAL
- Fix the identified issue
- Restore normal operation
- Prevent recurrence

## VERIFICATION GATE
Expected verification steps for {incident['error_class']}:
```
{incident['verification_command']}
```

## DEFINITION OF DONE
- [ ] Issue resolved according to specifications
- [ ] All verification steps passed
- [ ] No unintended side effects
- [ ] Evidence of fix documented
"""
        return brief

def main():
    print("Doctor MVP: Incident Analysis Demo")
    print("="*50)
    
    analyzer = IncidentAnalyzer()
    
    # Analyze chat export
    print("\n1. Analyzing chat export...")
    chat_incidents = analyzer.analyze_chat_export('fixtures/sample-chat-export.json')
    print(f"Found {len(chat_incidents)} incidents in chat export")
    
    # Analyze log file
    print("\n2. Analyzing log file...")
    log_incidents = analyzer.analyze_log_file('fixtures/sample-log-errors.txt')
    print(f"Found {len(log_incidents)} incidents in log file")
    
    # Combine incidents
    all_incidents = chat_incidents + log_incidents
    print(f"\nTotal incidents found: {len(all_incidents)}")
    
    # Display each incident
    for i, incident in enumerate(all_incidents, 1):
        print(f"\nIncident {i}:")
        print(json.dumps(incident, indent=2))
        
        # Generate brief for first incident
        if i == 1:
            print(f"\nGenerated Brief for {incident['id']}:")
            print(analyzer.generate_brief(incident))
    
    # Save analysis results
    output_file = 'examples/analysis-results.json'
    with open(output_file, 'w') as f:
        json.dump(all_incidents, f, indent=2)
    print(f"\nAnalysis results saved to: {output_file}")

if __name__ == "__main__":
    main()