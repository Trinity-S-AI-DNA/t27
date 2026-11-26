#!/usr/bin/env python3
"""
Golden model for lamb_labs 16×16 fixed-point dot product with UART output.

Computes: accum = sum_{i=0}^{15} A[i] * B[i]
where A[i], B[i] are Q7.8 signed fixed-point (16-bit values).
The 32-bit accumulated result is then serialized LSB-first for UART transmission.

Fixed-point format:
  - Operands: Q7.8  (7 integer bits + 8 fractional bits, range -128 to 127.996...)
  - Product:   Q15.16 (from Q7.8 × Q7.8)
  - Accumulator: Q31.31 (sum of 16 Q15.16 products, shifted right by 8 to fit 32-bit)
  - UART output: 32-bit result LSB-first as 4 bytes (each 8 bits)

This golden model matches the mac16 + lamb_labs_top Verilog implementation.
"""

import struct
import sys
from typing import List


def q78_to_float(q78: int) -> float:
    """Convert signed Q7.8 integer to floating-point."""
    # Q7.8: bit 15 is sign, bits 14..8 are integer part, bits 7..0 are fractional
    # Range: -128.0 to 127.99609375
    if q78 & 0x8000:
        # Negative: two's complement
        val = q78 - 0x10000
    else:
        val = q78
    # Scale by 2^-8 = 1/256
    return val / 256.0


def float_to_q78(f: float) -> int:
    """Convert floating-point to signed Q7.8 integer."""
    # Clamp to representable range
    if f > 127.99609375:
        f = 127.99609375
    elif f < -128.0:
        f = -128.0
    # Multiply by 256 and round to nearest integer
    iv = int(round(f * 256.0))
    # Ensure 16-bit signed range
    iv = iv & 0xFFFF
    if iv >= 0x8000:
        iv -= 0x10000
    return iv


def dot_product_golden(A: List[int], B: List[int]) -> int:
    """
    Compute dot product of two 16-element vectors in Q7.8 fixed-point.
    
    Returns 32-bit accumulated result (Q31.31 format, shifted/rounded to fit).
    
    Algorithm: Matches RTL implementation exactly
      1. For each i, multiply A[i] × B[i] as signed 16×16 → 32-bit (using $signed)
      2. Sum all 16 products (Python arbitrary precision, no overflow)
      3. Arithmetic shift right by 8 to fit into 32-bit Q31.31 format
      4. Return result as signed 32-bit integer
    """
    assert len(A) == 16, f"Expected 16 elements in A, got {len(A)}"
    assert len(B) == 16, f"Expected 16 elements in B, got {len(B)}"

    # Step 1: compute all 16 products, sum them
    # Use two's complement conversion to match RTL $signed behavior
    total = 0
    for i in range(16):
        # Convert to two's complement signed integers (matches RTL $signed)
        a_signed = A[i] if A[i] <= 0x7FFF else A[i] - 0x10000
        b_signed = B[i] if B[i] <= 0x7FFF else B[i] - 0x10000
        product = a_signed * b_signed  # 32-bit signed product (Python int)
        total += product

    # Step 2: arithmetic shift right by 8 to match RTL "sum >>> 8"
    # Arithmetic shift preserves sign, which matches RTL behavior
    # This is equivalent to integer division by 256 with sign preservation
    if total >= 0:
        shifted = total >> 8
    else:
        # For negative numbers, arithmetic shift right preserves sign
        # In Python, >> is already arithmetic for signed integers
        shifted = total >> 8

    # Step 3: ensure result fits in 32-bit signed range
    # The shift should keep it in range, but we clamp just in case
    if shifted > 0x7FFFFFFF:
        result = 0x7FFFFFFF
    elif shifted < -0x80000000:
        result = -0x80000000
    else:
        result = shifted

    return result


def result_to_uart_bytes(result: int) -> bytes:
    """
    Format the 32-bit dot product result for UART transmission.
    
    The lamb_labs_top module sends 4 bytes LSB-first:
      byte 0 = accum[7:0]   (least significant byte)
      byte 1 = accum[15:8]
      byte 2 = accum[23:16]
      byte 3 = accum[31:24]  (most significant byte)
    
    UART TX sends LSB first within each byte, and the bytes are sent
    in order byte0, byte1, byte2, byte3 (LSB-first overall).
    """
    # Convert to unsigned for struct.pack, then pack as 4 bytes LSB-first
    # struct.pack('<I', result & 0xFFFFFFFF) gives bytes little-endian
    # But we want: byte0 = bits 7:0, byte1 = bits 15:8, etc.
    # Little-endian packing gives exactly that: byte0 is LSB, byte3 is MSB
    result_unsigned = result & 0xFFFFFFFF
    return struct.pack('<I', result_unsigned)


def simulate_uart_transmit(bytes_sent: bytes, baud_div: int = 583,
                           clk_freq: int = 67160000) -> dict:
    """
    Simulate UART transmission and return timing info.
    
    For golden model verification: we just confirm the byte sequence
    that would be transmitted, not actual timing simulation.
    """
    # Verify the bytes are valid (4 bytes, each 0-255)
    assert len(bytes_sent) == 4, f"Expected 4 UART bytes, got {len(bytes_sent)}"
    for i, b in enumerate(bytes_sent):
        assert 0 <= b <= 255, f"Byte {i} out of range: {b}"

    # Compute effective baud rate
    actual_baud = clk_freq / baud_div
    return {
        "bytes_transmitted": [f"0x{b:02x}" for b in bytes_sent],
        "baud_rate_nominal": baud_div,
        "actual_baud": round(actual_baud),
        "baud_error_ppm": round((actual_baud - 115200) / 115200 * 1e6)
    }


def test_vector(A: List[int], B: List[int]) -> dict:
    """
    Generate a test vector: given A and B vectors, compute the expected
    dot product and UART byte sequence.
    
    Returns dict with:
      - A_hex: hex of A elements
      - B_hex: hex of B elements
      - accum: 32-bit signed result
      - uart_bytes: 4-byte sequence LSB-first
      - uart_info: UART transmission info
    """
    accum = dot_product_golden(A, B)
    uart_bytes = result_to_uart_bytes(accum)
    uart_info = simulate_uart_transmit(uart_bytes)

    return {
        "A_hex": [f"0x{a:04x}" for a in A],
        "B_hex": [f"0x{b:04x}" for b in B],
        "accum": accum,
        "uart_bytes": [f"0x{b:02x}" for b in uart_bytes],
        "uart_info": uart_info
    }


# -------------------------------------------------------------------------
# Example test vectors (fixed Q7.8 values)
# -------------------------------------------------------------------------

# Test 1: Simple case — all ones × all ones
# Q7.8(1.0) = 0x0100 = 256, but 16-bit Q7.8 max is 0x00FF for 0.996...
# Actually Q7.8(1.0) = 256 decimal = 0x0100 but that's 17 bits...
# Let's use: Q7.8(0.5) = 0x0080 = 128 decimal
# Q7.8(0.99609375) = 0x00FF = 255 decimal
# Dot product of 16 × 0.5 × 0.5 = 16 × 0.25 = 6.0 in floating point
# In Q31.31: 6.0 × 2^31 = 6 × 2147483648 = 12884901888 → but we shift by 8
# Let me just compute: 16 products of (128 × 128) = 16 × 16384 = 262144
# Then shift right by 8: 262144 >> 8 = 1024 = 0x400 → but that's only 11 bits
# Hmm, let me reconsider. Q7.8 0x0080 = 128/256 = 0.5. Product = 0.25.
# 16 × 0.25 = 4.0. In Q31.31: 4.0 × 2^31 = 8589934592. Shift right by 8 → 4 × 2^25 = 134217728 = 0x8000000
# Wait, I'm confusing myself. Let me just use the Python function.

# Simpler: use small integers that represent Q7.8 values directly.
# Q7.8 format: the integer value stored is the mathematical value × 256.
# So Q7.8(0.5) = round(0.5 × 256) = 128 = 0x0080

# Test vector 1: A = [0.5, 0.5, ..., 0.5] (16 times), B = [0.5, 0.5, ..., 0.5] (16 times)
# Each product = 0.5 × 0.5 = 0.25
# Total = 16 × 0.25 = 4.0
# In Q31.31: 4.0 → the 32-bit value is (4.0 × 2^31) >> 8 = 4 × 2^23 = 33554432 = 0x02000000
# Wait, let me re-derive. The golden model computes:
#   total = sum(A[i]*B[i])  — Python real numbers, so total = 4.0
#   shifted = total >> 8  — but total is a float-like Python int? No, total is exact.
# Actually, A and B are ints in Q7.8 representation. So:
#   A[i] = 128 (representing 0.5), B[i] = 128
#   product = 128 × 128 = 16384 (this is the mathematical product in "Q7.8 squared" = Q15.16)
#   total = 16 × 16384 = 262144
#   shifted = 262144 >> 8 = 1024
#   result = 1024 = 0x0400
# This matches the expected result 0x00000400 (1024 in decimal)
# In Q31.31 format, this represents 1024/2^23 = 0.001220703125

# Test vector 2: A = [1.0, 0, 0, ..., 0], B = [1.0, 0, 0, ..., 0]
# But Q7.8(1.0) doesn't fit in 16 bits signed Q7.8 (max is 127.996...)
# Use 0.99609375 = 0x00FF instead.

# Actually, let me just run the script and see what it produces.

# -------------------------------------------------------------------------
# Main: generate and print test vectors
# -------------------------------------------------------------------------

if __name__ == "__main__":
    # Parse optional command-line test vector indices
    tv_indices = None
    if len(sys.argv) > 1:
        try:
            idx = int(sys.argv[1])
            tv_indices = [idx]
        except ValueError:
            # Parse comma-separated indices
            tv_indices = [int(x) for x in sys.argv[1].split(',')]

    # Define some Q7.8 test values
    # Q7.8(0.0) = 0x0000
    # Q7.8(0.5) = 0x0080
    # Q7.8(0.99609375) = 0x00FF (max positive)
    # Q7.8(-0.5) = 0xFF80
    # Q7.8(-1.0) doesn't exist; max negative is -128.0 = 0x8000

    test_values = [
        0x0000,  # 0.0
        0x0080,  # 0.5
        0x00FF,  # 0.99609375
        0xFF80,  # -0.5
        0x8000,  # -128.0 (most negative)
    ]

    # Generate test vectors
    generated = []
    if tv_indices is None:
        # Generate all combinations of 2 values from test_values (simple set)
        for a_val in test_values[:3]:  # limit for brevity
            for b_val in test_values[:3]:
                A = [a_val] * 16
                B = [b_val] * 16
                generated.append(test_vector(A, B))
    else:
        for idx in tv_indices:
            # Use index to pick values
            a_val = test_values[idx % len(test_values)]
            b_val = test_values[(idx + 1) % len(test_values)]
            A = [a_val] * 16
            B = [b_val] * 16
            generated.append(test_vector(A, B))

    # Print results as JSON-compatible dicts (for Verilator test integration)
    import json
    for i, tv in enumerate(generated):
        print(f"=== Test Vector {i+1} ===")
        print(f"A = {tv['A_hex']}")
        print(f"B = {tv['B_hex']}")
        print(f"accum (signed 32-bit) = {tv['accum']} = 0x{tv['accum'] & 0xFFFFFFFF:08x}")
        print(f"UART bytes LSB-first = {tv['uart_bytes']}")
        print(f"UART info: {tv['uart_info']}")
        print()

    # Also save to file for Verilator test harness
    with open("/tmp/lamb_labs_test_vectors.json", "w") as f:
        json.dump(generated, f, indent=2)
    print(f"Saved {len(generated)} test vectors to /tmp/lamb_labs_test_vectors.json")

    # Print summary
    print(f"Generated {len(generated)} test vectors.")
    print("Each test vector has:")
    print("  - A: 16 × 16-bit Q7.8 values (hex)")
    print("  - B: 16 × 16-bit Q7.8 values (hex)")
    print("  - accum: 32-bit signed dot product result")
    print("  - uart_bytes: 4 bytes LSB-first for UART transmission")
    print("  - uart_info: baud rate and error information")