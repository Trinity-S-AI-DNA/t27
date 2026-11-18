#!/usr/bin/env python3
"""
Doctor CLI - Main interface for the Doctor MVP automation system
"""

import argparse
import json
import logging
import os
import sys
import uuid
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Any

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from scanner import IncidentScanner
from triage import IncidentTriage
from reporter import IncidentReporter

class DoctorCLI:
    """Main CLI interface for the Doctor system"""
    
    def __init__(self, config_path: Optional[str] = None):
        self.config_path = config_path or self._get_default_config_path()
        self.config = self._load_config()
        
        # Setup logging
        logging.basicConfig(
            level=getattr(logging, self.config.get('log_level', 'INFO')),
            format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
        )
        self.logger = logging.getLogger(__name__)
        
        # Initialize components
        self.scanner = IncidentScanner(self.config)
        self.triage = IncidentTriage(self.config)
        self.reporter = IncidentReporter(self.config)
        
        # Fixed output path resolution to prevent nested directory creation
        self.repo_root = Path(__file__).resolve().parents[2]  # Go from src/ to contrib/doctor-mvp/
        self.output_root = self.repo_root / "audit-output" / "doctor-mvp"
        
    def _get_default_config_path(self) -> str:
        """Get default config path"""
        return os.path.join(os.path.dirname(__file__), '..', 'config', 'doctor-config.json')
    
    def _resolve_output_path(self, relative_path: str) -> Path:
        """Resolve output path safely using repository root as base"""
        # Convert relative path to Path object
        relative = Path(relative_path)
        
        # Check for forbidden patterns in the relative path itself
        forbidden_patterns = [
            "contrib/doctor-mvp/contrib/doctor-mvp",
            "contrib/doctor-mvp/test.json",  # This is the actual bug pattern
            "contrib/doctor-mvp/"
        ]
        
        # Check if any forbidden pattern exists in the relative path
        for pattern in forbidden_patterns:
            if pattern in str(relative):
                raise ValueError(f"Forbidden output path pattern: {pattern}")
        
        # Create output path relative to output root
        path = (self.output_root / relative).resolve()
        
        # Validate path doesn't escape allowed root
        if not path.is_relative_to(self.output_root):
            raise ValueError(f"Output path escapes allowed root: {path}")
        
        # Additional validation: check if the final path contains forbidden patterns
        # This catches cases where the path construction accidentally creates nested dirs
        forbidden_in_final = "contrib/doctor-mvp/contrib/doctor-mvp"
        if forbidden_in_final in path.as_posix():
            raise ValueError(f"Forbidden nested output path in final result: {path}")
        
        return path
    
    def _create_output_dirs(self, output_path: Path) -> None:
        """Create output directories if they don't exist"""
        output_path.parent.mkdir(parents=True, exist_ok=True)
    
    def _load_config(self) -> Dict[str, Any]:
        """Load configuration from file"""
        if os.path.exists(self.config_path):
            try:
                with open(self.config_path, 'r') as f:
                    return json.load(f)
            except Exception as e:
                logging.warning(f"Failed to load config: {e}")
        return {}
    
    def scan(self, log_files: List[str], output_dir: Optional[str] = None) -> Dict[str, Any]:
        """Scan log files for incidents"""
        self.logger.info(f"Scanning {len(log_files)} log files for incidents")
        
        incidents = []
        for log_file in log_files:
            if not os.path.exists(log_file):
                self.logger.error(f"Log file not found: {log_file}")
                continue
                
            self.logger.info(f"Scanning: {log_file}")
            file_incidents = self.scanner.scan_file(log_file)
            incidents.extend(file_incidents)
        
        if output_dir:
            # Use fixed output path resolution
            output_file = self._resolve_output_path(output_dir + '/incidents.json')
            self._create_output_dirs(output_file)
            with open(output_file, 'w') as f:
                json.dump(incidents, f, indent=2, default=str)
            self.logger.info(f"Incidents saved to: {output_file}")
        
        return {
            'incidents': incidents,
            'total_incidents': len(incidents),
            'scan_time': datetime.now().isoformat()
        }
    
    def triage(self, incidents_file: str, output_dir: Optional[str] = None) -> Dict[str, Any]:
        """Triage incidents and classify them"""
        self.logger.info(f"Triage incidents from: {incidents_file}")
        
        if not os.path.exists(incidents_file):
            raise FileNotFoundError(f"Incidents file not found: {incidents_file}")
        
        with open(incidents_file, 'r') as f:
            incidents = json.load(f)
        
        triaged_incidents = []
        for incident in incidents:
            triaged = self.triage.triage_incident(incident)
            triaged_incidents.append(triaged)
        
        if output_dir:
            # Use fixed output path resolution
            output_file = self._resolve_output_path(output_dir + '/triaged-incidents.json')
            self._create_output_dirs(output_file)
            with open(output_file, 'w') as f:
                json.dump(triaged_incidents, f, indent=2, default=str)
            self.logger.info(f"Triage results saved to: {output_file}")
        
        return {
            'triaged_incidents': triaged_incidents,
            'total_triaged': len(triaged_incidents),
            'triage_time': datetime.now().isoformat()
        }
    
    def handoff(self, incident_id: str, output_dir: Optional[str] = None) -> Dict[str, Any]:
        """Create repair brief for a specific incident"""
        self.logger.info(f"Creating repair brief for incident: {incident_id}")
        
        # Load incidents file
        incidents_file = os.path.join(output_dir or '.', 'triaged-incidents.json')
        if not os.path.exists(incidents_file):
            raise FileNotFoundError(f"Triaged incidents file not found: {incidents_file}")
        
        with open(incidents_file, 'r') as f:
            incidents = json.load(f)
        
        # Find the specific incident
        target_incident = None
        for incident in incidents:
            if incident['incident_id'] == incident_id:
                target_incident = incident
                break
        
        if not target_incident:
            raise ValueError(f"Incident with ID {incident_id} not found")
        
        # Generate repair brief
        repair_brief = self.triage.generate_repair_brief(target_incident)
        
        # Save repair brief
        if output_dir:
            # Use fixed output path resolution
            output_file = self._resolve_output_path(output_dir + f'/repair-brief-{incident_id}.json')
            self._create_output_dirs(output_file)
            with open(output_file, 'w') as f:
                json.dump(repair_brief, f, indent=2, default=str)
            self.logger.info(f"Repair brief saved to: {output_file}")
        else:
            output_file = self._resolve_output_path(f'repair-brief-{incident_id}.json')
            self._create_output_dirs(output_file)
            with open(output_file, 'w') as f:
                json.dump(repair_brief, f, indent=2, default=str)
            self.logger.info(f"Repair brief saved to: {output_file}")
        
        return {
            'repair_brief': repair_brief,
            'output_file': output_file,
            'handoff_time': datetime.now().isoformat()
        }
    
    def verify(self, repair_brief_file: str) -> Dict[str, Any]:
        """Verify the repair using verification gates"""
        self.logger.info(f"Verifying repair from: {repair_brief_file}")
        
        if not os.path.exists(repair_brief_file):
            raise FileNotFoundError(f"Repair brief file not found: {repair_brief_file}")
        
        with open(repair_brief_file, 'r') as f:
            repair_brief = json.load(f)
        
        # Get verification gates for incident type
        incident_type = repair_brief['incident_type']
        verification_steps = self.triage.verification_gates.get(incident_type, {}).get('verification_steps', [])
        
        results = []
        for step in verification_steps:
            step_result = self.triage.verify_step(step, repair_brief)
            results.append(step_result)
        
        success = all(result['status'] == 'success' for result in results)
        
        return {
            'verification_results': results,
            'overall_success': success,
            'verification_time': datetime.now().isoformat()
        }
    
    def report(self, incidents_file: str, report_format: str = 'json') -> Dict[str, Any]:
        """Generate final report"""
        self.logger.info(f"Generating {report_format} report from: {incidents_file}")
        
        if not os.path.exists(incidents_file):
            raise FileNotFoundError(f"Incidents file not found: {incidents_file}")
        
        with open(incidents_file, 'r') as f:
            incidents = json.load(f)
        
        report = self.reporter.generate_report(incidents, report_format)
        
        # Save report
        output_file = os.path.join('.', f'doctor-report.{report_format}')
        with open(output_file, 'w') as f:
            if report_format == 'json':
                json.dump(report, f, indent=2, default=str)
            else:
                f.write(report)
        
        self.logger.info(f"Report saved to: {output_file}")
        
        return {
            'report': report,
            'output_file': output_file,
            'report_time': datetime.now().isoformat()
        }

def main():
    """Main CLI entry point"""
    parser = argparse.ArgumentParser(description='Doctor MVP - Incident Repair Automation System')
    parser.add_argument('--config', help='Path to configuration file')
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Scan command
    scan_parser = subparsers.add_parser('scan', help='Scan log files for incidents')
    scan_parser.add_argument('files', nargs='+', help='Log files to scan')
    scan_parser.add_argument('--output', help='Output directory for results')
    
    # Triage command
    triage_parser = subparsers.add_parser('triage', help='Classify incidents')
    triage_parser.add_argument('incidents_file', help='File containing incidents (JSON)')
    triage_parser.add_argument('--output', help='Output directory for results')
    
    # Handoff command
    handoff_parser = subparsers.add_parser('handoff', help='Create repair brief')
    handoff_parser.add_argument('incident_id', help='Incident ID to create repair brief for')
    handoff_parser.add_argument('--output', help='Output directory for results')
    
    # Verify command
    verify_parser = subparsers.add_parser('verify', help='Verify repair')
    verify_parser.add_argument('repair_brief_file', help='Repair brief file to verify')
    
    # Report command
    report_parser = subparsers.add_parser('report', help='Generate final report')
    report_parser.add_argument('incidents_file', help='File containing incidents (JSON)')
    report_parser.add_argument('--format', choices=['json', 'text'], default='json', help='Report format')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    try:
        cli = DoctorCLI(args.config)
        
        if args.command == 'scan':
            result = cli.scan(args.files, args.output)
            print(json.dumps(result, indent=2, default=str))
        
        elif args.command == 'triage':
            result = cli.triage(args.incidents_file, args.output)
            print(json.dumps(result, indent=2, default=str))
        
        elif args.command == 'handoff':
            result = cli.handoff(args.incident_id, args.output)
            print(json.dumps(result, indent=2, default=str))
        
        elif args.command == 'verify':
            result = cli.verify(args.repair_brief_file)
            print(json.dumps(result, indent=2, default=str))
        
        elif args.command == 'report':
            result = cli.report(args.incidents_file, args.format)
            print(json.dumps(result, indent=2, default=str))
    
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()