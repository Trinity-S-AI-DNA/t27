#!/usr/bin/env python3
"""
Incident Scanner - Detects incidents in log files
"""

import json
import logging
import os
import re
import uuid
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Any, Optional

class IncidentScanner:
    """Scans log files for potential incidents"""
    
    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.logger = logging.getLogger(__name__)
        
        # Load incident schema
        self.schema_path = os.path.join(
            os.path.dirname(__file__), '..', 'incident-schema.json'
        )
        self.schema = self._load_schema()
        
        # Common patterns for incident detection
        self.patterns = {
            'error': re.compile(r'\[ERROR\]', re.IGNORECASE),
            'failed': re.compile(r'failed', re.IGNORECASE),
            'blocked': re.compile(r'blocked', re.IGNORECASE),
            'timeout': re.compile(r'timeout', re.IGNORECASE),
            'exception': re.compile(r'exception|traceback', re.IGNORECASE),
            'http_error': re.compile(r'HTTP\s+(4\d{2}|5\d{2})', re.IGNORECASE),
            'disk_full': re.compile(r'disk\s+(full|no\s+space\s+left|quota\s+exceeded)', re.IGNORECASE),
            'git_error': re.compile(r'git\s+(push|pull).*?(blocked|failed|rejected)', re.IGNORECASE),
        }
    
    def _load_schema(self) -> Dict[str, Any]:
        """Load incident schema"""
        try:
            with open(self.schema_path, 'r') as f:
                return json.load(f)
        except Exception as e:
            self.logger.error(f"Failed to load schema: {e}")
            return {}
    
    def scan_file(self, file_path: str) -> List[Dict[str, Any]]:
        """Scan a single file for incidents"""
        incidents = []
        
        try:
            self.logger.info(f"Scanning file: {file_path}")
            
            with open(file_path, 'r') as f:
                lines = f.readlines()
            
            # Look for incident patterns
            for line_num, line in enumerate(lines, 1):
                incident = self._analyze_line(line, file_path, line_num)
                if incident:
                    incidents.append(incident)
            
            self.logger.info(f"Found {len(incidents)} incidents in {file_path}")
        
        except Exception as e:
            self.logger.error(f"Error scanning file {file_path}: {e}")
        
        return incidents
    
    def _analyze_line(self, line: str, file_path: str, line_num: int) -> Optional[Dict[str, Any]]:
        """Analyze a single line for potential incidents"""
        line = line.strip()
        if not line:
            return None
        
        # Check for error patterns
        incident_type = self._classify_line(line)
        if not incident_type:
            return None
        
        # Create incident object
        incident = {
            'incident_id': str(uuid.uuid4()),
            'timestamp': self._extract_timestamp(line),
            'source_log': file_path,
            'incident_type': incident_type,
            'severity': self._determine_severity(line, incident_type),
            'error_message': line,
            'error_code': self._extract_error_code(line),
            'context': self._extract_context(line, file_path, line_num),
            'evidence': [{
                'type': 'log_entry',
                'content': line,
                'timestamp': self._extract_timestamp(line) or datetime.now().isoformat(),
                'source_file': file_path,
                'line_number': line_num
            }],
            'resolution_status': 'OPEN',
            'tags': self._generate_tags(line, incident_type),
            'repair_attempts': []
        }
        
        return incident
    
    def _classify_line(self, line: str) -> Optional[str]:
        """Classify line to incident type based on patterns"""
        line_lower = line.lower()
        
        # Check each pattern
        if re.search(r'failed to compile|build error|cargo build failed', line_lower):
            return 'BUILD_FAILURE'
        
        if re.search(r'test failed|assertion failed|pytest: failed', line_lower):
            return 'TEST_FAILURE'
        
        if re.search(r'disk full|no space left on device|disk quota exceeded', line_lower):
            return 'DISK_PRESSURE'
        
        if re.search(r'provider funding|payment required|HTTP 402|403', line_lower):
            return 'PROVIDER_UNFUNDED'
        
        if re.search(r'wrong directory|not in repository', line_lower):
            return 'WRONG_WORKSPACE'
        
        if re.search(r'git push.*blocked|push rejected|blocked from being pushed', line_lower):
            return 'GIT_PUSH_BLOCKED'
        
        if re.search(r'protected file|state risk', line_lower):
            return 'PROTECTED_STATE_RISK'
        
        # Generic error classification
        if any(pattern.search(line_lower) for pattern in [
            self.patterns['error'],
            self.patterns['failed'],
            self.patterns['exception'],
            self.patterns['http_error']
        ]):
            return 'UNKNOWN'
        
        return None
    
    def _determine_severity(self, line: str, incident_type: str) -> str:
        """Determine severity based on incident type and content"""
        if incident_type in ['DISK_PRESSURE', 'PROVIDER_UNFUNDED', 'PROTECTED_STATE_RISK']:
            return 'CRITICAL'
        elif incident_type in ['BUILD_FAILURE', 'GIT_PUSH_BLOCKED']:
            return 'HIGH'
        elif incident_type == 'TEST_FAILURE':
            return 'MEDIUM'
        elif incident_type == 'UNKNOWN':
            return 'MEDIUM'
        else:
            return 'LOW'
    
    def _extract_timestamp(self, line: str) -> Optional[str]:
        """Extract timestamp from line"""
        # Try common timestamp formats
        patterns = [
            r'(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[Z\+\-]?\d*)',  # ISO format
            r'(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})',  # Standard format
        ]
        
        for pattern in patterns:
            match = re.search(pattern, line)
            if match:
                return match.group(1)
        
        return None
    
    def _extract_error_code(self, line: str) -> Optional[str]:
        """Extract error code from line"""
        patterns = [
            r'HTTP\s+(\d{3})',
            r'Error code: (\d+)',
            r'\[(\d{3})\]',
        ]
        
        for pattern in patterns:
            match = re.search(pattern, line)
            if match:
                return match.group(1)
        
        return None
    
    def _extract_context(self, line: str, file_path: str, line_num: int) -> Dict[str, Any]:
        """Extract context information from line"""
        context = {
            'line_number': line_num,
            'source_file': file_path,
            'session_id': self._extract_session_id(line)
        }
        
        # Add type-specific context
        incident_type = self._classify_line(line)
        
        if incident_type == 'GIT_PUSH_BLOCKED':
            context.update({
                'git_remote': self._extract_git_remote(line),
                'working_directory': os.path.dirname(file_path)
            })
        elif incident_type == 'PROVIDER_UNFUNDED':
            context.update({
                'provider_name': self._extract_provider_name(line),
                'provider_id': self._extract_provider_id(line)
            })
        
        return context
    
    def _extract_session_id(self, line: str) -> Optional[str]:
        """Extract session ID from line"""
        match = re.search(r'Session:\s*([^\s]+)', line)
        return match.group(1) if match else None
    
    def _extract_git_remote(self, line: str) -> Optional[str]:
        """Extract git remote from line"""
        match = re.search(r'Remote:\s*([^\s]+)', line)
        return match.group(1) if match else None
    
    def _extract_provider_name(self, line: str) -> Optional[str]:
        """Extract provider name from line"""
        match = re.search(r'Provider:\s*([^\s]+)', line)
        return match.group(1) if match else None
    
    def _extract_provider_id(self, line: str) -> Optional[str]:
        """Extract provider ID from line"""
        match = re.search(r'Provider ID:\s*([^\s]+)', line)
        return match.group(1) if match else None
    
    def _generate_tags(self, line: str, incident_type: str) -> List[str]:
        """Generate tags for the incident"""
        tags = [incident_type.lower()]
        
        # Add severity tag
        if incident_type in ['DISK_PRESSURE', 'PROVIDER_UNFUNDED', 'PROTECTED_STATE_RISK']:
            tags.append('critical')
        elif incident_type in ['BUILD_FAILURE', 'GIT_PUSH_BLOCKED']:
            tags.append('high')
        
        # Add session tag if present
        session_id = self._extract_session_id(line)
        if session_id:
            tags.append(f'session_{session_id.split("-")[0]}')
        
        return tags
    
    def scan_directory(self, directory: str, recursive: bool = True) -> List[Dict[str, Any]]:
        """Scan all log files in a directory"""
        incidents = []
        log_files = []
        
        # Find log files
        if recursive:
            pattern = "**/*.log"
            log_files = [str(p) for p in Path(directory).glob(pattern)]
        else:
            log_files = [str(p) for p in Path(directory).glob("*.log")]
        
        self.logger.info(f"Found {len(log_files)} log files to scan")
        
        for log_file in log_files:
            file_incidents = self.scan_file(log_file)
            incidents.extend(file_incidents)
        
        return incidents