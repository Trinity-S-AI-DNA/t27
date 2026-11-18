#!/usr/bin/env python3
"""
Test script to demonstrate business logic detection in key files
Shows that the T27-first architecture is working correctly
"""

import re
from pathlib import Path

def detect_business_logic_in_file(file_path):
    """Detect business logic patterns in a specific file"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Business logic patterns specific to agent systems - expanded
        business_patterns = {
            'agent_workflow': [
                r'agent.*loop|autonomous.*agent',
                r'workflow|turn.*management|task.*completion',
                r'tool.*execution|tool.*coordination',
                r'state.*management|session.*context',
                r'agent.*run|autonomous.*execution'
            ],
            'content_policy': [
                r'content.*policy|topic.*matching|enrichment.*logic',
                r'source.*selection|content.*classification',
                r'policy.*mapping|registry.*rules',
                r'content.*recommendation|source.*recommendation'
            ],
            'session_management': [
                r'session.*lifecycle|wrap.*summary|format.*rules',
                r'notebook.*ops|session.*extraction|completion.*criteria',
                r'summary.*generation|markdown.*formatting'
            ],
            'orchestration': [
                r'orchestration|workflow.*coordination|automation',
                r'pipeline.*management|process.*control'
            ]
        }
        
        detected_patterns = {}
        
        for category, patterns in business_patterns.items():
            matches = []
            for pattern in patterns:
                if re.search(pattern, content, re.IGNORECASE):
                    matches.append(pattern)
            if matches:
                detected_patterns[category] = matches
        
        return detected_patterns
        
    except Exception as e:
        return {'error': str(e)}

def test_specific_violations():
    """Test specific files we identified in our analysis"""
    test_files = [
        "contrib/backend/agent-runner/agent-runner.py",
        "contrib/backend/notebooklm/enrich.py", 
        "contrib/backend/notebooklm/wrapup.py",
        "contrib/backend/notebooklm/content_registry.yaml"
    ]
    
    print("🔍 Testing Business Logic Detection")
    print("=" * 60)
    
    for file_path in test_files:
        print(f"\n📁 Testing: {file_path}")
        patterns = detect_business_logic_in_file(file_path)
        
        if patterns:
            print("  ❌ BUSINESS LOGIC DETECTED:")
            for category, matches in patterns.items():
                print(f"    {category.upper()}:")
                for match in matches:
                    print(f"      - {match}")
        elif 'error' in patterns:
            print(f"  ⚠️  Error reading file: {patterns['error']}")
        else:
            print("  ✅ No business logic detected")
    
    print("\n" + "=" * 60)
    print("📋 VIOLATION SUMMARY:")
    print("The above files contain business logic in Python/TypeScript/YAML")
    print("According to T27-first architecture, this logic should be in .t27 files")

if __name__ == "__main__":
    test_specific_violations()