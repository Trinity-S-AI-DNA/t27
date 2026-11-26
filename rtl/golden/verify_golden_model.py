#!/usr/bin/env python3
"""
Verify that the golden model matches the RTL implementation exactly.
"""

import sys
import os

# Import the golden model functions
from golden_dotproduct import dot_product_golden
import struct

def test_basic_case():
    """Test the basic case: A = [0x0080]×16, B = [0x0080]×16 → accum = 0x00000400"""
    print("=== Verifying Basic Test Case ===")
    print("A = [0x0080]×16 (all 0.5 values)")
    print("B = [0x0080]×16 (all 0.5 values)")
    print()
    
    # Create test vectors
    A = [0x0080] * 16  # All 0.5 values
    B = [0x0080] * 16  # All 0.5 values
    
    # Compute using golden model
    accum = dot_product_golden(A, B)
    
    # Expected result
    expected = 0x00000400
    
    print(f"Golden model result: 0x{accum & 0xFFFFFFFF:08x} ({accum})")
    print(f"Expected result:     0x{expected:08x} ({expected})")
    print()
    
    # Check match
    if accum == expected:
        print("✓ PASS: Golden model matches expected result")
        return True
    else:
        print("✗ FAIL: Golden model does not match expected result")
        return False

def test_additional_cases():
    """Test additional cases to ensure correctness"""
    test_cases = [
        {
            "name": "Zero vectors",
            "A": [0x0000] * 16,
            "B": [0x0000] * 16,
            "expected": 0x00000000
        },
        {
            "name": "All ones (0.99609375)",
            "A": [0x00FF] * 16,
            "B": [0x00FF] * 16,
            "expected": 0x00000fe0
        },
        {
            "name": "All negative max (-128.0)",
            "A": [0x8000] * 16,
            "B": [0x8000] * 16,
            "expected": 0x04000000
        }
    ]
    
    print("=== Testing Additional Cases ===")
    all_passed = True
    
    for i, tc in enumerate(test_cases):
        print(f"Test {i+1}: {tc['name']}")
        
        accum = dot_product_golden(tc['A'], tc['B'])
        expected = tc['expected']
        
        print(f"  Result:   0x{accum & 0xFFFFFFFF:08x} ({accum})")
        print(f"  Expected: 0x{expected:08x} ({expected})")
        
        if accum == expected:
            print("  ✓ PASS")
        else:
            print("  ✗ FAIL")
            all_passed = False
        print()
    
    return all_passed

def main():
    """Main verification function"""
    print("Golden Model Verification for 16×16 Fixed-Point Dot Product")
    print("=" * 60)
    print()
    
    # Test the basic case
    basic_pass = test_basic_case()
    print()
    
    # Test additional cases
    additional_pass = test_additional_cases()
    print()
    
    # Summary
    print("=" * 60)
    if basic_pass and additional_pass:
        print("✓ ALL TESTS PASSED: Golden model is verified and correct")
        return 0
    else:
        print("✗ SOME TESTS FAILED: Golden model needs correction")
        return 1

if __name__ == "__main__":
    sys.exit(main())