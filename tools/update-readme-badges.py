#!/usr/bin/env python3
"""
README Badge Updater - Updates README with golden coverage and foreign line badges

This tool:
1. Reads census data for foreign line counts
2. Reads t27o scan data for coverage percentages
3. Updates README with badges showing:
   - Golden coverage percentage
   - Foreign line counts by language
   - Trend indicators
"""

import os
import json
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional
import argparse
import re

class ReadmeBadgeUpdater:
    def __init__(self, repo_path: str = "."):
        self.repo_path = Path(repo_path)
        self.readme_path = self.repo_path / "README.md"
        self.badges = []
    
    def read_census_data(self) -> Dict:
        """Read foreign line census data"""
        census_file = self.repo_path / "foreign_lines_census.txt"
        if not census_file.exists():
            return {}
        
        data = {}
        with open(census_file, 'r') as f:
            for line in f:
                line = line.strip()
                if line and '\t' in line:
                    parts = line.split('\t')
                    if len(parts) >= 2:
                        lang = parts[1]
                        count = int(parts[0])
                        data[lang] = count
        
        return data
    
    def read_t27o_scan_data(self) -> Dict:
        """Read t27o scan data"""
        scan_file = self.repo_path / "scan-results.json"
        if not scan_file.exists():
            return {}
        
        with open(scan_file, 'r') as f:
            return json.load(f)
    
    def calculate_coverage_percentage(self, scan_data: Dict) -> float:
        """Calculate golden coverage percentage"""
        total_functions = scan_data.get("total_functions", 0)
        convertible_functions = scan_data.get("convertible_functions", 0)
        
        if total_functions == 0:
            return 0.0
        
        return (convertible_functions / total_functions) * 100
    
    def generate_coverage_badge(self, percentage: float) -> str:
        """Generate coverage badge"""
        if percentage >= 80:
            color = "green"
        elif percentage >= 50:
            color = "yellow"
        else:
            color = "red"
        
        return f"[![Golden Coverage](https://img.shields.io/badge/Golden%20Coverage-{percentage:.1f}%25-{color})](docs/golden-coverage.md)"
    
    def generate_foreign_line_badge(self, census_data: Dict) -> str:
        """Generate foreign line count badge"""
        total_foreign = sum(census_data.values())
        return f"[![Foreign Lines](https://img.shields.io/badge/Foreign%20Lines-{total_foreign}-blue)](docs/foreign-debt.md)"
    
    def generate_language_badges(self, census_data: Dict) -> List[str]:
        """Generate language-specific badges"""
        badges = []
        
        for lang, count in sorted(census_data.items()):
            if count > 0:
                color = "red" if count > 100 else "orange" if count > 50 else "yellow"
                badges.append(f"[![{lang.title()}](https://img.shields.io/badge/{lang.title()}-{count}-{color})](docs/foreign-debt.md)")
        
        return badges
    
    def generate_trend_badge(self) -> str:
        """Generate trend badge (placeholder for now)"""
        return "[![Trend](https://img.shields.io/badge/Trend-📈-green)](docs/trend-analysis.md)"
    
    def find_badge_section(self, readme_content: str) -> Optional[int]:
        """Find the badge section in README"""
        # Look for common badge section patterns
        patterns = [
            r"##\s*Badges\s*\n",
            r"##\s*Status\s*\n",
            r"##\s*Metrics\s*\n",
            r"<details>\s*<summary>Badges</summary>",
        ]
        
        for pattern in patterns:
            match = re.search(pattern, readme_content, re.IGNORECASE | re.MULTILINE)
            if match:
                return match.end()
        
        # If no section found, look for the first heading
        heading_match = re.search(r'^#+\s', readme_content, re.MULTILINE)
        if heading_match:
            return heading_match.start()
        
        return None
    
    def generate_badge_section(self, census_data: Dict, scan_data: Dict) -> str:
        """Generate the complete badge section"""
        coverage_pct = self.calculate_coverage_percentage(scan_data)
        
        badges = []
        badges.append("## Quality Metrics")
        badges.append("")
        
        # Main badges
        badges.append(f"{self.generate_coverage_badge(coverage_pct)}")
        badges.append(f"{self.generate_foreign_line_badge(census_data)}")
        badges.append(f"{self.generate_trend_badge()}")
        badges.append("")
        
        # Language breakdown
        if census_data:
            badges.append("### Foreign Line Breakdown")
            badges.append("")
            language_badges = self.generate_language_badges(census_data)
            if language_badges:
                # Format badges in rows of 3
                for i in range(0, len(language_badges), 3):
                    row = language_badges[i:i+3]
                    badges.append(" ".join(row))
                badges.append("")
        
        # Statistics
        badges.append("### Statistics")
        badges.append("")
        badges.append(f"- **Golden Coverage**: {coverage_pct:.1f}% ({scan_data.get('convertible_functions', 0)}/{scan_data.get('total_functions', 0)} functions)")
        badges.append(f"- **Total Foreign Lines**: {sum(census_data.values())}")
        badges.append(f"- **Languages with Foreign Code**: {len([c for c in census_data.values() if c > 0])}")
        badges.append("")
        
        return "\n".join(badges)
    
    def update_readme(self):
        """Update README with badges"""
        if not self.readme_path.exists():
            print(f"README.md not found at {self.readme_path}")
            return False
        
        # Read current README
        with open(self.readme_path, 'r') as f:
            readme_content = f.read()
        
        # Read data
        census_data = self.read_census_data()
        scan_data = self.read_t27o_scan_data()
        
        if not census_data and not scan_data:
            print("No census or scan data found")
            return False
        
        # Generate new badge section
        badge_section = self.generate_badge_section(census_data, scan_data)
        
        # Find where to insert badges
        insert_pos = self.find_badge_section(readme_content)
        
        if insert_pos is None:
            # Append to end if no section found
            new_content = readme_content + "\n\n" + badge_section
        else:
            # Replace existing badge section or insert at found position
            # Look for end of badge section
            end_patterns = [
                r"(?m)^#+\s",  # Next heading
                r"</details>",  # End of details block
                r"\n\n",  # Double line break
            ]
            
            end_pos = len(readme_content)
            for pattern in end_patterns:
                match = re.search(pattern, readme_content[insert_pos:])
                if match:
                    end_pos = insert_pos + match.start()
                    break
            
            # Replace the section
            new_content = (readme_content[:insert_pos] + 
                          badge_section + 
                          readme_content[end_pos:].lstrip())
        
        # Write updated README
        with open(self.readme_path, 'w') as f:
            f.write(new_content)
        
        print("README badges updated successfully")
        return True
    
    def generate_documentation(self, census_data: Dict, scan_data: Dict):
        """Generate documentation files for badges"""
        docs_dir = self.repo_path / "docs"
        docs_dir.mkdir(exist_ok=True)
        
        # Generate golden coverage documentation
        coverage_pct = self.calculate_coverage_percentage(scan_data)
        
        with open(docs_dir / "golden-coverage.md", 'w') as f:
            f.write(f"""# Golden Coverage Report

## Current Status
- **Coverage**: {coverage_pct:.1f}%
- **Convertible Functions**: {scan_data.get('convertible_functions', 0)}
- **Total Functions**: {scan_data.get('total_functions', 0)}

## Language Breakdown

""")
        
        # Generate foreign debt documentation
        with open(docs_dir / "foreign-debt.md", 'w') as f:
            f.write("""# Foreign Line Debt Report

## Current Status

| Language | Hand-written Lines |
|----------|-------------------|
""")
            
            for lang, count in sorted(census_data.items()):
                f.write(f"| {lang} | {count} |\n")
            
            total = sum(census_data.values())
            f.write(f"| **Total** | **{total}** |\n\n")
            
            f.write("""## Exclusions
- Generated code (gen/, build/, etc.)
- Vendored dependencies
- Build output
- Third-party libraries

## Goals
- Foreign lines should only decrease over time
- Each golden PR should be net-negative in foreign lines
- Eventually eliminate all hand-written foreign code
""")
        
        # Generate trend analysis documentation
        with open(docs_dir / "trend-analysis.md", 'w') as f:
            f.write("""# Trend Analysis

This section tracks the progress of converting foreign code to .t27 specs.

## Key Metrics
- **Golden Coverage**: Percentage of functions convertible to .t27 specs
- **Foreign Line Debt**: Total lines of hand-written foreign code
- **Golden PRs**: Number of successful conversions

## Trend Goals
- Golden coverage should increase over time
- Foreign line debt should decrease over time
- Each target repository should have at least one golden PR per convertible language

## Current Status
*Data updated weekly via automated workflows.*
""")

def main():
    parser = argparse.ArgumentParser(description="Update README with quality badges")
    parser.add_argument("repo_path", nargs="?", default=".", help="Path to repository")
    parser.add_argument("--generate-docs", action="store_true", 
                       help="Generate documentation files only")
    
    args = parser.parse_args()
    
    updater = ReadmeBadgeUpdater(args.repo_path)
    
    if args.generate_docs:
        census_data = updater.read_census_data()
        scan_data = updater.read_t27o_scan_data()
        updater.generate_documentation(census_data, scan_data)
        print("Documentation files generated")
        return 0
    
    success = updater.update_readme()
    return 0 if success else 1

if __name__ == "__main__":
    sys.exit(main())