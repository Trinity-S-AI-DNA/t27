#!/usr/bin/env python3
"""
Handoff System - DSH Integration and Repair Task Management
Creates isolated repair chats with proper task management
"""

import json
import re
import os
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Any
import subprocess

class HandoffSystem:
    """Manages handoff from Doctor incidents to DSH repair tasks"""
    
    def __init__(self):
        self.task_templates = {
            'TEST_FAILURE': self._create_test_failure_task,
            'BUILD_FAILURE': self._create_build_failure_task,
            'DISK_PRESSURE': self._create_disk_pressure_task,
            'PROVIDER_UNFUNDED': self._create_provider_unfunded_task,
            'WRONG_WORKSPACE': self._create_wrong_workspace_task,
            'GIT_PUSH_BLOCKED': self._create_git_push_blocked_task,
            'PROTECTED_STATE_RISK': self._create_protected_state_risk_task,
            'UNKNOWN': self._create_generic_task
        }
        
        self.verification_gates = {
            'TEST_FAILURE': ['run_tests', 'assertion_check', 'code_review'],
            'BUILD_FAILURE': ['build_check', 'dependency_check', 'compile_test'],
            'DISK_PRESSURE': ['disk_space_check', 'cleanup_audit'],
            'PROVIDER_UNFUNDED': ['provider_status', 'quota_check', 'retry_limit'],
            'WRONG_WORKSPACE': ['workspace_verification', 'path_validation'],
            'GIT_PUSH_BLOCKED': ['git_audit', 'commit_check', 'permission_check'],
            'PROTECTED_STATE_RISK': ['file_protection_check', 'config_validation'],
            'UNKNOWN': ['manual_review', 'comprehensive_check']
        }
    
    def create_repair_brief(self, incident: Dict, dry_run: bool = True) -> Dict:
        """Create repair brief from incident"""
        
        # Validate incident structure
        if not self._validate_incident(incident):
            raise ValueError("Invalid incident structure")
        
        # Create task based on incident type
        task_type = incident.get('error_class', 'UNKNOWN')
        template_func = self.task_templates.get(task_type, self._create_generic_task)
        
        repair_brief = template_func(incident)
        
        # Add common fields
        repair_brief.update({
            'incident_id': incident.get('id'),
            'created_at': datetime.now().isoformat(),
            'source': incident.get('source'),
            'task_type': 'DOCTOR_REPAIR',
            'priority': self._calculate_priority(incident),
            'verification_gates': self.verification_gates.get(task_type, []),
            'forbidden_actions': self._get_forbidden_actions(incident),
            'allowed_actions': self._get_allowed_actions(incident),
            'evidence': incident.get('evidence', []),
            'status': 'REPAIR_BRIEF_CREATED'
        })
        
        # Validate the brief
        self._validate_repair_brief(repair_brief)
        
        if dry_run:
            repair_brief['dsh_task_creation'] = 'DRY_RUN - No task created'
            repair_brief['next_steps'] = [
                'Review repair brief',
                'Validate repair approach',
                'Execute repair in isolated environment',
                'Verify results against gates'
            ]
        else:
            # Would create DSH task here
            repair_brief['dshtask_creation'] = 'PENDING - DSH task creation not implemented'
            repair_brief['next_steps'] = [
                'Create DSH automation task',
                'Assign to repair agent',
                'Monitor execution',
                'Verify completion'
            ]
        
        return repair_brief
    
    def _validate_incident(self, incident: Dict) -> bool:
        """Validate incident structure"""
        required_fields = ['id', 'error_class', 'severity', 'symptom', 'evidence']
        return all(field in incident for field in required_fields)
    
    def _calculate_priority(self, incident: Dict) -> str:
        """Calculate task priority"""
        severity = incident.get('severity', 'medium')
        incident_type = incident.get('error_class', 'UNKNOWN')
        
        # High priority for critical errors
        if severity == 'high' or incident_type in ['PROVIDER_UNFUNDED', 'PROTECTED_STATE_RISK']:
            return 'high'
        elif severity == 'medium':
            return 'medium'
        else:
            return 'low'
    
    def _get_forbidden_actions(self, incident: Dict) -> List[str]:
        """Get list of forbidden actions for repair"""
        incident_type = incident.get('error_class', 'UNKNOWN')
        
        forbidden_actions = [
            'delete_files',
            'modify_production_config',
            'force_push',
            'blind_test_creation',
            'assertion_weakening',
            'destructive_cleanup',
            'auto_retry_503',
            'ignore_root_cause'
        ]
        
        # Add type-specific forbidden actions
        if incident_type == 'WRONG_WORKSPACE':
            forbidden_actions.extend(['change_directory', 'modify_path'])
        elif incident_type == 'GIT_PUSH_BLOCKED':
            forbidden_actions.extend(['force_push', 'skip_merge_check'])
        
        return forbidden_actions
    
    def _get_allowed_actions(self, incident: Dict) -> List[str]:
        """Get list of allowed actions for repair"""
        incident_type = incident.get('error_class', 'UNKNOWN')
        
        allowed_actions = [
            'inspect_logs',
            'run_tests',
            'check_dependencies',
            'create_backup',
            'validate_environment',
            'document_changes'
        ]
        
        # Add type-specific allowed actions
        if incident_type == 'TEST_FAILURE':
            allowed_actions.extend(['fix_test_code', 'update_assertions', 'mock_external_services'])
        elif incident_type == 'BUILD_FAILURE':
            allowed_actions.extend(['fix_compile_errors', 'update_dependencies', 'fix_imports'])
        
        return allowed_actions
    
    def _validate_repair_brief(self, brief: Dict):
        """Validate repair brief structure"""
        required_fields = [
            'incident_id', 'task_type', 'priority', 
            'verification_gates', 'forbidden_actions', 'allowed_actions'
        ]
        
        missing_fields = [field for field in required_fields if field not in brief]
        if missing_fields:
            raise ValueError(f"Missing required fields: {missing_fields}")
    
    # Template functions for different incident types
    def _create_test_failure_task(self, incident: Dict) -> Dict:
        """Create repair brief for test failures"""
        return {
            'subject': f"Fix {incident.get('symptom', 'test failure')}",
            'description': f"""TEST FAILURE REPAIR TASK

Incident ID: {incident.get('id')}
Symptom: {incident.get('symptom')}
Severity: {incident.get('severity')}

EVIDENCE:
{chr(10).join(f"- {evidence}" for evidence in incident.get('evidence', []))}

ROOT CAUSE ANALYSIS:
- Tests are failing due to implementation issues
- Need to identify why assertions are failing
- Verify actual behavior matches expected behavior

REPAIR APPROACH:
1. Analyze failing test cases
2. Identify the root cause of failures
3. Fix implementation while preserving test integrity
4. Verify all tests pass without weakening assertions

DEFINITION OF DONE:
- All tests pass on the target environment
- No assertions weakened
- Implementation fixes root cause
- Test behavior is unchanged for passing tests
""",
            'estimated_effort': '2-4 hours',
            'complexity': 'medium'
        }
    
    def _create_build_failure_task(self, incident: Dict) -> Dict:
        """Create repair brief for build failures"""
        return {
            'subject': f"Fix build error: {incident.get('symptom', 'build failure')}",
            'description': f"""BUILD FAILURE REPAIR TASK

Incident ID: {incident.get('id')}
Symptom: {incident.get('symptom')}
Severity: {incident.get('severity')}

EVIDENCE:
{chr(10).join(f"- {evidence}" for evidence in incident.get('evidence', []))}

ROOT CAUSE ANALYSIS:
- Build process is failing
- Need to identify compilation/link errors
- Dependencies or configuration may be incorrect

REPAIR APPROACH:
1. Analyze build logs for specific errors
2. Fix compilation/link errors
3. Update dependencies if needed
4. Verify build succeeds

DEFINITION OF DONE:
- Build completes successfully
- No new compilation errors introduced
- All expected outputs are generated
- Build artifacts are valid
""",
            'estimated_effort': '1-3 hours',
            'complexity': 'medium'
        }
    
    def _create_disk_pressure_task(self, incident: Dict) -> Dict:
        """Create repair brief for disk pressure issues"""
        return {
            'subject': f"Resolve disk pressure: {incident.get('symptom', 'disk full')}",
            'description': f"""DISK PRESSURE REPAIR TASK

Incident ID: {incident.get('id')}
Symptom: {incident.get('symptom')}
Severity: {incident.get('severity')}

EVIDENCE:
{chr(10).join(f"- {evidence}" for evidence in incident.get('evidence', []))}

ROOT CAUSE ANALYSIS:
- Disk space is exhausted or critically low
- Need to identify large files or processes
- Clean up without losing important data

REPAIR APPROACH:
1. Identify what's consuming disk space
2. Clean up temporary files, logs, caches
3. Compress large files if needed
4. Monitor disk usage after cleanup

DEFINITION OF DONE:
- Disk usage is at safe levels (>20% free)
- No important data deleted
- System functionality maintained
- Cleanup process documented
""",
            'estimated_effort': '1-2 hours',
            'complexity': 'low'
        }
    
    def _create_provider_unfunded_task(self, incident: Dict) -> Dict:
        """Create repair brief for provider unfunded issues"""
        return {
            'subject': f"Resolve provider unfunded: {incident.get('symptom', '503 error')}",
            'description': f"""PROVIDER UNFUNDED REPAIR TASK

Incident ID: {incident.get('id')}
Symptom: {incident.get('symptom')}
Severity: {incident.get('severity')}

EVIDENCE:
{chr(10).join(f"- {evidence}" for evidence in incident.get('evidence', []))}

ROOT CAUSE ANALYSIS:
- Provider API returned 503 (service unavailable)
- May be due to quota limits, payment issues, or service problems
- Need to verify service status and take appropriate action

REPAIR APPROACH:
1. Verify provider service status
2. Check quotas and billing status
3. Implement retry logic with proper limits
4. Monitor for recurring issues

DEFINITION OF DONE:
- Service is accessible (no 503 errors)
- Proper retry logic implemented
- Monitoring in place for future issues
- Root cause addressed (billing/quotas if applicable)
""",
            'estimated_effort': '2-6 hours',
            'complexity': 'high'
        }
    
    def _create_wrong_workspace_task(self, incident: Dict) -> Dict:
        """Create repair brief for wrong workspace issues"""
        return {
            'subject': f"Fix wrong workspace: {incident.get('symptom', 'directory mismatch')}",
            'description': f"""WORKSPACE REPAIR TASK

Incident ID: {incident.get('id')}
Symptom: {incident.get('symptom')}
Severity: {incident.get('severity')}

EVIDENCE:
{chr(10).join(f"- {evidence}" for evidence in incident.get('evidence', []))}

ROOT CAUSE ANALYSIS:
- Working in incorrect directory/workspace
- Expected and actual directories don't match
- May lead to wrong operations or data loss

REPAIR APPROACH:
1. Verify expected workspace directory
2. Navigate to correct location
3. Verify repository state
4. Document the correction

DEFINITION OF DONE:
- Working in correct directory
- Repository state is verified
- No operations performed in wrong location
- Workspace validation process documented
""",
            'estimated_effort': '15-30 minutes',
            'complexity': 'low'
        }
    
    def _create_git_push_blocked_task(self, incident: Dict) -> Dict:
        """Create repair brief for git push blocked issues"""
        return {
            'subject': f"Resolve git push blocked: {incident.get('symptom', 'push rejected')}",
            'description': f"""GIT PUSH BLOCKED REPAIR TASK

Incident ID: {incident.get('id')}
Symptom: {incident.get('symptom')}
Severity: {incident.get('severity')}

EVIDENCE:
{chr(10).join(f"- {evidence}" for evidence in incident.get('evidence', []))}

ROOT CAUSE ANALYSIS:
- Git push was rejected by remote
- May be due to conflicts, permissions, or branch protection
- Need to resolve before successful push

REPAIR APPROACH:
1. Analyze rejection reason
2. Resolve conflicts if present
3. Verify branch permissions
4. Ensure commit history is clean

DEFINITION OF DONE:
- Push successful without conflicts
- All commits are properly formatted
- Branch protection rules satisfied
- Push audit trail is complete
""",
            'estimated_effort': '1-3 hours',
            'complexity': 'medium'
        }
    
    def _create_protected_state_risk_task(self, incident: Dict) -> Dict:
        """Create repair brief for protected state risk issues"""
        return {
            'subject': f"Fix protected state risk: {incident.get('symptom', 'config modification')}",
            'description': f"""PROTECTED STATE RISK REPAIR TASK

Incident ID: {incident.get('id')}
Symptom: {incident.get('symptom')}
Severity: {incident.get('severity')}

EVIDENCE:
{chr(10).join(f"- {evidence}" for evidence in incident.get('evidence', []))}

ROOT CAUSE ANALYSIS:
- Protected files or configs were modified
- May affect system security or stability
- Need to restore or properly validate changes

REPAIR APPROACH:
1. Identify which protected files were modified
2. Restore from backup if necessary
3. Validate changes if intentional
4. Implement proper change process

DEFINITION OF DONE:
- Protected state is secure
- All modifications are validated and approved
- Backup restored if needed
- Change process is followed
""",
            'estimated_effort': '2-4 hours',
            'complexity': 'high'
        }
    
    def _create_generic_task(self, incident: Dict) -> Dict:
        """Create generic repair brief for unknown incident types"""
        return {
            'subject': f"Repair: {incident.get('symptom', 'unknown issue')}",
            'description': f"""GENERIC REPAIR TASK

Incident ID: {incident.get('id')}
Symptom: {incident.get('symptom')}
Severity: {incident.get('severity')}
Type: {incident.get('error_class', 'UNKNOWN')}

EVIDENCE:
{chr(10).join(f"- {evidence}" for evidence in incident.get('evidence', []))}

ROOT CAUSE ANALYSIS:
- Incident type is not specifically classified
- Need for manual investigation and analysis
- May require multiple approaches to resolve

REPAIR APPROACH:
1. Investigate the specific symptoms
2. Identify root cause through analysis
3. Implement appropriate fix
4. Verify resolution

DEFINITION OF_DONE:
- Issue is resolved
- Root cause identified and addressed
- No side effects introduced
- Resolution is documented
""",
            'estimated_effort': '2-6 hours',
            'complexity': 'variable'
        }

def main():
    """Test the handoff system"""
    handoff = HandoffSystem()
    
    print("🔧 Handoff System Test")
    print("=" * 30)
    
    # Sample incident for testing
    test_incident = {
        'id': 'INC-TEST-001',
        'error_class': 'TEST_FAILURE',
        'severity': 'high',
        'symptom': '5 tests failing after 503 classification change',
        'evidence': [
            'vitest output showing 5 failures',
            'transport test expecting true but got false',
            'provider returning 503 Service Unavailable'
        ],
        'workspace': '/Users/playom/deepseek-harness',
        'repository': 'deepseek-harness'
    }
    
    # Test repair brief creation
    try:
        brief = handoff.create_repair_brief(test_incident, dry_run=True)
        
        print("✅ Repair brief created successfully")
        print(f"\n📋 Task Subject: {brief['subject']}")
        print(f"🎯 Priority: {brief['priority']}")
        print(f"⏱️  Estimated Effort: {brief['estimated_effort']}")
        print(f"🔍 Verification Gates: {len(brief['verification_gates'])} gates")
        print(f"❌ Forbidden Actions: {len(brief['forbidden_actions'])} actions")
        print(f"✅ Allowed Actions: {len(brief['allowed_actions'])} actions")
        
        print(f"\n📝 Description Preview:")
        print(brief['description'][:200] + "...")
        
        print(f"\n🚀 Status: {brief['status']}")
        if brief.get('next_steps'):
            print("Next steps:")
            for step in brief['next_steps'][:3]:
                print(f"  - {step}")
                
    except Exception as e:
        print(f"❌ Error creating repair brief: {e}")

if __name__ == "__main__":
    main()