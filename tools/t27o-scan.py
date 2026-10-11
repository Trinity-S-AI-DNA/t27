#!/usr/bin/env python3
"""
t27o Scanner - Scans repositories for convertible and verifiable functions

This tool scans a repository to:
1. Find foreign code functions (starting with Rust, then Zig, Python, TypeScript)
2. Use t27o to convert functions to .t27 specs
3. Verify functions against the oracle
4. Report statistics and refusal reasons
"""

import os
import re
import json
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Tuple, Optional
import argparse

class T27oScanner:
    def __init__(self, repo_path: str = "."):
        self.repo_path = Path(repo_path)
        self.results = {
            "total_functions": 0,
            "convertible_functions": 0,
            "verifiable_functions": 0,
            "refusal_reasons": {},
            "language_stats": {},
            "function_details": []
        }
    
    def find_foreign_functions(self) -> List[Dict]:
        """Find foreign code functions in the repository"""
        functions = []
        
        # Language-specific patterns for function definitions
        language_patterns = {
            "rust": {
                "extensions": [".rs"],
                "pattern": r"^\s*(pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(",
                "exclude_paths": ["gen/", "build/", "target/", "vendor/"]
            },
            "zig": {
                "extensions": [".zig"],
                "pattern": r"^\s*(pub\s+)?fn\s+(\w+)\s*\(",
                "exclude_paths": ["gen/", "build/", "zig-cache/"]
            },
            "python": {
                "extensions": [".py"],
                "pattern": r"^\s*(?:async\s+)?def\s+(\w+)\s*\(",
                "exclude_paths": ["gen/", "build/", "__pycache__/", ".venv/"]
            },
            "typescript": {
                "extensions": [".ts", ".tsx"],
                "pattern": r"^\s*(?:export\s+)?(?:async\s+)?(?:function\s+(\w+)\s*\(|(\w+)\s*=\s*\(.*\)\s*=>)",
                "exclude_paths": ["gen/", "build/", "dist/", "node_modules/"]
            }
        }
        
        for lang, config in language_patterns.items():
            for ext in config["extensions"]:
                for file_path in self.repo_path.rglob(f"*{ext}"):
                    # Skip excluded paths
                    if any(excl in str(file_path) for excl in config["exclude_paths"]):
                        continue
                    
                    try:
                        with open(file_path, 'r', encoding='utf-8') as f:
                            content = f.read()
                            lines = content.split('\n')
                            
                            for line_num, line in enumerate(lines, 1):
                                match = re.search(config["pattern"], line)
                                if match:
                                    func_name = match.group(1) or match.group(2)
                                    functions.append({
                                        "language": lang,
                                        "file": str(file_path),
                                        "line": line_num,
                                        "name": func_name,
                                        "content": self.extract_function_content(lines, line_num - 1)
                                    })
                    except Exception as e:
                        print(f"Error reading {file_path}: {e}")
                        continue
        
        return functions
    
    def extract_function_content(self, lines: List[str], start_line: int) -> str:
        """Extract the complete function content starting from the given line"""
        content = []
        indent_level = None
        
        for i in range(start_line, len(lines)):
            line = lines[i]
            
            # Skip empty lines at the beginning
            if not line.strip() and not content:
                continue
            
            content.append(line)
            
            # Determine indentation level from first non-empty line
            if indent_level is None and line.strip():
                indent_level = len(line) - len(line.lstrip())
            
            # Stop when we reach a line with less indentation or end of file
            if indent_level is not None and line.strip() and len(line) - len(line.lstrip()) <= indent_level:
                break
        
        return '\n'.join(content)
    
    def call_t27o(self, function_data: Dict) -> Optional[Dict]:
        """Call t27o to convert and verify a function"""
        try:
            # Create temporary file with function content
            temp_file = self.repo_path / "temp_function.txt"
            with open(temp_file, 'w') as f:
                f.write(function_data["content"])
            
            # Try to convert with t27o
            convert_result = subprocess.run([
                "t27o", "convert", 
                "--language", function_data["language"],
                "--input", str(temp_file)
            ], capture_output=True, text=True, timeout=30)
            
            if convert_result.returncode == 0:
                # Try to verify with t27o
                verify_result = subprocess.run([
                    "t27o", "verify",
                    "--language", function_data["language"],
                    "--input", str(temp_file)
                ], capture_output=True, text=True, timeout=30)
                
                return {
                    "convertible": True,
                    "verifiable": verify_result.returncode == 0,
                    "conversion_output": convert_result.stdout,
                    "verification_output": verify_result.stdout,
                    "error": verify_result.stderr if verify_result.returncode != 0 else None
                }
            else:
                # Parse refusal reason from output
                refusal_reason = self.parse_refusal_reason(convert_result.stderr)
                return {
                    "convertible": False,
                    "verifiable": False,
                    "refusal_reason": refusal_reason,
                    "error": convert_result.stderr
                }
        
        except subprocess.TimeoutExpired:
            return {
                "convertible": False,
                "verifiable": False,
                "refusal_reason": "timeout",
                "error": "t27o conversion timed out"
            }
        except Exception as e:
            return {
                "convertible": False,
                "verifiable": False,
                "refusal_reason": "error",
                "error": str(e)
            }
        finally:
            # Clean up temp file
            if temp_file.exists():
                temp_file.unlink()
    
    def parse_refusal_reason(self, error_output: str) -> str:
        """Parse refusal reason from t27o error output"""
        if "not implemented" in error_output.lower():
            return "not_implemented"
        elif "unsupported" in error_output.lower():
            return "unsupported_feature"
        elif "syntax" in error_output.lower():
            return "syntax_error"
        elif "timeout" in error_output.lower():
            return "timeout"
        else:
            return "unknown"
    
    def scan_repository(self) -> Dict:
        """Scan the entire repository and return results"""
        print(f"Scanning repository: {self.repo_path}")
        
        # Find all foreign functions
        functions = self.find_foreign_functions()
        self.results["total_functions"] = len(functions)
        
        print(f"Found {len(functions)} foreign functions")
        
        # Process each function
        for i, func in enumerate(functions):
            print(f"Processing function {i+1}/{len(functions)}: {func['name']} ({func['language']})")
            
            # Call t27o
            t27o_result = self.call_t27o(func)
            
            # Update statistics
            if t27o_result["convertible"]:
                self.results["convertible_functions"] += 1
                if t27o_result["verifiable"]:
                    self.results["verifiable_functions"] += 1
            
            # Track refusal reasons
            if not t27o_result["convertible"]:
                reason = t27o_result.get("refusal_reason", "unknown")
                self.results["refusal_reasons"][reason] = self.results["refusal_reasons"].get(reason, 0) + 1
            
            # Track language statistics
            lang = func["language"]
            if lang not in self.results["language_stats"]:
                self.results["language_stats"][lang] = {
                    "total": 0,
                    "convertible": 0,
                    "verifiable": 0
                }
            self.results["language_stats"][lang]["total"] += 1
            if t27o_result["convertible"]:
                self.results["language_stats"][lang]["convertible"] += 1
                if t27o_result["verifiable"]:
                    self.results["language_stats"][lang]["verifiable"] += 1
            
            # Store function details
            self.results["function_details"].append({
                "function": func,
                "t27o_result": t27o_result
            })
        
        return self.results
    
    def generate_report(self) -> str:
        """Generate a human-readable report"""
        report = []
        report.append("# t27o Scan Report")
        report.append("")
        report.append(f"Repository: {self.repo_path}")
        report.append(f"Scan Date: {subprocess.run(['date'], capture_output=True, text=True).stdout.strip()}")
        report.append("")
        
        # Overall statistics
        report.append("## Overall Statistics")
        report.append("")
        report.append(f"- Total Functions Found: {self.results['total_functions']}")
        report.append(f"- Convertible Functions: {self.results['convertible_functions']}")
        report.append(f"- Verifiable Functions: {self.results['verifiable_functions']}")
        
        if self.results['total_functions'] > 0:
            convertible_pct = (self.results['convertible_functions'] / self.results['total_functions']) * 100
            verifiable_pct = (self.results['verifiable_functions'] / self.results['total_functions']) * 100
            report.append(f"- Convertible: {convertible_pct:.1f}%")
            report.append(f"- Verifiable: {verifiable_pct:.1f}%")
        
        report.append("")
        
        # Language statistics
        report.append("## Language Statistics")
        report.append("")
        for lang, stats in self.results["language_stats"].items():
            report.append(f"### {lang.upper()}")
            report.append(f"- Total Functions: {stats['total']}")
            report.append(f"- Convertible: {stats['convertible']}")
            report.append(f"- Verifiable: {stats['verifiable']}")
            
            if stats['total'] > 0:
                convertible_pct = (stats['convertible'] / stats['total']) * 100
                verifiable_pct = (stats['verifiable'] / stats['total']) * 100
                report.append(f"- Convertible: {convertible_pct:.1f}%")
                report.append(f"- Verifiable: {verifiable_pct:.1f}%")
            report.append("")
        
        # Refusal reasons
        if self.results["refusal_reasons"]:
            report.append("## Refusal Reasons")
            report.append("")
            for reason, count in self.results["refusal_reasons"].items():
                report.append(f"- {reason}: {count}")
            report.append("")
        
        return "\n".join(report)

def main():
    parser = argparse.ArgumentParser(description="Scan repository with t27o")
    parser.add_argument("repo_path", nargs="?", default=".", help="Path to repository (default: current directory)")
    parser.add_argument("--output", "-o", help="Output file for JSON results")
    parser.add_argument("--report", "-r", help="Output file for human-readable report")
    
    args = parser.parse_args()
    
    scanner = T27oScanner(args.repo_path)
    results = scanner.scan_repository()
    
    # Generate outputs
    if args.output:
        with open(args.output, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"JSON results saved to {args.output}")
    
    if args.report:
        report = scanner.generate_report()
        with open(args.report, 'w') as f:
            f.write(report)
        print(f"Report saved to {args.report}")
    
    # Print summary to console
    report = scanner.generate_report()
    print(report)
    
    # Return exit code based on results
    if results["convertible_functions"] > 0:
        print(f"\n✅ Found {results['convertible_functions']} convertible functions")
        return 0
    else:
        print(f"\n❌ No convertible functions found")
        return 1

if __name__ == "__main__":
    sys.exit(main())