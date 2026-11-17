#!/usr/bin/env python3
"""
T27-First Architecture Enforcement Script
Enforces that business logic is in .t27/t27b files, not in Python/TypeScript/etc.
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Dict, Tuple

class T27FirstEnforcer:
    def __init__(self):
        self.violations: List[Dict] = []
        self.scanned_files = 0
        
        # Business logic keywords that indicate policy/workflow/rules
        self.business_logic_patterns = [
            r'agent.*loop|workflow|policy|orchestration|rule',
            r'agent.*state|task.*management|tool.*execution',
            r'content.*policy|topic.*matching|source.*selection',
            r'session.*management|formatting.*rules|notebook.*ops',
            r'incident.*classification|retry.*policy|handoff.*rules',
            r'tri.*loop|stop.*condition|merge.*policy',
            r'priority.*routing|verification.*gate|anomaly.*recovery'
        ]
        
        # Forbidden languages for business logic
        self.forbidden_extensions = ['.py', '.ts', '.js', '.sh', '.yaml', '.json']
        
        # Directories that should contain only adapter code
        self.adapter_directories = [
            'bindings',
            'adapters', 
            'clients',
            'renderers',
            'test_runners'
        ]

    def scan_directory(self, directory: str) -> None:
        """Scan directory for business logic violations"""
        directory_path = Path(directory)
        if not directory_path.exists():
            print(f"Warning: Directory {directory} not found")
            return
            
        print(f"Scanning {directory} for T27-first violations...")
        
        # Scan .py files for business logic
        for py_file in directory_path.rglob("*.py"):
            self.scan_python_file(py_file)
            
        # Scan .ts files for business logic  
        for ts_file in directory_path.rglob("*.ts"):
            self.scan_typescript_file(ts_file)
            
        # Scan YAML files for policy definitions
        for yaml_file in directory_path.rglob("*.yaml"):
            self.scan_yaml_file(yaml_file)

    def scan_python_file(self, file_path: Path) -> None:
        """Scan Python file for business logic patterns"""
        self.scanned_files += 1
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # Check for business logic patterns
            business_logic_found = False
            violations = []
            
            for i, pattern in enumerate(self.business_logic_patterns, 1):
                if re.search(pattern, content, re.IGNORECASE):
                    business_logic_found = True
                    violations.append(f"Pattern {i}: {pattern}")
                    
            if business_logic_found:
                violation = {
                    'file': str(file_path),
                    'language': 'python',
                    'severity': 'high' if 'agent' in file_path.name.lower() else 'medium',
                    'violations': violations,
                    'recommendation': f'Move business logic to specs/ directory in .t27 file'
                }
                self.violations.append(violation)
                print(f"VIOLATION: {file_path}")
                
        except Exception as e:
            print(f"Error reading {file_path}: {e}")

    def scan_typescript_file(self, file_path: Path) -> None:
        """Scan TypeScript file for business logic patterns"""
        self.scanned_files += 1
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # Check for business logic patterns
            business_logic_found = False
            violations = []
            
            for i, pattern in enumerate(self.business_logic_patterns, 1):
                if re.search(pattern, content, re.IGNORECASE):
                    business_logic_found = True
                    violations.append(f"Pattern {i}: {pattern}")
                    
            if business_logic_found:
                violation = {
                    'file': str(file_path),
                    'language': 'typescript',
                    'severity': 'medium',
                    'violations': violations,
                    'recommendation': f'Move business logic to specs/ directory in .t27 file'
                }
                self.violations.append(violation)
                print(f"VIOLATION: {file_path}")
                
        except Exception as e:
            print(f"Error reading {file_path}: {e}")

    def scan_yaml_file(self, file_path: Path) -> None:
        """Scan YAML file for policy definitions"""
        self.scanned_files += 1
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # Check for policy-like content
            if any(keyword in content.lower() for keyword in ['policy', 'workflow', 'rule', 'mapping']):
                violation = {
                    'file': str(file_path),
                    'language': 'yaml',
                    'severity': 'medium',
                    'violations': ['Policy/workflow definitions in YAML'],
                    'recommendation': f'Convert policy definitions to .t27 specification'
                }
                self.violations.append(violation)
                print(f"VIOLATION: {file_path}")
                
        except Exception as e:
            print(f"Error reading {file_path}: {e}")

    def generate_report(self) -> Dict:
        """Generate violation report"""
        total_violations = len(self.violations)
        
        # Count by severity
        severity_count = {'critical': 0, 'high': 0, 'medium': 0, 'low': 0}
        for violation in self.violations:
            severity_count[violation['severity']] += 1
        
        return {
            'scanned_files': self.scanned_files,
            'total_violations': total_violations,
            'severity_breakdown': severity_count,
            'violations': self.violations
        }

    def print_summary(self, report: Dict) -> None:
        """Print enforcement summary"""
        print("\n" + "="*60)
        print("T27-FIRST ARCHITECTURE ENFORCEMENT SUMMARY")
        print("="*60)
        print(f"Files scanned: {report['scanned_files']}")
        print(f"Total violations: {report['total_violations']}")
        
        if report['total_violations'] > 0:
            print("\nSeverity Breakdown:")
            for severity, count in report['severity_breakdown'].items():
                if count > 0:
                    print(f"  {severity.upper()}: {count}")
            
            print(f"\nCritical Violations Require Immediate Action:")
            for violation in self.violations:
                if violation['severity'] in ['critical', 'high']:
                    print(f"  🔴 {violation['file']}")
                    print(f"     Reason: {len(violation['violations'])} business logic patterns detected")
                    print(f"     Recommendation: {violation['recommendation']}")
                    print()
        else:
            print("✅ No violations detected!")

    def enforce_t27_first(self, directories: List[str]) -> int:
        """Enforce T27-first architecture and return exit code"""
        print("🔍 Starting T27-first architecture enforcement...")
        
        # Scan specified directories
        for directory in directories:
            self.scan_directory(directory)
        
        # Generate and display report
        report = self.generate_report()
        self.print_summary(report)
        
        # Return appropriate exit code
        if report['total_violations'] > 0:
            print(f"\n❌ Architecture violations found: {report['total_violations']}")
            print("Business logic must be moved to .t27 or t27b files.")
            return 1
        else:
            print("\n✅ Architecture compliant - no violations detected!")
            return 0

def main():
    """Main enforcement script"""
    enforcer = T27FirstEnforcer()
    
    # Directories to scan for violations
    scan_directories = [
        'contrib',
        'scripts',
        'packages'
    ]
    
    # Run enforcement
    exit_code = enforcer.enforce_t27_first(scan_directories)
    
    # Provide recommendations
    if exit_code != 0:
        print("\n📋 RECOMMENDATIONS:")
        print("1. Move business logic from Python/TypeScript to .t27 specifications")
        print("2. Keep only adapter code in Python/TypeScript files")
        print("3. Convert YAML policy definitions to .t27 format")
        print("4. Review the generated violation report above")
    
    sys.exit(exit_code)

if __name__ == "__main__":
    main()