#!/usr/bin/env python3
"""
Doctor MVP CLI - Incident Detection, Classification, and Repair Automation

Usage:
    doctor scan [--input=<path>] [--output=<file>]
    doctor triage [--input=<file>] [--rules=<path>]
    doctor handoff --incident=<id> [--brief=<template>]
    doctor verify --incident=<id> [--gate=<type>]
    doctor report [--input=<file>] [--format=<json|md>]

Commands:
    scan      Scan logs/chat exports for incidents
    triage     Classify incidents using triage rules
    handoff    Create repair brief from incident
    verify     Verify repair results against gates
    report     Generate final incident reports
"""

import argparse
import json
import os
import sys
import re
from datetime import datetime
from typing import Dict, List, Optional
from pathlib import Path

class Incident:
    def __init__(self, data: Dict):
        self.id = data.get('id', f"INC-{datetime.now().strftime('%Y-%m-%d-%H%M%S')}")
        self.source = data.get('source', 'unknown')
        self.workspace = data.get('workspace', '')
        self.repository = data.get('repository', '')
        self.branch = data.get('branch', '')
        self.symptom = data.get('symptom', '')
        self.error_class = data.get('error_class', 'UNKNOWN')
        self.severity = data.get('severity', 'medium')
        self.blocked_by = data.get('blocked_by', [])
        self.evidence = data.get('evidence', [])
        self.suspected_root_cause = data.get('suspected_root_cause', 'unverified')
        self.repair_chat = data.get('repair_chat', None)
        self.status = data.get('status', 'OPEN')

    def to_dict(self) -> Dict:
        return {
            'id': self.id,
            'source': self.source,
            'workspace': self.workspace,
            'repository': self.repository,
            'branch': self.branch,
            'symptom': self.symptom,
            'error_class': self.error_class,
            'severity': self.severity,
            'blocked_by': self.blocked_by,
            'evidence': self.evidence,
            'suspected_root_cause': self.suspected_root_cause,
            'repair_chat': self.repair_chat,
            'status': self.status
        }

class IncidentScanner:
    def __init__(self):
        self.error_patterns = {
            'BUILD_FAILURE': [
                r'exit code:\s*(\d+)',
                r'compilation error',
                r'failed to compile',
                r'error:\s*(.*)'
            ],
            'TEST_FAILURE': [
                r'FAIL\s*\d+/\d+ tests',
                r'Expected\s+(true|false|\d+).+but got',
                r'AssertionError',
                r'test.*failed'
            ],
            'DISK_PRESSURE': [
                r'disk\s*full',
                r'no space left',
                r'only\s*\d+% free',
                r' disk pressure'
            ],
            'PROVIDER_UNFUNDED': [
                r'503\s+Service Unavailable',
                r'quota exceeded',
                r'payment required',
                r'provider.*error'
            ],
            'WRONG_WORKSPACE': [
                r'wrong repository',
                r'expected.*but.*in',
                r'git.*rev-parse.*failed',
                r'workspace.*mismatch'
            ],
            'GIT_PUSH_BLOCKED': [
                r'push rejected',
                r'permission denied',
                r'merge conflict',
                r'non-fast-forward'
            ],
            'PROTECTED_STATE_RISK': [
                r'protected.*modified',
                r'config.*changed',
                r'dangerous operation',
                r'unauthorized.*change'
            ]
        }

    def scan_file(self, file_path: str) -> List[Incident]:
        incidents = []
        
        if file_path.endswith('.json'):
            incidents.extend(self._scan_json_file(file_path))
        elif file_path.endswith('.txt') or file_path.endswith('.log'):
            incidents.extend(self._scan_text_file(file_path))
        else:
            print(f"Warning: Unsupported file format {file_path}")
        
        return incidents

    def _scan_json_file(self, file_path: str) -> List[Incident]:
        incidents = []
        try:
            with open(file_path, 'r') as f:
                data = json.load(f)
                
                # Handle chat export format
                if 'messages' in data:
                    incidents.extend(self._extract_from_chat(data))
                
                # Handle single incident format
                if 'error_class' in data:
                    incidents.append(Incident(data))
                    
        except Exception as e:
            print(f"Error reading JSON file {file_path}: {e}")
        
        return incidents

    def _scan_text_file(self, file_path: str) -> List[Incident]:
        incidents = []
        try:
            with open(file_path, 'r') as f:
                lines = f.readlines()
                
            for line_num, line in enumerate(lines, 1):
                for error_type, patterns in self.error_patterns.items():
                    for pattern in patterns:
                        if re.search(pattern, line, re.IGNORECASE):
                            incident = Incident({
                                'id': f"INC-{datetime.now().strftime('%Y-%m-%d-%H%M%S')}-L{line_num}",
                                'source': file_path,
                                'symptom': line.strip(),
                                'error_class': error_type,
                                'severity': 'medium',
                                'evidence': [f"Line {line_num}: {line.strip()}"],
                                'suspected_root_cause': 'pattern_detected'
                            })
                            incidents.append(incident)
                            break
                            
        except Exception as e:
            print(f"Error reading text file {file_path}: {e}")
        
        return incidents

    def _extract_from_chat(self, chat_data: Dict) -> List[Incident]:
        incidents = []
        
        for message in chat_data.get('messages', []):
            if 'system' in message.get('role', ''):
                # Extract exit code and error details
                content = message['content']
                exit_match = re.search(r'Exit code:\s*(\d+)', content)
                
                if exit_match and int(exit_match.group(1)) != 0:
                    incident = Incident({
                        'id': f"INC-{chat_data.get('session_id', 'unknown')}",
                        'source': 'chat-export',
                        'workspace': chat_data.get('metadata', {}).get('workspace', ''),
                        'repository': chat_data.get('metadata', {}).get('repository', ''),
                        'branch': chat_data.get('metadata', {}).get('branch', ''),
                        'symptom': 'Command failed with non-zero exit code',
                        'error_class': 'BUILD_FAILURE' if 'build' in content.lower() else 'TEST_FAILURE',
                        'severity': 'high',
                        'evidence': [content],
                        'suspected_root_cause': 'command_failure'
                    })
                    incidents.append(incident)
                
                # Check for test failures
                if 'FAIL' in content and 'tests failed' in content:
                    incident = Incident({
                        'id': f"INC-{chat_data.get('session_id', 'unknown')}",
                        'source': 'chat-export',
                        'workspace': chat_data.get('metadata', {}).get('workspace', ''),
                        'repository': chat_data.get('metadata', {}).get('repository', ''),
                        'branch': chat_data.get('metadata', {}).get('branch', ''),
                        'symptom': 'Tests failed',
                        'error_class': 'TEST_FAILURE',
                        'severity': 'high',
                        'evidence': [content],
                        'suspected_root_cause': 'test_failure'
                    })
                    incidents.append(incident)
        
        return incidents

def verify_worktree_isolation():
    """Verify worktree isolation rules before task creation"""
    import subprocess
    import os
    from pathlib import Path
    
    violations = []
    current_path = Path.cwd()
    
    try:
        # Check if we're in a worktree
        result = subprocess.run(['git', 'worktree', 'list'], 
                              capture_output=True, text=True, check=True)
        worktree_list = result.stdout.strip().split('\n')
        current_path_str = str(current_path)
        
        is_worktree = any(current_path_str in line for line in worktree_list)
        if not is_worktree:
            violations.append("Not in approved worktree")
            
        # Check branch name
        result = subprocess.run(['git', 'branch', '--show-current'], 
                              capture_output=True, text=True, check=True)
        branch_name = result.stdout.strip()
        
        if not branch_name.startswith('agent/'):
            violations.append(f"Branch '{branch_name}' doesn't match agent/ prefix")
            
        # Check for unrelated modified files in primary checkout
        result = subprocess.run(['git', 'status', '--porcelain'], 
                              capture_output=True, text=True, check=True)
        modified_files = result.stdout.strip().split('\n')
        
        # Filter worktree-specific files
        primary_repo_modifications = []
        for file_status in modified_files:
            if file_status.strip() and not file_status.startswith('wt/'):
                primary_repo_modifications.append(file_status.strip())
                
        if primary_repo_modifications:
            violations.append(f"Unmodified files in primary checkout: {primary_repo_modifications}")
            
    except subprocess.CalledProcessError as e:
        violations.append(f"Git command failed: {e}")
        
    return violations

def main():
    parser = argparse.ArgumentParser(description='Doctor MVP CLI')
    subparsers = parser.add_subparsers(dest='command', help='Available commands')

    # Scan command
    scan_parser = subparsers.add_parser('scan', help='Scan logs/chat exports for incidents')
    scan_parser.add_argument('--input', default='fixtures/', help='Input path to scan')
    scan_parser.add_argument('--output', help='Output file for incidents')

    # Triage command
    triage_parser = subparsers.add_parser('triage', help='Classify incidents using triage rules')
    triage_parser.add_argument('--input', required=True, help='Input incidents file')
    triage_parser.add_argument('--rules', default='docs/triage.rules.md', help='Triage rules file')

    # Handoff command
    handoff_parser = subparsers.add_parser('handoff', help='Create repair brief from incident')
    handoff_parser.add_argument('--incident', required=True, help='Incident ID to process')
    handoff_parser.add_argument('--brief', default='doctor-brief-template.md', help='Brief template')
    handoff_parser.add_argument('--dry-run', action='store_true', help='Generate task preview without DSH creation')

    # Verify command
    verify_parser = subparsers.add_parser('verify', help='Verify repair results against gates')
    verify_parser.add_argument('--incident', required=True, help='Incident ID to verify')
    verify_parser.add_argument('--gate', help='Verification gate type')

    # Report command
    report_parser = subparsers.add_parser('report', help='Generate final incident reports')
    report_parser.add_argument('--input', help='Input incidents file')
    report_parser.add_argument('--format', choices=['json', 'md'], default='md', help='Output format')

    # Worktree health check command
    health_parser = subparsers.add_parser('health', help='Check worktree isolation and system health')
    health_parser.add_argument('--verify-worktree', action='store_true', help='Verify worktree isolation rules')

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return

    if args.command == 'scan':
        scanner = IncidentScanner()
        incidents = []
        
        input_path = Path(args.input)
        if input_path.is_file():
            incidents.extend(scanner.scan_file(str(input_path)))
        elif input_path.is_dir():
            for file_path in input_path.rglob('*'):
                if file_path.is_file() and (file_path.suffix in ['.json', '.txt', '.log']):
                    incidents.extend(scanner.scan_file(str(file_path)))
        
        print(f"Found {len(incidents)} incidents")
        
        if args.output:
            with open(args.output, 'w') as f:
                json.dump([incident.to_dict() for incident in incidents], f, indent=2)
            print(f"Incidents saved to {args.output}")
        else:
            for incident in incidents:
                print(json.dumps(incident.to_dict(), indent=2))

    elif args.command == 'triage':
        print(f"Triage command - processing {args.input}")

    elif args.command == 'handoff':
        # Worktree isolation check for handoff
        violations = verify_worktree_isolation()
        if violations:
            print("❌ WORKTREE ISOLATION VIOLATIONS:")
            for violation in violations:
                print(f"  - {violation}")
            print("\nTask creation BLOCKED. Verify worktree isolation rules.")
            return
        
        if args.dry_run:
            print(f"🔍 DRY-RUN: Generating repair brief for incident {args.incident}")
            print("Task preview generated. No DSH task created.")
        else:
            print(f"Handoff command - creating brief for incident {args.incident}")
            print("⚠️  DSH task creation not implemented yet")

    elif args.command == 'verify':
        print(f"Verify command - checking incident {args.incident}")

    elif args.command == 'report':
        print(f"Report command - generating report in {args.format} format")

    elif args.command == 'health':
        if args.verify_worktree:
            violations = verify_worktree_isolation()
            if violations:
                print("❌ WORKTREE ISOLATION ISSUES:")
                for violation in violations:
                    print(f"  - {violation}")
            else:
                print("✅ WORKTREE ISOLATION: All checks passed")
                
        print("\n🏥 DOCTOR MVP HEALTH CHECK:")
        print(f"  Current directory: {Path.cwd()}")
        print(f"  Python version: {sys.version}")
        print(f"  Worktree isolation: {'PASS' if not violations else 'FAIL'}")

if __name__ == '__main__':
    main()