#!/usr/bin/env python3
"""Test script for swarm monitoring functionality."""

import subprocess
import sys
import time
import os

def test_health_check():
    """Test health check functionality."""
    print("Testing health check...")
    result = subprocess.run([sys.executable, "scripts/tri_loop/swarm.py", "--health-check"], 
                          capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✓ Health check passed (exit code 0)")
        print(f"  Output: {result.stdout.strip()}")
        return True
    else:
        print(f"✗ Health check failed (exit code {result.returncode})")
        print(f"  Output: {result.stdout.strip()}")
        print(f"  Error: {result.stderr.strip()}")
        return False

def test_simulate_failure():
    """Test simulate failure functionality."""
    print("\nTesting simulate failure...")
    result = subprocess.run([sys.executable, "scripts/tri_loop/swarm.py", "--simulate-failure"], 
                          capture_output=True, text=True)
    
    # Should return exit code 3 (restart failed) due to missing credentials
    if result.returncode == 3:
        print("✓ Simulate failure correctly detected missing credentials (exit code 3)")
        print(f"  Output: {result.stdout.strip()}")
        return True
    else:
        print(f"✗ Simulate failure unexpected exit code {result.returncode}")
        print(f"  Output: {result.stdout.strip()}")
        print(f"  Error: {result.stderr.strip()}")
        return False

def test_status_functionality():
    """Test original status functionality."""
    print("\nTesting status functionality...")
    result = subprocess.run([sys.executable, "scripts/tri_loop/swarm.py", "--json"], 
                          capture_output=True, text=True)
    
    if result.returncode == 0:
        try:
            import json
            data = json.loads(result.stdout)
            if "status" in data:
                print("✓ Status functionality works and returns valid JSON")
                print(f"  Swarm state: {data['status'].get('swarmState', 'unknown')}")
                return True
            else:
                print("✗ Status JSON missing 'status' field")
                return False
        except json.JSONDecodeError:
            print("✗ Status output is not valid JSON")
            return False
    else:
        print(f"✗ Status functionality failed (exit code {result.returncode})")
        return False

def test_monitor_help():
    """Test monitor help functionality."""
    print("\nTesting monitor help...")
    result = subprocess.run([sys.executable, "scripts/tri_loop/swarm.py", "--monitor", "--help"], 
                          capture_output=True, text=True)
    
    if result.returncode == 0 and "continuous monitoring" in result.stdout:
        print("✓ Monitor help is available")
        return True
    else:
        print(f"✗ Monitor help failed (exit code {result.returncode})")
        return False

def test_restart_without_credentials():
    """Test restart without credentials."""
    print("\nTesting restart without credentials...")
    result = subprocess.run([sys.executable, "scripts/tri_loop/swarm.py", "--restart"], 
                          capture_output=True, text=True)
    
    # Should fail due to missing credentials
    if result.returncode != 0 and "credentials" in result.stdout.lower():
        print("✓ Restart correctly fails without credentials")
        return True
    else:
        print(f"✗ Restart behavior unexpected (exit code {result.returncode})")
        return False

def main():
    """Run all tests."""
    print("Running swarm monitoring tests...\n")
    
    tests = [
        test_health_check,
        test_simulate_failure,
        test_status_functionality,
        test_monitor_help,
        test_restart_without_credentials,
    ]
    
    passed = 0
    total = len(tests)
    
    for test in tests:
        if test():
            passed += 1
    
    print(f"\n=== Test Results ===")
    print(f"Passed: {passed}/{total}")
    
    if passed == total:
        print("✓ All tests passed!")
        return 0
    else:
        print("✗ Some tests failed!")
        return 1

if __name__ == "__main__":
    sys.exit(main())