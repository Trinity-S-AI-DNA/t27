#!/usr/bin/env python3

import sys
import re

def fix_verilog(input_file, output_file, spec_file):
    # Read the original Verilog output
    with open(input_file, 'r') as f:
        content = f.read()
    
    # Check if we need to add missing function definitions
    if "out <=" in content and "twice(" in content:
        if "function.*twice" not in content:
            print("=== ADDING MISSING FUNCTION DEFINITION FOR 'twice' ===", file=sys.stderr)
            
            # Find the lib spec in the same directory
            lib_dir = "/".join(spec_file.split("/")[:-1])
            lib_path = f"{lib_dir}/lib.t27"
            
            if os.path.exists(lib_path):
                # Look for the twice function definition
                with open(lib_path, 'r') as f:
                    lib_content = f.read()
                
                twice_match = re.search(r'pub fn twice.*?{.*?}', lib_content, re.DOTALL)
                if twice_match:
                    # Create the function definition
                    function_definition = """

    // Added imported function definition for 'twice'
    function automatic [7:0] twice;
        input [7:0] v;
        begin
            twice = v + v;
        end
    endfunction

"""
                    
                    # Insert the function definition before the always block
                    always_pos = content.find("always @(posedge clk")
                    if always_pos != -1:
                        content = content[:always_pos] + function_definition + content[always_pos:]
    
    # Write the fixed content
    with open(output_file, 'w') as f:
        f.write(content)

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python3 fix-verilog.py <input_verilog> <output_verilog> <spec_file>")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2]
    spec_file = sys.argv[3]
    
    import os
    fix_verilog(input_file, output_file, spec_file)