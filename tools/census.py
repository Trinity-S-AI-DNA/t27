#!/usr/bin/env python3
"""
Hand-written foreign line debt census tool for gHashTag/t27#9000.

Measures hand-written foreign lines by language using specs/policy/own_language.t27
classification, excluding generated paths (gen/, built site output, vendored deps).

Output format: one line per language with hand-written debt: "<count>\t<lang>\t-"
"""

import os
import sys
import subprocess
from pathlib import Path
from collections import defaultdict

# Language mappings from own_language.t27
LANGUAGES = {
    'rust': ['rs'],
    'python': ['py', 'pyi'],
    'typescript': ['ts', 'tsx'],
    'javascript': ['js', 'jsx', 'mjs', 'cjs'],
    'zig': ['zig'],
    'c': ['c', 'h'],
    'cpp': ['cc', 'cpp', 'cxx', 'hpp', 'hh'],
    'go': ['go'],
    'swift': ['swift'],
    'shell': ['sh', 'bash', 'zsh', 'fish', 'ps1'],
    'verilog': ['v', 'sv', 'svh', 'vh', 'vhd', 'vhdl'],
    'lean': ['lean'],
    'config': ['yml', 'yaml', 'toml'],
    'build': ['mk', 'cmake'],
}

# Excluded prefixes (generated, vendored, build output)
EXCLUDED_PREFIXES = [
    'vendor/', 'vendors/', 'third_party/', 'thirdparty/', 'external/', 'node_modules/',
    'dist/', 'build/', '_build/', '_site/', 'target/', 'gen/', 'bootstrap/gen/',
    'bootstrap/src/memory/generated/', 'ghashtag.github.io/'
]

# Build and test directories that contain generated code
EXCLUDED_DIRS = [
    'conformance/vectors/',
    'outputs/',
    'sim/build/',
    'fpga/build/',
    'contrib/railway/*/build/',
]

def count_lines_in_file(filepath):
    """Count lines in a file, skipping empty lines and comments."""
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            lines = 0
            for line in f:
                line = line.strip()
                # Skip empty lines and comments
                if line and not line.startswith('#'):
                    lines += 1
            return lines
    except (IOError, UnicodeDecodeError):
        return 0

def is_excluded_path(filepath):
    """Check if a path should be excluded from the census."""
    # Convert to forward slashes for consistency
    path_str = str(filepath).replace('\\', '/')
    
    # Check excluded prefixes
    for prefix in EXCLUDED_PREFIXES:
        if path_str.startswith(prefix):
            return True
    
    # Check excluded directories
    for dir_prefix in EXCLUDED_DIRS:
        if path_str.startswith(dir_prefix):
            return True
    
    return False

def get_language_from_extension(filepath):
    """Get language classification from file extension."""
    suffix = filepath.suffix.lower().lstrip('.')
    
    for lang, extensions in LANGUAGES.items():
        if suffix in extensions:
            return lang
    
    return None

def scan_repository(repo_path):
    """Scan repository for hand-written foreign code."""
    repo_path = Path(repo_path)
    language_counts = defaultdict(int)
    total_files = 0
    excluded_files = 0
    
    print(f"Scanning repository: {repo_path}")
    
    # Walk through all files in the repository
    for root, dirs, files in os.walk(repo_path):
        # Skip hidden directories and common build directories
        dirs[:] = [d for d in dirs if not d.startswith('.') and d not in ['node_modules', 'dist', 'build']]
        
        for file in files:
            filepath = Path(root) / file
            total_files += 1
            
            # Skip excluded paths
            if is_excluded_path(filepath):
                excluded_files += 1
                continue
            
            # Get language from extension
            language = get_language_from_extension(filepath)
            
            if language:
                lines = count_lines_in_file(filepath)
                if lines > 0:
                    language_counts[language] += lines
    
    print(f"Total files scanned: {total_files}")
    print(f"Excluded files: {excluded_files}")
    print(f"Languages with hand-written code: {len(language_counts)}")
    
    return language_counts

def format_census_output(language_counts):
    """Format census output as specified: "<count>\t<lang>\t-"""
    output_lines = []
    
    # Sort languages alphabetically for consistent output
    for lang in sorted(language_counts.keys()):
        count = language_counts[lang]
        output_lines.append(f"{count}\t{lang}\t-")
    
    return '\n'.join(output_lines)

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 census.py <repo_path>")
        sys.exit(1)
    
    repo_path = sys.argv[1]
    
    if not Path(repo_path).exists():
        print(f"Error: Repository path '{repo_path}' does not exist")
        sys.exit(1)
    
    # Scan the repository
    language_counts = scan_repository(repo_path)
    
    # Format and output results
    output = format_census_output(language_counts)
    print("\nCensus Results:")
    print(output)
    
    # Write to file for CI integration
    with open('foreign_lines_census.txt', 'w') as f:
        f.write(output)
    
    print(f"\nResults saved to foreign_lines_census.txt")

if __name__ == '__main__':
    main()