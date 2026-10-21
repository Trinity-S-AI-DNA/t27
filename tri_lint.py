#!/usr/bin/env python3
"""
Enhanced Lint Command for tri CLI
This script provides static analysis for .t27 specifications.
"""

import argparse
import os
import re
import sys
from pathlib import Path
from typing import List, Dict, Any

class T27LintRule:
    """Base class for lint rules."""

    def __init__(self, name: str, severity: str = "warning"):
        self.name = name
        self.severity = severity

    def check(self, content: str, filepath: str) -> List[Dict[str, Any]]:
        """Check content for violations."""
        raise NotImplementedError

class NamingConventionRule(T27LintRule):
    """Check naming conventions for functions and constants."""

    def __init__(self):
        super().__init__("naming_convention", "warning")

    def check(self, content: str, filepath: str) -> List[Dict[str, Any]]:
        issues = []

        # Check function names (snake_case)
        function_pattern = r'pub fn ([a-z][a-z0-9_]*)\('
        for match in re.finditer(function_pattern, content):
            func_name = match.group(1)
            if '_' not in func_name and not func_name.islower():
                issues.append({
                    "type": "warning",
                    "rule": self.name,
                    "message": f"Function '{func_name}' should be snake_case",
                    "line": content[:match.start()].count('\n') + 1,
                    "column": match.start() - content.rfind('\n', 0, match.start()),
                    "file": filepath
                })

        # Check constant names (SCREAMING_SNAKE_CASE)
        const_pattern = r'pub const ([A-Z][A-Z0-9_]*)'
        for match in re.finditer(const_pattern, content):
            const_name = match.group(1)
            if const_name != const_name.upper():
                issues.append({
                    "type": "warning",
                    "rule": self.name,
                    "message": f"Constant '{const_name}' should be SCREAMING_SNAKE_CASE",
                    "line": content[:match.start()].count('\n') + 1,
                    "column": match.start() - content.rfind('\n', 0, match.start()),
                    "file": filepath
                })

        return issues

class BracesRule(T27LintRule):
    """Check for consistent brace usage."""

    def __init__(self):
        super().__init__("braces_consistency", "error")

    def check(self, content: str, filepath: str) -> List[Dict[str, Any]]:
        issues = []
        lines = content.split('\n')

        # Check for missing braces after control structures
        for i, line in enumerate(lines):
            stripped = line.strip()

            # Check if, for, while, etc. without braces
            if re.match(r'(if|for|while|unless)\s*\(.*\)\s*$', stripped) and not stripped.endswith('{'):
                issues.append({
                    "type": "error",
                    "rule": self.name,
                    "message": f"Missing opening brace after '{stripped.split('(')[0]}'",
                    "line": i + 1,
                    "column": 0,
                    "file": filepath
                })

        return issues

class TestRule(T27LintRule):
    """Check test coverage and naming."""

    def __init__(self):
        super().__init__("test_naming", "warning")

    def check(self, content: str, filepath: str) -> List[Dict[str, Any]]:
        issues = []

        # Check test names (should describe what is being tested)
        test_pattern = r'test\s+"([^"]+)"\s*\{'
        for match in re.finditer(test_pattern, content):
            test_name = match.group(1)

            # Check if test name is too generic
            generic_tests = ["test", "example", "basic"]
            if any(generic in test_name.lower() for generic in generic_tests):
                issues.append({
                    "type": "warning",
                    "rule": self.name,
                    "message": f"Test name '{test_name}' is too generic, be more specific",
                    "line": content[:match.start()].count('\n') + 1,
                    "column": match.start() - content.rfind('\n', 0, match.start()),
                    "file": filepath
                })

        # Check if there are tests in the file
        if 'test "' not in content:
            # But skip certain files
            if not any(skip in filepath for skip in ['policy', 'base']):
                issues.append({
                    "type": "info",
                    "rule": self.name,
                    "message": "No tests found in this specification",
                    "line": 1,
                    "column": 0,
                    "file": filepath
                })

        return issues

class CommentRule(T27LintRule):
    """Check comment quality and usage."""

    def __init__(self):
        super().__init__("comment_quality", "warning")

    def check(self, content: str, filepath: str) -> List[Dict[str, Any]]:
        issues = []
        lines = content.split('\n')

        for i, line in enumerate(lines):
            stripped = line.strip()

            # Check for comments that are just TODO
            if stripped.startswith('// TODO'):
                issues.append({
                    "type": "warning",
                    "rule": self.name,
                    "message": "TODO comments should have a description or be removed",
                    "line": i + 1,
                    "column": 0,
                    "file": filepath
                })

            # Check for comments that are too generic
            if stripped.startswith('// '):
                comment = stripped[3:]
                if len(comment) < 10:
                    issues.append({
                        "type": "info",
                        "rule": self.name,
                        "message": "Comment is too short, consider expanding",
                        "line": i + 1,
                        "column": 0,
                        "file": filepath
                    })

        return issues

class T27Linter:
    """Main linter class."""

    def __init__(self):
        self.rules = [
            NamingConventionRule(),
            BracesRule(),
            TestRule(),
            CommentRule()
        ]

    def lint_file(self, filepath: Path, fix: bool = False) -> List[Dict[str, Any]]:
        """Lint a single file."""
        if not filepath.suffix == '.t27':
            return []

        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return [{
                "type": "error",
                "rule": "file_read",
                "message": f"Could not read file: {e}",
                "line": 1,
                "column": 0,
                "file": str(filepath)
            }]

        all_issues = []

        for rule in self.rules:
            issues = rule.check(content, str(filepath))
            all_issues.extend(issues)

        if fix and filepath.exists():
            # Implement auto-fix logic here
            pass

        return all_issues

    def lint_directory(self, directory: Path, fix: bool = False, verbose: bool = False) -> Dict[str, Any]:
        """Lint all .t27 files in a directory."""
        all_issues = []
        files_processed = 0

        for t27_file in directory.rglob("*.t27"):
            files_processed += 1
            issues = self.lint_file(t27_file, fix)
            all_issues.extend(issues)

            if verbose:
                print(f"Linted: {t27_file}")

        # Count issues by severity
        severity_counts = {"error": 0, "warning": 0, "info": 0}
        for issue in all_issues:
            severity_counts[issue["type"]] += 1

        return {
            "files_processed": files_processed,
            "total_issues": len(all_issues),
            "severity_counts": severity_counts,
            "issues": all_issues
        }

def print_issues(issues: List[Dict[str, Any]], format_json: bool = False):
    """Print issues in various formats."""
    if format_json:
        print(json.dumps(issues, indent=2))
        return

    # Group issues by file
    issues_by_file = {}
    for issue in issues:
        file = issue["file"]
        if file not in issues_by_file:
            issues_by_file[file] = []
        issues_by_file[file].append(issue)

    # Print issues
    for file, file_issues in issues_by_file.items():
        print(f"\n📄 {file}")
        print("-" * 50)

        for issue in sorted(file_issues, key=lambda x: (x["line"], x["column"])):
            severity_icon = {
                "error": "❌",
                "warning": "⚠️",
                "info": "ℹ️"
            }.get(issue["type"], "❓")

            print(f"{severity_icon} Line {issue['line']}: {issue['message']}")
            if "rule" in issue:
                print(f"     Rule: {issue['rule']}")

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description='Enhanced linter for .t27 specifications')
    parser.add_argument('paths', nargs='*', default=['./specs'],
                       help='Paths to lint (default: ./specs)')
    parser.add_argument('--fix', action='store_true',
                       help='Attempt to auto-fix issues')
    parser.add_argument('--verbose', '-v', action='store_true',
                       help='Show verbose output')
    parser.add_argument('--json', action='store_true',
                       help='Output in JSON format')

    args = parser.parse_args()

    linter = T27Linter()
    all_results = {}

    for path in args.paths:
        path_obj = Path(path)
        if not path_obj.exists():
            print(f"❌ Path does not exist: {path}")
            continue

        if path_obj.is_file():
            issues = linter.lint_file(path_obj, args.fix)
            all_results[str(path_obj)] = {
                "issues": issues,
                "severity_counts": {"error": 0, "warning": 0, "info": 0}
            }
            for issue in issues:
                all_results[str(path_obj)]["severity_counts"][issue["type"]] += 1

        elif path_obj.is_dir():
            result = linter.lint_directory(path_obj, args.fix, args.verbose)
            all_results[str(path_obj)] = result

    # Print summary
    total_files = sum(r.get("files_processed", 0) for r in all_results.values())
    total_issues = sum(r.get("total_issues", 0) for r in all_results.values())

    if not args.json:
        print(f"\n📊 Lint Results Summary")
        print(f"Files processed: {total_files}")
        print(f"Total issues: {total_issues}")

        if total_issues > 0:
            severity_total = 0
            for result in all_results.values():
                counts = result.get("severity_counts", {})
                severity_total += counts.get("error", 0)
                severity_total += counts.get("warning", 0)
                severity_total += counts.get("info", 0)

            print(f"\n📈 Issue Distribution:")
            for result_path, result in all_results.items():
                if result.get("total_issues", 0) > 0:
                    counts = result.get("severity_counts", {})
                    print(f"  {result_path}:")
                    for severity, count in counts.items():
                        if count > 0:
                            icon = {"error": "❌", "warning": "⚠️", "info": "ℹ️"}.get(severity, "❓")
                            print(f"    {icon} {severity}: {count}")

    # Print all issues
    all_issues = []
    for result in all_results.values():
        all_issues.extend(result.get("issues", []))

    if all_issues:
        print_issues(all_issues, args.json)

    # Exit with error code if there are errors
    error_count = sum(1 for issue in all_issues if issue["type"] == "error")
    sys.exit(error_count if error_count > 0 else 0)

if __name__ == "__main__":
    main()