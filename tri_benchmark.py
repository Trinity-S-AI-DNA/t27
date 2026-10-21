#!/usr/bin/env python3
"""
Enhanced Benchmark Command for tri CLI
This script provides performance benchmarking for .t27 specifications.
"""

import argparse
import json
import os
import subprocess
import time
from pathlib import Path
from typing import Dict, List, Any, Optional

class T27Benchmark:
    """Benchmark class for .t27 specifications."""

    def __init__(self):
        self.results = []
        self.history_file = Path("./benchmark_history.json")
        self.load_history()

    def load_history(self):
        """Load benchmark history."""
        if self.history_file.exists():
            try:
                with open(self.history_file, 'r') as f:
                    self.history = json.load(f)
            except:
                self.history = []
        else:
            self.history = []

    def save_history(self):
        """Save benchmark history."""
        with open(self.history_file, 'w') as f:
            json.dump(self.history, f, indent=2)

    def benchmark_spec(self, spec_path: str, iterations: int = 5) -> Dict[str, Any]:
        """Benchmark a single specification."""
        spec_path = Path(spec_path)
        if not spec_path.exists():
            return {"error": f"Spec not found: {spec_path}"}

        results = {
            "spec": str(spec_path),
            "iterations": iterations,
            "times": [],
            "averages": {},
            "errors": []
        }

        print(f"🔬 Benchmarking {spec_path.name}...")

        # Benchmark t27c path command
        path_times = []
        for i in range(iterations):
            try:
                start_time = time.time()
                env = os.environ.copy()
                env['ZIG_GLOBAL_CACHE_DIR'] = '/tmp/claude-501/zig_cache'

                result = subprocess.run(
                    ['./target/release/t27c', 'path', str(spec_path)],
                    capture_output=True,
                    text=True,
                    timeout=30,
                    env=env
                )

                end_time = time.time()
                if result.returncode == 0:
                    elapsed = (end_time - start_time) * 1000  # Convert to ms
                    path_times.append(elapsed)
                    print(f"  Iteration {i+1}: {elapsed:.2f}ms")
                else:
                    results["errors"].append(f"t27c path failed: {result.stderr}")
            except subprocess.TimeoutExpired:
                results["errors"].append(f"t27c path timed out after 30s")
            except Exception as e:
                results["errors"].append(f"t27c path error: {str(e)}")

        # Benchmark tri health
        health_times = []
        for i in range(iterations):
            try:
                start_time = time.time()
                result = subprocess.run(
                    ['./target/release/tri', 'health'],
                    capture_output=True,
                    text=True,
                    timeout=10
                )

                end_time = time.time()
                if result.returncode == 0:
                    elapsed = (end_time - start_time) * 1000
                    health_times.append(elapsed)
                    print(f"  Health iteration {i+1}: {elapsed:.2f}ms")
                else:
                    results["errors"].append(f"tri health failed: {result.stderr}")
            except Exception as e:
                results["errors"].append(f"tri health error: {str(e)}")

        # Calculate averages
        if path_times:
            results["times"]["path"] = path_times
            results["averages"]["path"] = {
                "mean": sum(path_times) / len(path_times),
                "min": min(path_times),
                "max": max(path_times),
                "median": sorted(path_times)[len(path_times) // 2]
            }

        if health_times:
            results["times"]["health"] = health_times
            results["averages"]["health"] = {
                "mean": sum(health_times) / len(health_times),
                "min": min(health_times),
                "max": max(health_times),
                "median": sorted(health_times)[len(health_times) // 2]
            }

        # Add to history
        timestamp = time.strftime("%Y-%m-%dT%H:%M:%S")
        history_entry = {
            "timestamp": timestamp,
            "spec": str(spec_path),
            "results": results["averages"]
        }

        # Update or add to history
        found = False
        for entry in self.history:
            if entry["spec"] == str(spec_path):
                entry.update(history_entry)
                found = True
                break

        if not found:
            self.history.append(history_entry)

        self.save_history()

        return results

    def benchmark_directory(self, directory: str, iterations: int = 5,
                          max_specs: Optional[int] = None) -> Dict[str, Any]:
        """Benchmark all specs in a directory."""
        directory = Path(directory)
        if not directory.exists():
            return {"error": f"Directory not found: {directory}"}

        spec_files = list(directory.rglob("*.t27"))
        if max_specs:
            spec_files = spec_files[:max_specs]

        if not spec_files:
            return {"error": "No .t27 files found"}

        print(f"🔬 Benchmarking {len(spec_files)} specs in {directory}...")

        all_results = []
        total_time = 0

        for spec_file in spec_files:
            result = self.benchmark_spec(str(spec_file), iterations)
            all_results.append(result)

            if "averages" in result:
                for category, times in result["averages"].items():
                    total_time += times["mean"]

        # Calculate summary statistics
        summary = {
            "directory": str(directory),
            "total_specs": len(spec_files),
            "iterations": iterations,
            "total_time": total_time,
            "avg_time_per_spec": total_time / len(spec_files) if spec_files else 0,
            "results": all_results
        }

        return summary

    def compare_results(self, spec_path: str) -> Dict[str, Any]:
        """Compare current results with history."""
        spec_path = str(Path(spec_path))

        # Find history entries for this spec
        history_entries = [e for e in self.history if e["spec"] == spec_path]

        if not history_entries:
            return {"error": "No history found for this spec"}

        # Get latest result
        latest = history_entries[-1]

        comparison = {
            "spec": spec_path,
            "current": latest,
            "history": history_entries,
            "trends": {}
        }

        # Calculate trends
        if len(history_entries) >= 2:
            # Compare with previous
            prev = history_entries[-2]
            current = latest

            for category in current["results"]:
                if category in prev["results"]:
                    prev_val = prev["results"][category]["mean"]
                    current_val = current["results"][category]["mean"]
                    change = current_val - prev_val
                    percent_change = (change / prev_val) * 100 if prev_val != 0 else 0

                    comparison["trends"][category] = {
                        "previous": prev_val,
                        "current": current_val,
                        "change": change,
                        "percent_change": percent_change,
                        "trend": "improved" if change < 0 else "degraded"
                    }

        return comparison

    def generate_html_report(self, results: Dict[str, Any], output_file: str):
        """Generate HTML report for benchmark results."""
        html = f"""
<!DOCTYPE html>
<html>
<head>
    <title>T27 Benchmark Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .header {{ background-color: #f0f0f0; padding: 20px; border-radius: 5px; }}
        .metric {{ margin: 10px 0; }}
        .chart {{ width: 100%; height: 400px; }}
        .good {{ color: green; }}
        .bad {{ color: red; }}
        .warning {{ color: orange; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>T27 Benchmark Report</h1>
        <p>Generated on: {time.strftime("%Y-%m-%d %H:%M:%S")}</p>
    </div>

    <div class="content">
        <h2>Summary</h2>
        <div class="metric">
            <strong>Total Specs:</strong> {results.get('total_specs', 0)}
        </div>
        <div class="metric">
            <strong>Total Time:</strong> {results.get('total_time', 0):.2f}ms
        </div>
        <div class="metric">
            <strong>Avg Time per Spec:</strong> {results.get('avg_time_per_spec', 0):.2f}ms
        </div>

        <h2>Detailed Results</h2>
        <table border="1" style="width: 100%; border-collapse: collapse;">
            <tr>
                <th>Spec</th>
                <th>Path Avg (ms)</th>
                <th>Health Avg (ms)</th>
                <th>Status</th>
            </tr>
"""

        for result in results.get('results', []):
            spec_name = Path(result.get('spec', '')).name
            path_avg = result.get('averages', {}).get('path', {}).get('mean', 0)
            health_avg = result.get('averages', {}).get('health', {}).get('mean', 0)

            status = "✅ Success"
            if result.get('errors'):
                status = "❌ Errors"
            elif path_avg > 1000:
                status = "⚠️ Slow"

            html += f"""
            <tr>
                <td>{spec_name}</td>
                <td>{path_avg:.2f}</td>
                <td>{health_avg:.2f}</td>
                <td>{status}</td>
            </tr>
"""

        html += """
        </table>
    </div>
</body>
</html>
"""

        with open(output_file, 'w') as f:
            f.write(html)

        print(f"📊 HTML report generated: {output_file}")

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description='Enhanced benchmark for .t27 specifications')
    parser.add_argument('paths', nargs='*', default=['./specs'],
                       help='Paths to benchmark (default: ./specs)')
    parser.add_argument('--iterations', '-n', type=int, default=5,
                       help='Number of iterations per spec (default: 5)')
    parser.add_argument('--max-specs', type=int, default=None,
                       help='Maximum number of specs to benchmark')
    parser.add_argument('--compare', action='store_true',
                       help='Compare with previous benchmark results')
    parser.add_argument('--html', metavar='FILE', default=None,
                       help='Generate HTML report')
    parser.add_argument('--threshold', type=float, default=10.0,
                       help='Warning threshold in ms (default: 10)')
    parser.add_argument('--verbose', '-v', action='store_true',
                       help='Show verbose output')

    args = parser.parse_args()

    benchmark = T27Benchmark()
    all_results = []

    for path in args.paths:
        if Path(path).is_file() and path.endswith('.t27'):
            # Single spec
            result = benchmark.benchmark_spec(path, args.iterations)
            all_results.append(result)

            if args.compare:
                comparison = benchmark.compare_results(path)
                if "error" not in comparison:
                    print(f"\n📊 Comparison for {path}:")
                    for category, trend in comparison["trends"].items():
                        icon = "📉" if trend["trend"] == "degraded" else "📈"
                        print(f"  {icon} {category}: {trend['percent_change']:+.1f}%")

        elif Path(path).is_dir():
            # Directory
            result = benchmark.benchmark_directory(path, args.iterations, args.max_specs)
            all_results.append(result)

            # Generate HTML report if requested
            if args.html:
                benchmark.generate_html_report(result, args.html)

    # Print summary
    print(f"\n📊 Benchmark Summary")
    print("=" * 50)

    total_specs = sum(1 for r in all_results if "error" not in r)
    total_errors = sum(len(r.get("errors", [])) for r in all_results)

    print(f"Total specs benchmarked: {total_specs}")
    print(f"Total errors: {total_errors}")

    if total_errors == 0 and total_specs > 0:
        print("✅ All benchmarks successful!")
    elif total_errors > 0:
        print("⚠️ Some benchmarks had errors")
        print("\nErrors:")
        for result in all_results:
            if result.get("errors"):
                for error in result["errors"]:
                    print(f"  - {error}")

    # Check performance against threshold
    slow_specs = []
    for result in all_results:
        if "averages" in result:
            for category, times in result["averages"].items():
                if times["mean"] > args.threshold:
                    slow_specs.append((result["spec"], category, times["mean"]))

    if slow_specs:
        print(f"\n🐌 Slow operations (> {args.threshold}ms):")
        for spec, category, time_ms in slow_specs:
            print(f"  - {spec} ({category}): {time_ms:.2f}ms")

if __name__ == "__main__":
    main()