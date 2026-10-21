#!/usr/bin/env python3
"""
Enhanced Format Command for tri CLI
This script provides code formatting for .t27 specifications.
"""

import argparse
import re
import sys
from pathlib import Path
from typing import List, Dict, Any

class T27Formatter:
    """Main formatter class for .t27 files."""

    def __init__(self):
        self.indent_size = 2
        self.max_line_length = 80

    def format_content(self, content: str) -> str:
        """Format the content of a .t27 file."""
        lines = content.split('\n')
        formatted_lines = []
        indent_level = 0

        i = 0
        while i < len(lines):
            line = lines[i]
            stripped = line.strip()

            # Skip empty lines (keep one)
            if not stripped:
                # Check next line is not also empty
                if i + 1 < len(lines) and lines[i + 1].strip():
                    formatted_lines.append('')
                i += 1
                continue

            # Handle different types of lines
            if stripped.startswith('// SPDX-License-Identifier:'):
                # License header - keep at top with no indent
                formatted_lines.append(stripped)
                # Add blank line after license
                formatted_lines.append('')
                i += 1
            elif stripped.startswith(';'):
                # Comment line
                # Check if it's a module declaration
                if 'module' in stripped:
                    formatted_lines.append(stripped)
                else:
                    # Indent comment
                    formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif stripped.startswith('pub const'):
                # Constant declaration
                formatted_lines.append(self._format_constant(stripped, indent_level))
                i += 1
            elif stripped.startswith('pub const '):
                # Multiple constants (rare)
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif stripped.startswith('pub fn '):
                # Function declaration
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif stripped.startswith('test '):
                # Test declaration
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif stripped.startswith('invariant '):
                # Invariant declaration
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif stripped.startswith('module '):
                # Module declaration
                formatted_lines.append(stripped)
                i += 1
            elif stripped.startswith('}'):
                # Closing brace
                indent_level = max(0, indent_level - self.indent_size)
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif stripped.startswith('{'):
                # Opening brace
                formatted_lines.append(' ' * indent_level + stripped)
                indent_level += self.indent_size
                i += 1
            elif stripped.startswith(('if ', 'for ', 'while ', 'unless ')):
                # Control structures
                formatted_lines.append(' ' * indent_level + stripped)
                if not stripped.endswith('{'):
                    # Add opening brace on next line
                    i += 1
                    indent_level += self.indent_size
                    formatted_lines.append(' ' * indent_level + '{')
                else:
                    i += 1
                    indent_level += self.indent_size
            elif stripped.endswith('}'):
                # Closing brace on same line
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif stripped.startswith(('return ', 'break ', 'continue ')):
                # Control statements
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif stripped.startswith(('const ', 'var ', 'let ')):
                # Variable declarations
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif ':' in stripped and ('struct' in stripped or 'enum' in stripped):
                # Struct/enum declarations
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1
            elif '=' in stripped and not any(x in stripped for x in ['//', '/*']):
                # Assignment
                formatted_lines.append(self._format_assignment(stripped, indent_level))
                i += 1
            else:
                # Other statements
                formatted_lines.append(' ' * indent_level + stripped)
                i += 1

        # Join lines and add trailing newline
        result = '\n'.join(formatted_lines)
        if not result.endswith('\n'):
            result += '\n'

        return result

    def _format_constant(self, line: str, indent: int) -> str:
        """Format constant declaration."""
        # Add some padding around =
        parts = line.split('=', 1)
        if len(parts) == 2:
            name = parts[0].strip()
            value = parts[1].strip()
            return f"{' ' * indent}{name} = {value}"
        return f"{' ' * indent}{line}"

    def _format_assignment(self, line: str, indent: int) -> str:
        """Format assignment statement with proper spacing."""
        # Add space around =
        parts = line.split('=', 1)
        if len(parts) == 2:
            left = parts[0].strip()
            right = parts[1].strip()
            # Check for comments
            if '//' in right:
                comment = right.split('//')[1].strip()
                right = right.split('//')[0].strip()
                return f"{' ' * indent}{left} = {right} // {comment}"
            return f"{' ' * indent}{left} = {right}"
        return f"{' ' * indent}{line}"

    def format_file(self, filepath: Path, check: bool = False, diff: bool = False) -> Dict[str, Any]:
        """Format a single file."""
        if not filepath.suffix == '.t27':
            return {"error": "Not a .t27 file"}

        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                original_content = f.read()
        except Exception as e:
            return {"error": f"Could not read file: {e}"}

        formatted_content = self.format_content(original_content)

        if check:
            if original_content == formatted_content:
                return {"changed": False}
            else:
                if diff:
                    return {
                        "changed": True,
                        "diff": self._generate_diff(original_content, formatted_content)
                    }
                return {"changed": True}

        if original_content != formatted_content:
            if check:
                return {"changed": True}
            else:
                # Write formatted content
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(formatted_content)
                return {"changed": True}

        return {"changed": False}

    def _generate_diff(self, original: str, formatted: str) -> str:
        """Generate a diff between original and formatted content."""
        original_lines = original.split('\n')
        formatted_lines = formatted.split('\n')

        diff_lines = []
        diff_lines.append("--- Original")
        diff_lines.append("+++ Formatted")

        for i, (orig_line, fmt_line) in enumerate(zip(original_lines, formatted_lines)):
            if orig_line != fmt_line:
                diff_lines.append(f"- {orig_line}")
                diff_lines.append(f"+ {fmt_line}")

        return '\n'.join(diff_lines)

def print_diff(diff: str):
    """Print diff in a readable format."""
    print("📋 Code Diff:")
    print("-" * 40)
    for line in diff.split('\n'):
        if line.startswith('-'):
            print(f"\033[31m{line}\033[0m")  # Red for removed
        elif line.startswith('+'):
            print(f"\033[32m{line}\033[0m")  # Green for added
        else:
            print(line)

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description='Enhanced formatter for .t27 specifications')
    parser.add_argument('paths', nargs='*', default=['./specs'],
                       help='Paths to format (default: ./specs)')
    parser.add_argument('--check', '-c', action='store_true',
                       help='Check if files need formatting (don\'t modify)')
    parser.add_argument('--diff', action='store_true',
                       help='Show diff when --check is used')
    parser.add_argument('--verbose', '-v', action='store_true',
                       help='Show verbose output')

    args = parser.parse_args()

    formatter = T27Formatter()
    results = []
    total_files = 0
    total_changed = 0

    for path in args.paths:
        path_obj = Path(path)
        if not path_obj.exists():
            print(f"❌ Path does not exist: {path}")
            continue

        if path_obj.is_file():
            total_files += 1
            result = formatter.format_file(path_obj, args.check, args.diff)
            results.append((str(path_obj), result))

            if result.get("changed", False):
                total_changed += 1
                if args.verbose:
                    print(f"🔄 Would format: {path}")
            elif args.verbose:
                print(f"✅ Already formatted: {path}")

        elif path_obj.is_dir():
            # Process all .t27 files in directory
            for t27_file in path_obj.rglob("*.t27"):
                total_files += 1
                result = formatter.format_file(t27_file, args.check, args.diff)
                results.append((str(t27_file), result))

                if result.get("changed", False):
                    total_changed += 1
                    if args.verbose:
                        print(f"🔄 Would format: {t27_file}")
                elif args.verbose:
                    print(f"✅ Already formatted: {t27_file}")

    # Print summary
    if not args.diff:
        print(f"\n📊 Format Results Summary")
        print(f"Files checked: {total_files}")
        print(f"Files needing formatting: {total_changed}")

        if total_changed > 0 and not args.check:
            print(f"✅ Formatted {total_changed} files")
        elif total_changed > 0:
            print(f"⚠️  {total_changed} files need formatting")
            if args.check:
                print("   Use without --check to format files")

    # Print diffs
    for filepath, result in results:
        if result.get("diff"):
            print(f"\n{filepath}:")
            print_diff(result["diff"])

    # Exit with error code if files need formatting and --check is used
    if args.check and total_changed > 0:
        sys.exit(1)

    sys.exit(0)

if __name__ == "__main__":
    main()