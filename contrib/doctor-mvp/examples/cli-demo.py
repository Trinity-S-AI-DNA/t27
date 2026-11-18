#!/usr/bin/env python3
"""
Doctor MVP CLI Demo Script
Demonstrates the CLI functionality with sample data
"""

import subprocess
import json
import sys
from pathlib import Path

def run_command(cmd, description):
    """Run a command and display results"""
    print(f"\n{'='*50}")
    print(f"Demo: {description}")
    print(f"Command: {cmd}")
    print(f"{'='*50}")
    
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd="contrib/doctor-mvp")
        print("STDOUT:", result.stdout)
        if result.stderr:
            print("STDERR:", result.stderr)
        print("Return code:", result.returncode)
    except Exception as e:
        print(f"Error running command: {e}")

def main():
    print("Doctor MVP CLI Demo")
    print("Testing incident detection and classification functionality")
    
    # Navigate to the doctor directory
    doctor_dir = Path("contrib/doctor-mvp")
    if not doctor_dir.exists():
        print("Error: contrib/doctor-mvp directory not found")
        sys.exit(1)
    
    # Demo 1: Scan fixtures
    run_command(
        "python3 cli/doctor.py scan --input fixtures/",
        "Scan fixtures for incidents"
    )
    
    # Demo 2: Scan specific file
    run_command(
        "python3 cli/doctor.py scan --input fixtures/sample-chat-export.json",
        "Scan chat export file"
    )
    
    # Demo 3: Scan with output file
    run_command(
        "python3 cli/doctor.py scan --input fixtures/ --output output/incidents.json",
        "Scan and save incidents to file"
    )
    
    # Demo 4: Triage incidents
    run_command(
        "python3 cli/doctor.py triage --input fixtures/sample-chat-export.json",
        "Classify incidents using triage rules"
    )
    
    # Demo 5: Generate repair brief
    run_command(
        "python3 cli/doctor.py handoff --incident INC-2026-10-30-001",
        "Create repair brief for sample incident"
    )
    
    # Demo 6: Verify incident
    run_command(
        "python3 cli/doctor.py verify --incident INC-2026-10-30-001",
        "Verify incident resolution"
    )
    
    # Demo 7: Generate report
    run_command(
        "python3 cli/doctor.py report --input fixtures/sample-chat-export.json --format md",
        "Generate markdown report"
    )
    
    print("\n" + "="*50)
    print("Demo completed! Check the output files for results.")
    print("="*50)

if __name__ == "__main__":
    main()