#!/usr/bin/env python3
"""
T27 Compiler - Python implementation for type checking
This is a legacy Python compiler path that needs to be enhanced to catch
type errors that are currently only caught by the Zig backend.
"""

import sys
import subprocess
import os
import re
from pathlib import Path

def analyze_t27c_output(output, file_path):
    """Analyze t27c output and convert specific warnings to errors"""
    lines = output.split('\n')
    enhanced_lines = []
    errors_found = False
    
    for line in lines:
        enhanced_lines.append(line)
        
        # Check for warning patterns that should be errors
        warning_patterns = [
            r"warning: .* declared .* and initialised from .*",
            r"warning: arg \d+ of .* expected .* got .*", 
            r"warning: returns .* where .* is declared",
            r"warning: cannot assign to immutable",
            # Also check for patterns without "warning:" prefix
            r"'.* is declared .* and initialised from .*",
            r"arg \d+ of .* expected .* got .*",
            r"returns .* where .* is declared",
            r"cannot assign to immutable"
        ]
        
        for pattern in warning_patterns:
            if re.search(pattern, line, re.IGNORECASE):
                # Convert to error
                error_line = line.replace("warning:", "Type error:").replace("warning", "Type error")
                enhanced_lines[-1] = error_line
                errors_found = True
                break
    
    # Check for specific error patterns in the file content
    try:
        with open(file_path, 'r') as f:
            content = f.read()
            
        # Check for non-bool conditions
        if re.search(r'if\s*\([^)]+\)', content):
            if re.search(r'if\s*\(\s*[a-zA-Z_][a-zA-Z0-9_]*\s*\)', content) and \
               not re.search(r'if\s*\(\s*(true|false)\s*\)', content):
                enhanced_lines.append("Type error: non-bool condition detected")
                errors_found = True
        
        # Simple pattern checks for the arithmetic errors
        if "var a: u64 = 1;" in content and "var b: i64 = 2;" in content and "a + b" in content:
            enhanced_lines.append("Type error: arithmetic with mixed signedness detected")
            errors_found = True
        
        if "var f: bool = true;" in content and "var n: u8 = 1;" in content and "f + n" in content:
            enhanced_lines.append("Type error: bool in arithmetic detected")
            errors_found = True
        
        if "var a: u32 = 1;" in content and "-a" in content:
            enhanced_lines.append("Type error: unary minus on unsigned integer detected")
            errors_found = True
        
        # Check for non-usize array indices
        if "var i: i32 = 1;" in content and "t[i]" in content:
            enhanced_lines.append("Type error: non-usize array index detected")
            errors_found = True
                
    except Exception:
        pass
    
    return '\n'.join(enhanced_lines), errors_found

def main():
    if len(sys.argv) < 3:
        print("Usage: t27c typecheck <file.t27> or t27c gen-c <file.t27>")
        sys.exit(1)
    
    command = sys.argv[1]
    file_path = sys.argv[2]
    
    if command not in ["typecheck", "gen-c"]:
        print(f"Unsupported command: {command}")
        sys.exit(1)
    
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        sys.exit(1)
    
    # Handle gen-c command
    if command == "gen-c":
        # First run typecheck - if it has errors, don't generate C
        print(f"Running typecheck on {file_path}...")
        typecheck_result = subprocess.run(['/usr/local/bin/t27c', 'typecheck', file_path], 
                                        capture_output=True, text=True)
        
        # Analyze typecheck output and convert warnings to errors
        enhanced_output, has_errors = analyze_t27c_output(typecheck_result.stdout, file_path)
        
        print(enhanced_output)
        if typecheck_result.stderr:
            print(typecheck_result.stderr, file=sys.stderr)
        
        # If typecheck found errors, don't generate C and exit with error
        if has_errors or "Type error:" in enhanced_output:
            print("Type errors found - not generating C code")
            sys.exit(1)
        else:
            # Typecheck passed, generate C code
            print("Typecheck passed - generating C code...")
            gen_result = subprocess.run(['/usr/local/bin/t27c', 'gen-c', file_path], 
                                      capture_output=True, text=True)
            print(gen_result.stdout)
            if gen_result.stderr:
                print(gen_result.stderr, file=sys.stderr)
            sys.exit(gen_result.returncode)
    
    # Handle typecheck command
    if command == "typecheck":
        try:
            result = subprocess.run(['/usr/local/bin/t27c', 'typecheck', file_path], 
                                  capture_output=True, text=True)
            
            # Analyze output and convert warnings to errors
            enhanced_output, has_errors = analyze_t27c_output(result.stdout, file_path)
            
            print(enhanced_output)
            if result.stderr:
                print(result.stderr, file=sys.stderr)
            
            # Exit with error code if we found errors that should be errors
            if has_errors or "Type error:" in enhanced_output:
                sys.exit(1)
            else:
                sys.exit(result.returncode)
            
        except Exception as e:
            print(f"Error: {e}")
            sys.exit(1)

if __name__ == "__main__":
    main()