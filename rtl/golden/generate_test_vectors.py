#!/usr/bin/env python3
"""
Generate comprehensive test vectors for 16×16 fixed-point dot product.
"""

import sys
import json
import os
from typing import List

# Import the golden model functions
sys.path.append(os.path.dirname(__file__))
from golden_dotproduct import dot_product_golden, test_vector


def test_case_1_basic():
    """Test 1: Basic case - A = [0x0080]×16, B = [0x0080]×16 → accum = 0x00000400"""
    # Q7.8(0.5) = 0x0080
    A = [0x0080] * 16  # All 0.5 values
    B = [0x0080] * 16  # All 0.5 values
    return {"name": "Basic case - All 0.5 values", "A": A, "B": B, "expected_accum": 0x00000400, 
            "expected_hex": "0x00000400", "explanation": "Each product = 0.5×0.5=0.25, total=16×0.25=4.0, result=1024=0x0400"}


def test_case_2_zeros():
    """Test 2: Zero vectors"""
    A = [0x0000] * 16  # All 0.0 values
    B = [0x0000] * 16  # All 0.0 values
    return {"name": "Zero vectors", "A": A, "B": B, "expected_accum": 0x00000000, 
            "expected_hex": "0x00000000", "explanation": "All products are zero, total=0"}


def test_case_3_max_positive():
    """Test 3: Maximum positive values"""
    # Q7.8(0.99609375) = 0x00FF (max positive Q7.8 value)
    A = [0x00FF] * 16
    B = [0x00FF] * 16
    expected_accum = 0x00FF * 0x00FF * 16 >> 8  # Calculate expected
    return {"name": "Max positive values", "A": A, "B": B, "expected_accum": expected_accum,
            "expected_hex": f"0{expected_accum:08x}", "explanation": f"Max Q7.8 values: each={0x00FF/256:.6f}, product={(0x00FF/256)**2:.6f}, total={16*(0x00FF/256)**2:.6f}"}


def test_case_4_mixed():
    """Test 4: Mixed positive/negative values"""
    A = [0x0080, 0x00FF, 0x0000, 0xFF80, 0x0080, 0x00FF, 0x0000, 0xFF80,
         0x0080, 0x00FF, 0x0000, 0xFF80, 0x0080, 0x00FF, 0x0000, 0xFF80]
    B = [0x0080, 0x0080, 0x0080, 0x0080, 0xFF80, 0xFF80, 0xFF80, 0xFF80,
         0x00FF, 0x00FF, 0x00FF, 0x00FF, 0x0000, 0x0000, 0x0000, 0x0000]
    return {"name": "Mixed positive/negative values", "A": A, "B": B, "expected_accum": None,
            "expected_hex": "N/A", "explanation": "Combination of positive (0.5, 0.996), negative (-0.5), and zero values"}


def test_case_5_min_negative():
    """Test 5: Minimum negative values"""
    # Q7.8(-128.0) = 0x8000 (most negative Q7.8 value)
    A = [0x8000] * 16
    B = [0x8000] * 16
    expected_accum = 0x8000 * 0x8000 * 16 >> 8
    return {"name": "Min negative values", "A": A, "B": B, "expected_accum": expected_accum,
            "expected_hex": f"0{expected_accum:08x}", "explanation": f"Most negative Q7.8 values: each=-128.0, product=16384, total=262144, result={expected_accum}"}


def generate_all_test_vectors():
    """Generate all test vectors"""
    test_cases = [
        test_case_1_basic(),
        test_case_2_zeros(),
        test_case_3_max_positive(),
        test_case_4_mixed(),
        test_case_5_min_negative(),
    ]
    
    results = []
    for i, tc in enumerate(test_cases):
        print(f"=== Test Case {i+1}: {tc['name']} ===")
        
        # Generate the test vector using the golden model
        tv = test_vector(tc['A'], tc['B'])
        
        # Check if the result matches expectation
        actual_accum = tv['accum']
        expected_accum = tc['expected_accum']
        
        if expected_accum is not None:
            match = actual_accum == expected_accum
            print(f"Expected: {tc['expected_hex']}")
            print(f"Actual:   0x{actual_accum & 0xFFFFFFFF:08x}")
            print(f"Match:    {'✓' if match else '✗'}")
        else:
            print(f"Result:   0x{actual_accum & 0xFFFFFFFF:08x}")
            print("Expected: N/A (mixed values)")
        
        print(f"Explanation: {tc['explanation']}")
        print()
        
        # Add metadata to the result
        tv['test_case_info'] = tc
        results.append(tv)
    
    return results


def main():
    """Main function to generate test vectors and save to JSON"""
    print("Generating comprehensive test vectors for 16×16 fixed-point dot product...")
    print("=" * 70)
    
    # Generate all test vectors
    test_vectors = generate_all_test_vectors()
    
    # Save to JSON file
    output_file = "/tmp/m3_test_vectors.json"
    with open(output_file, "w") as f:
        json.dump(test_vectors, f, indent=2)
    
    print("=" * 70)
    print(f"Generated {len(test_vectors)} test vectors.")
    print(f"Saved to: {output_file}")
    print()
    print("Test vector format:")
    print("- A: 16 × 16-bit Q7.8 values (hex)")
    print("- B: 16 × 16-bit Q7.8 values (hex)")
    print("- accum: 32-bit signed dot product result")
    print("- uart_bytes: 4 bytes LSB-first for UART transmission")
    print("- uart_info: baud rate and error information")
    print("- test_case_info: metadata about the test case")
    
    # Also print a summary of the Q7.8 format
    print("\n" + "=" * 70)
    print("Q7.8 Fixed-Point Format Documentation:")
    print("=" * 70)
    print("Q7.8 Format:")
    print("- 16-bit signed fixed-point format")
    print("- 7 integer bits + 8 fractional bits")
    print("- Range: -128.0 to 127.99609375")
    print("- Resolution: 2^-8 = 1/256 ≈ 0.00390625")
    print()
    print("Conversion:")
    print("- float_to_q78(f): round(f * 256) & 0xFFFF")
    print("- q78_to_float(q78): q78 / 256.0 (with sign extension)")
    print()
    print("Dot Product Algorithm:")
    print("1. Each element is Q7.8 (16-bit signed)")
    print("2. Product of two elements: Q7.8 × Q7.8 = Q15.16 (32-bit)")
    print("3. Sum of 16 products: Q15.16 × 16 = Q19.20 (but uses 40-bit accumulator)")
    print("4. Shift right by 8 to fit in 32-bit: Q31.31 → Q23.23")
    print("5. Result is 32-bit signed integer representing the fixed-point value")
    print()
    print("Mathematical equivalent:")
    print("accum = Σ(A[i] × B[i]) / 256.0 (shift by 8 is division by 256)")


if __name__ == "__main__":
    main()