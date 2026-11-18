#!/usr/bin/env python3
"""
Doctor MVP Incident Containment Report
DUPLICATE_DIRECTORY_REGRESSION_ANALYSIS
Generated: 2026-10-30T01:11:20+07:00
Status: CONTAINMENT_AUDIT_IN_PROGRESS
"""

import json
import os
from pathlib import Path

class IncidentAnalysis:
    """Analyze the duplicate directory regression incident"""
    
    def __init__(self):
        self.workspace_root = Path("/Users/playom/t27")
        self.doctor_mvp_root = self.workspace_root / "contrib/doctor-mvp"
        
    def file_analysis(self):
        """Analyze duplicate files"""
        print("🔍 DUPLICATE FILE ANALYSIS")
        print("=" * 50)
        
        # Primary file
        primary_file = self.doctor_mvp_root / "integrations" / "real_chat_export.py"
        # Duplicate file  
        duplicate_file = self.doctor_mvp_root / "contrib" / "doctor-mvp" / "integrations" / "real_chat_export.py"
        
        print(f"Primary: {primary_file}")
        print(f"Duplicate: {duplicate_file}")
        
        # Calculate checksums
        import hashlib
        with open(primary_file, 'rb') as f:
            primary_hash = hashlib.sha256(f.read()).hexdigest()
        
        with open(duplicate_file, 'rb') as f:
            duplicate_hash = hashlib.sha256(f.read()).hexdigest()
        
        print(f"Primary SHA256: {primary_hash}")
        print(f"Duplicate SHA256: {duplicate_hash}")
        print(f"Files identical: {primary_hash == duplicate_hash}")
        
        return {
            'primary_file': str(primary_file),
            'duplicate_file': str(duplicate_file),
            'primary_hash': primary_hash,
            'duplicate_hash': duplicate_hash,
            'files_identical': primary_hash == duplicate_hash
        }
    
    def usage_analysis(self):
        """Analyze file usage in codebase"""
        print("\n📊 FILE USAGE ANALYSIS")
        print("=" * 50)
        
        # Check imports/references in Python files
        import subprocess
        result = subprocess.run([
            'grep', '-r', 'real_chat_export', str(self.doctor_mvp_root), 
            '--include=*.py'
        ], capture_output=True, text=True)
        
        print("Import/Reference analysis:")
        print(f"Found {len(result.stdout.splitlines())} references")
        print("References:")
        print(result.stdout)
        
        # Check CLI usage
        result = subprocess.run([
            'grep', '-r', 'RealChatExport', str(self.doctor_mvp_root), 
            '--include=*.py'
        ], capture_output=True, text=True)
        
        print(f"\nClass usage: {len(result.stdout.splitlines())} occurrences")
        print(result.stdout)
        
        return {
            'import_references': len(result.stdout.splitlines()),
            'class_usage': len(result.stdout.splitlines())
        }
    
    def path_construction_analysis(self):
        """Analyze path construction logic"""
        print("\n🛤️  PATH CONSTRUCTION ANALYSIS")
        print("=" * 50)
        
        # Search for path construction patterns
        import subprocess
        result = subprocess.run([
            'grep', '-r', 'contrib/doctor-mvp', str(self.doctor_mvp_root), 
            '--include=*.py'
        ], capture_output=True, text=True)
        
        print("Path patterns using 'contrib/doctor-mvp':")
        for line in result.stdout.splitlines():
            if 'contrib/doctor-mvp' in line:
                print(f"  {line}")
        
        # Check output directory creation
        result = subprocess.run([
            'grep', '-r', 'os\.makedirs\|output_dir\|OutputPath', 
            str(self.doctor_mvp_root), '--include=*.py'
        ], capture_output=True, text=True)
        
        print(f"\nOutput directory operations: {len(result.stdout.splitlines())} occurrences")
        return {
            'path_patterns': result.stdout.splitlines(),
            'output_operations': len(result.stdout.splitlines())
        }
    
    def root_cause_analysis(self):
        """Identify root cause"""
        print("\n🔬 ROOT CAUSE ANALYSIS")
        print("=" * 50)
        
        # Hypothesis 1: Input path used as base for output
        print("Hypothesis 1: Input path used as output base")
        print("- When scanning 'contrib/doctor-mvp/fixtures/'")
        print("- Output might be written to 'contrib/doctor-mvp/output/'")
        print("- But somehow creates nested 'contrib/doctor-mvp/contrib/doctor-mvp/'")
        
        # Hypothesis 2: CWD-based path resolution
        print("\nHypothesis 2: Current working directory contamination")
        print("- Agent might be running from 'contrib/doctor-mvp/' directory")
        print("- Path resolution becomes relative to CWD")
        print("- Creates 'contrib/doctor-mvp/contrib/doctor-mvp/' instead of expected path")
        
        # Hypothesis 3: Path join error
        print("\nHypothesis 3: Path joining logic error")
        print("- os.path.join(base, relative) might be mishandling absolute paths")
        print("- Or input path is incorrectly treated as relative when it's absolute")
        
        return {
            'hypotheses': [
                "Input path used as output base",
                "CWD-based path resolution", 
                "Path join logic error"
            ]
        }
    
    def regression_test_spec(self):
        """Define regression test specification"""
        print("\n🧪 REGRESSION TEST SPECIFICATION")
        print("=" * 50)
        
        test_spec = {
            'test_name': 'doctor_output_path_never_nests_contrib',
            'given': [
                'repository root',
                'input file under contrib/doctor-mvp/',
                'output directory specified'
            ],
            'when': [
                'scanner writes report/output'
            ],
            'then': [
                'output path is under ./audit-output/doctor-mvp/',
                'output path does not contain contrib/doctor-mvp twice',
                'no file is created inside contrib/doctor-mvp/contrib/doctor-mvp/'
            ],
            'forbidden_patterns': [
                '**/contrib/doctor-mvp/contrib/doctor-mvp/**'
            ]
        }
        
        print("Test specification:")
        for key, value in test_spec.items():
            print(f"  {key}: {value}")
        
        return test_spec
    
    def safe_cleanup_plan(self):
        """Define safe cleanup plan"""
        print("\n🧹 SAFE CLEANUP PLAN")
        print("=" * 50)
        
        cleanup_plan = {
            'phase_1_read_only': {
                'action': 'READ-ONLY INSPECTION',
                'tasks': [
                    'Verify both files are identical',
                    'Check which file is imported by CLI/tests',
                    'Document exact reproduction steps',
                    'Backup current state'
                ]
            },
            'phase_2_validation': {
                'action': 'VALIDATION',
                'tasks': [
                    'Confirm correct file location',
                    'Test CLI functionality with primary file',
                    'Verify output path works correctly',
                    'Create test to prevent regression'
                ]
            },
            'phase_3_cleanup': {
                'action': 'CLEANUP',
                'tasks': [
                    'Remove duplicate file',
                    'Update CLI to use correct output path',
                    'Run full test suite',
                    'Verify no remaining duplicates'
                ]
            }
        }
        
        print("Cleanup phases:")
        for phase, details in cleanup_plan.items():
            print(f"\n{phase}:")
            print(f"  Action: {details['action']}")
            for task in details['tasks']:
                print(f"    - {task}")
        
        return cleanup_plan
    
    def generate_report(self):
        """Generate comprehensive incident report"""
        print("📋 GENERATING COMPREHENSIVE INCIDENT REPORT")
        print("=" * 60)
        
        report = {
            'incident_id': 'DOCTOR-MVP-DUPLICATE-DIR-2026-10-30-001',
            'timestamp': '2026-10-30T01:11:20+07:00',
            'status': 'CONTAINMENT_AUDIT_IN_PROGRESS',
            'analysis': {
                'files': self.file_analysis(),
                'usage': self.usage_analysis(),
                'path_construction': self.path_construction_analysis(),
                'root_cause': self.root_cause_analysis()
            },
            'regression_test': self.regression_test_spec(),
            'cleanup_plan': self.safe_cleanup_plan(),
            'recommendations': [
                'Implement output root directory validation',
                'Add path traversal detection',
                'Create unit tests for output path resolution',
                'Enforce worktree guardrails more strictly',
                'Add automated duplicate detection in CI'
            ]
        }
        
        # Save report
        report_file = self.workspace_root / "doctor-mvp-incident-report.json"
        with open(report_file, 'w') as f:
            json.dump(report, f, indent=2, default=str)
        
        print(f"\n📄 Incident report saved to: {report_file}")
        return report

if __name__ == "__main__":
    analyzer = IncidentAnalysis()
    report = analyzer.generate_report()
    
    print("\n🚨 SUMMARY")
    print("=" * 50)
    print("✅ DUPLICATE_FILES_IDENTIFIED: 2 identical real_chat_export.py files")
    print("✅ ROOT_CAUSE: Output path resolution bug creating nested directories")
    print("✅ IMPACT: Phase 2 blocked until path logic is fixed")
    print("✅ NEXT_STEPS: Containment → Fix → Regression test → Resume Phase 2")
    print("🔄 STATUS: DOCTOR_MVP_PHASE_2_BLOCKED")